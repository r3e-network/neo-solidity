#[test]
fn supported_standards_flags_nep17() {
    let build_method = |name: &str| FunctionMetadata {
        name: name.to_string(),
        neo_name: name.to_string(),
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
            neo_name: "transfer".to_string(),
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
        neo_name: name.to_string(),
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

    let methods = vec![build_method("tokenUri")];
    let standards = detect_supported_standards(&methods);
    assert!(
        standards.iter().any(|s| s == "NEP-24"),
        "expected NEP-24 standard to be detected"
    );
}

#[test]
fn standards_not_detected_when_incomplete() {
    // Missing balanceOf should not trigger NEP-17
    let methods = vec![
        FunctionMetadata {
            name: "transfer".to_string(),
            neo_name: "transfer".to_string(),
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
            neo_name: "totalSupply".to_string(),
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

