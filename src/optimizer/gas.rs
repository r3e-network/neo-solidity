//! Gas Optimization Module
//!
//! Optimizes bytecode for reduced gas consumption on NeoVM.

use std::collections::HashMap;

/// Gas costs for common operations
#[derive(Debug, Clone, Copy)]
pub struct GasCosts {
    pub push: u64,
    pub pop: u64,
    pub dup: u64,
    pub swap: u64,
    pub add: u64,
    pub mul: u64,
    pub div: u64,
    pub load_local: u64,
    pub store_local: u64,
    pub jump: u64,
    pub call: u64,
    pub syscall: u64,
}

impl Default for GasCosts {
    fn default() -> Self {
        Self {
            push: 1,
            pop: 1,
            dup: 1,
            swap: 1,
            add: 2,
            mul: 2,
            div: 2,
            load_local: 1,
            store_local: 1,
            jump: 2,
            call: 512,
            syscall: 32768,
        }
    }
}

/// Gas optimization statistics
#[derive(Debug, Default)]
pub struct GasStats {
    pub estimated_before: u64,
    pub estimated_after: u64,
    pub optimizations_applied: u32,
}

impl GasStats {
    pub fn savings(&self) -> u64 {
        self.estimated_before.saturating_sub(self.estimated_after)
    }

    pub fn savings_percent(&self) -> f64 {
        if self.estimated_before == 0 {
            return 0.0;
        }
        (self.savings() as f64 / self.estimated_before as f64) * 100.0
    }
}

/// Gas optimizer
pub struct GasOptimizer {
    costs: GasCosts,
    stats: GasStats,
}

impl GasOptimizer {
    pub fn new() -> Self {
        Self {
            costs: GasCosts::default(),
            stats: GasStats::default(),
        }
    }

    pub fn with_costs(costs: GasCosts) -> Self {
        Self {
            costs,
            stats: GasStats::default(),
        }
    }

    pub fn stats(&self) -> &GasStats {
        &self.stats
    }
}

impl Default for GasOptimizer {
    fn default() -> Self {
        Self::new()
    }
}
