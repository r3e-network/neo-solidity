#[test]
fn manifest_methods_have_offsets_and_safe_flag() {
    let source = include_str!("../../../../examples/TestContract.sol");
    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    let manifest = &artifacts[0].manifest;

    let methods = manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    assert!(
        methods
            .iter()
            .all(|m| m.get("offset").and_then(Value::as_u64).is_some()),
        "all methods should have numeric offsets"
    );
    for method in methods {
        let name = method
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let safe = method.get("safe").and_then(Value::as_bool).unwrap_or(false);
        if name == "getValue" {
            assert!(safe, "view getter should be marked safe");
        }
    }
}

#[test]
fn manifest_storage_feature_set_for_storage_builtins() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract UsesStorageBuiltins {
        function put(bytes memory key, bytes memory value) public {
            Storage.put(key, value);
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let manifest = &artifacts[0].manifest;
    assert!(
        manifest
            .get("features")
            .and_then(Value::as_object)
            .is_some_and(|features| features.is_empty()),
        "expected `manifest.features` to be an empty object for Neo N3 compatibility"
    );
}

#[test]
fn manifest_events_do_not_include_indexed_fields() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract Events {
        event Transfer(address indexed from, address indexed to, uint256 amount);

        function emitIt(address to, uint256 amount) public {
            emit Transfer(msg.sender, to, amount);
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let manifest = &artifacts[0].manifest;
    let events = manifest["abi"]["events"].as_array().expect("events array");
    assert!(!events.is_empty(), "expected at least one event");

    for event in events {
        let params = event["parameters"]
            .as_array()
            .expect("event parameters array");
        for param in params {
            assert!(
                param.get("indexed").is_none(),
                "Neo N3 manifest event parameters must not include an 'indexed' field"
            );
        }
    }
}
