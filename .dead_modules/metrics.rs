//! Code Metrics Module
//!
//! Calculates code complexity and quality metrics.

/// Code metrics
#[derive(Debug, Default, Clone)]
pub struct CodeMetrics {
    pub lines_of_code: usize,
    pub functions: usize,
    pub contracts: usize,
    pub cyclomatic_complexity: u32,
    pub max_nesting_depth: u32,
}

impl CodeMetrics {
    pub fn avg_complexity(&self) -> f64 {
        if self.functions == 0 {
            0.0
        } else {
            self.cyclomatic_complexity as f64 / self.functions as f64
        }
    }
}
