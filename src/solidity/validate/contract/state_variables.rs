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
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        message: format!("duplicate state variable '{}'", name),
                    });
                }
            }
            None => diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Error,
                message: "state variable declared without a name".to_string(),
            }),
        }

        if state
            .visibility
            .as_deref()
            .map(|v| v.eq_ignore_ascii_case("public"))
            == Some(true)
        {
            if let Some(name) = state.name.as_deref() {
                if method_name_counts.get(name).copied().unwrap_or(0) > 1 {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        message: format!(
                            "public state variable '{}' conflicts with a function of the same name",
                            name
                        ),
                    });
                }
            }
        }

        if state.neo_type.is_none() {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Error,
                message: format!(
                    "state variable '{}' has unsupported type '{}'",
                    state.name.as_deref().unwrap_or("<unnamed>"),
                    state.ty
                ),
            });
        }

        if state.is_constant && !state.has_initializer {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Error,
                message: format!(
                    "constant state variable '{}' must have an initializer",
                    state.name.as_deref().unwrap_or("<unnamed>")
                ),
            });
        }
    }
}
