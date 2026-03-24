// Common Subexpression Elimination (CSE)
//
// Identifies and eliminates redundant computations.

use std::collections::HashMap;

use crate::parser::{AstNode, AstNodeType};

/// Expression hash for CSE
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ExprHash {
    op: String,
    args: Vec<ExprHash>,
    literal: Option<String>,
}

impl ExprHash {
    pub fn from_node(node: &AstNode) -> Option<Self> {
        match &node.node_type {
            AstNodeType::Literal { value } => Some(Self {
                op: "lit".to_string(),
                args: vec![],
                literal: Some(value.clone()),
            }),
            AstNodeType::Identifier { name } => Some(Self {
                op: "id".to_string(),
                args: vec![],
                literal: Some(name.clone()),
            }),
            AstNodeType::FunctionCall { name, arguments } => {
                // Only hash pure operations
                if !is_pure_op(name) {
                    return None;
                }
                let args: Option<Vec<_>> = arguments.iter().map(Self::from_node).collect();
                Some(Self {
                    op: name.clone(),
                    args: args?,
                    literal: None,
                })
            }
            _ => None,
        }
    }
}

pub(super) fn is_pure_op(name: &str) -> bool {
    matches!(
        name,
        "add"
            | "sub"
            | "mul"
            | "div"
            | "mod"
            | "eq"
            | "lt"
            | "gt"
            | "le"
            | "ge"
            | "and"
            | "or"
            | "xor"
            | "not"
            | "shl"
            | "shr"
            | "sar"
    )
}

/// CSE optimizer state
#[derive(Default)]
pub struct CseOptimizer {
    /// Map from expression hash to temp variable name
    cache: HashMap<ExprHash, String>,
    counter: usize,
}

impl CseOptimizer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate a unique temp variable name
    fn gen_temp(&mut self) -> String {
        let name = format!("_cse_{}", self.counter);
        self.counter += 1;
        name
    }

    /// Check if expression is already computed
    pub fn lookup(&self, node: &AstNode) -> Option<&String> {
        ExprHash::from_node(node).and_then(|h| self.cache.get(&h))
    }

    /// Register a computed expression
    pub fn register(&mut self, node: &AstNode) -> Option<String> {
        let hash = ExprHash::from_node(node)?;
        if self.cache.contains_key(&hash) {
            return None;
        }
        let temp = self.gen_temp();
        self.cache.insert(hash, temp.clone());
        Some(temp)
    }
}
