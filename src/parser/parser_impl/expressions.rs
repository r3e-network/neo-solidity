impl Parser {
    fn parse_expression(&mut self) -> Result<AstNode, CompilerError> {
        match &self.current_token()?.token_type {
            TokenType::Identifier | TokenType::BuiltinFunction => {
                let name = self.advance()?.value.clone();

                if self.check(&TokenType::LeftParen) {
                    // Function call
                    self.consume(TokenType::LeftParen)?;

                    let mut arguments = Vec::new();

                    if !self.check(&TokenType::RightParen) {
                        arguments.push(self.parse_expression()?);

                        while self.match_token(&TokenType::Comma)? {
                            arguments.push(self.parse_expression()?);
                        }
                    }

                    self.consume(TokenType::RightParen)?;

                    Ok(AstNode {
                        node_type: AstNodeType::FunctionCall { name, arguments },
                        line: self.current_token().map_or(1, |t| t.line),
                        column: self.current_token().map_or(1, |t| t.column),
                    })
                } else {
                    // Identifier
                    Ok(AstNode {
                        node_type: AstNodeType::Identifier { name },
                        line: self.current_token().map_or(1, |t| t.line),
                        column: self.current_token().map_or(1, |t| t.column),
                    })
                }
            }
            TokenType::Literal => {
                let value = self.advance()?.value.clone();
                Ok(AstNode {
                    node_type: AstNodeType::Literal { value },
                    line: self.current_token().map_or(1, |t| t.line),
                    column: self.current_token().map_or(1, |t| t.column),
                })
            }
            _ => Err(CompilerError::ParseError("Expected expression".to_string())),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Option<AstNode>, CompilerError> {
        let expr = self.parse_expression()?;
        Ok(Some(expr))
    }
}

