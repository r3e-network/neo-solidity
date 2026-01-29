//! Loop Optimization Module
//!
//! Optimizes loop constructs for better performance.

use crate::parser::{AstNode, AstNodeType};

/// Loop optimization statistics
#[derive(Debug, Default)]
pub struct LoopStats {
    pub loops_analyzed: u32,
    pub invariants_hoisted: u32,
    pub loops_unrolled: u32,
}

/// Loop optimizer
#[derive(Default)]
pub struct LoopOptimizer {
    stats: LoopStats,
    /// Max iterations for loop unrolling
    unroll_threshold: usize,
}

impl LoopOptimizer {
    pub fn new() -> Self {
        Self {
            stats: LoopStats::default(),
            unroll_threshold: 4,
        }
    }

    pub fn stats(&self) -> &LoopStats {
        &self.stats
    }

    /// Check if expression is loop-invariant
    pub fn is_invariant(&self, node: &AstNode, modified: &[String]) -> bool {
        match &node.node_type {
            AstNodeType::Literal { .. } => true,
            AstNodeType::Identifier { name } => !modified.contains(name),
            AstNodeType::FunctionCall { name, arguments } => {
                is_pure_function(name) && 
                arguments.iter().all(|a| self.is_invariant(a, modified))
            }
            _ => false,
        }
    }
}

fn is_pure_function(name: &str) -> bool {
    matches!(name, "add" | "sub" | "mul" | "div" | "mod" |
        "eq" | "lt" | "gt" | "and" | "or" | "xor")
}
