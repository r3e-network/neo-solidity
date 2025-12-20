impl Lexer {
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
}

