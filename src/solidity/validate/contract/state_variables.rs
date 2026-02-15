fn validate_state_variables(
    metadata: &ContractMetadata,
    method_name_counts: &std::collections::HashMap<String, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    use std::collections::HashSet;

    let mut state_names = HashSet::new();
    for state in &metadata.state_variables {
        match &state.name {
            Some(name) => {
                if !state_names.insert(name.clone()) {
                    diagnostics.push(Diagnostic::error(format!("duplicate state variable '{name}'")));
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
                    diagnostics.push(Diagnostic::error(format!(
                        "public state variable '{name}' conflicts with a function of the same name"
                    )));
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
