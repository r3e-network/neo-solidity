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
        is_virtual: false,
        is_override: false,
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
            is_virtual: false,
            is_override: false,
            documentation: NatspecDoc::default(),
        },
    ];

    let result = detect_supported_standards(&methods, &[]);
    assert!(
        result.standards.iter().any(|s| s == "NEP-17"),
        "expected NEP-17 standard to be detected with all required methods"
    );

    // Adding extra methods should not affect detection
    methods.push(build_method("approve"));
    let result_extra = detect_supported_standards(&methods, &[]);
    assert!(
        result_extra.standards.iter().any(|s| s == "NEP-17"),
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
        is_virtual: false,
        is_override: false,
        documentation: NatspecDoc::default(),
    };

    let methods = vec![build_method("tokenUri")];
    let result = detect_supported_standards(&methods, &[]);
    assert!(
        result.standards.iter().any(|s| s == "NEP-24"),
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
            is_virtual: false,
            is_override: false,
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
            is_virtual: false,
            is_override: false,
            documentation: NatspecDoc::default(),
        },
    ];

    let result = detect_supported_standards(&methods, &[]);
    assert!(
        result.standards.is_empty(),
        "incomplete method sets should not advertise standards"
    );
}

#[test]
fn near_miss_nep17_warns_on_missing_methods() {
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
        is_virtual: false,
        is_override: false,
        documentation: NatspecDoc::default(),
    };

    // 4 of 5 NEP-17 methods (missing: transfer) → near-miss warning
    let methods = vec![
        build_method("balanceOf"),
        build_method("totalSupply"),
        build_method("symbol"),
        build_method("decimals"),
    ];
    let result = detect_supported_standards(&methods, &[]);
    assert!(
        result.standards.is_empty(),
        "should not detect NEP-17 with only 4 of 5 methods"
    );
    assert!(
        result.diagnostics.iter().any(|d| {
            d.standard == "NEP-17"
                && d.level == StandardsDiagnosticLevel::Warning
                && d.message.contains("missing")
        }),
        "expected near-miss warning for NEP-17, got: {:?}",
        result.diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}

#[test]
fn near_miss_nep11_warns_ownerof_without_transfer() {
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
        is_virtual: false,
        is_override: false,
        documentation: NatspecDoc::default(),
    };

    // ownerOf present but no transfer mechanism → near-miss
    let methods = vec![
        build_method("balanceOf"),
        build_method("ownerOf"),
    ];
    let result = detect_supported_standards(&methods, &[]);
    assert!(
        result.standards.is_empty(),
        "should not detect NEP-11 without transfer mechanism"
    );
    assert!(
        result.diagnostics.iter().any(|d| {
            d.standard == "NEP-11"
                && d.level == StandardsDiagnosticLevel::Warning
                && d.message.contains("transfer mechanism")
        }),
        "expected near-miss warning for NEP-11"
    );
}

#[test]
fn near_miss_nep26_warns_partial() {
    let build_method = |name: &str| FunctionMetadata {
        name: name.to_string(),
        neo_name: name.to_string(),
        kind: FunctionKind::Regular,
        parameters: vec![],
        return_parameters: vec![],
        state_mutability: StateMutability::NonPayable,
        visibility: VisibilityKind::Public,
        offset: 0,
        body: None,
        selector: [0u8; 4],
        is_virtual: false,
        is_override: false,
        documentation: NatspecDoc::default(),
    };

    // update without destroy → near-miss
    let methods = vec![build_method("update")];
    let result = detect_supported_standards(&methods, &[]);
    assert!(
        !result.standards.iter().any(|s| s == "NEP-26"),
        "should not detect NEP-26 with only update"
    );
    assert!(
        result.diagnostics.iter().any(|d| {
            d.standard == "NEP-26" && d.message.contains("destroy")
        }),
        "expected near-miss warning for NEP-26"
    );
}

#[test]
fn supported_standards_flags_nep26() {
    let build_method = |name: &str| FunctionMetadata {
        name: name.to_string(),
        neo_name: name.to_string(),
        kind: FunctionKind::Regular,
        parameters: vec![],
        return_parameters: vec![],
        state_mutability: StateMutability::NonPayable,
        visibility: VisibilityKind::Public,
        offset: 0,
        body: None,
        selector: [0u8; 4],
        is_virtual: false,
        is_override: false,
        documentation: NatspecDoc::default(),
    };

    let methods = vec![build_method("update"), build_method("destroy")];
    let result = detect_supported_standards(&methods, &[]);
    assert!(
        result.standards.iter().any(|s| s == "NEP-26"),
        "expected NEP-26 to be detected when update + destroy are present"
    );
}

#[test]
fn event_validation_warns_missing_transfer_event() {
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
        is_virtual: false,
        is_override: false,
        documentation: NatspecDoc::default(),
    };

    // NEP-17 detected but no Transfer event → warning
    let methods = vec![
        build_method("balanceOf"),
        build_method("totalSupply"),
        build_method("symbol"),
        build_method("decimals"),
        build_method("transfer"),
    ];
    let result = detect_supported_standards(&methods, &[]);
    assert!(
        result.standards.iter().any(|s| s == "NEP-17"),
        "NEP-17 should be detected"
    );
    assert!(
        result.diagnostics.iter().any(|d| {
            d.standard == "NEP-17"
                && d.level == StandardsDiagnosticLevel::Warning
                && d.message.contains("Transfer")
        }),
        "expected warning about missing Transfer event"
    );
}

#[test]
fn event_validation_accepts_correct_transfer_event() {
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
        is_virtual: false,
        is_override: false,
        documentation: NatspecDoc::default(),
    };

    let methods = vec![
        build_method("balanceOf"),
        build_method("totalSupply"),
        build_method("symbol"),
        build_method("decimals"),
        build_method("transfer"),
    ];

    // Correct NEP-17 Transfer event: 3 parameters
    let events = vec![EventMetadata {
        name: "Transfer".to_string(),
        normalized_name: "transfer".to_string(),
        parameters: vec![
            EventParameter {
                name: Some("from".to_string()),
                ty: "address".to_string(),
                indexed: true,
            },
            EventParameter {
                name: Some("to".to_string()),
                ty: "address".to_string(),
                indexed: true,
            },
            EventParameter {
                name: Some("value".to_string()),
                ty: "uint256".to_string(),
                indexed: false,
            },
        ],
    }];

    let result = detect_supported_standards(&methods, &events);
    assert!(
        result.standards.iter().any(|s| s == "NEP-17"),
        "NEP-17 should be detected"
    );
    // No Transfer-event warning when event is correct
    assert!(
        !result.diagnostics.iter().any(|d| {
            d.standard == "NEP-17" && d.message.contains("Transfer")
        }),
        "should not warn about Transfer event when it has correct param count"
    );
}
