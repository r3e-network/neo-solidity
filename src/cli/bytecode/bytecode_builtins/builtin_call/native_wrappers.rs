use super::*;

pub(crate) fn emit_native_call(
    bytecode: &mut Vec<u8>,
    contract: ir::NativeContract,
    method: &str,
    arg_count: usize,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    emit_native_contract_call(
        bytecode,
        contract,
        method,
        arg_count,
        use_callt,
        token_patches,
    );
}

pub(crate) fn emit_deploy_contract(
    bytecode: &mut Vec<u8>,
    arg_count: usize,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // ContractManagement.deploy(nef, manifest[, data]) returns a ContractState.
    // NativeCalls.deployContract is a devpack convenience wrapper that returns
    // the deployed contract hash (UInt160) only.
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::ContractManagement,
        "deploy",
        arg_count,
        use_callt,
        token_patches,
    );
    bytecode.push(0x12); // PUSH2 (ContractState.Hash field index)
    bytecode.push(0xCE); // PICKITEM
}

pub(crate) fn emit_get_contract(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // ContractManagement.getContract(UInt160) returns a ContractState struct:
    // [id, updateCounter, hash, nef, manifestStruct]
    //
    // NativeCalls.getContract reshapes it into:
    // [hash, nef, serialize(manifestStruct), updateCounter]
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::ContractManagement,
        "getContract",
        1,
        use_callt,
        token_patches,
    );

    // Extract hash (index 2)
    bytecode.push(0x4A); // DUP
    bytecode.push(0x12); // PUSH2
    bytecode.push(0xCE); // PICKITEM -> [state, hash]

    // Extract nef (index 3)
    bytecode.push(0x4B); // OVER
    bytecode.push(0x13); // PUSH3
    bytecode.push(0xCE); // PICKITEM -> [state, hash, nef]

    // Extract manifest (index 4) and serialize it
    bytecode.push(0x4B); // OVER
    bytecode.push(0x14); // PUSH4
    bytecode.push(0xCE); // PICKITEM -> [state, hash, nef, manifestStruct]
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::StdLib,
        "serialize",
        1,
        use_callt,
        token_patches,
    ); // -> [state, hash, nef, manifestBytes]

    // Extract updateCounter (index 1)
    bytecode.push(0x13); // PUSH3 (duplicate state from depth 3)
    bytecode.push(0x4D); // PICK -> [state, hash, nef, manifestBytes, state]
    bytecode.push(0x11); // PUSH1
    bytecode.push(0xCE); // PICKITEM -> [state, hash, nef, manifestBytes, updateCounter]

    // Drop original state and pack struct
    bytecode.push(0x14); // PUSH4
    bytecode.push(0x52); // ROLL -> [..., state]
    bytecode.push(0x45); // DROP

    bytecode.push(0x14); // PUSH4
    bytecode.push(0xC0); // PACK
    // PACK reverses stack order; restore the field order expected by the devpack helper:
    // [hash, nef, manifestBytes, updateCounter]
    bytecode.push(0x4A); // DUP
    bytecode.push(0xD1); // REVERSEITEMS
}

pub(crate) fn emit_get_contract_script(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Syscalls.getContractScript is implemented as a convenience wrapper
    // that fetches the NEF file from a contract state.
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::ContractManagement,
        "getContract",
        1,
        use_callt,
        token_patches,
    );
    bytecode.push(0x13); // PUSH3 (ContractState.Nef field index)
    bytecode.push(0xCE); // PICKITEM
}

pub(crate) fn emit_get_neo_account_state(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // NeoToken.getAccountState(UInt160) returns NeoAccountState? (nullable) with shape:
    //   [balance, balanceHeight, voteTo (ECPoint bytes or null), lastGasPerVote]
    //
    // The devpack helper expects a non-null struct, so we replace `null` with a
    // default struct and normalize `voteTo` to an empty byte string.
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::Neo,
        "getAccountState",
        1,
        use_callt,
        token_patches,
    );

    // If result is null, replace with [0, 0, "", 0].
    bytecode.push(0x4A); // DUP
    bytecode.push(0xD8); // ISNULL
    let jmp_if_not_null_pos = bytecode.len();
    bytecode.push(0x27); // JMPIFNOT_L
    let jmp_if_not_null_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);

    // null path
    bytecode.push(0x45); // DROP
    bytecode.push(0x10); // PUSH0 (balance)
    bytecode.push(0x10); // PUSH0 (balanceHeight)
    push_data(bytecode, &[]); // empty voteTo
    bytecode.push(0x10); // PUSH0 (lastGasPerVote)
    bytecode.push(0x14); // PUSH4
    bytecode.push(0xC0); // PACK
    // PACK reverses stack order; restore [balance, balanceHeight, voteTo, lastGasPerVote].
    bytecode.push(0x4A); // DUP
    bytecode.push(0xD1); // REVERSEITEMS

    let jmp_end_pos = bytecode.len();
    bytecode.push(0x23); // JMP_L
    let jmp_end_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);

    // not-null path
    let not_null_pos = bytecode.len();

    // If voteTo (index 2) is null, set it to "".
    bytecode.push(0x4A); // DUP
    bytecode.push(0x12); // PUSH2
    bytecode.push(0xCE); // PICKITEM -> voteTo
    bytecode.push(0xD8); // ISNULL
    let jmp_vote_non_null_pos = bytecode.len();
    bytecode.push(0x27); // JMPIFNOT_L
    let jmp_vote_non_null_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);

    // voteTo is null path: state[2] = ""
    bytecode.push(0x4A); // DUP
    bytecode.push(0x12); // PUSH2
    push_data(bytecode, &[]);
    bytecode.push(0xD0); // SETITEM

    let vote_non_null_pos = bytecode.len();

    let rel_vote_non_null = (vote_non_null_pos as i32)
        .checked_sub(jmp_vote_non_null_pos as i32)
        .unwrap_or(0);
    bytecode[jmp_vote_non_null_operand..jmp_vote_non_null_operand + 4]
        .copy_from_slice(&rel_vote_non_null.to_le_bytes());

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
