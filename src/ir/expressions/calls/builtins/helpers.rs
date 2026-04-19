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
        ValueType::String => Some(matches!(expected, ManifestType::String | ManifestType::ByteArray)),
        ValueType::Address => Some(matches!(expected, ManifestType::Hash160 | ManifestType::ByteArray)),
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

    if base.name == "abi" && (member.name == "encode" || member.name == "encodePacked" || member.name == "encodeCall") {
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
