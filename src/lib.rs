//! Neo Solidity Compiler Library
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

// Allow referring to this crate by name (`neo_solidity::...`) internally.
extern crate self as neo_solidity;

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

// New optimization and analysis modules
pub mod benchmark;
pub mod bounds;
pub mod docs;
pub mod security;
pub mod testing;
pub mod utils;
pub mod validation;
pub mod warning;

// Analysis modules
pub mod abi_opt;
pub mod cache;
pub mod callgraph;
pub mod cfg;
pub mod codegen_helpers;
pub mod config;
pub mod context;
pub mod dataflow;
pub mod diagnostic_fmt;
pub mod liveness;
pub mod metrics;
pub mod pipeline;
pub mod regalloc;
pub mod scheduler;
pub mod sourcemap;
pub mod storage_opt;

// Public re-exports
pub use error::*;
pub use types::*;

/// Compiler version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
