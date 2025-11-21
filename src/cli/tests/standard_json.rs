use super::*;
use sha3::{Digest, Keccak256};
use std::fs;
use tempfile::tempdir;
use neo_solidity::solidity::StateMutability;

pub fn parse_manifest(path: &str) -> Value {
    let content = std::fs::read_to_string(path).expect("manifest should be readable");
    serde_json::from_str(&content).expect("manifest should parse as JSON")
}

#[test]
fn standard_json_includes_keccak_and_identifiers() {
    let temp = tempdir().expect("tempdir");
    let input_path = temp.path().join("input.json");
    let output_path = temp.path().join("out.json");

    let source = r#"
    pragma solidity ^0.8.19;

    contract C {
        function foo(uint256 a) public view returns (uint256) {
            return a;
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
    )
    .expect("standard-json processing should succeed");

    let output: Value =
        serde_json::from_str(&fs::read_to_string(&output_path).expect("read output"))
            .expect("output json");

    let expected_keccak = hex_prefixed(&keccak256_hex(source));
    assert_eq!(
        output["sources"]["C.sol"]["keccak256"],
        Value::String(expected_keccak)
    );

    let method_id = output["contracts"]["C.sol"]["C"]["evm"]["methodIdentifiers"]["foo(uint256)"]
        .as_str()
        .expect("method identifier string");
    let selector = {
        let mut hasher = Keccak256::new();
        hasher.update("foo(uint256)".as_bytes());
        let digest = hasher.finalize();
        hex_prefixed(&hex::encode(&digest[..4]))
    };
    assert_eq!(method_id, selector);

    assert!(output["contracts"]["C.sol"]["C"]["metadata"]
        .as_str()
        .map(|s| !s.is_empty())
        .unwrap_or(false));
}

#[test]
fn standard_json_handles_multiple_sources() {
    let temp = tempdir().expect("tempdir");
    let input_path = temp.path().join("input.json");
    let output_path = temp.path().join("out.json");

    let source_a = r#"
    pragma solidity ^0.8.19;

    contract A {
        function foo(uint256 a) public pure returns (uint256) {
            return a + 1;
        }
    }
    "#;

    let source_b = r#"
    pragma solidity ^0.8.19;

    contract B {
        function bar() public pure returns (uint256) {
            return 42;
        }
    }
    "#;

    let input_json = json!({
        "language": "Solidity",
        "sources": {
            "A.sol": { "content": source_a },
            "B.sol": { "content": source_b }
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
    )
    .expect("standard-json processing should succeed");

    let output: Value =
        serde_json::from_str(&fs::read_to_string(&output_path).expect("read output"))
            .expect("output json");

    let expected_keccak_a = hex_prefixed(&keccak256_hex(source_a));
    let expected_keccak_b = hex_prefixed(&keccak256_hex(source_b));

    assert_eq!(
        output["sources"]["A.sol"]["keccak256"],
        Value::String(expected_keccak_a)
    );
    assert_eq!(
        output["sources"]["B.sol"]["keccak256"],
        Value::String(expected_keccak_b)
    );

    let contracts = &output["contracts"];
    let a_contract = &contracts["A.sol"]["A"];
    let b_contract = &contracts["B.sol"]["B"];
    let a_method = a_contract["evm"]["methodIdentifiers"]["foo(uint256)"]
        .as_str()
        .expect("A.foo method id");
    let b_method = b_contract["evm"]["methodIdentifiers"]["bar()"]
        .as_str()
        .expect("B.bar method id");

    let selector_a = {
        let mut hasher = Keccak256::new();
        hasher.update("foo(uint256)".as_bytes());
        let digest = hasher.finalize();
        hex_prefixed(&hex::encode(&digest[..4]))
    };
    let selector_b = {
        let mut hasher = Keccak256::new();
        hasher.update("bar()".as_bytes());
        let digest = hasher.finalize();
        hex_prefixed(&hex::encode(&digest[..4]))
    };

    assert_eq!(a_method, selector_a);
    assert_eq!(b_method, selector_b);

    assert!(output
        .get("errors")
        .map(|v| v.as_array().map(|arr| arr.is_empty()).unwrap_or(true))
        .unwrap_or(true));
}

#[test]
fn standard_json_handles_multiple_contracts_per_source() {
    let temp = tempdir().expect("tempdir");
    let input_path = temp.path().join("input.json");
    let output_path = temp.path().join("out.json");

    let source = r#"
    pragma solidity ^0.8.19;

    contract A {
        function foo() public pure returns (uint256) {
            return 1;
        }
    }

    contract B {
        function bar(uint256 n) public pure returns (uint256) {
            return n * 2;
        }
    }
    "#;

    let input_json = json!({
        "language": "Solidity",
        "sources": {
            "Both.sol": { "content": source }
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
    )
    .expect("standard-json processing should succeed");

    let output: Value =
        serde_json::from_str(&fs::read_to_string(&output_path).expect("read output"))
            .expect("output json");

    let expected_keccak = hex_prefixed(&keccak256_hex(source));
    assert_eq!(
        output["sources"]["Both.sol"]["keccak256"],
        Value::String(expected_keccak)
    );

    let contracts = &output["contracts"]["Both.sol"];
    let a_method = contracts["A"]["evm"]["methodIdentifiers"]["foo()"]
        .as_str()
        .expect("A foo method id");
    let b_method = contracts["B"]["evm"]["methodIdentifiers"]["bar(uint256)"]
        .as_str()
        .expect("B bar method id");

    let selector_a = {
        let mut hasher = Keccak256::new();
        hasher.update("foo()".as_bytes());
        let digest = hasher.finalize();
        hex_prefixed(&hex::encode(&digest[..4]))
    };
    let selector_b = {
        let mut hasher = Keccak256::new();
        hasher.update("bar(uint256)".as_bytes());
        let digest = hasher.finalize();
        hex_prefixed(&hex::encode(&digest[..4]))
    };

    assert_eq!(a_method, selector_a);
    assert_eq!(b_method, selector_b);

    assert!(output
        .get("errors")
        .map(|v| v.as_array().map(|arr| arr.is_empty()).unwrap_or(true))
        .unwrap_or(true));
}

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

    let result = process_standard_json(input_path.to_str().unwrap(), None);
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
fn sanitize_contract_name_handles_invalid_chars() {
    assert_eq!(
        sanitize_contract_name("My Contract!").as_deref(),
        Some("My_Contract")
    );
    assert_eq!(
        sanitize_contract_name("___invalid***").as_deref(),
        Some("invalid")
    );
    assert_eq!(
        sanitize_contract_name("token-1").as_deref(),
        Some("token-1")
    );
}

#[test]
fn solidity_types_map_to_manifest_types() {
    assert_eq!(solidity_to_manifest_type("uint256"), "Integer");
    assert_eq!(solidity_to_manifest_type("int8"), "Integer");
    assert_eq!(solidity_to_manifest_type("bool"), "Boolean");
    assert_eq!(solidity_to_manifest_type("address"), "Hash160");
    assert_eq!(solidity_to_manifest_type("bytes32"), "ByteArray");
    assert_eq!(solidity_to_manifest_type("string"), "String");
    assert_eq!(solidity_to_manifest_type("uint256[]"), "Integer");
    assert_eq!(solidity_to_manifest_type("customStruct"), "Any");
}

#[test]
fn hex_prefixed_is_idempotent() {
    assert_eq!(hex_prefixed("deadbeef"), "0xdeadbeef");
    assert_eq!(hex_prefixed("0xdeadbeef"), "0xdeadbeef");
}

#[test]
fn metadata_blob_defaults_keccak_to_empty_hash() {
    let blob = build_metadata_blob(
        "Sample",
        &[],
        "Sample.sol",
        &json!({"optimizer": {"enabled": true}}),
        None,
    );
    let value: Value = serde_json::from_str(&blob).expect("metadata json");
    assert_eq!(
        value["sources"]["Sample.sol"]["keccak256"],
        Value::String(String::new()),
        "keccak field should be empty string when not provided"
    );
}

#[test]
fn state_mutability_label_maps_all_variants() {
    assert_eq!(state_mutability_label(StateMutability::Pure), "pure");
    assert_eq!(state_mutability_label(StateMutability::View), "view");
    assert_eq!(state_mutability_label(StateMutability::Payable), "payable");
    assert_eq!(
        state_mutability_label(StateMutability::NonPayable),
        "nonpayable"
    );
}
