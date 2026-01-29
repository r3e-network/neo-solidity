//! Compiler Context
//!
//! Shared state during compilation.

use std::collections::HashMap;

/// Compilation context
#[derive(Default)]
pub struct CompilerContext {
    pub symbols: HashMap<String, String>,
    pub errors_count: usize,
    pub warnings_count: usize,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_errors(&self) -> bool {
        self.errors_count > 0
    }
}
