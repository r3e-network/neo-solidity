/// A single structured parser diagnostic, carrying the byte range so
/// standard-JSON output can emit a precise `sourceLocation` (start/end) per
/// error instead of collapsing every parse error into one opaque blob.
#[derive(Debug, Clone)]
pub struct ParseDiagnostic {
    /// Human-readable message, prefixed with `line:column:` where known.
    pub message: String,
    /// Inclusive start byte offset in the source.
    pub start: usize,
    /// Exclusive end byte offset in the source.
    pub end: usize,
}

/// Errors emitted by the frontend while parsing Solidity code.
#[derive(Debug, Error)]
pub enum FrontendError {
    /// Parsing failed; the contained message aggregates all diagnostics.
    #[error("Solidity parsing failed:\n{0}")]
    Parse(String),

    /// Parsing failed; structured per-diagnostic form (preferred). Preserves
    /// each diagnostic's byte range for precise tooling output.
    #[error("Solidity parsing failed with {} error(s)", .0.len())]
    ParseDiagnostics(Vec<ParseDiagnostic>),

    /// Invalid Solidity version pragma
    #[error("Unsupported Solidity version: {0}")]
    UnsupportedVersion(String),

    /// Import resolution failed
    #[error("Failed to resolve import '{path}': {reason}")]
    ImportError { path: String, reason: String },

    /// Contract not found in source
    #[error("Contract '{0}' not found in source")]
    ContractNotFound(String),
}

impl FrontendError {
    /// Create a parse error with location info
    pub fn parse_at(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::Parse(format!("{}:{}: {}", line, column, message.into()))
    }

    /// Create an import error
    pub fn import_error(path: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ImportError {
            path: path.into(),
            reason: reason.into(),
        }
    }

    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        matches!(self, Self::UnsupportedVersion(_))
    }
}

