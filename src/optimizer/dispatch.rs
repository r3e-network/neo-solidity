use crate::error::CompilerError;
use crate::parser::{AstNode, AstNodeType};

use super::types::Optimizer;

impl Optimizer {
    /// Create a new optimizer with the specified level (0-3)
    pub fn new(level: u8) -> Self {
        use super::types::{OptimizationPasses, OptimizationStats};
        Self {
            level,
            stats: OptimizationStats::default(),
            inline_threshold: 50,
            enabled_passes: OptimizationPasses::for_level(level),
        }
    }

    /// Create optimizer with custom settings
    pub fn with_config(level: u8, inline_threshold: usize) -> Self {
        use super::types::{OptimizationPasses, OptimizationStats};
        Self {
            level,
            stats: OptimizationStats::default(),
            inline_threshold,
            enabled_passes: OptimizationPasses::for_level(level),
        }
    }

    /// Run all enabled optimization passes
    pub fn optimize(&mut self, mut ast: AstNode) -> Result<AstNode, CompilerError> {
        if self.level == 0 {
            return Ok(ast);
        }

        // Track initial node count
        self.stats.nodes_before = self.count_nodes(&ast);

        // Run enabled passes
        if self.enabled_passes.constant_folding {
            ast = self.constant_folding(ast)?;
            self.stats.passes_run += 1;
        }

        if self.enabled_passes.dead_code_elimination {
            ast = self.dead_code_elimination(ast)?;
            self.stats.passes_run += 1;
        }

        if self.enabled_passes.function_inlining {
            ast = self.function_inlining(ast)?;
            self.stats.passes_run += 1;
        }

        if self.enabled_passes.common_subexpression {
            ast = self.common_subexpression_elimination(ast)?;
            self.stats.passes_run += 1;
        }

        // Track final node count
        self.stats.nodes_after = self.count_nodes(&ast);

        Ok(ast)
    }

    /// Get optimization statistics
    pub fn get_stats(&self) -> &super::types::OptimizationStats {
        &self.stats
    }

    /// Get current optimization level
    pub fn level(&self) -> u8 {
        self.level
    }

    /// Count AST nodes for statistics
    #[allow(clippy::only_used_in_recursion)]
    pub(super) fn count_nodes(&self, node: &AstNode) -> usize {
        let mut count = 1;
        match &node.node_type {
            AstNodeType::Object { statements } => {
                for stmt in statements {
                    count += self.count_nodes(stmt);
                }
            }
            AstNodeType::Block { statements } => {
                for stmt in statements {
                    count += self.count_nodes(stmt);
                }
            }
            AstNodeType::If {
                condition,
                then_branch,
                else_branch,
            } => {
                count += self.count_nodes(condition);
                count += self.count_nodes(then_branch);
                if let Some(eb) = else_branch {
                    count += self.count_nodes(eb);
                }
            }
            AstNodeType::FunctionCall { arguments, .. } => {
                for arg in arguments {
                    count += self.count_nodes(arg);
                }
            }
            _ => {}
        }
        count
    }
}
