pub struct Optimizer {
    level: u8,
    stats: OptimizationStats,
}

pub struct OptimizationStats {
    pub eliminated_instructions: u32,
    pub inlined_functions: u32,
    pub folded_constants: u32,
}

/// Represents a function that can be inlined
struct InlineCandidate {
    /// Parameter names in order
    params: Vec<String>,
    /// Function body to inline
    body: AstNode,
}

