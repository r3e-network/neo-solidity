#[test]
fn standard_json_reports_missing_content_error() {
    let temp = tempdir().expect("tempdir");
    let input_path = temp.path().join("input.json");
    let output_path = temp.path().join("out.json");

    let input_json = json!({
        "language": "Solidity",
        "sources": {
            "Missing.sol": { "urls": ["ipfs://example"] }
        },
        "settings": {}
    });
    fs::write(
        &input_path,
        serde_json::to_string_pretty(&input_json).unwrap(),
    )
    .expect("write input");

    process_standard_json(
        input_path.to_str().unwrap(),
        Some(output_path.to_str().unwrap()),
        StandardJsonOptions {
            optimizer_level: 2,
            use_callt: false,
            deny_wildcard_permissions: false,
            deny_wildcard_contracts: false,
            deny_wildcard_methods: false,
            nef_source: None,
            manifest_permissions: None,
            contract_names: Vec::new(),
        },
    )
    .expect("standard-json processing should surface errors but not fail");

    let output: Value =
        serde_json::from_str(&fs::read_to_string(&output_path).expect("read output"))
            .expect("output json");

    let errors = output["errors"].as_array().expect("errors array expected");
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0]["type"],
        Value::String("MissingSourceContent".into())
    );
    assert_eq!(errors[0]["severity"], Value::String("error".into()));
}

#[test]
fn standard_json_rejects_non_solidity_language() {
    let temp = tempdir().expect("tempdir");
    let input_path = temp.path().join("input.json");

    let input_json = json!({
        "language": "Vyper",
        "sources": {
            "C.sol": { "content": "contract C {}" }
        },
        "settings": {}
    });
    fs::write(
        &input_path,
        serde_json::to_string_pretty(&input_json).unwrap(),
    )
    .expect("write input");

    let result = process_standard_json(
        input_path.to_str().unwrap(),
        None,
        StandardJsonOptions {
            optimizer_level: 2,
            use_callt: false,
            deny_wildcard_permissions: false,
            deny_wildcard_contracts: false,
            deny_wildcard_methods: false,
            nef_source: None,
            manifest_permissions: None,
            contract_names: Vec::new(),
        },
    );
    assert!(
        result
            .err()
            .map(|msg| msg.to_ascii_lowercase().contains("unsupported language"))
            .unwrap_or(false),
        "expected unsupported language error"
    );
}

#[test]
fn standard_json_reports_no_contracts() {
    let temp = tempdir().expect("tempdir");
    let input_path = temp.path().join("input.json");
    let output_path = temp.path().join("out.json");

    let input_json = json!({
        "language": "Solidity",
        "sources": {
            "Empty.sol": { "content": "" }
        },
        "settings": {}
    });
    fs::write(
        &input_path,
        serde_json::to_string_pretty(&input_json).unwrap(),
    )
    .expect("write input");

    process_standard_json(
        input_path.to_str().unwrap(),
        Some(output_path.to_str().unwrap()),
        StandardJsonOptions {
            optimizer_level: 2,
            use_callt: false,
            deny_wildcard_permissions: false,
            deny_wildcard_contracts: false,
            deny_wildcard_methods: false,
            nef_source: None,
            manifest_permissions: None,
            contract_names: Vec::new(),
        },
    )
    .expect("standard-json processing should surface errors but not fail");

    let output: Value =
        serde_json::from_str(&fs::read_to_string(&output_path).expect("read output"))
            .expect("output json");

    let errors = output["errors"].as_array().expect("errors array expected");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], Value::String("NoContracts".into()));
    assert_eq!(errors[0]["severity"], Value::String("error".into()));
}

#[test]
fn standard_json_reports_missing_import_error() {
    let temp = tempdir().expect("tempdir");
    let input_path = temp.path().join("input.json");
    let output_path = temp.path().join("out.json");

    let source = r#"
    pragma solidity ^0.8.19;
    import "./Missing.sol";

    contract A {
        function ok() public pure returns (uint256) {
            return 1;
        }
    }
    "#;

    let input_json = json!({
        "language": "Solidity",
        "sources": {
            "A.sol": { "content": source }
        },
        "settings": {}
    });
    fs::write(
        &input_path,
        serde_json::to_string_pretty(&input_json).unwrap(),
    )
    .expect("write input");

    process_standard_json(
        input_path.to_str().unwrap(),
        Some(output_path.to_str().unwrap()),
        StandardJsonOptions {
            optimizer_level: 2,
            use_callt: false,
            deny_wildcard_permissions: false,
            deny_wildcard_contracts: false,
            deny_wildcard_methods: false,
            nef_source: None,
            manifest_permissions: None,
            contract_names: Vec::new(),
        },
    )
    .expect("standard-json processing should surface errors but not fail");

    let output: Value =
        serde_json::from_str(&fs::read_to_string(&output_path).expect("read output"))
            .expect("output json");

    let errors = output["errors"].as_array().expect("errors array expected");
    assert!(
        errors
            .iter()
            .any(|e| e.get("type").and_then(Value::as_str) == Some("MissingImport")),
        "expected MissingImport error"
    );
}
