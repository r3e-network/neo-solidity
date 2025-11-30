//! Yul Lexer Module
//!
//! Lexical analyzer for the Yul intermediate language. Converts Yul source code
//! into a stream of tokens for parsing.
//!
//! # Supported Tokens
//!
//! - Keywords: `let`, `if`, `else`, `for`, `switch`, `case`, `default`, `function`, etc.
//! - Operators: `:=`, `->`, `+`, `-`
//! - Literals: decimal numbers, hex numbers (`0x...`), strings
//! - Identifiers: variable and function names
//! - Built-in functions: `add`, `sub`, `mul`, `div`, `mload`, `sstore`, etc.
//!
//! # Example
//!
//! ```ignore
//! use neo_solidity::lexer::Lexer;
//!
//! let mut lexer = Lexer::new("let x := add(1, 2)");
//! let tokens = lexer.tokenize()?;
//! ```

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
                    if self.peek() == '>' {
                        tokens.push(self.make_token(TokenType::Arrow, "->"));
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(self.make_token(TokenType::Minus, "-"));
                        self.advance();
                    }
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
            "else" => TokenType::Else,
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

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Basic Token Tests ====================

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_whitespace_only() {
        let mut lexer = Lexer::new("   \t\n  ");
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_braces() {
        let mut lexer = Lexer::new("{ }");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[1].token_type, TokenType::RightBrace);
    }

    #[test]
    fn test_parentheses() {
        let mut lexer = Lexer::new("( )");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[1].token_type, TokenType::RightParen);
    }

    #[test]
    fn test_comma() {
        let mut lexer = Lexer::new(",");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Comma);
    }

    // ==================== Operator Tests ====================

    #[test]
    fn test_plus_operator() {
        let mut lexer = Lexer::new("+");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Plus);
    }

    #[test]
    fn test_minus_operator() {
        let mut lexer = Lexer::new("-");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Minus);
    }

    #[test]
    fn test_arrow_operator() {
        let mut lexer = Lexer::new("->");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Arrow);
        assert_eq!(tokens[0].value, "->");
    }

    #[test]
    fn test_assignment_operator() {
        let mut lexer = Lexer::new(":=");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Assignment);
        assert_eq!(tokens[0].value, ":=");
    }

    // ==================== Keyword Tests ====================

    #[test]
    fn test_let_keyword() {
        let mut lexer = Lexer::new("let");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Let);
    }

    #[test]
    fn test_if_keyword() {
        let mut lexer = Lexer::new("if");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::If);
    }

    #[test]
    fn test_else_keyword() {
        let mut lexer = Lexer::new("else");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Else);
    }

    #[test]
    fn test_for_keyword() {
        let mut lexer = Lexer::new("for");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::For);
    }

    #[test]
    fn test_switch_keyword() {
        let mut lexer = Lexer::new("switch");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Switch);
    }

    #[test]
    fn test_case_keyword() {
        let mut lexer = Lexer::new("case");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Case);
    }

    #[test]
    fn test_default_keyword() {
        let mut lexer = Lexer::new("default");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Default);
    }

    #[test]
    fn test_leave_keyword() {
        let mut lexer = Lexer::new("leave");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Leave);
    }

    #[test]
    fn test_break_keyword() {
        let mut lexer = Lexer::new("break");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Break);
    }

    #[test]
    fn test_continue_keyword() {
        let mut lexer = Lexer::new("continue");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Continue);
    }

    #[test]
    fn test_function_keyword() {
        let mut lexer = Lexer::new("function");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Function);
    }

    // ==================== Builtin Function Tests ====================

    #[test]
    fn test_arithmetic_builtins() {
        let builtins = ["add", "sub", "mul", "div", "mod"];
        for builtin in builtins {
            let mut lexer = Lexer::new(builtin);
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
            assert_eq!(tokens[0].value, builtin);
        }
    }

    #[test]
    fn test_comparison_builtins() {
        let builtins = ["eq", "lt", "gt", "iszero"];
        for builtin in builtins {
            let mut lexer = Lexer::new(builtin);
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
        }
    }

    #[test]
    fn test_bitwise_builtins() {
        let builtins = ["and", "or", "xor", "not", "shl", "shr", "sar"];
        for builtin in builtins {
            let mut lexer = Lexer::new(builtin);
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
        }
    }

    #[test]
    fn test_memory_builtins() {
        let builtins = ["mload", "mstore"];
        for builtin in builtins {
            let mut lexer = Lexer::new(builtin);
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
        }
    }

    #[test]
    fn test_storage_builtins() {
        let builtins = ["sload", "sstore"];
        for builtin in builtins {
            let mut lexer = Lexer::new(builtin);
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
        }
    }

    #[test]
    fn test_crypto_builtins() {
        let builtins = ["keccak256", "sha256", "ripemd160", "ecrecover"];
        for builtin in builtins {
            let mut lexer = Lexer::new(builtin);
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
        }
    }

    #[test]
    fn test_context_builtins() {
        let builtins = ["caller", "callvalue", "gas", "origin", "address", "balance"];
        for builtin in builtins {
            let mut lexer = Lexer::new(builtin);
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
        }
    }

    #[test]
    fn test_block_builtins() {
        let builtins = ["timestamp", "number", "blockhash", "coinbase"];
        for builtin in builtins {
            let mut lexer = Lexer::new(builtin);
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
        }
    }

    #[test]
    fn test_log_builtins() {
        let builtins = ["log0", "log1", "log2", "log3", "log4"];
        for builtin in builtins {
            let mut lexer = Lexer::new(builtin);
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
        }
    }

    #[test]
    fn test_call_builtins() {
        let builtins = ["call", "callcode", "delegatecall", "staticcall"];
        for builtin in builtins {
            let mut lexer = Lexer::new(builtin);
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
        }
    }

    // ==================== Identifier Tests ====================

    #[test]
    fn test_simple_identifier() {
        let mut lexer = Lexer::new("foo");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "foo");
    }

    #[test]
    fn test_identifier_with_underscore() {
        let mut lexer = Lexer::new("_bar");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "_bar");
    }

    #[test]
    fn test_identifier_with_numbers() {
        let mut lexer = Lexer::new("baz123");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "baz123");
    }

    // ==================== Number Literal Tests ====================

    #[test]
    fn test_decimal_number() {
        let mut lexer = Lexer::new("42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Literal);
        assert_eq!(tokens[0].value, "42");
    }

    #[test]
    fn test_zero() {
        let mut lexer = Lexer::new("0");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Literal);
        assert_eq!(tokens[0].value, "0");
    }

    #[test]
    fn test_hex_number_lowercase() {
        let mut lexer = Lexer::new("0xdeadbeef");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Literal);
        assert_eq!(tokens[0].value, "0xdeadbeef");
    }

    #[test]
    fn test_hex_number_uppercase() {
        let mut lexer = Lexer::new("0XABCDEF");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Literal);
        assert_eq!(tokens[0].value, "0xABCDEF");
    }

    #[test]
    fn test_hex_number_mixed_case() {
        let mut lexer = Lexer::new("0x123AbC");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Literal);
        assert_eq!(tokens[0].value, "0x123AbC");
    }

    // ==================== String Literal Tests ====================

    #[test]
    fn test_simple_string() {
        let mut lexer = Lexer::new("\"hello\"");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Literal);
        assert_eq!(tokens[0].value, "\"hello\"");
    }

    #[test]
    fn test_empty_string() {
        let mut lexer = Lexer::new("\"\"");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Literal);
        assert_eq!(tokens[0].value, "\"\"");
    }

    #[test]
    fn test_string_with_escape() {
        let mut lexer = Lexer::new("\"hello\\nworld\"");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Literal);
        assert_eq!(tokens[0].value, "\"hello\\nworld\"");
    }

    #[test]
    fn test_unterminated_string() {
        let mut lexer = Lexer::new("\"hello");
        let result = lexer.tokenize();
        assert!(result.is_err());
    }

    // ==================== Comment Tests ====================

    #[test]
    fn test_line_comment() {
        let mut lexer = Lexer::new("// this is a comment\nfoo");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "foo");
    }

    #[test]
    fn test_comment_at_end() {
        let mut lexer = Lexer::new("foo // comment");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
    }

    // ==================== Position Tracking Tests ====================

    #[test]
    fn test_line_tracking() {
        let mut lexer = Lexer::new("a\nb\nc");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[1].line, 2);
        assert_eq!(tokens[2].line, 3);
    }

    #[test]
    fn test_column_tracking() {
        let mut lexer = Lexer::new("abc def");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[1].column, 5);
    }

    // ==================== Complex Expression Tests ====================

    #[test]
    fn test_variable_declaration() {
        let mut lexer = Lexer::new("let x := 42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Let);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Assignment);
        assert_eq!(tokens[3].token_type, TokenType::Literal);
    }

    #[test]
    fn test_function_call() {
        let mut lexer = Lexer::new("add(1, 2)");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
        assert_eq!(tokens[1].token_type, TokenType::LeftParen);
        assert_eq!(tokens[2].token_type, TokenType::Literal);
        assert_eq!(tokens[3].token_type, TokenType::Comma);
        assert_eq!(tokens[4].token_type, TokenType::Literal);
        assert_eq!(tokens[5].token_type, TokenType::RightParen);
    }

    #[test]
    fn test_function_definition() {
        // function foo(a, b) -> result { }
        // tokens: function, foo, (, a, ,, b, ), ->, result, {, }
        let mut lexer = Lexer::new("function foo(a, b) -> result { }");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0].token_type, TokenType::Function);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[7].token_type, TokenType::Arrow);
    }

    #[test]
    fn test_if_statement() {
        let mut lexer = Lexer::new("if eq(x, 0) { leave }");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::If);
        assert_eq!(tokens[1].token_type, TokenType::BuiltinFunction);
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Leave));
    }

    #[test]
    fn test_for_loop() {
        let mut lexer = Lexer::new("for { let i := 0 } lt(i, 10) { i := add(i, 1) } { }");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::For);
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Let));
        assert!(tokens
            .iter()
            .any(|t| t.token_type == TokenType::BuiltinFunction && t.value == "lt"));
    }

    #[test]
    fn test_switch_statement() {
        let mut lexer = Lexer::new("switch x case 0 { } case 1 { } default { }");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Switch);
        assert_eq!(
            tokens
                .iter()
                .filter(|t| t.token_type == TokenType::Case)
                .count(),
            2
        );
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Default));
    }

    // ==================== Error Handling Tests ====================

    #[test]
    fn test_unexpected_character() {
        let mut lexer = Lexer::new("@");
        let result = lexer.tokenize();
        assert!(result.is_err());
    }

    #[test]
    fn test_error_position() {
        let mut lexer = Lexer::new("foo @");
        let result = lexer.tokenize();
        assert!(result.is_err());
        if let Err(e) = result {
            let msg = format!("{}", e);
            assert!(msg.contains("line"));
            assert!(msg.contains("column"));
        }
    }
}
