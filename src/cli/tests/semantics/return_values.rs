#[test]
fn integer_type_constructors_truncate_like_solidity() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract CastU8FromBig {
        function run() public pure returns (uint256) {
            return uint8(300);
        }
    }

    contract CastU8FromNeg {
        function run() public pure returns (uint256) {
            return uint8(uint256(-1));
        }
    }

    contract CastI8FromU8 {
        function run() public pure returns (int256) {
            return int8(255);
        }
    }

    contract CastI8Wraps {
        function run() public pure returns (int256) {
            return int8(-129);
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");

    let u8_from_big = artifacts
        .iter()
        .find(|artifact| artifact.metadata.name == "CastU8FromBig")
        .expect("expected CastU8FromBig artifact");
    let result = execute_bytecode(&u8_from_big.bytecode);
    assert!(result.is_success(), "expected CastU8FromBig to succeed");
    assert_eq!(result.return_data, 44i64.to_le_bytes().to_vec());

    let u8_from_neg = artifacts
        .iter()
        .find(|artifact| artifact.metadata.name == "CastU8FromNeg")
        .expect("expected CastU8FromNeg artifact");
    let result = execute_bytecode(&u8_from_neg.bytecode);
    assert!(result.is_success(), "expected CastU8FromNeg to succeed");
    assert_eq!(result.return_data, 255i64.to_le_bytes().to_vec());

    let i8_from_u8 = artifacts
        .iter()
        .find(|artifact| artifact.metadata.name == "CastI8FromU8")
        .expect("expected CastI8FromU8 artifact");
    let result = execute_bytecode(&i8_from_u8.bytecode);
    assert!(result.is_success(), "expected CastI8FromU8 to succeed");
    assert_eq!(result.return_data, (-1i64).to_le_bytes().to_vec());

    let i8_wraps = artifacts
        .iter()
        .find(|artifact| artifact.metadata.name == "CastI8Wraps")
        .expect("expected CastI8Wraps artifact");
    let result = execute_bytecode(&i8_wraps.bytecode);
    assert!(result.is_success(), "expected CastI8Wraps to succeed");
    assert_eq!(result.return_data, 127i64.to_le_bytes().to_vec());
}

#[test]
fn return_without_value_returns_named_return_variable() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract ReturnNamedHarness {
        function value() public pure returns (uint256 x) {
            x = 7;
            return;
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    let result = execute_bytecode(&artifacts[0].bytecode);
    assert!(result.is_success(), "expected execution to succeed");
    assert_eq!(result.return_data, 7i64.to_le_bytes().to_vec());
}

#[test]
fn implicit_multi_return_is_array_for_neo_abi() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract MultiReturnHarness {
        function values() public pure returns (uint256 a, uint256 b) {
            a = 1;
            b = 2;
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    let result = execute_bytecode(&artifacts[0].bytecode);
    assert!(result.is_success(), "expected execution to succeed");

    let json: Value =
        serde_json::from_slice(&result.return_data).expect("expected JSON-encoded StackItem");
    assert_eq!(json["type"], "Array");
    assert_eq!(
        json["value"].as_array().map(|v| v.len()),
        Some(2),
        "expected 2-tuple packed into an Array"
    );
    assert_eq!(json["value"][0]["type"], "Integer");
    assert_eq!(json["value"][0]["value"].as_i64(), Some(1));
    assert_eq!(json["value"][1]["type"], "Integer");
    assert_eq!(json["value"][1]["value"].as_i64(), Some(2));
}

#[test]
fn explicit_return_without_value_packs_multi_returns_into_array() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract MultiReturnHarness {
        function values() public pure returns (uint256 a, uint256 b) {
            a = 1;
            b = 2;
            return;
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    let result = execute_bytecode(&artifacts[0].bytecode);
    assert!(result.is_success(), "expected execution to succeed");

    let json: Value =
        serde_json::from_slice(&result.return_data).expect("expected JSON-encoded StackItem");
    assert_eq!(json["type"], "Array");
    assert_eq!(
        json["value"].as_array().map(|v| v.len()),
        Some(2),
        "expected 2-tuple packed into an Array"
    );
    assert_eq!(json["value"][0]["type"], "Integer");
    assert_eq!(json["value"][0]["value"].as_i64(), Some(1));
    assert_eq!(json["value"][1]["type"], "Integer");
    assert_eq!(json["value"][1]["value"].as_i64(), Some(2));
}

#[test]
fn named_return_array_defaults_to_empty_array() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract DefaultArrayReturnHarness {
        function values() public pure returns (uint256[] memory out) {
            // No assignment; rely on Solidity default initialization.
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    let result = execute_bytecode(&artifacts[0].bytecode);
    assert!(result.is_success(), "expected execution to succeed");

    let json: Value =
        serde_json::from_slice(&result.return_data).expect("expected JSON-encoded StackItem");
    assert_eq!(json["type"], "Array");
    assert_eq!(
        json["value"].as_array().map(|v| v.len()),
        Some(0),
        "expected default-initialized dynamic array to be empty"
    );
}

#[test]
fn named_return_struct_defaults_fields_like_solidity() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract DefaultStructReturnHarness {
        struct S {
            uint256 a;
            bool b;
        }

        function value() public pure returns (S memory out) {
            // No assignment; rely on Solidity default initialization.
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    let result = execute_bytecode(&artifacts[0].bytecode);
    assert!(result.is_success(), "expected execution to succeed");

    let json: Value =
        serde_json::from_slice(&result.return_data).expect("expected JSON-encoded StackItem");
    assert_eq!(json["type"], "Array");
    assert_eq!(
        json["value"].as_array().map(|v| v.len()),
        Some(2),
        "expected struct to lower to a 2-element Array"
    );
    assert_eq!(json["value"][0]["type"], "Integer");
    assert_eq!(json["value"][0]["value"].as_i64(), Some(0));
    assert_eq!(json["value"][1]["type"], "Boolean");
    assert_eq!(json["value"][1]["value"].as_bool(), Some(false));
}
