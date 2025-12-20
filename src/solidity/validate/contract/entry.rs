pub fn validate_contract(metadata: &ContractMetadata) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let method_name_counts = build_method_name_counts(metadata);
    let constructor_count = validate_methods(metadata, &mut diagnostics);

    if constructor_count > 1 {
        diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Error,
            message: format!("multiple constructors defined ({} total)", constructor_count),
        });
    }

    validate_state_variables(metadata, &method_name_counts, &mut diagnostics);
    validate_events(metadata, &mut diagnostics);
    validate_return_types(metadata, &mut diagnostics);

    diagnostics
}
