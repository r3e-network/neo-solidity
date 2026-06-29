use super::*;

pub(crate) fn fixed_len_bytes_be_from_hex_number(
    expr: &Expression,
    fixed_len: u16,
) -> Option<Vec<u8>> {
    let Expression::HexNumberLiteral(_, value, unit) = expr else {
        return None;
    };

    if unit.is_some() {
        return None;
    }

    let raw = value.trim().trim_start_matches("0x");
    let mut hex: String = raw
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '_')
        .collect();
    if hex.is_empty() {
        return None;
    }

    if hex.len() % 2 == 1 {
        hex.insert(0, '0');
    }

    let bytes = hex_decode(&hex).ok()?;
    let fixed_len = fixed_len as usize;
    if bytes.len() > fixed_len {
        return None;
    }

    let mut out = vec![0u8; fixed_len - bytes.len()];
    out.extend_from_slice(&bytes);
    Some(out)
}

pub(crate) fn lower_bytes_eq_hex_number_literal(
    left: &Expression,
    right: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    operator: BinaryOperator,
) -> Option<bool> {
    if !matches!(operator, BinaryOperator::Eq | BinaryOperator::Ne) {
        return None;
    }

    // Preserve Solidity left-to-right evaluation order while ensuring `bytesN` comparisons work
    // with hex-number literals like `0x01ffc9a7` (commonly used for ERC165 interface IDs).
    if let Some(ValueType::ByteArray {
        fixed_len: Some(fixed_len),
    }) = infer_type_from_expression(left, ctx)
    {
        let literal_expr = match right {
            Expression::Parenthesis(_, inner) => inner.as_ref(),
            other => other,
        };

        if let Some(bytes) = fixed_len_bytes_be_from_hex_number(literal_expr, fixed_len) {
            if lower_expression(left, ctx, instructions) {
                instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(bytes)));
                instructions.push(Instruction::BinaryOp(operator));
                return Some(true);
            }
            return Some(false);
        }
    }

    if let Some(ValueType::ByteArray {
        fixed_len: Some(fixed_len),
    }) = infer_type_from_expression(right, ctx)
    {
        let literal_expr = match left {
            Expression::Parenthesis(_, inner) => inner.as_ref(),
            other => other,
        };

        if let Some(bytes) = fixed_len_bytes_be_from_hex_number(literal_expr, fixed_len) {
            instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(bytes)));
            if lower_expression(right, ctx, instructions) {
                instructions.push(Instruction::BinaryOp(operator));
                return Some(true);
            }
            return Some(false);
        }
    }

    None
}

/// operand *expressions* (types), not the stack, so this may run after the
/// operands have been lowered.
pub(crate) fn emit_arith_with_overflow_ladder(
    left: &Expression,
    right: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    operator: BinaryOperator,
    // The `unchecked` uint256 widen (Bug #16) ends with `emit_truncate_u256`,
    // which leaves the result as a 32-byte Buffer (via SUBSTR). That is fine for
    // a plain `x = x <op> y` value but breaks when the l-value is reused AS AN
    // INTEGER before the next coercion — e.g. a loop counter `for (uint i; ...;
    // i++) a[i]` where the post-incremented `i` feeds PICKITEM ("array index
    // must be non-negative integer"). The compound / ++ / -- path therefore
    // keeps the historical plain-`BinaryOp` (Integer-result) lowering for
    // unchecked uint256 by passing `false`; the binary path passes `true`.
    allow_unchecked_u256_widen: bool,
) {
    // uint256 ordered comparison must be UNSIGNED. NeoVM `LT/GT/LE/GE` are signed,
    // and uint256 values >= 2^255 are stored as their (negative-looking) 32-byte
    // two's-complement, so a native signed compare gives the wrong answer. Map
    // unsigned order to signed order via `a <u b  <=>  (a ^ 2^255) <s (b ^ 2^255)`
    // (the `2^255` literal emits as the 32-byte sign bit). Narrow uints (< 2^255)
    // and int256 keep the native compare.
    if matches!(
        operator,
        BinaryOperator::Lt | BinaryOperator::Gt | BinaryOperator::Le | BinaryOperator::Ge
    ) && (is_typed_uint256(left, ctx) || is_typed_uint256(right, ctx))
        && !is_int256_operand(left, ctx)
        && !is_int256_operand(right, ctx)
    {
        // At least one operand is a genuinely-typed `uint256` (bare literals are
        // excluded so a signed `int256 < 5` is not misrouted) and neither is
        // `int256`: Solidity forbids mixing signed/unsigned, so the comparison
        // is unsigned 256-bit.
        emit_u256_unsigned_compare(instructions, operator);
        return;
    }

    // uint256 `/` and `%` are UNSIGNED; native NeoVM DIV/MOD are signed and wrong
    // for operands >= 2^255. Route to the software unsigned divmod. (int256 keeps
    // native signed div/mod.)
    if matches!(operator, BinaryOperator::Div | BinaryOperator::Mod)
        && (is_typed_uint256(left, ctx) || is_typed_uint256(right, ctx))
        && !is_int256_operand(left, ctx)
        && !is_int256_operand(right, ctx)
    {
        emit_u256_divmod_ir(ctx, instructions, matches!(operator, BinaryOperator::Mod));
        return;
    }

    // uint256 `>>` is a LOGICAL (zero-filling) shift. Native NeoVM SHR is
    // ARITHMETIC: a uint256 >= 2^255 is stored as a 32-byte two's-complement
    // (negative-looking) word, so the sign bit would propagate (`type(uint256)
    // .max >> 1` gives `2^256-1` instead of `2^255-1`). Gate on the LEFT
    // operand (the value being shifted) being a genuine uint256; narrow uints
    // are stored as positive narrow integers whose native SHR is already
    // logical, and int256 keeps the arithmetic shift.
    if matches!(operator, BinaryOperator::Shr) && is_typed_uint256(left, ctx) {
        emit_u256_logical_shr_ir(ctx, instructions);
        return;
    }

    // uint256 / int256 `<<` must WRAP mod 2^256 and never revert. Native NeoVM
    // SHL faults once the full-precision result leaves the 32-byte integer range
    // (any result >= 2^255, or a shift amount > 256), so route both signed and
    // unsigned 256-bit left shifts through the software `a * 2^n mod 2^256`
    // routine. (Narrow widths keep the SHL + width-mask path below.)
    if matches!(operator, BinaryOperator::Shl)
        && (is_typed_uint256(left, ctx) || is_int256_operand(left, ctx))
    {
        emit_u256_shl_ir(ctx, instructions);
        return;
    }

    let emit_guard = should_emit_u256_arith_guard(left, right, ctx, operator);
    let emit_i256_guard = !emit_guard && should_emit_i256_arith_guard(left, right, ctx, operator);
    let emit_narrow_u_bits = if !emit_guard && !emit_i256_guard {
        should_emit_narrow_u_arith_guard(left, right, ctx, operator)
    } else {
        None
    };
    let emit_narrow_i_bits = if !emit_guard && !emit_i256_guard && emit_narrow_u_bits.is_none() {
        should_emit_narrow_i_arith_guard(left, right, ctx, operator)
    } else {
        None
    };
    let emit_unchecked_u256_widen = allow_unchecked_u256_widen
        && !emit_guard
        && !emit_i256_guard
        && emit_narrow_u_bits.is_none()
        && emit_narrow_i_bits.is_none()
        && should_widen_unchecked_u256(left, right, ctx, operator);
    // `unchecked` narrow truncation (self-gating on `in_unchecked_block`).
    let unchecked_narrow_u = should_truncate_unchecked_narrow_u(left, right, ctx, operator);
    let unchecked_narrow_i = if unchecked_narrow_u.is_none() {
        should_truncate_unchecked_narrow_i(left, right, ctx, operator)
    } else {
        None
    };
    // SHL width truncation (operator-gated; mutually exclusive with the
    // Add/Sub/Mul-only guards above).
    let shl_trunc = shl_narrow_truncation(left, ctx, operator);
    // Narrow SIGNED division overflow: `intN.min / -1` is the only division that
    // overflows (result == 2^(N-1) == intN_max + 1). Unsigned division never
    // overflows; the int256 case is handled by `div_i256` below (its quotient
    // exceeds the 32-byte integer, so it needs a PRE-check). This branch covers
    // only the narrow signed widths, whose quotient still fits a NeoVM integer
    // and so can be trapped by a POST-check.
    let div_narrow_i =
        if matches!(operator, BinaryOperator::Div) && is_narrow_result(left, right, ctx) {
            narrow_signed_bits(left, right, ctx)
        } else {
            None
        };
    // int256 SIGNED division overflow: `int256.min / -1` is the only int256
    // division that overflows (quotient 2^255 needs 33 bytes). Unlike the narrow
    // widths above — whose quotient still fits NeoVM's 32-byte integer, so a
    // POST-check works — a native int256 DIV FAULTS before any check can run,
    // and the fault is UNCATCHABLE (a `try/catch Panic` never binds it). Solidity
    // requires a catchable Panic(0x11), so int256 division needs a PRE-check.
    // (uint256 division is routed to the software divmod above and returns;
    // narrow widths are handled by `div_narrow_i`.)
    let div_i256 = matches!(operator, BinaryOperator::Div)
        && div_narrow_i.is_none()
        && (is_int256_operand(left, ctx) || is_int256_operand(right, ctx));

    if emit_guard {
        // The conformant uint256 checked add/sub/mul operate directly on the
        // 32-byte two's-complement operands (no widening — the old zero-pad widen
        // would reinterpret a negative-looking value >= 2^255 as positive).
        emit_checked_arith_guard(ctx, instructions, operator);
    } else if emit_i256_guard {
        emit_checked_arith_guard_i256(ctx, instructions, operator);
    } else if let Some(bits) = emit_narrow_u_bits {
        emit_checked_arith_guard_narrow_u(ctx, instructions, operator, bits);
    } else if let Some(bits) = emit_narrow_i_bits {
        emit_checked_arith_guard_narrow_i(ctx, instructions, operator, bits);
    } else if emit_unchecked_u256_widen {
        // `unchecked { uint256: a <op> b }` wraps mod 2^256. Use the software
        // limb routines for Add/Sub/Mul so the result is conformant (no 33-byte
        // intermediate) for operands >= 2^255; other operators keep the widen +
        // native-op + truncate path.
        match operator {
            BinaryOperator::Add => emit_u256_unchecked_add_ir(ctx, instructions),
            BinaryOperator::Sub => emit_u256_unchecked_sub_ir(ctx, instructions),
            BinaryOperator::Mul => emit_u256_unchecked_mul_ir(ctx, instructions),
            _ => {
                emit_widen_both_u256_unsigned(instructions);
                instructions.push(Instruction::BinaryOp(operator));
                emit_truncate_u256(instructions);
            }
        }
    } else if let Some(bits) = unchecked_narrow_u {
        // `unchecked { uintN: a <op> b }` wraps mod 2^N. For Mul with bits >=
        // 128 a bare MUL would fault on the >32-byte full product, so compute
        // the conformant `a*b mod 2^256` with the limb routine first (it never
        // materializes a >32-byte intermediate), then mask to N bits — the low
        // N bits of the true product survive mod 2^256 unchanged.
        if matches!(operator, BinaryOperator::Mul) && bits >= 128 {
            emit_u256_unchecked_mul_ir(ctx, instructions);
        } else {
            instructions.push(Instruction::BinaryOp(operator));
        }
        emit_truncate_narrow_unsigned(instructions, bits);
    } else if let Some(bits) = unchecked_narrow_i {
        // `unchecked { intN: a <op> b }` wraps mod 2^N (two's complement).
        instructions.push(Instruction::BinaryOp(operator));
        emit_truncate_narrow_signed(ctx, instructions, bits);
    } else if let Some((bits, signed)) = shl_trunc {
        // `a << b` truncates to the left operand's width (never panics).
        instructions.push(Instruction::BinaryOp(operator));
        if signed {
            emit_truncate_narrow_signed(ctx, instructions, bits);
        } else {
            emit_truncate_narrow_unsigned(instructions, bits);
        }
    } else if let Some(bits) = div_narrow_i {
        instructions.push(Instruction::BinaryOp(operator));
        if ctx.in_unchecked_block() {
            // `unchecked { intN.min / -1 }` wraps to intN.min (mod 2^N).
            emit_truncate_narrow_signed(ctx, instructions, bits);
        } else {
            // Checked: only `intN.min / -1` exceeds intN_max → Panic(0x11).
            let int_max = (BigInt::one() << (bits as usize - 1)) - BigInt::one();
            let tmp_id = ctx.next_label();
            let result_local = ctx.allocate_local(format!("__divovf_{tmp_id}"), None);
            instructions.push(Instruction::StoreLocal(result_local));
            let done = ctx.next_label();
            instructions.push(Instruction::LoadLocal(result_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(int_max)));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Gt));
            instructions.push(Instruction::JumpIf { target: done });
            emit_panic(0x11, instructions);
            instructions.push(Instruction::Label(done));
            instructions.push(Instruction::LoadLocal(result_local));
        }
    } else if div_i256 {
        // Operands `[a, b]` are on the stack (b reloaded by the div-by-zero
        // guard upstream, so b != 0 here). Pre-check the single overflowing
        // pair `a == int256.min && b == -1` before the native DIV can fault.
        let tmp_id = ctx.next_label();
        let b_local = ctx.allocate_local(format!("__i256div_b_{tmp_id}"), None);
        let a_local = ctx.allocate_local(format!("__i256div_a_{tmp_id}"), None);
        instructions.push(Instruction::StoreLocal(b_local)); // pop b (rhs)
        instructions.push(Instruction::StoreLocal(a_local)); // pop a (lhs)
        let int256_min = -(BigInt::one() << 255usize);
        let do_div = ctx.next_label();
        let done = ctx.next_label();
        // `JumpIf` branches when the condition is FALSE, so each check jumps to
        // the plain-DIV path as soon as one half of the pair does not match.
        instructions.push(Instruction::LoadLocal(b_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(-1))));
        instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
        instructions.push(Instruction::JumpIf { target: do_div }); // b != -1 -> DIV
        instructions.push(Instruction::LoadLocal(a_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(int256_min.clone())));
        instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
        instructions.push(Instruction::JumpIf { target: do_div }); // a != min -> DIV
        // Overflow: a == int256.min && b == -1.
        if ctx.in_unchecked_block() {
            // `unchecked` wraps mod 2^256: int256.min / -1 == int256.min.
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(int256_min)));
        } else {
            emit_panic(0x11, instructions); // throws; no fall-through to do_div
        }
        instructions.push(Instruction::Jump { target: done });
        instructions.push(Instruction::Label(do_div));
        instructions.push(Instruction::LoadLocal(a_local));
        instructions.push(Instruction::LoadLocal(b_local));
        instructions.push(Instruction::BinaryOp(BinaryOperator::Div));
        instructions.push(Instruction::Label(done));
    } else {
        instructions.push(Instruction::BinaryOp(operator));
    }
}

pub(crate) fn lower_binary_expr(
    left: &Expression,
    right: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    operator: BinaryOperator,
) -> bool {
    if let Some(result) =
        lower_bytes_eq_hex_number_literal(left, right, ctx, instructions, operator)
    {
        return result;
    }

    // Fixed-width byte/address casts (`bytesN(..)`, `address(..)`) now
    // canonicalize through `coerce_to_fixed_bytes` into a single ByteString
    // result. Earlier cleanup here assumed an extra leaked buffer lived under
    // leaked buffer with the canonical result instead of `x` with `bytes32(0)`
    // — which silently collapses the zero-literal sentinel check and blocks
    // role/sentinel idioms (`require(role != bytes32(0), ...)`). Mirror the
    // Swap;Drop cleanup pattern already used by `bytes.concat` and
    // `abi.encodePacked` at `src/ir/expressions/calls/builtins.rs:80-83` and
    // `src/ir/expressions/calls/builtins/resolved.rs:103-107`. See fuzz tests
    // `batch30_h4b_bytes32_ne_zero`.
    if !lower_expression(left, ctx, instructions) {
        return false;
    }
    // No Swap; Drop needed: real NeoVM MEMCPY pushes nothing.
    if !lower_expression(right, ctx, instructions) {
        return false;
    }
    // No Swap; Drop needed: real NeoVM MEMCPY pushes nothing.

    // Solidity 0.8+ panics on division/modulo by zero.
    if matches!(operator, BinaryOperator::Div | BinaryOperator::Mod) {
        let tmp_id = ctx.next_label();
        let rhs_local = ctx.allocate_local(format!("__div_rhs_{tmp_id}"), None);

        // Preserve left operand on the stack while we validate RHS.
        instructions.push(Instruction::StoreLocal(rhs_local)); // pops RHS

        // Task #103 / Task #107 — div/mod by zero must THROW the canonical
        // EVM Panic(uint256) envelope:
        //   keccak256("Panic(uint256)")[0..4] || abi.encode(0x12)
        // so `try { … } catch Panic(uint code) { … }` can decode
        // `code == 0x12` off the 4-byte selector + 32-byte big-endian payload.
        // Routes through the shared `emit_panic` helper; matches the shape
        // emitted by assert(false)=0x01, empty-pop=0x31, enum-cast=0x21,
        // arith-overflow=0x11, abi.decode-short=0x41.
        let ok_label = ctx.next_label();
        instructions.push(Instruction::LoadLocal(rhs_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::zero(),
        )));
        instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
        instructions.push(Instruction::JumpIf { target: ok_label });
        emit_panic(0x12, instructions);
        instructions.push(Instruction::Label(ok_label));

        // Reload RHS and perform the operation.
        instructions.push(Instruction::LoadLocal(rhs_local));
    }

    emit_arith_with_overflow_ladder(left, right, ctx, instructions, operator, true);
    true
}
