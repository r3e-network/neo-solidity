fn manifest_type_name(ty: ManifestType) -> &'static str {
    match ty {
        ManifestType::Integer => "Integer",
        ManifestType::Boolean => "Boolean",
        ManifestType::String => "String",
        ManifestType::Hash160 => "Hash160",
        ManifestType::Hash256 => "Hash256",
        ManifestType::ByteArray => "ByteArray",
        ManifestType::Array => "Array",
        ManifestType::Map => "Map",
        ManifestType::Any => "Any",
    }
}

fn value_type_satisfies_manifest_type(actual: &ValueType, expected: ManifestType) -> Option<bool> {
    if expected == ManifestType::Any {
        return Some(true);
    }

    match actual {
        ValueType::Any => None,
        ValueType::Integer { .. } => Some(expected == ManifestType::Integer),
        ValueType::Boolean => Some(expected == ManifestType::Boolean),
        ValueType::String => Some(matches!(
            expected,
            ManifestType::String | ManifestType::ByteArray
        )),
        ValueType::Address => Some(matches!(
            expected,
            ManifestType::Hash160 | ManifestType::ByteArray
        )),
        ValueType::ByteArray { fixed_len } => match expected {
            ManifestType::Hash160 => Some(matches!(fixed_len, Some(20))),
            ManifestType::Hash256 => Some(matches!(fixed_len, Some(32))),
            ManifestType::ByteArray => Some(true),
            _ => Some(false),
        },
        ValueType::Array(_) => Some(expected == ManifestType::Array),
        ValueType::Mapping { .. } => Some(expected == ManifestType::Map),
        ValueType::Struct { .. } => Some(expected == ManifestType::Array),
    }
}

fn extract_string_literal(expr: &Expression) -> Option<String> {
    match expr {
        Expression::StringLiteral(parts) => {
            Some(String::from_utf8_lossy(&string_literal_bytes(parts)).to_string())
        }
        _ => None,
    }
}

fn extract_abi_encode_args(expr: &Expression) -> Option<&[Expression]> {
    let Expression::FunctionCall(_, func, args) = expr else {
        return None;
    };

    let Expression::MemberAccess(_, inner, member) = func.as_ref() else {
        return None;
    };
    let Expression::Variable(base) = inner.as_ref() else {
        return None;
    };

    if base.name == "abi"
        && (member.name == "encode" || member.name == "encodePacked" || member.name == "encodeCall")
    {
        Some(args.as_slice())
    } else {
        None
    }
}

fn validate_runtime_notify_call(args: &[Expression], ctx: &mut LoweringContext) {
    if args.len() != 2 {
        return;
    }

    let Some(event_name) = extract_string_literal(&args[0]) else {
        return;
    };

    let Some(expected_sig) = ctx.event_signature(&event_name).map(|sig| sig.to_vec()) else {
        ctx.record_error(format!(
            "Runtime.notify refers to event '{event_name}' which is not declared; declare a matching Solidity `event` so it is included in the contract manifest ABI"
        ));
        return;
    };

    let Some(encoded_args) = extract_abi_encode_args(&args[1]) else {
        return;
    };

    if expected_sig.len() != encoded_args.len() {
        ctx.record_error(format!(
            "Runtime.notify event '{event_name}' expects {} argument(s), but abi.encode(...) provides {}",
            expected_sig.len(),
            encoded_args.len()
        ));
        return;
    }

    for (index, (expected_ty, arg_expr)) in expected_sig
        .iter()
        .copied()
        .zip(encoded_args.iter())
        .enumerate()
    {
        if expected_ty == ManifestType::Any {
            continue;
        }

        let Some(actual_ty) = infer_type_from_expression(arg_expr, ctx) else {
            continue;
        };

        if value_type_satisfies_manifest_type(&actual_ty, expected_ty) == Some(false) {
            ctx.record_error(format!(
                "Runtime.notify event '{event_name}' argument #{} has incompatible type (expected {}, got {:?})",
                index,
                manifest_type_name(expected_ty),
                actual_ty
            ));
        }
    }
}

/// Task #106 — when an expression passed to `abi.encodeCall` is a bare
/// Variable reference to a struct-typed function parameter, return the
/// flattened parameter-slot range `(base_slot, field_count)` so the caller
/// can emit `LoadParameter(base..base+count)` and feed the flattened fields
/// into `abiEncode`. This implements per-field expansion for struct args:
/// `abi.encodeCall(this.f, (p))` where `p = P{uint256 a; bool b}` lowers to
/// `sel || abiEncode(p.a, p.b)` instead of `sel || abiEncode(p)` (which
/// would produce zero-slot output for the struct-as-Array shape).
///
/// Returns `None` if `expr` is not a bare Variable reference, the variable
/// is not a function parameter, or the parameter is not a struct type. The
/// caller should then fall through to the standard `lower_expression`
/// path.
fn resolve_struct_param_flat_slots(
    expr: &Expression,
    ctx: &LoweringContext,
) -> Option<(usize, usize)> {
    // Look through parentheses.
    let mut current = expr;
    while let Expression::Parenthesis(_, inner) = current {
        current = inner.as_ref();
    }
    let Expression::Variable(id) = current else {
        return None;
    };
    let &param_index = ctx.param_index_map.get(&id.name)?;
    let param_ty = ctx.param_types.get(param_index)?;
    // Only expand struct-typed params. Returns the flat slot range in the
    // post-flattening param layout (see `flatten_param_slot_map` in
    // bytecode_emit_ir). For Task #106 the expansion is single-level: a
    // struct's direct fields become consecutive slots; nested structs
    // would require recursive expansion, which is a follow-up.
    let ValueType::Struct { fields, .. } = param_ty else {
        return None;
    };
    // Compute the flattened starting slot for this struct param by counting
    // flat slots for all preceding params.
    let mut base_slot: usize = 0;
    for i in 0..param_index {
        let prev = ctx.param_types.get(i)?;
        base_slot += flat_slot_count_for_param_type(prev);
    }
    Some((base_slot, fields.len()))
}

/// Returns the number of flattened parameter slots this `ValueType` occupies
/// in the post-Task-#106 INITSLOT layout. Structs flatten to their direct
/// field count; all other value types count as 1. Nested structs flatten
/// only one level (fields that are themselves structs stay as single slots
/// for now — nested expansion is a follow-up).
fn flat_slot_count_for_param_type(ty: &ValueType) -> usize {
    match ty {
        ValueType::Struct { fields, .. } => fields.len(),
        _ => 1,
    }
}

/// Task #124 — `abi.encode(struct)` whole-struct expansion. If `expr` has a
/// struct type, emit per-field extraction instructions so the downstream
/// `AbiEncode` builtin receives N individual stack items (one per direct
/// field) instead of a single `StackItem::Array` that the runtime's
/// `abiencode` handler would misclassify as a DYNAMIC type (Task #121's
/// offset+length+elements shape).
///
/// EVM ABI spec: a struct whose fields are all static types is itself a
/// STATIC type, and `abi.encode(s)` must equal the byte-for-byte concatenation
/// of each field's 32-byte BE-padded slot (no offset/length header). For the
/// `keccak256(abi.encode(Voucher{amount, recipient, expiry}))` case this is
/// the 96-byte buffer `BE(amount) || BE(addr) || BE(expiry)`.
///
/// Emission shape (per field `i`):
///   `lower_expression(expr); push i; ArrayGet`
/// which reads the struct's `StackItem::Array` backing storage and indexes
/// into it. For a bare Variable reference to a struct param this is cheap
/// (each `lower_expression` is a single `LoadParameter`); for storage-
/// backed struct locals it re-runs the storage-load but is still correct.
/// The runtime's `ArrayGet` unwraps the field value (Integer / ByteArray /
/// Boolean) so `abi_pad32_be` in the runtime handler sees a scalar it can
/// classify as STATIC.
///
/// Returns `Some(field_count)` if flattening fired, `None` if the expression
/// is not struct-typed (caller falls through to standard `lower_expression`).
/// `success` is threaded through so the caller can short-circuit when a
/// nested lowering fails.
fn try_flatten_struct_arg_for_abi_encode(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    success: &mut bool,
) -> Option<usize> {
    // Look through parentheses to reach the expression we'll type-check.
    let mut current = expr;
    while let Expression::Parenthesis(_, inner) = current {
        current = inner.as_ref();
    }
    let ValueType::Struct { fields, .. } = infer_type_from_expression(current, ctx)? else {
        return None;
    };
    // Only flatten when EVERY field is a STATIC EVM type. Dynamic fields
    // (string/bytes/T[]) would still need the head+tail layout, which the
    // runtime handler already produces for a top-level Array. Avoid muddying
    // the DD3 fix with nested dynamics — gate the flatten on all-static.
    let all_static = fields.iter().all(|f| is_static_abi_type_value(&f.ty));
    if !all_static {
        return None;
    }
    let field_count = fields.len();
    if field_count == 0 {
        return None;
    }
    for i in 0..field_count {
        if !lower_expression(current, ctx, instructions) {
            *success = false;
            return Some(field_count);
        }
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::from(i as u64),
        )));
        instructions.push(Instruction::ArrayGet);
    }
    Some(field_count)
}

/// Task #124 — true iff a `ValueType` is a STATIC EVM ABI type (all-static
/// structs fold to per-field 32-byte slots, no offset/length header). Mirrors
/// the `is_static_abi_type` predicate used by `abi_decode_expected_static_bytes`
/// but operates on the IR `ValueType` rather than a solang `PtType`.
fn is_static_abi_type_value(ty: &ValueType) -> bool {
    match ty {
        ValueType::Integer { .. } => true,
        ValueType::Boolean => true,
        ValueType::Address => true,
        ValueType::ByteArray { fixed_len: Some(_) } => true,
        // Unknown-width bytes arrays (`bytes memory`), strings, arrays, and
        // mappings are dynamic and must keep the head+tail layout.
        ValueType::ByteArray { fixed_len: None } => false,
        ValueType::String => false,
        ValueType::Array(_) => false,
        ValueType::Mapping { .. } => false,
        // Nested structs: conservative — keep at single-level and treat as
        // dynamic (so the outer encode uses the existing StackItem::Array
        // fallback path for nested structs). Nested-struct flattening is
        // a follow-up if harnesses need it.
        ValueType::Struct { .. } => false,
        ValueType::Any => false,
    }
}

fn lower_abi_encode_args_direct_from_slice(
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    let refs: Vec<&Expression> = args.iter().collect();
    lower_abi_encode_args_direct(&refs, ctx, instructions)
}

fn lower_abi_encode_args_direct(
    args: &[&Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    lower_abi_encode_args_direct_impl(args, ctx, instructions, false)
}

fn lower_abi_encode_args_direct_for_encode_call(
    args: &[&Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    lower_abi_encode_args_direct_impl(args, ctx, instructions, true)
}

fn lower_abi_encode_args_direct_impl(
    args: &[&Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    flatten_struct_params: bool,
) -> Option<bool> {
    if args.is_empty() {
        instructions.push(Instruction::PushLiteral(
            LiteralValue::ByteArray(Vec::new()),
        ));
        return Some(true);
    }

    if let Some(result) = lower_abi_encode_head_tail_direct(args, ctx, instructions) {
        return Some(result);
    }

    let pre_len = instructions.len();
    let mut slot_count = 0usize;
    for arg in args {
        match lower_static_abi_slots_for_expr(arg, ctx, instructions, flatten_struct_params) {
            Some(slots) => slot_count += slots,
            None => {
                instructions.truncate(pre_len);
                return None;
            }
        }
    }

    if slot_count == 0 {
        instructions.truncate(pre_len);
        return None;
    }

    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::BytesConcat,
        arg_count: slot_count,
    });
    Some(true)
}

fn lower_abi_encode_head_tail_direct(
    args: &[&Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    let arg_types: Vec<ValueType> = args
        .iter()
        .map(|arg| infer_type_from_expression(arg, ctx))
        .collect::<Option<Vec<_>>>()?;

    if !arg_types.iter().any(abi_value_type_is_dynamic) {
        return None;
    }

    let mut head_slot_count = 0usize;
    for value_type in &arg_types {
        if abi_value_type_is_dynamic(value_type) {
            if !abi_dynamic_value_type_is_supported(value_type) {
                return None;
            }
            head_slot_count += 1;
        } else if let Some(slots) = abi_static_slot_count(value_type) {
            head_slot_count += slots;
        } else {
            return None;
        }
    }

    let pre_len = instructions.len();
    let mut value_locals = Vec::with_capacity(args.len());
    for (index, (arg, value_type)) in args.iter().zip(arg_types.iter()).enumerate() {
        if !lower_expression(arg, ctx, instructions) {
            instructions.truncate(pre_len);
            return Some(false);
        }
        let tmp_id = ctx.next_label();
        let local = ctx.allocate_local(
            format!("__abi_arg_{index}_{tmp_id}"),
            Some(value_type.clone()),
        );
        instructions.push(Instruction::StoreLocal(local));
        value_locals.push(local);
    }

    let offset_local = ctx.allocate_local("__abi_tail_offset".to_string(), None);
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        (head_slot_count * 32) as u64,
    ))));
    instructions.push(Instruction::StoreLocal(offset_local));

    let mut tail_locals = Vec::new();
    let mut part_count = 0usize;

    for (local, value_type) in value_locals.iter().zip(arg_types.iter()) {
        if abi_value_type_is_dynamic(value_type) {
            instructions.push(Instruction::LoadLocal(offset_local));
            emit_abi_u256_slot(ctx, instructions);
            part_count += 1;

            instructions.push(Instruction::LoadLocal(*local));
            emit_abi_dynamic_tail_for_value_type(value_type, ctx, instructions)?;
            let tail_local = ctx.allocate_local("__abi_tail".to_string(), None);
            instructions.push(Instruction::StoreLocal(tail_local));

            instructions.push(Instruction::LoadLocal(offset_local));
            instructions.push(Instruction::LoadLocal(tail_local));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(offset_local));
            tail_locals.push(tail_local);
        } else {
            let slots =
                emit_abi_static_slots_from_local(*local, value_type, ctx, instructions)?;
            part_count += slots;
        }
    }

    for tail_local in tail_locals {
        instructions.push(Instruction::LoadLocal(tail_local));
        part_count += 1;
    }

    if part_count == 0 {
        instructions.truncate(pre_len);
        return None;
    }

    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::BytesConcat,
        arg_count: part_count,
    });
    Some(true)
}

fn lower_abi_encode_packed_args_direct_from_slice(
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if args.is_empty() {
        instructions.push(Instruction::PushLiteral(
            LiteralValue::ByteArray(Vec::new()),
        ));
        return Some(true);
    }

    let pre_len = instructions.len();
    for arg in args {
        if !lower_packed_abi_bytes_for_expr(arg, ctx, instructions)? {
            instructions.truncate(pre_len);
            return Some(false);
        }
    }

    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::BytesConcat,
        arg_count: args.len(),
    });
    Some(true)
}

fn lower_static_abi_slots_for_expr(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    flatten_struct_params: bool,
) -> Option<usize> {
    let value_type = infer_type_from_expression(expr, ctx)?;
    match value_type {
        ValueType::Struct { fields, .. } => {
            if fields.is_empty()
                || !fields
                    .iter()
                    .all(|field| is_static_abi_type_value(&field.ty))
            {
                return None;
            }

            if flatten_struct_params {
                if let Some((base_slot, field_count)) = resolve_struct_param_flat_slots(expr, ctx) {
                    if field_count != fields.len() {
                        return None;
                    }
                    for (index, field) in fields.iter().enumerate() {
                        instructions.push(Instruction::LoadParameter(base_slot + index));
                        emit_expr_static_abi_slot_for_value_type(&field.ty, ctx, instructions)?;
                    }
                    return Some(fields.len());
                }
            }

            let tmp_id = ctx.next_label();
            let struct_local = ctx.allocate_local(format!("__abi_static_struct_{tmp_id}"), None);
            if !lower_expression(expr, ctx, instructions) {
                return Some(0);
            }
            instructions.push(Instruction::StoreLocal(struct_local));

            for (index, field) in fields.iter().enumerate() {
                instructions.push(Instruction::LoadLocal(struct_local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(index as u64),
                )));
                instructions.push(Instruction::ArrayGet);
                emit_expr_static_abi_slot_for_value_type(&field.ty, ctx, instructions)?;
            }
            Some(fields.len())
        }
        other if is_static_abi_type_value(&other) => {
            if !lower_expression(expr, ctx, instructions) {
                return Some(0);
            }
            emit_expr_static_abi_slot_for_value_type(&other, ctx, instructions)?;
            Some(1)
        }
        _ => None,
    }
}

fn emit_expr_static_abi_slot_for_value_type(
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<()> {
    match value_type {
        ValueType::Integer { .. } | ValueType::Boolean | ValueType::Address => {
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            emit_abi_fixed_buffer(ctx, instructions, 32, true);
            Some(())
        }
        ValueType::ByteArray {
            fixed_len: Some(len),
        } if *len == 32 => Some(()),
        ValueType::ByteArray {
            fixed_len: Some(len),
        } if *len < 32 => {
            emit_abi_bytesn_slot(ctx, instructions, *len as usize);
            Some(())
        }
        _ => None,
    }
}

fn lower_packed_abi_bytes_for_expr(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    let value_type = infer_type_from_expression(expr, ctx)?;
    let pre_len = instructions.len();
    if !lower_expression(expr, ctx, instructions) {
        return Some(false);
    }

    let lowered = match value_type {
        ValueType::Integer { bits, .. } => {
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            emit_abi_fixed_buffer(ctx, instructions, (bits / 8) as usize, true);
            Some(true)
        }
        ValueType::Boolean => {
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            emit_abi_fixed_buffer(ctx, instructions, 1, true);
            Some(true)
        }
        ValueType::Address => {
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            emit_abi_fixed_buffer(ctx, instructions, 20, true);
            Some(true)
        }
        ValueType::ByteArray { fixed_len: Some(_) }
        | ValueType::ByteArray { fixed_len: None }
        | ValueType::String => Some(true),
        ValueType::Array(element_type) if is_static_abi_type_value(&element_type) => {
            emit_abi_packed_static_array(&element_type, ctx, instructions)?;
            Some(true)
        }
        _ => None,
    };

    if lowered.is_none() {
        instructions.truncate(pre_len);
    }
    lowered
}

fn lower_abi_decode_direct(
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if args.len() != 2 {
        return None;
    }

    let types = abi_decode_value_types(&args[1], ctx)?;
    if types.is_empty()
        || !types
            .iter()
            .all(|value_type| abi_static_slot_count(value_type) == Some(1))
    {
        return None;
    }

    let pre_len = instructions.len();
    if !lower_expression(&args[0], ctx, instructions) {
        instructions.truncate(pre_len);
        return Some(false);
    }

    let buffer_local = ctx.allocate_local("__abi_decode_buf".to_string(), None);
    instructions.push(Instruction::StoreLocal(buffer_local));

    let expected_bytes = types.len() * 32;
    let decode_ok_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(buffer_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        expected_bytes as u64,
    ))));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Ne));
    instructions.push(Instruction::JumpIf {
        target: decode_ok_label,
    });
    emit_panic(0x41, instructions);
    instructions.push(Instruction::Label(decode_ok_label));

    if types.len() == 1 {
        emit_abi_decode_static_slot(buffer_local, 0, &types[0], ctx, instructions);
        return Some(true);
    }

    let tuple_local = ctx.allocate_local(
        "__abi_decode_tuple".to_string(),
        Some(ValueType::Array(Box::new(ValueType::Any))),
    );
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        types.len() as u64,
    ))));
    instructions.push(Instruction::NewArray {
        element_type: ValueType::Any,
    });
    instructions.push(Instruction::StoreLocal(tuple_local));

    for (index, value_type) in types.iter().enumerate() {
        instructions.push(Instruction::LoadLocal(tuple_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
            index as u64,
        ))));
        emit_abi_decode_static_slot(buffer_local, index, value_type, ctx, instructions);
        instructions.push(Instruction::ArraySet);
    }

    instructions.push(Instruction::LoadLocal(tuple_local));
    Some(true)
}

fn lower_neo_serialized_arg_array(
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let tmp_id = ctx.next_label();
    let array_local = ctx.allocate_local(format!("__neo_serialized_args_{tmp_id}"), None);

    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        args.len() as u64,
    ))));
    instructions.push(Instruction::NewArray {
        element_type: ValueType::Any,
    });
    instructions.push(Instruction::StoreLocal(array_local));

    let mut success = true;
    for (index, arg) in args.iter().enumerate() {
        instructions.push(Instruction::LoadLocal(array_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
            index as u64,
        ))));
        if !lower_expression(arg, ctx, instructions) {
            success = false;
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
        }
        instructions.push(Instruction::ArraySet);
    }

    instructions.push(Instruction::LoadLocal(array_local));
    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::NativeCall {
            contract: NativeContract::StdLib,
            method: "serialize".to_string(),
        },
        arg_count: 1,
    });

    success
}

fn abi_decode_value_types(
    types_expr: &Expression,
    ctx: &LoweringContext,
) -> Option<Vec<ValueType>> {
    match types_expr {
        Expression::List(_, params) => params
            .iter()
            .map(|(_, param)| {
                param
                    .as_ref()
                    .and_then(|parameter| infer_type_from_expression(&parameter.ty, ctx))
            })
            .collect(),
        Expression::Parenthesis(_, inner) => infer_type_from_expression(inner, ctx).map(|ty| vec![ty]),
        other => infer_type_from_expression(other, ctx).map(|ty| vec![ty]),
    }
}

fn abi_static_slot_count(value_type: &ValueType) -> Option<usize> {
    match value_type {
        ValueType::Struct { fields, .. }
            if !fields.is_empty()
                && fields.iter().all(|field| is_static_abi_type_value(&field.ty)) =>
        {
            Some(fields.len())
        }
        other if is_static_abi_type_value(other) => Some(1),
        _ => None,
    }
}

fn abi_value_type_is_dynamic(value_type: &ValueType) -> bool {
    matches!(
        value_type,
        ValueType::ByteArray { fixed_len: None } | ValueType::String | ValueType::Array(_)
    )
}

fn abi_dynamic_value_type_is_supported(value_type: &ValueType) -> bool {
    match value_type {
        ValueType::ByteArray { fixed_len: None } | ValueType::String => true,
        ValueType::Array(element_type) => abi_static_slot_count(element_type).is_some(),
        _ => false,
    }
}

fn emit_abi_static_slots_from_local(
    local: usize,
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<usize> {
    match value_type {
        ValueType::Struct { fields, .. }
            if !fields.is_empty()
                && fields.iter().all(|field| is_static_abi_type_value(&field.ty)) =>
        {
            for (index, field) in fields.iter().enumerate() {
                instructions.push(Instruction::LoadLocal(local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
                    index as u64,
                ))));
                instructions.push(Instruction::ArrayGet);
                emit_expr_static_abi_slot_for_value_type(&field.ty, ctx, instructions)?;
            }
            Some(fields.len())
        }
        other if is_static_abi_type_value(other) => {
            instructions.push(Instruction::LoadLocal(local));
            emit_expr_static_abi_slot_for_value_type(other, ctx, instructions)?;
            Some(1)
        }
        _ => None,
    }
}

fn emit_abi_encode_single_stack_value_for_type(
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<()> {
    if abi_value_type_is_dynamic(value_type) {
        if !abi_dynamic_value_type_is_supported(value_type) {
            return None;
        }

        let value_local = ctx.allocate_local("__abi_single_value".to_string(), None);
        instructions.push(Instruction::StoreLocal(value_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
            32u8,
        ))));
        emit_abi_u256_slot(ctx, instructions);
        instructions.push(Instruction::LoadLocal(value_local));
        emit_abi_dynamic_tail_for_value_type(value_type, ctx, instructions)?;
        instructions.push(Instruction::CallBuiltin {
            builtin: BuiltinCall::BytesConcat,
            arg_count: 2,
        });
        return Some(());
    }

    if abi_static_slot_count(value_type) == Some(1) {
        emit_expr_static_abi_slot_for_value_type(value_type, ctx, instructions)?;
        return Some(());
    }

    None
}

fn emit_abi_dynamic_tail_for_value_type(
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<()> {
    match value_type {
        ValueType::ByteArray { fixed_len: None } | ValueType::String => {
            emit_abi_dynamic_bytes_tail(ctx, instructions);
            Some(())
        }
        ValueType::Array(element_type) if abi_static_slot_count(element_type).is_some() => {
            emit_abi_dynamic_static_array_tail(element_type, ctx, instructions)
        }
        _ => None,
    }
}

fn emit_abi_dynamic_bytes_tail(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    let tmp_id = ctx.next_label();
    let src_local = ctx.allocate_local(format!("__abi_dyn_bytes_src_{tmp_id}"), None);
    let len_local = ctx.allocate_local(format!("__abi_dyn_bytes_len_{tmp_id}"), None);
    let padded_len_local =
        ctx.allocate_local(format!("__abi_dyn_bytes_padded_len_{tmp_id}"), None);
    let padded_local = ctx.allocate_local(format!("__abi_dyn_bytes_padded_{tmp_id}"), None);
    let len_slot_local = ctx.allocate_local(format!("__abi_dyn_bytes_len_slot_{tmp_id}"), None);

    instructions.push(Instruction::StoreLocal(src_local));
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(len_local));

    instructions.push(Instruction::LoadLocal(len_local));
    emit_abi_u256_slot(ctx, instructions);
    instructions.push(Instruction::StoreLocal(len_slot_local));

    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        31u8,
    ))));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        32u8,
    ))));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Div));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        32u8,
    ))));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
    instructions.push(Instruction::StoreLocal(padded_len_local));

    instructions.push(Instruction::LoadLocal(padded_len_local));
    instructions.push(Instruction::NewBuffer);
    instructions.push(Instruction::StoreLocal(padded_local));

    instructions.push(Instruction::LoadLocal(padded_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::MemCpy);

    instructions.push(Instruction::LoadLocal(len_slot_local));
    instructions.push(Instruction::LoadLocal(padded_local));
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::BytesConcat,
        arg_count: 2,
    });
}

fn emit_abi_dynamic_static_array_tail(
    element_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<()> {
    emit_abi_static_array_buffer(element_type, ctx, instructions, true)
}

fn emit_abi_packed_static_array(
    element_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<()> {
    emit_abi_static_array_buffer(element_type, ctx, instructions, false)
}

fn emit_abi_static_slots_for_stack_value(
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<usize> {
    match value_type {
        ValueType::Struct { fields, .. }
            if !fields.is_empty()
                && fields.iter().all(|field| is_static_abi_type_value(&field.ty)) =>
        {
            let tmp_id = ctx.next_label();
            let struct_local = ctx.allocate_local(format!("__abi_stack_struct_{tmp_id}"), None);
            instructions.push(Instruction::StoreLocal(struct_local));
            for (index, field) in fields.iter().enumerate() {
                instructions.push(Instruction::LoadLocal(struct_local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
                    index as u64,
                ))));
                instructions.push(Instruction::ArrayGet);
                emit_expr_static_abi_slot_for_value_type(&field.ty, ctx, instructions)?;
            }
            Some(fields.len())
        }
        other if is_static_abi_type_value(other) => {
            emit_expr_static_abi_slot_for_value_type(other, ctx, instructions)?;
            Some(1)
        }
        _ => None,
    }
}

fn emit_abi_static_array_buffer(
    element_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    include_length: bool,
) -> Option<()> {
    let element_slot_count = abi_static_slot_count(element_type)?;
    if element_slot_count == 0 {
        return None;
    }
    let element_byte_len = element_slot_count * 32;
    let tmp_id = ctx.next_label();
    let arr_local = ctx.allocate_local(format!("__abi_arr_{tmp_id}"), None);
    let len_local = ctx.allocate_local(format!("__abi_arr_len_{tmp_id}"), None);
    let out_local = ctx.allocate_local(format!("__abi_arr_out_{tmp_id}"), None);
    let idx_local = ctx.allocate_local(format!("__abi_arr_idx_{tmp_id}"), None);
    let elem_slot_local = ctx.allocate_local(format!("__abi_arr_elem_slot_{tmp_id}"), None);

    instructions.push(Instruction::StoreLocal(arr_local));
    instructions.push(Instruction::LoadLocal(arr_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(len_local));

    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        element_byte_len as u64,
    ))));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
    if include_length {
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
            32u8,
        ))));
        instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    }
    instructions.push(Instruction::NewBuffer);
    instructions.push(Instruction::StoreLocal(out_local));

    if include_length {
        instructions.push(Instruction::LoadLocal(len_local));
        emit_abi_u256_slot(ctx, instructions);
        let len_slot_local = ctx.allocate_local(format!("__abi_arr_len_slot_{tmp_id}"), None);
        instructions.push(Instruction::StoreLocal(len_slot_local));
        instructions.push(Instruction::LoadLocal(out_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
        instructions.push(Instruction::LoadLocal(len_slot_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
            32u8,
        ))));
        instructions.push(Instruction::MemCpy);
    }

    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::StoreLocal(idx_local));

    let loop_label = ctx.next_label();
    let end_label = ctx.next_label();
    instructions.push(Instruction::Label(loop_label));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: end_label });

    instructions.push(Instruction::LoadLocal(arr_local));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::ArrayGet);
    let emitted_slots = emit_abi_static_slots_for_stack_value(element_type, ctx, instructions)?;
    if emitted_slots != element_slot_count {
        return None;
    }
    if emitted_slots > 1 {
        instructions.push(Instruction::CallBuiltin {
            builtin: BuiltinCall::BytesConcat,
            arg_count: emitted_slots,
        });
    }
    instructions.push(Instruction::StoreLocal(elem_slot_local));

    instructions.push(Instruction::LoadLocal(out_local));
    if include_length {
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
            32u8,
        ))));
    } else {
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    }
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        element_byte_len as u64,
    ))));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::LoadLocal(elem_slot_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        element_byte_len as u64,
    ))));
    instructions.push(Instruction::MemCpy);

    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(idx_local));
    instructions.push(Instruction::Jump { target: loop_label });
    instructions.push(Instruction::Label(end_label));

    instructions.push(Instruction::LoadLocal(out_local));
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
    Some(())
}

fn emit_abi_decode_static_slot(
    buffer_local: usize,
    index: usize,
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    match value_type {
        ValueType::Integer { .. } | ValueType::Boolean => {
            emit_abi_decode_slot_slice(buffer_local, index, 0, 32, instructions);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            materialize_byte_array_buffer(ctx, instructions, true);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::Integer,
            });
        }
        ValueType::Address => {
            emit_abi_decode_slot_slice(buffer_local, index, 12, 20, instructions);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            materialize_byte_array_buffer(ctx, instructions, true);
        }
        ValueType::ByteArray {
            fixed_len: Some(len),
        } => {
            let len = (*len).min(32) as usize;
            emit_abi_decode_slot_slice(buffer_local, index, 0, len, instructions);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
        }
        _ => {
            emit_abi_decode_slot_slice(buffer_local, index, 0, 32, instructions);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
        }
    }
}

fn emit_abi_decode_slot_slice(
    buffer_local: usize,
    index: usize,
    slot_offset: usize,
    len: usize,
    instructions: &mut Vec<Instruction>,
) {
    instructions.push(Instruction::LoadLocal(buffer_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        (index * 32 + slot_offset) as u64,
    ))));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        len as u64,
    ))));
    instructions.push(Instruction::Substr);
}

fn emit_abi_u256_slot(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    let _ = emit_expr_static_abi_slot_for_value_type(
        &ValueType::Integer {
            signed: false,
            bits: 256,
        },
        ctx,
        instructions,
    );
}

fn emit_abi_fixed_buffer(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    len: usize,
    reverse: bool,
) {
    let tmp_id = ctx.next_label();
    let src_local = ctx.allocate_local(format!("__abi_fixed_src_{tmp_id}"), None);
    let dst_local = ctx.allocate_local(format!("__abi_fixed_dst_{tmp_id}"), None);
    let size_local = ctx.allocate_local(format!("__abi_fixed_size_{tmp_id}"), None);
    let count_local = ctx.allocate_local(format!("__abi_fixed_count_{tmp_id}"), None);

    instructions.push(Instruction::StoreLocal(src_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(len as u64),
    )));
    instructions.push(Instruction::NewBuffer);
    instructions.push(Instruction::StoreLocal(dst_local));

    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(size_local));

    let ge_label = ctx.next_label();
    let end_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(len as u64),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: ge_label });
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::StoreLocal(count_local));
    instructions.push(Instruction::Jump { target: end_label });
    instructions.push(Instruction::Label(ge_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(len as u64),
    )));
    instructions.push(Instruction::StoreLocal(count_local));
    instructions.push(Instruction::Label(end_label));

    instructions.push(Instruction::LoadLocal(dst_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::MemCpy);

    if reverse {
        instructions.push(Instruction::LoadLocal(dst_local));
        instructions.push(Instruction::LoadLocal(dst_local));
        instructions.push(Instruction::ReverseItems);
    } else {
        instructions.push(Instruction::LoadLocal(dst_local));
    }
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
}

fn emit_abi_bytesn_slot(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    len: usize,
) {
    let tmp_id = ctx.next_label();
    let src_local = ctx.allocate_local(format!("__abi_bytesn_src_{tmp_id}"), None);
    let dst_local = ctx.allocate_local(format!("__abi_bytesn_dst_{tmp_id}"), None);
    let size_local = ctx.allocate_local(format!("__abi_bytesn_size_{tmp_id}"), None);
    let count_local = ctx.allocate_local(format!("__abi_bytesn_count_{tmp_id}"), None);

    instructions.push(Instruction::StoreLocal(src_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u64),
    )));
    instructions.push(Instruction::NewBuffer);
    instructions.push(Instruction::StoreLocal(dst_local));

    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(size_local));

    let ge_label = ctx.next_label();
    let end_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(len as u64),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: ge_label });
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::StoreLocal(count_local));
    instructions.push(Instruction::Jump { target: end_label });
    instructions.push(Instruction::Label(ge_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(len as u64),
    )));
    instructions.push(Instruction::StoreLocal(count_local));
    instructions.push(Instruction::Label(end_label));

    instructions.push(Instruction::LoadLocal(dst_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::MemCpy);
    instructions.push(Instruction::LoadLocal(dst_local));
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
}
