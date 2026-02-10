fn validate_return_types(metadata: &ContractMetadata, diagnostics: &mut Vec<Diagnostic>) {
    for function in &metadata.methods {
        let is_exposed = matches!(
            function.visibility,
            VisibilityKind::Public | VisibilityKind::External
        );

        if let Some(ret_param) = function.return_parameters.first() {
            if ret_param.neo_type.is_none() && is_exposed {
                let lower_ty = ret_param.ty.to_ascii_lowercase();
                if lower_ty.starts_with("fixed") || lower_ty.starts_with("ufixed") {
                    diagnostics.push(
                        Diagnostic::error(format!(
                            "function '{}' return type '{}' is not supported on NeoVM",
                            function.name, ret_param.ty
                        ))
                        .with_suggestion(
                            "use scaled integer arithmetic instead (e.g., multiply by 10^18 for 18 decimal places)"
                        ),
                    );
                } else {
                    diagnostics.push(Diagnostic::error(format!(
                        "function '{}' return type '{}' is unsupported",
                        function.name, ret_param.ty
                    )));
                }
            }

            let lowered = ret_param.ty.to_ascii_lowercase();
            let supported = match ret_param.neo_type.as_ref() {
                Some(NeoType::Any) | None => {
                    lowered.starts_with("uint")
                        || lowered.starts_with("int")
                        || lowered == "bool"
                        || lowered == "string"
                        || lowered == "address"
                        || lowered == "bytes"
                        || lowered == "bytearray"
                        || lowered.starts_with("bytes")
                        || lowered.ends_with("[]")
                        || lowered.starts_with("mapping")
                        || lowered.starts_with("syscalls.")
                        || lowered.starts_with("storage.iterator")
                }
                Some(_) => true,
            };

            if !supported {
                diagnostics.push(Diagnostic::warning(format!(
                    "function '{}' returns '{}', which may not map cleanly to Neo manifest types",
                    function.name, ret_param.ty
                )));
            }
        }

        for ret_param in &function.return_parameters {
            if let Some(storage) = &ret_param.storage {
                if storage == "storage" {
                    diagnostics.push(Diagnostic::warning(format!(
                        "function '{}' return value '{}' uses 'storage' data location (treated as Any)",
                        function.name, ret_param.ty
                    )));
                }
            }
        }
    }
}
