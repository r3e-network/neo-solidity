//! Call Graph Analysis
//!
//! Analyzes function call relationships.

use std::collections::{HashMap, HashSet};

/// Call graph node
#[derive(Debug, Default)]
pub struct CallNode {
    pub callers: HashSet<String>,
    pub callees: HashSet<String>,
}

/// Call graph
#[derive(Default)]
pub struct CallGraph {
    nodes: HashMap<String, CallNode>,
}

impl CallGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_call(&mut self, caller: &str, callee: &str) {
        self.nodes
            .entry(caller.to_string())
            .or_default()
            .callees
            .insert(callee.to_string());
        self.nodes
            .entry(callee.to_string())
            .or_default()
            .callers
            .insert(caller.to_string());
    }

    pub fn is_recursive(&self, func: &str) -> bool {
        self.reachable(func, func, &mut HashSet::new())
    }

    fn reachable(&self, from: &str, to: &str, visited: &mut HashSet<String>) -> bool {
        if !visited.insert(from.to_string()) {
            return false;
        }
        if let Some(node) = self.nodes.get(from) {
            if node.callees.contains(to) {
                return true;
            }
            for callee in &node.callees {
                if self.reachable(callee, to, visited) {
                    return true;
                }
            }
        }
        false
    }
}
