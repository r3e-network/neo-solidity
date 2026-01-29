//! Liveness Analysis
//!
//! Determines which variables are live at each point.

use std::collections::HashSet;

/// Liveness info for a block
#[derive(Debug, Default, Clone)]
pub struct LivenessInfo {
    pub live_in: HashSet<String>,
    pub live_out: HashSet<String>,
}

impl LivenessInfo {
    pub fn is_live(&self, var: &str) -> bool {
        self.live_in.contains(var) || self.live_out.contains(var)
    }
}
