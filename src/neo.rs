//! Neo N3 Utilities Module
//!
//! Provides Neo N3 blockchain-specific utilities for NEF (Neo Executable Format)
//! generation and method tokens.
//!
//! # Key Components
//!
//! - [`MethodToken`] - Cross-contract call optimization tokens
//! - [`build_nef_with_tokens`] - NEF file generation with method tokens
//! - [`clamp_nef_source_with_flag`] - Source code embedding in NEF
//!
//! # NEF Format
//!
//! The Neo Executable Format (NEF) is the standard container for Neo N3 smart
//! contracts, containing bytecode, metadata, and optional method tokens.

mod constants;
mod encoding;
mod method_token;
mod build;
mod source;
mod contract_hash;

// Re-export public API
pub use constants::{
    MAX_METHOD_TOKENS, MAX_TOKEN_METHOD_LENGTH, MAX_CALL_FLAGS,
    NEF_SOURCE_MAX_BYTES, NEF_MAGIC, NEF_HEADER_SIZE, MAX_SCRIPT_SIZE,
};
pub use method_token::MethodToken;
pub use build::{build_nef, build_nef_with_tokens};
pub use source::{clamp_nef_source, clamp_nef_source_with_flag};
pub use contract_hash::{compute_contract_hash, parse_uint160_hex_be, format_uint160_hex_be};

#[cfg(test)]
mod tests;
