use super::*;

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
}

#[test]
fn vault_example_compiles_and_exposes_balances() {
    let source = include_str!("../../../examples/new/Vault.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);
    let artifact = &artifacts[0];
    assert_eq!(artifact.metadata.name, "Vault");

    let manifest = &artifact.manifest;
    let methods = manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");

    for expected in ["deposit", "withdraw"] {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(expected)),
            "expected method '{}' in manifest",
            expected
        );
    }

    assert!(
        manifest["features"]["storage"].as_bool().unwrap_or(false),
        "vault should be marked as using storage"
    );
}
