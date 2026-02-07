// ── Helper: build minimal ContractMetadata for validation tests ───────────

fn build_test_contract(name: &str, methods: Vec<FunctionMetadata>) -> ContractMetadata {
    ContractMetadata {
        name: name.to_string(),
        methods,
        events: vec![],
        uses_storage: false,
        state_variables: vec![],
        structs: vec![],
        enums: vec![],
        contract_types: vec![],
        selector_registry: std::sync::Arc::new(SelectorRegistry::default()),
        documentation: NatspecDoc::default(),
    }
}

fn build_public_method(name: &str, param_count: usize) -> FunctionMetadata {
    let parameters: Vec<ParameterMetadata> = (0..param_count)
        .map(|i| ParameterMetadata {
            name: Some(format!("arg{}", i)),
            ty: "uint256".to_string(),
            neo_type: Some(NeoType::Integer {
                signed: false,
                bits: 256,
            }),
            storage: None,
        })
        .collect();

    FunctionMetadata {
        name: name.to_string(),
        neo_name: name.to_string(),
        kind: FunctionKind::Regular,
        parameters,
        return_parameters: vec![],
        state_mutability: StateMutability::NonPayable,
        visibility: VisibilityKind::Public,
        offset: 0,
        body: None,
        selector: [0u8; 4],
        documentation: NatspecDoc::default(),
    }
}

fn warnings_containing<'a>(diagnostics: &'a [Diagnostic], substring: &str) -> Vec<&'a Diagnostic> {
    diagnostics
        .iter()
        .filter(|d| {
            d.severity == DiagnosticSeverity::Warning && d.message.contains(substring)
        })
        .collect()
}

// ── ERC-20 transfer(to, amount) → NEP-17 transfer(from, to, amount, data) ──

#[test]
fn erc20_transfer_2_params_warns_nep17() {
    let methods = vec![
        build_public_method("transfer", 2),
        build_public_method("balanceOf", 1),
        build_public_method("totalSupply", 0),
    ];
    let metadata = build_test_contract("ERC20Token", methods);
    let diagnostics = validate_contract(&metadata);

    let transfer_warns = warnings_containing(&diagnostics, "NEP-17 requires 4 parameters");
    assert!(
        !transfer_warns.is_empty(),
        "expected warning about ERC-20 transfer(to, amount) needing 4 params for NEP-17, got: {:?}",
        diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}

#[test]
fn erc20_transfer_3_params_warns_missing_data() {
    let methods = vec![
        build_public_method("transfer", 3),
        build_public_method("balanceOf", 1),
    ];
    let metadata = build_test_contract("PartialNEP17", methods);
    let diagnostics = validate_contract(&metadata);

    let warns = warnings_containing(&diagnostics, "NEP-17 requires 4");
    assert!(
        !warns.is_empty(),
        "expected warning about 3-param transfer missing `data`, got: {:?}",
        diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}

#[test]
fn nep17_transfer_4_params_no_erc_warning() {
    let methods = vec![
        build_public_method("transfer", 4),
        build_public_method("balanceOf", 1),
        build_public_method("totalSupply", 0),
        build_public_method("symbol", 0),
        build_public_method("decimals", 0),
    ];
    let metadata = build_test_contract("GoodNEP17", methods);
    let diagnostics = validate_contract(&metadata);

    let erc_warns = warnings_containing(&diagnostics, "ERC-20 pattern");
    assert!(
        erc_warns.is_empty(),
        "NEP-17 compliant transfer should not trigger ERC-20 warning, got: {:?}",
        erc_warns.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}

// ── ERC-20 approve/allowance/transferFrom → not in NEP-17 ──────────────────

#[test]
fn erc20_approve_pattern_warns() {
    let methods = vec![
        build_public_method("transfer", 4),
        build_public_method("balanceOf", 1),
        build_public_method("approve", 2),
        build_public_method("allowance", 2),
    ];
    let metadata = build_test_contract("ERC20WithApprove", methods);
    let diagnostics = validate_contract(&metadata);

    let warns = warnings_containing(&diagnostics, "not part of the NEP-17 spec");
    assert!(
        !warns.is_empty(),
        "expected warning about approve/allowance not being in NEP-17, got: {:?}",
        diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}

#[test]
fn approve_without_token_signal_no_warning() {
    // Contract with approve but no balanceOf/transfer → not a token, no warning
    let methods = vec![
        build_public_method("approve", 2),
        build_public_method("doSomething", 1),
    ];
    let metadata = build_test_contract("NotAToken", methods);
    let diagnostics = validate_contract(&metadata);

    let warns = warnings_containing(&diagnostics, "NEP-17 spec");
    assert!(
        warns.is_empty(),
        "approve without token signals should not warn, got: {:?}",
        warns.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}

// ── ERC-721 transferFrom → NEP-11 transfer ─────────────────────────────────

#[test]
fn erc721_transferfrom_warns_nep11() {
    let methods = vec![
        build_public_method("ownerOf", 1),
        build_public_method("balanceOf", 1),
        build_public_method("transferFrom", 3),
    ];
    let metadata = build_test_contract("ERC721Token", methods);
    let diagnostics = validate_contract(&metadata);

    let warns = warnings_containing(&diagnostics, "NEP-11 uses transfer");
    assert!(
        !warns.is_empty(),
        "expected warning about ERC-721 transferFrom → NEP-11 transfer, got: {:?}",
        diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}

#[test]
fn transferfrom_without_ownerof_no_nft_warning() {
    // transferFrom without ownerOf → not an NFT contract, no NFT-specific warning
    let methods = vec![
        build_public_method("transferFrom", 3),
        build_public_method("balanceOf", 1),
    ];
    let metadata = build_test_contract("NotNFT", methods);
    let diagnostics = validate_contract(&metadata);

    let warns = warnings_containing(&diagnostics, "NEP-11 uses transfer");
    assert!(
        warns.is_empty(),
        "transferFrom without ownerOf should not trigger NFT warning, got: {:?}",
        warns.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}

// ── receive() / fallback() → onNEP17Payment() ─────────────────────────────

#[test]
fn receive_function_warns_onnep17payment() {
    let methods = vec![build_public_method("receive", 0)];
    let metadata = build_test_contract("ReceiverContract", methods);
    let diagnostics = validate_contract(&metadata);

    let warns = warnings_containing(&diagnostics, "onNEP17Payment");
    assert!(
        !warns.is_empty(),
        "expected warning about receive() → onNEP17Payment, got: {:?}",
        diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}

#[test]
fn fallback_function_warns_onnep17payment() {
    let methods = vec![build_public_method("fallback", 0)];
    let metadata = build_test_contract("FallbackContract", methods);
    let diagnostics = validate_contract(&metadata);

    let warns = warnings_containing(&diagnostics, "onNEP17Payment");
    assert!(
        !warns.is_empty(),
        "expected warning about fallback() → onNEP17Payment, got: {:?}",
        diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}

#[test]
fn receive_with_existing_onnep17payment_warns_no_effect() {
    let methods = vec![
        build_public_method("receive", 0),
        build_public_method("onNEP17Payment", 3),
    ];
    let metadata = build_test_contract("DualCallback", methods);
    let diagnostics = validate_contract(&metadata);

    let warns = warnings_containing(&diagnostics, "no effect");
    assert!(
        !warns.is_empty(),
        "expected 'no effect' warning when both receive and onNEP17Payment exist, got: {:?}",
        diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}

// ── supportsInterface(bytes4) → manifest-based ─────────────────────────────

#[test]
fn supports_interface_warns_manifest() {
    let methods = vec![build_public_method("supportsInterface", 1)];
    let metadata = build_test_contract("EIP165Contract", methods);
    let diagnostics = validate_contract(&metadata);

    let warns = warnings_containing(&diagnostics, "unnecessary on Neo N3");
    assert!(
        !warns.is_empty(),
        "expected warning about supportsInterface being unnecessary, got: {:?}",
        diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}

// ── Clean NEP-17 contract produces no ERC pattern warnings ─────────────────

#[test]
fn clean_nep17_contract_no_erc_pattern_warnings() {
    let methods = vec![
        build_public_method("symbol", 0),
        build_public_method("decimals", 0),
        build_public_method("totalSupply", 0),
        build_public_method("balanceOf", 1),
        build_public_method("transfer", 4),
        build_public_method("onNEP17Payment", 3),
    ];
    let metadata = build_test_contract("CleanNEP17", methods);
    let diagnostics = validate_contract(&metadata);

    let erc_warns: Vec<_> = diagnostics
        .iter()
        .filter(|d| {
            d.severity == DiagnosticSeverity::Warning
                && (d.message.contains("ERC-20")
                    || d.message.contains("ERC-721")
                    || d.message.contains("EIP-165")
                    || d.message.contains("no effect"))
        })
        .collect();

    assert!(
        erc_warns.is_empty(),
        "clean NEP-17 contract should have no ERC pattern warnings, got: {:?}",
        erc_warns.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}
