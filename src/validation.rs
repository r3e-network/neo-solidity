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

    pub fn errors(&self) -> &[CompilerError] {
        &self.errors
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
