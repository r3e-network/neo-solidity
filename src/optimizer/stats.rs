//! Optimization Statistics
//!
//! Aggregates statistics from all optimization passes.

/// Combined optimization statistics
#[derive(Debug, Default, Clone)]
pub struct CombinedStats {
    pub constants_folded: u32,
    pub dead_code_removed: u32,
    pub functions_inlined: u32,
    pub cse_eliminated: u32,
    pub strength_reduced: u32,
    pub loops_optimized: u32,
    pub total_passes: u32,
}

impl CombinedStats {
    pub fn total_optimizations(&self) -> u32 {
        self.constants_folded
            + self.dead_code_removed
            + self.functions_inlined
            + self.cse_eliminated
            + self.strength_reduced
            + self.loops_optimized
    }
}
