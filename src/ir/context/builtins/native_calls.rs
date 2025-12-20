fn resolve_native_calls_member(member: &str) -> Option<BuiltinCall> {
    match member {
        // ========== NEO native contract ==========
        "neoTotalSupply" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "totalSupply".to_string(),
        }),
        "neoBalanceOf" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "balanceOf".to_string(),
        }),
        "neoTransfer" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "transfer".to_string(),
        }),
        "vote" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "vote".to_string(),
        }),
        "getCandidates" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "getCandidates".to_string(),
        }),
        "registerCandidate" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "registerCandidate".to_string(),
        }),
        "unregisterCandidate" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "unregisterCandidate".to_string(),
        }),
        "getGasPerBlock" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "getGasPerBlock".to_string(),
        }),
        "setGasPerBlock" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "setGasPerBlock".to_string(),
        }),
        "getAccountState" => Some(BuiltinCall::GetNeoAccountState),
        "getCommittee" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "getCommittee".to_string(),
        }),
        "getNextBlockValidators" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "getNextBlockValidators".to_string(),
        }),

        // ========== GAS native contract ==========
        "gasTotalSupply" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Gas,
            method: "totalSupply".to_string(),
        }),
        "gasBalanceOf" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Gas,
            method: "balanceOf".to_string(),
        }),
        "gasTransfer" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Gas,
            method: "transfer".to_string(),
        }),

        // ========== ContractManagement native contract ==========
        "deployContract" => Some(BuiltinCall::DeployContract),
        "updateContract" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::ContractManagement,
            method: "update".to_string(),
        }),
        "destroyContract" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::ContractManagement,
            method: "destroy".to_string(),
        }),
        "getContract" => Some(BuiltinCall::GetContract),
        "listContracts" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::ContractManagement,
            method: "getContractHashes".to_string(),
        }),
        "hasMethod" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::ContractManagement,
            method: "hasMethod".to_string(),
        }),
        "getMinimumDeploymentFee" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::ContractManagement,
            method: "getMinimumDeploymentFee".to_string(),
        }),
        "setMinimumDeploymentFee" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::ContractManagement,
            method: "setMinimumDeploymentFee".to_string(),
        }),

        // ========== Policy native contract ==========
        "getFeePerByte" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "getFeePerByte".to_string(),
        }),
        "setFeePerByte" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "setFeePerByte".to_string(),
        }),
        "getExecFeeFactor" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "getExecFeeFactor".to_string(),
        }),
        "setExecFeeFactor" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "setExecFeeFactor".to_string(),
        }),
        "getStoragePrice" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "getStoragePrice".to_string(),
        }),
        "setStoragePrice" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "setStoragePrice".to_string(),
        }),
        "blockAccount" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "blockAccount".to_string(),
        }),
        "unblockAccount" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "unblockAccount".to_string(),
        }),
        "isBlocked" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Policy,
            method: "isBlocked".to_string(),
        }),

        // ========== Oracle native contract ==========
        "requestOracleData" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Oracle,
            method: "request".to_string(),
        }),
        "getOraclePrice" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Oracle,
            method: "getPrice".to_string(),
        }),
        "setOraclePrice" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Oracle,
            method: "setPrice".to_string(),
        }),

        // ========== RoleManagement native contract ==========
        "designateAsRole" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::RoleManagement,
            method: "designateAsRole".to_string(),
        }),
        "getDesignatedByRole" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::RoleManagement,
            method: "getDesignatedByRole".to_string(),
        }),

        _ => None,
    }
}
