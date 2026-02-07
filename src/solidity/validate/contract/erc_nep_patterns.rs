/// ERC → NEP pattern adaptation diagnostics.
///
/// Detects Ethereum-style patterns in Solidity contracts and emits warnings
/// with actionable guidance for migrating to Neo N3 equivalents.
///
/// Checked patterns:
/// - ERC-20 `transfer(to, amount)` → NEP-17 `transfer(from, to, amount, data)`
/// - ERC-20 `approve`/`allowance`/`transferFrom` → not in NEP-17 spec
/// - ERC-721 `transferFrom(from, to, tokenId)` → NEP-11 `transfer(to, tokenId, data)`
/// - `receive()` / `fallback()` → `onNEP17Payment()` callback
/// - `supportsInterface(bytes4)` → manifest `supportedstandards`
fn validate_erc_nep_patterns(metadata: &ContractMetadata, diagnostics: &mut Vec<Diagnostic>) {
    let public_methods: Vec<&FunctionMetadata> = metadata
        .methods
        .iter()
        .filter(|m| {
            !matches!(m.kind, FunctionKind::Constructor)
                && matches!(
                    m.visibility,
                    VisibilityKind::Public | VisibilityKind::External
                )
        })
        .collect();

    let names_lower: std::collections::HashSet<String> = public_methods
        .iter()
        .map(|m| m.name.to_ascii_lowercase())
        .collect();

    check_erc20_transfer_pattern(&public_methods, &names_lower, diagnostics);
    check_erc20_approve_pattern(&names_lower, diagnostics);
    check_erc721_transfer_from_pattern(&public_methods, &names_lower, diagnostics);
    check_receive_fallback_pattern(&metadata.methods, diagnostics);
    check_supports_interface_pattern(&public_methods, diagnostics);
}

/// Detect ERC-20 style `transfer(address, uint256)` and suggest NEP-17 4-param form.
fn check_erc20_transfer_pattern(
    public_methods: &[&FunctionMetadata],
    names: &std::collections::HashSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let has_ownerof = names.contains("ownerof");
    // Only flag for fungible-token-like contracts (no ownerOf → not NFT)
    if has_ownerof {
        return;
    }

    if let Some(transfer) = public_methods
        .iter()
        .find(|m| m.name.to_ascii_lowercase() == "transfer")
    {
        let param_count = transfer.parameters.len();
        if param_count == 2 {
            // Classic ERC-20: transfer(address to, uint256 amount)
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                message: format!(
                    "function 'transfer' has 2 parameters (ERC-20 pattern). \
                     NEP-17 requires 4 parameters: transfer(from, to, amount, data). \
                     The `from` address is verified via Runtime.checkWitness() and \
                     `data` (type Any) is forwarded to the recipient's onNEP17Payment callback. \
                     See STANDARDS_MAPPING.md §1 for migration details."
                ),
            });
        } else if param_count == 3 {
            // Partial migration: transfer(from, to, amount) — missing `data`
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                message: format!(
                    "function 'transfer' has 3 parameters, but NEP-17 requires 4: \
                     transfer(from, to, amount, data). The `data` parameter (type Any) \
                     is forwarded to the recipient's onNEP17Payment callback. \
                     See STANDARDS_MAPPING.md §1 for migration details."
                ),
            });
        }
    }
}

/// Detect ERC-20 approve/allowance/transferFrom and note they are not in NEP-17.
fn check_erc20_approve_pattern(
    names: &std::collections::HashSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let erc20_extras: Vec<&str> = ["approve", "allowance", "transferfrom"]
        .iter()
        .filter(|n| names.contains(**n))
        .copied()
        .collect();

    if !erc20_extras.is_empty() {
        // Only warn if the contract also looks like a token (has balanceOf or transfer)
        let has_token_signal = names.contains("balanceof") || names.contains("transfer");
        if has_token_signal {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                message: format!(
                    "ERC-20 method(s) [{}] detected. These are not part of the NEP-17 spec; \
                     Neo uses Runtime.checkWitness() for authorization instead of the \
                     approve/allowance pattern. You may keep them as extensions, but they \
                     will not contribute to NEP-17 standard detection.",
                    erc20_extras.join(", ")
                ),
            });
        }
    }
}

/// Detect ERC-721 `transferFrom(from, to, tokenId)` and suggest NEP-11 `transfer(to, tokenId, data)`.
fn check_erc721_transfer_from_pattern(
    public_methods: &[&FunctionMetadata],
    names: &std::collections::HashSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let has_ownerof = names.contains("ownerof");
    if !has_ownerof {
        return; // Not an NFT contract
    }

    if let Some(xfer_from) = public_methods
        .iter()
        .find(|m| m.name.to_ascii_lowercase() == "transferfrom")
    {
        let param_count = xfer_from.parameters.len();
        if param_count == 3 {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                message: format!(
                    "function 'transferFrom' with 3 parameters (ERC-721 pattern) detected. \
                     NEP-11 uses transfer(to, tokenId, data) with 3 parameters instead. \
                     Authorization is via Runtime.checkWitness(owner), not msg.sender. \
                     See STANDARDS_MAPPING.md §2 for migration details."
                ),
            });
        }
    }
}

/// Detect `receive()` / `fallback()` and suggest `onNEP17Payment()`.
fn check_receive_fallback_pattern(
    all_methods: &[FunctionMetadata],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let has_onnep17 = all_methods
        .iter()
        .any(|m| m.name.to_ascii_lowercase() == "onnep17payment");

    for method in all_methods {
        let name_lower = method.name.to_ascii_lowercase();
        if name_lower == "receive" || name_lower == "fallback" {
            if has_onnep17 {
                diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Warning,
                    message: format!(
                        "function '{}' has no effect on Neo N3. The contract already defines \
                         onNEP17Payment which is the correct Neo callback for receiving tokens.",
                        method.name
                    ),
                });
            } else {
                diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Warning,
                    message: format!(
                        "function '{}' has no effect on Neo N3. Use onNEP17Payment(address from, \
                         uint256 amount, bytes data) to handle incoming token payments. \
                         See STANDARDS_MAPPING.md §1 for details.",
                        method.name
                    ),
                });
            }
        }
    }
}

/// Detect `supportsInterface(bytes4)` and note that Neo uses manifest instead.
fn check_supports_interface_pattern(
    public_methods: &[&FunctionMetadata],
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(si) = public_methods
        .iter()
        .find(|m| m.name == "supportsInterface")
    {
        if si.parameters.len() == 1 {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                message: "function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. \
                         Neo uses the manifest 'supportedstandards' array for interface \
                         detection, which the compiler populates automatically. \
                         See STANDARDS_MAPPING.md §5 for details."
                    .to_string(),
            });
        }
    }
}
