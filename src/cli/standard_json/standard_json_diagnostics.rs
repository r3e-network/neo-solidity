pub(crate) fn diagnostic_to_standard_error(
    diagnostic: &neo_devpack_solidity::solidity::Diagnostic,
    file: &str,
) -> Value {
    let severity = match diagnostic.severity {
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Error => "error",
    };
    let code = diagnostic
        .code
        .as_deref()
        .unwrap_or_else(|| infer_validation_code(&diagnostic.message, diagnostic.severity));

    let mut value = json!({
        "component": "neo-devpack-solidity",
        "severity": severity,
        "type": "Validation",
        "code": code,
        "sourceLocation": { "file": file },
        "formattedMessage": diagnostic.message,
        "message": diagnostic.message,
    });

    if let Some(suggestion) = diagnostic.suggestion.as_deref() {
        value["suggestion"] = json!(suggestion);
    }

    value
}

pub(crate) fn infer_validation_code(message: &str, severity: DiagnosticSeverity) -> &'static str {
    let msg = message.to_ascii_lowercase();

    if msg.contains("full wildcard manifest permissions") {
        "MANIFEST_FULL_WILDCARD"
    } else if msg.contains("wildcard contract manifest permissions") {
        "MANIFEST_WILDCARD_CONTRACT"
    } else if msg.contains("wildcard method manifest permissions") {
        "MANIFEST_WILDCARD_METHODS"
    } else if msg.contains("duplicate function signature") {
        "DUPLICATE_SIGNATURE"
    } else if msg.contains("duplicate parameter name") {
        "DUPLICATE_PARAMETER_NAME"
    } else if msg.contains("unsupported type") && msg.contains("parameter") {
        "UNSUPPORTED_PARAMETER_TYPE"
    } else if msg.contains("unsupported type") && msg.contains("state variable") {
        "UNSUPPORTED_STATE_TYPE"
    } else if msg.contains("unsupported type") && msg.contains("return type") {
        "UNSUPPORTED_RETURN_TYPE"
    } else if msg.contains("unsupported type") {
        "UNSUPPORTED_TYPE"
    } else if msg.contains("constructor must not specify a return type") {
        "INVALID_CONSTRUCTOR_RETURN"
    } else if msg.contains("multiple constructors defined") {
        "MULTIPLE_CONSTRUCTORS"
    } else if msg.contains("state variable declared without a name") {
        "STATE_VARIABLE_NAME_MISSING"
    } else if msg.contains("duplicate state variable") {
        "DUPLICATE_STATE_VARIABLE"
    } else if msg.contains("constant state variable") && msg.contains("must have an initializer") {
        "CONSTANT_MISSING_INITIALIZER"
    } else if msg.contains("may not use 'storage'") {
        "INVALID_STORAGE_PARAM"
    } else if msg.contains("return value") && msg.contains("storage") {
        "INVALID_STORAGE_RETURN"
    } else if msg.contains("returns multiple values") {
        "MULTIPLE_RETURN_VALUES_UNSUPPORTED"
    } else if msg.contains("expected") && msg.contains("return") && msg.contains("values") {
        "RETURN_MISMATCH"
    } else if msg.contains("returns") && msg.contains("may not map cleanly") {
        "RETURN_TYPE_UNMAPPED"
    } else if msg.contains("return type") && msg.contains("unsupported") {
        "UNSUPPORTED_RETURN_TYPE"
    } else if msg.contains("event") && msg.contains("exceeds neo abi limits") {
        "EVENT_PARAM_LIMIT"
    } else if msg.contains("declares a return type but has no implementation") {
        "MISSING_IMPLEMENTATION_RETURN"
    } else {
        match severity {
            DiagnosticSeverity::Warning => "VALIDATION_WARNING",
            DiagnosticSeverity::Error => "VALIDATION_ERROR",
        }
    }
}
