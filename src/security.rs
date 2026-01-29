//! Security Analysis Module
//!
//! Detects potential security vulnerabilities in Solidity code.

use crate::error::{ErrorCode, SourceLocation, FixSuggestion};

/// Security issue severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecuritySeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// A detected security issue
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub severity: SecuritySeverity,
    pub code: ErrorCode,
    pub message: String,
    pub location: SourceLocation,
    pub suggestion: Option<FixSuggestion>,
}

impl SecurityIssue {
    pub fn critical(code: ErrorCode, msg: impl Into<String>, loc: SourceLocation) -> Self {
        Self {
            severity: SecuritySeverity::Critical,
            code,
            message: msg.into(),
            location: loc,
            suggestion: None,
        }
    }

    pub fn high(code: ErrorCode, msg: impl Into<String>, loc: SourceLocation) -> Self {
        Self {
            severity: SecuritySeverity::High,
            code,
            message: msg.into(),
            location: loc,
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, s: FixSuggestion) -> Self {
        self.suggestion = Some(s);
        self
    }
}

/// Security checker
#[derive(Default)]
pub struct SecurityChecker {
    issues: Vec<SecurityIssue>,
}

impl SecurityChecker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_issue(&mut self, issue: SecurityIssue) {
        self.issues.push(issue);
    }

    pub fn issues(&self) -> &[SecurityIssue] {
        &self.issues
    }

    pub fn has_critical(&self) -> bool {
        self.issues.iter().any(|i| i.severity == SecuritySeverity::Critical)
    }

    pub fn has_high(&self) -> bool {
        self.issues.iter().any(|i| i.severity == SecuritySeverity::High)
    }

    pub fn count_by_severity(&self, sev: SecuritySeverity) -> usize {
        self.issues.iter().filter(|i| i.severity == sev).count()
    }

    pub fn clear(&mut self) {
        self.issues.clear();
    }
}
