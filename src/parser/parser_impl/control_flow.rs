impl Parser {
    fn parse_if(&mut self) -> Result<Option<AstNode>, CompilerError> {
        let start_line = self.current_token()?.line;
        let start_column = self.current_token()?.column;

        self.consume(TokenType::If)?;
        let condition = Box::new(self.parse_expression()?);
        let then_branch = Box::new(self.parse_statement()?.ok_or_else(|| {
            CompilerError::ParseError("Expected statement after if condition".to_string())
        })?);

        let else_branch = if self.match_token(&TokenType::Else)? {
            Some(Box::new(self.parse_statement()?.ok_or_else(|| {
                CompilerError::ParseError("Expected statement after else".to_string())
            })?))
        } else {
            None
        };

        Ok(Some(AstNode {
            node_type: AstNodeType::If {
                condition,
                then_branch,
                else_branch,
            },
            line: start_line,
            column: start_column,
        }))
    }

    fn parse_for(&mut self) -> Result<Option<AstNode>, CompilerError> {
        let start_line = self.current_token()?.line;
        let start_column = self.current_token()?.column;

        self.consume(TokenType::For)?;

        // Parse init block
        let init = if self.check(&TokenType::LeftBrace) {
            Some(Box::new(self.parse_block()?.ok_or_else(|| {
                CompilerError::ParseError("Expected init block".to_string())
            })?))
        } else {
            None
        };

        // Parse condition
        let condition = Box::new(self.parse_expression()?);

        // Parse update block
        let update = if self.check(&TokenType::LeftBrace) {
            Some(Box::new(self.parse_block()?.ok_or_else(|| {
                CompilerError::ParseError("Expected update block".to_string())
            })?))
        } else {
            None
        };

        // Parse body
        let body = Box::new(
            self.parse_statement()?
                .ok_or_else(|| CompilerError::ParseError("Expected for loop body".to_string()))?,
        );

        Ok(Some(AstNode {
            node_type: AstNodeType::For {
                init,
                condition,
                update,
                body,
            },
            line: start_line,
            column: start_column,
        }))
    }

    fn parse_switch(&mut self) -> Result<Option<AstNode>, CompilerError> {
        let start_line = self.current_token()?.line;
        let start_column = self.current_token()?.column;

        self.consume(TokenType::Switch)?;
        let expression = Box::new(self.parse_expression()?);

        let mut cases = Vec::new();
        let mut default = None;

        while self.check(&TokenType::Case) || self.check(&TokenType::Default) {
            if self.check(&TokenType::Case) {
                self.consume(TokenType::Case)?;
                let value = self.parse_expression()?;
                let body = self
                    .parse_statement()?
                    .ok_or_else(|| CompilerError::ParseError("Expected case body".to_string()))?;

                cases.push(SwitchCase { value, body });
            } else if self.check(&TokenType::Default) {
                self.consume(TokenType::Default)?;
                default = Some(Box::new(self.parse_statement()?.ok_or_else(|| {
                    CompilerError::ParseError("Expected default body".to_string())
                })?));
            }
        }

        Ok(Some(AstNode {
            node_type: AstNodeType::Switch {
                expression,
                cases,
                default,
            },
            line: start_line,
            column: start_column,
        }))
    }
}

