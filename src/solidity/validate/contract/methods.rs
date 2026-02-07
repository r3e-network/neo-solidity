fn validate_methods(metadata: &ContractMetadata, diagnostics: &mut Vec<Diagnostic>) -> usize {
    use std::collections::{HashMap, HashSet};

    fn is_intrinsic_library(name: &str) -> bool {
        matches!(name, "Runtime" | "Storage" | "Syscalls" | "NativeCalls" | "Neo" | "abi")
    }

    let mut signatures = HashSet::new();
    let mut overload_counts: HashSet<(String, usize)> = HashSet::new();
    let mut overload_has_exposed: HashMap<(String, usize), bool> = HashMap::new();
    let mut constructor_count = 0usize;

    // Used to reduce false-positive diagnostics for `return foo();` in
    // multi-return functions. NeoVM lowering represents tuples as arrays, so
    // returning another tuple-returning function call is valid.
    let mut return_arities: HashMap<(String, usize), usize> = HashMap::new();
    for function in &metadata.methods {
        return_arities.insert(
            (function.name.clone(), function.parameters.len()),
            function.return_parameters.len(),
        );
    }

    for function in &metadata.methods {
        match function.kind {
            FunctionKind::Constructor => {
                constructor_count += 1;
                if !function.return_parameters.is_empty() {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        message: "constructor must not specify a return type".to_string(),
                    });
                }
            }
            FunctionKind::Regular => {
                let count_key = (function.name.clone(), function.parameters.len());
                let is_exposed = matches!(
                    function.visibility,
                    VisibilityKind::Public | VisibilityKind::External
                );
                let has_exposed = overload_has_exposed
                    .get(&count_key)
                    .copied()
                    .unwrap_or(false);

                let intrinsic_internal_overload = is_intrinsic_library(&metadata.name)
                    && !is_exposed
                    && !has_exposed;

                if !overload_counts.insert(count_key.clone()) && !intrinsic_internal_overload {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        message: format!(
                            "overloaded function '{}' with {} parameter(s) is not supported; \
                             Neo ABI dispatches by name and argument count only, so overloads \
                             that differ only in parameter types cannot be distinguished at runtime",
                            count_key.0, count_key.1
                        ),
                    });
                }

                overload_has_exposed.insert(count_key.clone(), has_exposed || is_exposed);

                let param_signature: Vec<String> = function
                    .parameters
                    .iter()
                    .map(|param| canonical_param_type(&param.ty))
                    .collect();
                let signature = format!("{}({})", function.name, param_signature.join(","));

                if !signatures.insert(signature.clone()) {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        message: format!("duplicate function signature '{}'", signature),
                    });
                }
            }
        }

        let mut params = HashSet::new();
        for param in &function.parameters {
            if let Some(name) = &param.name {
                if !params.insert(name.clone()) {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        message: format!(
                            "function '{}' has duplicate parameter name '{}'",
                            function.name, name
                        ),
                    });
                }
            }

            if param.neo_type.is_none() {
                diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    message: format!(
                        "function '{}' parameter '{}' uses unsupported type '{}'",
                        function.name,
                        param
                            .name
                            .clone()
                            .unwrap_or_else(|| "<unnamed>".to_string()),
                        param.ty
                    ),
                });
            }

            // Validate mapping key types: arrays, structs, and mappings are
            // not valid as mapping keys because they lack a stable hash on NeoVM.
            if let Some(NeoType::Mapping { ref key, .. }) = param.neo_type {
                fn is_invalid_mapping_key(ty: &NeoType) -> bool {
                    matches!(
                        ty,
                        NeoType::Array(_)
                            | NeoType::Struct { .. }
                            | NeoType::Mapping { .. }
                    )
                }
                if is_invalid_mapping_key(key) {
                    let param_name = param
                        .name
                        .clone()
                        .unwrap_or_else(|| "<unnamed>".to_string());
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        message: format!(
                            "function '{}' parameter '{}': mapping key type must be \
                             an elementary type (integer, bool, address, string, bytes); \
                             arrays, structs, and mappings are not allowed as keys",
                            function.name, param_name
                        ),
                    });
                }
            }

            if let Some(storage) = &param.storage {
                if storage == "storage"
                    && matches!(
                        function.visibility,
                        VisibilityKind::External | VisibilityKind::Public
                    )
                {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        message: format!(
                            "public/external function '{}' parameter '{}' may not use 'storage' data location",
                            function.name,
                            param
                                .name
                                .clone()
                                .unwrap_or_else(|| "<unnamed>".to_string())
                        ),
                    });
                }
            }
        }

        if let Some(body) = &function.body {
            check_return_statements(
                body,
                function.return_parameters.len(),
                &function.name,
                &return_arities,
                diagnostics,
            );
        } else if !function.return_parameters.is_empty()
            && !matches!(function.kind, FunctionKind::Constructor)
        {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Error,
                message: format!(
                    "function '{}' declares a return type but has no implementation; \
                     provide a body or mark the contract as abstract",
                    function.name
                ),
            });
        }
    }

    constructor_count
}
