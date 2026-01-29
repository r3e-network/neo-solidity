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
//!
//! # Fix Suggestions
//!
//! Errors can include [`FixSuggestion`] to help users resolve issues.

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
    /// End line for multi-line spans
    pub end_line: Option<usize>,
    /// End column for multi-line spans
    pub end_column: Option<usize>,
}

impl SourceLocation {
    /// Create a new source location
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            file: None,
            line,
            column,
            length: 0,
            end_line: None,
            end_column: None,
        }
    }

    /// Create a source location with file information
    pub fn with_file(file: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            file: Some(file.into()),
            line,
            column,
            length: 0,
            end_line: None,
            end_column: None,
        }
    }

    /// Set the span length
    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    /// Set the end position for multi-line spans
    pub fn with_end(mut self, end_line: usize, end_column: usize) -> Self {
        self.end_line = Some(end_line);
        self.end_column = Some(end_column);
        self
    }

    /// Check if this is a valid location
    pub fn is_valid(&self) -> bool {
        self.line > 0 && self.column > 0
    }

    /// Create a span from start to end location
    pub fn span_to(&self, end: &SourceLocation) -> Self {
        Self {
            file: self.file.clone(),
            line: self.line,
            column: self.column,
            length: 0,
            end_line: Some(end.line),
            end_column: Some(end.column),
        }
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

/// A suggested fix for an error
#[derive(Debug, Clone)]
pub struct FixSuggestion {
    /// Description of the fix
    pub message: String,
    /// The replacement text (if applicable)
    pub replacement: Option<String>,
    /// Location where the fix should be applied
    pub location: Option<SourceLocation>,
    /// Whether this fix can be applied automatically
    pub is_auto_fixable: bool,
}

impl FixSuggestion {
    /// Create a simple suggestion message
    pub fn hint(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            replacement: None,
            location: None,
            is_auto_fixable: false,
        }
    }

    /// Create a suggestion with replacement text
    pub fn replace(message: impl Into<String>, replacement: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            replacement: Some(replacement.into()),
            location: None,
            is_auto_fixable: true,
        }
    }

    /// Create a suggestion with location
    pub fn at(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl std::fmt::Display for FixSuggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "help: {}", self.message)?;
        if let Some(ref replacement) = self.replacement {
            write!(f, "\n  suggestion: `{}`", replacement)?;
        }
        Ok(())
    }
}

/// Related diagnostic information
#[derive(Debug, Clone)]
pub struct RelatedInfo {
    /// Location of related code
    pub location: SourceLocation,
    /// Description of the relation
    pub message: String,
}

impl RelatedInfo {
    pub fn new(location: SourceLocation, message: impl Into<String>) -> Self {
        Self {
            location,
            message: message.into(),
        }
    }
}

/// Diagnostic data (boxed to reduce CompilerError size)
#[derive(Debug, Clone)]
pub struct DiagnosticData {
    pub severity: ErrorSeverity,
    pub location: SourceLocation,
    pub message: String,
    pub code: ErrorCode,
    pub suggestions: Vec<FixSuggestion>,
    pub related: Vec<RelatedInfo>,
    pub source_snippet: Option<String>,
}

impl std::fmt::Display for DiagnosticData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}] at {}: {}", self.severity, self.code, self.location, self.message)
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

    #[error("{0}")]
    Diagnostic(Box<DiagnosticData>),
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // Parse errors (1xxx)
    UnexpectedToken = 1001,
    UnexpectedEof = 1002,
    InvalidSyntax = 1003,
    MissingSemicolon = 1004,
    MissingBrace = 1005,
    InvalidNumber = 1006,
    InvalidString = 1007,
    InvalidIdentifier = 1008,

    // Semantic errors (2xxx)
    UndefinedVariable = 2001,
    TypeMismatch = 2002,
    DuplicateDefinition = 2003,
    UndefinedFunction = 2004,
    UndefinedType = 2005,
    InvalidAssignment = 2006,
    ImmutableModification = 2007,
    VisibilityError = 2008,
    InvalidOverride = 2009,
    MissingReturn = 2010,
    UnreachableCode = 2011,
    UnusedVariable = 2012,
    UnusedFunction = 2013,
    ShadowedVariable = 2014,
    InvalidModifier = 2015,

    // Type errors (25xx)
    IntegerOverflow = 2501,
    IntegerUnderflow = 2502,
    DivisionByZero = 2503,
    ArrayOutOfBounds = 2504,
    InvalidCast = 2505,
    NullReference = 2506,

    // Codegen errors (3xxx)
    UnsupportedFeature = 3001,
    InvalidBytecode = 3002,
    StackOverflow = 3003,
    GasLimitExceeded = 3004,
    ContractTooLarge = 3005,
    InvalidOpcode = 3006,

    // IO errors (4xxx)
    FileNotFound = 4001,
    PermissionDenied = 4002,
    ImportNotFound = 4003,
    CircularImport = 4004,

    // Security warnings (5xxx)
    ReentrancyRisk = 5001,
    UncheckedCall = 5002,
    TxOriginUsage = 5003,
    UnsafeDelegate = 5004,
    IntegerOverflowRisk = 5005,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E{:04}", *self as u32)
    }
}

impl ErrorCode {
    /// Get a human-readable description of the error code
    pub fn description(&self) -> &'static str {
        match self {
            Self::UnexpectedToken => "unexpected token in input",
            Self::UnexpectedEof => "unexpected end of file",
            Self::InvalidSyntax => "invalid syntax",
            Self::MissingSemicolon => "missing semicolon",
            Self::MissingBrace => "missing brace",
            Self::InvalidNumber => "invalid number literal",
            Self::InvalidString => "invalid string literal",
            Self::InvalidIdentifier => "invalid identifier",
            Self::UndefinedVariable => "undefined variable",
            Self::TypeMismatch => "type mismatch",
            Self::DuplicateDefinition => "duplicate definition",
            Self::UndefinedFunction => "undefined function",
            Self::UndefinedType => "undefined type",
            Self::InvalidAssignment => "invalid assignment",
            Self::ImmutableModification => "cannot modify immutable value",
            Self::VisibilityError => "visibility error",
            Self::InvalidOverride => "invalid override",
            Self::MissingReturn => "missing return statement",
            Self::UnreachableCode => "unreachable code",
            Self::UnusedVariable => "unused variable",
            Self::UnusedFunction => "unused function",
            Self::ShadowedVariable => "variable shadows outer scope",
            Self::InvalidModifier => "invalid modifier",
            Self::IntegerOverflow => "integer overflow",
            Self::IntegerUnderflow => "integer underflow",
            Self::DivisionByZero => "division by zero",
            Self::ArrayOutOfBounds => "array index out of bounds",
            Self::InvalidCast => "invalid type cast",
            Self::NullReference => "null reference",
            Self::UnsupportedFeature => "unsupported feature",
            Self::InvalidBytecode => "invalid bytecode",
            Self::StackOverflow => "stack overflow",
            Self::GasLimitExceeded => "gas limit exceeded",
            Self::ContractTooLarge => "contract too large",
            Self::InvalidOpcode => "invalid opcode",
            Self::FileNotFound => "file not found",
            Self::PermissionDenied => "permission denied",
            Self::ImportNotFound => "import not found",
            Self::CircularImport => "circular import detected",
            Self::ReentrancyRisk => "potential reentrancy vulnerability",
            Self::UncheckedCall => "unchecked external call",
            Self::TxOriginUsage => "tx.origin used for authorization",
            Self::UnsafeDelegate => "unsafe delegatecall",
            Self::IntegerOverflowRisk => "potential integer overflow",
        }
    }
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
            Self::Diagnostic(d) => d.severity == ErrorSeverity::Error,
            _ => true,
        }
    }

    /// Check if this is a warning
    pub fn is_warning(&self) -> bool {
        match self {
            Self::Located { severity, .. } => *severity == ErrorSeverity::Warning,
            Self::Diagnostic(d) => d.severity == ErrorSeverity::Warning,
            _ => false,
        }
    }

    /// Get the error code if available
    pub fn code(&self) -> Option<ErrorCode> {
        match self {
            Self::Located { code, .. } => *code,
            Self::Diagnostic(d) => Some(d.code),
            _ => None,
        }
    }

    /// Get suggestions if available
    pub fn suggestions(&self) -> &[FixSuggestion] {
        match self {
            Self::Diagnostic(d) => &d.suggestions,
            _ => &[],
        }
    }
}

impl From<std::io::Error> for CompilerError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}

/// Builder for creating rich diagnostic messages
#[derive(Debug)]
pub struct DiagnosticBuilder {
    severity: ErrorSeverity,
    location: SourceLocation,
    message: String,
    code: ErrorCode,
    suggestions: Vec<FixSuggestion>,
    related: Vec<RelatedInfo>,
    source_snippet: Option<String>,
}

impl DiagnosticBuilder {
    /// Create a new error diagnostic
    pub fn error(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            severity: ErrorSeverity::Error,
            location: SourceLocation::default(),
            message: message.into(),
            code,
            suggestions: Vec::new(),
            related: Vec::new(),
            source_snippet: None,
        }
    }

    /// Create a new warning diagnostic
    pub fn warning(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            severity: ErrorSeverity::Warning,
            location: SourceLocation::default(),
            message: message.into(),
            code,
            suggestions: Vec::new(),
            related: Vec::new(),
            source_snippet: None,
        }
    }

    /// Set the source location
    pub fn at(mut self, location: SourceLocation) -> Self {
        self.location = location;
        self
    }

    /// Add a fix suggestion
    pub fn suggest(mut self, suggestion: FixSuggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    /// Add related information
    pub fn related(mut self, info: RelatedInfo) -> Self {
        self.related.push(info);
        self
    }

    /// Add source code snippet
    pub fn with_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.source_snippet = Some(snippet.into());
        self
    }

    /// Build the final CompilerError
    pub fn build(self) -> CompilerError {
        CompilerError::Diagnostic(Box::new(DiagnosticData {
            severity: self.severity,
            location: self.location,
            message: self.message,
            code: self.code,
            suggestions: self.suggestions,
            related: self.related,
            source_snippet: self.source_snippet,
        }))
    }
}
