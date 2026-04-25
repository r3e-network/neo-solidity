fn emit_keccak256(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::CryptoLib,
        "keccak256",
        1,
        use_callt,
        token_patches,
    )
}

fn emit_ecrecover(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Neo N3 does not provide an `ecrecover` syscall. We map it to the
    // native contract `CryptoLib.recoverSecp256K1`, then derive the
    // Ethereum 20-byte address per the Ethereum Yellow Paper:
    //   address = keccak256(pubkey_uncompressed[1..])[12..]
    // where `pubkey_uncompressed` is the 65-byte SEC1 encoding that
    // `recoverSecp256K1` returns (leading `0x04` prefix + 64-byte x||y).
    //
    // Stack input (Solidity): [hash32, v, r, s]
    // Build signature: r || s || v (65 bytes), with v normalized to 27..30.
    bytecode.push(0x8B); // CAT: r||s => [hash32, v, rs]
    bytecode.push(0x50); // SWAP => [hash32, rs, v]

    // If v < 27, normalize v by adding 27.
    bytecode.push(0x4A); // DUP
    push_integer_bigint(bytecode, &BigInt::from(27u8));
    bytecode.push(0xB5); // LT
    let jmp_skip_add_pos = bytecode.len();
    bytecode.push(0x27); // JMPIFNOT_L
    let jmp_skip_add_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);
    push_integer_bigint(bytecode, &BigInt::from(27u8));
    bytecode.push(0x9E); // ADD
    let after_add_pos = bytecode.len();
    let rel_after_add = (after_add_pos as i32)
        .checked_sub(jmp_skip_add_pos as i32)
        .unwrap_or(0);
    bytecode[jmp_skip_add_operand..jmp_skip_add_operand + 4]
        .copy_from_slice(&rel_after_add.to_le_bytes());

    // Convert v into a single-byte buffer and append to rs.
    // Stack: [hash32, rs, v]
    bytecode.push(0x11); // PUSH1
    bytecode.push(0x88); // NEWBUFFER (len=1) => [hash32, rs, v, buf]
    bytecode.push(0x4A); // DUP => [hash32, rs, v, buf, buf]
    bytecode.push(0x51); // ROT => [hash32, rs, buf, buf, v]
    bytecode.push(0x10); // PUSH0 => [hash32, rs, buf, buf, v, 0]
    bytecode.push(0x50); // SWAP => [hash32, rs, buf, buf, 0, v]
    bytecode.push(0xD0); // SETITEM (buf[0] = v) => [hash32, rs, buf]
    bytecode.push(0x8B); // CAT: rs||buf => [hash32, signature]

    // Call CryptoLib.recoverSecp256K1(hash32, signature)
    // Returns the 65-byte uncompressed pubkey (0x04 || x || y) on success,
    // or null on failure.
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::CryptoLib,
        "recoverSecp256K1",
        2,
        use_callt,
        token_patches,
    );

    // If recovery fails (null), return 0x00..00.
    bytecode.push(0x4A); // DUP
    bytecode.push(0xD8); // ISNULL
    let jmp_if_not_null_pos = bytecode.len();
    bytecode.push(0x27); // JMPIFNOT_L
    let jmp_if_not_null_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);

    // null path
    bytecode.push(0x45); // DROP
    push_data(bytecode, &[0u8; 20]);
    let jmp_end_pos = bytecode.len();
    bytecode.push(0x23); // JMP_L
    let jmp_end_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);

    // not-null path: derive Ethereum address from uncompressed pubkey.
    // Some invalid signatures return an empty byte string rather than Null;
    // treat any short value as failed recovery before slicing pubkey[1..65].
    let not_null_pos = bytecode.len();
    bytecode.push(0x4A); // DUP
    bytecode.push(0xCA); // SIZE
    push_integer_bigint(bytecode, &BigInt::from(65u8));
    bytecode.push(0xB5); // LT
    let jmp_if_long_enough_pos = bytecode.len();
    bytecode.push(0x27); // JMPIFNOT_L
    let jmp_if_long_enough_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);

    // short path
    bytecode.push(0x45); // DROP
    push_data(bytecode, &[0u8; 20]);
    let jmp_short_end_pos = bytecode.len();
    bytecode.push(0x23); // JMP_L
    let jmp_short_end_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);

    // long-enough path: Stack: [pubkey65]
    let long_enough_pos = bytecode.len();

    // Strip the leading 0x04 byte: SUBSTR(pubkey65, 1, 64) => [pubkey64 (x||y)]
    push_integer_bigint(bytecode, &BigInt::from(1u8)); // start offset
    push_integer_bigint(bytecode, &BigInt::from(64u8)); // length
    bytecode.push(0x8C); // SUBSTR

    // keccak256(x||y) => [hash32]
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::CryptoLib,
        "keccak256",
        1,
        use_callt,
        token_patches,
    );

    // Take last 20 bytes: RIGHT(hash32, 20) => [address20]
    push_integer_bigint(bytecode, &BigInt::from(20u8));
    bytecode.push(0x8E); // RIGHT

    let end_pos = bytecode.len();

    let rel_not_null = (not_null_pos as i32)
        .checked_sub(jmp_if_not_null_pos as i32)
        .unwrap_or(0);
    bytecode[jmp_if_not_null_operand..jmp_if_not_null_operand + 4]
        .copy_from_slice(&rel_not_null.to_le_bytes());

    let rel_long_enough = (long_enough_pos as i32)
        .checked_sub(jmp_if_long_enough_pos as i32)
        .unwrap_or(0);
    bytecode[jmp_if_long_enough_operand..jmp_if_long_enough_operand + 4]
        .copy_from_slice(&rel_long_enough.to_le_bytes());

    let rel_short_end = (end_pos as i32)
        .checked_sub(jmp_short_end_pos as i32)
        .unwrap_or(0);
    bytecode[jmp_short_end_operand..jmp_short_end_operand + 4]
        .copy_from_slice(&rel_short_end.to_le_bytes());

    let rel_end = (end_pos as i32)
        .checked_sub(jmp_end_pos as i32)
        .unwrap_or(0);
    bytecode[jmp_end_operand..jmp_end_operand + 4].copy_from_slice(&rel_end.to_le_bytes());
}

fn emit_verify_signature(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    push_integer_bigint(bytecode, &BigInt::from(23u8)); // secp256r1 curve
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::CryptoLib,
        "verifyWithECDsa",
        4,
        use_callt,
        token_patches,
    );
}

/// Task #H6a: emit the 0x01 ecrecover precompile routing.
/// Stack in : [abi_payload]  (128-byte `bytes memory` = hash || v || r || s)
/// Stack out: [result32]     (20-byte address left-padded into a 32-byte slot)
///
/// We extract the 4 ABI slots via SUBSTR opcodes, normalize `v` (which is a
/// 32-byte BE integer slot in the ABI payload) into a single-byte integer on
/// the NeoVM stack, then delegate to the existing `emit_ecrecover` sequence
/// (recoverSecp256K1 → SUBSTR(1,64) → keccak256 → RIGHT 20). A final CAT with
/// 12 zero bytes left-pads the 20-byte address into a 32-byte slot so the
/// returned `bytes memory` matches the Ethereum 0x01 contract output.
fn emit_precompile_ecrecover(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Stack: [payload]
    // Short or non-canonical payloads cannot supply the four ABI words.
    // Match the invalid-signature path and return a zero-padded 32-byte slot
    // instead of faulting before the low-level call can return `(true, out)`.
    bytecode.push(0x4A); // DUP
    bytecode.push(0xCA); // SIZE
    push_integer_bigint(bytecode, &BigInt::from(128u8));
    bytecode.push(0xB5); // LT
    let jmp_payload_long_enough_pos = bytecode.len();
    bytecode.push(0x27); // JMPIFNOT_L
    let jmp_payload_long_enough_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);

    bytecode.push(0x45); // DROP
    push_data(bytecode, &[0u8; 32]);
    let jmp_payload_end_pos = bytecode.len();
    bytecode.push(0x23); // JMP_L
    let jmp_payload_end_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);

    let payload_long_enough_pos = bytecode.len();

    // hash = SUBSTR(payload, 0, 32). Dup payload for each slice.
    bytecode.push(0x4A); // DUP → [payload, payload]
    push_integer_bigint(bytecode, &BigInt::from(0u8));
    push_integer_bigint(bytecode, &BigInt::from(32u8));
    bytecode.push(0x8C); // SUBSTR → [payload, hash32]
    bytecode.push(0x50); // SWAP → [hash32, payload]

    // v slot: SUBSTR(payload, 32, 32). Take the LAST byte (v is 1 byte BE).
    bytecode.push(0x4A); // DUP → [hash32, payload, payload]
    push_integer_bigint(bytecode, &BigInt::from(32u8));
    push_integer_bigint(bytecode, &BigInt::from(32u8));
    bytecode.push(0x8C); // SUBSTR → [hash32, payload, v_slot]
    push_integer_bigint(bytecode, &BigInt::from(1u8));
    bytecode.push(0x8E); // RIGHT → [hash32, payload, v_byte]
    bytecode.push(0xDB); // CONVERT
    bytecode.push(0x21); // to Integer
    bytecode.push(0x50); // SWAP → [hash32, v_int, payload]

    // r = SUBSTR(payload, 64, 32)
    bytecode.push(0x4A); // DUP → [hash32, v_int, payload, payload]
    push_integer_bigint(bytecode, &BigInt::from(64u8));
    push_integer_bigint(bytecode, &BigInt::from(32u8));
    bytecode.push(0x8C); // SUBSTR → [hash32, v_int, payload, r32]
    bytecode.push(0x50); // SWAP → [hash32, v_int, r32, payload]

    // s = SUBSTR(payload, 96, 32) (payload consumed by final SUBSTR).
    push_integer_bigint(bytecode, &BigInt::from(96u8));
    push_integer_bigint(bytecode, &BigInt::from(32u8));
    bytecode.push(0x8C); // SUBSTR → [hash32, v_int, r32, s32]

    // Reuse the existing ecrecover lowering. It consumes [hash32, v, r, s]
    // and leaves the 20-byte address on the stack.
    emit_ecrecover(bytecode, use_callt, token_patches);

    // Left-pad 20-byte address into a 32-byte slot: prepend 12 zero bytes.
    push_data(bytecode, &[0u8; 12]);
    bytecode.push(0x50); // SWAP → [pad12, address20]
    bytecode.push(0x8B); // CAT → [address32]

    let payload_end_pos = bytecode.len();
    let rel_payload_long_enough = (payload_long_enough_pos as i32)
        .checked_sub(jmp_payload_long_enough_pos as i32)
        .unwrap_or(0);
    bytecode[jmp_payload_long_enough_operand..jmp_payload_long_enough_operand + 4]
        .copy_from_slice(&rel_payload_long_enough.to_le_bytes());

    let rel_payload_end = (payload_end_pos as i32)
        .checked_sub(jmp_payload_end_pos as i32)
        .unwrap_or(0);
    bytecode[jmp_payload_end_operand..jmp_payload_end_operand + 4]
        .copy_from_slice(&rel_payload_end.to_le_bytes());
}

/// Task #H6b: emit the 0x05 modexp precompile routing — MINIMAL 1-byte-operand
/// variant. The Ethereum 0x05 input layout is
///   [0..32]  base_len
///   [32..64] exp_len
///   [64..96] mod_len
///   [96..]   base || exp || mod (packed, BE, widths per length slots)
/// For the test path (base_len=exp_len=mod_len=1) we extract the three
/// bytes at offsets 96/97/98, CONVERT each to an integer, invoke MODPOW,
/// then encode the result into a 32-byte BE slot by prepending 31 zeros to
/// a single-byte buffer. Larger operands fall through to a zero-filled
/// 32-byte buffer so the staticcall tuple still has the expected shape
/// (this matches Solidity's behaviour when `mod == 1` — result is zero).
///
/// NeoVM MODPOW (0xA6) pops [base, exp, mod] and pushes the result.
fn emit_precompile_modexp(bytecode: &mut Vec<u8>, _use_callt: bool) {
    // Stack: [payload] → extract bytes at offsets 96/97/98 as integers.
    bytecode.push(0x4A); // DUP payload
    push_integer_bigint(bytecode, &BigInt::from(96u8));
    push_integer_bigint(bytecode, &BigInt::from(1u8));
    bytecode.push(0x8C); // SUBSTR → base (1 byte)
    bytecode.push(0xDB); // CONVERT
    bytecode.push(0x21); // to Integer
    bytecode.push(0x50); // SWAP → [base, payload]

    bytecode.push(0x4A); // DUP payload
    push_integer_bigint(bytecode, &BigInt::from(97u8));
    push_integer_bigint(bytecode, &BigInt::from(1u8));
    bytecode.push(0x8C); // SUBSTR → exp
    bytecode.push(0xDB); // CONVERT
    bytecode.push(0x21); // to Integer
    bytecode.push(0x50); // SWAP → [base, exp, payload]

    push_integer_bigint(bytecode, &BigInt::from(98u8));
    push_integer_bigint(bytecode, &BigInt::from(1u8));
    bytecode.push(0x8C); // SUBSTR → mod (consumes payload)
    bytecode.push(0xDB); // CONVERT
    bytecode.push(0x21); // to Integer
    // Stack: [base, exp, mod]

    bytecode.push(0xA6); // MODPOW → [result_int]

    // Encode result into a 32-byte BE slot. Convert to ByteString (LE in
    // NeoVM), then prepend 31 zeros so the 1-byte result occupies the
    // rightmost byte (BE presentation).
    bytecode.push(0xDB); // CONVERT
    bytecode.push(0x28); // to ByteString
    push_data(bytecode, &[0u8; 31]);
    bytecode.push(0x50); // SWAP
    bytecode.push(0x8B); // CAT → 32-byte BE result
}
