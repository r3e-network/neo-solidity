fn try_lower_expression_unary(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    match expr {
        Expression::Not(_, inner) => Some(if lower_expression(inner, ctx, instructions) {
            instructions.push(Instruction::LogicalNot);
            true
        } else {
            false
        }),
        Expression::BitwiseNot(_, inner) => Some(if lower_expression(inner, ctx, instructions) {
            instructions.push(Instruction::BitwiseNot);
            true
        } else {
            false
        }),
        Expression::Power(_, left, right) => Some(lower_power_expression(
            left.as_ref(),
            right.as_ref(),
            ctx,
            instructions,
        )),
        Expression::UnaryPlus(_, inner) => Some(lower_expression(inner, ctx, instructions)),
        Expression::Negate(_, inner) => Some(if lower_expression(inner, ctx, instructions) {
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(-1),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
            true
        } else {
            false
        }),
        _ => None,
    }
}
