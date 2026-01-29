/// A token produced by the lexer
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    /// Create a new token
    pub fn new(token_type: TokenType, value: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            token_type,
            value: value.into(),
            line,
            column,
        }
    }

    /// Check if this token is a keyword
    pub fn is_keyword(&self) -> bool {
        self.token_type.is_keyword()
    }

    /// Check if this token is an operator
    pub fn is_operator(&self) -> bool {
        self.token_type.is_operator()
    }

    /// Get the span length of this token
    pub fn span_len(&self) -> usize {
        self.value.len()
    }
}

/// Token types for the Yul language
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Delimiters
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Comma,

    // Keywords
    Let,
    If,
    Else,
    For,
    Switch,
    Case,
    Default,
    Leave,
    Break,
    Continue,
    Function,

    // Operators
    Assignment,
    Arrow,
    Plus,
    Minus,

    // Values
    Identifier,
    Literal,
    BuiltinFunction,
}

impl TokenType {
    /// Check if this token type is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Let
                | Self::If
                | Self::Else
                | Self::For
                | Self::Switch
                | Self::Case
                | Self::Default
                | Self::Leave
                | Self::Break
                | Self::Continue
                | Self::Function
        )
    }

    /// Check if this token type is an operator
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Self::Assignment | Self::Arrow | Self::Plus | Self::Minus
        )
    }

    /// Check if this token type is a delimiter
    pub fn is_delimiter(&self) -> bool {
        matches!(
            self,
            Self::LeftBrace | Self::RightBrace | Self::LeftParen | Self::RightParen | Self::Comma
        )
    }

    /// Get the string representation of this token type
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::Comma => ",",
            Self::Let => "let",
            Self::If => "if",
            Self::Else => "else",
            Self::For => "for",
            Self::Switch => "switch",
            Self::Case => "case",
            Self::Default => "default",
            Self::Leave => "leave",
            Self::Break => "break",
            Self::Continue => "continue",
            Self::Function => "function",
            Self::Assignment => ":=",
            Self::Arrow => "->",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Identifier => "<identifier>",
            Self::Literal => "<literal>",
            Self::BuiltinFunction => "<builtin>",
        }
    }
}

/// Yul language lexer
pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Get current position in the input
    pub fn position(&self) -> usize {
        self.position
    }

    /// Get current line number
    pub fn line(&self) -> usize {
        self.line
    }

    /// Get current column number
    pub fn column(&self) -> usize {
        self.column
    }
}

