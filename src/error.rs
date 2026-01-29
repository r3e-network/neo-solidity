//! Compiler Error Types
//!
//! Defines error types used throughout the Neo Solidity compiler pipeline.
//! All errors implement the standard `Error` trait for easy integration.
//!
//! # Error Categories
//!
//! - [`CompilerError::IoError`] - File system and I/O errors
//! - [`CompilerError::ParseError`] - Syntax and parsing errors
//! - [`CompilerError::SemanticError`] - Type checking and semantic errors
//! - [`CompilerError::CodegenError`] - Code generation errors
//!
//! # Source Location
//!
//! Errors can include source location information via [`SourceLocation`] for
//! better diagnostic messages.

use thiserror::Error;

/// Source location information for error reporting
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceLocation {
    /// Source file path (if available)
    pub file: Option<String>,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Length of the error span
    pub length: usize,
}

impl SourceLocation {
    /// Create a new source location
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            file: None,
            line,
            column,
            length: 0,
        }
    }

    /// Create a source location with file information
    pub fn with_file(file: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            file: Some(file.into()),
            line,
            column,
            length: 0,
        }
    }

    /// Set the span length
    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.file {
            Some(file) => write!(f, "{}:{}:{}", file, self.line, self.column),
            None => write!(f, "{}:{}", self.line, self.column),
        }
    }
}

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("IO error: {0}")]
    IoError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Semantic error: {0}")]
    SemanticError(String),

    #[error("Codegen error: {0}")]
    CodegenError(String),

    #[error("{severity} at {location}: {message}")]
    Located {
        severity: ErrorSeverity,
        location: SourceLocation,
        message: String,
        code: Option<ErrorCode>,
    },
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Info => write!(f, "info"),
            Self::Hint => write!(f, "hint"),
        }
    }
}

/// Error codes for programmatic error handling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // Parse errors (1xxx)
    UnexpectedToken = 1001,
    UnexpectedEof = 1002,
    InvalidSyntax = 1003,

    // Semantic errors (2xxx)
    UndefinedVariable = 2001,
    TypeMismatch = 2002,
    DuplicateDefinition = 2003,

    // Codegen errors (3xxx)
    UnsupportedFeature = 3001,
    InvalidBytecode = 3002,

    // IO errors (4xxx)
    FileNotFound = 4001,
    PermissionDenied = 4002,
}

impl CompilerError {
    /// Create a located error with full context
    pub fn located(
        severity: ErrorSeverity,
        location: SourceLocation,
        message: impl Into<String>,
        code: Option<ErrorCode>,
    ) -> Self {
        Self::Located {
            severity,
            location,
            message: message.into(),
            code,
        }
    }

    /// Create a parse error at a specific location
    pub fn parse_at(location: SourceLocation, message: impl Into<String>) -> Self {
        Self::located(ErrorSeverity::Error, location, message, Some(ErrorCode::InvalidSyntax))
    }

    /// Create a semantic error at a specific location
    pub fn semantic_at(location: SourceLocation, message: impl Into<String>) -> Self {
        Self::located(ErrorSeverity::Error, location, message, Some(ErrorCode::TypeMismatch))
    }

    /// Check if this is a fatal error
    pub fn is_fatal(&self) -> bool {
        match self {
            Self::Located { severity, .. } => *severity == ErrorSeverity::Error,
            _ => true,
        }
    }
}

impl From<std::io::Error> for CompilerError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}
