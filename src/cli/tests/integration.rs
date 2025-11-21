use super::*;
#[test]
fn example_contract_compiles_and_manifest_is_populated() {
    let source = include_str!("../../../examples/TestContract.sol");
    let artifacts = compile_contracts(source, false).expect("compilation failed");
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
