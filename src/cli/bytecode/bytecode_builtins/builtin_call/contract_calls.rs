fn emit_contract_call(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Stack: [contract_hash, method, params(bytes)]
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::StdLib,
        "deserialize",
        1,
        use_callt,
        token_patches,
    );
    push_integer_bigint(bytecode, &BigInt::from(CALLFLAGS_ALL));
    bytecode.push(0x50); // SWAP -> [contract, method, flags, args]
    bytecode.push(0x54); // REVERSE4 -> [args, flags, method, contract]
    emit_syscall(bytecode, "System.Contract.Call");
    // Task #194 — Solidity low-level `.call()` returns `(bool, bytes)` where
    // `bytes` is the EVM-canonical ABI-encoded return value of the callee.
    // Previously we ran `StdLib.serialize` on the raw return StackItem, which
    // produces NEO-binary format (type tag + length + value), not the
    // 32-byte BE-padded encoding that `abi.decode(data, (T))` expects. Wrap
    // the return in a single-element array and emit `StdLib.abiEncode` so
    // the byte shape matches what Solidity callers see on an EVM, making
    // `abi.decode(data, (uint))` round-trip on simple scalar returns.
    emit_abi_encode_single_return(bytecode, use_callt, token_patches);
}

fn emit_contract_call_with_flags(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Stack: [contract_hash, method, params(bytes), flags]
    bytecode.push(0x50); // SWAP -> [contract, method, flags, params]
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::StdLib,
        "deserialize",
        1,
        use_callt,
        token_patches,
    );
    bytecode.push(0x54); // REVERSE4 -> [args, flags, method, contract]
    emit_syscall(bytecode, "System.Contract.Call");
    // Task #194 — same ABI-encoded wrapping as the non-flags path so that
    // `target.staticcall(abi.encodeWithSelector(…))` round-trips through
    // `abi.decode` on the return side.
    emit_abi_encode_single_return(bytecode, use_callt, token_patches);
}

/// Task #194 — wrap a single return `StackItem` on the top of the stack in
/// a one-element Array and hand it to `StdLib.abiEncode`, yielding the
/// EVM-canonical ABI-encoded `bytes` blob expected by Solidity's low-level
/// `(bool, bytes)` tuple. The wrapper is necessary because `abiEncode`
/// takes an outer "positional args" array (how the bytecode emitter packs
/// multi-arg `abi.encode(a,b,c)` callsites) and we only have one value.
fn emit_abi_encode_single_return(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Stack: [returnValue]
    // NeoVM uses `Null` for "no return value". Solidity low-level calls
    // surface that as empty returndata, not as an ABI-encoded null sentinel.
    // Short-circuit the null case so void methods can succeed on chain without
    // depending on the pseudo-native `StdLib.abiEncode` helper.
    bytecode.push(0x4A); // DUP
    bytecode.push(0xD8); // ISNULL
    let jmp_if_not_null_pos = bytecode.len();
    bytecode.push(0x27); // JMPIFNOT_L
    let jmp_if_not_null_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);

    // null path -> empty bytes
    bytecode.push(0x45); // DROP
    push_data(bytecode, &[]);
    let jmp_end_pos = bytecode.len();
    bytecode.push(0x23); // JMP_L
    let jmp_end_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);

    let not_null_pos = bytecode.len();

    // PUSH1 — arity for PACK.
    bytecode.push(0x11); // PUSH1
    bytecode.push(0xC0); // PACK -> [Array([returnValue])]
    // abiEncode unwraps the single-element outer Array and treats the
    // sole inner item as a positional arg, producing the 32-byte BE slot
    // (scalar) or offset+length+padded-data tail (dynamic).
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::StdLib,
        "abiEncode",
        1,
        use_callt,
        token_patches,
    );

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
