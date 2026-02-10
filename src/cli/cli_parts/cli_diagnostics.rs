fn emit_warning(message: &str, contract: Option<&str>, json: bool, code: Option<&str>) {
    emit_warning_with_suggestion(message, contract, json, code, None);
}

fn emit_warning_with_suggestion(
    message: &str,
    contract: Option<&str>,
    json: bool,
    code: Option<&str>,
    suggestion: Option<&str>,
) {
    use std::io::{self, Write};

    if json {
        let warning = json!({
            "component": "neo-solidity",
            "severity": "warning",
            "type": "CompilerWarning",
            "code": code.unwrap_or("COMPILER_WARNING"),
            "contract": contract,
            "message": message,
            "formattedMessage": message,
            "suggestion": suggestion,
        });
        let mut stderr = io::stderr().lock();
        let _ = stderr.write_all(warning.to_string().as_bytes());
        let _ = stderr.write_all(b"\n");
        return;
    }

    let code_tag = code.map(|c| format!("[{c}]")).unwrap_or_default();
    let line = if let Some(contract) = contract {
        format!("warning{code_tag} ({contract}): {message}\n")
    } else {
        format!("warning{code_tag}: {message}\n")
    };

    let mut stderr = io::stderr().lock();
    let _ = stderr.write_all(line.as_bytes());
    if let Some(hint) = suggestion {
        let _ = stderr.write_all(format!("  help: {hint}\n").as_bytes());
    }
}

fn emit_error(message: &str, code: &str, json: bool) {
    use std::io::{self, Write};

    if json {
        let error = json!({
            "component": "neo-solidity",
            "severity": "error",
            "type": "CompilerError",
            "code": code,
            "message": message,
            "formattedMessage": message,
        });
        let mut stderr = io::stderr().lock();
        let _ = stderr.write_all(error.to_string().as_bytes());
        let _ = stderr.write_all(b"\n");
    } else {
        let mut stderr = io::stderr().lock();
        let _ = stderr.write_all(format!("error: {message}\n").as_bytes());
    }
}
