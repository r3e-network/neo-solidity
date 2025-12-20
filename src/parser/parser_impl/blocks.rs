impl Parser {
    fn parse_block(&mut self) -> Result<Option<AstNode>, CompilerError> {
        let start_line = self.current_token()?.line;
        let start_column = self.current_token()?.column;

        self.consume(TokenType::LeftBrace)?;

        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
        }

        self.consume(TokenType::RightBrace)?;

        Ok(Some(AstNode {
            node_type: AstNodeType::Block { statements },
            line: start_line,
            column: start_column,
        }))
    }

    fn parse_assignment(&mut self) -> Result<Option<AstNode>, CompilerError> {
        let start_line = self.current_token()?.line;
        let start_column = self.current_token()?.column;

        self.consume(TokenType::Let)?;

        let mut targets = Vec::new();
        targets.push(self.consume_identifier()?);

        while self.match_token(&TokenType::Comma)? {
            targets.push(self.consume_identifier()?);
        }

        self.consume(TokenType::Assignment)?;

        let value = Box::new(self.parse_expression()?);

        Ok(Some(AstNode {
            node_type: AstNodeType::Assignment { targets, value },
            line: start_line,
            column: start_column,
        }))
    }
}

