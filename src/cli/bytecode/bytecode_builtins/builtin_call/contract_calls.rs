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
    // Solidity expects `bytes` for low-level calls; serialize the return stack item.
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::StdLib,
        "serialize",
        1,
        use_callt,
        token_patches,
    );
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
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::StdLib,
        "serialize",
        1,
        use_callt,
        token_patches,
    );
}
