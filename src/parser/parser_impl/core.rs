impl Parser {
    /// Create a new parser from a token stream
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    /// Parse the token stream into an AST
    pub fn parse(&mut self) -> Result<AstNode, CompilerError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
        }

        Ok(AstNode {
            node_type: AstNodeType::Object { statements },
            line: 1,
            column: 1,
        })
    }

    fn parse_statement(&mut self) -> Result<Option<AstNode>, CompilerError> {
        if self.is_at_end() {
            return Ok(None);
        }

        match &self.current_token()?.token_type {
            TokenType::LeftBrace => self.parse_block(),
            TokenType::Let => self.parse_assignment(),
            TokenType::If => self.parse_if(),
            TokenType::For => self.parse_for(),
            TokenType::Switch => self.parse_switch(),
            TokenType::Function => self.parse_function(),
            TokenType::Identifier => self.parse_expression_statement(),
            _ => {
                self.advance()?;
                Ok(None)
            }
        }
    }

    // ========== Helper methods ==========

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn current_token(&self) -> Result<&Token, CompilerError> {
        self.tokens
            .get(self.position)
            .ok_or_else(|| self.error_eof())
    }

    fn advance(&mut self) -> Result<&Token, CompilerError> {
        if !self.is_at_end() {
            self.position += 1;
        }
        self.tokens
            .get(self.position - 1)
            .ok_or_else(|| self.error_eof())
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            matches!(self.tokens.get(self.position), Some(token) if &token.token_type == token_type)
        }
    }

    fn match_token(&mut self, token_type: &TokenType) -> Result<bool, CompilerError> {
        if self.check(token_type) {
            self.advance()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn consume(&mut self, token_type: TokenType) -> Result<&Token, CompilerError> {
        if self.check(&token_type) {
            self.advance()
        } else {
            Err(self.error_expected(&token_type))
        }
    }

    fn consume_identifier(&mut self) -> Result<String, CompilerError> {
        match &self.current_token()?.token_type {
            TokenType::Identifier | TokenType::BuiltinFunction => Ok(self.advance()?.value.clone()),
            _ => Err(self.error_expected_identifier()),
        }
    }

    // ========== Error helpers ==========

    /// Create an "unexpected end of input" error
    fn error_eof(&self) -> CompilerError {
        let (line, column) = self.last_position();
        CompilerError::ParseError(format!(
            "Unexpected end of input at {}:{}",
            line, column
        ))
    }

    /// Create an "expected X" error with location
    fn error_expected(&self, expected: &TokenType) -> CompilerError {
        if let Some(token) = self.tokens.get(self.position) {
            CompilerError::ParseError(format!(
                "Expected '{}' but found '{}' at {}:{}",
                expected.as_str(),
                token.value,
                token.line,
                token.column
            ))
        } else {
            self.error_eof()
        }
    }

    /// Create an "expected identifier" error
    fn error_expected_identifier(&self) -> CompilerError {
        if let Some(token) = self.tokens.get(self.position) {
            CompilerError::ParseError(format!(
                "Expected identifier but found '{}' at {}:{}",
                token.value,
                token.line,
                token.column
            ))
        } else {
            self.error_eof()
        }
    }

    /// Get the position of the last token (for EOF errors)
    fn last_position(&self) -> (usize, usize) {
        self.tokens
            .last()
            .map(|t| (t.line, t.column + t.value.len()))
            .unwrap_or((1, 1))
    }
}

