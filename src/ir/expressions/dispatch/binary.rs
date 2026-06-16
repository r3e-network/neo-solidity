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

/// True only when `expr` is a *genuinely-typed* `uint256` operand. A bare
/// integer literal infers as `uint256` by default (see `infer_type_from_expression`
/// in `src/ir/build/inference.rs`), so it is explicitly excluded here: in
/// `narrowVar OP literal` the literal adapts to the narrow operand's type and
/// must NOT count as a real `uint256` that suppresses the narrow guard.
fn is_typed_uint256(expr: &Expression, ctx: &LoweringContext) -> bool {
    !is_literal_number(expr)
        && matches!(
            infer_type_from_expression(expr, ctx),
            Some(ValueType::Integer {
                signed: false,
                bits: 256,
            })
        )
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

/// True when the arithmetic's RESULT type is a narrow integer (uintN/intN, N<256)
/// — i.e. a narrow-typed operand is present AND no operand is a genuinely-typed
/// `uint256`/`int256`. This distinguishes pure-narrow arithmetic (`uint8 + uint8`,
/// `uint8 + literal`), which the narrow guard / narrow truncation owns, from
/// MIXED-width arithmetic (`uint256 + uint32`), where Solidity widens the narrow
/// operand so the result is `uint256` and the 256-bit path must own it. Untyped
/// literals adapt to the narrow operand's type and never count as wide here.
fn is_narrow_result(left: &Expression, right: &Expression, ctx: &LoweringContext) -> bool {
    if is_typed_uint256(left, ctx) || is_typed_uint256(right, ctx) {
        return false;
    }
    if is_int256_operand(left, ctx) || is_int256_operand(right, ctx) {
        return false;
    }
    narrow_unsigned_bits(left, right, ctx).is_some() || narrow_signed_bits(left, right, ctx).is_some()
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
    // `narrowVar OP literal` (and `uintN OP uintN`) is narrow arithmetic — the
    // untyped literal adapts to the narrow operand's type. Without this, the
    // literal's `uint256` default would route `uint8 x; x + 1` through the
    // 256-bit guard (which only checks `result > 2^256-1` and never trips for a
    // narrow overflow like 255+1=256), silently skipping the required
    // Panic(0x11). The narrow guard owns it. NOTE: mixed `uint256 OP uintN` is
    // NOT narrow (the narrow operand widens to uint256) and stays on this path.
    if is_narrow_result(left, right, ctx) {
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
    // Don't clash with the uint256 or int256 guards. Use the *typed* uint256
    // check (not `is_uint256_operand`) so a literal partner — which defaults to
    // uint256 — to a narrow-typed operand does NOT suppress the narrow guard.
    if is_typed_uint256(left, ctx) || is_typed_uint256(right, ctx) {
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
    // Don't clash with the uint256 or int256 guards. Use the *typed* uint256
    // check so a literal partner (uint256 by default) to a narrow-typed operand
    // doesn't suppress the narrow signed guard.
    if is_typed_uint256(left, ctx) || is_typed_uint256(right, ctx) {
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


/// Bug #16: widen the top-of-stack uint256 operand into a >8-byte ByteArray
/// whose `from_signed_bytes_le` decoding is **non-negative** — i.e. an
/// unsigned-magnitude representation. This routes the runtime through its
/// wide-BigInt arithmetic path (`cmp_needs_bigint_path` triggers when either
/// operand is a ByteArray > 8 bytes) without the signed-BigInt accident that
/// reinterprets `[0xFF; 32]` as `-1`.
///
/// Strategy:
///   1. CONVERT operand to ByteArray (signed-LE encoding from the runtime).
///   2. Append a single 0x00 byte. In LE, the LAST byte's high bit is the
///      sign bit for `from_signed_bytes_le`; a trailing 0x00 forces a
///      positive interpretation regardless of the original payload.
///
/// Stack transformation:
///   [..., x] -> [..., bytes_le(x) ++ 0x00]
fn emit_widen_to_u256_unsigned(instructions: &mut Vec<Instruction>) {
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(vec![0u8])));
    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::BytesConcat,
        arg_count: 2,
    });
}

/// Bug #16: widen both operands currently on the stack ([lhs, rhs]) using the
/// unsigned-magnitude representation (see `emit_widen_to_u256_unsigned`). Used
/// by the `unchecked { ... }` uint256 Add/Sub/Mul lowering so all four
/// optimizer levels behave the same way.
fn emit_widen_both_u256_unsigned(instructions: &mut Vec<Instruction>) {
    emit_widen_to_u256_unsigned(instructions);
    instructions.push(Instruction::Swap);
    emit_widen_to_u256_unsigned(instructions);
    instructions.push(Instruction::Swap);
}

/// Bug #16: post-op truncate the top-of-stack result to its low 32 bytes
/// (mod 2^256). Pairs with `emit_widen_both_u256_unsigned` for unchecked
/// uint256 Add/Sub/Mul: after the BigInt-wide op, a result like 2^256 sits
/// on the stack as a 33-byte signed-LE ByteArray — without this truncation,
/// `decode_uint_le` would yield 2^256 instead of 0.
///
/// Strategy:
///   1. CONVERT result to ByteArray.
///   2. Append 32 zero bytes (so a too-narrow result becomes ≥ 32 bytes).
///   3. SUBSTR(0, 32) — take the low 32 bytes.
///
/// Stack transformation:
///   [..., x] -> [..., bytes_le(x)[0..32]]
fn emit_truncate_u256(instructions: &mut Vec<Instruction>) {
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(vec![0u8; 32])));
    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::BytesConcat,
        arg_count: 2,
    });
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(32u64))));
    instructions.push(Instruction::Substr);
}

/// Bug #16: gate for unchecked uint256 Add/Sub/Mul. Returns `true` when:
///   - in an `unchecked { ... }` block (so the checked guard does NOT fire);
///   - operator is Add/Sub/Mul;
///   - at least one operand is `uint256`;
///   - not both operands are compile-time literals (constant-folded);
///   - not an int256 op (those flow through their own guard).
///
/// When this fires, the lowering widens both operands to a >8-byte unsigned
/// representation so the runtime's wide-BigInt arithmetic path takes over —
/// avoiding the i64 narrow-path strict-overflow fault that diverges from
/// constant-folded results at higher optimizer levels.
fn should_widen_unchecked_u256(
    left: &Expression,
    right: &Expression,
    ctx: &LoweringContext,
    operator: BinaryOperator,
) -> bool {
    if !ctx.in_unchecked_block() {
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
    if is_int256_operand(left, ctx) || is_int256_operand(right, ctx) {
        return false;
    }
    // Pure-narrow `unchecked` arithmetic truncates mod 2^N (N<256), not mod
    // 2^256; the narrow unchecked-truncation path owns it. Without this, the
    // literal's uint256 default would widen `unchecked { uint8 x; x + 1 }` to
    // 256-bit and truncate mod 2^256 (yielding 256 instead of the correct 0).
    // Mixed `uint256 OP uintN` is NOT narrow (the narrow operand widens) and
    // stays on this 256-bit widen+truncate path.
    if is_narrow_result(left, right, ctx) {
        return false;
    }
    is_uint256_operand(left, ctx) || is_uint256_operand(right, ctx)
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

// ---- Inline software uint256 routines (32-byte two's-complement) -------------
//
// NeoVM integers are signed two's-complement capped at 32 bytes, so a `uint256`
// value >= 2^255 is stored as its negative-looking two's-complement and a native
// ADD/SUB/MUL can both fault (a 33-byte intermediate) and wrap in a way the
// `GetSize > 32` overflow heuristic cannot see. These helpers compute the
// UNSIGNED result over 128-bit (add/sub) or 64-bit (mul) limbs so no intermediate
// ever exceeds 32 bytes — the same routines validated in
// `cli/bytecode/uint256_ops.rs` against a faithful reference VM — emitted here as
// IR over a shared scratch-local pool.

fn u256_push(ins: &mut Vec<Instruction>, v: BigInt) {
    ins.push(Instruction::PushLiteral(LiteralValue::Integer(v)));
}
fn u256_bop(ins: &mut Vec<Instruction>, op: BinaryOperator) {
    ins.push(Instruction::BinaryOp(op));
}
fn u256_mask128() -> BigInt {
    (BigInt::one() << 128usize) - BigInt::one()
}
fn u256_bias127() -> BigInt {
    BigInt::one() << 127usize
}
fn u256_mask64() -> BigInt {
    (BigInt::one() << 64usize) - BigInt::one()
}

/// `[a, b] -> [a + b mod 2^256]` over two 128-bit limbs (no 33-byte intermediate).
fn emit_u256_unchecked_add_ir(ctx: &mut LoweringContext, ins: &mut Vec<Instruction>) {
    let s = ctx.u256_scratch_locals(3);
    let (al, bl, lo) = (s[0], s[1], s[2]);
    ins.push(Instruction::StoreLocal(bl));
    ins.push(Instruction::StoreLocal(al));
    // lo = (a & M128) + (b & M128)
    ins.push(Instruction::LoadLocal(al));
    u256_push(ins, u256_mask128());
    u256_bop(ins, BinaryOperator::BitAnd);
    ins.push(Instruction::LoadLocal(bl));
    u256_push(ins, u256_mask128());
    u256_bop(ins, BinaryOperator::BitAnd);
    u256_bop(ins, BinaryOperator::Add);
    ins.push(Instruction::StoreLocal(lo));
    // hi = (a>>128 & M128) + (b>>128 & M128) + (lo>>128)
    emit_u256_hi_limb(ins, al);
    emit_u256_hi_limb(ins, bl);
    u256_bop(ins, BinaryOperator::Add);
    ins.push(Instruction::LoadLocal(lo));
    u256_push(ins, BigInt::from(128u32));
    u256_bop(ins, BinaryOperator::Shr);
    u256_bop(ins, BinaryOperator::Add);
    emit_u256_combine_limbs(ins, lo);
}

/// `[a, b] -> [a - b mod 2^256]` (borrow folded through the limb boundary).
fn emit_u256_unchecked_sub_ir(ctx: &mut LoweringContext, ins: &mut Vec<Instruction>) {
    let s = ctx.u256_scratch_locals(3);
    let (al, bl, lo) = (s[0], s[1], s[2]);
    ins.push(Instruction::StoreLocal(bl));
    ins.push(Instruction::StoreLocal(al));
    // lo = (a & M128) - (b & M128)
    ins.push(Instruction::LoadLocal(al));
    u256_push(ins, u256_mask128());
    u256_bop(ins, BinaryOperator::BitAnd);
    ins.push(Instruction::LoadLocal(bl));
    u256_push(ins, u256_mask128());
    u256_bop(ins, BinaryOperator::BitAnd);
    u256_bop(ins, BinaryOperator::Sub);
    ins.push(Instruction::StoreLocal(lo));
    // hi = (a>>128 & M128) - (b>>128 & M128) + (lo>>128)
    emit_u256_hi_limb(ins, al);
    emit_u256_hi_limb(ins, bl);
    u256_bop(ins, BinaryOperator::Sub);
    ins.push(Instruction::LoadLocal(lo));
    u256_push(ins, BigInt::from(128u32));
    u256_bop(ins, BinaryOperator::Shr);
    u256_bop(ins, BinaryOperator::Add);
    emit_u256_combine_limbs(ins, lo);
}

/// Push `(loc >> 128) & M128` (the unsigned high 128-bit limb of `loc`).
fn emit_u256_hi_limb(ins: &mut Vec<Instruction>, loc: usize) {
    ins.push(Instruction::LoadLocal(loc));
    u256_push(ins, BigInt::from(128u32));
    u256_bop(ins, BinaryOperator::Shr);
    u256_push(ins, u256_mask128());
    u256_bop(ins, BinaryOperator::BitAnd);
}

/// Given a FULL high limb `hi` on the stack and the low sum in `lo`, leave
/// `sign_ext128(hi & M128) << 128 + (lo & M128)` — the 32-byte two's-complement
/// result, where `sign_ext128(x) = (x ^ 2^127) - 2^127`.
fn emit_u256_combine_limbs(ins: &mut Vec<Instruction>, lo: usize) {
    u256_push(ins, u256_mask128());
    u256_bop(ins, BinaryOperator::BitAnd);
    u256_push(ins, u256_bias127());
    u256_bop(ins, BinaryOperator::BitXor);
    u256_push(ins, u256_bias127());
    u256_bop(ins, BinaryOperator::Sub);
    u256_push(ins, BigInt::from(128u32));
    u256_bop(ins, BinaryOperator::Shl);
    ins.push(Instruction::LoadLocal(lo));
    u256_push(ins, u256_mask128());
    u256_bop(ins, BinaryOperator::BitAnd);
    u256_bop(ins, BinaryOperator::Add);
}

/// Run the 64-bit-limb schoolbook columns. Consumes `[a, b]`; leaves limbs
/// `a0..a3 -> s[0..3]`, `b0..b3 -> s[4..7]`, low result limbs `r0..r3 -> s[9..12]`,
/// and the carry into column 4 in `s[8]`. Returns the 15-slot scratch vector.
fn emit_u256_mul_columns_ir(ctx: &mut LoweringContext, ins: &mut Vec<Instruction>) -> Vec<usize> {
    let s = ctx.u256_scratch_locals(15);
    // 0..3 a0..a3, 4..7 b0..b3, 8 acc, 9..12 r0..r3, 13 a, 14 b
    ins.push(Instruction::StoreLocal(s[14]));
    ins.push(Instruction::StoreLocal(s[13]));
    for i in 0..4usize {
        ins.push(Instruction::LoadLocal(s[13]));
        if i > 0 {
            u256_push(ins, BigInt::from(64u32 * i as u32));
            u256_bop(ins, BinaryOperator::Shr);
        }
        u256_push(ins, u256_mask64());
        u256_bop(ins, BinaryOperator::BitAnd);
        ins.push(Instruction::StoreLocal(s[i]));
    }
    for j in 0..4usize {
        ins.push(Instruction::LoadLocal(s[14]));
        if j > 0 {
            u256_push(ins, BigInt::from(64u32 * j as u32));
            u256_bop(ins, BinaryOperator::Shr);
        }
        u256_push(ins, u256_mask64());
        u256_bop(ins, BinaryOperator::BitAnd);
        ins.push(Instruction::StoreLocal(s[4 + j]));
    }
    u256_push(ins, BigInt::zero());
    ins.push(Instruction::StoreLocal(s[8]));
    for k in 0..4usize {
        ins.push(Instruction::LoadLocal(s[8]));
        for i in 0..=k {
            let j = k - i;
            ins.push(Instruction::LoadLocal(s[i]));
            ins.push(Instruction::LoadLocal(s[4 + j]));
            u256_bop(ins, BinaryOperator::Mul);
            u256_bop(ins, BinaryOperator::Add);
        }
        ins.push(Instruction::Dup);
        u256_push(ins, u256_mask64());
        u256_bop(ins, BinaryOperator::BitAnd);
        ins.push(Instruction::StoreLocal(s[9 + k]));
        u256_push(ins, BigInt::from(64u32));
        u256_bop(ins, BinaryOperator::Shr);
        ins.push(Instruction::StoreLocal(s[8]));
    }
    s
}

/// Build the 32-byte two's-complement result from `r0..r3` (`s[9..12]`):
/// `sign_ext128(r2 + (r3<<64)) << 128 + (r0 + (r1<<64))`. Reuses `s[13]`.
fn emit_u256_mul_build_result_ir(ins: &mut Vec<Instruction>, s: &[usize]) {
    // lo128 = r0 + (r1 << 64) -> reuse s[13]
    ins.push(Instruction::LoadLocal(s[9]));
    ins.push(Instruction::LoadLocal(s[10]));
    u256_push(ins, BigInt::from(64u32));
    u256_bop(ins, BinaryOperator::Shl);
    u256_bop(ins, BinaryOperator::Add);
    ins.push(Instruction::StoreLocal(s[13]));
    // hi128 = r2 + (r3 << 64)
    ins.push(Instruction::LoadLocal(s[11]));
    ins.push(Instruction::LoadLocal(s[12]));
    u256_push(ins, BigInt::from(64u32));
    u256_bop(ins, BinaryOperator::Shl);
    u256_bop(ins, BinaryOperator::Add);
    // result = sign_ext128(hi128) << 128 + lo128
    u256_push(ins, u256_bias127());
    u256_bop(ins, BinaryOperator::BitXor);
    u256_push(ins, u256_bias127());
    u256_bop(ins, BinaryOperator::Sub);
    u256_push(ins, BigInt::from(128u32));
    u256_bop(ins, BinaryOperator::Shl);
    ins.push(Instruction::LoadLocal(s[13]));
    u256_bop(ins, BinaryOperator::Add);
}

/// `[a, b] -> [a * b mod 2^256]` via 64-bit-limb schoolbook (low 256 bits).
fn emit_u256_unchecked_mul_ir(ctx: &mut LoweringContext, ins: &mut Vec<Instruction>) {
    let s = emit_u256_mul_columns_ir(ctx, ins);
    emit_u256_mul_build_result_ir(ins, &s);
}

/// Conformant uint256 CHECKED `add`/`sub`/`mul` for operands `[a, b]`. Panics
/// (0x11) on unsigned overflow/underflow. `JumpIf { target }` branches when the
/// popped condition is FALSE, so each guard pushes the OVERFLOW predicate and
/// jumps PAST the panic when it is false.
fn emit_u256_checked_arith(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    operator: BinaryOperator,
) {
    match operator {
        BinaryOperator::Add => {
            // result = a + b (mod 2^256); overflow iff result <u a.
            emit_u256_unchecked_add_ir(ctx, instructions); // [result], scratch s[0]=a
            let s = ctx.u256_scratch_locals(3);
            instructions.push(Instruction::StoreLocal(s[2])); // res (reuse lo slot)
            instructions.push(Instruction::LoadLocal(s[2]));
            instructions.push(Instruction::LoadLocal(s[0])); // a
            emit_u256_unsigned_compare(instructions, BinaryOperator::Lt); // [res <u a]
            let done = ctx.next_label();
            instructions.push(Instruction::JumpIf { target: done });
            emit_panic(0x11, instructions);
            instructions.push(Instruction::Label(done));
            instructions.push(Instruction::LoadLocal(s[2]));
        }
        BinaryOperator::Sub => {
            // underflow iff a <u b; else result = a - b.
            let s = ctx.u256_scratch_locals(3);
            instructions.push(Instruction::StoreLocal(s[1])); // b
            instructions.push(Instruction::StoreLocal(s[0])); // a
            instructions.push(Instruction::LoadLocal(s[0]));
            instructions.push(Instruction::LoadLocal(s[1]));
            emit_u256_unsigned_compare(instructions, BinaryOperator::Lt); // [a <u b]
            let safe = ctx.next_label();
            instructions.push(Instruction::JumpIf { target: safe });
            emit_panic(0x11, instructions);
            instructions.push(Instruction::Label(safe));
            instructions.push(Instruction::LoadLocal(s[0]));
            instructions.push(Instruction::LoadLocal(s[1]));
            emit_u256_unchecked_sub_ir(ctx, instructions); // [result]
        }
        BinaryOperator::Mul => {
            // overflow iff any high-column term or the column-3 carry is non-zero.
            let s = emit_u256_mul_columns_ir(ctx, instructions);
            instructions.push(Instruction::LoadLocal(s[8])); // acc (carry into col 4)
            for (i, j) in [(1usize, 3usize), (2, 2), (3, 1), (2, 3), (3, 2), (3, 3)] {
                instructions.push(Instruction::LoadLocal(s[i]));
                instructions.push(Instruction::LoadLocal(s[4 + j]));
                u256_bop(instructions, BinaryOperator::Mul);
                u256_bop(instructions, BinaryOperator::Add);
            }
            // [high]; overflow iff high != 0.
            let no_overflow = ctx.next_label();
            instructions.push(Instruction::JumpIf { target: no_overflow }); // jumps if high == 0
            emit_panic(0x11, instructions);
            instructions.push(Instruction::Label(no_overflow));
            emit_u256_mul_build_result_ir(instructions, &s);
        }
        _ => unreachable!("emit_u256_checked_arith only handles Add/Sub/Mul"),
    }
}

/// Emit the conformant uint256 CHECKED `add`/`sub`/`mul`: compute the unsigned
/// result with the limb routines, and panic (0x11) on unsigned overflow/underflow
/// detected via the carry/borrow signal (NOT the old `GetSize > 32` heuristic,
/// which a two's-complement wrap defeats).
fn emit_checked_arith_guard(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    operator: BinaryOperator,
) {
    if matches!(
        operator,
        BinaryOperator::Add | BinaryOperator::Sub | BinaryOperator::Mul
    ) {
        emit_u256_checked_arith(ctx, instructions, operator);
        return;
    }
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

    match operator {
        BinaryOperator::Add => {
            // Compute result = lhs + rhs.
            instructions.push(Instruction::LoadLocal(lhs_local));
            instructions.push(Instruction::LoadLocal(rhs_local));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(result_local));

            // Overflow guard: the runtime adds at full BigInt width and stores
            // the raw result as a signed-LE ByteArray via
            // `bigint_to_stack_item`. For values > 2^256-1 the ByteArray exceeds
            // 32 bytes. We detect overflow by checking `GetSize(result) > 32`.
            // This avoids pushing a 33-byte UINT256_MAX literal (which real
            // NeoVM rejects — Integer max is 32 bytes).
            instructions.push(Instruction::LoadLocal(result_local));
            instructions.push(Instruction::Convert { target: ConvertTarget::ByteArray });
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(32u64))));
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

            // Overflow guard: same size-check approach as Add.
            instructions.push(Instruction::LoadLocal(result_local));
            instructions.push(Instruction::Convert { target: ConvertTarget::ByteArray });
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(32u64))));
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

/// Truncate the top-of-stack integer to `bits` low bits (unsigned): `& (2^bits-1)`.
/// Used for `unchecked` narrow Add/Sub/Mul and narrow `<<` results, which wrap
/// mod 2^bits rather than panicking.
fn emit_truncate_narrow_unsigned(instructions: &mut Vec<Instruction>, bits: u16) {
    let mask = (BigInt::one() << bits as usize) - BigInt::one();
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(mask)));
    instructions.push(Instruction::BinaryOp(BinaryOperator::BitAnd));
}

/// Truncate the top-of-stack integer to a signed `bits`-wide value: mask to the
/// low `bits` bits, then two's-complement sign-extend (subtract 2^bits when the
/// sign bit is set). Mirrors the `intN(..)` cast lowering in
/// `src/ir/expressions/calls/type_constructors.rs`.
fn emit_truncate_narrow_signed(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    bits: u16,
) {
    let bits = bits as usize;
    let mask = (BigInt::one() << bits) - BigInt::one();
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(mask)));
    instructions.push(Instruction::BinaryOp(BinaryOperator::BitAnd));

    let tmp_id = ctx.next_label();
    let value_local = ctx.allocate_local(format!("__narrow_trunc_{tmp_id}"), None);
    instructions.push(Instruction::StoreLocal(value_local));

    let sign_bit = BigInt::one() << (bits.saturating_sub(1));
    let modulus = BigInt::one() << bits;
    let positive_label = ctx.next_label();
    let end_label = ctx.next_label();

    // if value < sign_bit -> already positive (JumpIf branches when false).
    instructions.push(Instruction::LoadLocal(value_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(sign_bit)));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Ge));
    instructions.push(Instruction::JumpIf {
        target: positive_label,
    });

    // negative: value - 2^bits
    instructions.push(Instruction::LoadLocal(value_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(modulus)));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Sub));
    instructions.push(Instruction::Jump { target: end_label });

    // positive: value
    instructions.push(Instruction::Label(positive_label));
    instructions.push(Instruction::LoadLocal(value_local));

    instructions.push(Instruction::Label(end_label));
}

/// `unchecked` narrow unsigned (uintN, N<256) Add/Sub/Mul truncation gate. Same
/// preconditions as the checked narrow guard but for `unchecked` blocks, where
/// the result wraps mod 2^N instead of panicking. Returns the bit width.
fn should_truncate_unchecked_narrow_u(
    left: &Expression,
    right: &Expression,
    ctx: &LoweringContext,
    operator: BinaryOperator,
) -> Option<u16> {
    if !ctx.in_unchecked_block() {
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
    if is_typed_uint256(left, ctx) || is_typed_uint256(right, ctx) {
        return None;
    }
    if is_int256_operand(left, ctx) || is_int256_operand(right, ctx) {
        return None;
    }
    narrow_unsigned_bits(left, right, ctx)
}

/// `unchecked` narrow signed (intN, N<256) Add/Sub/Mul truncation gate.
fn should_truncate_unchecked_narrow_i(
    left: &Expression,
    right: &Expression,
    ctx: &LoweringContext,
    operator: BinaryOperator,
) -> Option<u16> {
    if !ctx.in_unchecked_block() {
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
    if is_typed_uint256(left, ctx) || is_typed_uint256(right, ctx) {
        return None;
    }
    if is_int256_operand(left, ctx) || is_int256_operand(right, ctx) {
        return None;
    }
    narrow_signed_bits(left, right, ctx)
}

/// In Solidity the result of `a << b` has the type of the left operand and is
/// truncated to that width (shift overflow never panics, in checked OR unchecked
/// mode). The runtime/optimizer don't wrap at narrow widths, so `uint8(200) << 1`
/// leaves `400` on the stack instead of `144`. Returns `(bits, signed)` of the
/// left operand when it is a narrow integer (N<256); uint256/int256 are already
/// clamped to 256 bits by the runtime SHL path.
fn shl_narrow_truncation(
    left: &Expression,
    ctx: &LoweringContext,
    operator: BinaryOperator,
) -> Option<(u16, bool)> {
    if !matches!(operator, BinaryOperator::Shl) {
        return None;
    }
    match infer_type_from_expression(left, ctx) {
        Some(ValueType::Integer { signed, bits }) if matches!(bits, 8 | 16 | 32 | 64 | 128) => {
            Some((bits, signed))
        }
        _ => None,
    }
}

/// Emit `<op>` for two operands already on the stack (`[.., lhs, rhs]`), applying
/// the full Solidity-0.8 checked-arithmetic / unchecked-truncation ladder:
/// uint256, int256, narrow uintN, narrow intN overflow guards (checked mode);
/// mod-2^256 / mod-2^N truncation (`unchecked` mode); narrow `<<` width
/// truncation; plain op otherwise. Shared by `lower_binary_expr` and the
/// compound-assignment / ++/-- paths so `x <op>= y`, `x++`, `--x` are
/// byte-for-byte consistent with `x = x <op> y`. The gate predicates inspect the
/// Emit an UNSIGNED 256-bit comparison for operands already on the stack as
/// `[.., a, b]`. Uses the order-preserving map `x -> x ^ 2^255`, after which a
/// native (signed) compare yields the unsigned result. `2^255` is pushed as a
/// `uint256` literal, which lowers to the 32-byte two's-complement sign bit.
fn emit_u256_unsigned_compare(instructions: &mut Vec<Instruction>, operator: BinaryOperator) {
    let sign_bit: BigInt = BigInt::one() << 255usize; // 2^255
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(sign_bit.clone())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::BitXor)); // [a, b^S]
    instructions.push(Instruction::Swap); // [b^S, a]
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(sign_bit)));
    instructions.push(Instruction::BinaryOp(BinaryOperator::BitXor)); // [b^S, a^S]
    instructions.push(Instruction::Swap); // [a^S, b^S]
    instructions.push(Instruction::BinaryOp(operator)); // (a^S) <s (b^S) == a <u b
}

/// Emit a LOGICAL (zero-filling) uint256 right shift for operands `[a, n]`
/// (n on top). Native NeoVM SHR is arithmetic, so for a uint256 `a >= 2^255`
/// (stored as a 32-byte two's-complement word) the sign bit propagates. Solidity
/// `>>` on an unsigned type is logical, reproduced as:
///   n == 0  ->  a
///   n >= 1  ->  ((a >>arith 1) & (2^255-1)) >>arith (n-1)
/// The `& (2^255-1)` clears the bit the first arithmetic shift pushed into
/// position 255, turning the whole sequence into a zero-fill. (Mirrors the
/// bytecode-level `emit_uint256_logical_shr` in cli/bytecode/uint256_ops.rs.)
/// Uses scratch slots s[0..1]; it performs only native shift/and/sub ops (no
/// nested limb routines), so it cannot collide with an in-flight u256 op.
fn emit_u256_logical_shr_ir(ctx: &mut LoweringContext, instructions: &mut Vec<Instruction>) {
    let scratch = ctx.u256_scratch_locals(2);
    let n_local = scratch[0];
    let a_local = scratch[1];
    instructions.push(Instruction::StoreLocal(n_local)); // pop n
    instructions.push(Instruction::StoreLocal(a_local)); // pop a

    let nonzero_label = ctx.next_label();
    let end_label = ctx.next_label();

    // if n == 0 -> result = a
    instructions.push(Instruction::LoadLocal(n_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
    // JumpIf -> JMPIFNOT: jump to the n>=1 path when (n == 0) is FALSE.
    instructions.push(Instruction::JumpIf {
        target: nonzero_label,
    });
    instructions.push(Instruction::LoadLocal(a_local));
    instructions.push(Instruction::Jump { target: end_label });

    instructions.push(Instruction::Label(nonzero_label));
    let max_int256: BigInt = (BigInt::one() << 255usize) - BigInt::one(); // 2^255 - 1
    instructions.push(Instruction::LoadLocal(a_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Shr)); // a >>arith 1
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(max_int256)));
    instructions.push(Instruction::BinaryOp(BinaryOperator::BitAnd)); // logical (a>>1)
    instructions.push(Instruction::LoadLocal(n_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Sub)); // n - 1
    instructions.push(Instruction::BinaryOp(BinaryOperator::Shr)); // >> (n-1)
    instructions.push(Instruction::Label(end_label));
}

// Emit unsigned `a / b` (or `a % b` if `want_remainder`) for uint256 operands
// `[a, b]`. Native NeoVM DIV/MOD are signed, so they are wrong for operands at
// or above 2^255. Reduction (Hacker's Delight 9-3): when the divisor is at or
// above 2^255 the quotient is 0 or 1 by an unsigned compare; otherwise reduce to
// one signed DIV/MOD on the provably-non-negative `(a>>1, b)` and correct by one
// step. The limb-unsafe steps (`2t`, `2*rem`, `r-b`, `a-b`) reuse the inline
// add/sub helpers (which use scratch slots `s[0..3]`), so divmod keeps its own
// state in `s[8..15]`. Caller guarantees `b != 0` (div/mod-by-zero panics upstream).
fn emit_u256_divmod_ir(
    ctx: &mut LoweringContext,
    ins: &mut Vec<Instruction>,
    want_remainder: bool,
) {
    let s = ctx.u256_scratch_locals(15);
    let (a, b, q, r, m, t, rem) = (s[8], s[9], s[10], s[11], s[12], s[13], s[14]);
    let max_int256 = (BigInt::one() << 255usize) - BigInt::one();
    ins.push(Instruction::StoreLocal(b));
    ins.push(Instruction::StoreLocal(a));

    let big_b = ctx.next_label();
    let done = ctx.next_label();
    // jump to big_b when b < 0 (i.e. b >= 2^255 unsigned) == NOT (b >= 0).
    ins.push(Instruction::LoadLocal(b));
    u256_push(ins, BigInt::zero());
    u256_bop(ins, BinaryOperator::Ge);
    ins.push(Instruction::JumpIf { target: big_b });

    // ---- small divisor: b in [1, 2^255) ----
    // m = (a >>arith 1) & (2^255-1)   [logical shift right by 1]
    ins.push(Instruction::LoadLocal(a));
    u256_push(ins, BigInt::one());
    u256_bop(ins, BinaryOperator::Shr);
    u256_push(ins, max_int256.clone());
    u256_bop(ins, BinaryOperator::BitAnd);
    ins.push(Instruction::StoreLocal(m));
    // t = m / b ; rem = m % b   (both non-negative -> signed == unsigned)
    ins.push(Instruction::LoadLocal(m));
    ins.push(Instruction::LoadLocal(b));
    u256_bop(ins, BinaryOperator::Div);
    ins.push(Instruction::StoreLocal(t));
    ins.push(Instruction::LoadLocal(m));
    ins.push(Instruction::LoadLocal(b));
    u256_bop(ins, BinaryOperator::Mod);
    ins.push(Instruction::StoreLocal(rem));
    // q = 2t
    ins.push(Instruction::LoadLocal(t));
    ins.push(Instruction::LoadLocal(t));
    emit_u256_unchecked_add_ir(ctx, ins);
    ins.push(Instruction::StoreLocal(q));
    // r = 2*rem + (a & 1)
    ins.push(Instruction::LoadLocal(rem));
    ins.push(Instruction::LoadLocal(rem));
    emit_u256_unchecked_add_ir(ctx, ins);
    ins.push(Instruction::LoadLocal(a));
    u256_push(ins, BigInt::one());
    u256_bop(ins, BinaryOperator::BitAnd);
    u256_bop(ins, BinaryOperator::Add);
    ins.push(Instruction::StoreLocal(r));
    // if r >=u b: q = q+1; r = r-b
    ins.push(Instruction::LoadLocal(r));
    ins.push(Instruction::LoadLocal(b));
    emit_u256_unsigned_compare(ins, BinaryOperator::Ge);
    let skip_corr = ctx.next_label();
    ins.push(Instruction::JumpIf { target: skip_corr });
    ins.push(Instruction::LoadLocal(q));
    u256_push(ins, BigInt::one());
    u256_bop(ins, BinaryOperator::Add);
    ins.push(Instruction::StoreLocal(q));
    ins.push(Instruction::LoadLocal(r));
    ins.push(Instruction::LoadLocal(b));
    emit_u256_unchecked_sub_ir(ctx, ins);
    ins.push(Instruction::StoreLocal(r));
    ins.push(Instruction::Label(skip_corr));
    ins.push(Instruction::Jump { target: done });

    // ---- big divisor: b >= 2^255 ----
    ins.push(Instruction::Label(big_b));
    // q = (a >=u b) ? 1 : 0
    ins.push(Instruction::LoadLocal(a));
    ins.push(Instruction::LoadLocal(b));
    emit_u256_unsigned_compare(ins, BinaryOperator::Ge);
    ins.push(Instruction::StoreLocal(q));
    // r = q == 1 ? a - b : a
    ins.push(Instruction::LoadLocal(q));
    let q_zero = ctx.next_label();
    let big_done = ctx.next_label();
    ins.push(Instruction::JumpIf { target: q_zero });
    ins.push(Instruction::LoadLocal(a));
    ins.push(Instruction::LoadLocal(b));
    emit_u256_unchecked_sub_ir(ctx, ins);
    ins.push(Instruction::StoreLocal(r));
    ins.push(Instruction::Jump { target: big_done });
    ins.push(Instruction::Label(q_zero));
    ins.push(Instruction::LoadLocal(a));
    ins.push(Instruction::StoreLocal(r));
    ins.push(Instruction::Label(big_done));

    ins.push(Instruction::Label(done));
    ins.push(Instruction::LoadLocal(if want_remainder { r } else { q }));
}

/// operand *expressions* (types), not the stack, so this may run after the
/// operands have been lowered.
fn emit_arith_with_overflow_ladder(
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
    // overflows; int256 / i64 min/-1 are caught inside the runtime divmod, so
    // this covers only the narrow signed widths the runtime leaves un-trapped.
    let div_narrow_i = if matches!(operator, BinaryOperator::Div) && is_narrow_result(left, right, ctx)
    {
        narrow_signed_bits(left, right, ctx)
    } else {
        None
    };

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
        // `unchecked { uintN: a <op> b }` wraps mod 2^N.
        instructions.push(Instruction::BinaryOp(operator));
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
    } else {
        instructions.push(Instruction::BinaryOp(operator));
    }
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
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
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
