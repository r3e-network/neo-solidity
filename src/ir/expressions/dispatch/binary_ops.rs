fn try_lower_expression_binary_ops(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    match expr {
        Expression::Add(_, left, right) => Some(lower_binary_expr(
            left,
            right,
            ctx,
            instructions,
            BinaryOperator::Add,
        )),
        Expression::Subtract(_, left, right) => Some(lower_binary_expr(
            left,
            right,
            ctx,
            instructions,
            BinaryOperator::Sub,
        )),
        Expression::Multiply(_, left, right) => Some(lower_binary_expr(
            left,
            right,
            ctx,
            instructions,
            BinaryOperator::Mul,
        )),
        Expression::Divide(_, left, right) => Some(lower_binary_expr(
            left,
            right,
            ctx,
            instructions,
            BinaryOperator::Div,
        )),
        Expression::Modulo(_, left, right) => Some(lower_binary_expr(
            left,
            right,
            ctx,
            instructions,
            BinaryOperator::Mod,
        )),
        Expression::ShiftLeft(_, left, right) => Some(lower_binary_expr(
            left,
            right,
            ctx,
            instructions,
            BinaryOperator::Shl,
        )),
        Expression::ShiftRight(_, left, right) => Some(lower_binary_expr(
            left,
            right,
            ctx,
            instructions,
            BinaryOperator::Shr,
        )),
        Expression::BitwiseAnd(_, left, right) => Some(lower_binary_expr(
            left,
            right,
            ctx,
            instructions,
            BinaryOperator::BitAnd,
        )),
        Expression::BitwiseOr(_, left, right) => Some(lower_binary_expr(
            left,
            right,
            ctx,
            instructions,
            BinaryOperator::BitOr,
        )),
        Expression::BitwiseXor(_, left, right) => Some(lower_binary_expr(
            left,
            right,
            ctx,
            instructions,
            BinaryOperator::BitXor,
        )),
        _ => None,
    }
}
