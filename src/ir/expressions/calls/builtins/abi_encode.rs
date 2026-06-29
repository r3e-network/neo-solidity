use super::*;

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
pub(crate) fn try_flatten_struct_arg_for_abi_encode(
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

pub(crate) fn lower_abi_encode_args_direct_from_slice(
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    let refs: Vec<&Expression> = args.iter().collect();
    lower_abi_encode_args_direct(&refs, ctx, instructions)
}

pub(crate) fn lower_abi_encode_args_direct(
    args: &[&Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    lower_abi_encode_args_direct_impl(args, ctx, instructions, false)
}

pub(crate) fn lower_abi_encode_args_direct_for_encode_call(
    args: &[&Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    lower_abi_encode_args_direct_impl(args, ctx, instructions, true)
}

pub(crate) fn lower_abi_encode_args_direct_impl(
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

pub(crate) fn lower_abi_encode_head_tail_direct(
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
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from((head_slot_count * 32) as u64),
    )));
    instructions.push(Instruction::StoreLocal(offset_local));

    let mut tail_locals = Vec::new();
    let mut part_count = 0usize;

    for (local, value_type) in value_locals.iter().zip(arg_types.iter()) {
        if abi_value_type_is_dynamic(value_type) {
            instructions.push(Instruction::LoadLocal(offset_local));
            emit_abi_u256_slot(ctx, instructions);
            part_count += 1;

            instructions.push(Instruction::LoadLocal(*local));
            emit_abi_dynamic_tail_for_value_type(value_type, 0, ctx, instructions)?;
            let tail_local = ctx.allocate_local("__abi_tail".to_string(), None);
            instructions.push(Instruction::StoreLocal(tail_local));

            instructions.push(Instruction::LoadLocal(offset_local));
            instructions.push(Instruction::LoadLocal(tail_local));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(offset_local));
            tail_locals.push(tail_local);
        } else {
            let slots = emit_abi_static_slots_from_local(*local, value_type, ctx, instructions)?;
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

pub(crate) fn lower_abi_encode_packed_args_direct_from_slice(
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

pub(crate) fn lower_static_abi_slots_for_expr(
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
                if let Some((param_slot, field_count)) = resolve_struct_param_flat_slots(expr, ctx) {
                    if field_count != fields.len() {
                        return None;
                    }
                    // The struct param arrives as a single `Array` (one arg
                    // slot); read field `index` by indexing into it (PICKITEM).
                    for (index, field) in fields.iter().enumerate() {
                        instructions.push(Instruction::LoadParameter(param_slot));
                        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                            BigInt::from(index),
                        )));
                        instructions.push(Instruction::ArrayGet);
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
            // A `bytesN` argument that is INTEGER-backed (a hex literal or a
            // named `bytesN` constant) is pushed little-endian / pre-reversed,
            // which the slot encoder would emit byte-reversed (N==32) or fault on
            // (N<32 via `GetSize`). Resolve it to its big-endian bytes and emit a
            // left-aligned 32-byte slot directly. ByteArray-backed values
            // (keccak/cast/storage/param) keep the normal (correct) path.
            if let ValueType::ByteArray {
                fixed_len: Some(n),
            } = other
            {
                if is_integer_backed_bytesn_operand(expr, ctx) {
                    if let Some(be) = fixed_len_bytes_be_from_hex_or_const(expr, n, ctx) {
                        let mut slot = be; // exactly `n` big-endian bytes
                        slot.resize(32, 0); // left-aligned, zero-padded to the 32-byte slot
                        instructions
                            .push(Instruction::PushLiteral(LiteralValue::ByteArray(slot)));
                        return Some(1);
                    }
                }
            }
            if !lower_expression(expr, ctx, instructions) {
                return Some(0);
            }
            emit_expr_static_abi_slot_for_value_type(&other, ctx, instructions)?;
            Some(1)
        }
        _ => None,
    }
}

pub(crate) fn emit_expr_static_abi_slot_for_value_type(
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<()> {
    match value_type {
        // Bug #23: signed integers must be sign-extended (0xff..) when negative,
        // not zero-padded. Route signed-true through the sign-aware buffer.
        ValueType::Integer { signed: true, .. } => {
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            emit_abi_fixed_buffer_signed(ctx, instructions, 32);
            Some(())
        }
        ValueType::Integer { signed: false, .. } | ValueType::Boolean | ValueType::Address => {
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
            // KNOWN LIMITATION (deep-review #4): a `bytesN` operand may be
            // ByteArray-backed (already big-endian: keccak/sha output, a
            // `bytesN(..)` cast, storage, a param) OR Integer-backed (a hex
            // literal/constant, pushed pre-reversed). The two need OPPOSITE
            // handling — Integer-backed must be reversed to big-endian, the
            // ByteArray-backed must not — but this type-only context cannot
            // tell them apart, so neither a blanket reverse nor a blanket
            // pass-through is correct. A proper fix needs the lowered value's
            // backing tracked (or bytesN literals normalized to a ByteArray at
            // the source). Until then the common keccak/cast/param cases are
            // correct; Integer-backed bytesN literals encode wrong (N==32) or
            // fault on `GetSize` (N<32).
            emit_abi_bytesn_slot(ctx, instructions, *len as usize);
            Some(())
        }
        _ => None,
    }
}

pub(crate) fn lower_packed_abi_bytes_for_expr(
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
        // Bug #23 (packed variant): negative signed integers must be
        // SIGN-EXTENDED (0xff fill) to their declared width, not zero-padded
        // — `abi.encodePacked(int16(-1))` is 0xffff, not 0x00ff. Route signed
        // through the sign-aware buffer at the packed width.
        ValueType::Integer { bits, signed: true } => {
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            emit_abi_fixed_buffer_signed(ctx, instructions, (bits / 8) as usize);
            Some(true)
        }
        ValueType::Integer {
            bits,
            signed: false,
        } => {
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

pub(crate) fn emit_abi_static_slots_from_local(
    local: usize,
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<usize> {
    match value_type {
        ValueType::Struct { fields, .. }
            if !fields.is_empty()
                && fields
                    .iter()
                    .all(|field| is_static_abi_type_value(&field.ty)) =>
        {
            for (index, field) in fields.iter().enumerate() {
                instructions.push(Instruction::LoadLocal(local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(index as u64),
                )));
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

pub(crate) fn emit_abi_encode_single_stack_value_for_type(
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
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::from(32u8),
        )));
        emit_abi_u256_slot(ctx, instructions);
        instructions.push(Instruction::LoadLocal(value_local));
        emit_abi_dynamic_tail_for_value_type(value_type, 0, ctx, instructions)?;
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

pub(crate) fn emit_abi_dynamic_tail_for_value_type(
    value_type: &ValueType,
    depth: usize,
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
        // `T[]` where each element is itself dynamic (string[], bytes[],
        // uint256[][], ...). The element offsets form a head section and the
        // element encodings a tail section, recursively — full EVM layout.
        ValueType::Array(element_type) if abi_dynamic_value_type_is_supported(element_type) => {
            emit_abi_dynamic_nested_array_tail(element_type, depth, ctx, instructions)
        }
        // Dynamic struct → encode as a tuple of its fields (head+tail).
        ValueType::Struct { fields, .. } if abi_dynamic_value_type_is_supported(value_type) => {
            emit_abi_dynamic_struct_tail(fields, depth, ctx, instructions)
        }
        _ => None,
    }
}

pub(crate) fn emit_abi_dynamic_bytes_tail(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    let tmp_id = ctx.next_label();
    let src_local = ctx.allocate_local(format!("__abi_dyn_bytes_src_{tmp_id}"), None);
    let len_local = ctx.allocate_local(format!("__abi_dyn_bytes_len_{tmp_id}"), None);
    let padded_len_local = ctx.allocate_local(format!("__abi_dyn_bytes_padded_len_{tmp_id}"), None);
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
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(31u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Div));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
    instructions.push(Instruction::StoreLocal(padded_len_local));

    instructions.push(Instruction::LoadLocal(padded_len_local));
    instructions.push(Instruction::NewBuffer);
    instructions.push(Instruction::StoreLocal(padded_local));

    instructions.push(Instruction::LoadLocal(padded_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
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

pub(crate) fn emit_abi_dynamic_static_array_tail(
    element_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<()> {
    emit_abi_static_array_buffer(element_type, ctx, instructions, true)
}

/// Emit the ABI tail for a `T[]` whose element type `T` is itself dynamic
/// (e.g. `string[]`, `bytes[]`, `uint256[][]`). Consumes the array value from
/// the top of the stack and leaves the encoded tail (a `ByteString`) in its
/// place. Layout (all offsets relative to the start of the head section, i.e.
/// the slot immediately after the length word — standard EVM ABI):
///
/// ```text
///   [ length n ]                                  # 32-byte word
///   [ off_0 ][ off_1 ] ... [ off_{n-1} ]          # head: n × 32-byte words
///   [ tail_0 ][ tail_1 ] ... [ tail_{n-1} ]       # tail: element encodings
/// ```
///
/// where `off_i = n*32 + Σ_{j<i} len(tail_j)` and each `tail_i` is produced by
/// recursively encoding element `i` via [`emit_abi_dynamic_tail_for_value_type`].
pub(crate) fn emit_abi_dynamic_nested_array_tail(
    element_type: &ValueType,
    depth: usize,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<()> {
    // Seven reusable scratch locals for this nesting depth. Distinct depths
    // never collide (the inner element is encoded while these are live); call
    // sites at the same depth share the block.
    let scratch = ctx.abi_nested_scratch_locals(depth, 7);
    let arr_local = scratch[0];
    let n_local = scratch[1];
    let off_local = scratch[2];
    let heads_local = scratch[3];
    let tails_local = scratch[4];
    let idx_local = scratch[5];
    let et_local = scratch[6];

    // arr := top of stack; n := arr.length
    instructions.push(Instruction::StoreLocal(arr_local));
    instructions.push(Instruction::LoadLocal(arr_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(n_local));

    // off := n * 32  (head section size; first element tail begins here)
    instructions.push(Instruction::LoadLocal(n_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
    instructions.push(Instruction::StoreLocal(off_local));

    // heads := "" ; tails := ""
    instructions.push(Instruction::PushLiteral(
        LiteralValue::ByteArray(Vec::new()),
    ));
    instructions.push(Instruction::StoreLocal(heads_local));
    instructions.push(Instruction::PushLiteral(
        LiteralValue::ByteArray(Vec::new()),
    ));
    instructions.push(Instruction::StoreLocal(tails_local));

    // idx := 0
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::StoreLocal(idx_local));

    let loop_label = ctx.next_label();
    let end_label = ctx.next_label();
    instructions.push(Instruction::Label(loop_label));
    // while idx < n  (JumpIf -> JMPIFNOT exits when the condition is false)
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::LoadLocal(n_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: end_label });

    // et := encode(arr[idx])  (recursive element tail)
    instructions.push(Instruction::LoadLocal(arr_local));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::ArrayGet);
    emit_abi_dynamic_tail_for_value_type(element_type, depth + 1, ctx, instructions)?;
    instructions.push(Instruction::StoreLocal(et_local));

    // heads := heads ++ u256_slot(off)
    instructions.push(Instruction::LoadLocal(heads_local));
    instructions.push(Instruction::LoadLocal(off_local));
    emit_abi_u256_slot(ctx, instructions);
    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::BytesConcat,
        arg_count: 2,
    });
    instructions.push(Instruction::StoreLocal(heads_local));

    // off := off + len(et)
    instructions.push(Instruction::LoadLocal(off_local));
    instructions.push(Instruction::LoadLocal(et_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(off_local));

    // tails := tails ++ et
    instructions.push(Instruction::LoadLocal(tails_local));
    instructions.push(Instruction::LoadLocal(et_local));
    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::BytesConcat,
        arg_count: 2,
    });
    instructions.push(Instruction::StoreLocal(tails_local));

    // idx := idx + 1
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::one(),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(idx_local));
    instructions.push(Instruction::Jump { target: loop_label });
    instructions.push(Instruction::Label(end_label));

    // result := u256_slot(n) ++ heads ++ tails
    instructions.push(Instruction::LoadLocal(n_local));
    emit_abi_u256_slot(ctx, instructions);
    instructions.push(Instruction::LoadLocal(heads_local));
    instructions.push(Instruction::LoadLocal(tails_local));
    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::BytesConcat,
        arg_count: 3,
    });
    Some(())
}

/// Emit the ABI tail for a DYNAMIC struct (one with ≥1 dynamic field).
/// Consumes the struct value (a NeoVM `Array` of field values) from the top
/// of the stack and leaves its tuple encoding (a `ByteString`). The layout
/// is the standard EVM tuple: a head section (static fields inline, dynamic
/// fields as 32-byte offsets relative to the start of the head) followed by
/// the dynamic-field tails — identical to `lower_abi_encode_head_tail_direct`
/// but reading fields from the struct array rather than argument expressions.
pub(crate) fn emit_abi_dynamic_struct_tail(
    fields: &[StructField],
    depth: usize,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<()> {
    // Head size in 32-byte words: dynamic fields take one offset word; static
    // fields occupy their inline slot count.
    let mut head_slot_count = 0usize;
    for field in fields {
        if abi_value_type_is_dynamic(&field.ty) {
            if !abi_dynamic_value_type_is_supported(&field.ty) {
                return None;
            }
            head_slot_count += 1;
        } else if let Some(slots) = abi_static_slot_count(&field.ty) {
            head_slot_count += slots;
        } else {
            return None;
        }
    }

    let tmp_id = ctx.next_label();
    let struct_local = ctx.allocate_local(format!("__abi_dstruct_{tmp_id}"), None);
    instructions.push(Instruction::StoreLocal(struct_local));

    let offset_local = ctx.allocate_local(format!("__abi_dstruct_off_{tmp_id}"), None);
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from((head_slot_count * 32) as u64),
    )));
    instructions.push(Instruction::StoreLocal(offset_local));

    let mut tail_locals = Vec::new();
    let mut part_count = 0usize;

    for (field_index, field) in fields.iter().enumerate() {
        if abi_value_type_is_dynamic(&field.ty) {
            // Head: the offset word pointing at this field's tail.
            instructions.push(Instruction::LoadLocal(offset_local));
            emit_abi_u256_slot(ctx, instructions);
            part_count += 1;

            // Compute the field tail and stash it for the tail section.
            instructions.push(Instruction::LoadLocal(struct_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(field_index as u64),
            )));
            instructions.push(Instruction::ArrayGet);
            emit_abi_dynamic_tail_for_value_type(&field.ty, depth + 1, ctx, instructions)?;
            let tail_local =
                ctx.allocate_local(format!("__abi_dstruct_tail_{tmp_id}_{field_index}"), None);
            instructions.push(Instruction::StoreLocal(tail_local));

            // offset += len(tail)
            instructions.push(Instruction::LoadLocal(offset_local));
            instructions.push(Instruction::LoadLocal(tail_local));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(offset_local));
            tail_locals.push(tail_local);
        } else {
            // Static field: inline its slot(s) into the head section.
            instructions.push(Instruction::LoadLocal(struct_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(field_index as u64),
            )));
            instructions.push(Instruction::ArrayGet);
            let slots = emit_abi_static_slots_for_stack_value(&field.ty, ctx, instructions)?;
            part_count += slots;
        }
    }

    for tail_local in tail_locals {
        instructions.push(Instruction::LoadLocal(tail_local));
        part_count += 1;
    }

    if part_count == 0 {
        return None;
    }
    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::BytesConcat,
        arg_count: part_count,
    });
    Some(())
}

pub(crate) fn emit_abi_packed_static_array(
    element_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<()> {
    emit_abi_static_array_buffer(element_type, ctx, instructions, false)
}

pub(crate) fn emit_abi_static_slots_for_stack_value(
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<usize> {
    match value_type {
        ValueType::Struct { fields, .. }
            if !fields.is_empty()
                && fields
                    .iter()
                    .all(|field| is_static_abi_type_value(&field.ty)) =>
        {
            let tmp_id = ctx.next_label();
            let struct_local = ctx.allocate_local(format!("__abi_stack_struct_{tmp_id}"), None);
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
        other if is_static_abi_type_value(other) => {
            emit_expr_static_abi_slot_for_value_type(other, ctx, instructions)?;
            Some(1)
        }
        _ => None,
    }
}

pub(crate) fn emit_abi_static_array_buffer(
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
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(element_byte_len as u64),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
    if include_length {
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::from(32u8),
        )));
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
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::zero(),
        )));
        instructions.push(Instruction::LoadLocal(len_slot_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::zero(),
        )));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::from(32u8),
        )));
        instructions.push(Instruction::MemCpy);
    }

    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
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
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::from(32u8),
        )));
    } else {
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::zero(),
        )));
    }
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(element_byte_len as u64),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::LoadLocal(elem_slot_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(element_byte_len as u64),
    )));
    instructions.push(Instruction::MemCpy);

    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::one(),
    )));
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
