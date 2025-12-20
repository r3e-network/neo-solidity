impl Lexer {
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
}

