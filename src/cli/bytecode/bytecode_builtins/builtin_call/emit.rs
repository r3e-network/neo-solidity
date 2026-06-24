pub(crate) fn emit_builtin_call(
    bytecode: &mut Vec<u8>,
    builtin: &ir::BuiltinCall,
    arg_count: usize,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    match builtin {
        ir::BuiltinCall::RuntimeNotify => {
            emit_runtime_notify(bytecode, use_callt, token_patches);
        }
        ir::BuiltinCall::RuntimeCheckWitness => emit_runtime_check_witness(bytecode),
        ir::BuiltinCall::Keccak256 => emit_keccak256(bytecode, use_callt, token_patches),
        ir::BuiltinCall::Ecrecover => emit_ecrecover(bytecode, use_callt, token_patches),
        ir::BuiltinCall::PrecompileEcrecover => {
            emit_precompile_ecrecover(bytecode, use_callt, token_patches)
        }
        ir::BuiltinCall::PrecompileModexp => emit_precompile_modexp(bytecode, use_callt),
        ir::BuiltinCall::StorageFind => emit_storage_find(bytecode, arg_count),
        ir::BuiltinCall::StoragePut => emit_storage_put(bytecode),
        ir::BuiltinCall::StorageGet => emit_storage_get(bytecode),
        ir::BuiltinCall::StorageDelete => emit_storage_delete(bytecode),
        ir::BuiltinCall::AbiEncode | ir::BuiltinCall::AbiEncodeCall => {
            emit_abi_encode(bytecode, arg_count, use_callt, token_patches)
        }
        ir::BuiltinCall::AbiEncodePacked => {
            emit_abi_encode_packed(bytecode, arg_count, use_callt, token_patches)
        }
        ir::BuiltinCall::AbiDecode => emit_abi_decode(bytecode, use_callt, token_patches),
        ir::BuiltinCall::NativeCall { contract, method } => emit_native_call(
            bytecode,
            *contract,
            method.as_str(),
            arg_count,
            use_callt,
            token_patches,
        ),
        ir::BuiltinCall::Syscall(name) => emit_syscall_builtin(bytecode, name, arg_count),
        ir::BuiltinCall::ContractCall => emit_contract_call(bytecode, use_callt, token_patches),
        ir::BuiltinCall::ContractCallWithFlags => {
            emit_contract_call_with_flags(bytecode, use_callt, token_patches)
        }
        ir::BuiltinCall::NotifySerialized => {
            emit_notify_serialized(bytecode, use_callt, token_patches)
        }
        ir::BuiltinCall::VerifySignature => emit_verify_signature(bytecode, use_callt, token_patches),
        ir::BuiltinCall::DeployContract => {
            emit_deploy_contract(bytecode, arg_count, use_callt, token_patches)
        }
        ir::BuiltinCall::GetContract => emit_get_contract(bytecode, use_callt, token_patches),
        ir::BuiltinCall::GetContractScript => {
            emit_get_contract_script(bytecode, use_callt, token_patches)
        }
        ir::BuiltinCall::GetNeoAccountState => {
            emit_get_neo_account_state(bytecode, use_callt, token_patches)
        }
        ir::BuiltinCall::TypeOf | ir::BuiltinCall::AbiEncodeWithSignature => {}
        ir::BuiltinCall::BytesConcat => {
            // bytes.concat(a, b, c, ...) → chain NeoVM CAT opcodes.
            // Args are already on the stack. For N args we need N-1 CAT ops.
            // CAT pops two byte arrays and pushes their concatenation.
            for _ in 1..arg_count {
                bytecode.push(0x8B); // CAT opcode
            }
            // If zero args, push empty byte array.
            if arg_count == 0 {
                bytecode.push(0x00); // PUSH0 (empty buffer)
                bytecode.push(0x28); // NEWBUFFER
            }
        }
    }
}
