use crate::error::CompilerError;
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub struct AstNode {
    pub node_type: AstNodeType,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum AstNodeType {
    Object { statements: Vec<AstNode> },
    Function { name: String, params: Vec<String>, returns: Vec<String>, body: Box<AstNode> },
    Block { statements: Vec<AstNode> },
    If { condition: Box<AstNode>, then_branch: Box<AstNode>, else_branch: Option<Box<AstNode>> },
    For { init: Option<Box<AstNode>>, condition: Box<AstNode>, update: Option<Box<AstNode>>, body: Box<AstNode> },
    Switch { expression: Box<AstNode>, cases: Vec<SwitchCase>, default: Option<Box<AstNode>> },
    FunctionCall { name: String, arguments: Vec<AstNode> },
    Assignment { targets: Vec<String>, value: Box<AstNode> },
    Identifier { name: String },
    Literal { value: String },
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
        Self { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Result<AstNode, CompilerError> {
        Ok(AstNode {
            node_type: AstNodeType::Object { statements: vec![] },
            line: 1,
            column: 1,
        })
    }
}
