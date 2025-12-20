#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Let,
    Assignment,
    Arrow,
    Identifier,
    Literal,
    Comma,
    Plus,
    Minus,
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
    BuiltinFunction,
}

pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

