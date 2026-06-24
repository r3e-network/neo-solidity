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

