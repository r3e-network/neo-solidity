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

        while self.position < self.input.len() {
            self.skip_whitespace();

            if self.position >= self.input.len() {
                break;
            }

            let ch = self.current_char();

            match ch {
                '{' => {
                    tokens.push(self.make_token(TokenType::LeftBrace, "{"));
                    self.advance();
                }
                '}' => {
                    tokens.push(self.make_token(TokenType::RightBrace, "}"));
                    self.advance();
                }
                '(' => {
                    tokens.push(self.make_token(TokenType::LeftParen, "("));
                    self.advance();
                }
                ')' => {
                    tokens.push(self.make_token(TokenType::RightParen, ")"));
                    self.advance();
                }
                ',' => {
                    tokens.push(self.make_token(TokenType::Comma, ","));
                    self.advance();
                }
                '+' => {
                    tokens.push(self.make_token(TokenType::Plus, "+"));
                    self.advance();
                }
                '-' => {
                    tokens.push(self.make_token(TokenType::Minus, "-"));
                    self.advance();
                }
                ':' if self.peek() == '=' => {
                    tokens.push(self.make_token(TokenType::Assignment, ":="));
                    self.advance();
                    self.advance();
                }
                '"' => {
                    tokens.push(self.read_string()?);
                }
                '0'..='9' => {
                    tokens.push(self.read_number()?);
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    tokens.push(self.read_identifier());
                }
                _ => {
                    return Err(CompilerError::ParseError(format!(
                        "Unexpected character '{}' at line {}, column {}",
                        ch, self.line, self.column
                    )));
                }
            }
        }

        Ok(tokens)
    }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }

    fn peek(&self) -> char {
        self.input.chars().nth(self.position + 1).unwrap_or('\0')
    }

    fn advance(&mut self) {
        if self.position < self.input.len() && self.current_char() == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            match self.current_char() {
                ' ' | '\t' | '\r' | '\n' => self.advance(),
                '/' if self.peek() == '/' => {
                    // Skip line comment
                    while self.position < self.input.len() && self.current_char() != '\n' {
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn make_token(&self, token_type: TokenType, value: &str) -> Token {
        Token {
            token_type,
            value: value.to_string(),
            line: self.line,
            column: self.column,
        }
    }

    fn read_string(&mut self) -> Result<Token, CompilerError> {
        let start_line = self.line;
        let start_column = self.column;
        let mut value = String::from("\"");

        self.advance(); // Skip opening quote

        while self.position < self.input.len() && self.current_char() != '"' {
            if self.current_char() == '\\' {
                value.push(self.current_char());
                self.advance();
                if self.position < self.input.len() {
                    value.push(self.current_char());
                    self.advance();
                }
            } else {
                value.push(self.current_char());
                self.advance();
            }
        }

        if self.position >= self.input.len() {
            return Err(CompilerError::ParseError(
                "Unterminated string literal".to_string(),
            ));
        }

        value.push('"');
        self.advance(); // Skip closing quote

        Ok(Token {
            token_type: TokenType::Literal,
            value,
            line: start_line,
            column: start_column,
        })
    }

    fn read_number(&mut self) -> Result<Token, CompilerError> {
        let start_line = self.line;
        let start_column = self.column;
        let mut value = String::new();

        // Handle hex numbers
        if self.current_char() == '0' && (self.peek() == 'x' || self.peek() == 'X') {
            value.push_str("0x");
            self.advance();
            self.advance();

            while self.position < self.input.len() {
                match self.current_char() {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => {
                        value.push(self.current_char());
                        self.advance();
                    }
                    _ => break,
                }
            }
        } else {
            // Decimal number
            while self.position < self.input.len() && self.current_char().is_ascii_digit() {
                value.push(self.current_char());
                self.advance();
            }
        }

        Ok(Token {
            token_type: TokenType::Literal,
            value,
            line: start_line,
            column: start_column,
        })
    }

    fn read_identifier(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut value = String::new();

        while self.position < self.input.len() {
            match self.current_char() {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    value.push(self.current_char());
                    self.advance();
                }
                _ => break,
            }
        }

        let token_type = match value.as_str() {
            "let" => TokenType::Let,
            "if" => TokenType::If,
            "for" => TokenType::For,
            "switch" => TokenType::Switch,
            "case" => TokenType::Case,
            "default" => TokenType::Default,
            "leave" => TokenType::Leave,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "function" => TokenType::Function,
            // Built-in functions
            "add" | "sub" | "mul" | "div" | "mod" | "eq" | "lt" | "gt" | "iszero" | "and"
            | "or" | "xor" | "not" | "byte" | "shl" | "shr" | "sar" | "keccak256" | "sha256"
            | "ripemd160" | "ecrecover" | "mload" | "mstore" | "sload" | "sstore" | "caller"
            | "callvalue" | "calldataload" | "calldatasize" | "calldatacopy" | "gas"
            | "gasprice" | "gaslimit" | "origin" | "address" | "balance" | "selfbalance"
            | "basefee" | "chainid" | "timestamp" | "number" | "difficulty" | "blockhash"
            | "coinbase" | "log0" | "log1" | "log2" | "log3" | "log4" | "create" | "create2"
            | "call" | "callcode" | "delegatecall" | "staticcall" | "return" | "revert"
            | "selfdestruct" => TokenType::BuiltinFunction,
            _ => TokenType::Identifier,
        };

        Token {
            token_type,
            value,
            line: start_line,
            column: start_column,
        }
    }
}
