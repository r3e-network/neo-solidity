use super::*;
use neo_solidity::frontend::VisibilityKind;
use neo_solidity::solidity::{ParameterMetadata, StateMutability};
use neo_solidity::type_system::NeoType;
use sha3::{Digest, Keccak256};

#[test]
fn method_identifiers_include_selectors() {
    let selector_bytes = {
        let mut hasher = Keccak256::new();
        hasher.update("foo(uint256)".as_bytes());
        let digest = hasher.finalize();
        let mut selector = [0u8; 4];
        selector.copy_from_slice(&digest[..4]);
        selector
    };

    let metadata = ContractMetadata {
        name: "Test".to_string(),
        methods: vec![FunctionMetadata {
            name: "foo".to_string(),
            kind: FunctionKind::Regular,
            parameters: vec![ParameterMetadata {
                name: Some("a".to_string()),
                ty: "uint256 memory".to_string(),
                neo_type: Some(NeoType::Integer {
                    signed: false,
                    bits: 256,
                }),
                storage: None,
            }],
            return_parameters: vec![],
            state_mutability: StateMutability::View,
            visibility: VisibilityKind::Public,
            offset: 0,
            body: None,
            selector: selector_bytes,
        }],
        events: vec![],
        uses_storage: false,
        state_variables: vec![],
        structs: vec![],
    };

    let identifiers = build_method_identifiers(&metadata);
    assert_eq!(identifiers.len(), 1);
    assert_eq!(
        identifiers
            .get("foo(uint256)")
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
        &hex_prefixed(&hex::encode(selector_bytes))
    );
}

#[test]
fn metadata_blob_populates_keccak() {
    let metadata = build_metadata_blob(
        "Example",
        &[],
        "Contract.sol",
        &json!({"optimizer": true}),
        Some("deadbeef"),
    );

    let value: Value = serde_json::from_str(&metadata).expect("metadata should be valid JSON");
    assert_eq!(
        value["sources"]["Contract.sol"]["keccak256"],
        Value::String("0xdeadbeef".to_string())
    );
    assert_eq!(value["compiler"]["name"], COMPILER_ID);
}

#[test]
fn supported_standards_flags_nep17() {
    let build_method = |name: &str| FunctionMetadata {
        name: name.to_string(),
        kind: FunctionKind::Regular,
        parameters: vec![ParameterMetadata {
            name: Some("addr".to_string()),
            ty: "address".to_string(),
            neo_type: Some(NeoType::Address),
            storage: None,
        }],
        return_parameters: vec![ParameterMetadata {
            name: None,
            ty: "uint256".to_string(),
            neo_type: Some(NeoType::Integer {
                signed: false,
                bits: 256,
            }),
            storage: None,
        }],
        state_mutability: StateMutability::View,
        visibility: VisibilityKind::Public,
        offset: 0,
        body: None,
        selector: [0u8; 4],
    };

    let mut methods = vec![
        build_method("balanceOf"),
        build_method("totalSupply"),
        FunctionMetadata {
            name: "transfer".to_string(),
            kind: FunctionKind::Regular,
            parameters: vec![
                ParameterMetadata {
                    name: Some("to".to_string()),
                    ty: "address".to_string(),
                    neo_type: Some(NeoType::Address),
                    storage: None,
                },
                ParameterMetadata {
                    name: Some("amount".to_string()),
                    ty: "uint256".to_string(),
                    neo_type: Some(NeoType::Integer {
                        signed: false,
                        bits: 256,
                    }),
                    storage: None,
                },
            ],
            return_parameters: vec![ParameterMetadata {
                name: None,
                ty: "bool".to_string(),
                neo_type: Some(NeoType::Boolean),
                storage: None,
            }],
            state_mutability: StateMutability::NonPayable,
            visibility: VisibilityKind::Public,
            offset: 0,
            body: None,
            selector: [0u8; 4],
        },
    ];

    let standards = detect_supported_standards(&methods);
    assert!(
        standards.iter().any(|s| s == "NEP-17"),
        "expected NEP-17 standard to be detected"
    );

    methods.push(build_method("symbol"));
    let standards_with_symbol = detect_supported_standards(&methods);
    assert!(
        standards_with_symbol.iter().any(|s| s == "NEP-17"),
        "NEP-17 should still be detected with extra methods"
    );
}

#[test]
fn supported_standards_flags_nep24() {
    let build_method = |name: &str| FunctionMetadata {
        name: name.to_string(),
        kind: FunctionKind::Regular,
        parameters: vec![],
        return_parameters: vec![],
        state_mutability: StateMutability::View,
        visibility: VisibilityKind::Public,
        offset: 0,
        body: None,
        selector: [0u8; 4],
    };

    let methods = vec![
        build_method("symbol"),
        build_method("decimals"),
        build_method("tokenSupply"),
    ];
    let standards = detect_supported_standards(&methods);
    assert!(
        standards.iter().any(|s| s == "NEP-24"),
        "expected NEP-24 standard to be detected"
    );
}

#[test]
fn contract_output_prefix_sanitizes_and_indexes() {
    // Single contract preserves base name
    assert_eq!(
        contract_output_prefix("out.nef", "MyContract", 0, 1),
        "out.nef"
    );

    // Multiple contracts append sanitized name and keep extension
    let prefixed = contract_output_prefix("bundle.nef", "My Contract!", 1, 3);
    assert_eq!(prefixed, "bundle-My_Contract.nef");
}
