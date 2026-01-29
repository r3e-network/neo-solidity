//! Control Flow Graph
//!
//! Represents program control flow for analysis.

use std::collections::HashMap;

/// Basic block
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: usize,
    pub predecessors: Vec<usize>,
    pub successors: Vec<usize>,
}

/// Control flow graph
#[derive(Default)]
pub struct ControlFlowGraph {
    blocks: HashMap<usize, BasicBlock>,
    entry: Option<usize>,
}

impl ControlFlowGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_block(&mut self, id: usize) {
        self.blocks.insert(id, BasicBlock {
            id,
            predecessors: Vec::new(),
            successors: Vec::new(),
        });
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if let Some(b) = self.blocks.get_mut(&from) {
            b.successors.push(to);
        }
        if let Some(b) = self.blocks.get_mut(&to) {
            b.predecessors.push(from);
        }
    }

    pub fn set_entry(&mut self, id: usize) {
        self.entry = Some(id);
    }
}
