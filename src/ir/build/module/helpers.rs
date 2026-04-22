/// Normalize a Solidity type string into its EVM-canonical signature form.
///
/// Canonicalization is the minimum set of rewrites Solidity itself applies
/// when computing `keccak256(signature)` for `event` topic-0 and `function`
/// selectors:
///   * `uint`  → `uint256`
///   * `int`   → `int256`
///   * `byte`  → `bytes1`
///   * enum types (`enum Foo` or a bare name matching a declared enum) →
///     `uint8` per the Solidity ABI spec (all enums have ≤256 variants, so
///     they are canonically encoded as `uint8`).
///   * unknown user-defined types (structs, contracts) pass through by name
///     — proper tuple expansion (`(uint256,uint256)` for a struct
///     `{uint256 x; uint256 y;}`) is a follow-up; today's harnesses pin the
///     struct-name form (see fuzz `event_with_struct_arg_payload_shape`).
///
/// Array suffixes (`[]`, `[N]`) and whitespace are preserved verbatim after
/// trimming — Solidity's canonical form omits all whitespace around commas.
///
/// `enum_names` is the set of declared enum type names in the current
/// contract scope. Bare references like `State` (without the `enum` prefix
/// that `solang-parser`'s Display impl may or may not produce) are matched
/// against this set so they canonicalize to `uint8`. Task #204.
fn event_canonical_param_type(raw: &str, enum_names: &HashSet<String>) -> String {
    let trimmed = raw.trim();

    // Strip Solidity data-location keywords (memory/calldata/storage) that may
    // appear in parameter declarations — they never participate in the
    // canonical signature.
    let without_location = trimmed
        .replace(" memory", "")
        .replace(" calldata", "")
        .replace(" storage", "");

    // Handle the `enum Name` prefix form before we strip whitespace — the
    // leading `enum` keyword is only meaningful when separated by a space.
    let tokens: Vec<&str> = without_location.split_whitespace().collect();
    if tokens.first().copied() == Some("enum") {
        // `enum Name` or `enum Name[]` → canonicalize base to `uint8`, keep
        // any array suffix attached to the `Name` token.
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

    // Separate any array/brackets suffix from the base type so we can
    // canonicalize the base alone.
    let (base, suffix) = match compact.find('[') {
        Some(idx) => (&compact[..idx], &compact[idx..]),
        None => (compact.as_str(), ""),
    };

    let canonical_base: &str = match base {
        "uint" => "uint256",
        "int" => "int256",
        "byte" => "bytes1",
        other => {
            // Task #204 — bare-name enum reference (e.g. `State` when the
            // source declared `enum State { ... }`). `solang-parser`'s
            // Display impl emits the bare name in this path, so without
            // consulting `enum_names` we would pass `State` through
            // verbatim and produce `Transition(State,State)` — which
            // disagrees with Solidity's canonical form.
            if enum_names.contains(other) {
                "uint8"
            } else {
                other
            }
        }
    };

    format!("{canonical_base}{suffix}")
}

/// Returns whether a canonical Solidity type is a "dynamic" type for the
/// purposes of indexed-event-argument topic hashing.
///
/// Per the EVM ABI spec (https://docs.soliditylang.org/en/latest/abi-spec.html
/// section "Encoding of Indexed Event Parameters"):
///   * `string`, `bytes`, and any array type (`T[]`, `T[N]`, struct/tuple)
///     are hashed: `topic = keccak256(value)`.
///   * All other value types (integer, bool, address, bytesN, enum) are
///     left-padded to 32 bytes and emitted verbatim.
fn event_canonical_type_is_dynamic(canonical: &str) -> bool {
    if canonical == "string" || canonical == "bytes" {
        return true;
    }
    // Any array (dynamic `T[]` or fixed `T[N]`) and user-defined struct are
    // considered dynamic for indexed-topic hashing.
    if canonical.contains('[') {
        return true;
    }
    // Heuristic: a canonical name that is neither a built-in scalar type nor
    // a fixed `bytesN` / `uintN` / `intN` is a user-defined type (struct,
    // enum, contract). Structs require keccak-hash for indexed topics.
    matches!(canonical, "tuple") || is_user_defined_canonical_type(canonical)
}

/// Heuristic check: does `canonical` name a user-defined (non-built-in) type?
fn is_user_defined_canonical_type(canonical: &str) -> bool {
    if canonical == "bool" || canonical == "string" || canonical == "bytes" {
        return false;
    }
    if canonical == "address" || canonical == "function" {
        return false;
    }
    // uintN / intN / bytesN.
    for prefix in ["uint", "int", "bytes"] {
        if let Some(rest) = canonical.strip_prefix(prefix) {
            if rest.is_empty() || rest.chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
        }
    }
    // Capitalized names are almost always user-defined types (structs, enums,
    // contracts). Lowercase unknowns we treat as non-dynamic to stay safe.
    canonical
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_uppercase())
}

/// Build the EVM-canonical event signature string:
/// `Name(type1,type2,...)` with the types canonicalized per
/// `event_canonical_param_type`. This is the exact input to
/// `keccak256(...)` for `topic[0]`.
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
