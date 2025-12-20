fn resolve_builtin_call(expr: &Expression) -> Option<BuiltinCall> {
    if let Expression::MemberAccess(_, inner, member) = expr {
        if let Expression::Variable(base) = inner.as_ref() {
            let member_name = member.name.as_str();
            match base.name.as_str() {
                "Runtime" => return resolve_runtime_member(member_name),
                "abi" => return resolve_abi_member(member_name),
                "Storage" => return resolve_storage_member(member_name),
                "Syscalls" => return resolve_syscalls_member(member_name),
                "NativeCalls" => return resolve_native_calls_member(member_name),
                "Neo" => return resolve_neo_member(member_name),
                _ => {}
            }
        }
    }

    if let Expression::Variable(identifier) = expr {
        if identifier.name == "ecrecover" {
            return Some(BuiltinCall::Ecrecover);
        }
        if identifier.name == "keccak256" {
            return Some(BuiltinCall::Keccak256);
        }
        if identifier.name == "type" {
            return Some(BuiltinCall::TypeOf);
        }
    }

    None
}

fn builtin_library_supported_members(base: &str) -> Option<&'static [&'static str]> {
    match base {
        "Runtime" => Some(&[
            "notify",
            "notifyIndexed",
            "checkWitness",
            "gasLeft",
            "burnGas",
            "log",
            "getTime",
            "getTrigger",
            "getInvocationCounter",
            "initializeServices",
        ]),
        "abi" => Some(&["encode", "encodePacked", "encodeWithSignature", "decode"]),
        "Storage" => Some(&[
            "find",
            "put",
            "get",
            "remove",
            "initializeContext",
            "getContext",
            "getReadOnlyContext",
            "getUsage",
            "putContractMetadata",
        ]),
        "Syscalls" => Some(&[
            "contractCall",
            "contractCallWithFlags",
            "contractCreate",
            "contractUpdate",
            "contractDestroy",
            "createStandardAccount",
            "createMultisigAccount",
            "notify",
            "getCurrentIndex",
            "getBlock",
            "getTransaction",
            "getTransactionHeight",
            "getTransactionFromBlock",
            "getExecutingScriptHash",
            "getCallingScriptHash",
            "getEntryScriptHash",
            "getStorageContext",
            "getReadOnlyStorageContext",
            "storageGet",
            "storagePut",
            "storageDelete",
            "storageFind",
            "checkWitness",
            "getTime",
            "gasLeft",
            "getPlatform",
            "getTrigger",
            "getNotifications",
            "log",
            "sha256",
            "ripemd160",
            "verifyWithECDsa",
            "murmur32",
            "jsonSerialize",
            "jsonDeserialize",
            "base64Encode",
            "base64Decode",
            "iteratorNext",
            "iteratorValue",
            "getCurrentRandom",
            "getNetwork",
            "getAddressVersion",
            "burnGas",
            "getInvocationCounter",
            "getFeePerByte",
            "getExecFeeFactor",
            "getStoragePrice",
            "isBlocked",
            "oracleRequest",
            "getOraclePrice",
            "getDesignatedByRole",
            "scriptHashToAddress",
            "addressToScriptHash",
            "isValidAddress",
            "getContractScript",
            "contractExists",
        ]),
        "NativeCalls" => Some(&[
            "neoTotalSupply",
            "neoBalanceOf",
            "neoTransfer",
            "vote",
            "getCandidates",
            "registerCandidate",
            "unregisterCandidate",
            "getGasPerBlock",
            "setGasPerBlock",
            "getAccountState",
            "getCommittee",
            "isCommittee",
            "getNextBlockValidators",
            "isValidator",
            "gasTotalSupply",
            "gasBalanceOf",
            "gasTransfer",
            "deployContract",
            "updateContract",
            "destroyContract",
            "getContract",
            "listContracts",
            "hasMethod",
            "getMinimumDeploymentFee",
            "setMinimumDeploymentFee",
            "getFeePerByte",
            "setFeePerByte",
            "getExecFeeFactor",
            "setExecFeeFactor",
            "getStoragePrice",
            "setStoragePrice",
            "blockAccount",
            "unblockAccount",
            "isBlocked",
            "requestOracleData",
            "getOraclePrice",
            "setOraclePrice",
            "designateAsRole",
            "getDesignatedByRole",
        ]),
        "Neo" => Some(&[
            "verifySignature",
            "callContract",
            "deployContract",
            "getNeoBalance",
            "getGasBalance",
            "getGasPrice",
            "getStoragePrice",
            "getCommittee",
            "getValidators",
            "isCommittee",
            "isValidator",
            "getRandom",
        ]),
        _ => None,
    }
}

fn resolve_runtime_member(member: &str) -> Option<BuiltinCall> {
    match member {
        "notify" => Some(BuiltinCall::RuntimeNotify),
        "checkWitness" => Some(BuiltinCall::RuntimeCheckWitness),
        "gasLeft" => Some(BuiltinCall::Syscall("System.Runtime.GasLeft".to_string())),
        "burnGas" => Some(BuiltinCall::Syscall("System.Runtime.BurnGas".to_string())),
        "log" => Some(BuiltinCall::Syscall("System.Runtime.Log".to_string())),
        "getTime" => Some(BuiltinCall::Syscall("System.Runtime.GetTime".to_string())),
        "getTrigger" => Some(BuiltinCall::Syscall("System.Runtime.GetTrigger".to_string())),
        "getInvocationCounter" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetInvocationCounter".to_string(),
        )),
        _ => None,
    }
}

fn resolve_abi_member(member: &str) -> Option<BuiltinCall> {
    match member {
        "encode" => Some(BuiltinCall::AbiEncode),
        "encodePacked" => Some(BuiltinCall::AbiEncodePacked),
        "encodeWithSignature" => Some(BuiltinCall::AbiEncodeWithSignature),
        "decode" => Some(BuiltinCall::AbiDecode),
        _ => None,
    }
}

fn resolve_storage_member(member: &str) -> Option<BuiltinCall> {
    match member {
        "find" => Some(BuiltinCall::StorageFind),
        "put" => Some(BuiltinCall::StoragePut),
        "get" => Some(BuiltinCall::StorageGet),
        "remove" => Some(BuiltinCall::StorageDelete),
        "initializeContext" => Some(BuiltinCall::Syscall("System.Storage.GetContext".to_string())),
        "getContext" => Some(BuiltinCall::Syscall("System.Storage.GetContext".to_string())),
        "getReadOnlyContext" => Some(BuiltinCall::Syscall(
            "System.Storage.GetReadOnlyContext".to_string(),
        )),
        _ => None,
    }
}

fn resolve_neo_member(member: &str) -> Option<BuiltinCall> {
    match member {
        "verifySignature" => Some(BuiltinCall::VerifySignature),
        "callContract" => Some(BuiltinCall::ContractCall),
        "deployContract" => Some(BuiltinCall::DeployContract),
        "getNeoBalance" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "balanceOf".to_string(),
        }),
        "getGasBalance" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Gas,
            method: "balanceOf".to_string(),
        }),
        "getGasPrice" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "getFeePerByte".to_string(),
        }),
        "getStoragePrice" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "getStoragePrice".to_string(),
        }),
        "getCommittee" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "getCommittee".to_string(),
        }),
        "getRandom" => Some(BuiltinCall::Syscall("System.Runtime.GetRandom".to_string())),
        _ => None,
    }
}
