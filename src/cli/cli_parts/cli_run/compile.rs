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
                match diag.severity {
                    DiagnosticSeverity::Warning => emit_warning(
                        &diag.message,
                        None,
                        json_warnings,
                        Some(standard_json::infer_validation_code(
                            &diag.message,
                            diag.severity,
                        )),
                    ),
                    DiagnosticSeverity::Error => emit_error(
                        &diag.message,
                        standard_json::infer_validation_code(&diag.message, diag.severity),
                        json_errors,
                    ),
                }
            }
            std::process::exit(1);
        }
        Err(CompileError::Ir(errors)) => {
            for error in errors {
                emit_error(&error.display(), "IR_GENERATION_ERROR", json_errors);
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
