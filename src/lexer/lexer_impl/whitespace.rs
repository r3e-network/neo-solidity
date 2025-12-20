impl Lexer {
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
}

