fn validate_state_variables(
    metadata: &ContractMetadata,
    method_name_counts: &std::collections::HashMap<String, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    use std::collections::HashMap;

    // Track whether a previously seen variable name has any non-constant
    // declaration. Duplicate constant names can appear when third-party
    // libraries are merged for compatibility.
    let mut state_names: HashMap<String, bool> = HashMap::new();
    for state in &metadata.state_variables {
        match &state.name {
            Some(name) => {
                let current_non_constant = !state.is_constant;
                if let Some(previous_non_constant) = state_names.get_mut(name) {
                    if *previous_non_constant || current_non_constant {
                        diagnostics.push(
                            Diagnostic::warning(format!(
                                "duplicate state variable '{name}' detected while flattening/merging contracts"
                            ))
                            .with_code("W122")
                            .with_suggestion(
                                "confirm inherited/storage layout expectations; Neo lowering keeps the first declaration semantics where possible",
                            ),
                        );
                    } else {
                        diagnostics.push(
                            Diagnostic::warning(format!(
                                "duplicate constant state variable '{name}' detected while merging libraries"
                            ))
                            .with_code("W121")
                            .with_suggestion(
                                "qualify the constant by library name in Solidity source to avoid ambiguity",
                            ),
                        );
                    }
                    *previous_non_constant |= current_non_constant;
                } else {
                    state_names.insert(name.clone(), current_non_constant);
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
