//! Source Map Module
//!
//! Maps bytecode positions to source code locations.

use crate::error::SourceLocation;

/// Source map entry
#[derive(Debug, Clone)]
pub struct SourceMapEntry {
    pub bytecode_offset: usize,
    pub source_location: SourceLocation,
}

/// Source map builder
#[derive(Default)]
pub struct SourceMapBuilder {
    entries: Vec<SourceMapEntry>,
}

impl SourceMapBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, offset: usize, loc: SourceLocation) {
        self.entries.push(SourceMapEntry {
            bytecode_offset: offset,
            source_location: loc,
        });
    }

    pub fn build(self) -> SourceMap {
        SourceMap {
            entries: self.entries,
        }
    }
}

/// Compiled source map
pub struct SourceMap {
    entries: Vec<SourceMapEntry>,
}

impl SourceMap {
    pub fn lookup(&self, offset: usize) -> Option<&SourceLocation> {
        self.entries
            .iter()
            .rev()
            .find(|e| e.bytecode_offset <= offset)
            .map(|e| &e.source_location)
    }
}
