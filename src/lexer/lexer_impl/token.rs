impl Lexer {
    fn make_token(&self, token_type: TokenType, value: &str) -> Token {
        Token {
            token_type,
            value: value.to_string(),
            line: self.line,
            column: self.column,
        }
    }
}

