impl CompileError {
    pub(crate) fn into_errors(self, file: &str) -> Vec<Value> {
        match self {
            CompileError::Diagnostics(diags) | CompileError::Semantic(diags) => diags
                .into_iter()
                .map(|diag| standard_json::diagnostic_to_standard_error(&diag, file))
                .collect(),
            CompileError::Ir(errors) => errors
                .into_iter()
                .map(|diag| {
                    let formatted = diag.display();
                    let code = diag
                        .code
                        .unwrap_or_else(|| "IR_GENERATION_ERROR".to_string());
                    let mut obj = json!({
                        "component": "neo-devpack-solidity",
                        "severity": "error",
                        "type": "IrGeneration",
                        "code": code,
                        "sourceLocation": { "file": file },
                        "formattedMessage": formatted,
                        "message": diag.message,
                        "functionName": diag.function_name,
                    });
                    if let Some(suggestion) = &diag.suggestion {
                        obj["suggestion"] = json!(suggestion);
                    }
                    obj
                })
                .collect(),
            CompileError::Manifest(message) => vec![json!({
                "component": "neo-devpack-solidity",
                "severity": "error",
                "type": "ManifestGeneration",
                "code": "MANIFEST_GENERATION_ERROR",
                "sourceLocation": { "file": file },
                "formattedMessage": message,
                "message": message,
            })],
            CompileError::ParseErrors(diags) => diags
                .into_iter()
                .map(|diag| {
                    json!({
                        "component": "neo-devpack-solidity",
                        "severity": "error",
                        "type": "ParseError",
                        "code": "PARSE_ERROR",
                        "sourceLocation": {
                            "file": file,
                            "start": diag.start,
                            "end": diag.end,
                        },
                        "formattedMessage": diag.message,
                        "message": diag.message,
                    })
                })
                .collect(),
            CompileError::Message(message) => vec![json!({
                "component": "neo-devpack-solidity",
                "severity": "error",
                "type": "Generic",
                "code": "GENERIC_ERROR",
                "sourceLocation": { "file": file },
                "formattedMessage": message,
                "message": message,
            })],
        }
    }
}
