/// Severity level for standards-detection diagnostics.
#[derive(Debug, Clone, PartialEq)]
enum StandardsDiagnosticLevel {
    /// Contract almost matches a standard, or a detected standard has issues.
    Warning,
    /// Informational hint about parameter signatures or events.
    Info,
}

/// A diagnostic emitted during standards detection.
#[derive(Debug, Clone)]
struct StandardsDiagnostic {
    level: StandardsDiagnosticLevel,
    standard: &'static str,
    message: String,
}

/// Result of standards detection: detected standards + any diagnostics.
struct StandardsDetectionResult {
    standards: Vec<String>,
    diagnostics: Vec<StandardsDiagnostic>,
}

fn detect_supported_standards(
    methods: &[FunctionMetadata],
    events: &[EventMetadata],
) -> StandardsDetectionResult {
    let public_methods: Vec<&FunctionMetadata> = methods
        .iter()
        .filter(|m| {
            !matches!(m.kind, FunctionKind::Constructor)
                && matches!(m.visibility, VisibilityKind::Public | VisibilityKind::External)
        })
        .collect();
    let names: HashSet<String> = public_methods
        .iter()
        .map(|m| m.name.to_ascii_lowercase())
        .collect();
    let mut standards = Vec::new();
    let mut diagnostics: Vec<StandardsDiagnostic> = Vec::new();

    let has_ownerof = names.contains("ownerof");

    // ── NEP-17: Fungible Token Standard (ERC-20 equivalent) ──────────
    let nep17_required = ["symbol", "decimals", "totalsupply", "balanceof", "transfer"];
    let nep17_present: Vec<&&str> = nep17_required.iter().filter(|m| names.contains(**m)).collect();
    let nep17_match = nep17_present.len() == nep17_required.len() && !has_ownerof;

    if nep17_match {
        standards.push("NEP-17".to_string());
        // Validate Transfer event
        validate_transfer_event(events, "NEP-17", 3, &mut diagnostics);
        // Hint: NEP-17 transfer should have 4 params (from, to, amount, data)
        check_transfer_params(&public_methods, "NEP-17", 4, &mut diagnostics);
    } else if nep17_present.len() >= 3 && !has_ownerof {
        // Near-miss: contract has most NEP-17 methods but not all
        let missing: Vec<&str> = nep17_required
            .iter()
            .filter(|m| !names.contains(**m))
            .copied()
            .collect();
        diagnostics.push(StandardsDiagnostic {
            level: StandardsDiagnosticLevel::Warning,
            standard: "NEP-17",
            message: format!(
                "contract has {} of {} required NEP-17 methods (missing: {}). \
                 Add the missing method(s) to enable NEP-17 standard detection.",
                nep17_present.len(),
                nep17_required.len(),
                missing.join(", "),
            ),
        });
    }

    // ── NEP-11: Non-Fungible Token Standard (ERC-721 equivalent) ─────
    let nep11_core = ["balanceof", "ownerof"];
    let has_nep11_xfer = names.contains("transfer")
        || names.contains("transferfrom")
        || names.contains("tokensof");
    let nep11_match =
        nep11_core.iter().all(|m| names.contains(*m)) && has_nep11_xfer;

    if nep11_match {
        standards.push("NEP-11".to_string());
        // Validate Transfer event (NEP-11 requires 4-param Transfer)
        validate_transfer_event(events, "NEP-11", 4, &mut diagnostics);
        // Hint: NEP-11 transfer should have 3 params (to, tokenId, data)
        check_transfer_params(&public_methods, "NEP-11", 3, &mut diagnostics);
    } else if has_ownerof && !has_nep11_xfer {
        // Near-miss: has ownerOf (NFT signal) but no transfer mechanism
        diagnostics.push(StandardsDiagnostic {
            level: StandardsDiagnosticLevel::Warning,
            standard: "NEP-11",
            message: "contract has `ownerOf` (NFT signal) but no transfer mechanism. \
                      Add `transfer`, `transferFrom`, or `tokensOf` to enable NEP-11."
                .to_string(),
        });
    } else if has_ownerof && has_nep11_xfer && !names.contains("balanceof") {
        diagnostics.push(StandardsDiagnostic {
            level: StandardsDiagnosticLevel::Warning,
            standard: "NEP-11",
            message: "contract has `ownerOf` and a transfer mechanism but is missing \
                      `balanceOf`. Add it to enable NEP-11 standard detection."
                .to_string(),
        });
    }

    // ── NEP-24: Token Discovery / Royalty Standard ───────────────────
    if names.contains("tokenuri") || names.contains("royaltyinfo") {
        standards.push("NEP-24".to_string());
    }

    // ── NEP-26: Contract Upgrade Standard ────────────────────────────
    let has_update = names.contains("update");
    let has_destroy = names.contains("destroy");
    if has_update && has_destroy {
        standards.push("NEP-26".to_string());
    } else if has_update && !has_destroy {
        diagnostics.push(StandardsDiagnostic {
            level: StandardsDiagnosticLevel::Warning,
            standard: "NEP-26",
            message: "contract has `update` but is missing `destroy`. \
                      Add both to enable NEP-26 standard detection."
                .to_string(),
        });
    } else if !has_update && has_destroy {
        diagnostics.push(StandardsDiagnostic {
            level: StandardsDiagnosticLevel::Warning,
            standard: "NEP-26",
            message: "contract has `destroy` but is missing `update`. \
                      Add both to enable NEP-26 standard detection."
                .to_string(),
        });
    }

    StandardsDetectionResult {
        standards,
        diagnostics,
    }
}

/// Check that a `Transfer` event exists with the expected parameter count.
fn validate_transfer_event(
    events: &[EventMetadata],
    standard: &'static str,
    expected_params: usize,
    diagnostics: &mut Vec<StandardsDiagnostic>,
) {
    let transfer_event = events.iter().find(|e| e.name == "Transfer");
    match transfer_event {
        None => {
            diagnostics.push(StandardsDiagnostic {
                level: StandardsDiagnosticLevel::Warning,
                standard,
                message: format!(
                    "{standard} detected but contract is missing the required `Transfer` event \
                     ({expected_params} parameters expected).",
                ),
            });
        }
        Some(evt) if evt.parameters.len() != expected_params => {
            diagnostics.push(StandardsDiagnostic {
                level: StandardsDiagnosticLevel::Info,
                standard,
                message: format!(
                    "{standard} `Transfer` event has {} parameter(s), expected {expected_params}.",
                    evt.parameters.len(),
                ),
            });
        }
        _ => {} // Event present with correct param count — all good.
    }
}

/// Hint when the `transfer` method parameter count doesn't match the standard.
fn check_transfer_params(
    public_methods: &[&FunctionMetadata],
    standard: &'static str,
    expected_params: usize,
    diagnostics: &mut Vec<StandardsDiagnostic>,
) {
    if let Some(transfer) = public_methods
        .iter()
        .find(|m| m.name.to_ascii_lowercase() == "transfer")
    {
        let actual = transfer.parameters.len();
        if actual != expected_params {
            diagnostics.push(StandardsDiagnostic {
                level: StandardsDiagnosticLevel::Info,
                standard,
                message: format!(
                    "{standard} `transfer` method has {actual} parameter(s), \
                     spec expects {expected_params}. See STANDARDS_MAPPING.md for details.",
                ),
            });
        }
    }
}
