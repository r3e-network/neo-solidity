/// Normalize a Solidity type string into its EVM-canonical signature form.
///
/// Canonicalization is the minimum set of rewrites Solidity itself applies
/// when computing `keccak256(signature)` for `event` topic-0 and `function`
/// selectors:
///   * `uint`  -> `uint256`
///   * `int`   -> `int256`
///   * `byte`  -> `bytes1`
///   * enum types (`enum Foo` or a bare name matching a declared enum) ->
///     `uint8` per the Solidity ABI spec.
///   * unknown user-defined types (structs, contracts) pass through by name.
fn event_canonical_param_type(raw: &str, enum_names: &HashSet<String>) -> String {
    let trimmed = raw.trim();
    let without_location = trimmed
        .replace(" memory", "")
        .replace(" calldata", "")
        .replace(" storage", "");

    let tokens: Vec<&str> = without_location.split_whitespace().collect();
    if tokens.first().copied() == Some("enum") {
        if let Some(name_with_suffix) = tokens.get(1) {
            let suffix = match name_with_suffix.find('[') {
                Some(idx) => &name_with_suffix[idx..],
                None => "",
            };
            return format!("uint8{suffix}");
        }
        return "uint8".to_string();
    }

    let compact: String = without_location
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();
    let (base, suffix) = match compact.find('[') {
        Some(idx) => (&compact[..idx], &compact[idx..]),
        None => (compact.as_str(), ""),
    };

    let canonical_base: &str = match base {
        "uint" => "uint256",
        "int" => "int256",
        "byte" => "bytes1",
        other => {
            if enum_names.contains(other) {
                "uint8"
            } else {
                other
            }
        }
    };

    format!("{canonical_base}{suffix}")
}

/// Returns whether a canonical Solidity type is dynamic for indexed-event
/// topic hashing.
fn event_canonical_type_is_dynamic(canonical: &str) -> bool {
    if canonical == "string" || canonical == "bytes" {
        return true;
    }
    if canonical.contains('[') {
        return true;
    }
    matches!(canonical, "tuple") || is_user_defined_canonical_type(canonical)
}

fn is_user_defined_canonical_type(canonical: &str) -> bool {
    if canonical == "bool" || canonical == "string" || canonical == "bytes" {
        return false;
    }
    if canonical == "address" || canonical == "function" {
        return false;
    }
    for prefix in ["uint", "int", "bytes"] {
        if let Some(rest) = canonical.strip_prefix(prefix) {
            if rest.is_empty() || rest.chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
        }
    }
    canonical
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_uppercase())
}

fn event_canonical_signature(event: &EventMetadata, enum_names: &HashSet<String>) -> String {
    let joined_types: Vec<String> = event
        .parameters
        .iter()
        .map(|p| event_canonical_param_type(&p.ty, enum_names))
        .collect();
    format!("{}({})", event.name, joined_types.join(","))
}

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
