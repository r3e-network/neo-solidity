fn validate_return_types(metadata: &ContractMetadata, diagnostics: &mut Vec<Diagnostic>) {
    for function in &metadata.methods {
        if function.return_parameters.len() > 1 {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                message: format!(
                    "function '{}' returns multiple values; Neo ABI will expose this as an Array return",
                    function.name
                ),
            });
        }

        if let Some(ret_param) = function.return_parameters.first() {
            if ret_param.neo_type.is_none() {
                diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    message: format!(
                        "function '{}' return type '{}' is unsupported",
                        function.name, ret_param.ty
                    ),
                });
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
                }
                Some(_) => true,
            };

            if !supported {
                diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Warning,
                    message: format!(
                        "function '{}' returns '{}', which may not map cleanly to Neo manifest types",
                        function.name, ret_param.ty
                    ),
                });
            }
        }

        for ret_param in &function.return_parameters {
            if let Some(storage) = &ret_param.storage {
                if storage == "storage" {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Warning,
                        message: format!(
                            "function '{}' return value '{}' uses 'storage' data location (treated as Any)",
                            function.name, ret_param.ty
                        ),
                    });
                }
            }
        }
    }
}
