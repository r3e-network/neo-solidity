fn detect_supported_standards(methods: &[FunctionMetadata]) -> Vec<String> {
    let names: HashSet<String> = methods
        .iter()
        .filter(|m| {
            !matches!(m.kind, FunctionKind::Constructor)
                && matches!(m.visibility, VisibilityKind::Public | VisibilityKind::External)
        })
        .map(|m| m.name.to_ascii_lowercase())
        .collect();
    let mut standards = Vec::new();

    // NEP-17: Fungible Token Standard (equivalent to ERC-20)
    // Required: symbol, decimals, totalSupply, balanceOf, transfer
    let nep17_required = ["symbol", "decimals", "totalsupply", "balanceof", "transfer"];
    if nep17_required.iter().all(|m| names.contains(*m)) {
        standards.push("NEP-17".to_string());
    }

    // NEP-11: Non-Fungible Token Standard (equivalent to ERC-721)
    // Core: balanceOf, ownerOf + at least one of: transfer, transferFrom, tokensOf
    let nep11_core = ["balanceof", "ownerof"];
    let nep11_transfer = ["transfer", "transferfrom", "tokensof"];
    if nep11_core.iter().all(|m| names.contains(*m))
        && nep11_transfer.iter().any(|m| names.contains(*m))
    {
        standards.push("NEP-11".to_string());
    }

    // NEP-24: Token Discovery Standard
    // For contracts that implement royalty info, token URIs, etc.
    if names.contains("tokenuri") || names.contains("royaltyinfo") {
        standards.push("NEP-24".to_string());
    }

    // NEP-26: Contract Upgrade Standard
    if names.contains("update") && names.contains("destroy") {
        standards.push("NEP-26".to_string());
    }

    // NEP-27: Invoke File Standard (for dynamic invocation)
    if names.contains("onpayment") || names.contains("onnep17payment") {
        standards.push("NEP-27".to_string());
    }

    standards
}
