impl Parser {
    fn parse_function(&mut self) -> Result<Option<AstNode>, CompilerError> {
        let start_line = self.current_token()?.line;
        let start_column = self.current_token()?.column;

        self.consume(TokenType::Function)?;
        let name = self.consume_identifier()?;

        self.consume(TokenType::LeftParen)?;

        let mut params = Vec::new();
        if !self.check(&TokenType::RightParen) {
            params.push(self.consume_identifier()?);

            while self.match_token(&TokenType::Comma)? {
                params.push(self.consume_identifier()?);
            }
        }

        self.consume(TokenType::RightParen)?;

        // Parse return parameters
        let mut returns = Vec::new();
        if self.match_token(&TokenType::Arrow)? {
            returns.push(self.consume_identifier()?);
            while self.match_token(&TokenType::Comma)? {
                returns.push(self.consume_identifier()?);
            }
        }

        let body = Box::new(
            self.parse_statement()?
                .ok_or_else(|| CompilerError::ParseError("Expected function body".to_string()))?,
        );

        Ok(Some(AstNode {
            node_type: AstNodeType::Function {
                name,
                params,
                returns,
                body,
            },
            line: start_line,
            column: start_column,
        }))
    }
}

