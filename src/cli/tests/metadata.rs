use super::*;
use neo_solidity::frontend::VisibilityKind;
use neo_solidity::solidity::{
    EventMetadata, EventParameter, NatspecDoc, ParameterMetadata, StateMutability,
    StateVariableMetadata,
};
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
            documentation: NatspecDoc::default(),
        }],
        events: vec![],
        uses_storage: false,
        state_variables: vec![],
        structs: vec![],
        documentation: NatspecDoc::default(),
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
        documentation: NatspecDoc::default(),
    };

    // NEP-17 requires all 5 methods: symbol, decimals, totalSupply, balanceOf, transfer
    let mut methods = vec![
        build_method("balanceOf"),
        build_method("totalSupply"),
        build_method("symbol"),
        build_method("decimals"),
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
            documentation: NatspecDoc::default(),
        },
    ];

    let standards = detect_supported_standards(&methods);
    assert!(
        standards.iter().any(|s| s == "NEP-17"),
        "expected NEP-17 standard to be detected with all required methods"
    );

    // Adding extra methods should not affect detection
    methods.push(build_method("approve"));
    let standards_with_extra = detect_supported_standards(&methods);
    assert!(
        standards_with_extra.iter().any(|s| s == "NEP-17"),
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
        documentation: NatspecDoc::default(),
    };

    let methods = vec![
        build_method("tokenUri"),
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

#[test]
fn standards_not_detected_when_incomplete() {
    // Missing balanceOf should not trigger NEP-17
    let methods = vec![
        FunctionMetadata {
            name: "transfer".to_string(),
            kind: FunctionKind::Regular,
            parameters: vec![],
            return_parameters: vec![],
            state_mutability: StateMutability::NonPayable,
            visibility: VisibilityKind::Public,
            offset: 0,
            body: None,
            selector: [0u8; 4],
            documentation: NatspecDoc::default(),
        },
        FunctionMetadata {
            name: "totalSupply".to_string(),
            kind: FunctionKind::Regular,
            parameters: vec![],
            return_parameters: vec![],
            state_mutability: StateMutability::View,
            visibility: VisibilityKind::Public,
            offset: 0,
            body: None,
            selector: [0u8; 4],
            documentation: NatspecDoc::default(),
        },
    ];

    let standards = detect_supported_standards(&methods);
    assert!(
        standards.is_empty(),
        "incomplete method sets should not advertise standards"
    );
}

#[test]
fn storage_map_assigns_slots_and_names() {
    let metadata = ContractMetadata {
        name: "StorageExample".to_string(),
        methods: vec![],
        events: vec![],
        uses_storage: true,
        structs: vec![],
        state_variables: vec![
            StateVariableMetadata {
                name: Some("alpha".to_string()),
                ty: "uint256".to_string(),
                is_constant: false,
                is_immutable: false,
                visibility: Some("public".to_string()),
                neo_type: None,
                has_initializer: false,
            },
            StateVariableMetadata {
                name: None,
                ty: "address".to_string(),
                is_constant: false,
                is_immutable: false,
                visibility: Some("internal".to_string()),
                neo_type: None,
                has_initializer: false,
            },
            StateVariableMetadata {
                name: Some("gamma".to_string()),
                ty: "bool".to_string(),
                is_constant: false,
                is_immutable: false,
                visibility: None,
                neo_type: None,
                has_initializer: false,
            },
        ],
        documentation: NatspecDoc::default(),
    };

    let map = build_storage_map(&metadata);
    let obj = map.as_object().expect("storage map object");

    assert_eq!(obj["alpha"]["slot"], Value::from(0));
    assert_eq!(obj["alpha"]["type"], Value::from("uint256"));
    assert_eq!(obj["slot_1"]["slot"], Value::from(1));
    assert_eq!(
        obj["slot_1"]["description"],
        Value::from("internal".to_string())
    );
    assert_eq!(obj["gamma"]["slot"], Value::from(2));
}

#[test]
fn standard_abi_includes_constructor_and_event() {
    let constructor = FunctionMetadata {
        name: "constructor".to_string(),
        kind: FunctionKind::Constructor,
        parameters: vec![
            ParameterMetadata {
                name: Some("owner".to_string()),
                ty: "address".to_string(),
                neo_type: Some(NeoType::Address),
                storage: None,
            },
            ParameterMetadata {
                name: Some("initialSupply".to_string()),
                ty: "uint256".to_string(),
                neo_type: Some(NeoType::Integer {
                    signed: false,
                    bits: 256,
                }),
                storage: None,
            },
        ],
        return_parameters: vec![],
        state_mutability: StateMutability::NonPayable,
        visibility: VisibilityKind::Public,
        offset: 0,
        body: None,
        selector: [0u8; 4],
        documentation: NatspecDoc::default(),
    };

    let regular = FunctionMetadata {
        name: "balanceOf".to_string(),
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
        documentation: NatspecDoc::default(),
    };

    let metadata = ContractMetadata {
        name: "Token".to_string(),
        methods: vec![constructor, regular],
        events: vec![EventMetadata {
            name: "Mint".to_string(),
            normalized_name: "Mint".to_string(),
            parameters: vec![
                EventParameter {
                    name: Some("to".to_string()),
                    ty: "address".to_string(),
                    indexed: true,
                },
                EventParameter {
                    name: Some("amount".to_string()),
                    ty: "uint256".to_string(),
                    indexed: false,
                },
            ],
        }],
        uses_storage: true,
        state_variables: vec![],
        structs: vec![],
        documentation: NatspecDoc::default(),
    };

    let abi = build_standard_abi(&metadata);
    let ctor_entry = abi
        .iter()
        .find(|entry| entry["type"] == "constructor")
        .expect("constructor entry");
    assert_eq!(
        ctor_entry["inputs"]
            .as_array()
            .map(|a| a.len())
            .unwrap_or(0),
        2
    );
    assert_eq!(ctor_entry["stateMutability"], Value::from("nonpayable"));

    let function_entry = abi
        .iter()
        .find(|entry| entry["name"] == "balanceOf")
        .expect("balanceOf");
    assert_eq!(function_entry["outputs"][0]["type"], Value::from("uint256"));

    let event_entry = abi
        .iter()
        .find(|entry| entry["type"] == "event" && entry["name"] == "Mint")
        .expect("Mint event");
    assert_eq!(
        event_entry["inputs"][0]["indexed"],
        Value::Bool(true),
        "first parameter should be indexed"
    );
}
