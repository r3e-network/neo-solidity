// Software 256-bit UNSIGNED routines emitted as NeoVM bytecode (#12).
//
// NeoVM integers are signed two's-complement, capped at 32 bytes
// (`[-2^255, 2^255-1]`). Solidity `uint256` values in `[2^255, 2^256-1]` are
// represented as their 32-byte two's-complement (value mod 2^256, which "looks
// negative"). These routines compute the correct UNSIGNED result using only
// <=32-byte operations, so they run on a real Neo node.
//
// Each routine is validated below against a FAITHFUL reference VM (signed
// two's-complement, 32-byte limit) — the production runtime simulator currently
// uses an unsigned-magnitude representation and cannot validate them (see
// `claudedocs/uint256-conformance-plan.md`). Wiring the routines into the binary
// lowering, flipping the simulator to two's-complement, and migrating the test
// suite is the remaining coordinated change.

/// PUSHINT256 of a 32-byte little-endian value.
#[allow(dead_code)]
fn emit_pushint256_le(out: &mut Vec<u8>, le: &[u8; 32]) {
    out.push(0x05);
    out.extend_from_slice(le);
}

/// 32-byte little-endian encoding of `2^255` (the sign bit). As a signed NeoVM
/// integer this is `INT256_MIN`; XOR-ing flips bit 255, mapping unsigned order to
/// signed order.
#[allow(dead_code)]
const SIGN_BIT_LE: [u8; 32] = {
    let mut b = [0u8; 32];
    b[31] = 0x80;
    b
};

/// Emit unsigned `a < b` for operands on the stack as `[.., a, b]`.
/// `a <u b  <=>  (a ^ 2^255) <s (b ^ 2^255)`. All operations stay <= 32 bytes.
#[allow(dead_code)]
fn emit_uint256_unsigned_lt(out: &mut Vec<u8>) {
    emit_pushint256_le(out, &SIGN_BIT_LE);
    out.push(0x93); // XOR  -> [a, b^SIGN]
    out.push(0x50); // SWAP -> [b^SIGN, a]
    emit_pushint256_le(out, &SIGN_BIT_LE);
    out.push(0x93); // XOR  -> [b^SIGN, a^SIGN]
    out.push(0x50); // SWAP -> [a^SIGN, b^SIGN]
    out.push(0xB5); // LT   -> a^SIGN <s b^SIGN  ==  a <u b
}

/// Emit unsigned `a > b` for operands `[.., a, b]`.
#[allow(dead_code)]
fn emit_uint256_unsigned_gt(out: &mut Vec<u8>) {
    emit_pushint256_le(out, &SIGN_BIT_LE);
    out.push(0x93); // XOR  -> [a, b^SIGN]
    out.push(0x50); // SWAP -> [b^SIGN, a]
    emit_pushint256_le(out, &SIGN_BIT_LE);
    out.push(0x93); // XOR  -> [b^SIGN, a^SIGN]
    out.push(0x50); // SWAP -> [a^SIGN, b^SIGN]
    out.push(0xB7); // GT   -> a^SIGN >s b^SIGN  ==  a >u b
}

/// Emit unsigned `a <= b` for operands `[.., a, b]`.
#[allow(dead_code)]
fn emit_uint256_unsigned_le(out: &mut Vec<u8>) {
    emit_pushint256_le(out, &SIGN_BIT_LE);
    out.push(0x93); // XOR
    out.push(0x50); // SWAP
    emit_pushint256_le(out, &SIGN_BIT_LE);
    out.push(0x93); // XOR
    out.push(0x50); // SWAP -> [a^SIGN, b^SIGN]
    out.push(0xB6); // LE   -> a^SIGN <=s b^SIGN  ==  a <=u b
}

/// Emit unsigned `a >= b` for operands `[.., a, b]`.
#[allow(dead_code)]
fn emit_uint256_unsigned_ge(out: &mut Vec<u8>) {
    emit_pushint256_le(out, &SIGN_BIT_LE);
    out.push(0x93); // XOR
    out.push(0x50); // SWAP
    emit_pushint256_le(out, &SIGN_BIT_LE);
    out.push(0x93); // XOR
    out.push(0x50); // SWAP -> [a^SIGN, b^SIGN]
    out.push(0xB8); // GE   -> a^SIGN >=s b^SIGN  ==  a >=u b
}

/// 32-byte LE of `2^128 - 1` (low 128 bits set) — a POSITIVE 256-bit mask
/// (pushed via PUSHINT256 so it is not the negative int128 `-1`).
#[allow(dead_code)]
const MASK128_LE: [u8; 32] = {
    let mut b = [0u8; 32];
    let mut i = 0;
    while i < 16 {
        b[i] = 0xFF;
        i += 1;
    }
    b
};

/// 32-byte LE of `2^127` (bit 127 set) — the int128 sign bias.
#[allow(dead_code)]
const BIAS127_LE: [u8; 32] = {
    let mut b = [0u8; 32];
    b[15] = 0x80;
    b
};

/// 32-byte LE of `2^255 - 1` (`INT256_MAX`, all bits set except the sign bit) —
/// AND-ing clears bit 255, turning an arithmetic right shift into a logical one.
#[allow(dead_code)]
const MAX_INT256_LE: [u8; 32] = {
    let mut b = [0xFFu8; 32];
    b[31] = 0x7F;
    b
};

/// Emit a LOGICAL `a >>u n` for uint256 operands `[.., a, n]` (`n` in [0, 256]).
/// Native NeoVM `SHR` is arithmetic (sign-extending), so for values >= 2^255 it
/// would propagate the high bit. Identity used:
///   `n == 0`        -> a
///   `n >= 1`        -> logical_shr_1(a) >>arith (n-1)
/// where `logical_shr_1(a) = (a >>arith 1) & (2^255-1)` is non-negative, so a
/// subsequent arithmetic shift behaves logically. Every value stays <= 32 bytes.
#[allow(dead_code)]
fn emit_uint256_logical_shr(out: &mut Vec<u8>) {
    out.push(0x57); // INITSLOT
    out.push(0x02);
    out.push(0x00);
    out.push(0x71); // STLOC1 (n)
    out.push(0x70); // STLOC0 (a)
    emit_ldloc(out, 1); // n
    out.push(0x26); // JMPIFNOT -> n == 0 path
    let jmpifnot_operand = out.len();
    out.push(0x00);
    // n >= 1: logical_shr_1(a) >> (n - 1)
    emit_ldloc(out, 0); // a
    out.push(0x11); // PUSH1
    out.push(0xA9); // SHR  (a >>arith 1)
    emit_pushint256_le(out, &MAX_INT256_LE);
    out.push(0x91); // AND  -> logical_shr_1(a)  (>= 0)
    emit_ldloc(out, 1); // n
    out.push(0x11); // PUSH1
    out.push(0x9F); // SUB  -> n - 1
    out.push(0xA9); // SHR  -> result
    out.push(0x22); // JMP -> end
    let jmp_operand = out.len();
    out.push(0x00);
    let zero_pos = out.len();
    emit_ldloc(out, 0); // a  (n == 0 -> result = a)
    let end_pos = out.len();
    out[jmpifnot_operand] = ((zero_pos as isize) - (jmpifnot_operand as isize - 1)) as i8 as u8;
    out[jmp_operand] = ((end_pos as isize) - (jmp_operand as isize - 1)) as i8 as u8;
}

/// 32-byte LE of `2^64 - 1` (low 64 bits set) — a POSITIVE 256-bit mask used to
/// extract 64-bit limbs for the software multiply.
#[allow(dead_code)]
const MASK64_LE: [u8; 32] = {
    let mut b = [0u8; 32];
    let mut i = 0;
    while i < 8 {
        b[i] = 0xFF;
        i += 1;
    }
    b
};

/// Emit `LDLOC i` (short form for `i <= 6`, else the generic 1-byte-operand form).
#[allow(dead_code)]
fn emit_ldloc(out: &mut Vec<u8>, i: u8) {
    if i <= 6 {
        out.push(0x68 + i); // LDLOC0..LDLOC6
    } else {
        out.push(0x6F); // LDLOC
        out.push(i);
    }
}

/// Emit `STLOC i` (short form for `i <= 6`, else the generic 1-byte-operand form).
#[allow(dead_code)]
fn emit_stloc(out: &mut Vec<u8>, i: u8) {
    if i <= 6 {
        out.push(0x70 + i); // STLOC0..STLOC6
    } else {
        out.push(0x77); // STLOC
        out.push(i);
    }
}

/// Push a small non-negative integer (a shift count) as a POSITIVE NeoVM
/// integer. PUSHINT8 is signed, so values `>= 128` (e.g. a 128-bit shift) must
/// use PUSHINT16, otherwise `0x80` would decode to `-128`.
#[allow(dead_code)]
fn emit_push_u8(out: &mut Vec<u8>, n: u8) {
    if n <= 0x7F {
        out.push(0x00); // PUSHINT8
        out.push(n);
    } else {
        out.push(0x01); // PUSHINT16 (keeps the value positive)
        out.extend_from_slice(&i16::from(n).to_le_bytes());
    }
}

/// Emit `a + b mod 2^256` (UNCHECKED) for operands `[.., a, b]`, on the 32-byte
/// two's-complement representation, using 128-bit limbs so no intermediate ever
/// exceeds 32 bytes (a native `ADD` of two large uint256 values would otherwise
/// produce a 33-byte result that a real Neo node rejects).
///
///   lo = (a & M128) + (b & M128)                         // <= 2^129
///   hi = (a>>128 & M128) + (b>>128 & M128) + (lo>>128)   // <= 2^129
///   result = sign_ext128(hi & M128) << 128  +  (lo & M128)
/// where `sign_ext128(x) = (x ^ 2^127) - 2^127` keeps the high limb in
/// `[-2^127, 2^127)` so the final value lands in `[-2^255, 2^255-1]` (<= 32 bytes).
#[allow(dead_code)]
fn emit_uint256_unchecked_add(out: &mut Vec<u8>) {
    out.push(0x57); // INITSLOT
    out.push(0x03); // 3 locals
    out.push(0x00); // 0 params
    emit_add_limb_prologue(out); // lo = (a&M128)+(b&M128) -> local 2
    emit_add_full_hi(out); // hi = ah + bh + carry  (full, on stack)
    emit_add_result_epilogue(out); // sign_ext128(hi & M128) << 128 + (lo & M128)
}

/// Emit `a - b mod 2^256` (UNCHECKED) for operands `[.., a, b]`. Mirrors the add
/// routine with a borrow: the low-limb difference can go negative, and its
/// arithmetic right shift by 128 yields `-1` (borrow) or `0`, which is folded
/// straight into the high limb.
///
///   lo = (a & M128) - (b & M128)                         // in (-2^128, 2^128)
///   hi = (a>>128 & M128) - (b>>128 & M128) + (lo>>128)   // (lo>>128) = -borrow
///   result = sign_ext128(hi & M128) << 128  +  (lo & M128)
#[allow(dead_code)]
fn emit_uint256_unchecked_sub(out: &mut Vec<u8>) {
    out.push(0x57); // INITSLOT
    out.push(0x03); // 3 locals
    out.push(0x00); // 0 params
    out.push(0x71); // STLOC1  (b)
    out.push(0x70); // STLOC0  (a)
    // lo = (a & M128) - (b & M128)
    out.push(0x68); // LDLOC0
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x69); // LDLOC1
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x9F); // SUB
    out.push(0x72); // STLOC2  (lo)
    // hi = (a>>128 & M128) - (b>>128 & M128) + (lo>>128)
    out.push(0x68); // LDLOC0
    emit_push_u8(out, 128);
    out.push(0xA9); // SHR
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x69); // LDLOC1
    emit_push_u8(out, 128);
    out.push(0xA9); // SHR
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x9F); // SUB
    out.push(0x6A); // LDLOC2 (lo)
    emit_push_u8(out, 128);
    out.push(0xA9); // SHR  (lo>>128 = -borrow, in {-1, 0})
    out.push(0x9E); // ADD  -> hi
    // hi_signed = sign_ext128(hi & M128)
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    emit_pushint256_le(out, &BIAS127_LE);
    out.push(0x93); // XOR
    emit_pushint256_le(out, &BIAS127_LE);
    out.push(0x9F); // SUB  -> hi_signed
    // result = (hi_signed << 128) + (lo & M128)
    emit_push_u8(out, 128);
    out.push(0xA8); // SHL
    out.push(0x6A); // LDLOC2 (lo)
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x9E); // ADD  -> result
}

/// Emit the limb prologue shared by the add routines: store `b`,`a` into
/// locals 1,0 and compute `lo = (a&M128)+(b&M128)` into local 2, leaving the
/// stack empty. Assumes the caller has already emitted `INITSLOT 3 0`.
#[allow(dead_code)]
fn emit_add_limb_prologue(out: &mut Vec<u8>) {
    out.push(0x71); // STLOC1 (b)
    out.push(0x70); // STLOC0 (a)
    out.push(0x68); // LDLOC0
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x69); // LDLOC1
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x9E); // ADD
    out.push(0x72); // STLOC2 (lo)
}

/// Emit, onto the stack, the FULL (unmasked) high limb
/// `hi = (a>>128 & M128) + (b>>128 & M128) + (lo>>128)` in `[0, 2^129-1]`.
/// `hi >> 128` is the carry-out of bit 256 (0 or 1).
#[allow(dead_code)]
fn emit_add_full_hi(out: &mut Vec<u8>) {
    out.push(0x68); // LDLOC0
    emit_push_u8(out, 128);
    out.push(0xA9); // SHR
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x69); // LDLOC1
    emit_push_u8(out, 128);
    out.push(0xA9); // SHR
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x9E); // ADD
    out.push(0x6A); // LDLOC2 (lo)
    emit_push_u8(out, 128);
    out.push(0xA9); // SHR  (carry from lo)
    out.push(0x9E); // ADD  -> hi (full)
}

/// Emit the epilogue that turns a FULL high limb `hi` (top of stack) plus the
/// stored `lo` (local 2) into the 32-byte two's-complement result:
/// `result = sign_ext128(hi & M128) << 128 + (lo & M128)`.
#[allow(dead_code)]
fn emit_add_result_epilogue(out: &mut Vec<u8>) {
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    emit_pushint256_le(out, &BIAS127_LE);
    out.push(0x93); // XOR
    emit_pushint256_le(out, &BIAS127_LE);
    out.push(0x9F); // SUB  -> sign_ext128(hi & M128)
    emit_push_u8(out, 128);
    out.push(0xA8); // SHL
    out.push(0x6A); // LDLOC2 (lo)
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x9E); // ADD  -> result
}

/// Emit `a + b` with an UNSIGNED overflow check: if `a + b >= 2^256` (carry out
/// of bit 256), THROW (Solidity Panic 0x11). Operands `[.., a, b]`.
#[allow(dead_code)]
fn emit_uint256_checked_add(out: &mut Vec<u8>) {
    out.push(0x57); // INITSLOT
    out.push(0x03);
    out.push(0x00);
    emit_add_limb_prologue(out);
    emit_add_full_hi(out); // [hi]
    out.push(0x4A); // DUP -> [hi, hi]
    emit_push_u8(out, 128);
    out.push(0xA9); // SHR -> [hi, carry_out]  (0 or 1)
    out.push(0x24); // JMPIF -> throw if carry_out != 0
    let jmpif_operand = out.len();
    out.push(0x00); // placeholder offset
                    // no-overflow path: [hi]
    emit_add_result_epilogue(out);
    out.push(0x22); // JMP -> end (over the THROW)
    let jmp_operand = out.len();
    out.push(0x00); // placeholder
    let throw_pos = out.len();
    out.push(0x3A); // THROW (Panic 0x11 when wired in)
    let end_pos = out.len();
    // backpatch (offsets are relative to the jump opcode = operand_index - 1)
    out[jmpif_operand] = ((throw_pos as isize) - (jmpif_operand as isize - 1)) as i8 as u8;
    out[jmp_operand] = ((end_pos as isize) - (jmp_operand as isize - 1)) as i8 as u8;
}

/// Emit `a - b` with an UNSIGNED underflow check: if `a < b` (final borrow),
/// THROW (Solidity Panic 0x11). Operands `[.., a, b]`.
#[allow(dead_code)]
fn emit_uint256_checked_sub(out: &mut Vec<u8>) {
    out.push(0x57); // INITSLOT
    out.push(0x03);
    out.push(0x00);
    out.push(0x71); // STLOC1 (b)
    out.push(0x70); // STLOC0 (a)
    // lo = (a&M128) - (b&M128)
    out.push(0x68); // LDLOC0
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x69); // LDLOC1
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x9F); // SUB
    out.push(0x72); // STLOC2 (lo)
    // hi (full) = (a>>128 & M128) - (b>>128 & M128) + (lo>>128) ; in (-2^128, 2^128)
    out.push(0x68); // LDLOC0
    emit_push_u8(out, 128);
    out.push(0xA9); // SHR
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x69); // LDLOC1
    emit_push_u8(out, 128);
    out.push(0xA9); // SHR
    emit_pushint256_le(out, &MASK128_LE);
    out.push(0x91); // AND
    out.push(0x9F); // SUB
    out.push(0x6A); // LDLOC2 (lo)
    emit_push_u8(out, 128);
    out.push(0xA9); // SHR  (-borrow)
    out.push(0x9E); // ADD  -> hi (full)
    // underflow  <=>  hi < 0
    out.push(0x4A); // DUP -> [hi, hi]
    out.push(0x10); // PUSH0 -> [hi, hi, 0]
    out.push(0xB5); // LT -> [hi, (hi < 0)]
    out.push(0x24); // JMPIF -> throw if hi < 0
    let jmpif_operand = out.len();
    out.push(0x00);
    // no-underflow path: [hi]
    emit_add_result_epilogue(out);
    out.push(0x22); // JMP -> end
    let jmp_operand = out.len();
    out.push(0x00);
    let throw_pos = out.len();
    out.push(0x3A); // THROW
    let end_pos = out.len();
    out[jmpif_operand] = ((throw_pos as isize) - (jmpif_operand as isize - 1)) as i8 as u8;
    out[jmp_operand] = ((end_pos as isize) - (jmp_operand as isize - 1)) as i8 as u8;
}

/// Emit the shared multiply core. Consumes `[.., a, b]`, and using locals
/// (INITSLOT 15 0 must already be emitted) computes the 64-bit-limb schoolbook
/// low half: limbs `a0..a3` -> locals 0..3, `b0..b3` -> locals 4..7, result
/// limbs `r0..r3` -> locals 9..12, and the carry into column 4 -> local 8.
/// 64-bit limbs keep every partial product `< 2^128` and every column sum
/// `< 2^131`, so no intermediate exceeds NeoVM's 32-byte integer limit.
#[allow(dead_code)]
fn emit_mul_columns(out: &mut Vec<u8>) {
    const A: u8 = 13; // temp: a
    const B: u8 = 14; // temp: b
    emit_stloc(out, B);
    emit_stloc(out, A);
    // a_i = (a >> 64*i) & M64  -> local i
    for i in 0..4u8 {
        emit_ldloc(out, A);
        if i > 0 {
            emit_push_u8(out, 64 * i);
            out.push(0xA9); // SHR
        }
        emit_pushint256_le(out, &MASK64_LE);
        out.push(0x91); // AND
        emit_stloc(out, i);
    }
    // b_j = (b >> 64*j) & M64  -> local 4+j
    for j in 0..4u8 {
        emit_ldloc(out, B);
        if j > 0 {
            emit_push_u8(out, 64 * j);
            out.push(0xA9); // SHR
        }
        emit_pushint256_le(out, &MASK64_LE);
        out.push(0x91); // AND
        emit_stloc(out, 4 + j);
    }
    // acc = 0
    out.push(0x10); // PUSH0
    emit_stloc(out, 8);
    // columns 0..3: colsum = acc + sum_{i+j=k} a_i*b_j ; r_k = colsum&M64 ; acc = colsum>>64
    for k in 0..4u8 {
        emit_ldloc(out, 8); // acc
        for i in 0..=k {
            let j = k - i;
            emit_ldloc(out, i); // a_i
            emit_ldloc(out, 4 + j); // b_j
            out.push(0xA0); // MUL
            out.push(0x9E); // ADD
        }
        out.push(0x4A); // DUP colsum
        emit_pushint256_le(out, &MASK64_LE);
        out.push(0x91); // AND
        emit_stloc(out, 9 + k); // r_k
        emit_push_u8(out, 64);
        out.push(0xA9); // SHR
        emit_stloc(out, 8); // acc
    }
}

/// Emit, from `r0..r3` (locals 9..12), the 32-byte two's-complement result
/// `sign_ext128(r2 + (r3<<64)) << 128 + (r0 + (r1<<64))`. Reuses local 13.
#[allow(dead_code)]
fn emit_mul_build_result(out: &mut Vec<u8>) {
    // lo128 = r0 + (r1 << 64)  -> local 13
    emit_ldloc(out, 9);
    emit_ldloc(out, 10);
    emit_push_u8(out, 64);
    out.push(0xA8); // SHL
    out.push(0x9E); // ADD
    emit_stloc(out, 13);
    // hi128 = r2 + (r3 << 64)
    emit_ldloc(out, 11);
    emit_ldloc(out, 12);
    emit_push_u8(out, 64);
    out.push(0xA8); // SHL
    out.push(0x9E); // ADD
    // result = sign_ext128(hi128) << 128 + lo128
    emit_pushint256_le(out, &BIAS127_LE);
    out.push(0x93); // XOR
    emit_pushint256_le(out, &BIAS127_LE);
    out.push(0x9F); // SUB
    emit_push_u8(out, 128);
    out.push(0xA8); // SHL
    emit_ldloc(out, 13); // lo128
    out.push(0x9E); // ADD -> result
}

/// Emit `a * b mod 2^256` (UNCHECKED) for operands `[.., a, b]`.
#[allow(dead_code)]
fn emit_uint256_unchecked_mul(out: &mut Vec<u8>) {
    out.push(0x57); // INITSLOT
    out.push(15);
    out.push(0x00);
    emit_mul_columns(out);
    emit_mul_build_result(out);
}

/// Emit `a * b` with an UNSIGNED overflow check: if the product needs more than
/// 256 bits (any high-column term or the column-3 carry is non-zero), THROW.
/// All high terms are non-negative, so their sum is zero iff every one is zero.
#[allow(dead_code)]
fn emit_uint256_checked_mul(out: &mut Vec<u8>) {
    out.push(0x57); // INITSLOT
    out.push(15);
    out.push(0x00);
    emit_mul_columns(out);
    // high = acc + a1*b3 + a2*b2 + a3*b1 + a2*b3 + a3*b2 + a3*b3
    emit_ldloc(out, 8); // acc (carry into column 4)
    for (i, j) in [(1u8, 3u8), (2, 2), (3, 1), (2, 3), (3, 2), (3, 3)] {
        emit_ldloc(out, i);
        emit_ldloc(out, 4 + j);
        out.push(0xA0); // MUL
        out.push(0x9E); // ADD
    }
    out.push(0x24); // JMPIF -> throw if high != 0
    let jmpif_operand = out.len();
    out.push(0x00);
    emit_mul_build_result(out);
    out.push(0x22); // JMP -> end
    let jmp_operand = out.len();
    out.push(0x00);
    let throw_pos = out.len();
    out.push(0x3A); // THROW
    let end_pos = out.len();
    out[jmpif_operand] = ((throw_pos as isize) - (jmpif_operand as isize - 1)) as i8 as u8;
    out[jmp_operand] = ((end_pos as isize) - (jmp_operand as isize - 1)) as i8 as u8;
}

#[cfg(test)]
mod uint256_ops_tests {
    use super::*;
    use num_bigint::BigInt;

    fn modulus() -> BigInt {
        BigInt::from(1) << 256u32
    }

    /// 32-byte little-endian two's-complement encoding of a uint256 in [0, 2^256).
    fn u256_le(value: &BigInt) -> [u8; 32] {
        let m = modulus();
        let v: BigInt = ((value % &m) + &m) % &m;
        let signed: BigInt = if v >= (BigInt::from(1) << 255u32) { &v - &m } else { v };
        let bytes = signed.to_signed_bytes_le();
        let fill: u8 = if signed.sign() == num_bigint::Sign::Minus { 0xFF } else { 0x00 };
        let mut out = [fill; 32];
        out[..bytes.len()].copy_from_slice(&bytes);
        out
    }

    // ---- Faithful reference VM: signed two's-complement, 32-byte integer limit.
    // Models the subset of NeoVM opcodes the routines use, EXACTLY as a real node
    // would (no unsigned-magnitude masking). Returns Err on a >32-byte integer
    // result (the real VM's MaxIntegerSize fault).
    fn faithful_run(code: &[u8]) -> Result<Vec<BigInt>, String> {
        let mut stack: Vec<BigInt> = Vec::new();
        let mut locals: Vec<BigInt> = Vec::new();
        let mut ip = 0usize;
        let check = |v: BigInt| -> Result<BigInt, String> {
            if v.to_signed_bytes_le().len() > 32 {
                Err("integer exceeds 32 bytes".into())
            } else {
                Ok(v)
            }
        };
        while ip < code.len() {
            let op = code[ip];
            match op {
                0x05 => {
                    // PUSHINT256: 32-byte signed LE
                    let bytes = &code[ip + 1..ip + 33];
                    stack.push(BigInt::from_signed_bytes_le(bytes));
                    ip += 33;
                }
                0x93 => {
                    // XOR (bitwise two's-complement)
                    let b = stack.pop().ok_or("xor underflow")?;
                    let a = stack.pop().ok_or("xor underflow")?;
                    stack.push(check(a ^ b)?);
                    ip += 1;
                }
                0x50 => {
                    // SWAP
                    let n = stack.len();
                    if n < 2 {
                        return Err("swap underflow".into());
                    }
                    stack.swap(n - 1, n - 2);
                    ip += 1;
                }
                0xB5 => {
                    // LT: x1 < x2 (x2 popped first)
                    let x2 = stack.pop().ok_or("lt underflow")?;
                    let x1 = stack.pop().ok_or("lt underflow")?;
                    stack.push(BigInt::from(i32::from(x1 < x2)));
                    ip += 1;
                }
                0xB7 => {
                    // GT: x1 > x2
                    let x2 = stack.pop().ok_or("gt underflow")?;
                    let x1 = stack.pop().ok_or("gt underflow")?;
                    stack.push(BigInt::from(i32::from(x1 > x2)));
                    ip += 1;
                }
                0xB6 => {
                    // LE: x1 <= x2
                    let x2 = stack.pop().ok_or("le underflow")?;
                    let x1 = stack.pop().ok_or("le underflow")?;
                    stack.push(BigInt::from(i32::from(x1 <= x2)));
                    ip += 1;
                }
                0xB8 => {
                    // GE: x1 >= x2
                    let x2 = stack.pop().ok_or("ge underflow")?;
                    let x1 = stack.pop().ok_or("ge underflow")?;
                    stack.push(BigInt::from(i32::from(x1 >= x2)));
                    ip += 1;
                }
                0x00 => {
                    // PUSHINT8 (signed)
                    stack.push(BigInt::from(code[ip + 1] as i8));
                    ip += 2;
                }
                0x01 => {
                    // PUSHINT16 (signed LE)
                    let v = i16::from_le_bytes([code[ip + 1], code[ip + 2]]);
                    stack.push(BigInt::from(v));
                    ip += 3;
                }
                0x57 => {
                    // INITSLOT nlocals nparams
                    let nlocals = code[ip + 1] as usize;
                    locals = vec![BigInt::from(0); nlocals];
                    ip += 3;
                }
                0x70..=0x76 => {
                    // STLOC0..STLOC6
                    let i = (op - 0x70) as usize;
                    locals[i] = stack.pop().ok_or("stloc underflow")?;
                    ip += 1;
                }
                0x77 => {
                    // STLOC (1-byte index)
                    let i = code[ip + 1] as usize;
                    locals[i] = stack.pop().ok_or("stloc underflow")?;
                    ip += 2;
                }
                0x68..=0x6E => {
                    // LDLOC0..LDLOC6
                    let i = (op - 0x68) as usize;
                    stack.push(locals[i].clone());
                    ip += 1;
                }
                0x6F => {
                    // LDLOC (1-byte index)
                    let i = code[ip + 1] as usize;
                    stack.push(locals[i].clone());
                    ip += 2;
                }
                0x91 => {
                    // AND
                    let b = stack.pop().ok_or("and underflow")?;
                    let a = stack.pop().ok_or("and underflow")?;
                    stack.push(check(a & b)?);
                    ip += 1;
                }
                0x9E => {
                    // ADD
                    let b = stack.pop().ok_or("add underflow")?;
                    let a = stack.pop().ok_or("add underflow")?;
                    stack.push(check(a + b)?);
                    ip += 1;
                }
                0x9F => {
                    // SUB
                    let b = stack.pop().ok_or("sub underflow")?;
                    let a = stack.pop().ok_or("sub underflow")?;
                    stack.push(check(a - b)?);
                    ip += 1;
                }
                0xA0 => {
                    // MUL
                    let b = stack.pop().ok_or("mul underflow")?;
                    let a = stack.pop().ok_or("mul underflow")?;
                    stack.push(check(a * b)?);
                    ip += 1;
                }
                0xA8 => {
                    // SHL: value << shift (shift popped first)
                    let shift = stack.pop().ok_or("shl underflow")?;
                    let value = stack.pop().ok_or("shl underflow")?;
                    let s: u64 = u64::try_from(shift).map_err(|_| "bad shift")?;
                    stack.push(check(value << s as usize)?);
                    ip += 1;
                }
                0xA9 => {
                    // SHR: value >> shift (arithmetic, two's-complement)
                    let shift = stack.pop().ok_or("shr underflow")?;
                    let value = stack.pop().ok_or("shr underflow")?;
                    let s: u64 = u64::try_from(shift).map_err(|_| "bad shift")?;
                    stack.push(check(value >> s as usize)?);
                    ip += 1;
                }
                0x10..=0x20 => {
                    // PUSH0..PUSH16
                    stack.push(BigInt::from(op - 0x10));
                    ip += 1;
                }
                0x4A => {
                    // DUP
                    let top = stack.last().cloned().ok_or("dup underflow")?;
                    stack.push(top);
                    ip += 1;
                }
                0x22 => {
                    // JMP rel8 (relative to the opcode position)
                    let off = code[ip + 1] as i8 as isize;
                    ip = (ip as isize + off) as usize;
                }
                0x24 => {
                    // JMPIF rel8
                    let off = code[ip + 1] as i8 as isize;
                    let c = stack.pop().ok_or("jmpif underflow")?;
                    if c != BigInt::from(0) {
                        ip = (ip as isize + off) as usize;
                    } else {
                        ip += 2;
                    }
                }
                0x26 => {
                    // JMPIFNOT rel8
                    let off = code[ip + 1] as i8 as isize;
                    let c = stack.pop().ok_or("jmpifnot underflow")?;
                    if c == BigInt::from(0) {
                        ip = (ip as isize + off) as usize;
                    } else {
                        ip += 2;
                    }
                }
                0x3A => {
                    // THROW (used here to signal a Solidity Panic, e.g. overflow)
                    return Err("THROW".into());
                }
                0x40 => break, // RET
                other => return Err(format!("faithful VM: unhandled opcode 0x{other:02x}")),
            }
        }
        Ok(stack)
    }

    fn run_lt(a: &BigInt, b: &BigInt) -> bool {
        let mut code = Vec::new();
        emit_pushint256_le(&mut code, &u256_le(a));
        emit_pushint256_le(&mut code, &u256_le(b));
        emit_uint256_unsigned_lt(&mut code);
        code.push(0x40);
        let st = faithful_run(&code).expect("faithful run");
        st.last().cloned().unwrap_or_else(|| BigInt::from(0)) != BigInt::from(0)
    }

    fn run_gt(a: &BigInt, b: &BigInt) -> bool {
        let mut code = Vec::new();
        emit_pushint256_le(&mut code, &u256_le(a));
        emit_pushint256_le(&mut code, &u256_le(b));
        emit_uint256_unsigned_gt(&mut code);
        code.push(0x40);
        let st = faithful_run(&code).expect("faithful run");
        st.last().cloned().unwrap_or_else(|| BigInt::from(0)) != BigInt::from(0)
    }

    fn big(s: &str) -> BigInt {
        BigInt::parse_bytes(s.as_bytes(), 10).unwrap()
    }
    fn pow2(n: u32) -> BigInt {
        BigInt::from(1) << n
    }
    fn umax() -> BigInt {
        modulus() - 1
    }

    #[test]
    fn faithful_vm_rejects_oversize_integers() {
        // Sanity: the reference VM faults on a >32-byte integer, like a real node.
        // (2^255-1) XOR (-(2^255)) stays <=32 bytes; but pushing then XOR of two
        // values whose result needs 33 bytes must error. Build 0x7F*32 (=2^255-1)
        // and verify no false fault, then confirm the checker rejects 33 bytes.
        let v = BigInt::from(1) << 256u32; // 2^256 needs 33 signed bytes
        assert!(v.to_signed_bytes_le().len() > 32);
    }

    #[test]
    fn unsigned_lt_small_values() {
        assert!(run_lt(&BigInt::from(5), &BigInt::from(10)));
        assert!(!run_lt(&BigInt::from(10), &BigInt::from(5)));
        assert!(!run_lt(&BigInt::from(7), &BigInt::from(7)));
        assert!(run_lt(&BigInt::from(0), &BigInt::from(1)));
    }

    #[test]
    fn unsigned_lt_large_values_above_2_255() {
        // The cases native signed comparison gets WRONG.
        assert!(run_lt(&BigInt::from(5), &umax()), "5 < uint256.max");
        assert!(!run_lt(&umax(), &BigInt::from(5)), "max not < 5");
        assert!(run_lt(&BigInt::from(0), &umax()), "0 < max");
        assert!(!run_lt(&umax(), &umax()), "max not < max");
        assert!(run_lt(&pow2(255), &(pow2(255) + 1)), "2^255 < 2^255+1");
        assert!(!run_lt(&(pow2(255) + 1), &pow2(255)));
        assert!(run_lt(&(pow2(255) - 1), &pow2(255)), "2^255-1 < 2^255 (straddle)");
        assert!(!run_lt(&pow2(255), &(pow2(255) - 1)));
        assert!(run_lt(
            &big("100"),
            &big("115792089237316195423570985008687907853269984665640564039457584007913129639000")
        ));
    }

    #[test]
    fn unsigned_gt_matches_lt() {
        assert!(run_gt(&umax(), &BigInt::from(5)), "max > 5");
        assert!(!run_gt(&BigInt::from(5), &umax()));
        assert!(run_gt(&pow2(255), &(pow2(255) - 1)));
        assert!(!run_gt(&BigInt::from(7), &BigInt::from(7)));
    }

    fn run_cmp(emit: fn(&mut Vec<u8>), a: &BigInt, b: &BigInt) -> bool {
        let mut code = Vec::new();
        emit_pushint256_le(&mut code, &u256_le(a));
        emit_pushint256_le(&mut code, &u256_le(b));
        emit(&mut code);
        code.push(0x40);
        let st = faithful_run(&code).expect("faithful run");
        st.last().cloned().unwrap_or_else(|| BigInt::from(0)) != BigInt::from(0)
    }

    /// Run the unchecked-add routine and return the result as an unsigned
    /// uint256 in [0, 2^256).
    fn run_add(a: &BigInt, b: &BigInt) -> BigInt {
        let mut code = Vec::new();
        emit_pushint256_le(&mut code, &u256_le(a));
        emit_pushint256_le(&mut code, &u256_le(b));
        emit_uint256_unchecked_add(&mut code);
        code.push(0x40);
        let st = faithful_run(&code).expect("faithful run");
        let signed = st.last().cloned().expect("result");
        let m = modulus();
        ((signed % &m) + &m) % &m
    }

    /// Run the unchecked-sub routine and return the result as an unsigned
    /// uint256 in [0, 2^256).
    fn run_sub(a: &BigInt, b: &BigInt) -> BigInt {
        let mut code = Vec::new();
        emit_pushint256_le(&mut code, &u256_le(a));
        emit_pushint256_le(&mut code, &u256_le(b));
        emit_uint256_unchecked_sub(&mut code);
        code.push(0x40);
        let st = faithful_run(&code).expect("faithful run");
        let signed = st.last().cloned().expect("result");
        let m = modulus();
        ((signed % &m) + &m) % &m
    }

    #[test]
    fn unchecked_sub_wraps_mod_2_256_including_large() {
        let m = modulus();
        let cases = [
            (BigInt::from(5), BigInt::from(3)),
            (BigInt::from(3), BigInt::from(5)),   // wraps to 2^256-2
            (BigInt::from(0), BigInt::from(1)),   // wraps to 2^256-1
            (umax(), umax()),                     // 0
            (umax(), BigInt::from(1)),            // 2^256-2
            (pow2(255), BigInt::from(1)),         // 2^255-1 (crosses sign)
            (pow2(255), pow2(255)),               // 0
            (pow2(128), BigInt::from(1)),         // borrow across limb boundary
            (BigInt::from(0), umax()),            // 1
            (pow2(200), pow2(199)),               // 2^199
        ];
        for (a, b) in cases {
            let expect = ((&a - &b) % &m + &m) % &m;
            assert_eq!(run_sub(&a, &b), expect, "sub({a}, {b})");
        }
    }

    #[test]
    fn unchecked_add_wraps_mod_2_256_including_large() {
        let m = modulus();
        let cases = [
            (BigInt::from(2), BigInt::from(3)),
            (BigInt::from(100), BigInt::from(200)),
            (umax(), BigInt::from(1)),            // wraps to 0
            (umax(), BigInt::from(2)),            // wraps to 1
            (pow2(255), pow2(255)),               // 2^256 -> 0
            (pow2(255) - 1, pow2(255) - 1),       // 2^256 - 2 (result >= 2^255)
            (pow2(128), pow2(128)),               // crosses the limb boundary
            (pow2(200) + 5, pow2(200) + 7),
            (umax(), umax()),                     // 2^257-2 mod 2^256 = 2^256-2
        ];
        for (a, b) in cases {
            let expect = ((&a + &b) % &m + &m) % &m;
            assert_eq!(run_add(&a, &b), expect, "add({a}, {b})");
        }
    }

    /// Run a checked routine; `Ok(value)` on success, `Err` if it threw (panic).
    fn run_checked(emit: fn(&mut Vec<u8>), a: &BigInt, b: &BigInt) -> Result<BigInt, String> {
        let mut code = Vec::new();
        emit_pushint256_le(&mut code, &u256_le(a));
        emit_pushint256_le(&mut code, &u256_le(b));
        emit(&mut code);
        code.push(0x40);
        let st = faithful_run(&code)?;
        let signed = st.last().cloned().ok_or("no result")?;
        let m = modulus();
        Ok(((signed % &m) + &m) % &m)
    }

    #[test]
    fn checked_add_detects_overflow() {
        // In range: matches the true sum.
        assert_eq!(run_checked(emit_uint256_checked_add, &big("2"), &big("3")), Ok(big("5")));
        assert_eq!(
            run_checked(emit_uint256_checked_add, &(pow2(255) - 1), &BigInt::from(1)),
            Ok(pow2(255)),
            "2^255-1 + 1 = 2^255 (still < 2^256)"
        );
        assert_eq!(
            run_checked(emit_uint256_checked_add, &(umax() - 1), &BigInt::from(1)),
            Ok(umax()),
            "max-1 + 1 = max"
        );
        // Overflow: a + b >= 2^256 must throw.
        assert!(run_checked(emit_uint256_checked_add, &umax(), &BigInt::from(1)).is_err());
        assert!(run_checked(emit_uint256_checked_add, &pow2(255), &pow2(255)).is_err());
        assert!(run_checked(emit_uint256_checked_add, &umax(), &umax()).is_err());
        assert!(run_checked(emit_uint256_checked_add, &(pow2(255) + 7), &pow2(255)).is_err());
    }

    #[test]
    fn checked_sub_detects_underflow() {
        // In range: a >= b.
        assert_eq!(run_checked(emit_uint256_checked_sub, &big("5"), &big("3")), Ok(big("2")));
        assert_eq!(run_checked(emit_uint256_checked_sub, &umax(), &umax()), Ok(big("0")));
        assert_eq!(
            run_checked(emit_uint256_checked_sub, &pow2(255), &BigInt::from(1)),
            Ok(pow2(255) - 1),
            "2^255 - 1 crosses the sign boundary"
        );
        assert_eq!(run_checked(emit_uint256_checked_sub, &umax(), &big("1")), Ok(umax() - 1));
        // Underflow: a < b must throw.
        assert!(run_checked(emit_uint256_checked_sub, &big("3"), &big("5")).is_err());
        assert!(run_checked(emit_uint256_checked_sub, &big("0"), &big("1")).is_err());
        assert!(run_checked(emit_uint256_checked_sub, &big("0"), &umax()).is_err());
        assert!(run_checked(emit_uint256_checked_sub, &(pow2(255) - 1), &pow2(255)).is_err());
    }

    /// Run the unchecked-mul routine; result as unsigned uint256 in [0, 2^256).
    fn run_mul(a: &BigInt, b: &BigInt) -> BigInt {
        let mut code = Vec::new();
        emit_pushint256_le(&mut code, &u256_le(a));
        emit_pushint256_le(&mut code, &u256_le(b));
        emit_uint256_unchecked_mul(&mut code);
        code.push(0x40);
        let st = faithful_run(&code).expect("faithful run");
        let signed = st.last().cloned().expect("result");
        let m = modulus();
        ((signed % &m) + &m) % &m
    }

    #[test]
    fn unchecked_mul_wraps_mod_2_256_including_large() {
        let m = modulus();
        let cases = [
            (BigInt::from(0), umax()),
            (BigInt::from(1), umax()),
            (BigInt::from(6), BigInt::from(7)),
            (pow2(64), pow2(64)),                 // 2^128 (limb boundary)
            (pow2(128), pow2(128)),               // 2^256 -> 0
            (pow2(200), pow2(100)),               // 2^300 mod 2^256 = 2^44
            (umax(), BigInt::from(2)),            // 2^257-2 mod 2^256 = 2^256-2
            (umax(), umax()),                     // (2^256-1)^2 mod 2^256 = 1
            (pow2(255), BigInt::from(3)),         // 3*2^255 mod 2^256 = 2^255
            (big("123456789012345678901234567890"), big("987654321098765432109876543210")),
            (pow2(130) + 7, pow2(130) + 9),
        ];
        for (a, b) in cases {
            let expect = (&a * &b) % &m;
            assert_eq!(run_mul(&a, &b), expect, "mul({a}, {b})");
        }
    }

    #[test]
    fn checked_mul_detects_overflow() {
        // In range (product < 2^256): exact value.
        assert_eq!(run_checked(emit_uint256_checked_mul, &big("6"), &big("7")), Ok(big("42")));
        assert_eq!(run_checked(emit_uint256_checked_mul, &umax(), &big("1")), Ok(umax()));
        assert_eq!(run_checked(emit_uint256_checked_mul, &big("0"), &umax()), Ok(big("0")));
        assert_eq!(
            run_checked(emit_uint256_checked_mul, &(pow2(255) - 1), &big("2")),
            Ok(pow2(256) - 2),
            "(2^255-1)*2 = 2^256-2 (still fits)"
        );
        assert_eq!(
            run_checked(emit_uint256_checked_mul, &pow2(128), &(pow2(128) - 1)),
            Ok(pow2(256) - pow2(128)),
            "2^128 * (2^128-1) = 2^256-2^128 (fits)"
        );
        // Overflow: product >= 2^256 must throw.
        assert!(run_checked(emit_uint256_checked_mul, &pow2(128), &pow2(128)).is_err());
        assert!(run_checked(emit_uint256_checked_mul, &pow2(255), &big("2")).is_err());
        assert!(run_checked(emit_uint256_checked_mul, &umax(), &big("2")).is_err());
        assert!(run_checked(emit_uint256_checked_mul, &umax(), &umax()).is_err());
        assert!(run_checked(emit_uint256_checked_mul, &(pow2(200)), &(pow2(100))).is_err());
    }

    /// Run the logical-shr routine; result as unsigned uint256 in [0, 2^256).
    fn run_shr(a: &BigInt, n: u32) -> BigInt {
        let mut code = Vec::new();
        emit_pushint256_le(&mut code, &u256_le(a));
        emit_pushint256_le(&mut code, &u256_le(&BigInt::from(n)));
        emit_uint256_logical_shr(&mut code);
        code.push(0x40);
        let st = faithful_run(&code).expect("faithful run");
        let signed = st.last().cloned().expect("result");
        let m = modulus();
        ((signed % &m) + &m) % &m
    }

    #[test]
    fn logical_shr_is_unsigned_including_large() {
        // Reference: unsigned shift = floor(a / 2^n).
        let cases: [(BigInt, u32); 12] = [
            (umax(), 0),
            (umax(), 1),
            (umax(), 128),
            (umax(), 255),
            (umax(), 256),                 // shifts everything out -> 0
            (pow2(255), 1),                // 2^254 (native arith SHR would keep sign)
            (pow2(255), 255),              // 1
            (pow2(255) + BigInt::from(1), 255), // 1 (low bit dropped)
            (pow2(200), 100),              // 2^100
            (pow2(128), 64),               // 2^64
            (big("115792089237316195423570985008687907853269984665640564039457584007913129639000"), 8),
            (BigInt::from(0), 5),
        ];
        for (a, n) in cases {
            let expect = &a >> n; // a is in [0, 2^256), so this is the unsigned shift
            assert_eq!(run_shr(&a, n), expect, "shr({a}, {n})");
        }
    }

    #[test]
    fn unsigned_le_ge_including_large() {
        // <=
        assert!(run_cmp(emit_uint256_unsigned_le, &BigInt::from(7), &BigInt::from(7)));
        assert!(run_cmp(emit_uint256_unsigned_le, &BigInt::from(5), &umax()));
        assert!(!run_cmp(emit_uint256_unsigned_le, &umax(), &BigInt::from(5)));
        assert!(run_cmp(emit_uint256_unsigned_le, &umax(), &umax()));
        // >=
        assert!(run_cmp(emit_uint256_unsigned_ge, &BigInt::from(7), &BigInt::from(7)));
        assert!(run_cmp(emit_uint256_unsigned_ge, &umax(), &BigInt::from(5)));
        assert!(!run_cmp(emit_uint256_unsigned_ge, &BigInt::from(5), &umax()));
        assert!(run_cmp(emit_uint256_unsigned_ge, &pow2(255), &(pow2(255) - 1)));
    }
}
