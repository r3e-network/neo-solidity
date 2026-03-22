//! Input Validation Module
//!
//! Validates compiler inputs and configurations.

use crate::error::CompilerError;

/// Validation result
pub type ValidationResult<T> = Result<T, Vec<CompilerError>>;

/// Input validator
#[derive(Default)]
pub struct InputValidator {
    errors: Vec<CompilerError>,
}

impl InputValidator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate_source(&mut self, source: &str) -> bool {
        if source.is_empty() {
            self.errors
                .push(CompilerError::ParseError("Empty source file".to_string()));
            return false;
        }
        if source.len() > 10_000_000 {
            self.errors.push(CompilerError::ParseError(
                "Source file too large (>10MB)".to_string(),
            ));
            return false;
        }
        true
    }

    /// Validate that raw bytes are valid UTF-8 before use as source.
    pub fn validate_utf8<'a>(&mut self, raw: &'a [u8]) -> Option<&'a str> {
        match std::str::from_utf8(raw) {
            Ok(s) => Some(s),
            Err(e) => {
                self.errors.push(CompilerError::ParseError(format!(
                    "Source is not valid UTF-8: {e}"
                )));
                None
            }
        }
    }

    /// Validate an import path, rejecting path traversal attempts.
    pub fn validate_import_path(&mut self, path: &str) -> bool {
        if path.contains("..") {
            self.errors.push(CompilerError::ParseError(format!(
                "Import path contains disallowed '..': '{path}'"
            )));
            return false;
        }
        true
    }

    pub fn errors(&self) -> &[CompilerError] {
        &self.errors
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
