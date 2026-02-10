// Expression lowering.
fn lower_expression(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    if let Some(result) = try_lower_expression_binary_ops(expr, ctx, instructions) {
        return result;
    }
    if let Some(result) = try_lower_expression_assignments(expr, ctx, instructions) {
        return result;
    }
    if let Some(result) = try_lower_expression_unary(expr, ctx, instructions) {
        return result;
    }
    if let Some(result) = try_lower_expression_comparisons(expr, ctx, instructions) {
        return result;
    }
    if let Some(result) = try_lower_expression_primary(expr, ctx, instructions) {
        return result;
    }
    if let Some(result) = try_lower_expression_calls(expr, ctx, instructions) {
        return result;
    }
    if let Some(result) = try_lower_expression_tuple(expr, ctx, instructions) {
        return result;
    }
    if let Some(result) = try_lower_expression_conditional(expr, ctx, instructions) {
        return result;
    }

    if let Some(literal) = literal_from_expression(expr) {
        // Warn when Solidity ether units are used -- the numeric conversion is
        // technically correct (wei=1, gwei=10^9, ether=10^18) but semantically
        // misleading on Neo N3 where the native token is GAS with 10^8 decimals.
        if has_ether_unit(expr) {
            ctx.record_error_with_suggestion(
                "ether units (wei/gwei/ether) are not applicable on Neo N3",
                "use GAS token with 10^8 decimals (1 GAS = 100_000_000 fractions)",
            );
        }
        instructions.push(Instruction::PushLiteral(literal));
        true
    } else {
        ctx.record_error_with_suggestion(
            format!("unsupported expression '{:?}'", expr),
            "Neo N3 supports: int, string, bytes, bool, address, arrays, maps, structs",
        );
        false
    }
}
