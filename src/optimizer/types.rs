use crate::parser::AstNode;

/// Yul AST optimizer with configurable optimization levels
pub struct Optimizer {
    pub(super) level: u8,
    pub(super) stats: OptimizationStats,
    /// Maximum function size for inlining (in AST nodes)
    pub(super) inline_threshold: usize,
    /// Track which optimizations are enabled
    pub(super) enabled_passes: OptimizationPasses,
}

/// Statistics collected during optimization
#[derive(Debug, Clone, Default)]
pub struct OptimizationStats {
    pub eliminated_instructions: u32,
    pub inlined_functions: u32,
    pub folded_constants: u32,
    /// Number of optimization passes run
    pub passes_run: u32,
    /// Total AST nodes before optimization
    pub nodes_before: usize,
    /// Total AST nodes after optimization
    pub nodes_after: usize,
}

impl OptimizationStats {
    /// Calculate the reduction percentage
    pub fn reduction_percent(&self) -> f64 {
        if self.nodes_before == 0 {
            return 0.0;
        }
        let reduced = self.nodes_before.saturating_sub(self.nodes_after);
        (reduced as f64 / self.nodes_before as f64) * 100.0
    }

    /// Merge stats from another optimization pass
    pub fn merge(&mut self, other: &OptimizationStats) {
        self.eliminated_instructions += other.eliminated_instructions;
        self.inlined_functions += other.inlined_functions;
        self.folded_constants += other.folded_constants;
        self.passes_run += other.passes_run;
    }
}

/// Bitflags for enabled optimization passes
#[derive(Debug, Clone, Copy, Default)]
pub struct OptimizationPasses {
    pub constant_folding: bool,
    pub dead_code_elimination: bool,
    pub function_inlining: bool,
    pub common_subexpression: bool,
}

impl OptimizationPasses {
    /// Create passes for a given optimization level
    pub fn for_level(level: u8) -> Self {
        match level {
            0 => Self::default(),
            1 => Self {
                constant_folding: true,
                ..Default::default()
            },
            2 => Self {
                constant_folding: true,
                dead_code_elimination: true,
                ..Default::default()
            },
            _ => Self {
                constant_folding: true,
                dead_code_elimination: true,
                function_inlining: true,
                common_subexpression: true,
            },
        }
    }
}

/// Represents a function that can be inlined
#[allow(dead_code)]
pub(super) struct InlineCandidate {
    /// Parameter names in order
    pub(super) params: Vec<String>,
    /// Function body to inline
    pub(super) body: AstNode,
    /// Estimated cost of inlining (for future use)
    pub(super) cost: usize,
}
