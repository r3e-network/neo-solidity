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
                // Task — `StdLib.<method>(...)` and `CryptoLib.<method>(...)`
                // in Solidity source must lower to a `BuiltinCall::NativeCall`
                // to the matching Neo N3 native contract. Before this was
                // wired, the compiler fell through to the generic function-
                // call handler, which evaluated and dropped the args then
                // pushed PUSH0 as the result. Fuzz regression: the
                // Solidity→bytecode path for `StdLib.itoa(12345, 10)` and
                // `CryptoLib.sha256(bytes)` returned zeros at runtime even
                // though the underlying native implementations are correct
                // (see baseline_tests.rs::callt_stdlib_itoa_roundtrip_via_token).
                "StdLib" => return resolve_stdlib_member(member_name),
                "CryptoLib" => return resolve_cryptolib_member(member_name),
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
        // Solidity exposes `sha256(bytes)` and `ripemd160(bytes)` as
        // global built-in hashers (EVM precompiles 0x02 and 0x03). Route
        // them to the CryptoLib native contract's `sha256`/`ripemd160`
        // methods. Before this wiring the compiler fell through to the
        // generic function-call handler, which evaluated and dropped the
        // argument and pushed 0 — so `sha256(b"abc")` returned 8 zero
        // bytes at runtime instead of the canonical digest. `keccak256`
        // already has a dedicated `BuiltinCall::Keccak256` variant
        // because its EVM semantics differ from Neo's default; `sha256`
        // and `ripemd160` have a direct 1:1 native mapping.
        if identifier.name == "sha256" {
            return Some(BuiltinCall::NativeCall {
                contract: NativeContract::CryptoLib,
                method: "sha256".to_string(),
            });
        }
        if identifier.name == "ripemd160" {
            return Some(BuiltinCall::NativeCall {
                contract: NativeContract::CryptoLib,
                method: "ripemd160".to_string(),
            });
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
            "requireWitness",
            "checkAnyWitness",
            "checkAllWitnesses",
            "checkMultiSigWitness",
            "gasLeft",
            "burnGas",
            "log",
            "getTime",
            "getTrigger",
            "getInvocationCounter",
            "getCurrentSigners",
            "getCallFlags",
            "getScriptContainer",
            "loadScript",
            "initializeServices",
            "getNetwork",
            "getPlatform",
            "getAddressVersion",
            "getRandom",
            "getExecutingScriptHash",
            "getCallingScriptHash",
            "getEntryScriptHash",
        ]),
        "abi" => Some(&[
            "encode",
            "encodePacked",
            "encodeCall",
            "encodeWithSignature",
            "encodeWithSelector",
            "decode",
        ]),
        "Storage" => Some(&[
            "find",
            "put",
            "get",
            "remove",
            "findLocal",
            "putLocal",
            "getLocal",
            "removeLocal",
            "asReadOnly",
            "initializeContext",
            "getContext",
            "getReadOnlyContext",
            "getUsage",
            "putContractMetadata",
            "batchPut",
            "batchGet",
            "batchDelete",
            "count",
            "countLocal",
            "findValues",
            "findLocalValues",
            "findKeys",
            "findLocalKeys",
            "clearPrefix",
            "exists",
            "isValidKey",
        ]),
        "Syscalls" => Some(&[
            "contractCall",
            "contractCallWithFlags",
            "getCallFlags",
            "contractCreate",
            "contractUpdate",
            "contractDestroy",
            "createStandardAccount",
            "createMultisigAccount",
            "notify",
            "getCurrentIndex",
            "getCurrentHash",
            "getBlock",
            "getTransaction",
            "getTransactionHeight",
            "getTransactionFromBlock",
            "getTransactionSigners",
            "getTransactionVMState",
            "getExecutingScriptHash",
            "getCallingScriptHash",
            "getEntryScriptHash",
            "getScriptContainer",
            "loadScript",
            "getStorageContext",
            "getReadOnlyStorageContext",
            "storageAsReadOnly",
            "storageGet",
            "storagePut",
            "storageDelete",
            "storageFind",
            "storageGetLocal",
            "storagePutLocal",
            "storageDeleteLocal",
            "storageFindLocal",
            "checkWitness",
            "getTime",
            "gasLeft",
            "getPlatform",
            "getTrigger",
            "getNotifications",
            "log",
            "getCurrentSigners",
            "checkSig",
            "checkMultisig",
            "sha256",
            "ripemd160",
            "verifyWithECDsa",
            "murmur32",
            "keccak256",
            "recoverSecp256K1",
            "verifyWithEd25519",
            "bls12381Serialize",
            "bls12381Deserialize",
            "bls12381Equal",
            "bls12381Add",
            "bls12381Mul",
            "bls12381Pairing",
            // G1/G2 affine ops mirror the CryptoLib.* path resolver at
            // roughly line 585. Keeping them out of the Syscalls whitelist
            // would make `Syscalls.bls12381G1Add(...)` fail with an
            // "unsupported builtin library call" diagnostic even though the
            // CryptoLib side accepts the call — the audit agent flagged
            // this asymmetry.
            "bls12381G1Add",
            "bls12381G1Mul",
            "bls12381G1Neg",
            "bls12381G2Add",
            "bls12381G2Mul",
            "bls12381G2Neg",
            // Alias for `keccak256` — the devpack's `Syscalls.sol` exposes
            // it as `neoKeccak256` so the name doesn't shadow Solidity's
            // bare `keccak256` intrinsic. Unwired here before the audit.
            "neoKeccak256",
            "serialize",
            "deserialize",
            "itoa",
            "atoi",
            "jsonSerialize",
            "jsonDeserialize",
            "base64Encode",
            "base64Decode",
            "base64UrlEncode",
            "base64UrlDecode",
            "base58Encode",
            "base58Decode",
            "base58CheckEncode",
            "base58CheckDecode",
            "hexEncode",
            "hexDecode",
            "memoryCompare",
            "memorySearch",
            "stringSplit",
            "strLen",
            "iteratorNext",
            "iteratorValue",
            "getCurrentRandom",
            "getNetwork",
            "getAddressVersion",
            "burnGas",
            "getInvocationCounter",
            "getFeePerByte",
            "getExecFeeFactor",
            "getExecPicoFeeFactor",
            "getStoragePrice",
            "getMillisecondsPerBlock",
            "getMaxValidUntilBlockIncrement",
            "getMaxTraceableBlocks",
            "getAttributeFee",
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
            "neoDecimals",
            "neoSymbol",
            "neoName",
            "vote",
            "getCandidates",
            "registerCandidate",
            "unregisterCandidate",
            "getGasPerBlock",
            "getRegisterPrice",
            "setRegisterPrice",
            "setGasPerBlock",
            "getAccountState",
            "unclaimedGas",
            "getCandidateVote",
            "getCommittee",
            "getCommitteeAddress",
            "isCommittee",
            "getNextBlockValidators",
            "getAllCandidates",
            "isValidator",
            "gasTotalSupply",
            "gasBalanceOf",
            "gasTransfer",
            "gasDecimals",
            "gasSymbol",
            "gasName",
            "deployContract",
            "updateContract",
            "destroyContract",
            "getContract",
            "listContracts",
            "hasMethod",
            "getMinimumDeploymentFee",
            "setMinimumDeploymentFee",
            "getContractById",
            "isContract",
            "getFeePerByte",
            "setFeePerByte",
            "getExecFeeFactor",
            "getExecPicoFeeFactor",
            "setExecFeeFactor",
            "getStoragePrice",
            "getMillisecondsPerBlock",
            "setMillisecondsPerBlock",
            "getMaxValidUntilBlockIncrement",
            "setMaxValidUntilBlockIncrement",
            "getMaxTraceableBlocks",
            "setMaxTraceableBlocks",
            "getAttributeFee",
            "setAttributeFee",
            "setStoragePrice",
            "blockAccount",
            "unblockAccount",
            "isBlocked",
            "getBlockedAccounts",
            "recoverFund",
            "setWhitelistFeeContract",
            "removeWhitelistFeeContract",
            "getWhitelistFeeContracts",
            "requestOracleData",
            "getOraclePrice",
            "setOraclePrice",
            "oracleFinish",
            "oracleVerify",
            "oracleRequest",
            "designateAsRole",
            "getDesignatedByRole",
            "currentIndex",
            "currentHash",
            "getBlock",
            "getBlockHash",
            "getBlockByIndex",
            "getBlockByHash",
            "getTransaction",
            "getTransactionHeight",
            "getTransactionFromBlock",
            "getTransactionSigners",
            "getTransactionVMState",
            "getBlockSystemFee",
            "notaryVerify",
            "notaryBalanceOf",
            "notaryExpirationOf",
            "notaryLockDepositUntil",
            "notaryWithdraw",
            "notaryGetMaxNotValidBeforeDelta",
            "notarySetMaxNotValidBeforeDelta",
            "notaryOnNEP17Payment",
            "treasuryVerify",
            "treasuryOnNEP17Payment",
            "treasuryOnNEP11Payment",
            "isNativeContract",
            "getNativeContractName",
            "getAllNativeContracts",
            "estimateNativeCallGas",
            "batchNativeCalls",
            "getNetworkConfiguration",
            "getNativeContractManifest",
            "safeNativeCall",
            "externalNativeCall",
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
            "getBlockHeight",
            "getCurrentBlock",
            "getCurrentBlock",
            "getBlockByIndex",
            "getBlockTime",
            "getTransaction",
            "getTransactionHeight",
            "transactionExists",
            "getNetworkMagic",
            "verifyWithWitness",
            "sha256Hash",
            "ripemd160Hash",
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
        "getTrigger" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetTrigger".to_string(),
        )),
        "getInvocationCounter" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetInvocationCounter".to_string(),
        )),
        "getCurrentSigners" => Some(BuiltinCall::Syscall(
            "System.Runtime.CurrentSigners".to_string(),
        )),
        "getCallFlags" => Some(BuiltinCall::Syscall(
            "System.Contract.GetCallFlags".to_string(),
        )),
        "getScriptContainer" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetScriptContainer".to_string(),
        )),
        "loadScript" => Some(BuiltinCall::Syscall(
            "System.Runtime.LoadScript".to_string(),
        )),
        "getNetwork" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetNetwork".to_string(),
        )),
        "getPlatform" => Some(BuiltinCall::Syscall("System.Runtime.Platform".to_string())),
        "getAddressVersion" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetAddressVersion".to_string(),
        )),
        "getRandom" => Some(BuiltinCall::Syscall("System.Runtime.GetRandom".to_string())),
        "getExecutingScriptHash" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetExecutingScriptHash".to_string(),
        )),
        "getCallingScriptHash" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetCallingScriptHash".to_string(),
        )),
        "getEntryScriptHash" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetEntryScriptHash".to_string(),
        )),
        _ => None,
    }
}

fn resolve_abi_member(member: &str) -> Option<BuiltinCall> {
    match member {
        "encode" => Some(BuiltinCall::AbiEncode),
        "encodePacked" => Some(BuiltinCall::AbiEncodePacked),
        "encodeCall" => Some(BuiltinCall::AbiEncodeCall),
        "encodeWithSignature" => Some(BuiltinCall::AbiEncodeWithSignature),
        "encodeWithSelector" => Some(BuiltinCall::AbiEncodeWithSignature),
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
        "findLocal" => Some(BuiltinCall::Syscall(
            "System.Storage.Local.Find".to_string(),
        )),
        "putLocal" => Some(BuiltinCall::Syscall("System.Storage.Local.Put".to_string())),
        "getLocal" => Some(BuiltinCall::Syscall("System.Storage.Local.Get".to_string())),
        "removeLocal" => Some(BuiltinCall::Syscall(
            "System.Storage.Local.Delete".to_string(),
        )),
        "initializeContext" => Some(BuiltinCall::Syscall(
            "System.Storage.GetContext".to_string(),
        )),
        "getContext" => Some(BuiltinCall::Syscall(
            "System.Storage.GetContext".to_string(),
        )),
        "getReadOnlyContext" => Some(BuiltinCall::Syscall(
            "System.Storage.GetReadOnlyContext".to_string(),
        )),
        "asReadOnly" => Some(BuiltinCall::Syscall(
            "System.Storage.AsReadOnly".to_string(),
        )),
        "getUsage" => Some(BuiltinCall::Syscall("System.Storage.GetUsage".to_string())),
        "batchPut" => Some(BuiltinCall::Syscall("System.Storage.Put".to_string())),
        "batchGet" => Some(BuiltinCall::Syscall("System.Storage.Get".to_string())),
        "batchDelete" => Some(BuiltinCall::Syscall("System.Storage.Delete".to_string())),
        "count" => Some(BuiltinCall::Syscall("System.Storage.Find".to_string())),
        "countLocal" => Some(BuiltinCall::Syscall(
            "System.Storage.Local.Find".to_string(),
        )),
        "findValues" => Some(BuiltinCall::Syscall("System.Storage.Find".to_string())),
        "findLocalValues" => Some(BuiltinCall::Syscall(
            "System.Storage.Local.Find".to_string(),
        )),
        "findKeys" => Some(BuiltinCall::Syscall("System.Storage.Find".to_string())),
        "findLocalKeys" => Some(BuiltinCall::Syscall(
            "System.Storage.Local.Find".to_string(),
        )),
        "clearPrefix" => Some(BuiltinCall::Syscall("System.Storage.Delete".to_string())),
        "exists" => Some(BuiltinCall::Syscall("System.Storage.Get".to_string())),
        "isValidKey" => Some(BuiltinCall::Syscall("System.Storage.Get".to_string())),
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
        "getValidators" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "getNextBlockValidators".to_string(),
        }),
        "isCommittee" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "getCommittee".to_string(),
        }),
        "isValidator" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Neo,
            method: "getNextBlockValidators".to_string(),
        }),
        "getRandom" => Some(BuiltinCall::Syscall("System.Runtime.GetRandom".to_string())),
        "getBlockHeight" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Ledger,
            method: "currentIndex".to_string(),
        }),
        "getCurrentBlock" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Ledger,
            method: "currentIndex".to_string(),
        }),
        "transactionExists" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Ledger,
            method: "getTransaction".to_string(),
        }),
        "getNetworkMagic" => Some(BuiltinCall::Syscall(
            "System.Runtime.GetNetwork".to_string(),
        )),
        "getBlockByIndex" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Ledger,
            method: "getBlock".to_string(),
        }),
        "getBlockTime" => Some(BuiltinCall::Syscall("System.Runtime.GetTime".to_string())),
        "getTransaction" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Ledger,
            method: "getTransaction".to_string(),
        }),
        "getTransactionHeight" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::Ledger,
            method: "getTransactionHeight".to_string(),
        }),
        "verifyWithWitness" => Some(BuiltinCall::Syscall(
            "System.Runtime.CheckWitness".to_string(),
        )),
        "sha256Hash" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::CryptoLib,
            method: "sha256".to_string(),
        }),
        "ripemd160Hash" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::CryptoLib,
            method: "ripemd160".to_string(),
        }),
        _ => None,
    }
}

/// `StdLib.<method>(...)` — resolve bare member-access into a NativeCall to
/// the StdLib native contract (hash c0ef39cee0e4e925c6c2a06a79e1440dd86fceac).
/// Solidity source can write either `StdLib.itoa(v, 10)` or the devpack's
/// `Syscalls.itoa(v, 10)`; both must land at the same CALLT. See
/// `resolve_syscalls_member` for the sibling dispatch.
fn resolve_stdlib_member(member: &str) -> Option<BuiltinCall> {
    match member {
        "serialize" | "deserialize" | "jsonSerialize" | "jsonDeserialize" | "itoa" | "atoi"
        | "base64Encode" | "base64Decode" | "base64UrlEncode" | "base64UrlDecode"
        | "base58Encode" | "base58Decode" | "base58CheckEncode" | "base58CheckDecode"
        | "hexEncode" | "hexDecode" | "memoryCompare" | "memorySearch" | "stringSplit"
        | "strLen" => Some(BuiltinCall::NativeCall {
            contract: NativeContract::StdLib,
            method: member.to_string(),
        }),
        _ => None,
    }
}

/// `CryptoLib.<method>(...)` — resolve bare member-access into a NativeCall
/// to the CryptoLib native contract (hash 1bf575ab11896884136110a35a12886cde0b66c72).
/// Keeps parity with `Syscalls.sha256` / `Syscalls.ripemd160` / etc.
fn resolve_cryptolib_member(member: &str) -> Option<BuiltinCall> {
    match member {
        "sha256" | "ripemd160" | "verifyWithECDsa" | "murmur32" | "keccak256"
        | "recoverSecp256K1" | "verifyWithEd25519" | "bls12381Serialize"
        | "bls12381Deserialize" | "bls12381Equal" | "bls12381Add" | "bls12381Mul"
        | "bls12381Pairing" | "bls12381G1Add" | "bls12381G1Mul" | "bls12381G2Add"
        | "bls12381G2Mul" | "bls12381G1Neg" | "bls12381G2Neg" => {
            Some(BuiltinCall::NativeCall {
                contract: NativeContract::CryptoLib,
                method: member.to_string(),
            })
        }
        _ => None,
    }
}
