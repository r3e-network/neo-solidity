fn resolve_syscalls_member(member: &str) -> Option<BuiltinCall> {
    match member {
        "contractCall" => Some(BuiltinCall::ContractCall),
        "contractCallWithFlags" => Some(BuiltinCall::ContractCallWithFlags),
        // Neo N3 contract management is implemented via the native ContractManagement contract.
        // Keep compatibility with older devpack helpers that model these as syscalls.
        "contractCreate" => Some(BuiltinCall::DeployContract),
        "contractUpdate" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::ContractManagement,
            method: "update".to_string(),
        }),
        "contractDestroy" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::ContractManagement,
            method: "destroy".to_string(),
        }),
        "createStandardAccount" => Some(BuiltinCall::Syscall(
            "System.Contract.CreateStandardAccount".to_string(),
        )),
        "createMultisigAccount" => Some(BuiltinCall::Syscall(
            "System.Contract.CreateMultisigAccount".to_string(),
        )),
        "notify" => Some(BuiltinCall::NotifySerialized),
        "getCurrentIndex" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Ledger,
            method: "currentIndex".to_string(),
        }),
        "getBlock" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Ledger,
            method: "getBlock".to_string(),
        }),
        "getTransaction" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Ledger,
            method: "getTransaction".to_string(),
        }),
        "getTransactionHeight" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Ledger,
            method: "getTransactionHeight".to_string(),
        }),
        "getTransactionFromBlock" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Ledger,
            method: "getTransactionFromBlock".to_string(),
        }),
        "getExecutingScriptHash" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetExecutingScriptHash".to_string(),
        )),
        "getCallingScriptHash" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetCallingScriptHash".to_string(),
        )),
        "getEntryScriptHash" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetEntryScriptHash".to_string(),
        )),
        "getStorageContext" => Some(BuiltinCall::Syscall("System.Storage.GetContext".to_string())),
        "getReadOnlyStorageContext" => Some(BuiltinCall::Syscall(
            "System.Storage.GetReadOnlyContext".to_string(),
        )),
        "storageGet" => Some(BuiltinCall::Syscall("System.Storage.Get".to_string())),
        "storagePut" => Some(BuiltinCall::Syscall("System.Storage.Put".to_string())),
        "storageDelete" => Some(BuiltinCall::Syscall("System.Storage.Delete".to_string())),
        "storageFind" => Some(BuiltinCall::Syscall("System.Storage.Find".to_string())),
        "checkWitness" => Some(BuiltinCall::Syscall("System.Runtime.CheckWitness".to_string())),
        "getTime" => Some(BuiltinCall::Syscall("System.Runtime.GetTime".to_string())),
        "gasLeft" => Some(BuiltinCall::Syscall("System.Runtime.GasLeft".to_string())),
        "getPlatform" => Some(BuiltinCall::Syscall("System.Runtime.Platform".to_string())),
        "getTrigger" => Some(BuiltinCall::Syscall("System.Runtime.GetTrigger".to_string())),
        "getNotifications" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetNotifications".to_string(),
        )),
        "log" => Some(BuiltinCall::Syscall("System.Runtime.Log".to_string())),
        "sha256" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::CryptoLib,
            method: "sha256".to_string(),
        }),
        "ripemd160" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::CryptoLib,
            method: "ripemd160".to_string(),
        }),
        "verifyWithECDsa" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::CryptoLib,
            method: "verifyWithECDsa".to_string(),
        }),
        "murmur32" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::CryptoLib,
            method: "murmur32".to_string(),
        }),
        "jsonSerialize" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::StdLib,
            method: "jsonSerialize".to_string(),
        }),
        "jsonDeserialize" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::StdLib,
            method: "jsonDeserialize".to_string(),
        }),
        "base64Encode" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::StdLib,
            method: "base64Encode".to_string(),
        }),
        "base64Decode" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::StdLib,
            method: "base64Decode".to_string(),
        }),
        "iteratorNext" => Some(BuiltinCall::Syscall("System.Iterator.Next".to_string())),
        "iteratorValue" => Some(BuiltinCall::Syscall("System.Iterator.Value".to_string())),
        "getCurrentRandom" => Some(BuiltinCall::Syscall("System.Runtime.GetRandom".to_string())),
        "getNetwork" => Some(BuiltinCall::Syscall("System.Runtime.GetNetwork".to_string())),
        "getAddressVersion" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetAddressVersion".to_string(),
        )),
        "burnGas" => Some(BuiltinCall::Syscall("System.Runtime.BurnGas".to_string())),
        "getInvocationCounter" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetInvocationCounter".to_string(),
        )),
        "getFeePerByte" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "getFeePerByte".to_string(),
        }),
        "getExecFeeFactor" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "getExecFeeFactor".to_string(),
        }),
        "getStoragePrice" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "getStoragePrice".to_string(),
        }),
        "isBlocked" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "isBlocked".to_string(),
        }),
        "oracleRequest" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Oracle,
            method: "request".to_string(),
        }),
        "getOraclePrice" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Oracle,
            method: "getPrice".to_string(),
        }),
        "getDesignatedByRole" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::RoleManagement,
            method: "getDesignatedByRole".to_string(),
        }),
        "hasRole" => None,
        "getContractScript" => Some(BuiltinCall::GetContractScript),
        "contractExists" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::ContractManagement,
            method: "isContract".to_string(),
        }),
        _ => None,
    }
}
