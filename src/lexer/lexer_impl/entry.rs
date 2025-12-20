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
}

