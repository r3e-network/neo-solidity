//! Warning System for Neo Solidity Compiler
//!
//! Provides configurable warnings for code quality and security issues.

use crate::error::{ErrorCode, ErrorSeverity, SourceLocation, FixSuggestion};
use std::collections::HashSet;

/// Warning configuration
#[derive(Debug, Clone)]
pub struct WarningConfig {
    /// Disabled warning codes
    pub disabled: HashSet<ErrorCode>,
    /// Warnings treated as errors
    pub errors: HashSet<ErrorCode>,
    /// Enable all warnings
    pub all: bool,
    /// Enable pedantic warnings
    pub pedantic: bool,
}

impl Default for WarningConfig {
    fn default() -> Self {
        Self {
            disabled: HashSet::new(),
            errors: HashSet::new(),
            all: false,
            pedantic: false,
        }
    }
}

impl WarningConfig {
    /// Create config with all warnings enabled
    pub fn all() -> Self {
        Self { all: true, ..Default::default() }
    }

    /// Disable a specific warning
    pub fn disable(mut self, code: ErrorCode) -> Self {
        self.disabled.insert(code);
        self
    }

    /// Treat warning as error
    pub fn as_error(mut self, code: ErrorCode) -> Self {
        self.errors.insert(code);
        self
    }

    /// Check if warning is enabled
    pub fn is_enabled(&self, code: ErrorCode) -> bool {
        !self.disabled.contains(&code)
    }

    /// Get severity for a warning code
    pub fn severity(&self, code: ErrorCode) -> ErrorSeverity {
        if self.errors.contains(&code) {
            ErrorSeverity::Error
        } else {
            ErrorSeverity::Warning
        }
    }
}

/// A collected warning
#[derive(Debug, Clone)]
pub struct Warning {
    pub code: ErrorCode,
    pub message: String,
    pub location: SourceLocation,
    pub suggestion: Option<FixSuggestion>,
}

impl Warning {
    pub fn new(code: ErrorCode, message: impl Into<String>, location: SourceLocation) -> Self {
        Self {
            code,
            message: message.into(),
            location,
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: FixSuggestion) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
}

/// Warning collector
#[derive(Debug, Default)]
pub struct WarningCollector {
    warnings: Vec<Warning>,
    config: WarningConfig,
}

impl WarningCollector {
    pub fn new(config: WarningConfig) -> Self {
        Self { warnings: Vec::new(), config }
    }

    pub fn add(&mut self, warning: Warning) {
        if self.config.is_enabled(warning.code) {
            self.warnings.push(warning);
        }
    }

    pub fn warnings(&self) -> &[Warning] {
        &self.warnings
    }

    pub fn has_errors(&self) -> bool {
        self.warnings.iter().any(|w| self.config.errors.contains(&w.code))
    }

    pub fn count(&self) -> usize {
        self.warnings.len()
    }

    pub fn clear(&mut self) {
        self.warnings.clear();
    }
}
