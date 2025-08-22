use crate::error::CompilerError;

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
    Identifier,
    Literal,
    Comma,
    Plus,
    Minus,
    If,
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

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CompilerError> {
        let mut tokens = Vec::new();
        
        // Simple tokenization for demo
        tokens.push(Token {
            token_type: TokenType::LeftBrace,
            value: "{".to_string(),
            line: 1,
            column: 1,
        });
        
        tokens.push(Token {
            token_type: TokenType::RightBrace,
            value: "}".to_string(),
            line: 1,
            column: 2,
        });
        
        Ok(tokens)
    }
}