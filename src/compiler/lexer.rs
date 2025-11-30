//! 词法分析器 - 将Solidity源代码转换为Token流
//!
//! 支持完整的Solidity 0.8.x语法，包括：
//! - 基本类型和运算符
//! - 控制流结构
//! - 函数定义和调用
//! - 内置函数和关键字
//! - 注释和文档

use crate::error::CompilerError;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // 关键字
    Pragma,
    Contract,
    Library,
    Interface,
    Function,
    Modifier,
    Event,
    Struct,
    Enum,
    Constructor,
    Using,
    If,
    Else,
    For,
    While,
    Do,
    Break,
    Continue,
    Return,
    Revert,
    Emit,
    Try,
    Catch,
    Throw,
    DelegateCall,

    // 类型相关
    Address,
    Bool,
    String,
    Bytes,
    Int,
    Uint,
    Fixed,
    Ufixed,

    // 可见性和修饰符
    Public,
    Private,
    Internal,
    External,
    Pure,
    View,
    Payable,
    Memory,
    Storage,
    Calldata,
    Constant,
    Immutable,

    // 字面量
    Identifier,
    Literal(LiteralType),
    StringLiteral,

    // 运算符
    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
    Modulo,         // %
    Power,          // **
    Assign,         // =
    PlusAssign,     // +=
    MinusAssign,    // -=
    MultiplyAssign, // *=
    DivideAssign,   // /=
    ModuloAssign,   // %=

    // 比较运算符
    Equal,          // ==
    NotEqual,       // !=
    LessThan,       // <
    GreaterThan,    // >
    LessThanOrEq,   // <=
    GreaterThanOrEq,// >=

    // 逻辑运算符
    And,            // &&
    Or,             // ||
    Not,            // !
    BitAnd,         // &
    BitOr,          // |
    BitXor,         // ^
    BitNot,         // ~
    LeftShift,      // <<
    RightShift,     // >>

    // 分隔符
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    Semicolon,      // ;
    Colon,          // :
    DoubleColon,    // ::
    Dot,            // .
    Comma,          // ,
    Question,       // ?
    Arrow,          // =>

    // 特殊符号
    Dollar,         // $
    At,             // @
    Hash,           // #

    // 内置函数和变量
    BuiltinFunction,
    BuiltinVariable,

    // 特殊token
    EndOfFile,
    Whitespace,
    Comment,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralType {
    Integer,
    Hexadecimal,
    Boolean,
    String,
    Address,
    Rational,
}

pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
    tokens: VecDeque<Token>,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.to_string(),
            position: 0,
            line: 1,
            column: 1,
            tokens: VecDeque::new(),
            current_char: input.chars().next(),
        };
        lexer.tokenize_all().expect("Tokenization should succeed");
        lexer
    }

    fn tokenize_all(&mut self) -> Result<(), CompilerError> {
        while !self.is_at_end() {
            if let Some(token) = self.scan_token()? {
                self.tokens.push_back(token);
            }
        }

        // 添加EOF标记
        self.tokens.push_back(Token {
            token_type: TokenType::EndOfFile,
            value: String::new(),
            line: self.line,
            column: self.column,
            source_span: SourceSpan {
                start: self.position,
                end: self.position,
            },
        });

        Ok(())
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.front()
    }

    pub fn peek_next(&self) -> Option<&Token> {
        if self.tokens.len() >= 2 {
            self.tokens.get(1)
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    pub fn expect(&mut self, expected_type: TokenType) -> Result<Token, CompilerError> {
        match self.next() {
            Some(token) if token.token_type == expected_type => Ok(token),
            Some(token) => Err(CompilerError::ParseError(format!(
                "Expected {:?}, found {:?} at line {}, column {}",
                expected_type, token.token_type, token.line, token.column
            ))),
            None => Err(CompilerError::ParseError(format!(
                "Expected {:?}, but reached end of input",
                expected_type
            ))),
        }
    }

    fn scan_token(&mut self) -> Result<Option<Token>, CompilerError> {
        let start_line = self.line;
        let start_column = self.column;
        let start_position = self.position;

        match self.current_char {
            Some('(') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::LeftParen, "(", start_line, start_column, start_position)))
            }
            Some(')') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::RightParen, ")", start_line, start_column, start_position)))
            }
            Some('{') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::LeftBrace, "{", start_line, start_column, start_position)))
            }
            Some('}') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::RightBrace, "}", start_line, start_column, start_position)))
            }
            Some('[') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::LeftBracket, "[", start_line, start_column, start_position)))
            }
            Some(']') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::RightBracket, "]", start_line, start_column, start_position)))
            }
            Some(';') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::Semicolon, ";", start_line, start_column, start_position)))
            }
            Some(':') => {
                if self.match_char(':') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::DoubleColon, "::", start_line, start_column, start_position)))
                } else {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::Colon, ":", start_line, start_column, start_position)))
                }
            }
            Some('.') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::Dot, ".", start_line, start_column, start_position)))
            }
            Some(',') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::Comma, ",", start_line, start_column, start_position)))
            }
            Some('?') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::Question, "?", start_line, start_column, start_position)))
            }
            Some('+') => {
                if self.match_char('=') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::PlusAssign, "+=", start_line, start_column, start_position)))
                } else {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::Plus, "+", start_line, start_column, start_position)))
                }
            }
            Some('-') => {
                if self.match_char('=') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::MinusAssign, "-=", start_line, start_column, start_position)))
                } else if self.match_char('>') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::Arrow, "->", start_line, start_column, start_position)))
                } else {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::Minus, "-", start_line, start_column, start_position)))
                }
            }
            Some('*') => {
                if self.match_char('=') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::MultiplyAssign, "*=", start_line, start_column, start_position)))
                } else if self.match_char('*') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::Power, "**", start_line, start_column, start_position)))
                } else {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::Multiply, "*", start_line, start_column, start_position)))
                }
            }
            Some('/') => {
                if self.match_char('/') {
                    // 单行注释
                    self.advance();
                    self.skip_line_comment();
                    Ok(None)
                } else if self.match_char('*') {
                    // 多行注释
                    self.advance();
                    self.skip_block_comment()?;
                    Ok(None)
                } else if self.match_char('=') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::DivideAssign, "/=", start_line, start_column, start_position)))
                } else {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::Divide, "/", start_line, start_column, start_position)))
                }
            }
            Some('%') => {
                if self.match_char('=') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::ModuloAssign, "%=", start_line, start_column, start_position)))
                } else {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::Modulo, "%", start_line, start_column, start_position)))
                }
            }
            Some('=') => {
                if self.match_char('=') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::Equal, "==", start_line, start_column, start_position)))
                } else {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::Assign, "=", start_line, start_column, start_position)))
                }
            }
            Some('!') => {
                if self.match_char('=') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::NotEqual, "!=", start_line, start_column, start_position)))
                } else {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::Not, "!", start_line, start_column, start_position)))
                }
            }
            Some('<') => {
                if self.match_char('=') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::LessThanOrEq, "<=", start_line, start_column, start_position)))
                } else if self.match_char('<') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::LeftShift, "<<", start_line, start_column, start_position)))
                } else {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::LessThan, "<", start_line, start_column, start_position)))
                }
            }
            Some('>') => {
                if self.match_char('=') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::GreaterThanOrEq, ">=", start_line, start_column, start_position)))
                } else if self.match_char('>') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::RightShift, ">>", start_line, start_column, start_position)))
                } else {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::GreaterThan, ">", start_line, start_column, start_position)))
                }
            }
            Some('&') => {
                if self.match_char('=') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::BitAnd, "&=", start_line, start_column, start_position)))
                } else if self.match_char('&') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::And, "&&", start_line, start_column, start_position)))
                } else {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::BitAnd, "&", start_line, start_column, start_position)))
                }
            }
            Some('|') => {
                if self.match_char('=') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::BitOr, "|=", start_line, start_column, start_position)))
                } else if self.match_char('|') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::Or, "||", start_line, start_column, start_position)))
                } else {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::BitOr, "|", start_line, start_column, start_position)))
                }
            }
            Some('^') => {
                if self.match_char('=') {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::BitXor, "^=", start_line, start_column, start_position)))
                } else {
                    self.advance();
                    Ok(Some(self.make_token(TokenType::BitXor, "^", start_line, start_column, start_position)))
                }
            }
            Some('~') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::BitNot, "~", start_line, start_column, start_position)))
            }
            Some('"') => {
                let value = self.scan_string()?;
                Ok(Some(self.make_token(TokenType::StringLiteral, &value, start_line, start_column, start_position)))
            }
            Some('\'') => {
                let value = self.scan_character()?;
                Ok(Some(self.make_token(TokenType::Literal(LiteralType::Integer), &value, start_line, start_column, start_position)))
            }
            Some(c) if c.is_ascii_digit() => {
                let (value, literal_type) = self.scan_number()?;
                Ok(Some(self.make_token(TokenType::Literal(literal_type), &value, start_line, start_column, start_position)))
            }
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                let value = self.scan_identifier()?;
                let token_type = self.identifier_to_token_type(&value);
                Ok(Some(self.make_token(token_type, &value, start_line, start_column, start_position)))
            }
            Some(c) if c.is_whitespace() => {
                self.skip_whitespace();
                Ok(None)
            }
            Some(c) => {
                Err(CompilerError::ParseError(format!(
                    "Unexpected character '{}' at line {}, column {}",
                    c, start_line, start_column
                )))
            }
            None => Ok(None),
        }
    }

    fn identifier_to_token_type(&self, identifier: &str) -> TokenType {
        match identifier {
            // 编译指令
            "pragma" => TokenType::Pragma,

            // 合约定义
            "contract" => TokenType::Contract,
            "library" => TokenType::Library,
            "interface" => TokenType::Interface,

            // 函数和事件
            "function" => TokenType::Function,
            "modifier" => TokenType::Modifier,
            "event" => TokenType::Event,
            "constructor" => TokenType::Constructor,

            // 类型定义
            "struct" => TokenType::Struct,
            "enum" => TokenType::Enum,

            // 控制流
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "do" => TokenType::Do,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "return" => TokenType::Return,
            "revert" => TokenType::Revert,
            "emit" => TokenType::Emit,
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "throw" => TokenType::Throw,

            // 类型
            "address" => TokenType::Address,
            "bool" => TokenType::Bool,
            "string" => TokenType::String,
            "bytes" => TokenType::Bytes,
            "int" => TokenType::Int,
            "uint" => TokenType::Uint,
            "fixed" => TokenType::Fixed,
            "ufixed" => TokenType::Ufixed,

            // 可见性和修饰符
            "public" => TokenType::Public,
            "private" => TokenType::Private,
            "internal" => TokenType::Internal,
            "external" => TokenType::External,
            "pure" => TokenType::Pure,
            "view" => TokenType::View,
            "payable" => TokenType::Payable,
            "memory" => TokenType::Memory,
            "storage" => TokenType::Storage,
            "calldata" => TokenType::Calldata,
            "constant" => TokenType::Constant,
            "immutable" => TokenType::Immutable,

            // 内置变量
            "msg" | "block" | "tx" | "this" | "super" => TokenType::BuiltinVariable,

            // 内置函数
            "require" | "assert" | "revert" | "keccak256" | "sha256" | "ripemd160"
            | "ecrecover" | "addmod" | "mulmod" | "selfdestruct" | "suicide"
            | "delegatecall" | "call" | "staticcall" | "send" | "transfer"
            | "balance" | "blockhash" | "now" | "gasleft" => TokenType::BuiltinFunction,

            _ => TokenType::Identifier,
        }
    }

    fn scan_string(&mut self) -> Result<String, CompilerError> {
        let mut value = String::new();
        self.advance(); // Skip opening quote

        while let Some(c) = self.current_char {
            match c {
                '"' => {
                    self.advance(); // Skip closing quote
                    return Ok(value);
                }
                '\\' => {
                    self.advance();
                    if let Some(escaped) = self.current_char {
                        match escaped {
                            'n' => value.push('\n'),
                            't' => value.push('\t'),
                            'r' => value.push('\r'),
                            '\\' => value.push('\\'),
                            '"' => value.push('"'),
                            '\'' => value.push('\''),
                            'x' => {
                                self.advance();
                                let hex1 = self.current_char.ok_or_else(|| {
                                    CompilerError::ParseError("Invalid hex escape sequence".to_string())
                                })?;
                                self.advance();
                                let hex2 = self.current_char.ok_or_else(|| {
                                    CompilerError::ParseError("Invalid hex escape sequence".to_string())
                                })?;
                                let hex_str = format!("{}{}", hex1, hex2);
                                let code_point = u8::from_str_radix(&hex_str, 16).map_err(|_| {
                                    CompilerError::ParseError("Invalid hex escape sequence".to_string())
                                })?;
                                value.push(code_point as char);
                            }
                            _ => {
                                return Err(CompilerError::ParseError(format!(
                                    "Invalid escape sequence '\\{}'",
                                    escaped
                                )));
                            }
                        }
                        self.advance();
                    }
                }
                _ => {
                    value.push(c);
                    self.advance();
                }
            }
        }

        Err(CompilerError::ParseError("Unterminated string literal".to_string()))
    }

    fn scan_character(&mut self) -> Result<String, CompilerError> {
        self.advance(); // Skip opening quote
        let c = self.current_char.ok_or_else(|| {
            CompilerError::ParseError("Empty character literal".to_string())
        })?;

        let value = if c == '\\' {
            self.advance();
            match self.current_char {
                Some('n') => '\n',
                Some('t') => '\t',
                Some('r') => '\r',
                Some('\\') => '\\',
                Some('\'') => '\'',
                Some('"') => '"',
                _ => {
                    return Err(CompilerError::ParseError("Invalid character escape".to_string()));
                }
            }
        } else {
            c
        };

        self.advance(); // Skip character
        if self.current_char != Some('\'') {
            return Err(CompilerError::ParseError("Unterminated character literal".to_string()));
        }
        self.advance(); // Skip closing quote

        Ok((value as u32).to_string())
    }

    fn scan_number(&mut self) -> Result<(String, LiteralType), CompilerError> {
        let mut value = String::new();
        let mut literal_type = LiteralType::Integer;

        // 处理十六进制数
        if self.current_char == Some('0') && self.peek_char() == Some('x') {
            value.push('0');
            self.advance();
            value.push('x');
            self.advance();

            while let Some(c) = self.current_char {
                if c.is_ascii_hexdigit() {
                    value.push(c);
                    self.advance();
                } else {
                    break;
                }
            }

            if value.len() == 2 {
                return Err(CompilerError::ParseError("Invalid hexadecimal literal".to_string()));
            }

            return Ok((value, LiteralType::Hexadecimal));
        }

        // 处理十进制或浮点数
        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                value.push(c);
                self.advance();
            } else if c == '.' {
                literal_type = LiteralType::Rational;
                value.push(c);
                self.advance();

                // 小数部分
                while let Some(c) = self.current_char {
                    if c.is_ascii_digit() {
                        value.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }
            } else if c == 'e' || c == 'E' {
                literal_type = LiteralType::Rational;
                value.push(c);
                self.advance();

                if self.current_char == Some('+') || self.current_char == Some('-') {
                    value.push(self.current_char.unwrap());
                    self.advance();
                }

                // 指数部分
                while let Some(c) = self.current_char {
                    if c.is_ascii_digit() {
                        value.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        Ok((value, literal_type))
    }

    fn scan_identifier(&mut self) -> Result<String, CompilerError> {
        let mut identifier = String::new();

        while let Some(c) = self.current_char {
            if c.is_ascii_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Ok(identifier)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
                self.advance();
            } else if c.is_whitespace() {
                self.column += 1;
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) {
        while let Some(c) = self.current_char {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) -> Result<(), CompilerError> {
        while let Some(c) = self.current_char {
            if c == '*' && self.peek_char() == Some('/') {
                self.advance();
                self.advance();
                return Ok(());
            } else if c == '\n' {
                self.line += 1;
                self.column = 1;
                self.advance();
            } else {
                self.advance();
            }
        }

        Err(CompilerError::ParseError("Unterminated block comment".to_string()))
    }

    fn advance(&mut self) {
        if self.current_char.is_some() {
            self.position += 1;
            self.column += 1;
            self.current_char = self.input.chars().nth(self.position);
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.current_char == Some(expected) {
            true
        } else {
            false
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn make_token(&self, token_type: TokenType, value: &str, line: usize, column: usize, start_pos: usize) -> Token {
        Token {
            token_type,
            value: value.to_string(),
            line,
            column,
            source_span: SourceSpan {
                start: start_pos,
                end: self.position,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Basic Token Tests ====================

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("function test() { return 42; }");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Function);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::LeftParen);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::RightParen);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::LeftBrace);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Return);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Literal(LiteralType::Integer));
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Semicolon);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::RightBrace);
    }

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next().unwrap().token_type, TokenType::EndOfFile);
    }

    #[test]
    fn test_whitespace_only() {
        let mut lexer = Lexer::new("   \t\n  ");
        assert_eq!(lexer.next().unwrap().token_type, TokenType::EndOfFile);
    }

    // ==================== String Literal Tests ====================

    #[test]
    fn test_string_literals() {
        let mut lexer = Lexer::new("\"hello world\" \"escaped\\nstring\"");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::StringLiteral);
        assert_eq!(token.value, "hello world");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::StringLiteral);
        assert_eq!(token.value, "escaped\nstring");
    }

    #[test]
    fn test_string_escape_sequences() {
        let mut lexer = Lexer::new("\"\\t\\r\\\\\\\"\"");
        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::StringLiteral);
        assert_eq!(token.value, "\t\r\\\"");
    }

    #[test]
    fn test_string_hex_escape() {
        let mut lexer = Lexer::new("\"\\x41\\x42\"");
        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::StringLiteral);
        assert_eq!(token.value, "AB");
    }

    #[test]
    fn test_empty_string() {
        let mut lexer = Lexer::new("\"\"");
        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::StringLiteral);
        assert_eq!(token.value, "");
    }

    // ==================== Number Literal Tests ====================

    #[test]
    fn test_hexadecimal_literals() {
        let mut lexer = Lexer::new("0x123ABC 0xdeadbeef");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Literal(LiteralType::Hexadecimal));
        assert_eq!(token.value, "0x123ABC");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Literal(LiteralType::Hexadecimal));
        assert_eq!(token.value, "0xdeadbeef");
    }

    #[test]
    fn test_integer_literals() {
        let mut lexer = Lexer::new("0 42 123456789");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Literal(LiteralType::Integer));
        assert_eq!(token.value, "0");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Literal(LiteralType::Integer));
        assert_eq!(token.value, "42");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Literal(LiteralType::Integer));
        assert_eq!(token.value, "123456789");
    }

    #[test]
    fn test_rational_literals() {
        let mut lexer = Lexer::new("3.14 0.5 1e10 2.5e-3");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Literal(LiteralType::Rational));
        assert_eq!(token.value, "3.14");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Literal(LiteralType::Rational));
        assert_eq!(token.value, "0.5");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Literal(LiteralType::Rational));
        assert_eq!(token.value, "1e10");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Literal(LiteralType::Rational));
        assert_eq!(token.value, "2.5e-3");
    }

    #[test]
    fn test_character_literal() {
        let mut lexer = Lexer::new("'A'");
        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Literal(LiteralType::Integer));
        assert_eq!(token.value, "65"); // ASCII value of 'A'
    }

    // ==================== Comment Tests ====================

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("/* block comment */ x // line comment\n y");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier);
    }

    #[test]
    fn test_nested_block_comment_content() {
        let mut lexer = Lexer::new("/* comment with * inside */ x");
        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "x");
    }

    #[test]
    fn test_multiline_block_comment() {
        let mut lexer = Lexer::new("/*\n * multi\n * line\n */ x");
        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
    }

    // ==================== Operator Tests ====================

    #[test]
    fn test_arithmetic_operators() {
        let mut lexer = Lexer::new("+ - * / % **");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Plus);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Minus);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Multiply);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Divide);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Modulo);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Power);
    }

    #[test]
    fn test_comparison_operators() {
        let mut lexer = Lexer::new("== != < > <= >=");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Equal);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::NotEqual);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::LessThan);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::GreaterThan);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::LessThanOrEq);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::GreaterThanOrEq);
    }

    #[test]
    fn test_logical_operators() {
        let mut lexer = Lexer::new("&& || !");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::And);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Or);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Not);
    }

    #[test]
    fn test_bitwise_operators() {
        let mut lexer = Lexer::new("& | ^ ~ << >>");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::BitAnd);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::BitOr);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::BitXor);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::BitNot);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::LeftShift);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::RightShift);
    }

    #[test]
    fn test_assignment_operators() {
        let mut lexer = Lexer::new("= += -= *= /= %=");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Assign);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::PlusAssign);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::MinusAssign);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::MultiplyAssign);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::DivideAssign);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::ModuloAssign);
    }

    // ==================== Delimiter Tests ====================

    #[test]
    fn test_delimiters() {
        let mut lexer = Lexer::new("( ) { } [ ] ; : :: . , ? ->");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::LeftParen);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::RightParen);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::LeftBrace);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::RightBrace);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::LeftBracket);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::RightBracket);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Semicolon);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Colon);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::DoubleColon);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Dot);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Comma);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Question);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Arrow);
    }

    // ==================== Keyword Tests ====================

    #[test]
    fn test_contract_keywords() {
        let mut lexer = Lexer::new("contract library interface pragma");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Contract);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Library);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Interface);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Pragma);
    }

    #[test]
    fn test_function_keywords() {
        let mut lexer = Lexer::new("function modifier event constructor");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Function);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Modifier);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Event);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Constructor);
    }

    #[test]
    fn test_type_keywords() {
        let mut lexer = Lexer::new("address bool string bytes int uint");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Address);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Bool);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::String);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Bytes);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Int);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Uint);
    }

    #[test]
    fn test_visibility_keywords() {
        let mut lexer = Lexer::new("public private internal external");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Public);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Private);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Internal);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::External);
    }

    #[test]
    fn test_mutability_keywords() {
        let mut lexer = Lexer::new("pure view payable constant immutable");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Pure);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::View);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Payable);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Constant);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Immutable);
    }

    #[test]
    fn test_storage_keywords() {
        let mut lexer = Lexer::new("memory storage calldata");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Memory);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Storage);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Calldata);
    }

    #[test]
    fn test_control_flow_keywords() {
        let mut lexer = Lexer::new("if else for while do break continue return");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::If);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Else);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::For);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::While);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Do);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Break);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Continue);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Return);
    }

    #[test]
    fn test_error_handling_keywords() {
        let mut lexer = Lexer::new("revert emit try catch throw");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Revert);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Emit);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Try);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Catch);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Throw);
    }

    #[test]
    fn test_type_definition_keywords() {
        let mut lexer = Lexer::new("struct enum");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Struct);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Enum);
    }

    // ==================== Builtin Tests ====================

    #[test]
    fn test_builtin_variables() {
        let mut lexer = Lexer::new("msg block tx this super");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::BuiltinVariable);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::BuiltinVariable);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::BuiltinVariable);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::BuiltinVariable);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::BuiltinVariable);
    }

    #[test]
    fn test_builtin_functions() {
        let mut lexer = Lexer::new("require assert keccak256 sha256 ecrecover");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::BuiltinFunction);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::BuiltinFunction);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::BuiltinFunction);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::BuiltinFunction);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::BuiltinFunction);
    }

    // ==================== Identifier Tests ====================

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("foo _bar baz123 _123");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "foo");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "_bar");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "baz123");

        let token = lexer.next().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "_123");
    }

    // ==================== Position Tracking Tests ====================

    #[test]
    fn test_line_tracking() {
        let mut lexer = Lexer::new("a\nb\nc");

        let token = lexer.next().unwrap();
        assert_eq!(token.line, 1);

        let token = lexer.next().unwrap();
        assert_eq!(token.line, 2);

        let token = lexer.next().unwrap();
        assert_eq!(token.line, 3);
    }

    #[test]
    fn test_column_tracking() {
        let mut lexer = Lexer::new("abc def");

        let token = lexer.next().unwrap();
        assert_eq!(token.column, 1);

        let token = lexer.next().unwrap();
        assert_eq!(token.column, 5);
    }

    #[test]
    fn test_source_span() {
        let mut lexer = Lexer::new("function");
        let token = lexer.next().unwrap();
        assert_eq!(token.source_span.start, 0);
        assert_eq!(token.source_span.end, 8);
    }

    // ==================== Peek Tests ====================

    #[test]
    fn test_peek() {
        let lexer = Lexer::new("a b");
        let peeked = lexer.peek().unwrap();
        assert_eq!(peeked.value, "a");
    }

    #[test]
    fn test_peek_next() {
        let lexer = Lexer::new("a b c");
        let peeked = lexer.peek_next().unwrap();
        assert_eq!(peeked.value, "b");
    }

    #[test]
    fn test_peek_does_not_consume() {
        let mut lexer = Lexer::new("a b");
        let _ = lexer.peek();
        let _ = lexer.peek();
        let token = lexer.next().unwrap();
        assert_eq!(token.value, "a");
    }

    // ==================== Expect Tests ====================

    #[test]
    fn test_expect_success() {
        let mut lexer = Lexer::new("function");
        let result = lexer.expect(TokenType::Function);
        assert!(result.is_ok());
    }

    #[test]
    fn test_expect_failure() {
        let mut lexer = Lexer::new("contract");
        let result = lexer.expect(TokenType::Function);
        assert!(result.is_err());
    }

    // ==================== Complex Expression Tests ====================

    #[test]
    fn test_complex_expression() {
        let mut lexer = Lexer::new("a + b * c - d / e % f");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Plus);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Multiply);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Minus);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Divide);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Modulo);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier);
    }

    #[test]
    fn test_function_call() {
        let mut lexer = Lexer::new("transfer(to, amount)");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::BuiltinFunction);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::LeftParen);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Comma);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::RightParen);
    }

    #[test]
    fn test_array_access() {
        let mut lexer = Lexer::new("arr[0]");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::LeftBracket);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Literal(LiteralType::Integer));
        assert_eq!(lexer.next().unwrap().token_type, TokenType::RightBracket);
    }

    #[test]
    fn test_member_access() {
        let mut lexer = Lexer::new("msg.sender");

        assert_eq!(lexer.next().unwrap().token_type, TokenType::BuiltinVariable);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Dot);
        assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier);
    }

    // ==================== Full Contract Tests ====================

    #[test]
    fn test_simple_contract() {
        let source = r#"
            contract Token {
                uint256 public totalSupply;

                function transfer(address to, uint256 amount) public returns (bool) {
                    return true;
                }
            }
        "#;

        let mut lexer = Lexer::new(source);
        let mut token_count = 0;

        while let Some(token) = lexer.next() {
            if token.token_type == TokenType::EndOfFile {
                break;
            }
            token_count += 1;
        }

        assert!(token_count > 20);
    }
}