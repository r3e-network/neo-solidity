//! Data Flow Analysis
//!
//! Tracks variable definitions and uses.

use std::collections::HashSet;

/// Variable state
#[derive(Debug, Clone, Default)]
pub struct VarState {
    pub defined: HashSet<String>,
    pub used: HashSet<String>,
}

impl VarState {
    pub fn define(&mut self, name: &str) {
        self.defined.insert(name.to_string());
    }

    pub fn use_var(&mut self, name: &str) {
        self.used.insert(name.to_string());
    }

    pub fn unused(&self) -> Vec<&String> {
        self.defined.difference(&self.used).collect()
    }
}
