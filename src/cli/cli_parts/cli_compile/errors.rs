impl CompileError {
    fn into_errors(self, file: &str) -> Vec<Value> {
        match self {
            CompileError::Diagnostics(diags) | CompileError::Semantic(diags) => diags
                .into_iter()
                .map(|diag| standard_json::diagnostic_to_standard_error(&diag, file))
                .collect(),
            CompileError::Ir(errors) => errors
                .into_iter()
                .map(|diag| {
                    let formatted = diag.display();
                    let mut obj = json!({
                        "component": "neo-solidity",
                        "severity": "error",
                        "type": "IrGeneration",
                        "sourceLocation": { "file": file },
                        "formattedMessage": formatted,
                        "message": diag.message,
                        "functionName": diag.function_name,
                    });
                    if let Some(suggestion) = &diag.suggestion {
                        obj["suggestion"] = json!(suggestion);
                    }
                    if let Some(code) = &diag.code {
                        obj["errorCode"] = json!(code);
                    }
                    obj
                })
                .collect(),
            CompileError::Manifest(message) => vec![json!({
                "component": "neo-solidity",
                "severity": "error",
                "type": "ManifestGeneration",
                "sourceLocation": { "file": file },
                "formattedMessage": message,
                "message": message,
            })],
            CompileError::Message(message) => vec![json!({
                "component": "neo-solidity",
                "severity": "error",
                "type": "Generic",
                "sourceLocation": { "file": file },
                "formattedMessage": message,
                "message": message,
            })],
        }
    }
}
