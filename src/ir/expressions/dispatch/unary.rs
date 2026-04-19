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
        Expression::Negate(_, inner) => Some(lower_negate_expression(inner, ctx, instructions)),
        _ => None,
    }
}

/// Task #30 slice 2: unary-minus lowering with a `-type(intN).min` guard for
/// signed-integer operands. For `intN`, negating the min value produces
/// `2^(N-1)`, which is not representable — Solidity 0.8.x must revert with
/// Panic(0x11).
///
/// The guard emits:
/// 1. Evaluate the inner expression.
/// 2. If the operand is signed intN and not inside `unchecked { }` and not a
///    literal number, emit:
///      DUP
///      Push(type(intN).min)
///      Eq
///      JumpIf (branch on FALSE — skip THROW when condition FALSE (safe))
///      emit_panic(0x11)  // canonical EVM Panic(uint256) envelope
///    leaving the original operand on the stack for the subsequent negation.
/// 3. Multiply by -1 to compute the negation.
fn lower_negate_expression(
    inner: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    // Check whether to emit the guard before lowering (since inference needs
    // the pre-lowered AST).
    let emit_guard = should_emit_negate_guard(inner, ctx);
    let intn_min = signed_intn_min_literal(inner, ctx);

    if !lower_expression(inner, ctx, instructions) {
        return false;
    }

    if emit_guard {
        if let Some(min_value) = intn_min {
            // DUP the operand so the post-check can compare it without
            // consuming the value (we still need it for the Mul below).
            instructions.push(Instruction::Dup);
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(min_value)));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            // JumpIf-on-false: safe path; if operand != intN::min, skip the
            // THROW. Fall through to THROW when operand == intN::min.
            // Task #107 — canonical EVM Panic(uint256) envelope so
            // `catch Panic(uint code)` can bind code = 0x11.
            let safe_label = ctx.next_label();
            instructions.push(Instruction::JumpIf { target: safe_label });
            emit_panic(0x11, instructions);
            instructions.push(Instruction::Label(safe_label));
        }
    }

    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(-1),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
    true
}

/// Gate for Task #30 slice 2 unary-minus guard emission.
fn should_emit_negate_guard(inner: &Expression, ctx: &LoweringContext) -> bool {
    if ctx.in_unchecked_block() {
        return false;
    }
    if is_literal_number(inner) {
        return false;
    }
    matches!(
        infer_type_from_expression(inner, ctx),
        Some(ValueType::Integer { signed: true, .. })
    )
}

/// Returns the `type(intN).min` literal value for the inferred signed-int
/// type of the operand, or `None` for unsigned / unknown types.
fn signed_intn_min_literal(inner: &Expression, ctx: &LoweringContext) -> Option<BigInt> {
    if let Some(ValueType::Integer {
        signed: true,
        bits,
    }) = infer_type_from_expression(inner, ctx)
    {
        // intN::min = -2^(N-1).
        let one: BigInt = BigInt::from(1);
        let shifted: BigInt = one << (bits as u32 - 1);
        Some(-shifted)
    } else {
        None
    }
}
