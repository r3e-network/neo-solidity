fn build_enum_variant_map(enums: &[EnumMetadata]) -> HashMap<String, HashMap<String, u64>> {
    let mut map = HashMap::new();

    for enum_meta in enums {
        let mut variants = HashMap::new();
        for (index, variant) in enum_meta.values.iter().enumerate() {
            variants.insert(variant.clone(), index as u64);
        }
        map.insert(enum_meta.name.clone(), variants);
    }

    map
}

fn manifest_type_from_solidity_type(solidity_type: &str) -> ManifestType {
    let ty = solidity_type.trim().to_ascii_lowercase();

    if ty.ends_with("[]") {
        return ManifestType::Array;
    }

    if ty.starts_with("mapping") {
        return ManifestType::Map;
    }

    if ty.starts_with("uint") || ty.starts_with("int") {
        return ManifestType::Integer;
    }

    if ty == "bool" || ty == "boolean" {
        return ManifestType::Boolean;
    }

    if ty == "string" {
        return ManifestType::String;
    }

    if ty == "address" || ty == "address payable" || ty == "bytes20" || ty == "hash160" {
        return ManifestType::Hash160;
    }

    if ty == "bytes32" || ty == "hash256" {
        return ManifestType::Hash256;
    }

    if ty == "bytes" {
        return ManifestType::ByteArray;
    }

    if ty.starts_with("bytes") {
        if let Some(size_str) = ty.strip_prefix("bytes") {
            if size_str.parse::<u8>().is_ok() {
                return if size_str == "32" {
                    ManifestType::Hash256
                } else {
                    ManifestType::ByteArray
                };
            }
        }
        return ManifestType::ByteArray;
    }

    if ty == "any" {
        return ManifestType::Any;
    }

    ManifestType::Any
}

fn call_flags_allow_write_or_notify(flags: u8) -> bool {
    // Neo N3 CallFlags:
    // - ReadStates   = 0x01
    // - WriteStates  = 0x02
    // - AllowCall    = 0x04
    // - AllowNotify  = 0x08
    // Safe (view/pure) code must not grant WriteStates or AllowNotify.
    flags & (0x02 | 0x08) != 0
}

fn parse_u8_literal(value: &BigInt) -> Option<u8> {
    value.to_u8()
}

fn native_call_is_mutating(contract: NativeContract, method: &str) -> bool {
    match contract {
        NativeContract::Neo => matches!(
            method,
            "transfer"
                | "vote"
                | "registerCandidate"
                | "unregisterCandidate"
                | "setGasPerBlock"
                | "setRegisterPrice"
        ),
        NativeContract::Gas => matches!(method, "transfer"),
        NativeContract::ContractManagement => {
            matches!(method, "deploy" | "update" | "destroy" | "setMinimumDeploymentFee")
        }
        NativeContract::Policy => matches!(
            method,
            "setFeePerByte"
                | "setExecFeeFactor"
                | "setStoragePrice"
                | "setMillisecondsPerBlock"
                | "setMaxValidUntilBlockIncrement"
                | "setMaxTraceableBlocks"
                | "setAttributeFee"
                | "blockAccount"
                | "unblockAccount"
                | "recoverFund"
                | "setWhitelistFeeContract"
                | "removeWhitelistFeeContract"
        ),
        NativeContract::Oracle => matches!(method, "request" | "setPrice" | "finish"),
        NativeContract::RoleManagement => matches!(method, "designateAsRole"),
        NativeContract::Notary => {
            matches!(
                method,
                "lockDepositUntil" | "withdraw" | "setMaxNotValidBeforeDelta" | "onNEP17Payment"
            )
        }
        NativeContract::Treasury => false,
        NativeContract::Ledger | NativeContract::CryptoLib | NativeContract::StdLib => false,
    }
}
