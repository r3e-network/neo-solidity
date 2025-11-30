//! 语法分析器 - 将Token流转换为抽象语法树(AST)
//!
//! 支持完整的Solidity 0.8.x语法，包括：
//! - 合约、接口、库定义
//! - 函数和事件定义
//! - 复杂类型系统
//! - 继承和修饰符
//! - 控制流结构

use crate::compiler::lexer::{Lexer, Token, TokenType, LiteralType};
use crate::error::CompilerError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AstNode {
    pub node_type: AstNodeType,
    pub line: usize,
    pub column: usize,
    pub source_span: crate::compiler::lexer::SourceSpan,
}

#[derive(Debug, Clone)]
pub enum AstNodeType {
    // 编译单元
    SourceUnit {
        pragma_directives: Vec<AstNode>,
        import_directives: Vec<AstNode>,
        contract_definitions: Vec<AstNode>,
    },

    // 编译指令
    PragmaDirective {
        name: String,
        version: String,
    },

    // 导入指令
    ImportDirective {
        path: String,
        alias: Option<String>,
        symbols: Vec<String>,
    },

    // 合约定义
    ContractDefinition {
        name: String,
        contract_type: ContractType,
        inheritance: Vec<String>,
        members: Vec<AstNode>,
    },

    // 结构体定义
    StructDefinition {
        name: String,
        fields: Vec<StructField>,
    },

    // 枚举定义
    EnumDefinition {
        name: String,
        members: Vec<String>,
    },

    // 事件定义
    EventDefinition {
        name: String,
        parameters: Vec<Parameter>,
        anonymous: bool,
    },

    // 函数定义
    FunctionDefinition {
        name: String,
        visibility: Option<Visibility>,
        mutability: Option<StateMutability>,
        modifiers: Vec<ModifierInvocation>,
        parameters: Vec<Parameter>,
        return_parameters: Vec<Parameter>,
        body: Option<AstNode>,
        is_constructor: bool,
        is_fallback: bool,
        is_receive: bool,
    },

    // 修饰符定义
    ModifierDefinition {
        name: String,
        parameters: Vec<Parameter>,
        body: AstNode,
    },

    // 语句
    Block {
        statements: Vec<AstNode>,
    },

    ExpressionStatement {
        expression: Box<AstNode>,
    },

    IfStatement {
        condition: Box<AstNode>,
        true_body: Box<AstNode>,
        false_body: Option<Box<AstNode>>,
    },

    ForStatement {
        initialization: Option<Box<AstNode>>,
        condition: Option<Box<AstNode>>,
        update: Option<Box<AstNode>>,
        body: Box<AstNode>,
    },

    WhileStatement {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },

    DoWhileStatement {
        body: Box<AstNode>,
        condition: Box<AstNode>,
    },

    ReturnStatement {
        expression: Option<Box<AstNode>>,
    },

    BreakStatement,
    ContinueStatement,

    EmitStatement {
        event_call: Box<AstNode>,
    },

    TryStatement {
        expression: Box<AstNode>,
        returns: Vec<Parameter>,
        catch_clauses: Vec<CatchClause>,
    },

    // 表达式
    Literal {
        value: String,
        literal_type: LiteralType,
    },

    Identifier {
        name: String,
    },

    TupleExpression {
        components: Vec<Option<AstNode>>,
    },

    Assignment {
        left: Box<AstNode>,
        operator: AssignmentOperator,
        right: Box<AstNode>,
    },

    BinaryOperation {
        left: Box<AstNode>,
        operator: BinaryOperator,
        right: Box<AstNode>,
    },

    UnaryOperation {
        operator: UnaryOperator,
        sub_expression: Box<AstNode>,
        prefix: bool,
    },

    Conditional {
        condition: Box<AstNode>,
        true_expression: Box<AstNode>,
        false_expression: Box<AstNode>,
    },

    FunctionCall {
        function: Box<AstNode>,
        arguments: Vec<AstNode>,
        names: Vec<String>,
    },

    MemberAccess {
        object: Box<AstNode>,
        member_name: String,
    },

    IndexAccess {
        base: Box<AstNode>,
        index: Box<AstNode>,
    },

    NewExpression {
        contract_name: String,
    },

    TypeConversion {
        type_name: TypeName,
        expression: Box<AstNode>,
    },

    // 变量声明
    VariableDeclaration {
        type_name: TypeName,
        name: String,
        value: Option<Box<AstNode>>,
        storage_location: Option<StorageLocation>,
        is_state_var: bool,
        is_constant: bool,
        is_immutable: bool,
        visibility: Option<Visibility>,
    },

    // 类型
    ElementaryTypeName {
        name: String,
    },

    UserDefinedTypeName {
        name: String,
    },

    Mapping {
        key_type: Box<TypeName>,
        value_type: Box<TypeName>,
    },

    ArrayTypeName {
        base_type: Box<TypeName>,
        length: Option<Box<AstNode>>,
    },

    // 辅助结构
    Empty,
}

#[derive(Debug, Clone)]
pub enum ContractType {
    Contract,
    Interface,
    Library,
}

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Private,
    Internal,
    External,
}

#[derive(Debug, Clone)]
pub enum StateMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

#[derive(Debug, Clone)]
pub enum StorageLocation {
    Memory,
    Storage,
    Calldata,
}

#[derive(Debug, Clone)]
pub enum AssignmentOperator {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    LeftShiftAssign,
    RightShiftAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Power,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    LeftShift,
    RightShift,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    BitNot,
    PreIncrement,
    PreDecrement,
    PostIncrement,
    PostDecrement,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub type_name: TypeName,
    pub name: Option<String>,
    pub storage_location: Option<StorageLocation>,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub type_name: TypeName,
    pub name: String,
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone)]
pub struct ModifierInvocation {
    pub name: String,
    pub arguments: Vec<AstNode>,
}

#[derive(Debug, Clone)]
pub struct CatchClause {
    pub error_name: Option<String>,
    pub parameters: Vec<Parameter>,
    pub body: AstNode,
}

#[derive(Debug, Clone)]
pub enum TypeName {
    Elementary(ElementaryTypeName),
    UserDefined(UserDefinedTypeName),
    Mapping(Mapping),
    Array(ArrayTypeName),
    Function(FunctionType),
}

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub parameters: Vec<Parameter>,
    pub return_parameters: Vec<Parameter>,
    pub mutability: Option<StateMutability>,
    pub visibility: Option<Visibility>,
}

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: None,
            peek_token: None,
        };
        parser.advance();
        parser.advance(); // Initialize both current and peek
        parser
    }

    pub fn parse(&mut self) -> Result<AstNode, CompilerError> {
        self.parse_source_unit()
    }

    fn parse_source_unit(&mut self) -> Result<AstNode, CompilerError> {
        let mut pragma_directives = Vec::new();
        let mut import_directives = Vec::new();
        let mut contract_definitions = Vec::new();

        while !self.is_at_end() {
            match self.current_token_type()? {
                TokenType::Pragma => {
                    pragma_directives.push(self.parse_pragma_directive()?);
                }
                TokenType::Identifier if self.current_token()?.value == "import" => {
                    import_directives.push(self.parse_import_directive()?);
                }
                TokenType::Contract | TokenType::Interface | TokenType::Library | TokenType::Struct | TokenType::Enum | TokenType::Event | TokenType::Function => {
                    contract_definitions.push(self.parse_contract_or_top_level_declaration()?);
                }
                _ => {
                    return Err(CompilerError::ParseError(format!(
                        "Unexpected token {:?} at top level",
                        self.current_token()
                    )));
                }
            }
        }

        Ok(self.create_node(
            AstNodeType::SourceUnit {
                pragma_directives,
                import_directives,
                contract_definitions,
            },
            1,
            1,
        ))
    }

    fn parse_pragma_directive(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume(TokenType::Pragma)?;
        let name = self.consume(TokenType::Identifier)?.value;

        // 处理版本表达式，如 ">=0.8.0 <0.9.0"
        let mut version = String::new();
        while !self.check(TokenType::Semicolon) && !self.is_at_end() {
            if let Some(token) = &self.current_token {
                if !version.is_empty() {
                    version.push(' ');
                }
                version.push_str(&token.value);
            }
            self.advance();
        }

        self.consume(TokenType::Semicolon)?;

        Ok(self.create_node(
            AstNodeType::PragmaDirective { name, version },
            start_line,
            start_column,
        ))
    }

    fn parse_import_directive(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume_identifier("import")?;

        let path = if self.check(TokenType::StringLiteral) {
            self.consume(TokenType::StringLiteral)?.value
        } else {
            self.consume(TokenType::Identifier)?.value
        };

        let mut alias = None;
        let mut symbols = Vec::new();

        if self.match_token(TokenType::As) {
            alias = Some(self.consume(TokenType::Identifier)?.value);
        } else if self.match_token(TokenType::LeftBrace) {
            while !self.check(TokenType::RightBrace) && !self.is_at_end() {
                symbols.push(self.consume(TokenType::Identifier)?.value);
                if !self.check(TokenType::RightBrace) {
                    self.consume(TokenType::Comma)?;
                }
            }
            self.consume(TokenType::RightBrace)?;
            if self.match_token(TokenType::As) {
                alias = Some(self.consume(TokenType::Identifier)?.value);
            }
        }

        self.consume(TokenType::Semicolon)?;

        Ok(self.create_node(
            AstNodeType::ImportDirective {
                path,
                alias,
                symbols,
            },
            start_line,
            start_column,
        ))
    }

    fn parse_contract_or_top_level_declaration(&mut self) -> Result<AstNode, CompilerError> {
        match self.current_token_type()? {
            TokenType::Contract | TokenType::Interface | TokenType::Library => {
                self.parse_contract_definition()
            }
            TokenType::Struct => self.parse_struct_definition(),
            TokenType::Enum => self.parse_enum_definition(),
            TokenType::Event => self.parse_event_definition(),
            TokenType::Function => self.parse_function_definition(),
            _ => Err(CompilerError::ParseError(format!(
                "Expected contract, struct, enum, event, or function definition"
            ))),
        }
    }

    fn parse_contract_definition(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let contract_type = match self.advance()?.token_type {
            TokenType::Contract => ContractType::Contract,
            TokenType::Interface => ContractType::Interface,
            TokenType::Library => ContractType::Library,
            _ => unreachable!(),
        };

        let name = self.consume(TokenType::Identifier)?.value;

        let mut inheritance = Vec::new();
        if self.match_token(TokenType::Identifier) {
            inheritance.push(self.current_token().unwrap().value.clone());
            while self.match_token(TokenType::Comma) {
                inheritance.push(self.consume(TokenType::Identifier)?.value);
            }
        }

        self.consume(TokenType::LeftBrace)?;

        let mut members = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            members.push(self.parse_contract_member()?);
        }

        self.consume(TokenType::RightBrace)?;

        Ok(self.create_node(
            AstNodeType::ContractDefinition {
                name,
                contract_type,
                inheritance,
                members,
            },
            start_line,
            start_column,
        ))
    }

    fn parse_contract_member(&mut self) -> Result<AstNode, CompilerError> {
        match self.current_token_type()? {
            TokenType::Struct => self.parse_struct_definition(),
            TokenType::Enum => self.parse_enum_definition(),
            TokenType::Event => self.parse_event_definition(),
            TokenType::Function => self.parse_function_definition(),
            TokenType::Modifier => self.parse_modifier_definition(),
            TokenType::Identifier => {
                // 可能是变量定义或状态变量
                self.parse_variable_declaration(true)
            }
            _ => Err(CompilerError::ParseError(format!(
                "Expected contract member, found {:?}",
                self.current_token_type()
            ))),
        }
    }

    fn parse_struct_definition(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume(TokenType::Struct)?;
        let name = self.consume(TokenType::Identifier)?.value;
        self.consume(TokenType::LeftBrace)?;

        let mut fields = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            fields.push(self.parse_struct_field()?);
            self.consume(TokenType::Semicolon)?;
        }

        self.consume(TokenType::RightBrace)?;

        Ok(self.create_node(
            AstNodeType::StructDefinition { name, fields },
            start_line,
            start_column,
        ))
    }

    fn parse_struct_field(&mut self) -> Result<StructField, CompilerError> {
        let type_name = self.parse_type_name()?;
        let name = self.consume(TokenType::Identifier)?.value;

        let visibility = if self.match_token(TokenType::Public) {
            Some(Visibility::Public)
        } else if self.match_token(TokenType::Private) {
            Some(Visibility::Private)
        } else if self.match_token(TokenType::Internal) {
            Some(Visibility::Internal)
        } else {
            None
        };

        Ok(StructField {
            type_name,
            name,
            visibility,
        })
    }

    fn parse_enum_definition(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume(TokenType::Enum)?;
        let name = self.consume(TokenType::Identifier)?.value;
        self.consume(TokenType::LeftBrace)?;

        let mut members = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            members.push(self.consume(TokenType::Identifier)?.value);
            if !self.check(TokenType::RightBrace) {
                self.consume(TokenType::Comma)?;
            }
        }

        self.consume(TokenType::RightBrace)?;

        Ok(self.create_node(
            AstNodeType::EnumDefinition { name, members },
            start_line,
            start_column,
        ))
    }

    fn parse_event_definition(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume(TokenType::Event)?;
        let name = self.consume(TokenType::Identifier)?.value;
        self.consume(TokenType::LeftParen)?;

        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            parameters.push(self.parse_parameter()?);
            while self.match_token(TokenType::Comma) {
                parameters.push(self.parse_parameter()?);
            }
        }

        self.consume(TokenType::RightParen)?;

        let anonymous = self.match_token(TokenType::Identifier) &&
                       self.current_token().unwrap().value == "anonymous";

        self.consume(TokenType::Semicolon)?;

        Ok(self.create_node(
            AstNodeType::EventDefinition {
                name,
                parameters,
                anonymous,
            },
            start_line,
            start_column,
        ))
    }

    fn parse_function_definition(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume(TokenType::Function)?;

        let name = if self.check(TokenType::Identifier) {
            self.consume(TokenType::Identifier)?.value
        } else {
            String::new()
        };

        let is_constructor = name == "constructor";
        let is_fallback = name == "fallback";
        let is_receive = name == "receive";

        self.consume(TokenType::LeftParen)?;

        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            parameters.push(self.parse_parameter()?);
            while self.match_token(TokenType::Comma) {
                parameters.push(self.parse_parameter()?);
            }
        }

        self.consume(TokenType::RightParen)?;

        // 解析可见性、可变性修饰符
        let mut visibility = None;
        let mut mutability = None;
        let mut modifiers = Vec::new();
        let mut return_parameters = Vec::new();

        while !self.check(TokenType::LeftBrace) && !self.check(TokenType::Semicolon) && !self.is_at_end() {
            match self.current_token_type()? {
                TokenType::Public => {
                    visibility = Some(Visibility::Public);
                    self.advance();
                }
                TokenType::Private => {
                    visibility = Some(Visibility::Private);
                    self.advance();
                }
                TokenType::Internal => {
                    visibility = Some(Visibility::Internal);
                    self.advance();
                }
                TokenType::External => {
                    visibility = Some(Visibility::External);
                    self.advance();
                }
                TokenType::Pure => {
                    mutability = Some(StateMutability::Pure);
                    self.advance();
                }
                TokenType::View => {
                    mutability = Some(StateMutability::View);
                    self.advance();
                }
                TokenType::Payable => {
                    mutability = Some(StateMutability::Payable);
                    self.advance();
                }
                TokenType::Identifier => {
                    // 修饰符调用或返回类型
                    if self.check_next TokenType::LeftParen {
                        modifiers.push(self.parse_modifier_invocation()?);
                    } else if self.match_token(TokenType::Returns) {
                        return_parameters = self.parse_return_parameters()?;
                    } else {
                        modifiers.push(self.parse_modifier_invocation()?);
                    }
                }
                _ => {
                    return Err(CompilerError::ParseError(format!(
                        "Unexpected token in function definition: {:?}",
                        self.current_token_type()
                    )));
                }
            }
        }

        let body = if self.match_token(TokenType::LeftBrace) {
            Some(self.parse_block()?)
        } else {
            self.consume(TokenType::Semicolon)?;
            None
        };

        Ok(self.create_node(
            AstNodeType::FunctionDefinition {
                name,
                visibility,
                mutability,
                modifiers,
                parameters,
                return_parameters,
                body,
                is_constructor,
                is_fallback,
                is_receive,
            },
            start_line,
            start_column,
        ))
    }

    fn parse_parameter(&mut self) -> Result<Parameter, CompilerError> {
        let mut storage_location = None;

        // 可选的存储位置
        if self.match_token(TokenType::Memory) {
            storage_location = Some(StorageLocation::Memory);
        } else if self.match_token(TokenType::Storage) {
            storage_location = Some(StorageLocation::Storage);
        } else if self.match_token(TokenType::Calldata) {
            storage_location = Some(StorageLocation::Calldata);
        }

        let type_name = self.parse_type_name()?;
        let name = if self.check(TokenType::Identifier) {
            Some(self.consume(TokenType::Identifier)?.value)
        } else {
            None
        };

        Ok(Parameter {
            type_name,
            name,
            storage_location,
        })
    }

    fn parse_return_parameters(&mut self) -> Result<Vec<Parameter>, CompilerError> {
        self.consume(TokenType::LeftParen)?;
        let mut parameters = Vec::new();

        if !self.check(TokenType::RightParen) {
            parameters.push(self.parse_parameter()?);
            while self.match_token(TokenType::Comma) {
                parameters.push(self.parse_parameter()?);
            }
        }

        self.consume(TokenType::RightParen)?;
        Ok(parameters)
    }

    fn parse_modifier_invocation(&mut self) -> Result<ModifierInvocation, CompilerError> {
        let name = self.consume(TokenType::Identifier)?.value;
        let mut arguments = Vec::new();

        if self.match_token(TokenType::LeftParen) {
            if !self.check(TokenType::RightParen) {
                arguments.push(self.parse_expression()?);
                while self.match_token(TokenType::Comma) {
                    arguments.push(self.parse_expression()?);
                }
            }
            self.consume(TokenType::RightParen)?;
        }

        Ok(ModifierInvocation { name, arguments })
    }

    fn parse_modifier_definition(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume(TokenType::Modifier)?;
        let name = self.consume(TokenType::Identifier)?.value;

        let mut parameters = Vec::new();
        if self.match_token(TokenType::LeftParen) {
            if !self.check(TokenType::RightParen) {
                parameters.push(self.parse_parameter()?);
                while self.match_token(TokenType::Comma) {
                    parameters.push(self.parse_parameter()?);
                }
            }
            self.consume(TokenType::RightParen)?;
        }

        self.consume(TokenType::LeftBrace)?;
        let body = self.parse_block()?;

        Ok(self.create_node(
            AstNodeType::ModifierDefinition {
                name,
                parameters,
                body,
            },
            start_line,
            start_column,
        ))
    }

    fn parse_variable_declaration(&mut self, is_state_var: bool) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut type_name = self.parse_type_name()?;
        let name = self.consume(TokenType::Identifier)?.value;

        let mut storage_location = None;
        let mut is_constant = false;
        let mut is_immutable = false;
        let mut visibility = None;
        let mut value = None;

        // 处理修饰符
        while !self.check(TokenType::Semicolon) && !self.check(TokenType::Assign) && !self.is_at_end() {
            match self.current_token_type()? {
                TokenType::Memory => {
                    storage_location = Some(StorageLocation::Memory);
                    self.advance();
                }
                TokenType::Storage => {
                    storage_location = Some(StorageLocation::Storage);
                    self.advance();
                }
                TokenType::Calldata => {
                    storage_location = Some(StorageLocation::Calldata);
                    self.advance();
                }
                TokenType::Constant => {
                    is_constant = true;
                    self.advance();
                }
                TokenType::Immutable => {
                    is_immutable = true;
                    self.advance();
                }
                TokenType::Public => {
                    visibility = Some(Visibility::Public);
                    self.advance();
                }
                TokenType::Private => {
                    visibility = Some(Visibility::Private);
                    self.advance();
                }
                TokenType::Internal => {
                    visibility = Some(Visibility::Internal);
                    self.advance();
                }
                TokenType::Assign => {
                    self.advance();
                    value = Some(self.parse_expression()?);
                    break;
                }
                _ => break,
            }
        }

        if is_state_var {
            self.consume(TokenType::Semicolon)?;
        }

        Ok(self.create_node(
            AstNodeType::VariableDeclaration {
                type_name,
                name,
                value,
                storage_location,
                is_state_var,
                is_constant,
                is_immutable,
                visibility,
            },
            start_line,
            start_column,
        ))
    }

    fn parse_type_name(&mut self) -> Result<TypeName, CompilerError> {
        if self.check(TokenType::Mapping) {
            self.parse_mapping_type()
        } else if self.check(TokenType::Identifier) {
            let name = self.current_token().unwrap().value.clone();
            self.advance();

            if name == "mapping" {
                // 带括号的映射类型
                self.consume(TokenType::LeftParen)?;
                let key_type = Box::new(self.parse_type_name()?);
                self.consume_identifier("=>")?;
                let value_type = Box::new(self.parse_type_name()?);
                self.consume(TokenType::RightParen)?;

                Ok(TypeName::Mapping(Mapping { key_type, value_type }))
            } else {
                Ok(TypeName::UserDefined(UserDefinedTypeName { name }))
            }
        } else {
            // 基础类型
            let name = self.consume_identifier("")?.value;
            Ok(TypeName::Elementary(ElementaryTypeName { name }))
        }
    }

    fn parse_mapping_type(&mut self) -> Result<TypeName, CompilerError> {
        self.consume(TokenType::Mapping)?;
        self.consume(TokenType::LeftParen)?;
        let key_type = Box::new(self.parse_type_name()?);
        self.consume_identifier("=>")?;
        let value_type = Box::new(self.parse_type_name()?);
        self.consume(TokenType::RightParen)?;

        Ok(TypeName::Mapping(Mapping { key_type, value_type }))
    }

    fn parse_block(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(TokenType::RightBrace)?;

        Ok(self.create_node(
            AstNodeType::Block { statements },
            start_line,
            start_column,
        ))
    }

    fn parse_statement(&mut self) -> Result<AstNode, CompilerError> {
        match self.current_token_type()? {
            TokenType::LeftBrace => {
                self.consume(TokenType::LeftBrace)?;
                self.parse_block()
            }
            TokenType::If => self.parse_if_statement(),
            TokenType::For => self.parse_for_statement(),
            TokenType::While => self.parse_while_statement(),
            TokenType::Do => self.parse_do_while_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Break => {
                let start_line = self.current_line();
                let start_column = self.current_column();
                self.consume(TokenType::Break)?;
                self.consume(TokenType::Semicolon)?;
                Ok(self.create_node(AstNodeType::BreakStatement, start_line, start_column))
            }
            TokenType::Continue => {
                let start_line = self.current_line();
                let start_column = self.current_column();
                self.consume(TokenType::Continue)?;
                self.consume(TokenType::Semicolon)?;
                Ok(self.create_node(AstNodeType::ContinueStatement, start_line, start_column))
            }
            TokenType::Emit => self.parse_emit_statement(),
            TokenType::Try => self.parse_try_statement(),
            _ => {
                // 可能是表达式语句或变量声明
                if self.is_type_name_start() {
                    self.parse_variable_declaration(false)
                } else {
                    self.parse_expression_statement()
                }
            }
        }
    }

    fn parse_if_statement(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume(TokenType::If)?;
        self.consume(TokenType::LeftParen)?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen)?;
        let true_body = self.parse_statement()?;

        let false_body = if self.match_token(TokenType::Else) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(self.create_node(
            AstNodeType::IfStatement {
                condition: Box::new(condition),
                true_body: Box::new(true_body),
                false_body,
            },
            start_line,
            start_column,
        ))
    }

    fn parse_for_statement(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume(TokenType::For)?;
        self.consume(TokenType::LeftParen)?;

        let initialization = if !self.check(TokenType::Semicolon) {
            Some(Box::new(self.parse_statement()?))
        } else {
            self.consume(TokenType::Semicolon)?;
            None
        };

        let condition = if !self.check(TokenType::Semicolon) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        self.consume(TokenType::Semicolon)?;

        let update = if !self.check(TokenType::RightParen) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        self.consume(TokenType::RightParen)?;

        let body = Box::new(self.parse_statement()?);

        Ok(self.create_node(
            AstNodeType::ForStatement {
                initialization,
                condition,
                update,
                body,
            },
            start_line,
            start_column,
        ))
    }

    fn parse_while_statement(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume(TokenType::While)?;
        self.consume(TokenType::LeftParen)?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen)?;
        let body = Box::new(self.parse_statement()?);

        Ok(self.create_node(
            AstNodeType::WhileStatement {
                condition: Box::new(condition),
                body,
            },
            start_line,
            start_column,
        ))
    }

    fn parse_do_while_statement(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume(TokenType::Do)?;
        let body = Box::new(self.parse_statement()?);
        self.consume(TokenType::While)?;
        self.consume(TokenType::LeftParen)?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen)?;
        self.consume(TokenType::Semicolon)?;

        Ok(self.create_node(
            AstNodeType::DoWhileStatement {
                body,
                condition: Box::new(condition),
            },
            start_line,
            start_column,
        ))
    }

    fn parse_return_statement(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume(TokenType::Return)?;
        let expression = if !self.check(TokenType::Semicolon) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        self.consume(TokenType::Semicolon)?;

        Ok(self.create_node(
            AstNodeType::ReturnStatement { expression },
            start_line,
            start_column,
        ))
    }

    fn parse_emit_statement(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume(TokenType::Emit)?;
        let event_call = self.parse_expression()?;
        self.consume(TokenType::Semicolon)?;

        Ok(self.create_node(
            AstNodeType::EmitStatement {
                event_call: Box::new(event_call),
            },
            start_line,
            start_column,
        ))
    }

    fn parse_try_statement(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        self.consume(TokenType::Try)?;
        let expression = self.parse_expression()?;

        let returns = if self.match_token(TokenType::Returns) {
            self.parse_return_parameters()?
        } else {
            Vec::new()
        };

        let body = Box::new(self.parse_statement()?);

        let mut catch_clauses = Vec::new();
        while self.match_token(TokenType::Catch) {
            catch_clauses.push(self.parse_catch_clause()?);
        }

        Ok(self.create_node(
            AstNodeType::TryStatement {
                expression: Box::new(expression),
                returns,
                catch_clauses,
            },
            start_line,
            start_column,
        ))
    }

    fn parse_catch_clause(&mut self) -> Result<CatchClause, CompilerError> {
        let error_name = if self.check(TokenType::Identifier) {
            Some(self.consume(TokenType::Identifier)?.value)
        } else {
            None
        };

        let mut parameters = Vec::new();
        if self.match_token(TokenType::LeftParen) {
            if !self.check(TokenType::RightParen) {
                parameters.push(self.parse_parameter()?);
                while self.match_token(TokenType::Comma) {
                    parameters.push(self.parse_parameter()?);
                }
            }
            self.consume(TokenType::RightParen)?;
        }

        let body = self.parse_statement()?;

        Ok(CatchClause {
            error_name,
            parameters,
            body,
        })
    }

    fn parse_expression_statement(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let expression = self.parse_expression()?;
        self.consume(TokenType::Semicolon)?;

        Ok(self.create_node(
            AstNodeType::ExpressionStatement {
                expression: Box::new(expression),
            },
            start_line,
            start_column,
        ))
    }

    fn parse_expression(&mut self) -> Result<AstNode, CompilerError> {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let left = self.parse_conditional_expression()?;

        if let Some(operator) = self.assignment_operator() {
            self.advance();
            let right = self.parse_assignment_expression()?;

            return Ok(self.create_node(
                AstNodeType::Assignment {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                },
                start_line,
                start_column,
            ));
        }

        Ok(left)
    }

    fn parse_conditional_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let condition = self.parse_logical_or_expression()?;

        if self.match_token(TokenType::Question) {
            let true_expression = self.parse_expression()?;
            self.consume(TokenType::Colon)?;
            let false_expression = self.parse_expression()?;

            return Ok(self.create_node(
                AstNodeType::Conditional {
                    condition: Box::new(condition),
                    true_expression: Box::new(true_expression),
                    false_expression: Box::new(false_expression),
                },
                start_line,
                start_column,
            ));
        }

        Ok(condition)
    }

    fn parse_logical_or_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut left = self.parse_logical_and_expression()?;

        while self.match_token(TokenType::Or) {
            let right = self.parse_logical_and_expression()?;
            left = self.create_node(
                AstNodeType::BinaryOperation {
                    left: Box::new(left),
                    operator: BinaryOperator::Or,
                    right: Box::new(right),
                },
                start_line,
                start_column,
            );
        }

        Ok(left)
    }

    fn parse_logical_and_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut left = self.parse_bitwise_or_expression()?;

        while self.match_token(TokenType::And) {
            let right = self.parse_bitwise_or_expression()?;
            left = self.create_node(
                AstNodeType::BinaryOperation {
                    left: Box::new(left),
                    operator: BinaryOperator::And,
                    right: Box::new(right),
                },
                start_line,
                start_column,
            );
        }

        Ok(left)
    }

    fn parse_bitwise_or_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut left = self.parse_bitwise_xor_expression()?;

        while self.match_token(TokenType::BitOr) {
            let right = self.parse_bitwise_xor_expression()?;
            left = self.create_node(
                AstNodeType::BinaryOperation {
                    left: Box::new(left),
                    operator: BinaryOperator::BitOr,
                    right: Box::new(right),
                },
                start_line,
                start_column,
            );
        }

        Ok(left)
    }

    fn parse_bitwise_xor_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut left = self.parse_bitwise_and_expression()?;

        while self.match_token(TokenType::BitXor) {
            let right = self.parse_bitwise_and_expression()?;
            left = self.create_node(
                AstNodeType::BinaryOperation {
                    left: Box::new(left),
                    operator: BinaryOperator::BitXor,
                    right: Box::new(right),
                },
                start_line,
                start_column,
            );
        }

        Ok(left)
    }

    fn parse_bitwise_and_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut left = self.parse_equality_expression()?;

        while self.match_token(TokenType::BitAnd) {
            let right = self.parse_equality_expression()?;
            left = self.create_node(
                AstNodeType::BinaryOperation {
                    left: Box::new(left),
                    operator: BinaryOperator::BitAnd,
                    right: Box::new(right),
                },
                start_line,
                start_column,
            );
        }

        Ok(left)
    }

    fn parse_equality_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut left = self.parse_relational_expression()?;

        while self.match_token(TokenType::Equal) || self.match_token(TokenType::NotEqual) {
            let operator = if self.previous_token_type() == TokenType::Equal {
                BinaryOperator::Equal
            } else {
                BinaryOperator::NotEqual
            };
            let right = self.parse_relational_expression()?;
            left = self.create_node(
                AstNodeType::BinaryOperation {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                },
                start_line,
                start_column,
            );
        }

        Ok(left)
    }

    fn parse_relational_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut left = self.parse_shift_expression()?;

        while self.match_token(TokenType::LessThan) ||
              self.match_token(TokenType::GreaterThan) ||
              self.match_token(TokenType::LessThanOrEq) ||
              self.match_token(TokenType::GreaterThanOrEq) {
            let operator = match self.previous_token_type() {
                TokenType::LessThan => BinaryOperator::LessThan,
                TokenType::GreaterThan => BinaryOperator::GreaterThan,
                TokenType::LessThanOrEq => BinaryOperator::LessThanOrEqual,
                TokenType::GreaterThanOrEq => BinaryOperator::GreaterThanOrEqual,
                _ => unreachable!(),
            };
            let right = self.parse_shift_expression()?;
            left = self.create_node(
                AstNodeType::BinaryOperation {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                },
                start_line,
                start_column,
            );
        }

        Ok(left)
    }

    fn parse_shift_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut left = self.parse_additive_expression()?;

        while self.match_token(TokenType::LeftShift) || self.match_token(TokenType::RightShift) {
            let operator = if self.previous_token_type() == TokenType::LeftShift {
                BinaryOperator::LeftShift
            } else {
                BinaryOperator::RightShift
            };
            let right = self.parse_additive_expression()?;
            left = self.create_node(
                AstNodeType::BinaryOperation {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                },
                start_line,
                start_column,
            );
        }

        Ok(left)
    }

    fn parse_additive_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut left = self.parse_multiplicative_expression()?;

        while self.match_token(TokenType::Plus) || self.match_token(TokenType::Minus) {
            let operator = if self.previous_token_type() == TokenType::Plus {
                BinaryOperator::Add
            } else {
                BinaryOperator::Sub
            };
            let right = self.parse_multiplicative_expression()?;
            left = self.create_node(
                AstNodeType::BinaryOperation {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                },
                start_line,
                start_column,
            );
        }

        Ok(left)
    }

    fn parse_multiplicative_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut left = self.parse_power_expression()?;

        while self.match_token(TokenType::Multiply) ||
              self.match_token(TokenType::Divide) ||
              self.match_token(TokenType::Modulo) {
            let operator = match self.previous_token_type() {
                TokenType::Multiply => BinaryOperator::Mul,
                TokenType::Divide => BinaryOperator::Div,
                TokenType::Modulo => BinaryOperator::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_power_expression()?;
            left = self.create_node(
                AstNodeType::BinaryOperation {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                },
                start_line,
                start_column,
            );
        }

        Ok(left)
    }

    fn parse_power_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut left = self.parse_unary_expression()?;

        while self.match_token(TokenType::Power) {
            let right = self.parse_unary_expression()?;
            left = self.create_node(
                AstNodeType::BinaryOperation {
                    left: Box::new(left),
                    operator: BinaryOperator::Power,
                    right: Box::new(right),
                },
                start_line,
                start_column,
            );
        }

        Ok(left)
    }

    fn parse_unary_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        if let Some(operator) = self.unary_operator() {
            self.advance();
            let sub_expression = self.parse_unary_expression()?;

            return Ok(self.create_node(
                AstNodeType::UnaryOperation {
                    operator,
                    sub_expression: Box::new(sub_expression),
                    prefix: true,
                },
                start_line,
                start_column,
            ));
        }

        self.parse_postfix_expression()
    }

    fn parse_postfix_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let mut left = self.parse_primary_expression()?;

        loop {
            if self.match_token(TokenType::LeftParen) {
                // 函数调用
                let mut arguments = Vec::new();
                if !self.check(TokenType::RightParen) {
                    arguments.push(self.parse_expression()?);
                    while self.match_token(TokenType::Comma) {
                        arguments.push(self.parse_expression()?);
                    }
                }
                self.consume(TokenType::RightParen)?;

                left = self.create_node(
                    AstNodeType::FunctionCall {
                        function: Box::new(left),
                        arguments,
                        names: Vec::new(),
                    },
                    start_line,
                    start_column,
                );
            } else if self.match_token(TokenType::LeftBracket) {
                // 数组索引
                let index = self.parse_expression()?;
                self.consume(TokenType::RightBracket)?;

                left = self.create_node(
                    AstNodeType::IndexAccess {
                        base: Box::new(left),
                        index: Box::new(index),
                    },
                    start_line,
                    start_column,
                );
            } else if self.match_token(TokenType::Dot) {
                // 成员访问
                let member_name = self.consume(TokenType::Identifier)?.value;
                left = self.create_node(
                    AstNodeType::MemberAccess {
                        object: Box::new(left),
                        member_name,
                    },
                    start_line,
                    start_column,
                );
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_primary_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        match self.current_token_type()? {
            TokenType::Literal(literal_type) => {
                let value = self.advance()?.value.clone();
                Ok(self.create_node(
                    AstNodeType::Literal { value, literal_type },
                    start_line,
                    start_column,
                ))
            }
            TokenType::StringLiteral => {
                let value = self.advance()?.value.clone();
                Ok(self.create_node(
                    AstNodeType::Literal {
                        value,
                        literal_type: LiteralType::String,
                    },
                    start_line,
                    start_column,
                ))
            }
            TokenType::Identifier => {
                let name = self.advance()?.value.clone();
                if name == "new" {
                    self.parse_new_expression()
                } else {
                    Ok(self.create_node(
                        AstNodeType::Identifier { name },
                        start_line,
                        start_column,
                    ))
                }
            }
            TokenType::LeftParen => {
                self.advance();
                let expression = self.parse_expression()?;
                self.consume(TokenType::RightParen)?;
                Ok(expression)
            }
            TokenType::LeftBracket => {
                self.advance();
                let mut components = Vec::new();
                if !self.check(TokenType::RightBracket) {
                    components.push(Some(self.parse_expression()?));
                    while self.match_token(TokenType::Comma) {
                        if self.check(TokenType::RightBracket) {
                            components.push(None);
                        } else {
                            components.push(Some(self.parse_expression()?));
                        }
                    }
                }
                self.consume(TokenType::RightBracket)?;
                Ok(self.create_node(
                    AstNodeType::TupleExpression { components },
                    start_line,
                    start_column,
                ))
            }
            _ => Err(CompilerError::ParseError(format!(
                "Unexpected token in expression: {:?}",
                self.current_token_type()
            ))),
        }
    }

    fn parse_new_expression(&mut self) -> Result<AstNode, CompilerError> {
        let start_line = self.current_line();
        let start_column = self.current_column();

        let contract_name = self.consume(TokenType::Identifier)?.value;

        Ok(self.create_node(
            AstNodeType::NewExpression { contract_name },
            start_line,
            start_column,
        ))
    }

    // 辅助方法
    fn assignment_operator(&self) -> Option<AssignmentOperator> {
        match self.current_token_type() {
            TokenType::Assign => Some(AssignmentOperator::Assign),
            TokenType::PlusAssign => Some(AssignmentOperator::AddAssign),
            TokenType::MinusAssign => Some(AssignmentOperator::SubAssign),
            TokenType::MultiplyAssign => Some(AssignmentOperator::MulAssign),
            TokenType::DivideAssign => Some(AssignmentOperator::DivAssign),
            TokenType::ModuloAssign => Some(AssignmentOperator::ModAssign),
            TokenType::LeftShift => Some(AssignmentOperator::LeftShiftAssign),
            TokenType::RightShift => Some(AssignmentOperator::RightShiftAssign),
            _ => None,
        }
    }

    fn unary_operator(&self) -> Option<UnaryOperator> {
        match self.current_token_type() {
            TokenType::Plus => Some(UnaryOperator::Plus),
            TokenType::Minus => Some(UnaryOperator::Minus),
            TokenType::Not => Some(UnaryOperator::Not),
            TokenType::BitNot => Some(UnaryOperator::BitNot),
            _ => None,
        }
    }

    fn is_type_name_start(&self) -> bool {
        match self.current_token_type() {
            TokenType::Address | TokenType::Bool | TokenType::String | TokenType::Bytes |
            TokenType::Int | TokenType::Uint | TokenType::Fixed | TokenType::Ufixed => true,
            TokenType::Identifier => {
                if let Some(token) = &self.current_token {
                    // 检查是否是用户定义类型名
                    ["mapping", "contract", "interface", "library", "struct", "enum"].contains(&token.value.as_str())
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn create_node(&self, node_type: AstNodeType, line: usize, column: usize) -> AstNode {
        AstNode {
            node_type,
            line,
            column,
            source_span: crate::compiler::lexer::SourceSpan { start: 0, end: 0 },
        }
    }

    fn advance(&mut self) -> Option<Token> {
        self.current_token = self.peek_token.take();
        self.peek_token = self.lexer.next();
        self.current_token.clone()
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.current_token_type() == token_type
    }

    fn consume(&mut self, token_type: TokenType) -> Result<Token, CompilerError> {
        if self.check(token_type) {
            Ok(self.advance().unwrap())
        } else {
            Err(CompilerError::ParseError(format!(
                "Expected {:?}, found {:?}",
                token_type,
                self.current_token_type()
            )))
        }
    }

    fn consume_identifier(&mut self, expected: &str) -> Result<Token, CompilerError> {
        if self.check(TokenType::Identifier) {
            let token = self.advance().unwrap();
            if !expected.is_empty() && token.value != expected {
                return Err(CompilerError::ParseError(format!(
                    "Expected identifier '{}', found '{}'",
                    expected, token.value
                )));
            }
            Ok(token)
        } else {
            Err(CompilerError::ParseError(format!(
                "Expected identifier '{}'",
                expected
            )))
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_token_type() == TokenType::EndOfFile
    }

    fn current_token(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }

    fn current_token_type(&self) -> TokenType {
        self.current_token
            .as_ref()
            .map(|token| token.token_type.clone())
            .unwrap_or(TokenType::EndOfFile)
    }

    fn previous_token_type(&self) -> TokenType {
        self.current_token_type()
    }

    fn current_line(&self) -> usize {
        self.current_token
            .as_ref()
            .map(|token| token.line)
            .unwrap_or(1)
    }

    fn current_column(&self) -> usize {
        self.current_token
            .as_ref()
            .map(|token| token.column)
            .unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::lexer::Lexer;

    #[test]
    fn test_simple_contract() {
        let source = r#"
        pragma solidity ^0.8.0;

        contract Simple {
            uint256 public value;

            function setValue(uint256 _value) public {
                value = _value;
            }

            function getValue() public view returns (uint256) {
                return value;
            }
        }
        "#;

        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().expect("Parsing should succeed");

        match ast.node_type {
            AstNodeType::SourceUnit { contract_definitions, .. } => {
                assert_eq!(contract_definitions.len(), 1);
            }
            _ => panic!("Expected SourceUnit"),
        }
    }

    #[test]
    fn test_function_parsing() {
        let source = r#"
        function add(uint256 a, uint256 b) public pure returns (uint256) {
            return a + b;
        }
        "#;

        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().expect("Parsing should succeed");

        match ast.node_type {
            AstNodeType::SourceUnit { contract_definitions, .. } => {
                assert_eq!(contract_definitions.len(), 1);
            }
            _ => panic!("Expected SourceUnit"),
        }
    }
}