#[test]
fn example_contract_compiles_and_manifest_is_populated() {
    let source = include_str!("../../../examples/TestContract.sol");
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
    let source = include_str!("../../../examples/new/Counter.sol");
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
    let source = include_str!("../../../examples/new/NFT.sol");
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
    let source = include_str!("../../../examples/new/Vault.sol");
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
