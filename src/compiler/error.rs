//! Error Handling Module
//! 
//! Comprehensive error handling for the Neo Solidity compiler with detailed
//! error messages, recovery strategies, and diagnostic information.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

/// Comprehensive compiler error types
#[derive(Debug, Error)]
pub enum CompilerError {
    // Lexical errors
    #[error("Lexical error at {location}: {message}")]
    LexicalError {
        message: String,
        location: ErrorLocation,
        suggestion: Option<String>,
    },

    // Parsing errors  
    #[error("Parse error at {location}: {message}")]
    ParseError {
        message: String,
        location: ErrorLocation,
        expected: Option<String>,
        found: Option<String>,
        suggestion: Option<String>,
    },

    // Semantic errors
    #[error("Semantic error at {location}: {message}")]
    SemanticError {
        message: String,
        location: ErrorLocation,
        error_code: String,
        severity: ErrorSeverity,
        related_locations: Vec<ErrorLocation>,
        suggestion: Option<String>,
    },

    // Type errors
    #[error("Type error at {location}: {message}")]
    TypeError {
        message: String,
        location: ErrorLocation,
        expected_type: Option<String>,
        actual_type: Option<String>,
        suggestion: Option<String>,
    },

    // Code generation errors
    #[error("Code generation error at {location}: {message}")]
    CodeGenError {
        message: String,
        location: Option<ErrorLocation>,
        instruction: Option<String>,
        suggestion: Option<String>,
    },

    // Optimization errors
    #[error("Optimization error in pass '{pass}': {message}")]
    OptimizationError {
        message: String,
        pass: String,
        location: Option<ErrorLocation>,
    },

    // Runtime errors
    #[error("Runtime error: {message}")]
    RuntimeError {
        message: String,
        stack_trace: Vec<String>,
        location: Option<ErrorLocation>,
    },

    // IO errors
    #[error("IO error: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
        context: Option<String>,
    },

    // Configuration errors
    #[error("Configuration error: {message}")]
    ConfigError {
        message: String,
        field: Option<String>,
        suggestion: Option<String>,
    },

    // Internal errors
    #[error("Internal compiler error: {message}")]
    InternalError {
        message: String,
        location: Option<ErrorLocation>,
        debug_info: HashMap<String, String>,
    },

    // Multiple errors
    #[error("Multiple errors occurred ({count} total)")]
    MultipleErrors {
        errors: Vec<CompilerError>,
        count: usize,
    },
}

/// Error location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub length: Option<u32>,
    pub offset: Option<usize>,
    pub source_line: Option<String>,
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Error context for recovery and reporting
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub source_files: HashMap<String, String>,
    pub compilation_phase: CompilationPhase,
    pub error_recovery_enabled: bool,
    pub max_errors: usize,
    pub collected_errors: Vec<CompilerError>,
}

/// Compilation phases for error context
#[derive(Debug, Clone)]
pub enum CompilationPhase {
    Lexing,
    Parsing,
    SemanticAnalysis,
    Optimization,
    CodeGeneration,
    Linking,
}

/// Error recovery strategies
#[derive(Debug)]
pub enum RecoveryStrategy {
    Skip,
    Insert(String),
    Replace(String),
    Synchronize,
    Abort,
}

/// Error reporter for formatted output
#[derive(Debug)]
pub struct ErrorReporter {
    pub colored_output: bool,
    pub show_suggestions: bool,
    pub show_source_context: bool,
    pub max_context_lines: usize,
}

/// Diagnostic information for IDE integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub code: Option<String>,
    pub message: String,
    pub source: String,
    pub range: DiagnosticRange,
    pub related_information: Vec<DiagnosticRelatedInformation>,
    pub tags: Vec<DiagnosticTag>,
    pub code_description: Option<CodeDescription>,
    pub data: Option<serde_json::Value>,
}

/// Diagnostic severity (LSP-compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// Diagnostic range (LSP-compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticRange {
    pub start: Position,
    pub end: Position,
}

/// Position in source file (LSP-compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

/// Related diagnostic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticRelatedInformation {
    pub location: Location,
    pub message: String,
}

/// File location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: DiagnosticRange,
}

/// Diagnostic tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticTag {
    Unnecessary = 1,
    Deprecated = 2,
}

/// Code description for diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeDescription {
    pub href: String,
}

impl ErrorContext {
    /// Create new error context
    pub fn new(phase: CompilationPhase) -> Self {
        Self {
            source_files: HashMap::new(),
            compilation_phase: phase,
            error_recovery_enabled: true,
            max_errors: 100,
            collected_errors: Vec::new(),
        }
    }

    /// Add source file to context
    pub fn add_source_file(&mut self, filename: String, content: String) {
        self.source_files.insert(filename, content);
    }

    /// Collect error with recovery
    pub fn collect_error(&mut self, error: CompilerError) -> RecoveryStrategy {
        self.collected_errors.push(error);
        
        if self.collected_errors.len() >= self.max_errors {
            return RecoveryStrategy::Abort;
        }

        if self.error_recovery_enabled {
            match &self.compilation_phase {
                CompilationPhase::Lexing => RecoveryStrategy::Skip,
                CompilationPhase::Parsing => RecoveryStrategy::Synchronize,
                CompilationPhase::SemanticAnalysis => RecoveryStrategy::Skip,
                CompilationPhase::Optimization => RecoveryStrategy::Skip,
                CompilationPhase::CodeGeneration => RecoveryStrategy::Abort,
                CompilationPhase::Linking => RecoveryStrategy::Abort,
            }
        } else {
            RecoveryStrategy::Abort
        }
    }

    /// Check if there are fatal errors
    pub fn has_fatal_errors(&self) -> bool {
        self.collected_errors.iter().any(|err| {
            matches!(err, 
                CompilerError::InternalError { .. } |
                CompilerError::IoError { .. } |
                CompilerError::ConfigError { .. }
            )
        })
    }

    /// Get error count by severity
    pub fn error_count_by_severity(&self) -> HashMap<ErrorSeverity, usize> {
        let mut counts = HashMap::new();
        
        for error in &self.collected_errors {
            let severity = self.get_error_severity(error);
            *counts.entry(severity).or_insert(0) += 1;
        }
        
        counts
    }

    /// Get error severity
    fn get_error_severity(&self, error: &CompilerError) -> ErrorSeverity {
        match error {
            CompilerError::SemanticError { severity, .. } => severity.clone(),
            CompilerError::InternalError { .. } => ErrorSeverity::Error,
            CompilerError::IoError { .. } => ErrorSeverity::Error,
            CompilerError::ConfigError { .. } => ErrorSeverity::Error,
            CompilerError::RuntimeError { .. } => ErrorSeverity::Error,
            CompilerError::OptimizationError { .. } => ErrorSeverity::Warning,
            _ => ErrorSeverity::Error,
        }
    }
}

impl ErrorReporter {
    /// Create new error reporter
    pub fn new() -> Self {
        Self {
            colored_output: true,
            show_suggestions: true,
            show_source_context: true,
            max_context_lines: 3,
        }
    }

    /// Format error for display
    pub fn format_error(&self, error: &CompilerError, context: &ErrorContext) -> String {
        let mut output = String::new();
        
        match error {
            CompilerError::LexicalError { message, location, suggestion } => {
                output.push_str(&self.format_error_header("Lexical Error", message));
                output.push_str(&self.format_location(location, context));
                if let Some(sugg) = suggestion {
                    output.push_str(&self.format_suggestion(sugg));
                }
            },
            CompilerError::ParseError { message, location, expected, found, suggestion } => {
                output.push_str(&self.format_error_header("Parse Error", message));
                output.push_str(&self.format_location(location, context));
                if let (Some(exp), Some(fnd)) = (expected, found) {
                    output.push_str(&format!("  Expected: {}\n", exp));
                    output.push_str(&format!("  Found: {}\n", fnd));
                }
                if let Some(sugg) = suggestion {
                    output.push_str(&self.format_suggestion(sugg));
                }
            },
            CompilerError::SemanticError { message, location, error_code, severity, related_locations, suggestion } => {
                let severity_str = match severity {
                    ErrorSeverity::Error => "Error",
                    ErrorSeverity::Warning => "Warning", 
                    ErrorSeverity::Info => "Info",
                    ErrorSeverity::Hint => "Hint",
                };
                output.push_str(&self.format_error_header(
                    &format!("Semantic {} [{}]", severity_str, error_code), 
                    message
                ));
                output.push_str(&self.format_location(location, context));
                for related in related_locations {
                    output.push_str(&format!("  Related: {}\n", self.format_location_brief(related)));
                }
                if let Some(sugg) = suggestion {
                    output.push_str(&self.format_suggestion(sugg));
                }
            },
            CompilerError::TypeError { message, location, expected_type, actual_type, suggestion } => {
                output.push_str(&self.format_error_header("Type Error", message));
                output.push_str(&self.format_location(location, context));
                if let (Some(exp), Some(act)) = (expected_type, actual_type) {
                    output.push_str(&format!("  Expected type: {}\n", exp));
                    output.push_str(&format!("  Actual type: {}\n", act));
                }
                if let Some(sugg) = suggestion {
                    output.push_str(&self.format_suggestion(sugg));
                }
            },
            CompilerError::MultipleErrors { errors, count } => {
                output.push_str(&format!("Multiple errors occurred ({} total):\n", count));
                for (i, err) in errors.iter().enumerate() {
                    output.push_str(&format!("\n{}. {}", i + 1, self.format_error(err, context)));
                }
            },
            _ => {
                output.push_str(&format!("{}\n", error));
            }
        }
        
        output
    }

    /// Format error header with colors
    fn format_error_header(&self, error_type: &str, message: &str) -> String {
        if self.colored_output {
            format!("\x1b[1;31m{}\x1b[0m: {}\n", error_type, message)
        } else {
            format!("{}: {}\n", error_type, message)
        }
    }

    /// Format location with source context
    fn format_location(&self, location: &ErrorLocation, context: &ErrorContext) -> String {
        let mut output = format!("  --> {}:{}:{}\n", location.file, location.line, location.column);
        
        if self.show_source_context {
            if let Some(source) = context.source_files.get(&location.file) {
                output.push_str(&self.format_source_context(source, location));
            }
        }
        
        output
    }

    /// Format brief location reference
    fn format_location_brief(&self, location: &ErrorLocation) -> String {
        format!("{}:{}:{}", location.file, location.line, location.column)
    }

    /// Format suggestion
    fn format_suggestion(&self, suggestion: &str) -> String {
        if self.show_suggestions {
            if self.colored_output {
                format!("  \x1b[1;36mSuggestion\x1b[0m: {}\n", suggestion)
            } else {
                format!("  Suggestion: {}\n", suggestion)
            }
        } else {
            String::new()
        }
    }

    /// Format source context around error location
    fn format_source_context(&self, source: &str, location: &ErrorLocation) -> String {
        let lines: Vec<&str> = source.lines().collect();
        let line_num = location.line as usize;
        
        if line_num == 0 || line_num > lines.len() {
            return String::new();
        }
        
        let mut output = String::new();
        let start = line_num.saturating_sub(self.max_context_lines).max(1);
        let end = (line_num + self.max_context_lines).min(lines.len());
        
        for i in start..=end {
            let line = lines.get(i - 1).unwrap_or(&"");
            let line_marker = if i == line_num { ">" } else { " " };
            
            if self.colored_output {
                if i == line_num {
                    output.push_str(&format!("  \x1b[1;34m{:4}\x1b[0m {} {}\n", i, line_marker, line));
                    // Add caret indicator
                    let spaces = " ".repeat(location.column as usize + 8);
                    let caret_length = location.length.unwrap_or(1) as usize;
                    let carets = "^".repeat(caret_length);
                    output.push_str(&format!("  {}\x1b[1;31m{}\x1b[0m\n", spaces, carets));
                } else {
                    output.push_str(&format!("  {:4} {} {}\n", i, line_marker, line));
                }
            } else {
                output.push_str(&format!("  {:4} {} {}\n", i, line_marker, line));
                if i == line_num {
                    let spaces = " ".repeat(location.column as usize + 8);
                    let caret_length = location.length.unwrap_or(1) as usize;
                    let carets = "^".repeat(caret_length);
                    output.push_str(&format!("  {}{}\n", spaces, carets));
                }
            }
        }
        
        output
    }

    /// Convert compiler error to LSP diagnostic
    pub fn to_diagnostic(&self, error: &CompilerError) -> Option<Diagnostic> {
        match error {
            CompilerError::SemanticError { message, location, error_code, severity, .. } => {
                Some(Diagnostic {
                    severity: match severity {
                        ErrorSeverity::Error => DiagnosticSeverity::Error,
                        ErrorSeverity::Warning => DiagnosticSeverity::Warning,
                        ErrorSeverity::Info => DiagnosticSeverity::Information,
                        ErrorSeverity::Hint => DiagnosticSeverity::Hint,
                    },
                    code: Some(error_code.clone()),
                    message: message.clone(),
                    source: "neo-solc".to_string(),
                    range: DiagnosticRange {
                        start: Position {
                            line: location.line.saturating_sub(1), // LSP is 0-based
                            character: location.column.saturating_sub(1),
                        },
                        end: Position {
                            line: location.line.saturating_sub(1),
                            character: location.column.saturating_sub(1) + location.length.unwrap_or(1),
                        },
                    },
                    related_information: Vec::new(),
                    tags: Vec::new(),
                    code_description: None,
                    data: None,
                })
            },
            // Add conversions for other error types as needed
            _ => None,
        }
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

impl fmt::Display for CompilationPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let phase_name = match self {
            CompilationPhase::Lexing => "Lexical Analysis",
            CompilationPhase::Parsing => "Parsing",
            CompilationPhase::SemanticAnalysis => "Semantic Analysis",
            CompilationPhase::Optimization => "Optimization",
            CompilationPhase::CodeGeneration => "Code Generation",
            CompilationPhase::Linking => "Linking",
        };
        write!(f, "{}", phase_name)
    }
}

/// Helper functions for creating errors
pub mod error_builders {
    use super::*;

    pub fn lexical_error(message: String, location: ErrorLocation) -> CompilerError {
        CompilerError::LexicalError {
            message,
            location,
            suggestion: None,
        }
    }

    pub fn parse_error(message: String, location: ErrorLocation) -> CompilerError {
        CompilerError::ParseError {
            message,
            location,
            expected: None,
            found: None,
            suggestion: None,
        }
    }

    pub fn semantic_error(message: String, location: ErrorLocation, code: String) -> CompilerError {
        CompilerError::SemanticError {
            message,
            location,
            error_code: code,
            severity: ErrorSeverity::Error,
            related_locations: Vec::new(),
            suggestion: None,
        }
    }

    pub fn type_error(message: String, location: ErrorLocation) -> CompilerError {
        CompilerError::TypeError {
            message,
            location,
            expected_type: None,
            actual_type: None,
            suggestion: None,
        }
    }

    pub fn internal_error(message: String) -> CompilerError {
        CompilerError::InternalError {
            message,
            location: None,
            debug_info: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_context() {
        let mut context = ErrorContext::new(CompilationPhase::Parsing);
        context.add_source_file("test.yul".to_string(), "let x := 1".to_string());
        
        let error = error_builders::parse_error(
            "Unexpected token".to_string(),
            ErrorLocation {
                file: "test.yul".to_string(),
                line: 1,
                column: 5,
                length: Some(1),
                offset: Some(4),
                source_line: None,
            }
        );
        
        let strategy = context.collect_error(error);
        assert!(matches!(strategy, RecoveryStrategy::Synchronize));
        assert_eq!(context.collected_errors.len(), 1);
    }

    #[test]
    fn test_error_reporter() {
        let reporter = ErrorReporter::new();
        let context = ErrorContext::new(CompilationPhase::SemanticAnalysis);
        
        let error = error_builders::semantic_error(
            "Variable not found".to_string(),
            ErrorLocation {
                file: "test.yul".to_string(),
                line: 10,
                column: 15,
                length: Some(3),
                offset: Some(150),
                source_line: None,
            },
            "E001".to_string(),
        );
        
        let formatted = reporter.format_error(&error, &context);
        assert!(formatted.contains("Semantic Error"));
        assert!(formatted.contains("Variable not found"));
        assert!(formatted.contains("test.yul:10:15"));
    }

    #[test]
    fn test_diagnostic_conversion() {
        let reporter = ErrorReporter::new();
        let error = error_builders::semantic_error(
            "Undefined variable".to_string(),
            ErrorLocation {
                file: "test.yul".to_string(),
                line: 5,
                column: 10,
                length: Some(8),
                offset: Some(42),
                source_line: None,
            },
            "E002".to_string(),
        );
        
        let diagnostic = reporter.to_diagnostic(&error).unwrap();
        assert_eq!(diagnostic.code, Some("E002".to_string()));
        assert_eq!(diagnostic.message, "Undefined variable");
        assert_eq!(diagnostic.range.start.line, 4); // 0-based
        assert_eq!(diagnostic.range.start.character, 9); // 0-based
    }

    #[test]
    fn test_multiple_errors() {
        let errors = vec![
            error_builders::lexical_error("Invalid character".to_string(), ErrorLocation {
                file: "test.yul".to_string(),
                line: 1,
                column: 1,
                length: Some(1),
                offset: Some(0),
                source_line: None,
            }),
            error_builders::parse_error("Expected '}'".to_string(), ErrorLocation {
                file: "test.yul".to_string(),
                line: 5,
                column: 10,
                length: Some(1),
                offset: Some(50),
                source_line: None,
            }),
        ];
        
        let multiple = CompilerError::MultipleErrors {
            count: errors.len(),
            errors,
        };
        
        if let CompilerError::MultipleErrors { count, .. } = multiple {
            assert_eq!(count, 2);
        } else {
            panic!("Expected MultipleErrors variant");
        }
    }
}