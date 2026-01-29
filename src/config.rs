//! Compiler Configuration
//!
//! Centralized configuration for all compiler options.

/// Compiler configuration
#[derive(Debug, Clone)]
pub struct CompilerConfig {
    pub optimization_level: u8,
    pub debug_info: bool,
    pub warnings_as_errors: bool,
    pub max_contract_size: usize,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            optimization_level: 2,
            debug_info: false,
            warnings_as_errors: false,
            max_contract_size: 512 * 1024,
        }
    }
}
