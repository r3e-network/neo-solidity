pub(crate) fn try_lower_expression_primary(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    match expr {
        Expression::Variable(identifier) => Some(lower_variable_expression(identifier, ctx, instructions)),
        Expression::ArraySubscript(_, _, None) => {
            instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(Vec::new())));
            Some(true)
        }
        Expression::ArraySubscript(_, array, Some(index)) => Some(
            lower_array_subscript_expression(expr, array.as_ref(), index.as_ref(), ctx, instructions),
        ),
        Expression::ArraySlice(_, array, start, end) => {
            Some(lower_array_slice_expression(
                array.as_ref(),
                start.as_deref(),
                end.as_deref(),
                ctx,
                instructions,
            ))
        }
        Expression::ArrayLiteral(_, elements) => {
            Some(lower_array_literal_expression(elements, ctx, instructions))
        }
        Expression::Or(_, left, right) => Some(lower_logical_or(left, right, ctx, instructions)),
        Expression::And(_, left, right) => Some(lower_logical_and(left, right, ctx, instructions)),
        _ => None,
    }
}
