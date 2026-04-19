fn emit_abi_encode(
    bytecode: &mut Vec<u8>,
    arg_count: usize,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    let total_bigint = BigInt::from(arg_count);
    push_integer_bigint(bytecode, &total_bigint);
    bytecode.push(0xC0); // PACK
    // PACK pops values from the stack and therefore reverses their order.
    // For `abi.encode(a, b, c)` we must preserve the original argument order.
    if arg_count > 1 {
        bytecode.push(0x4A); // DUP (keep a reference to the array on the stack)
        bytecode.push(0xD1); // REVERSEITEMS (consumes one reference, reverses in-place)
    }
    // Task #44 — target the EVM-canonical encoder (BE-padded 32-byte slots).
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::StdLib,
        "abiEncode",
        1,
        use_callt,
        token_patches,
    );
}

fn emit_abi_encode_packed(
    bytecode: &mut Vec<u8>,
    arg_count: usize,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    let total_bigint = BigInt::from(arg_count);
    push_integer_bigint(bytecode, &total_bigint);
    bytecode.push(0xC0); // PACK
    if arg_count > 1 {
        bytecode.push(0x4A); // DUP
        bytecode.push(0xD1); // REVERSEITEMS
    }
    // Task #44 — target the packed encoder (per-element BE width, no padding
    // between slots).
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::StdLib,
        "abiEncodePacked",
        1,
        use_callt,
        token_patches,
    );
}

fn emit_abi_decode(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Task #44 — round-trip decoder. Accepts the BE-packed payload produced
    // by `abiEncode` and yields an Array of UnsignedInteger slots.
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::StdLib,
        "abiDecode",
        1,
        use_callt,
        token_patches,
    );
}

fn emit_notify_serialized(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Syscalls.notify expects a serialized payload: [eventName, stateArray]
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::StdLib,
        "deserialize",
        1,
        use_callt,
        token_patches,
    );
    // Extract eventName/state via PICKITEM and call Runtime.Notify.
    bytecode.push(0x4A); // DUP (payload)
    bytecode.push(0x10); // PUSH0
    bytecode.push(0xCE); // PICKITEM -> eventName
    bytecode.push(0x50); // SWAP -> [eventName, payload]
    bytecode.push(0x4A); // DUP (payload)
    bytecode.push(0x11); // PUSH1
    bytecode.push(0xCE); // PICKITEM -> state
    bytecode.push(0x46); // NIP (drop payload) -> [eventName, state]
    bytecode.push(0x50); // SWAP -> [state, eventName]
    emit_syscall(bytecode, "System.Runtime.Notify");
    bytecode.push(0x11); // PUSH1
}
