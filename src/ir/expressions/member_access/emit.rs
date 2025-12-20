fn lower_member_access_expression(
    expr: &Expression,
    inner: &Expression,
    member: &Identifier,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    if let Some(result) = try_lower_native_contract_constant(inner, member, instructions) {
        return result;
    }

    if let Some(result) = try_lower_selector_member_access(inner, member, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_runtime_member_access(inner, member, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_address_balance(inner, member, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_length_property(inner, member, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_current_key(inner, member, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_code_property(inner, member, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_type_bound_max(inner, member, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_type_bound_min(inner, member, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_interface_id(inner, member, ctx, instructions) {
        return result;
    }

    lower_generic_member_access(expr, inner, member, ctx, instructions)
}
