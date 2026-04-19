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

    // ── NEP-22: Contract Update Function ─────────────────────────────
    let update_methods = methods_named(&public_methods, "update");
    if update_methods
        .iter()
        .any(|method| method.parameters.len() == 3)
    {
        standards.push("NEP-22".to_string());
    } else if !update_methods.is_empty() {
        diagnostics.push(StandardsDiagnostic {
            level: StandardsDiagnosticLevel::Info,
            standard: "NEP-22",
            message: "contract has `update` but no 3-parameter NEP-22 signature \
                      `update(nefFile, manifest, data)`."
                .to_string(),
        });
    }

    // ── NEP-26: NEP-11 Receiver Callback ─────────────────────────────
    let nep11_payment_methods = methods_named(&public_methods, "onNEP11Payment");
    if nep11_payment_methods
        .iter()
        .any(|method| method.parameters.len() == 4)
    {
        standards.push("NEP-26".to_string());
    } else if !nep11_payment_methods.is_empty() {
        diagnostics.push(StandardsDiagnostic {
            level: StandardsDiagnosticLevel::Info,
            standard: "NEP-26",
            message: "contract has `onNEP11Payment` but signature does not match \
                      NEP-26 (`onNEP11Payment(from, amount, tokenId, data)`)."
                .to_string(),
        });
    }

    // ── NEP-27: NEP-17 Receiver Callback ─────────────────────────────
    let nep17_payment_methods = methods_named(&public_methods, "onNEP17Payment");
    if nep17_payment_methods
        .iter()
        .any(|method| method.parameters.len() == 3)
    {
        standards.push("NEP-27".to_string());
    } else if !nep17_payment_methods.is_empty() {
        diagnostics.push(StandardsDiagnostic {
            level: StandardsDiagnosticLevel::Info,
            standard: "NEP-27",
            message: "contract has `onNEP17Payment` but signature does not match \
                      NEP-27 (`onNEP17Payment(from, amount, data)`)."
                .to_string(),
        });
    }

    // ── NEP-29: Deployment/Update Callback ───────────────────────────
    let deploy_methods = methods_named(&public_methods, "_deploy");
    if deploy_methods
        .iter()
        .any(|method| method.parameters.len() == 2)
    {
        standards.push("NEP-29".to_string());
    } else if !deploy_methods.is_empty() {
        diagnostics.push(StandardsDiagnostic {
            level: StandardsDiagnosticLevel::Info,
            standard: "NEP-29",
            message: "contract has `_deploy` but signature does not match \
                      NEP-29 (`_deploy(data, update)`)."
                .to_string(),
        });
    }

    // ── NEP-30: Contract Witness Verification Callback ───────────────
    let verify_methods = methods_named(&public_methods, "verify");
    if verify_methods.len() == 1 && method_returns_bool(verify_methods[0]) {
        standards.push("NEP-30".to_string());
    } else if verify_methods.len() > 1 {
        diagnostics.push(StandardsDiagnostic {
            level: StandardsDiagnosticLevel::Warning,
            standard: "NEP-30",
            message: "contract has multiple `verify` methods. NEP-30 requires a \
                      single `verify` entrypoint."
                .to_string(),
        });
    } else if verify_methods.len() == 1 {
        diagnostics.push(StandardsDiagnostic {
            level: StandardsDiagnosticLevel::Info,
            standard: "NEP-30",
            message: "contract has `verify` but it does not return `bool` as \
                      required by NEP-30."
                .to_string(),
        });
    }

    // ── NEP-31: Contract Destroy Guideline ───────────────────────────
    let destroy_methods = methods_named(&public_methods, "destroy");
    if destroy_methods
        .iter()
        .any(|method| method.parameters.is_empty() && method.return_parameters.is_empty())
    {
        standards.push("NEP-31".to_string());
    } else if !destroy_methods.is_empty() {
        diagnostics.push(StandardsDiagnostic {
            level: StandardsDiagnosticLevel::Info,
            standard: "NEP-31",
            message: "contract has `destroy` but no parameterless void signature \
                      required by NEP-31."
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
        .find(|m| m.name.eq_ignore_ascii_case("transfer"))
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

/// Returns all public/external methods with a given name (case-insensitive).
fn methods_named<'a>(
    public_methods: &[&'a FunctionMetadata],
    name: &str,
) -> Vec<&'a FunctionMetadata> {
    public_methods
        .iter()
        .copied()
        .filter(|method| method.name.eq_ignore_ascii_case(name))
        .collect()
}

/// Best-effort bool return detection for standards validation.
fn method_returns_bool(method: &FunctionMetadata) -> bool {
    if method.return_parameters.len() != 1 {
        return false;
    }
    let ret = &method.return_parameters[0];
    if let Some(neo_type) = &ret.neo_type {
        if matches!(neo_type, NeoType::Boolean) {
            return true;
        }
    }
    ret.ty.eq_ignore_ascii_case("bool")
}

/// Task #28: hard-validate standards the user EXPLICITLY asserted via
/// `@custom:neo.manifest.supportedstandards`. For NEP-17 / NEP-11 the
/// required methods and `Transfer` event are mandatory — shipping a
/// contract that claims the standard without them is a manifest error,
/// not a warning. Implicit (auto-detected) standards remain advisory
/// because detection already requires the full method set.
fn validate_declared_standards(
    declared: &[String],
    methods: &[FunctionMetadata],
    events: &[EventMetadata],
) -> Result<(), String> {
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
    let transfer_params = events
        .iter()
        .find(|e| e.name == "Transfer")
        .map(|e| e.parameters.len());

    for std in declared {
        let (required, expected_transfer_params): (&[&str], usize) = match std.as_str() {
            "NEP-17" => (&["symbol", "decimals", "totalsupply", "balanceof", "transfer"], 3),
            "NEP-11" => (&["symbol", "decimals", "balanceof", "ownerof", "transfer"], 4),
            _ => continue,
        };
        let missing: Vec<&str> = required
            .iter()
            .copied()
            .filter(|m| !names.contains(*m))
            .collect();
        if !missing.is_empty() {
            return Err(format!(
                "contract declares `{std}` in supportedstandards but is missing required method(s): {}. \
                 Either implement the missing method(s) or remove `{std}` from @custom:neo.manifest.supportedstandards.",
                missing.join(", "),
            ));
        }
        match transfer_params {
            None => {
                return Err(format!(
                    "contract declares `{std}` in supportedstandards but is missing the required `Transfer` event \
                     ({expected_transfer_params} parameters expected). Either emit a `Transfer` event or remove `{std}` \
                     from @custom:neo.manifest.supportedstandards."
                ));
            }
            Some(n) if n != expected_transfer_params => {
                return Err(format!(
                    "contract declares `{std}` in supportedstandards but its `Transfer` event has {n} parameter(s); \
                     {std} requires {expected_transfer_params}. Fix the event signature or remove `{std}` \
                     from @custom:neo.manifest.supportedstandards."
                ));
            }
            _ => {}
        }
    }
    Ok(())
}
