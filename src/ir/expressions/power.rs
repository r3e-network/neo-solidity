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
            // Constant-fold `base ** exp` at compile time, but cap the
            // iteration count so a pathological source like
            // `uint x = 2 ** 4294967295;` doesn't spin the compiler in
            // a 4-billion-iter BigInt loop. Real Solidity literals that
            // fit in a uint256 slot have `exp < 256` for `base >= 2`, so
            // any legal use is well under the cap. Above the cap we
            // fall through to the runtime-loop lowering below, which is
            // bounded by the executing contract's gas budget rather than
            // the compiler's memory.
            const MAX_LITERAL_POW_EXP: u32 = 1024;
            if exp <= MAX_LITERAL_POW_EXP {
                let mut result = BigInt::one();
                for _ in 0..exp {
                    result *= &base;
                }
                instructions
                    .push(Instruction::PushLiteral(LiteralValue::Integer(result)));
                return true;
            }
            // exp > MAX_LITERAL_POW_EXP: fall through to the runtime
            // exponentiation loop below. The compiler stays bounded.
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

    // Solidity `**` is a CHECKED operation: the result must fit the base type
    // (the result type of `a ** b` is the type of `a`). The square-and-multiply
    // loop above computes the un-truncated BigInt, so apply the same width-aware
    // overflow handling as Add/Sub/Mul: Panic(0x11) on overflow in checked mode,
    // wrap mod 2^N in `unchecked`. Narrow widths (N<256) are the silent-overflow
    // case — `uint8 a = 2; a ** 8` yields 256, which fits NeoVM's 256-bit integer
    // but overflows uint8. (uint256/int256 powers that exceed 256 bits already
    // fault on NeoVM's integer-size limit inside the loop, so they are not
    // silently wrong.) Constant-folded literal powers are handled above.
    if let Some(ValueType::Integer { signed, bits }) = infer_type_from_expression(left, ctx) {
        if matches!(bits, 8 | 16 | 32 | 64 | 128) {
            let bits_usize = bits as usize;
            if ctx.in_unchecked_block() {
                instructions.push(Instruction::LoadLocal(result_local));
                if signed {
                    emit_truncate_narrow_signed(ctx, instructions, bits);
                } else {
                    emit_truncate_narrow_unsigned(instructions, bits);
                }
                instructions.push(Instruction::StoreLocal(result_local));
            } else if signed {
                // checked intN: [-2^(bits-1), 2^(bits-1) - 1].
                let int_max = (BigInt::one() << (bits_usize - 1)) - BigInt::one();
                let int_min = -(BigInt::one() << (bits_usize - 1));
                let after_max = ctx.next_label();
                instructions.push(Instruction::LoadLocal(result_local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(int_max)));
                instructions.push(Instruction::BinaryOp(BinaryOperator::Gt));
                instructions.push(Instruction::JumpIf { target: after_max });
                emit_panic(0x11, instructions);
                instructions.push(Instruction::Label(after_max));
                let after_min = ctx.next_label();
                instructions.push(Instruction::LoadLocal(result_local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(int_min)));
                instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
                instructions.push(Instruction::JumpIf { target: after_min });
                emit_panic(0x11, instructions);
                instructions.push(Instruction::Label(after_min));
            } else {
                // checked uintN: [0, 2^bits - 1]. A power of a non-negative base
                // is always >= 0, so only the upper bound can be violated.
                let uint_max = (BigInt::one() << bits_usize) - BigInt::one();
                let after_max = ctx.next_label();
                instructions.push(Instruction::LoadLocal(result_local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(uint_max)));
                instructions.push(Instruction::BinaryOp(BinaryOperator::Gt));
                instructions.push(Instruction::JumpIf { target: after_max });
                emit_panic(0x11, instructions);
                instructions.push(Instruction::Label(after_max));
            }
        }
    }

    instructions.push(Instruction::LoadLocal(result_local));
    true
}
