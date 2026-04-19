fn fixed_len_bytes_be_from_hex_number(expr: &Expression, fixed_len: u16) -> Option<Vec<u8>> {
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

fn lower_bytes_eq_hex_number_literal(
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

/// True if the expression is a literal number (number/hex). Used to avoid
/// emitting the overflow guard when both operands are compile-time constants —
/// the compiler's constant folder already handles those.
fn is_literal_number(expr: &Expression) -> bool {
    matches!(
        match expr {
            Expression::Parenthesis(_, inner) => inner.as_ref(),
            other => other,
        },
        Expression::NumberLiteral(..)
            | Expression::HexNumberLiteral(..)
            | Expression::RationalNumberLiteral(..)
    )
}

/// Infer whether an operand is `uint256` (256-bit unsigned integer). Defaults
/// to `true` for literal numbers because Solidity number literals have type
/// `uint256` by default.
fn is_uint256_operand(expr: &Expression, ctx: &LoweringContext) -> bool {
    match infer_type_from_expression(expr, ctx) {
        Some(ValueType::Integer {
            signed: false,
            bits: 256,
        }) => true,
        Some(_) => false,
        // Unknown type (complex expressions): assume uint256 for literal
        // numbers, otherwise bail out.
        None => is_literal_number(expr),
    }
}

/// Infer whether an operand is `int256` (256-bit signed integer).
fn is_int256_operand(expr: &Expression, ctx: &LoweringContext) -> bool {
    matches!(
        infer_type_from_expression(expr, ctx),
        Some(ValueType::Integer {
            signed: true,
            bits: 256,
        })
    )
}

/// Task #30 slice 2: gate for `uint256` Add/Sub/Mul overflow-guard emission.
/// Returns `true` when the binary op needs a Solidity-0.8.x checked-arithmetic
/// panic guard emitted around it:
///   - operator is Add/Sub/Mul
///   - at least one operand is `uint256`
///   - not inside an `unchecked { ... }` block
///   - not both operands are compile-time literals (constant-folded)
fn should_emit_u256_arith_guard(
    left: &Expression,
    right: &Expression,
    ctx: &LoweringContext,
    operator: BinaryOperator,
) -> bool {
    if ctx.in_unchecked_block() {
        return false;
    }
    if !matches!(
        operator,
        BinaryOperator::Add | BinaryOperator::Sub | BinaryOperator::Mul
    ) {
        return false;
    }
    if is_literal_number(left) && is_literal_number(right) {
        return false;
    }
    // Task #67: when either operand is typed `int256`, Solidity's type-coercion
    // rules widen the other operand (typically a literal) to `int256`. The
    // int256 guard path owns the lowering in that case.
    if is_int256_operand(left, ctx) || is_int256_operand(right, ctx) {
        return false;
    }
    is_uint256_operand(left, ctx) || is_uint256_operand(right, ctx)
}

/// Batch-#30 H1: detect narrow unsigned integer operands (uint8/16/32/64/128)
/// so we can emit a width-aware post-op overflow check. The runtime performs
/// arithmetic through the i64/BigInt stack-item paths which DON'T wrap at the
/// narrow Solidity width; `uint8(255) + uint8(1)` evaluates to `256` on the
/// stack instead of Panic(0x11). Returns the bit width when BOTH operand
/// types are the same narrow unsigned width (Solidity rejects mixed-width
/// arithmetic at parse time unless one side is an untyped literal, in which
/// case we rely on the typed operand's width).
fn narrow_unsigned_bits(
    left: &Expression,
    right: &Expression,
    ctx: &LoweringContext,
) -> Option<u16> {
    fn narrow_bits(expr: &Expression, ctx: &LoweringContext) -> Option<u16> {
        match infer_type_from_expression(expr, ctx) {
            Some(ValueType::Integer {
                signed: false,
                bits,
            }) if matches!(bits, 8 | 16 | 32 | 64 | 128) => Some(bits),
            _ => None,
        }
    }
    narrow_bits(left, ctx).or_else(|| narrow_bits(right, ctx))
}

/// Batch-#30 H1: gate for narrow unsigned (uintN, N in {8,16,32,64,128})
/// Add/Sub/Mul overflow-guard emission. Same preconditions as
/// `should_emit_u256_arith_guard` except it fires for narrow widths instead
/// of 256-bit. Returns the target bit width when a guard should be emitted.
fn should_emit_narrow_u_arith_guard(
    left: &Expression,
    right: &Expression,
    ctx: &LoweringContext,
    operator: BinaryOperator,
) -> Option<u16> {
    if ctx.in_unchecked_block() {
        return None;
    }
    if !matches!(
        operator,
        BinaryOperator::Add | BinaryOperator::Sub | BinaryOperator::Mul
    ) {
        return None;
    }
    if is_literal_number(left) && is_literal_number(right) {
        return None;
    }
    // Don't clash with the uint256 or int256 guards.
    if is_uint256_operand(left, ctx) || is_uint256_operand(right, ctx) {
        return None;
    }
    if is_int256_operand(left, ctx) || is_int256_operand(right, ctx) {
        return None;
    }
    narrow_unsigned_bits(left, right, ctx)
}

/// Task #154: detect narrow signed integer operands (int8/16/32/64/128) so we
/// can emit a width-aware post-op overflow check. The runtime performs
/// arithmetic through the i64/BigInt stack-item paths which DON'T wrap at the
/// narrow Solidity width; `int128(type(int128).max) + int128(1)` evaluates to
/// `2^127` on the stack instead of Panic(0x11). Returns the bit width when
/// BOTH operand types are the same narrow signed width (Solidity rejects
/// mixed-width arithmetic at parse time unless one side is an untyped literal,
/// in which case we rely on the typed operand's width).
fn narrow_signed_bits(
    left: &Expression,
    right: &Expression,
    ctx: &LoweringContext,
) -> Option<u16> {
    fn narrow_bits(expr: &Expression, ctx: &LoweringContext) -> Option<u16> {
        match infer_type_from_expression(expr, ctx) {
            Some(ValueType::Integer {
                signed: true,
                bits,
            }) if matches!(bits, 8 | 16 | 32 | 64 | 128) => Some(bits),
            _ => None,
        }
    }
    narrow_bits(left, ctx).or_else(|| narrow_bits(right, ctx))
}

/// Task #154: gate for narrow signed (intN, N in {8,16,32,64,128}) Add/Sub/Mul
/// overflow-guard emission. Mirrors `should_emit_narrow_u_arith_guard` for the
/// signed domain. Returns the target bit width when a guard should be emitted.
fn should_emit_narrow_i_arith_guard(
    left: &Expression,
    right: &Expression,
    ctx: &LoweringContext,
    operator: BinaryOperator,
) -> Option<u16> {
    if ctx.in_unchecked_block() {
        return None;
    }
    if !matches!(
        operator,
        BinaryOperator::Add | BinaryOperator::Sub | BinaryOperator::Mul
    ) {
        return None;
    }
    if is_literal_number(left) && is_literal_number(right) {
        return None;
    }
    // Don't clash with the uint256 or int256 guards.
    if is_uint256_operand(left, ctx) || is_uint256_operand(right, ctx) {
        return None;
    }
    if is_int256_operand(left, ctx) || is_int256_operand(right, ctx) {
        return None;
    }
    narrow_signed_bits(left, right, ctx)
}

/// Batch-#30 H1: emit the narrow-width overflow/underflow check for uintN
/// Add/Sub/Mul. Consumes `[lhs, rhs]` and pushes the result. The runtime's
/// i64/BigInt arithmetic doesn't wrap at `uintN`, so we compute the result
/// and then range-check against `[0, 2^bits - 1]`; out-of-range → Panic(0x11).
fn emit_checked_arith_guard_narrow_u(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    operator: BinaryOperator,
    bits: u16,
) {
    let tmp_id = ctx.next_label();
    let result_local = ctx.allocate_local(format!("__narith_res_{tmp_id}"), None);

    // uintN_max = (1 << bits) - 1, encoded as a literal BigInt.
    let uint_max = (BigInt::one() << bits as usize) - BigInt::one();

    // Compute result = lhs op rhs.
    instructions.push(Instruction::BinaryOp(operator));
    instructions.push(Instruction::StoreLocal(result_local));

    // Check: result > uintN_max → Panic(0x11). Catches Add/Mul overflow.
    // Task #107 — route through canonical EVM Panic(uint256) envelope so
    // `catch Panic(uint code)` can bind code = 0x11.
    let after_max_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(result_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(uint_max)));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Gt));
    instructions.push(Instruction::JumpIf {
        target: after_max_label,
    });
    emit_panic(0x11, instructions);
    instructions.push(Instruction::Label(after_max_label));

    // Check: result < 0 → Panic(0x11). Catches Sub underflow (lhs < rhs).
    let done_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(result_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: done_label });
    emit_panic(0x11, instructions);

    // Done: push result back onto the stack.
    instructions.push(Instruction::Label(done_label));
    instructions.push(Instruction::LoadLocal(result_local));
}

/// Task #30 slice 4 / Task #67: gate for `int256` Add/Sub/Mul overflow-guard
/// emission. The runtime performs wide arithmetic via `BigInt` at true signed
/// precision (`to_signed_bytes_le`/`from_signed_bytes_le`), so post-op range
/// checks against `INT256_MIN..=INT256_MAX` observe the un-wrapped result and
/// can detect overflow/underflow directly. Accepted when either operand is
/// typed `int256` (literal numbers on one side inherit the typed operand's
/// width via Solidity's implicit conversion rules).
fn should_emit_i256_arith_guard(
    left: &Expression,
    right: &Expression,
    ctx: &LoweringContext,
    operator: BinaryOperator,
) -> bool {
    if ctx.in_unchecked_block() {
        return false;
    }
    if !matches!(
        operator,
        BinaryOperator::Add | BinaryOperator::Sub | BinaryOperator::Mul
    ) {
        return false;
    }
    if is_literal_number(left) && is_literal_number(right) {
        return false;
    }
    is_int256_operand(left, ctx) || is_int256_operand(right, ctx)
}

/// Widen the top-of-stack operand from a narrow integer (may be pushed as
/// PUSHINT8..PUSHINT64) to a 32-byte ByteArray so BigInt comparisons fire at
/// the full 256-bit width. Uses CONVERT ByteArray + CAT 24 zero bytes (on the
/// high side for signed representation, which is LE) to pad the narrow value
/// to 32 bytes.
fn emit_widen_to_u256(instructions: &mut Vec<Instruction>) {
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
    // BytesConcat takes (left, right) → left ++ right; push 24 zero bytes on
    // the right so the little-endian representation is padded on the high side.
    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(vec![0u8; 24])));
    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::BytesConcat,
        arg_count: 2,
    });
}

/// Widen both operands currently on the stack ([lhs, rhs]) to 32-byte ByteArray
/// so subsequent BigInt comparisons fire at the full 256-bit width.
///
/// Stack transformation:
///   [lhs, rhs] -> [wide(lhs), wide(rhs)]
///
/// Strategy: widen rhs (top), swap, widen lhs (now top), swap back.
fn emit_widen_both_u256(instructions: &mut Vec<Instruction>) {
    emit_widen_to_u256(instructions);
    instructions.push(Instruction::Swap);
    emit_widen_to_u256(instructions);
    instructions.push(Instruction::Swap);
}

/// Task #67: emit the int256 checked-arithmetic range-check guard for
/// Add/Sub/Mul. Consumes [lhs, rhs] and pushes the result.
///
/// Strategy: rely on the runtime's true signed-BigInt arithmetic (no wrap at
/// the bigint layer — see `bigint_to_stack_item`). Compute the result, then
/// range-check against `[INT256_MIN, INT256_MAX]`; panic on out-of-range.
/// Works for all three ops because the runtime never wraps the pre-check
/// result. INT256_MIN/MAX are pushed as 32-byte signed-LE ByteArrays so the
/// `less_than` / `greater_than` helpers automatically route through the
/// BigInt comparison path.
fn emit_checked_arith_guard_i256(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    operator: BinaryOperator,
) {
    let tmp_id = ctx.next_label();
    let result_local = ctx.allocate_local(format!("__sarith_res_{tmp_id}"), None);

    // INT256_MAX = 2^255 - 1, signed-LE 32 bytes: [0xFF*31, 0x7F].
    let mut int256_max_bytes: Vec<u8> = vec![0xffu8; 32];
    int256_max_bytes[31] = 0x7f;
    // INT256_MIN = -2^255, signed-LE 32 bytes: [0x00*31, 0x80].
    let mut int256_min_bytes: Vec<u8> = vec![0u8; 32];
    int256_min_bytes[31] = 0x80;

    // Compute result = lhs op rhs using the native signed BigInt path.
    instructions.push(Instruction::BinaryOp(operator));
    instructions.push(Instruction::StoreLocal(result_local));

    // Check: result > INT256_MAX → Panic(0x11).
    // Task #107 — canonical EVM Panic(uint256) envelope for `catch Panic(uint)`.
    let after_max_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(result_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
        int256_max_bytes,
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Gt));
    // JumpIf-on-false: safe when Gt is false.
    instructions.push(Instruction::JumpIf {
        target: after_max_label,
    });
    emit_panic(0x11, instructions);
    instructions.push(Instruction::Label(after_max_label));

    // Check: result < INT256_MIN → Panic(0x11).
    let done_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(result_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
        int256_min_bytes,
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: done_label });
    emit_panic(0x11, instructions);

    // Done: push result back onto the stack.
    instructions.push(Instruction::Label(done_label));
    instructions.push(Instruction::LoadLocal(result_local));
}

/// Task #154: emit the narrow signed (intN, N ∈ {8,16,32,64,128})
/// checked-arithmetic range-check guard for Add/Sub/Mul. Consumes `[lhs, rhs]`
/// and pushes the result.
///
/// Strategy mirrors `emit_checked_arith_guard_i256`: compute the op directly
/// (the runtime's wide-BigInt path does not wrap when either operand is a
/// ByteArray > 8 bytes, and the narrow i64 path fits any result of narrow-int
/// arithmetic at widths ≤ 64 bits), then range-check against
/// `[-(2^(bits-1)), 2^(bits-1) - 1]`. Bound literals are pushed as 32-byte
/// signed-LE ByteArrays so the `less_than` / `greater_than` helpers route
/// through the BigInt comparison path (see `cmp_needs_bigint_path`).
///
/// At bit widths ≤ 64 the narrow i64 arithmetic path may itself raise an
/// `ExecutionError` (strict-arithmetic mode) before we reach this guard — that
/// is fine: the caller still surfaces a fault, and the fault shape is
/// upgraded to the canonical Panic(0x11) envelope by this guard whenever the
/// BigInt path is taken (narrow int128, or narrow-int summands whose results
/// happen to overflow i64 without overflowing the Solidity-level width).
fn emit_checked_arith_guard_narrow_i(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    operator: BinaryOperator,
    bits: u16,
) {
    let tmp_id = ctx.next_label();
    let result_local = ctx.allocate_local(format!("__sarith_n_res_{tmp_id}"), None);

    // intN_MAX = 2^(bits-1) - 1, signed-LE 32 bytes.
    // Low `bytes_payload = bits / 8` bytes encode `2^(bits-1) - 1` =
    //   [0xff * (payload-1), 0x7f]; remaining high bytes are sign-extended
    //   zeros (positive).
    let payload = (bits as usize) / 8;
    let mut int_max_bytes: Vec<u8> = vec![0u8; 32];
    for b in int_max_bytes.iter_mut().take(payload - 1) {
        *b = 0xff;
    }
    int_max_bytes[payload - 1] = 0x7f;
    // intN_MIN = -2^(bits-1), signed-LE 32 bytes.
    // Low `payload` bytes encode `-2^(bits-1)` as [0x00 * (payload-1), 0x80];
    // remaining high bytes are sign-extended 0xff (negative).
    let mut int_min_bytes: Vec<u8> = vec![0xffu8; 32];
    for b in int_min_bytes.iter_mut().take(payload - 1) {
        *b = 0x00;
    }
    int_min_bytes[payload - 1] = 0x80;

    // Compute result = lhs op rhs.
    instructions.push(Instruction::BinaryOp(operator));
    instructions.push(Instruction::StoreLocal(result_local));

    // Check: result > intN_MAX → Panic(0x11).
    let after_max_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(result_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(int_max_bytes)));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Gt));
    instructions.push(Instruction::JumpIf {
        target: after_max_label,
    });
    emit_panic(0x11, instructions);
    instructions.push(Instruction::Label(after_max_label));

    // Check: result < intN_MIN → Panic(0x11).
    let done_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(result_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(int_min_bytes)));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: done_label });
    emit_panic(0x11, instructions);

    // Done: push result back onto the stack.
    instructions.push(Instruction::Label(done_label));
    instructions.push(Instruction::LoadLocal(result_local));
}

/// Emit the in-body checked-arithmetic guard sequence for Add/Sub/Mul. The
/// caller must have already pushed [lhs, rhs] onto the stack (widened if
/// appropriate).
///
/// Stack contract: consumes [lhs, rhs] and pushes the result.
///
///   Add: computes result, then asserts `result >= lhs` (else Panic 0x11).
///   Sub: pre-check `rhs > lhs` (else Panic 0x11), then computes result.
///   Mul: computes result, then asserts `rhs == 0 || result / rhs == lhs`.
fn emit_checked_arith_guard(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    operator: BinaryOperator,
) {
    let tmp_id = ctx.next_label();
    let lhs_local = ctx.allocate_local(format!("__arith_lhs_{tmp_id}"), None);
    let rhs_local = ctx.allocate_local(format!("__arith_rhs_{tmp_id}"), None);
    let result_local = ctx.allocate_local(format!("__arith_res_{tmp_id}"), None);

    // Store [lhs, rhs] into locals so we can reuse them for the guard check.
    instructions.push(Instruction::StoreLocal(rhs_local));
    instructions.push(Instruction::StoreLocal(lhs_local));

    // Note on JumpIf semantics: in this compiler's IR, `JumpIf { target }`
    // branches when the popped condition is **FALSE** (lowers to NeoVM
    // JMPIFNOT_L). The guards below push the OVERFLOW condition and use
    // JumpIf to skip the THROW when overflow is FALSE (i.e. safe), falling
    // through to THROW when overflow is TRUE.
    //
    // The runtime performs wide ops via BigInt (slice 1 Part C), so arithmetic
    // is effectively arbitrary-precision — guards compare against
    // `UINT256_MAX = 2^256 - 1` to detect out-of-range results rather than
    // looking for a numeric wrap (which doesn't happen).
    let done_label = ctx.next_label();

    // UINT256_MAX literal used by the Add/Mul post-checks. Encoded as a
    // 33-byte signed LE ByteArray where the high byte is `0x00` so
    // `BigInt::from_signed_bytes_le` decodes it as the positive `2^256 - 1`
    // rather than the signed `-1`.
    let mut uint256_max_bytes: Vec<u8> = vec![0xffu8; 32];
    uint256_max_bytes.push(0x00);

    match operator {
        BinaryOperator::Add => {
            // Compute result = lhs + rhs.
            instructions.push(Instruction::LoadLocal(lhs_local));
            instructions.push(Instruction::LoadLocal(rhs_local));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(result_local));

            // Overflow if `result > UINT256_MAX`. Since runtime Adds at the
            // full BigInt width, the sum of two uint256 values can exceed
            // 2^256-1 by up to +2^256-1.
            // Task #107 — canonical EVM Panic(uint256) envelope.
            instructions.push(Instruction::LoadLocal(result_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                uint256_max_bytes.clone(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Gt));
            // JumpIf-on-false: if overflow is FALSE (safe), skip the THROW.
            instructions.push(Instruction::JumpIf { target: done_label });
            // Fall through: overflow is TRUE → THROW.
            emit_panic(0x11, instructions);
        }
        BinaryOperator::Sub => {
            // Pre-check: underflow iff `rhs > lhs`. Runs BEFORE the Sub so we
            // never observe a signed wrapped value.
            instructions.push(Instruction::LoadLocal(rhs_local));
            instructions.push(Instruction::LoadLocal(lhs_local));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Gt));
            // JumpIf-on-false: if underflow is FALSE (safe), skip THROW and
            // compute the Sub below.
            let safe_label = ctx.next_label();
            instructions.push(Instruction::JumpIf {
                target: safe_label,
            });
            // Task #107 — canonical EVM Panic(uint256) envelope.
            emit_panic(0x11, instructions);
            instructions.push(Instruction::Label(safe_label));

            // Compute result = lhs - rhs.
            instructions.push(Instruction::LoadLocal(lhs_local));
            instructions.push(Instruction::LoadLocal(rhs_local));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Sub));
            instructions.push(Instruction::StoreLocal(result_local));
        }
        BinaryOperator::Mul => {
            // Compute result = lhs * rhs.
            instructions.push(Instruction::LoadLocal(lhs_local));
            instructions.push(Instruction::LoadLocal(rhs_local));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
            instructions.push(Instruction::StoreLocal(result_local));

            // Overflow if `result > UINT256_MAX`.
            // Task #107 — canonical EVM Panic(uint256) envelope.
            instructions.push(Instruction::LoadLocal(result_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                uint256_max_bytes.clone(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Gt));
            instructions.push(Instruction::JumpIf { target: done_label });
            emit_panic(0x11, instructions);
        }
        _ => {
            // Non-arith ops: this function should not be called; just emit the
            // operator directly to preserve semantics in case of misuse.
            instructions.push(Instruction::LoadLocal(lhs_local));
            instructions.push(Instruction::LoadLocal(rhs_local));
            instructions.push(Instruction::BinaryOp(operator));
            instructions.push(Instruction::StoreLocal(result_local));
        }
    }

    // Done: push result back onto the stack.
    instructions.push(Instruction::Label(done_label));
    instructions.push(Instruction::LoadLocal(result_local));
}

fn lower_binary_expr(
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

    // Task #30: decide up-front whether to emit the checked-arithmetic guard so
    // we can widen operands (slice 3) before lowering to BigInt-safe shapes.
    let emit_guard = should_emit_u256_arith_guard(left, right, ctx, operator);
    // Task #67: signed int256 overflow guard (applies only when u256 guard
    // doesn't — a variable typed int256 cannot also be uint256).
    let emit_i256_guard =
        !emit_guard && should_emit_i256_arith_guard(left, right, ctx, operator);
    // Batch-#30 H1: narrow unsigned (uint8/16/32/64/128) width-aware overflow
    // guard. Fires only when neither the uint256 nor int256 guards are active.
    let emit_narrow_u_bits = if !emit_guard && !emit_i256_guard {
        should_emit_narrow_u_arith_guard(left, right, ctx, operator)
    } else {
        None
    };
    // Task #154: narrow signed (int8/16/32/64/128) width-aware overflow guard.
    // Fires only when none of the above guards are active. Mirrors the narrow
    // unsigned path for the signed domain — Task #67 only threaded the int256
    // case, so `int128(type(int128).max) + int128(1)` previously silently
    // wrapped to `INT128_MIN` instead of panicking.
    let emit_narrow_i_bits = if !emit_guard && !emit_i256_guard && emit_narrow_u_bits.is_none() {
        should_emit_narrow_i_arith_guard(left, right, ctx, operator)
    } else {
        None
    };

    // Batch-#30 H4b fix: `coerce_to_fixed_bytes` (invoked by `bytesN(..)` /
    // `address(..)` casts — see `is_fixed_bytes_cast_expr`) leaves the
    // MEMCPY-returned destination buffer on the stack BENEATH the canonical
    // ByteString result. Binary ops like `Eq` / `Ne` pop the top two stack
    // items, so an expression of the form `x OP bytes32(0)` would compare the
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
    if is_fixed_bytes_cast_expr(left) {
        instructions.push(Instruction::Swap);
        instructions.push(Instruction::Drop(ValueType::Any));
    }
    if !lower_expression(right, ctx, instructions) {
        return false;
    }
    if is_fixed_bytes_cast_expr(right) {
        instructions.push(Instruction::Swap);
        instructions.push(Instruction::Drop(ValueType::Any));
    }

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
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
        instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
        instructions.push(Instruction::JumpIf { target: ok_label });
        emit_panic(0x12, instructions);
        instructions.push(Instruction::Label(ok_label));

        // Reload RHS and perform the operation.
        instructions.push(Instruction::LoadLocal(rhs_local));
    }

    if emit_guard {
        // Slice 3: widen both operands to 32-byte ByteArray so BigInt-based
        // comparisons in the guard body work at full 256-bit width.
        emit_widen_both_u256(instructions);
        emit_checked_arith_guard(ctx, instructions, operator);
    } else if emit_i256_guard {
        // Task #67: post-op range check against INT256_MIN/MAX. No pre-op
        // widening needed — the INT256 bound literals are 32-byte ByteArrays
        // which force the `less_than`/`greater_than` BigInt comparison path
        // (see `cmp_needs_bigint_path` in
        // `src/runtime/execution/helpers/arithmetic/basic_ops.rs`). Narrow
        // int256 operands are coerced to BigInt via the Integer stack-item
        // arm of `coerce_item_to_bigint`, so no operand-side widening is
        // required.
        emit_checked_arith_guard_i256(ctx, instructions, operator);
    } else if let Some(bits) = emit_narrow_u_bits {
        // Batch-#30 H1: post-op range check against `[0, 2^bits - 1]`. The
        // runtime's i64-backed arithmetic doesn't wrap at narrow widths, so
        // `uint8(255) + uint8(1)` leaves `256` on the stack — out-of-range
        // for uint8. Without this guard, a silent overflow returns the
        // widened result and violates the Solidity 0.8.x checked-arithmetic
        // invariant. See fuzz test `batch30_h1_narrow_uint8_overflow_at_max`.
        emit_checked_arith_guard_narrow_u(ctx, instructions, operator, bits);
    } else if let Some(bits) = emit_narrow_i_bits {
        // Task #154: post-op range check against `[-(2^(bits-1)), 2^(bits-1) - 1]`.
        // Bound literals are 32-byte signed-LE ByteArrays so the comparison
        // routes through the BigInt path regardless of how narrow the result
        // lands. Without this guard, `int128(type(int128).max) + int128(1)`
        // silently evaluates to `2^127` (wide BigInt result) instead of
        // panicking. See fuzz test
        // `batch65_oo3_int128_checked_arithmetic_endpoints`.
        emit_checked_arith_guard_narrow_i(ctx, instructions, operator, bits);
    } else {
        instructions.push(Instruction::BinaryOp(operator));
    }
    true
}
