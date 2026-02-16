//! Compilation Cache Module
//!
//! Caches compilation results for incremental builds.

use std::collections::HashMap;

/// Cache entry
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub hash: String,
    pub bytecode: Vec<u8>,
    pub timestamp: u64,
}

/// Compilation cache
#[derive(Default)]
pub struct CompilationCache {
    entries: HashMap<String, CacheEntry>,
}

impl CompilationCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<&CacheEntry> {
        self.entries.get(key)
    }

    pub fn insert(&mut self, key: String, entry: CacheEntry) {
        self.entries.insert(key, entry);
    }

    pub fn invalidate(&mut self, key: &str) {
        self.entries.remove(key);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
