fn emit_runtime_notify(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Runtime.notify(eventName, data) expects `data` to be NeoVM-serialized
    // (i.e., produced by abi.encode(...)). We deserialize it into an array
    // and pass it as the Notify state.
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::StdLib,
        "deserialize",
        1,
        use_callt,
        token_patches,
    );
    bytecode.push(0x50); // SWAP -> [stateArray, eventName]
    emit_syscall(bytecode, "System.Runtime.Notify");
    bytecode.push(0x11); // PUSH1 (truthy) to satisfy Solidity's expression semantics
}

fn emit_runtime_check_witness(bytecode: &mut Vec<u8>) {
    emit_syscall(bytecode, "System.Runtime.CheckWitness");
}
