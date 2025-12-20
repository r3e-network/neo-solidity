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
        instructions.push(Instruction::PushLiteral(literal));
        true
    } else {
        ctx.record_error(format!("unsupported expression '{:?}'", expr));
        false
    }
}
