impl Lexer {
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
}

