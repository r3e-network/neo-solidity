fn lower_power_expression(
    left: &Expression,
    right: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    if let (Some(LiteralValue::Integer(base)), Some(LiteralValue::Integer(exp_lit))) = (
        literal_from_expression(left),
        literal_from_expression(right),
    ) {
        if let Some(exp) = exp_lit.to_u32() {
            let mut result = BigInt::one();
            for _ in 0..exp {
                result *= &base;
            }
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(result)));
            return true;
        }
    }

    let base_local = ctx.allocate_local("__pow_base".to_string(), None);
    let exp_local = ctx.allocate_local("__pow_exp".to_string(), None);
    let result_local = ctx.allocate_local("__pow_result".to_string(), None);

    if !lower_expression(left, ctx, instructions) {
        return false;
    }
    instructions.push(Instruction::StoreLocal(base_local));

    if !lower_expression(right, ctx, instructions) {
        return false;
    }
    instructions.push(Instruction::StoreLocal(exp_local));

    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
    instructions.push(Instruction::StoreLocal(result_local));

    let loop_label = ctx.next_label();
    let end_label = ctx.next_label();
    let skip_mul_label = ctx.next_label();

    instructions.push(Instruction::Label(loop_label));
    instructions.push(Instruction::LoadLocal(exp_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Ne));
    instructions.push(Instruction::JumpIf { target: end_label });

    instructions.push(Instruction::LoadLocal(exp_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::BitAnd));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Ne));
    // JumpIf branches when the condition is false; skip multiply when exp is even.
    instructions.push(Instruction::JumpIf {
        target: skip_mul_label,
    });
    instructions.push(Instruction::LoadLocal(result_local));
    instructions.push(Instruction::LoadLocal(base_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
    instructions.push(Instruction::StoreLocal(result_local));

    instructions.push(Instruction::Label(skip_mul_label));
    instructions.push(Instruction::LoadLocal(base_local));
    instructions.push(Instruction::LoadLocal(base_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
    instructions.push(Instruction::StoreLocal(base_local));

    instructions.push(Instruction::LoadLocal(exp_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Shr));
    instructions.push(Instruction::StoreLocal(exp_local));

    instructions.push(Instruction::Jump { target: loop_label });
    instructions.push(Instruction::Label(end_label));
    instructions.push(Instruction::LoadLocal(result_local));
    true
}
