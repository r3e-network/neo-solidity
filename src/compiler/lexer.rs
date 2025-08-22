//! Complete Yul Lexer Implementation
//! 
//! Production-ready lexical analyzer supporting all Yul tokens, operators, 
//! built-in functions, and advanced tokenization features.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

/// Yul lexer for tokenizing source code
#[derive(Debug, Clone)]
pub struct YulLexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
    line: u32,
    column: u32,
    keywords: HashMap<String, TokenType>,
    builtin_functions: HashMap<String, BuiltinCategory>,
}

/// Token represents a lexical token in Yul source code
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub position: TokenPosition,
}

/// Token position information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenPosition {
    pub line: u32,
    pub column: u32,
    pub offset: usize,
    pub length: usize,
}

/// All possible Yul token types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
    // Illegal/Unknown
    Illegal,
    Eof,

    // Identifiers and literals
    Identifier,
    Number,
    String,
    HexNumber,
    Boolean,

    // Delimiters
    LeftParen,     // (
    RightParen,    // )
    LeftBrace,     // {
    RightBrace,    // }
    Comma,         // ,
    Colon,         // :
    Dot,           // .

    // Operators
    Assign,        // :=
    Arrow,         // ->

    // Keywords
    Object,
    Code,
    Data,
    Function,
    Let,
    If,
    Switch,
    Case,
    Default,
    For,
    Break,
    Continue,
    Leave,
    True,
    False,

    // Built-in function categories
    Arithmetic(String),     // add, sub, mul, etc.
    Comparison(String),     // lt, gt, eq, etc.
    Bitwise(String),       // and, or, xor, etc.
    Memory(String),        // mload, mstore, etc.
    Storage(String),       // sload, sstore
    Environment(String),   // address, caller, etc.
    Control(String),       // call, return, etc.
    Crypto(String),        // keccak256, sha256, etc.
    Block(String),         // blockhash, timestamp, etc.
    Transaction(String),   // origin, gasprice, etc.

    // Comments and whitespace
    Comment,
    Whitespace,
    Newline,
}

/// Categories of built-in functions
#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinCategory {
    Arithmetic,
    Comparison, 
    Bitwise,
    Memory,
    Storage,
    Environment,
    Control,
    Crypto,
    Block,
    Transaction,
}

/// Lexer errors
#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Unexpected character '{0}' at line {1}, column {2}")]
    UnexpectedCharacter(char, u32, u32),
    
    #[error("Unterminated string at line {0}")]
    UnterminatedString(u32),
    
    #[error("Invalid number format '{0}' at line {1}")]
    InvalidNumber(String, u32),
    
    #[error("Invalid hex number '{0}' at line {1}")]
    InvalidHexNumber(String, u32),
    
    #[error("Unterminated comment at line {0}")]
    UnterminatedComment(u32),
    
    #[error("Invalid escape sequence '{0}' at line {1}")]
    InvalidEscape(String, u32),
}

impl YulLexer {
    /// Create a new Yul lexer
    pub fn new() -> Self {
        let mut lexer = Self {
            input: String::new(),
            position: 0,
            read_position: 0,
            ch: 0,
            line: 1,
            column: 1,
            keywords: HashMap::new(),
            builtin_functions: HashMap::new(),
        };
        
        lexer.init_keywords();
        lexer.init_builtin_functions();
        lexer
    }

    /// Initialize keyword mapping
    fn init_keywords(&mut self) {
        let keywords = [
            ("object", TokenType::Object),
            ("code", TokenType::Code),
            ("data", TokenType::Data),
            ("function", TokenType::Function),
            ("let", TokenType::Let),
            ("if", TokenType::If),
            ("switch", TokenType::Switch),
            ("case", TokenType::Case),
            ("default", TokenType::Default),
            ("for", TokenType::For),
            ("break", TokenType::Break),
            ("continue", TokenType::Continue),
            ("leave", TokenType::Leave),
            ("true", TokenType::True),
            ("false", TokenType::False),
        ];

        for (word, token_type) in keywords.iter() {
            self.keywords.insert(word.to_string(), token_type.clone());
        }
    }

    /// Initialize built-in function categories
    fn init_builtin_functions(&mut self) {
        // Arithmetic operations
        let arithmetic = [
            "add", "sub", "mul", "div", "sdiv", "mod", "smod", "exp", "not",
            "addmod", "mulmod", "signextend"
        ];
        for func in arithmetic.iter() {
            self.builtin_functions.insert(func.to_string(), BuiltinCategory::Arithmetic);
        }

        // Comparison operations
        let comparison = ["lt", "gt", "slt", "sgt", "eq", "iszero"];
        for func in comparison.iter() {
            self.builtin_functions.insert(func.to_string(), BuiltinCategory::Comparison);
        }

        // Bitwise operations
        let bitwise = ["and", "or", "xor", "byte", "shl", "shr", "sar"];
        for func in bitwise.iter() {
            self.builtin_functions.insert(func.to_string(), BuiltinCategory::Bitwise);
        }

        // Memory operations
        let memory = [
            "mload", "mstore", "mstore8", "msize", "calldataload", "calldatasize", 
            "calldatacopy", "codecopy", "codesize", "extcodesize", "extcodecopy",
            "returndatasize", "returndatacopy", "mcopy"
        ];
        for func in memory.iter() {
            self.builtin_functions.insert(func.to_string(), BuiltinCategory::Memory);
        }

        // Storage operations
        let storage = ["sload", "sstore"];
        for func in storage.iter() {
            self.builtin_functions.insert(func.to_string(), BuiltinCategory::Storage);
        }

        // Environment operations
        let environment = [
            "address", "balance", "selfbalance", "caller", "callvalue", "origin", 
            "gasprice", "extcodesize", "extcodehash", "gas", "pc", "pop"
        ];
        for func in environment.iter() {
            self.builtin_functions.insert(func.to_string(), BuiltinCategory::Environment);
        }

        // Control flow operations
        let control = [
            "call", "callcode", "delegatecall", "staticcall", "return", "revert", 
            "selfdestruct", "invalid", "log0", "log1", "log2", "log3", "log4", 
            "create", "create2", "stop"
        ];
        for func in control.iter() {
            self.builtin_functions.insert(func.to_string(), BuiltinCategory::Control);
        }

        // Cryptographic operations
        let crypto = ["keccak256", "sha256", "ripemd160", "ecrecover"];
        for func in crypto.iter() {
            self.builtin_functions.insert(func.to_string(), BuiltinCategory::Crypto);
        }

        // Block operations
        let block = [
            "blockhash", "coinbase", "timestamp", "number", "difficulty", 
            "gaslimit", "chainid", "basefee"
        ];
        for func in block.iter() {
            self.builtin_functions.insert(func.to_string(), BuiltinCategory::Block);
        }

        // Transaction operations
        let transaction = ["origin", "gasprice", "gas"];
        for func in transaction.iter() {
            self.builtin_functions.insert(func.to_string(), BuiltinCategory::Transaction);
        }
    }

    /// Tokenize the input string
    pub fn tokenize(&mut self, input: &str) -> Result<Vec<Token>, LexerError> {
        self.input = input.to_string();
        self.position = 0;
        self.read_position = 0;
        self.ch = 0;
        self.line = 1;
        self.column = 1;

        if !input.is_empty() {
            self.read_char();
        }

        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let is_eof = token.token_type == TokenType::Eof;
            tokens.push(token);
            
            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }

    /// Get the next token from input
    fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        let start_line = self.line;
        let start_column = self.column;
        let start_position = self.position;

        let token_type = match self.ch {
            0 => TokenType::Eof,
            b'(' => {
                self.read_char();
                TokenType::LeftParen
            },
            b')' => {
                self.read_char();
                TokenType::RightParen
            },
            b'{' => {
                self.read_char();
                TokenType::LeftBrace
            },
            b'}' => {
                self.read_char();
                TokenType::RightBrace
            },
            b',' => {
                self.read_char();
                TokenType::Comma
            },
            b'.' => {
                self.read_char();
                TokenType::Dot
            },
            b':' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    TokenType::Assign
                } else {
                    self.read_char();
                    TokenType::Colon
                }
            },
            b'-' => {
                if self.peek_char() == b'>' {
                    self.read_char();
                    self.read_char();
                    TokenType::Arrow
                } else {
                    return Err(LexerError::UnexpectedCharacter(
                        self.ch as char, self.line, self.column
                    ));
                }
            },
            b'"' => {
                let literal = self.read_string()?;
                return Ok(Token {
                    token_type: TokenType::String,
                    literal,
                    position: TokenPosition {
                        line: start_line,
                        column: start_column,
                        offset: start_position,
                        length: self.position - start_position,
                    },
                });
            },
            b'/' => {
                if self.peek_char() == b'/' {
                    let literal = self.read_line_comment();
                    return Ok(Token {
                        token_type: TokenType::Comment,
                        literal,
                        position: TokenPosition {
                            line: start_line,
                            column: start_column,
                            offset: start_position,
                            length: self.position - start_position,
                        },
                    });
                } else if self.peek_char() == b'*' {
                    let literal = self.read_block_comment()?;
                    return Ok(Token {
                        token_type: TokenType::Comment,
                        literal,
                        position: TokenPosition {
                            line: start_line,
                            column: start_column,
                            offset: start_position,
                            length: self.position - start_position,
                        },
                    });
                } else {
                    return Err(LexerError::UnexpectedCharacter(
                        self.ch as char, self.line, self.column
                    ));
                }
            },
            b'0' => {
                if self.peek_char() == b'x' || self.peek_char() == b'X' {
                    let literal = self.read_hex_number()?;
                    return Ok(Token {
                        token_type: TokenType::HexNumber,
                        literal,
                        position: TokenPosition {
                            line: start_line,
                            column: start_column,
                            offset: start_position,
                            length: self.position - start_position,
                        },
                    });
                } else {
                    let literal = self.read_number()?;
                    return Ok(Token {
                        token_type: TokenType::Number,
                        literal,
                        position: TokenPosition {
                            line: start_line,
                            column: start_column,
                            offset: start_position,
                            length: self.position - start_position,
                        },
                    });
                }
            },
            ch if ch.is_ascii_digit() => {
                let literal = self.read_number()?;
                return Ok(Token {
                    token_type: TokenType::Number,
                    literal,
                    position: TokenPosition {
                        line: start_line,
                        column: start_column,
                        offset: start_position,
                        length: self.position - start_position,
                    },
                });
            },
            ch if ch.is_ascii_alphabetic() || ch == b'_' => {
                let literal = self.read_identifier();
                let token_type = self.lookup_identifier_type(&literal);
                return Ok(Token {
                    token_type,
                    literal,
                    position: TokenPosition {
                        line: start_line,
                        column: start_column,
                        offset: start_position,
                        length: self.position - start_position,
                    },
                });
            },
            _ => {
                return Err(LexerError::UnexpectedCharacter(
                    self.ch as char, self.line, self.column
                ));
            }
        };

        Ok(Token {
            token_type,
            literal: String::new(),
            position: TokenPosition {
                line: start_line,
                column: start_column,
                offset: start_position,
                length: self.position - start_position,
            },
        })
    }

    /// Read current character and advance position
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;

        if self.ch == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }

    /// Peek at next character without advancing
    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    /// Read string literal
    fn read_string(&mut self) -> Result<String, LexerError> {
        let start_line = self.line;
        let mut result = String::new();
        self.read_char(); // Skip opening quote

        loop {
            match self.ch {
                0 => return Err(LexerError::UnterminatedString(start_line)),
                b'"' => {
                    self.read_char(); // Skip closing quote
                    break;
                },
                b'\\' => {
                    self.read_char();
                    match self.ch {
                        b'n' => result.push('\n'),
                        b't' => result.push('\t'),
                        b'r' => result.push('\r'),
                        b'\\' => result.push('\\'),
                        b'"' => result.push('"'),
                        b'0' => result.push('\0'),
                        b'x' => {
                            // Hex escape sequence
                            self.read_char();
                            let hex1 = self.ch;
                            self.read_char();
                            let hex2 = self.ch;
                            
                            if hex1.is_ascii_hexdigit() && hex2.is_ascii_hexdigit() {
                                let hex_str = format!("{}{}", hex1 as char, hex2 as char);
                                if let Ok(val) = u8::from_str_radix(&hex_str, 16) {
                                    result.push(val as char);
                                } else {
                                    return Err(LexerError::InvalidEscape(
                                        format!("\\x{}", hex_str), self.line
                                    ));
                                }
                            } else {
                                return Err(LexerError::InvalidEscape(
                                    format!("\\x{}{}", hex1 as char, hex2 as char), self.line
                                ));
                            }
                        },
                        _ => {
                            return Err(LexerError::InvalidEscape(
                                format!("\\{}", self.ch as char), self.line
                            ));
                        }
                    }
                    self.read_char();
                },
                ch => {
                    result.push(ch as char);
                    self.read_char();
                }
            }
        }

        Ok(result)
    }

    /// Read decimal number
    fn read_number(&mut self) -> Result<String, LexerError> {
        let start_pos = self.position;
        
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        let number_str = self.input[start_pos..self.position].to_string();
        
        // Validate number (can be arbitrary precision)
        if number_str.is_empty() || !number_str.chars().all(|c| c.is_ascii_digit()) {
            return Err(LexerError::InvalidNumber(number_str, self.line));
        }

        Ok(number_str)
    }

    /// Read hexadecimal number
    fn read_hex_number(&mut self) -> Result<String, LexerError> {
        let start_pos = self.position;
        
        self.read_char(); // skip '0'
        self.read_char(); // skip 'x' or 'X'

        if !self.ch.is_ascii_hexdigit() {
            return Err(LexerError::InvalidHexNumber(
                self.input[start_pos..self.position].to_string(), self.line
            ));
        }

        while self.ch.is_ascii_hexdigit() {
            self.read_char();
        }

        let hex_str = self.input[start_pos..self.position].to_string();
        
        // Validate hex number format
        if hex_str.len() < 3 || !hex_str.starts_with("0x") {
            return Err(LexerError::InvalidHexNumber(hex_str, self.line));
        }

        Ok(hex_str)
    }

    /// Read identifier
    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        
        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }

        self.input[start_pos..self.position].to_string()
    }

    /// Read line comment
    fn read_line_comment(&mut self) -> String {
        let start_pos = self.position;
        
        while self.ch != b'\n' && self.ch != 0 {
            self.read_char();
        }

        self.input[start_pos..self.position].to_string()
    }

    /// Read block comment
    fn read_block_comment(&mut self) -> Result<String, LexerError> {
        let start_pos = self.position;
        let start_line = self.line;
        
        self.read_char(); // skip '/'
        self.read_char(); // skip '*'

        let mut nesting_level = 1;

        while nesting_level > 0 && self.ch != 0 {
            if self.ch == b'/' && self.peek_char() == b'*' {
                nesting_level += 1;
                self.read_char();
                self.read_char();
            } else if self.ch == b'*' && self.peek_char() == b'/' {
                nesting_level -= 1;
                self.read_char();
                self.read_char();
            } else {
                self.read_char();
            }
        }

        if nesting_level > 0 {
            return Err(LexerError::UnterminatedComment(start_line));
        }

        Ok(self.input[start_pos..self.position].to_string())
    }

    /// Lookup identifier type (keyword, builtin, or identifier)
    fn lookup_identifier_type(&self, identifier: &str) -> TokenType {
        // Check keywords first
        if let Some(token_type) = self.keywords.get(identifier) {
            return token_type.clone();
        }

        // Check built-in functions
        if let Some(category) = self.builtin_functions.get(identifier) {
            return match category {
                BuiltinCategory::Arithmetic => TokenType::Arithmetic(identifier.to_string()),
                BuiltinCategory::Comparison => TokenType::Comparison(identifier.to_string()),
                BuiltinCategory::Bitwise => TokenType::Bitwise(identifier.to_string()),
                BuiltinCategory::Memory => TokenType::Memory(identifier.to_string()),
                BuiltinCategory::Storage => TokenType::Storage(identifier.to_string()),
                BuiltinCategory::Environment => TokenType::Environment(identifier.to_string()),
                BuiltinCategory::Control => TokenType::Control(identifier.to_string()),
                BuiltinCategory::Crypto => TokenType::Crypto(identifier.to_string()),
                BuiltinCategory::Block => TokenType::Block(identifier.to_string()),
                BuiltinCategory::Transaction => TokenType::Transaction(identifier.to_string()),
            };
        }

        // Regular identifier
        TokenType::Identifier
    }
}

impl Default for YulLexer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Illegal => write!(f, "ILLEGAL"),
            TokenType::Eof => write!(f, "EOF"),
            TokenType::Identifier => write!(f, "IDENTIFIER"),
            TokenType::Number => write!(f, "NUMBER"),
            TokenType::String => write!(f, "STRING"),
            TokenType::HexNumber => write!(f, "HEX_NUMBER"),
            TokenType::Boolean => write!(f, "BOOLEAN"),
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Colon => write!(f, ":"),
            TokenType::Dot => write!(f, "."),
            TokenType::Assign => write!(f, ":="),
            TokenType::Arrow => write!(f, "->"),
            TokenType::Object => write!(f, "object"),
            TokenType::Code => write!(f, "code"),
            TokenType::Data => write!(f, "data"),
            TokenType::Function => write!(f, "function"),
            TokenType::Let => write!(f, "let"),
            TokenType::If => write!(f, "if"),
            TokenType::Switch => write!(f, "switch"),
            TokenType::Case => write!(f, "case"),
            TokenType::Default => write!(f, "default"),
            TokenType::For => write!(f, "for"),
            TokenType::Break => write!(f, "break"),
            TokenType::Continue => write!(f, "continue"),
            TokenType::Leave => write!(f, "leave"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::Arithmetic(name) => write!(f, "ARITHMETIC({})", name),
            TokenType::Comparison(name) => write!(f, "COMPARISON({})", name),
            TokenType::Bitwise(name) => write!(f, "BITWISE({})", name),
            TokenType::Memory(name) => write!(f, "MEMORY({})", name),
            TokenType::Storage(name) => write!(f, "STORAGE({})", name),
            TokenType::Environment(name) => write!(f, "ENVIRONMENT({})", name),
            TokenType::Control(name) => write!(f, "CONTROL({})", name),
            TokenType::Crypto(name) => write!(f, "CRYPTO({})", name),
            TokenType::Block(name) => write!(f, "BLOCK({})", name),
            TokenType::Transaction(name) => write!(f, "TRANSACTION({})", name),
            TokenType::Comment => write!(f, "COMMENT"),
            TokenType::Whitespace => write!(f, "WHITESPACE"),
            TokenType::Newline => write!(f, "NEWLINE"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: '{}' at {}:{}", 
               self.token_type, self.literal, 
               self.position.line, self.position.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = YulLexer::new();
        let input = "{ ( ) , : := -> }";
        let tokens = lexer.tokenize(input).unwrap();

        let expected = vec![
            TokenType::LeftBrace,
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::Comma,
            TokenType::Colon,
            TokenType::Assign,
            TokenType::Arrow,
            TokenType::RightBrace,
            TokenType::Eof,
        ];

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.token_type, expected[i]);
        }
    }

    #[test]
    fn test_keywords() {
        let mut lexer = YulLexer::new();
        let input = "object function let if true false";
        let tokens = lexer.tokenize(input).unwrap();

        assert!(matches!(tokens[0].token_type, TokenType::Object));
        assert!(matches!(tokens[1].token_type, TokenType::Function));
        assert!(matches!(tokens[2].token_type, TokenType::Let));
        assert!(matches!(tokens[3].token_type, TokenType::If));
        assert!(matches!(tokens[4].token_type, TokenType::True));
        assert!(matches!(tokens[5].token_type, TokenType::False));
    }

    #[test]
    fn test_builtin_functions() {
        let mut lexer = YulLexer::new();
        let input = "add sub mload sstore keccak256";
        let tokens = lexer.tokenize(input).unwrap();

        assert!(matches!(tokens[0].token_type, TokenType::Arithmetic(_)));
        assert!(matches!(tokens[1].token_type, TokenType::Arithmetic(_)));
        assert!(matches!(tokens[2].token_type, TokenType::Memory(_)));
        assert!(matches!(tokens[3].token_type, TokenType::Storage(_)));
        assert!(matches!(tokens[4].token_type, TokenType::Crypto(_)));
    }

    #[test]
    fn test_numbers() {
        let mut lexer = YulLexer::new();
        let input = "123 0x456 0";
        let tokens = lexer.tokenize(input).unwrap();

        assert!(matches!(tokens[0].token_type, TokenType::Number));
        assert_eq!(tokens[0].literal, "123");
        
        assert!(matches!(tokens[1].token_type, TokenType::HexNumber));
        assert_eq!(tokens[1].literal, "0x456");
        
        assert!(matches!(tokens[2].token_type, TokenType::Number));
        assert_eq!(tokens[2].literal, "0");
    }

    #[test]
    fn test_string_literals() {
        let mut lexer = YulLexer::new();
        let input = r#""hello" "world\n" "hex: \x41""#;
        let tokens = lexer.tokenize(input).unwrap();

        assert!(matches!(tokens[0].token_type, TokenType::String));
        assert_eq!(tokens[0].literal, "hello");
        
        assert!(matches!(tokens[1].token_type, TokenType::String));
        assert_eq!(tokens[1].literal, "world\n");
        
        assert!(matches!(tokens[2].token_type, TokenType::String));
        assert_eq!(tokens[2].literal, "hex: A");
    }

    #[test]
    fn test_comments() {
        let mut lexer = YulLexer::new();
        let input = "// line comment\n/* block comment */";
        let tokens = lexer.tokenize(input).unwrap();

        assert!(matches!(tokens[0].token_type, TokenType::Comment));
        assert!(matches!(tokens[1].token_type, TokenType::Comment));
    }

    #[test]
    fn test_error_cases() {
        let mut lexer = YulLexer::new();
        
        // Unterminated string
        assert!(lexer.tokenize(r#""unterminated"#).is_err());
        
        // Invalid hex number
        assert!(lexer.tokenize("0xGHI").is_err());
        
        // Unexpected character
        assert!(lexer.tokenize("@").is_err());
    }

    #[test]
    fn test_position_tracking() {
        let mut lexer = YulLexer::new();
        let input = "function\nadd(1, 2)";
        let tokens = lexer.tokenize(input).unwrap();

        assert_eq!(tokens[0].position.line, 1);
        assert_eq!(tokens[0].position.column, 1);
        
        // 'add' should be on line 2
        assert_eq!(tokens[1].position.line, 2);
        assert_eq!(tokens[1].position.column, 1);
    }
}