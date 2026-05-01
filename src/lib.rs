//! Neo DevPack for Solidity Library
//!
//! A Solidity to NeoVM bytecode compiler for Neo N3 smart contracts.
//!
//! # Architecture
//!
//! The compiler is organized into the following modules:
//!
//! - `solidity` - Solidity source parsing and metadata extraction
//! - `ir` - Intermediate representation for code generation
//! - `runtime` - NeoVM execution environment
//! - `neo` - Neo-specific utilities (NEF, manifest)
//! - `codegen`, `lexer`, `parser`, `optimizer`, `semantic` - Yul language support
//!
//! Author: Jimmy <jimmy@r3e.network>

// Allow referring to this crate by name (`neo_devpack_solidity::...`) internally.
extern crate self as neo_devpack_solidity;

// Core compilation modules
pub mod ir;
pub mod solidity;

// Public CLI APIs (standard-json, NEF/manifest output, etc.)
pub mod cli;

// Yul language support modules
pub mod codegen;
pub mod lexer;
pub mod optimizer;
pub mod parser;
pub mod semantic;

// Runtime and execution
pub mod runtime;

// Neo-specific utilities
pub mod neo;
pub mod storage_key;

// Supporting modules
pub mod error;
pub mod frontend;
pub mod semantic_model;
pub mod type_system;
pub mod types;

// Supporting analysis and quality modules
pub mod codegen_helpers;
pub mod docs;
pub mod security;
pub mod testing;
pub mod utils;
pub mod validation;
pub mod warning;

// Public re-exports
pub use error::*;
pub use types::*;

/// Compiler version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
