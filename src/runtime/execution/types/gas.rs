//! Gas tracking types for execution context.
//!
//! Provides structures for tracking gas consumption during execution.

use std::collections::HashMap;

/// Gas tracker for execution costs
#[derive(Debug)]
pub struct GasTracker {
    pub(crate) limit: u64,
    pub(crate) used: u64,
    pub(crate) base_cost: u64,
    pub(crate) operation_costs: HashMap<String, u64>,
}
