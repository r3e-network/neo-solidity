use super::*;

pub(crate) fn require_native_method(
    native_methods: &mut BTreeMap<String, BTreeSet<String>>,
    contract: ir::NativeContract,
    method: &str,
) {
    let hash_le = bytecode::native_contract_hash(contract);
    let hash_be = hash_le.iter().rev().copied().collect::<Vec<_>>();
    let contract_str = format!("0x{}", hex::encode(hash_be));
    native_methods
        .entry(contract_str)
        .or_default()
        .insert(method.to_string());
}

pub(crate) fn collect_native_permissions(
    ir_module: &ir::Module,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut native_methods: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for function in &ir_module.functions {
        for block in &function.basic_blocks {
            for instr in &block.instructions {
                match instr {
                    ir::Instruction::CallBuiltin { builtin, .. } => match builtin {
                        ir::BuiltinCall::ContractCall | ir::BuiltinCall::ContractCallWithFlags => {
                            // Builtin lowering always deserializes args and serializes return.
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::StdLib,
                                "deserialize",
                            );
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::StdLib,
                                "serialize",
                            );
                        }
                        ir::BuiltinCall::NativeCall { contract, method } => {
                            // Backwards-compatible alias: older devpacks used
                            // `ContractManagement.listContracts`, but the Neo native contract
                            // exposes this as `getContractHashes`.
                            if matches!(contract, ir::NativeContract::ContractManagement)
                                && method == "listContracts"
                            {
                                require_native_method(
                                    &mut native_methods,
                                    ir::NativeContract::ContractManagement,
                                    "getContractHashes",
                                );
                            } else {
                                require_native_method(
                                    &mut native_methods,
                                    *contract,
                                    method.as_str(),
                                );
                            }
                        }
                        ir::BuiltinCall::DeployContract => {
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::ContractManagement,
                                "deploy",
                            );
                        }
                        ir::BuiltinCall::GetContract => {
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::ContractManagement,
                                "getContract",
                            );
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::StdLib,
                                "serialize",
                            );
                        }
                        ir::BuiltinCall::GetContractScript => {
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::ContractManagement,
                                "getContract",
                            );
                        }
                        ir::BuiltinCall::GetNeoAccountState => {
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::Neo,
                                "getAccountState",
                            );
                        }
                        ir::BuiltinCall::AbiEncode | ir::BuiltinCall::AbiEncodePacked => {
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::StdLib,
                                "serialize",
                            );
                        }
                        ir::BuiltinCall::AbiDecode => {
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::StdLib,
                                "deserialize",
                            );
                        }
                        ir::BuiltinCall::RuntimeNotify | ir::BuiltinCall::NotifySerialized => {
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::StdLib,
                                "deserialize",
                            );
                        }
                        ir::BuiltinCall::Keccak256 => {
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::CryptoLib,
                                "keccak256",
                            );
                        }
                        ir::BuiltinCall::Ecrecover | ir::BuiltinCall::PrecompileEcrecover => {
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::CryptoLib,
                                "recoverSecp256K1",
                            );
                            // Task #20: ecrecover lowers to
                            //   recoverSecp256K1 → SUBSTR(1,64) → keccak256 → RIGHT 20
                            // so keccak256 is also a required CryptoLib method.
                            // Task #H6a extends the same lowering to the 0x01
                            // precompile staticcall routing.
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::CryptoLib,
                                "keccak256",
                            );
                        }
                        ir::BuiltinCall::VerifySignature => {
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::CryptoLib,
                                "verifyWithECDsa",
                            );
                        }
                        ir::BuiltinCall::Syscall(name) => {
                            let _ = name;
                        }
                        _ => {}
                    },
                    ir::Instruction::LoadRuntimeValue(value) => {
                        if matches!(value, ir::RuntimeValue::BlockNumber) {
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::Ledger,
                                "currentIndex",
                            );
                        }
                    }
                    ir::Instruction::LoadMappingElement { key_types, .. }
                    | ir::Instruction::StoreMappingElement { key_types, .. }
                        if !key_types.is_empty() =>
                    {
                        // Mapping storage slots are derived via:
                        //   StdLib.serialize(key) + CryptoLib.keccak256(...)
                        //
                        // When `key_types` is empty, the slot is the base slot and no hashing is
                        // performed, so avoid adding unnecessary permissions.
                        require_native_method(
                            &mut native_methods,
                            ir::NativeContract::StdLib,
                            "serialize",
                        );
                        require_native_method(
                            &mut native_methods,
                            ir::NativeContract::CryptoLib,
                            "keccak256",
                        );
                    }
                    ir::Instruction::LoadStructField { key_types, .. }
                    | ir::Instruction::StoreStructField { key_types, .. } => {
                        // Struct field storage slots always require at least one keccak256
                        // (base slot + field key). If the struct is nested under a mapping,
                        // additional key serialization + hashing occurs.
                        require_native_method(
                            &mut native_methods,
                            ir::NativeContract::CryptoLib,
                            "keccak256",
                        );
                        if !key_types.is_empty() {
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::StdLib,
                                "serialize",
                            );
                        }
                    }
                    ir::Instruction::LoadStructArrayElement { key_types, .. }
                    | ir::Instruction::StoreStructArrayElement { key_types, .. } => {
                        // Array elements stored under a struct field require:
                        // - keccak256 for the field-slot derivation and the element-slot derivation
                        // - StdLib.serialize for the element index (and any mapping keys)
                        require_native_method(
                            &mut native_methods,
                            ir::NativeContract::CryptoLib,
                            "keccak256",
                        );
                        require_native_method(
                            &mut native_methods,
                            ir::NativeContract::StdLib,
                            "serialize",
                        );
                        if !key_types.is_empty() {
                            // mapping keys also require serialize/keccak256; already covered above,
                            // but keep this for clarity in case the rules evolve.
                            require_native_method(
                                &mut native_methods,
                                ir::NativeContract::CryptoLib,
                                "keccak256",
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    native_methods
}
