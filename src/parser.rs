use crate::error::CompilerError;
use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct AstNode {
    pub node_type: AstNodeType,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum AstNodeType {
    Object {
        statements: Vec<AstNode>,
    },
    Function {
        name: String,
        params: Vec<String>,
        returns: Vec<String>,
        body: Box<AstNode>,
    },
    Block {
        statements: Vec<AstNode>,
    },
    If {
        condition: Box<AstNode>,
        then_branch: Box<AstNode>,
        else_branch: Option<Box<AstNode>>,
    },
    For {
        init: Option<Box<AstNode>>,
        condition: Box<AstNode>,
        update: Option<Box<AstNode>>,
        body: Box<AstNode>,
    },
    Switch {
        expression: Box<AstNode>,
        cases: Vec<SwitchCase>,
        default: Option<Box<AstNode>>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<AstNode>,
    },
    Assignment {
        targets: Vec<String>,
        value: Box<AstNode>,
    },
    Identifier {
        name: String,
    },
    Literal {
        value: String,
    },
}

#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub value: AstNode,
    pub body: AstNode,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

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

    // Helper methods
    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn current_token(&self) -> Result<&Token, CompilerError> {
        self.tokens
            .get(self.position)
            .ok_or_else(|| CompilerError::ParseError("Unexpected end of input".to_string()))
    }

    fn advance(&mut self) -> Result<&Token, CompilerError> {
        if !self.is_at_end() {
            self.position += 1;
        }
        self.tokens
            .get(self.position - 1)
            .ok_or_else(|| CompilerError::ParseError("Unexpected end of input".to_string()))
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.current_token().unwrap().token_type == token_type
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
            Err(CompilerError::ParseError(format!(
                "Expected {:?}, found {:?}",
                token_type,
                self.current_token()?.token_type
            )))
        }
    }

    fn consume_identifier(&mut self) -> Result<String, CompilerError> {
        if let TokenType::Identifier = self.current_token()?.token_type {
            Ok(self.advance()?.value.clone())
        } else {
            Err(CompilerError::ParseError("Expected identifier".to_string()))
        }
    }

    // Complete implementations for all language constructs
    fn parse_if(&mut self) -> Result<Option<AstNode>, CompilerError> {
        let start_line = self.current_token()?.line;
        let start_column = self.current_token()?.column;

        self.consume(TokenType::If)?;
        let condition = Box::new(self.parse_expression()?);
        let then_branch = Box::new(self.parse_statement()?.ok_or_else(|| {
            CompilerError::ParseError("Expected statement after if condition".to_string())
        })?);

        let else_branch =
            if self.check(&TokenType::Identifier) && self.current_token()?.value == "else" {
                self.advance()?;
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
        if self.check(&TokenType::Identifier) && self.current_token()?.value == "->" {
            self.advance()?; // consume "->"

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

    fn parse_expression_statement(&mut self) -> Result<Option<AstNode>, CompilerError> {
        let expr = self.parse_expression()?;
        Ok(Some(expr))
    }
}
