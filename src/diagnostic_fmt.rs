//! Diagnostic Formatter
//!
//! Pretty-prints compiler diagnostics.

use crate::error::CompilerError;

/// Diagnostic output format
#[derive(Debug, Clone, Copy)]
pub enum DiagnosticFormat {
    Human,
    Json,
    Compact,
}

/// Format a compiler error for display
pub fn format_error(err: &CompilerError, fmt: DiagnosticFormat) -> String {
    match fmt {
        DiagnosticFormat::Human => format_human(err),
        DiagnosticFormat::Json => format_json(err),
        DiagnosticFormat::Compact => format_compact(err),
    }
}

fn format_human(err: &CompilerError) -> String {
    format!("{}", err)
}

fn format_json(err: &CompilerError) -> String {
    format!("{{\"error\": \"{}\"}}", err)
}

fn format_compact(err: &CompilerError) -> String {
    format!("{}", err)
}
