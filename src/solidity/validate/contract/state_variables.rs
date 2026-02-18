#[derive(Debug, Clone, PartialEq, Eq)]
struct ConstantStateSignature {
    ty: String,
    neo_type: Option<NeoType>,
    is_immutable: bool,
    has_initializer: bool,
    initializer_fingerprint: Option<String>,
}

#[derive(Debug, Default)]
struct SeenStateVariables {
    has_non_constant: bool,
    constant_signatures: Vec<ConstantStateSignature>,
}

fn build_constant_state_signature(state: &StateVariableMetadata) -> ConstantStateSignature {
    ConstantStateSignature {
        ty: state.ty.trim().to_ascii_lowercase(),
        neo_type: state.neo_type.clone(),
        is_immutable: state.is_immutable,
        has_initializer: state.has_initializer,
        initializer_fingerprint: state
            .initializer
            .as_ref()
            .map(build_initializer_fingerprint),
    }
}

fn build_initializer_fingerprint(expr: &Expression) -> String {
    match expr {
        Expression::Parenthesis(_, inner) => build_initializer_fingerprint(inner),
        Expression::BoolLiteral(_, value) => format!("bool:{value}"),
        Expression::AddressLiteral(_, value) => format!("address:{}", value.to_ascii_lowercase()),
        Expression::NumberLiteral(_, integer, exponent, unit) => format!(
            "number:{}:{}:{}",
            integer.to_ascii_lowercase(),
            exponent.to_ascii_lowercase(),
            unit.as_ref()
                .map(|identifier| identifier.name.to_ascii_lowercase())
                .unwrap_or_default()
        ),
        Expression::HexNumberLiteral(_, value, unit) => format!(
            "hex-number:{}:{}",
            value.to_ascii_lowercase(),
            unit.as_ref()
                .map(|identifier| identifier.name.to_ascii_lowercase())
                .unwrap_or_default()
        ),
        Expression::RationalNumberLiteral(_, integer, fraction, exponent, unit) => format!(
            "rational:{}:{}:{}:{}",
            integer.to_ascii_lowercase(),
            fraction.to_ascii_lowercase(),
            exponent.to_ascii_lowercase(),
            unit.as_ref()
                .map(|identifier| identifier.name.to_ascii_lowercase())
                .unwrap_or_default()
        ),
        _ => format!("{expr:?}"),
    }
}

impl SeenStateVariables {
    fn from_state(state: &StateVariableMetadata) -> Self {
        if state.is_constant {
            Self {
                has_non_constant: false,
                constant_signatures: vec![build_constant_state_signature(state)],
            }
        } else {
            Self {
                has_non_constant: true,
                constant_signatures: Vec::new(),
            }
        }
    }
}

fn validate_state_variables(
    metadata: &ContractMetadata,
    method_name_counts: &std::collections::HashMap<String, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    use std::collections::{HashMap, HashSet};

    let mut state_names: HashMap<String, SeenStateVariables> = HashMap::new();
    let mut warned_non_constant_duplicates: HashSet<String> = HashSet::new();
    let mut warned_conflicting_constant_duplicates: HashSet<String> = HashSet::new();

    for state in &metadata.state_variables {
        match &state.name {
            Some(name) => {
                let current_non_constant = !state.is_constant;
                if let Some(seen) = state_names.get_mut(name) {
                    if seen.has_non_constant || current_non_constant {
                        if warned_non_constant_duplicates.insert(name.clone()) {
                            diagnostics.push(
                                Diagnostic::warning(format!(
                                    "duplicate state variable '{name}' detected while flattening/merging contracts"
                                ))
                                .with_code("W122")
                                .with_suggestion(
                                    "confirm inherited/storage layout expectations; Neo lowering keeps the first declaration semantics where possible",
                                ),
                            );
                        }
                        seen.has_non_constant = true;
                    } else {
                        let current_signature = build_constant_state_signature(state);
                        let is_identical_duplicate = seen
                            .constant_signatures
                            .iter()
                            .any(|signature| signature == &current_signature);

                        if !is_identical_duplicate {
                            if warned_conflicting_constant_duplicates.insert(name.clone()) {
                                diagnostics.push(
                                    Diagnostic::warning(format!(
                                        "conflicting duplicate constant state variable '{name}' detected while merging libraries"
                                    ))
                                    .with_code("W121")
                                    .with_suggestion(
                                        "keep merged constants bit-for-bit identical, or qualify by library name to avoid ambiguity",
                                    ),
                                );
                            }
                            seen.constant_signatures.push(current_signature);
                        }
                    }
                } else {
                    state_names.insert(name.clone(), SeenStateVariables::from_state(state));
                }
            }
            None => diagnostics.push(Diagnostic::error("state variable declared without a name")),
        }

        if state
            .visibility
            .as_deref()
            .map(|v| v.eq_ignore_ascii_case("public"))
            == Some(true)
        {
            if let Some(name) = state.name.as_deref() {
                if method_name_counts.get(name).copied().unwrap_or(0) > 1 {
                    diagnostics.push(
                        Diagnostic::warning(format!(
                            "public state variable '{name}' conflicts with a function of the same name"
                        ))
                        .with_code("W123")
                        .with_suggestion(
                            "if this came from merged dependencies, prefer explicit wrapper names in user-facing ABIs",
                        ),
                    );
                }
            }
        }

        if state.neo_type.is_none() {
            let lower_ty = state.ty.to_ascii_lowercase();
            if lower_ty.starts_with("fixed") || lower_ty.starts_with("ufixed") {
                diagnostics.push(
                    Diagnostic::error(format!(
                        "state variable '{}' uses fixed-point type '{}' which is not supported on NeoVM",
                        state.name.as_deref().unwrap_or("<unnamed>"),
                        state.ty
                    ))
                    .with_suggestion(
                        "use scaled integer arithmetic instead (e.g., multiply by 10^18 for 18 decimal places)"
                    ),
                );
            } else {
                diagnostics.push(Diagnostic::error(format!(
                    "state variable '{}' has unsupported type '{}'",
                    state.name.as_deref().unwrap_or("<unnamed>"),
                    state.ty
                )));
            }
        }

        if state.is_constant && !state.has_initializer {
            diagnostics.push(Diagnostic::error(format!(
                "constant state variable '{}' must have an initializer",
                state.name.as_deref().unwrap_or("<unnamed>")
            )));
        }
    }
}
