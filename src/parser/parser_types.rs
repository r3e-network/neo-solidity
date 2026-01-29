/// AST node with source location
#[derive(Debug, Clone)]
pub struct AstNode {
    pub node_type: AstNodeType,
    pub line: usize,
    pub column: usize,
}

impl AstNode {
    /// Create a new AST node
    pub fn new(node_type: AstNodeType, line: usize, column: usize) -> Self {
        Self { node_type, line, column }
    }

    /// Check if this is a function node
    pub fn is_function(&self) -> bool {
        matches!(self.node_type, AstNodeType::Function { .. })
    }

    /// Check if this is a literal node
    pub fn is_literal(&self) -> bool {
        matches!(self.node_type, AstNodeType::Literal { .. })
    }

    /// Get the node type name
    pub fn type_name(&self) -> &'static str {
        match &self.node_type {
            AstNodeType::Object { .. } => "object",
            AstNodeType::Function { .. } => "function",
            AstNodeType::Block { .. } => "block",
            AstNodeType::If { .. } => "if",
            AstNodeType::For { .. } => "for",
            AstNodeType::Switch { .. } => "switch",
            AstNodeType::FunctionCall { .. } => "call",
            AstNodeType::Assignment { .. } => "assignment",
            AstNodeType::Identifier { .. } => "identifier",
            AstNodeType::Literal { .. } => "literal",
        }
    }
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

/// Switch case in a switch statement
#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub value: AstNode,
    pub body: AstNode,
}

impl SwitchCase {
    /// Create a new switch case
    pub fn new(value: AstNode, body: AstNode) -> Self {
        Self { value, body }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

