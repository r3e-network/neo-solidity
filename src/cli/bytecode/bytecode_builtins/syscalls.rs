fn is_void_syscall(name: &str) -> bool {
    matches!(
        name,
        "System.Storage.Put"
            | "System.Storage.Delete"
            | "System.Runtime.Notify"
            | "System.Runtime.Log"
            | "System.Runtime.BurnGas"
            | "System.Runtime.LoadScript"
    )
}

pub(crate) fn emit_native_contract_call(
    bytecode: &mut Vec<u8>,
    contract: ir::NativeContract,
    method: &str,
    arg_count: usize,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Backwards-compatible aliases for older devpack method names.
    let method = match (contract, method) {
        (ir::NativeContract::ContractManagement, "listContracts") => "getContractHashes",
        _ => method,
    };

    let call_flags = native_method_call_flags(contract, method);

    if use_callt {
        if let Ok(params) = u16::try_from(arg_count) {
            if !method.starts_with('_')
                && method.len() <= neo_devpack_solidity::neo::MAX_TOKEN_METHOD_LENGTH
            {
                if let Some(has_return_value) = native_method_has_return_value(contract, method) {
                    bytecode.push(0x37); // CALLT
                    let position = bytecode.len();
                    bytecode.extend_from_slice(&[0, 0]); // token index placeholder

                    token_patches.push(MethodTokenPatch {
                        position,
                        token: MethodTokenKey {
                            hash: native_contract_hash(contract),
                            method: method.to_string(),
                            parameters_count: params,
                            has_return_value,
                            call_flags,
                        },
                    });

                    if !has_return_value {
                        // CALLT will not push a return value for void methods; keep stack behavior
                        // consistent with other void builtins by pushing a placeholder item.
                        bytecode.push(0x11); // PUSH1
                    }
                    return;
                }
            }
        }
    }

    // Build argument array for System.Contract.Call.
    let arg_count_bigint = BigInt::from(arg_count);
    push_integer_bigint(bytecode, &arg_count_bigint);
    bytecode.push(0xC0); // PACK
                         // PACK pops values from the stack and reverses their order. Native contract
                         // calls must preserve the original argument ordering.
    if arg_count > 1 {
        bytecode.push(0x4A); // DUP (keep a reference to the array on the stack)
        bytecode.push(0xD1); // REVERSEITEMS (consumes one reference, reverses in-place)
    }

    // Push call flags, method name, and contract hash.
    push_integer_bigint(bytecode, &BigInt::from(call_flags));
    push_data(bytecode, method.as_bytes());
    let hash = native_contract_hash(contract);
    push_data(bytecode, &hash);

    // `System.Contract.Call` expects stack order: [args, flags, method, hash]
    // (top-of-stack is `hash`, popped first).
    emit_syscall(bytecode, "System.Contract.Call");
}

fn native_method_call_flags(contract: ir::NativeContract, method: &str) -> u8 {
    match native_method_is_mutating(contract, method) {
        Some(true) => CALLFLAGS_ALL,
        Some(false) => CALLFLAGS_READ_ONLY,
        None => CALLFLAGS_ALL,
    }
}

fn native_method_is_mutating(contract: ir::NativeContract, method: &str) -> Option<bool> {
    use ir::NativeContract::{
        ContractManagement, Gas, Neo, Notary, Oracle, Policy, RoleManagement,
    };

    let is_mutating = match (contract, method) {
        (
            Neo,
            "transfer"
            | "vote"
            | "registerCandidate"
            | "unregisterCandidate"
            | "setGasPerBlock"
            | "setRegisterPrice",
        ) => true,
        (Gas, "transfer") => true,
        (ContractManagement, "deploy" | "update" | "destroy" | "setMinimumDeploymentFee") => true,
        (
            Policy,
            "setFeePerByte"
            | "setExecFeeFactor"
            | "setStoragePrice"
            | "setAttributeFee"
            | "setMillisecondsPerBlock"
            | "setMaxValidUntilBlockIncrement"
            | "setMaxTraceableBlocks"
            | "blockAccount"
            | "unblockAccount"
            | "recoverFund"
            | "setWhitelistFeeContract"
            | "removeWhitelistFeeContract",
        ) => true,
        (Oracle, "request" | "setPrice" | "finish") => true,
        (RoleManagement, "designateAsRole") => true,
        (
            Notary,
            "lockDepositUntil" | "withdraw" | "setMaxNotValidBeforeDelta" | "onNEP17Payment",
        ) => true,
        _ => {
            if native_method_has_return_value(contract, method).is_some() {
                return Some(false);
            }
            return None;
        }
    };

    Some(is_mutating)
}

fn native_method_has_return_value(contract: ir::NativeContract, method: &str) -> Option<bool> {
    use ir::NativeContract::{
        ContractManagement, CryptoLib, Gas, Ledger, Neo, Notary, Oracle, Policy, RoleManagement,
        StdLib, Treasury,
    };

    let has_return = match (contract, method) {
        (
            StdLib,
            "serialize" | "deserialize" | "jsonSerialize" | "jsonDeserialize" | "base64Encode"
            | "base64Decode" | "base64UrlEncode" | "base64UrlDecode" | "base58Encode"
            | "base58Decode" | "base58CheckEncode" | "base58CheckDecode" | "hexEncode"
            | "hexDecode" | "itoa" | "atoi" | "memoryCompare" | "memorySearch" | "stringSplit"
            | "strLen",
        ) => true,
        (
            CryptoLib,
            "sha256"
            | "ripemd160"
            | "keccak256"
            | "murmur32"
            | "recoverSecp256K1"
            | "verifyWithECDsa"
            | "verifyWithEd25519"
            | "bls12381Serialize"
            | "bls12381Deserialize"
            | "bls12381Equal"
            | "bls12381Add"
            | "bls12381Mul"
            | "bls12381Pairing",
        ) => true,
        (
            Ledger,
            "currentIndex"
            | "getBlock"
            | "getTransaction"
            | "getTransactionHeight"
            | "getTransactionFromBlock",
        ) => true,
        (
            Neo,
            "symbol"
            | "decimals"
            | "totalSupply"
            | "balanceOf"
            | "transfer"
            | "vote"
            | "getCandidates"
            | "registerCandidate"
            | "unregisterCandidate"
            | "getGasPerBlock"
            | "getRegisterPrice"
            | "getAccountState"
            | "unclaimedGas"
            | "getCandidateVote"
            | "getCommittee"
            | "getCommitteeAddress"
            | "getNextBlockValidators"
            | "getAllCandidates",
        ) => true,
        (Neo, "setGasPerBlock" | "setRegisterPrice") => false,
        (Gas, "symbol" | "decimals" | "totalSupply" | "balanceOf" | "transfer") => true,
        (
            ContractManagement,
            "deploy"
            | "getContract"
            | "getContractById"
            | "getContractHashes"
            | "hasMethod"
            | "isContract"
            | "getMinimumDeploymentFee",
        ) => true,
        (ContractManagement, "update" | "destroy" | "setMinimumDeploymentFee") => false,
        (
            Policy,
            "getFeePerByte"
            | "getExecFeeFactor"
            | "getExecPicoFeeFactor"
            | "getStoragePrice"
            | "getMillisecondsPerBlock"
            | "getMaxValidUntilBlockIncrement"
            | "getMaxTraceableBlocks"
            | "getAttributeFee"
            | "blockAccount"
            | "unblockAccount"
            | "isBlocked"
            | "getBlockedAccounts"
            | "recoverFund"
            | "getWhitelistFeeContracts",
        ) => true,
        (
            Policy,
            "setFeePerByte"
            | "setExecFeeFactor"
            | "setStoragePrice"
            | "setAttributeFee"
            | "setMillisecondsPerBlock"
            | "setMaxValidUntilBlockIncrement"
            | "setMaxTraceableBlocks"
            | "setWhitelistFeeContract"
            | "removeWhitelistFeeContract",
        ) => false,
        (Oracle, "getPrice" | "verify") => true,
        (Oracle, "request" | "setPrice" | "finish") => false,
        (RoleManagement, "getDesignatedByRole") => true,
        (RoleManagement, "designateAsRole") => false,
        (
            Notary,
            "balanceOf"
            | "expirationOf"
            | "lockDepositUntil"
            | "withdraw"
            | "getMaxNotValidBeforeDelta"
            | "verify",
        ) => true,
        (Notary, "setMaxNotValidBeforeDelta" | "onNEP17Payment") => false,
        (Treasury, "verify") => true,
        (Treasury, "onNEP17Payment" | "onNEP11Payment") => false,
        (Ledger, "currentHash" | "getTransactionSigners" | "getTransactionVMState") => true,
        _ => return None,
    };

    Some(has_return)
}

pub(crate) fn emit_syscall(bytecode: &mut Vec<u8>, name: &str) {
    bytecode.push(0x41);
    bytecode.extend_from_slice(&crate::interop::interop_id_bytes(name));
}
