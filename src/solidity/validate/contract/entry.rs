pub fn validate_contract(metadata: &ContractMetadata) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let method_name_counts = build_method_name_counts(metadata);
    let constructor_count = validate_methods(metadata, &mut diagnostics);

    if constructor_count > 1 {
        diagnostics.push(Diagnostic::error(format!("multiple constructors defined ({} total)", constructor_count)));
    }

    validate_state_variables(metadata, &method_name_counts, &mut diagnostics);
    validate_events(metadata, &mut diagnostics);
    validate_return_types(metadata, &mut diagnostics);
    validate_erc_nep_patterns(metadata, &mut diagnostics);
    validate_using_directives(metadata, &mut diagnostics);
    validate_type_definitions(metadata, &mut diagnostics);
    validate_library(metadata, &mut diagnostics);
    validate_abstract_contract(metadata, &mut diagnostics);
    validate_flatten_warnings(metadata, &mut diagnostics);

    diagnostics
}

fn validate_using_directives(metadata: &ContractMetadata, diagnostics: &mut Vec<Diagnostic>) {
    if metadata.has_using_for_star {
        diagnostics.push(
            Diagnostic::warning(
                "'using X for *' is supported; all library functions are available \
                 but type-specific filtering is not enforced at compile time"
            )
            .with_suggestion(
                "this is functionally correct — library functions can be called \
                 as member functions on any type"
            ),
        );
    }

    if metadata.has_using_function_list {
        diagnostics.push(
            Diagnostic::warning(
                "'using { f, g } for Type' is supported; all library functions are \
                 merged (selective filtering is not enforced at compile time)"
            )
            .with_suggestion(
                "this is functionally correct — all library functions are available; \
                 unused functions are excluded during bytecode optimization"
            ),
        );
    }
}

fn validate_type_definitions(_metadata: &ContractMetadata, _diagnostics: &mut Vec<Diagnostic>) {
    // User-defined value types (`type X is Y`) are now supported.
    // They are treated as transparent type aliases; `wrap`/`unwrap` compile to no-ops.
}

fn validate_abstract_contract(metadata: &ContractMetadata, diagnostics: &mut Vec<Diagnostic>) {
    let unimplemented: Vec<&str> = metadata
        .methods
        .iter()
        .filter(|m| {
            matches!(m.kind, FunctionKind::Regular)
                && m.body.is_none()
        })
        .map(|m| m.name.as_str())
        .collect();

    if unimplemented.is_empty() {
        return;
    }

    if metadata.is_abstract {
        // Abstract contracts: informational warning listing unimplemented methods.
        // This helps developers track what still needs implementation in derived contracts.
        diagnostics.push(
            Diagnostic::warning(format!(
                "abstract contract '{}' has {} unimplemented function(s): [{}]",
                metadata.name,
                unimplemented.len(),
                unimplemented.join(", "),
            ))
            .with_suggestion(
                "derived contracts must implement these functions or also be declared abstract"
            ),
        );
    } else {
        // Non-abstract contracts must implement all declared functions.
        diagnostics.push(
            Diagnostic::error(format!(
                "contract '{}' has {} unimplemented function(s) [{}] but is not declared abstract; \
                 either provide implementations or declare the contract as 'abstract contract {}'",
                metadata.name,
                unimplemented.len(),
                unimplemented.join(", "),
                metadata.name,
            ))
            .with_suggestion(
                "add 'abstract' before 'contract', or provide function bodies for all declared functions"
            ),
        );
    }
}

fn validate_flatten_warnings(metadata: &ContractMetadata, diagnostics: &mut Vec<Diagnostic>) {
    for warning in &metadata.flatten_warnings {
        diagnostics.push(
            Diagnostic::warning(warning.clone())
                .with_code("W200")
                .with_suggestion(
                    "mark the base function 'virtual' and the overriding function 'override'",
                ),
        );
    }
}
