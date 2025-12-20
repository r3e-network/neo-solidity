impl CompileError {
    fn into_errors(self, file: &str) -> Vec<Value> {
        match self {
            CompileError::Diagnostics(diags) | CompileError::Semantic(diags) => diags
                .into_iter()
                .map(|diag| standard_json::diagnostic_to_standard_error(&diag, file))
                .collect(),
            CompileError::Ir(errors) => errors
                .into_iter()
                .map(|message| {
                    json!({
                        "component": "neo-solidity",
                        "severity": "error",
                        "type": "IrGeneration",
                        "sourceLocation": { "file": file },
                        "formattedMessage": message,
                        "message": message,
                    })
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
