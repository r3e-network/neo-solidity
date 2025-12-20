#[test]
fn standard_json_reports_unsupported_settings_warning() {
    let temp = tempdir().expect("tempdir");
    let input_path = temp.path().join("input.json");
    let output_path = temp.path().join("out.json");

    let input_json = json!({
        "language": "Solidity",
        "sources": {
            "C.sol": { "content": "contract C { function f() public {} }" }
        },
        "settings": { "metadata": { "bytecodeHash": "ipfs" } }
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
    .expect("standard-json processing should succeed");

    let output: Value =
        serde_json::from_str(&fs::read_to_string(&output_path).expect("read output"))
            .expect("output json");

    let errors = output["errors"].as_array().unwrap_or(&Vec::new()).clone();
    assert!(
        errors
            .iter()
            .any(|err| err["type"] == "UnsupportedSettings"),
        "expected UnsupportedSettings warning when unsupported settings are present"
    );
}

#[test]
fn standard_json_warns_when_nef_source_truncated() {
    let temp = tempdir().expect("tempdir");
    let input_path = temp.path().join("input.json");
    let output_path = temp.path().join("out.json");

    let long_source = "s".repeat(NEF_SOURCE_MAX_BYTES + 10);

    let source = r#"
    pragma solidity ^0.8.19;

    contract C {
        function f() public pure returns (uint256) { return 1; }
    }
    "#;

    let input_json = json!({
        "language": "Solidity",
        "sources": {
            "C.sol": { "content": source }
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
            nef_source: Some(&long_source),
            manifest_permissions: None,
            contract_names: Vec::new(),
        },
    )
    .expect("standard-json processing should succeed");

    let output: Value =
        serde_json::from_str(&fs::read_to_string(&output_path).expect("read output"))
            .expect("output json");

    let errors = output["errors"].as_array().unwrap_or(&Vec::new()).clone();
    assert!(
        errors.iter().any(|err| {
            err["type"] == "NefSourceTruncated"
                && err["severity"] == "warning"
                && err["code"] == "NEF_SOURCE_TRUNCATED"
        }),
        "expected NefSourceTruncated warning"
    );

    let source_value = output["contracts"]["C.sol"]["C"]["neo"]["nef"]["source"]
        .as_str()
        .expect("nef source string");
    assert_eq!(source_value.len(), NEF_SOURCE_MAX_BYTES);
}

#[test]
fn standard_json_warns_on_wildcard_contract_permissions() {
    let temp = tempdir().expect("tempdir");
    let input_path = temp.path().join("input.json");
    let output_path = temp.path().join("out.json");

    let source = r#"
    pragma solidity ^0.8.19;

    interface IFace {
        function ping(uint256 value) external returns (uint256);
    }

    contract C {
        function callIt(address target) public returns (uint256) {
            return IFace(target).ping(1);
        }
    }
    "#;

    let input_json = json!({
        "language": "Solidity",
        "sources": {
            "C.sol": { "content": source }
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
    .expect("standard-json processing should succeed");

    let output: Value =
        serde_json::from_str(&fs::read_to_string(&output_path).expect("read output"))
            .expect("output json");

    let errors = output["errors"].as_array().unwrap_or(&Vec::new()).clone();
    assert!(
        errors.iter().any(|err| {
            err["severity"] == "warning" && err["code"] == "MANIFEST_WILDCARD_CONTRACT"
        }),
        "expected MANIFEST_WILDCARD_CONTRACT warning"
    );
}

#[test]
fn standard_json_warns_on_wildcard_method_permissions() {
    let temp = tempdir().expect("tempdir");
    let input_path = temp.path().join("input.json");
    let output_path = temp.path().join("out.json");

    let source = r#"
    pragma solidity ^0.8.19;

    contract C {
        function callGas(string memory method) public returns (bytes memory) {
            return Syscalls.contractCall(NativeCalls.GAS_CONTRACT, method, abi.encode());
        }
    }
    "#;

    let input_json = json!({
        "language": "Solidity",
        "sources": {
            "C.sol": { "content": source }
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
    .expect("standard-json processing should succeed");

    let output: Value =
        serde_json::from_str(&fs::read_to_string(&output_path).expect("read output"))
            .expect("output json");

    let errors = output["errors"].as_array().unwrap_or(&Vec::new()).clone();
    assert!(
        errors.iter().any(|err| {
            err["severity"] == "warning" && err["code"] == "MANIFEST_WILDCARD_METHODS"
        }),
        "expected MANIFEST_WILDCARD_METHODS warning"
    );
}
