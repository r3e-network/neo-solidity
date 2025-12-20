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
    // native contract `CryptoLib.recoverSecp256K1` and then return the
    // Neo account script hash produced by `System.Contract.CreateStandardAccount`.
    //
    // This makes the recovered address compatible with Neo's witness
    // model (i.e., it can be fed into `System.Runtime.CheckWitness`).
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

    // not-null path
    let not_null_pos = bytecode.len();
    emit_syscall(bytecode, "System.Contract.CreateStandardAccount");

    let end_pos = bytecode.len();

    let rel_not_null = (not_null_pos as i32)
        .checked_sub(jmp_if_not_null_pos as i32)
        .unwrap_or(0);
    bytecode[jmp_if_not_null_operand..jmp_if_not_null_operand + 4]
        .copy_from_slice(&rel_not_null.to_le_bytes());

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
