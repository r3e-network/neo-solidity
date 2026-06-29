use super::*;

pub(crate) fn lower_power_expression(
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
            // the compiler's memory. Reuses `MAX_DECIMAL_EXPONENT` from
            // `ir::ir_build` (the re-export of `build/literals.rs`) so
            // the two exponent limits stay in lockstep — a single source
            // of truth.
            if exp <= crate::ir::ir_build::MAX_DECIMAL_EXPONENT {
                let mut result = BigInt::one();
                for _ in 0..exp {
                    result *= &base;
                }
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(result)));
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

    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::one(),
    )));
    instructions.push(Instruction::StoreLocal(result_local));

    let loop_label = ctx.next_label();
    let end_label = ctx.next_label();
    let skip_mul_label = ctx.next_label();

    instructions.push(Instruction::Label(loop_label));
    instructions.push(Instruction::LoadLocal(exp_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Ne));
    instructions.push(Instruction::JumpIf { target: end_label });

    instructions.push(Instruction::LoadLocal(exp_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::one(),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::BitAnd));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
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
    // Square `base` ONLY while more exponent bits remain (current exp > 1).
    // The final iteration (exp == 1) does its `result *= base` above and then
    // must NOT square again: that wasted squaring can overshoot NeoVM's 32-byte
    // integer limit and FAULT on-chain even when the final result is in range
    // (e.g. `2 ** 200` — result 2^200 fits, but `(2^128)^2 = 2^256` would
    // fault). The simulator's arbitrary-precision BigInt masked this.
    let after_square_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(exp_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::one(),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Gt));
    // JumpIf branches when FALSE — skip the squaring when exp <= 1.
    instructions.push(Instruction::JumpIf {
        target: after_square_label,
    });
    instructions.push(Instruction::LoadLocal(base_local));
    instructions.push(Instruction::LoadLocal(base_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
    instructions.push(Instruction::StoreLocal(base_local));
    instructions.push(Instruction::Label(after_square_label));

    instructions.push(Instruction::LoadLocal(exp_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::one(),
    )));
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
        if bits == 256 && !signed {
            // uint256 `**`: the square-and-multiply loop above leaves the
            // un-truncated value. `unchecked` wraps mod 2^256; checked Panics
            // 0x11 when the true result exceeds 2^256-1. Either way, canonicalize
            // to the 32-byte two's-complement form via `emit_truncate_u256`.
            if !ctx.in_unchecked_block() {
                let ok = ctx.next_label();
                instructions.push(Instruction::LoadLocal(result_local));
                // Overflow check: `result >= 2^256` iff `(result >> 255) >= 2`.
                // (2^255 >> 255 = 1 [in range], 2^256 >> 255 = 2 [overflow].)
                //
                // The previous code pushed `2^256` as a literal, but 2^256 is a
                // 33-byte signed value that real NeoVM rejects (Integer max is
                // 32 bytes) — `push_integer_bigint` falls back to PUSHDATA+ADD
                // and the ADD's implicit ByteString→Integer conversion faults
                // on-chain (`MaxSize of Integer is exceeded: 33/32`, caught by
                // the neoxp differential harness). Shifting by 255 only pushes
                // the small literal `255`/`2` (1 byte each) and works on the
                // BigInt-backed result regardless of width. POW of a non-
                // negative base is non-negative, so SHR is safe. (`>> 256` is
                // unusable: NeoVM/EIP-145 collapses any shift ≥ 256 to 0.)
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(255u64),
                )));
                instructions.push(Instruction::BinaryOp(BinaryOperator::Shr));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(2u64),
                )));
                instructions.push(Instruction::BinaryOp(BinaryOperator::Ge));
                // `JumpIf` branches when FALSE — skip the panic when
                // `(result >> 255) >= 2` is FALSE (i.e. in range).
                instructions.push(Instruction::JumpIf { target: ok });
                emit_panic(0x11, instructions);
                instructions.push(Instruction::Label(ok));
            }
            instructions.push(Instruction::LoadLocal(result_local));
            emit_truncate_u256(instructions);
            // `emit_truncate_u256` leaves a 32-byte ByteArray (the two's-
            // complement word). CONVERT it back to an Integer so the result
            // is a canonical NeoVM Integer — matching the soft-arith Add/Sub/Mul
            // path (which builds Integers directly) and the `uint256` ABI return
            // encoding (a real node serializes an Integer return as a decimal
            // stack value, not a base64 ByteString).
            instructions.push(Instruction::Convert {
                target: ConvertTarget::Integer,
            });
            instructions.push(Instruction::StoreLocal(result_local));
        } else if bits == 256 && signed {
            // int256 `**`: the result type is `int256`, range
            // [-2^255, 2^255-1]. `unchecked` wraps mod 2^256 then reinterprets
            // the 32-byte word as signed; checked Panics 0x11 when the true
            // result leaves the range. Mirrors the uint256 arm above and the
            // narrow signed arm below — previously int256 fell through with NO
            // handling (no overflow check, no wrap).
            if ctx.in_unchecked_block() {
                instructions.push(Instruction::LoadLocal(result_local));
                emit_truncate_u256(instructions);
                instructions.push(Instruction::Convert {
                    target: ConvertTarget::Integer,
                });
                instructions.push(Instruction::StoreLocal(result_local));
            } else {
                // int256 bounds are 32-byte literals (0x7f.. / 0x80..), so a
                // direct Gt/Lt compare is safe (unlike the 33-byte 2^256).
                let int_max = (BigInt::one() << 255usize) - BigInt::one();
                let int_min = -(BigInt::one() << 255usize);
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
            }
        } else if matches!(bits, 8 | 16 | 32 | 64 | 128) {
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
