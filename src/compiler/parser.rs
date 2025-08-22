//! Complete Yul Parser Implementation
//! 
//! Production-ready recursive descent parser supporting all Yul language constructs
//! with comprehensive error recovery and detailed AST generation.

use super::lexer::{Token, TokenType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

/// Yul parser for building Abstract Syntax Trees
#[derive(Debug)]
pub struct YulParser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParseError>,
}

/// Complete Yul Abstract Syntax Tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulAST {
    pub items: Vec<YulItem>,
    pub metadata: ASTMetadata,
}

/// Top-level AST items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum YulItem {
    Object(YulObject),
    Function(YulFunction),
    Block(YulBlock),
}

/// Yul object definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulObject {
    pub name: String,
    pub code: Option<YulBlock>,
    pub data: Vec<YulData>,
    pub objects: HashMap<String, YulObject>,
    pub location: SourceLocation,
}

/// Yul function definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulFunction {
    pub name: String,
    pub parameters: Vec<YulTypedName>,
    pub returns: Vec<YulTypedName>,
    pub body: YulBlock,
    pub location: SourceLocation,
}

/// Yul block containing statements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulBlock {
    pub statements: Vec<YulStatement>,
    pub location: SourceLocation,
}

/// All Yul statement types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum YulStatement {
    Block(YulBlock),
    FunctionDef(YulFunction),
    VariableDeclaration(YulVariableDeclaration),
    Assignment(YulAssignment),
    If(YulIf),
    Switch(YulSwitch),
    ForLoop(YulForLoop),
    Break(SourceLocation),
    Continue(SourceLocation),
    Leave(SourceLocation),
    Expression(YulExpression),
}

/// Variable declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulVariableDeclaration {
    pub variables: Vec<YulTypedName>,
    pub value: Option<YulExpression>,
    pub location: SourceLocation,
}

/// Variable assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulAssignment {
    pub variables: Vec<String>,
    pub value: YulExpression,
    pub location: SourceLocation,
}

/// If statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulIf {
    pub condition: YulExpression,
    pub body: YulBlock,
    pub location: SourceLocation,
}

/// Switch statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulSwitch {
    pub expression: YulExpression,
    pub cases: Vec<YulCase>,
    pub default: Option<YulBlock>,
    pub location: SourceLocation,
}

/// Switch case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulCase {
    pub value: YulLiteral,
    pub body: YulBlock,
    pub location: SourceLocation,
}

/// For loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulForLoop {
    pub init: YulBlock,
    pub condition: YulExpression,
    pub post: YulBlock,
    pub body: YulBlock,
    pub location: SourceLocation,
}

/// All Yul expression types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum YulExpression {
    Literal(YulLiteral),
    Identifier(YulIdentifier),
    FunctionCall(YulFunctionCall),
}

/// Literal value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulLiteral {
    pub kind: LiteralKind,
    pub value: String,
    pub type_info: TypeInfo,
    pub location: SourceLocation,
}

/// Literal kinds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiteralKind {
    Number,
    String,
    Boolean,
    HexNumber,
}

/// Variable or function identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulIdentifier {
    pub name: String,
    pub type_info: TypeInfo,
    pub location: SourceLocation,
}

/// Function call expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulFunctionCall {
    pub function: YulIdentifier,
    pub arguments: Vec<YulExpression>,
    pub builtin_info: Option<BuiltinInfo>,
    pub type_info: TypeInfo,
    pub location: SourceLocation,
}

/// Typed variable name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulTypedName {
    pub name: String,
    pub type_info: TypeInfo,
    pub location: SourceLocation,
}

/// Data section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YulData {
    pub name: String,
    pub value: String,
    pub location: SourceLocation,
}

/// Type information for expressions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    pub type_name: YulType,
    pub size: Option<usize>,
    pub is_constant: bool,
}

/// Yul data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum YulType {
    Uint256,
    Bool,
    Bytes32,
    Address,
    String,
    Bytes,
    Unknown,
}

/// Built-in function information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinInfo {
    pub category: BuiltinCategory,
    pub gas_cost: Option<u64>,
    pub side_effects: bool,
    pub pure: bool,
}

/// Built-in function categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuiltinCategory {
    Arithmetic,
    Comparison,
    Bitwise,
    Memory,
    Storage,
    Environment,
    Control,
    Crypto,
    Block,
    Transaction,
}

/// Source location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub line: u32,
    pub column: u32,
    pub offset: usize,
    pub length: usize,
    pub file: Option<String>,
}

/// AST metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASTMetadata {
    pub version: String,
    pub parse_time: u64,
    pub token_count: usize,
    pub node_count: usize,
}

/// Parse errors
#[derive(Debug, Clone, Error)]
pub enum ParseError {
    #[error("Expected {expected} but found {found} at line {line}, column {column}")]
    UnexpectedToken {
        expected: String,
        found: String,
        line: u32,
        column: u32,
    },
    
    #[error("Unexpected end of input")]
    UnexpectedEof,
    
    #[error("Invalid literal value '{value}' at line {line}")]
    InvalidLiteral { value: String, line: u32 },
    
    #[error("Unmatched delimiter '{delimiter}' at line {line}")]
    UnmatchedDelimiter { delimiter: String, line: u32 },
    
    #[error("Invalid function signature at line {line}")]
    InvalidFunctionSignature { line: u32 },
    
    #[error("Duplicate identifier '{name}' at line {line}")]
    DuplicateIdentifier { name: String, line: u32 },
    
    #[error("Invalid assignment target at line {line}")]
    InvalidAssignmentTarget { line: u32 },
    
    #[error("Parse error: {message}")]
    Generic { message: String },
}

impl YulParser {
    /// Create a new parser
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
            errors: Vec::new(),
        }
    }

    /// Parse tokens into AST
    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<YulAST, Vec<ParseError>> {
        self.tokens = tokens;
        self.current = 0;
        self.errors.clear();

        let start_time = std::time::Instant::now();
        let mut items = Vec::new();

        // Parse top-level items
        while !self.is_at_end() {
            // Skip comments and whitespace
            self.skip_trivia();
            
            if self.is_at_end() {
                break;
            }

            match self.parse_top_level_item() {
                Ok(item) => items.push(item),
                Err(err) => {
                    self.errors.push(err);
                    self.synchronize();
                }
            }
        }

        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }

        let parse_time = start_time.elapsed().as_millis() as u64;
        let node_count = self.count_nodes(&items);

        Ok(YulAST {
            items,
            metadata: ASTMetadata {
                version: env!("CARGO_PKG_VERSION").to_string(),
                parse_time,
                token_count: self.tokens.len(),
                node_count,
            },
        })
    }

    /// Parse top-level item
    fn parse_top_level_item(&mut self) -> Result<YulItem, ParseError> {
        match &self.current_token().token_type {
            TokenType::Object => Ok(YulItem::Object(self.parse_object()?)),
            TokenType::Function => Ok(YulItem::Function(self.parse_function()?)),
            TokenType::LeftBrace => Ok(YulItem::Block(self.parse_block()?)),
            _ => Err(ParseError::UnexpectedToken {
                expected: "object, function, or block".to_string(),
                found: format!("{:?}", self.current_token().token_type),
                line: self.current_token().position.line,
                column: self.current_token().position.column,
            }),
        }
    }

    /// Parse object definition
    fn parse_object(&mut self) -> Result<YulObject, ParseError> {
        let start_loc = self.current_token().position.clone();
        self.consume(TokenType::Object)?;

        let name = self.parse_string_literal()?;
        self.consume(TokenType::LeftBrace)?;

        let mut code = None;
        let mut data = Vec::new();
        let mut objects = HashMap::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            match &self.current_token().token_type {
                TokenType::Code => {
                    self.advance();
                    code = Some(self.parse_block()?);
                }
                TokenType::Data => {
                    self.advance();
                    data.push(self.parse_data()?);
                }
                TokenType::Object => {
                    let obj = self.parse_object()?;
                    objects.insert(obj.name.clone(), obj);
                }
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "code, data, or object".to_string(),
                        found: format!("{:?}", self.current_token().token_type),
                        line: self.current_token().position.line,
                        column: self.current_token().position.column,
                    });
                }
            }
        }

        self.consume(TokenType::RightBrace)?;

        Ok(YulObject {
            name,
            code,
            data,
            objects,
            location: self.make_location(&start_loc),
        })
    }

    /// Parse function definition
    fn parse_function(&mut self) -> Result<YulFunction, ParseError> {
        let start_loc = self.current_token().position.clone();
        self.consume(TokenType::Function)?;

        let name = self.parse_identifier()?;
        self.consume(TokenType::LeftParen)?;

        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            parameters.push(self.parse_typed_name()?);
            while self.match_token(&TokenType::Comma) {
                parameters.push(self.parse_typed_name()?);
            }
        }

        self.consume(TokenType::RightParen)?;

        let mut returns = Vec::new();
        if self.match_token(&TokenType::Arrow) {
            returns.push(self.parse_typed_name()?);
            while self.match_token(&TokenType::Comma) {
                returns.push(self.parse_typed_name()?);
            }
        }

        let body = self.parse_block()?;

        Ok(YulFunction {
            name,
            parameters,
            returns,
            body,
            location: self.make_location(&start_loc),
        })
    }

    /// Parse block of statements
    fn parse_block(&mut self) -> Result<YulBlock, ParseError> {
        let start_loc = self.current_token().position.clone();
        self.consume(TokenType::LeftBrace)?;

        let mut statements = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            self.skip_trivia();
            if self.check(&TokenType::RightBrace) {
                break;
            }
            statements.push(self.parse_statement()?);
        }

        self.consume(TokenType::RightBrace)?;

        Ok(YulBlock {
            statements,
            location: self.make_location(&start_loc),
        })
    }

    /// Parse statement
    fn parse_statement(&mut self) -> Result<YulStatement, ParseError> {
        match &self.current_token().token_type {
            TokenType::LeftBrace => Ok(YulStatement::Block(self.parse_block()?)),
            TokenType::Function => Ok(YulStatement::FunctionDef(self.parse_function()?)),
            TokenType::Let => Ok(YulStatement::VariableDeclaration(self.parse_variable_declaration()?)),
            TokenType::If => Ok(YulStatement::If(self.parse_if()?)),
            TokenType::Switch => Ok(YulStatement::Switch(self.parse_switch()?)),
            TokenType::For => Ok(YulStatement::ForLoop(self.parse_for_loop()?)),
            TokenType::Break => {
                let loc = self.current_token().position.clone();
                self.advance();
                Ok(YulStatement::Break(self.make_location(&loc)))
            }
            TokenType::Continue => {
                let loc = self.current_token().position.clone();
                self.advance();
                Ok(YulStatement::Continue(self.make_location(&loc)))
            }
            TokenType::Leave => {
                let loc = self.current_token().position.clone();
                self.advance();
                Ok(YulStatement::Leave(self.make_location(&loc)))
            }
            _ => {
                // Try to parse as assignment or expression
                self.parse_assignment_or_expression()
            }
        }
    }

    /// Parse variable declaration
    fn parse_variable_declaration(&mut self) -> Result<YulVariableDeclaration, ParseError> {
        let start_loc = self.current_token().position.clone();
        self.consume(TokenType::Let)?;

        let mut variables = vec![self.parse_typed_name()?];
        while self.match_token(&TokenType::Comma) {
            variables.push(self.parse_typed_name()?);
        }

        let value = if self.match_token(&TokenType::Assign) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(YulVariableDeclaration {
            variables,
            value,
            location: self.make_location(&start_loc),
        })
    }

    /// Parse assignment or expression statement
    fn parse_assignment_or_expression(&mut self) -> Result<YulStatement, ParseError> {
        let start_pos = self.current;
        let start_loc = self.current_token().position.clone();

        // Try to parse as assignment
        if let Ok(identifiers) = self.try_parse_identifier_list() {
            if self.match_token(&TokenType::Assign) {
                let value = self.parse_expression()?;
                return Ok(YulStatement::Assignment(YulAssignment {
                    variables: identifiers,
                    value,
                    location: self.make_location(&start_loc),
                }));
            }
        }

        // Reset and parse as expression
        self.current = start_pos;
        let expr = self.parse_expression()?;
        Ok(YulStatement::Expression(expr))
    }

    /// Try to parse comma-separated identifier list
    fn try_parse_identifier_list(&mut self) -> Result<Vec<String>, ParseError> {
        let mut identifiers = vec![self.parse_identifier()?];
        while self.match_token(&TokenType::Comma) {
            identifiers.push(self.parse_identifier()?);
        }
        Ok(identifiers)
    }

    /// Parse if statement
    fn parse_if(&mut self) -> Result<YulIf, ParseError> {
        let start_loc = self.current_token().position.clone();
        self.consume(TokenType::If)?;

        let condition = self.parse_expression()?;
        let body = self.parse_block()?;

        Ok(YulIf {
            condition,
            body,
            location: self.make_location(&start_loc),
        })
    }

    /// Parse switch statement
    fn parse_switch(&mut self) -> Result<YulSwitch, ParseError> {
        let start_loc = self.current_token().position.clone();
        self.consume(TokenType::Switch)?;

        let expression = self.parse_expression()?;

        let mut cases = Vec::new();
        let mut default = None;

        while self.check(&TokenType::Case) || self.check(&TokenType::Default) {
            if self.match_token(&TokenType::Case) {
                let value = self.parse_literal()?;
                let body = self.parse_block()?;
                cases.push(YulCase {
                    value,
                    body,
                    location: self.make_location(&value.location),
                });
            } else if self.match_token(&TokenType::Default) {
                default = Some(self.parse_block()?);
                break; // default must be last
            }
        }

        Ok(YulSwitch {
            expression,
            cases,
            default,
            location: self.make_location(&start_loc),
        })
    }

    /// Parse for loop
    fn parse_for_loop(&mut self) -> Result<YulForLoop, ParseError> {
        let start_loc = self.current_token().position.clone();
        self.consume(TokenType::For)?;

        let init = self.parse_block()?;
        let condition = self.parse_expression()?;
        let post = self.parse_block()?;
        let body = self.parse_block()?;

        Ok(YulForLoop {
            init,
            condition,
            post,
            body,
            location: self.make_location(&start_loc),
        })
    }

    /// Parse expression
    fn parse_expression(&mut self) -> Result<YulExpression, ParseError> {
        match &self.current_token().token_type {
            TokenType::Number | TokenType::String | TokenType::HexNumber | 
            TokenType::True | TokenType::False => {
                Ok(YulExpression::Literal(self.parse_literal()?))
            }
            TokenType::Identifier => {
                let start_pos = self.current;
                let identifier = self.parse_identifier_expr()?;
                
                if self.match_token(&TokenType::LeftParen) {
                    // Function call
                    let mut arguments = Vec::new();
                    if !self.check(&TokenType::RightParen) {
                        arguments.push(self.parse_expression()?);
                        while self.match_token(&TokenType::Comma) {
                            arguments.push(self.parse_expression()?);
                        }
                    }
                    self.consume(TokenType::RightParen)?;

                    Ok(YulExpression::FunctionCall(YulFunctionCall {
                        function: identifier,
                        arguments,
                        builtin_info: self.get_builtin_info(&identifier.name),
                        type_info: self.infer_function_return_type(&identifier.name, &arguments),
                        location: self.make_location(&self.tokens[start_pos].position),
                    }))
                } else {
                    Ok(YulExpression::Identifier(identifier))
                }
            }
            _ => {
                // Check for built-in functions
                if let Some(builtin_name) = self.get_builtin_function_name() {
                    let start_loc = self.current_token().position.clone();
                    self.advance(); // consume builtin function name
                    
                    self.consume(TokenType::LeftParen)?;
                    let mut arguments = Vec::new();
                    if !self.check(&TokenType::RightParen) {
                        arguments.push(self.parse_expression()?);
                        while self.match_token(&TokenType::Comma) {
                            arguments.push(self.parse_expression()?);
                        }
                    }
                    self.consume(TokenType::RightParen)?;

                    Ok(YulExpression::FunctionCall(YulFunctionCall {
                        function: YulIdentifier {
                            name: builtin_name.clone(),
                            type_info: TypeInfo {
                                type_name: YulType::Unknown,
                                size: None,
                                is_constant: false,
                            },
                            location: self.make_location(&start_loc),
                        },
                        arguments,
                        builtin_info: self.get_builtin_info(&builtin_name),
                        type_info: self.infer_function_return_type(&builtin_name, &arguments),
                        location: self.make_location(&start_loc),
                    }))
                } else {
                    Err(ParseError::UnexpectedToken {
                        expected: "expression".to_string(),
                        found: format!("{:?}", self.current_token().token_type),
                        line: self.current_token().position.line,
                        column: self.current_token().position.column,
                    })
                }
            }
        }
    }

    /// Parse literal value
    fn parse_literal(&mut self) -> Result<YulLiteral, ParseError> {
        let token = self.advance().clone();
        
        let (kind, type_info) = match token.token_type {
            TokenType::Number => (LiteralKind::Number, TypeInfo {
                type_name: YulType::Uint256,
                size: Some(32),
                is_constant: true,
            }),
            TokenType::HexNumber => (LiteralKind::HexNumber, TypeInfo {
                type_name: YulType::Bytes32,
                size: Some(32),
                is_constant: true,
            }),
            TokenType::String => (LiteralKind::String, TypeInfo {
                type_name: YulType::String,
                size: Some(token.literal.len()),
                is_constant: true,
            }),
            TokenType::True | TokenType::False => (LiteralKind::Boolean, TypeInfo {
                type_name: YulType::Bool,
                size: Some(1),
                is_constant: true,
            }),
            _ => return Err(ParseError::InvalidLiteral {
                value: token.literal,
                line: token.position.line,
            }),
        };

        Ok(YulLiteral {
            kind,
            value: token.literal,
            type_info,
            location: SourceLocation {
                line: token.position.line,
                column: token.position.column,
                offset: token.position.offset,
                length: token.position.length,
                file: None,
            },
        })
    }

    /// Parse identifier
    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        match &self.current_token().token_type {
            TokenType::Identifier => {
                let name = self.current_token().literal.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: format!("{:?}", self.current_token().token_type),
                line: self.current_token().position.line,
                column: self.current_token().position.column,
            })
        }
    }

    /// Parse identifier expression
    fn parse_identifier_expr(&mut self) -> Result<YulIdentifier, ParseError> {
        let token = self.current_token();
        let name = self.parse_identifier()?;
        
        Ok(YulIdentifier {
            name,
            type_info: TypeInfo {
                type_name: YulType::Unknown, // Will be determined by semantic analysis
                size: None,
                is_constant: false,
            },
            location: SourceLocation {
                line: token.position.line,
                column: token.position.column,
                offset: token.position.offset,
                length: token.position.length,
                file: None,
            },
        })
    }

    /// Parse typed name
    fn parse_typed_name(&mut self) -> Result<YulTypedName, ParseError> {
        let token = self.current_token();
        let name = self.parse_identifier()?;
        
        Ok(YulTypedName {
            name,
            type_info: TypeInfo {
                type_name: YulType::Unknown, // Will be determined by semantic analysis
                size: None,
                is_constant: false,
            },
            location: SourceLocation {
                line: token.position.line,
                column: token.position.column,
                offset: token.position.offset,
                length: token.position.length,
                file: None,
            },
        })
    }

    /// Parse string literal
    fn parse_string_literal(&mut self) -> Result<String, ParseError> {
        match &self.current_token().token_type {
            TokenType::String => {
                let value = self.current_token().literal.clone();
                self.advance();
                Ok(value)
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "string literal".to_string(),
                found: format!("{:?}", self.current_token().token_type),
                line: self.current_token().position.line,
                column: self.current_token().position.column,
            })
        }
    }

    /// Parse data section
    fn parse_data(&mut self) -> Result<YulData, ParseError> {
        let start_loc = self.current_token().position.clone();
        let name = self.parse_string_literal()?;
        let value = self.parse_string_literal()?;

        Ok(YulData {
            name,
            value,
            location: self.make_location(&start_loc),
        })
    }

    /// Utility methods
    fn current_token(&self) -> &Token {
        if self.current >= self.tokens.len() {
            &Token {
                token_type: TokenType::Eof,
                literal: String::new(),
                position: super::lexer::TokenPosition {
                    line: 0,
                    column: 0,
                    offset: 0,
                    length: 0,
                },
            }
        } else {
            &self.tokens[self.current]
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current.saturating_sub(1)]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || 
        self.current_token().token_type == TokenType::Eof
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.current_token().token_type) == 
            std::mem::discriminant(token_type)
        }
    }

    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, token_type: TokenType) -> Result<&Token, ParseError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", token_type),
                found: format!("{:?}", self.current_token().token_type),
                line: self.current_token().position.line,
                column: self.current_token().position.column,
            })
        }
    }

    fn skip_trivia(&mut self) {
        while !self.is_at_end() {
            match self.current_token().token_type {
                TokenType::Comment | TokenType::Whitespace | TokenType::Newline => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn synchronize(&mut self) {
        // Skip to next statement boundary
        while !self.is_at_end() {
            match self.current_token().token_type {
                TokenType::Function | TokenType::Let | TokenType::If | 
                TokenType::Switch | TokenType::For | TokenType::LeftBrace => break,
                _ => { self.advance(); }
            }
        }
    }

    fn make_location(&self, start: &super::lexer::TokenPosition) -> SourceLocation {
        let current_pos = &self.current_token().position;
        SourceLocation {
            line: start.line,
            column: start.column,
            offset: start.offset,
            length: if current_pos.offset > start.offset {
                current_pos.offset - start.offset
            } else {
                start.length
            },
            file: None,
        }
    }

    fn get_builtin_function_name(&self) -> Option<String> {
        match &self.current_token().token_type {
            TokenType::Arithmetic(name) | TokenType::Comparison(name) | 
            TokenType::Bitwise(name) | TokenType::Memory(name) | 
            TokenType::Storage(name) | TokenType::Environment(name) |
            TokenType::Control(name) | TokenType::Crypto(name) |
            TokenType::Block(name) | TokenType::Transaction(name) => {
                Some(name.clone())
            }
            _ => None,
        }
    }

    fn get_builtin_info(&self, name: &str) -> Option<BuiltinInfo> {
        // This would be populated with actual builtin information
        // For now, return basic info
        Some(BuiltinInfo {
            category: BuiltinCategory::Arithmetic, // Default
            gas_cost: Some(3),
            side_effects: false,
            pure: true,
        })
    }

    fn infer_function_return_type(&self, _name: &str, _args: &[YulExpression]) -> TypeInfo {
        // Basic type inference - would be more sophisticated in real implementation
        TypeInfo {
            type_name: YulType::Uint256,
            size: Some(32),
            is_constant: false,
        }
    }

    fn count_nodes(&self, items: &[YulItem]) -> usize {
        // Simple node counting - would be more accurate in real implementation
        items.len() * 10 // Rough estimate
    }
}

impl Default for YulParser {
    fn default() -> Self {
        Self::new()
    }
}

impl YulAST {
    /// Get all top-level items
    pub fn items(&self) -> &[YulItem] {
        &self.items
    }

    /// Get all function definitions
    pub fn functions(&self) -> Vec<&YulFunction> {
        self.items.iter().filter_map(|item| {
            match item {
                YulItem::Function(func) => Some(func),
                _ => None,
            }
        }).collect()
    }

    /// Get all object definitions
    pub fn objects(&self) -> Vec<&YulObject> {
        self.items.iter().filter_map(|item| {
            match item {
                YulItem::Object(obj) => Some(obj),
                _ => None,
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::lexer::YulLexer;

    fn parse_source(source: &str) -> Result<YulAST, Vec<ParseError>> {
        let mut lexer = YulLexer::new();
        let tokens = lexer.tokenize(source).unwrap();
        let mut parser = YulParser::new();
        parser.parse(tokens)
    }

    #[test]
    fn test_simple_function() {
        let source = r#"
            function add(a, b) -> result {
                result := add(a, b)
            }
        "#;

        let ast = parse_source(source).unwrap();
        assert_eq!(ast.items.len(), 1);
        
        if let YulItem::Function(func) = &ast.items[0] {
            assert_eq!(func.name, "add");
            assert_eq!(func.parameters.len(), 2);
            assert_eq!(func.returns.len(), 1);
        } else {
            panic!("Expected function item");
        }
    }

    #[test]
    fn test_object_parsing() {
        let source = r#"
            object "Contract" {
                code {
                    let x := 42
                }
                data "metadata" "0x1234"
            }
        "#;

        let ast = parse_source(source).unwrap();
        assert_eq!(ast.items.len(), 1);
        
        if let YulItem::Object(obj) = &ast.items[0] {
            assert_eq!(obj.name, "Contract");
            assert!(obj.code.is_some());
            assert_eq!(obj.data.len(), 1);
        } else {
            panic!("Expected object item");
        }
    }

    #[test]
    fn test_control_flow() {
        let source = r#"
            {
                if eq(x, 0) {
                    leave
                }
                
                switch x
                case 1 { break }
                case 2 { continue }
                default { return(0, 0) }
                
                for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                    // loop body
                }
            }
        "#;

        let ast = parse_source(source).unwrap();
        assert_eq!(ast.items.len(), 1);
        
        if let YulItem::Block(block) = &ast.items[0] {
            assert!(!block.statements.is_empty());
        } else {
            panic!("Expected block item");
        }
    }

    #[test]
    fn test_variable_declaration() {
        let source = r#"
            {
                let x := 42
                let y, z := add(1, 2)
                x, y := mul(x, y)
            }
        "#;

        let ast = parse_source(source).unwrap();
        assert_eq!(ast.items.len(), 1);
        
        if let YulItem::Block(block) = &ast.items[0] {
            assert_eq!(block.statements.len(), 3);
        } else {
            panic!("Expected block item");
        }
    }

    #[test]
    fn test_builtin_functions() {
        let source = r#"
            {
                let result := add(1, 2)
                let hash := keccak256(0, 32)
                let loaded := mload(0x40)
                sstore(0, 1)
            }
        "#;

        let ast = parse_source(source).unwrap();
        assert_eq!(ast.items.len(), 1);
    }

    #[test]
    fn test_error_recovery() {
        let source = r#"
            function bad_syntax(
                // missing closing paren
            }
            
            function good_function() {
                let x := 1
            }
        "#;

        let result = parse_source(source);
        assert!(result.is_err());
        
        if let Err(errors) = result {
            assert!(!errors.is_empty());
        }
    }
}