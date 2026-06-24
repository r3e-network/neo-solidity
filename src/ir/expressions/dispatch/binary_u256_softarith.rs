// ---- Inline software uint256 routines (32-byte two's-complement) -------------
//
// NeoVM integers are signed two's-complement capped at 32 bytes, so a `uint256`
// value >= 2^255 is stored as its negative-looking two's-complement and a native
// ADD/SUB/MUL can both fault (a 33-byte intermediate) and wrap in a way the
// `GetSize > 32` overflow heuristic cannot see. These helpers compute the
// UNSIGNED result over 128-bit (add/sub) or 64-bit (mul) limbs so no intermediate
// ever exceeds 32 bytes — the same routines validated in a now-removed
// bytecode-level prototype (`cli/bytecode/uint256_ops.rs`; see git history)
// against a faithful reference VM — emitted here as IR over a shared
// scratch-local pool.

pub(crate) fn u256_push(ins: &mut Vec<Instruction>, v: BigInt) {
    ins.push(Instruction::PushLiteral(LiteralValue::Integer(v)));
}
pub(crate) fn u256_bop(ins: &mut Vec<Instruction>, op: BinaryOperator) {
    ins.push(Instruction::BinaryOp(op));
}
pub(crate) fn u256_mask128() -> BigInt {
    (BigInt::one() << 128usize) - BigInt::one()
}
pub(crate) fn u256_bias127() -> BigInt {
    BigInt::one() << 127usize
}
pub(crate) fn u256_mask64() -> BigInt {
    (BigInt::one() << 64usize) - BigInt::one()
}

/// `[a, b] -> [a + b mod 2^256]` over two 128-bit limbs (no 33-byte intermediate).
pub(crate) fn emit_u256_unchecked_add_ir(ctx: &mut LoweringContext, ins: &mut Vec<Instruction>) {
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
pub(crate) fn emit_u256_unchecked_sub_ir(ctx: &mut LoweringContext, ins: &mut Vec<Instruction>) {
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
pub(crate) fn emit_u256_hi_limb(ins: &mut Vec<Instruction>, loc: usize) {
    ins.push(Instruction::LoadLocal(loc));
    u256_push(ins, BigInt::from(128u32));
    u256_bop(ins, BinaryOperator::Shr);
    u256_push(ins, u256_mask128());
    u256_bop(ins, BinaryOperator::BitAnd);
}

/// Given a FULL high limb `hi` on the stack and the low sum in `lo`, leave
/// `sign_ext128(hi & M128) << 128 + (lo & M128)` — the 32-byte two's-complement
/// result, where `sign_ext128(x) = (x ^ 2^127) - 2^127`.
pub(crate) fn emit_u256_combine_limbs(ins: &mut Vec<Instruction>, lo: usize) {
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
pub(crate) fn emit_u256_mul_columns_ir(ctx: &mut LoweringContext, ins: &mut Vec<Instruction>) -> Vec<usize> {
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
pub(crate) fn emit_u256_mul_build_result_ir(ins: &mut Vec<Instruction>, s: &[usize]) {
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
pub(crate) fn emit_u256_unchecked_mul_ir(ctx: &mut LoweringContext, ins: &mut Vec<Instruction>) {
    let s = emit_u256_mul_columns_ir(ctx, ins);
    emit_u256_mul_build_result_ir(ins, &s);
}

/// Conformant uint256 CHECKED `add`/`sub`/`mul` for operands `[a, b]`. Panics
/// (0x11) on unsigned overflow/underflow. `JumpIf { target }` branches when the
/// popped condition is FALSE, so each guard pushes the OVERFLOW predicate and
/// jumps PAST the panic when it is false.
pub(crate) fn emit_u256_checked_arith(
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
pub(crate) fn emit_u256_unsigned_compare(instructions: &mut Vec<Instruction>, operator: BinaryOperator) {
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
/// position 255, turning the whole sequence into a zero-fill. (Mirrors a
/// bytecode-level `emit_uint256_logical_shr` from the now-removed
/// cli/bytecode/uint256_ops.rs; see git history.)
/// Uses scratch slots s[0..1]; it performs only native shift/and/sub ops (no
/// nested limb routines), so it cannot collide with an in-flight u256 op.
pub(crate) fn emit_u256_logical_shr_ir(ctx: &mut LoweringContext, instructions: &mut Vec<Instruction>) {
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
pub(crate) fn emit_u256_divmod_ir(
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
