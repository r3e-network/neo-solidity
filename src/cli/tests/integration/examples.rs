#[test]
fn example_contract_compiles_and_manifest_is_populated() {
    let source = include_str!("../../../../examples/TestContract.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let artifact = &artifacts[0];
    assert_eq!(artifact.metadata.name, "TestContract");

    let methods = artifact.manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    assert!(
        methods.len() >= 2,
        "expected at least setter/getter methods in manifest"
    );

    let get_value = methods
        .iter()
        .find(|m| m.get("name").and_then(Value::as_str) == Some("getValue"))
        .expect("getValue method present");
    assert!(
        get_value
            .get("safe")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        "view function should be marked safe"
    );
}

#[test]
fn counter_example_compiles_and_has_events() {
    let source = include_str!("../../../../examples/new/Counter.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let artifact = &artifacts[0];
    assert_eq!(artifact.metadata.name, "Counter");

    let manifest = &artifact.manifest;
    let methods = manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    assert!(
        methods
            .iter()
            .any(|m| m.get("name").and_then(Value::as_str) == Some("get")),
        "getter should be present in manifest"
    );

    let events = manifest["abi"]["events"].as_array().expect("events array");
    let event_names: Vec<_> = events
        .iter()
        .filter_map(|e| e.get("name").and_then(Value::as_str))
        .collect();
    assert!(
        event_names.contains(&"Incremented") && event_names.contains(&"Decremented"),
        "expected Incremented and Decremented events"
    );
}

#[test]
fn nft_example_advertises_nep11() {
    let source = include_str!("../../../../examples/new/NFT.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);
    let artifact = &artifacts[0];
    assert_eq!(artifact.metadata.name, "SimpleNFT");

    let manifest = &artifact.manifest;
    let standards = manifest["supportedstandards"]
        .as_array()
        .expect("supportedstandards array");
    assert!(
        standards.iter().any(|s| s.as_str() == Some("NEP-11")),
        "SimpleNFT should advertise NEP-11"
    );

    let methods = manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    for required in ["ownerOf", "transferFrom"] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected method '{}' in manifest",
            required
        );
    }

    // Public state variables should be exposed as Solidity-style getters.
    for required in ["name", "symbol"] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected public getter '{}' in manifest",
            required
        );
    }
}

#[test]
fn vault_example_compiles_with_external_calls() {
    let source = include_str!("../../../../examples/new/Vault.sol");
    let artifacts = compile_contracts(source, false, 2).expect("vault compilation failed");
    assert_eq!(artifacts.len(), 1);

    let methods = artifacts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    for required in ["deposit", "withdraw"] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected method '{}' in manifest",
            required
        );
    }
}

#[test]
fn neo_interop_showcase_compiles_with_expected_methods() {
    let source = include_str!("../../../../examples/new/NeoInteropShowcase.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let methods = artifacts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");

    for required in [
        "credit",
        "creditOf",
        "transferGasFromSelf",
        "gasBalanceViaSyscall",
    ] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected method '{}' in manifest",
            required
        );
    }

    let permissions = artifacts[0].manifest["permissions"]
        .as_array()
        .expect("permissions array");
    assert!(
        permissions
            .iter()
            .all(|entry| entry["contract"].as_str() != Some("*")),
        "neo interop showcase should avoid wildcard contract permissions"
    );
}

#[test]
fn low_level_call_showcase_compiles_with_manifest_methods() {
    let source = include_str!("../../../../examples/new/LowLevelCallShowcase.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let methods = artifacts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    for required in ["readViaSignature", "readViaSelector", "writeViaSignature"] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected method '{}' in manifest",
            required
        );
    }
}

#[test]
fn enum_array_showcase_compiles_and_returns_array() {
    let source = include_str!("../../../../examples/new/EnumArrayShowcase.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let methods = artifacts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    let method = methods
        .iter()
        .find(|m| m.get("name").and_then(Value::as_str) == Some("statesForReview"))
        .expect("statesForReview method");

    assert_eq!(
        method.get("returntype").and_then(Value::as_str),
        Some("Array"),
        "enum array return should map to Neo ABI Array"
    );
}

#[test]
fn nep17_is_detected_when_core_methods_are_public_getters() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract AutoGetterToken {
        string public symbol = "AUTO";
        uint8 public decimals = 8;
        uint256 public totalSupply = 100;
        mapping(address => uint256) public balanceOf;

        constructor() {
            balanceOf[msg.sender] = totalSupply;
        }

        function transfer(address to, uint256 amount) public returns (bool) {
            require(to != address(0), "bad to");
            require(balanceOf[msg.sender] >= amount, "insufficient");
            balanceOf[msg.sender] -= amount;
            balanceOf[to] += amount;
            return true;
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);
    let manifest = &artifacts[0].manifest;
    let standards = manifest["supportedstandards"]
        .as_array()
        .expect("supportedstandards array");
    assert!(
        standards.iter().any(|s| s.as_str() == Some("NEP-17")),
        "AutoGetterToken should advertise NEP-17 when getters are present"
    );
}

#[test]
fn custom_errors_showcase_compiles_with_expected_methods() {
    let source = include_str!("../../../../examples/new/CustomErrorsShowcase.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let methods = artifacts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    for required in ["deposit", "withdraw", "adminReset"] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected method '{}' in manifest",
            required
        );
    }
}

#[test]
fn modifier_showcase_compiles_with_expected_methods() {
    let source = include_str!("../../../../examples/new/ModifierShowcase.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let methods = artifacts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    for required in ["pause", "unpause", "setValue", "getCallCount"] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected method '{}' in ModifierShowcase manifest",
            required
        );
    }
}

#[test]
fn struct_mapping_showcase_compiles_with_expected_methods() {
    let source = include_str!("../../../../examples/new/StructMappingShowcase.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let methods = artifacts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    for required in ["createProfile", "getProfile", "createOrg", "getOrg"] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected method '{}' in StructMappingShowcase manifest",
            required
        );
    }
}

#[test]
fn type_casting_showcase_compiles_with_expected_methods() {
    let source = include_str!("../../../../examples/new/TypeCastingShowcase.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let methods = artifacts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    for required in ["widenUint", "narrowUint", "addressToUint", "signedCast"] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected method '{}' in TypeCastingShowcase manifest",
            required
        );
    }
}

#[test]
fn bitwise_showcase_compiles_with_expected_methods() {
    let source = include_str!("../../../../examples/new/BitwiseShowcase.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let methods = artifacts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    for required in ["grantFlag", "hasFlag", "pack", "unpack"] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected method '{}' in BitwiseShowcase manifest",
            required
        );
    }
}

#[test]
fn constants_immutable_showcase_compiles_with_expected_methods() {
    let source = include_str!("../../../../examples/new/ConstantsImmutableShowcase.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let methods = artifacts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    for required in ["mint", "getInfo"] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected method '{}' in ConstantsImmutableShowcase manifest",
            required
        );
    }
}

#[test]
fn event_indexed_showcase_compiles_with_events() {
    let source = include_str!("../../../../examples/new/EventIndexedShowcase.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let manifest = &artifacts[0].manifest;
    let events = manifest["abi"]["events"].as_array().expect("events array");
    let event_names: Vec<_> = events
        .iter()
        .filter_map(|e| e.get("name").and_then(Value::as_str))
        .collect();
    assert!(
        event_names.contains(&"Transfer"),
        "expected Transfer event in EventIndexedShowcase"
    );
    assert!(
        event_names.contains(&"Approval"),
        "expected Approval event in EventIndexedShowcase"
    );
}

#[test]
fn try_catch_showcase_compiles_with_expected_methods() {
    let source = include_str!("../../../../examples/new/TryCatchShowcase.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let methods = artifacts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    for required in ["tryCatchSuccess", "tryCatchRevert", "tryCatchPanic"] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected method '{}' in TryCatchShowcase manifest",
            required
        );
    }
}

#[test]
fn interface_showcase_compiles_with_expected_methods() {
    let source = include_str!("../../../../examples/new/InterfaceShowcase.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert!(!artifacts.is_empty(), "should produce at least one artifact");

    let token = artifacts
        .iter()
        .find(|a| a.metadata.name == "SimpleToken")
        .expect("SimpleToken artifact");
    let methods = token.manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    for required in ["name", "symbol", "totalSupply", "balanceOf", "transfer"] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected method '{}' in SimpleToken manifest",
            required
        );
    }
}
