fn compile_input_or_exit(
    input_content: &str,
    verbose: bool,
    options: CompileOptions,
    json_errors: bool,
    json_warnings: bool,
) -> Vec<CompilationArtifacts> {
    match compile_contracts_with_options(input_content, verbose, options) {
        Ok(list) => list,
        Err(CompileError::Diagnostics(diags)) | Err(CompileError::Semantic(diags)) => {
            for diag in diags {
                let code = diag.code.as_deref().unwrap_or_else(|| {
                    standard_json::infer_validation_code(&diag.message, diag.severity)
                });
                match diag.severity {
                    DiagnosticSeverity::Warning => emit_warning_with_suggestion(
                        &diag.message,
                        None,
                        json_warnings,
                        Some(code),
                        diag.suggestion.as_deref(),
                    ),
                    DiagnosticSeverity::Error => emit_error_with_suggestion(
                        &diag.message,
                        code,
                        json_errors,
                        diag.suggestion.as_deref(),
                    ),
                }
            }
            std::process::exit(1);
        }
        Err(CompileError::Ir(errors)) => {
            for error in errors {
                let message = format!("function '{}': {}", error.function_name, error.message);
                let code = error.code.as_deref().unwrap_or("IR_GENERATION_ERROR");
                emit_error_with_suggestion(
                    &message,
                    code,
                    json_errors,
                    error.suggestion.as_deref(),
                );
            }
            std::process::exit(1);
        }
        Err(CompileError::Manifest(message)) => {
            emit_error(&message, "MANIFEST_GENERATION_ERROR", json_errors);
            std::process::exit(1);
        }
        Err(CompileError::Message(message)) => {
            emit_error(&message, "GENERIC_ERROR", json_errors);
            std::process::exit(1);
        }
    }
}
