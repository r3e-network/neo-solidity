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
//! - `interop` - Neo N3 interop service identifiers (syscall IDs)
//!
//! Author: Jimmy <jimmy@r3e.network>

// Allow referring to this crate by name (`neo_devpack_solidity::...`) internally.
extern crate self as neo_devpack_solidity;

// Core compilation modules
pub mod ir;
pub mod solidity;

// Public CLI APIs (standard-json, NEF/manifest output, etc.)
pub mod cli;

// Neo N3 interop service identifiers
pub mod interop;

// Runtime and execution
pub mod runtime;

// Neo-specific utilities
pub mod neo;
pub mod storage_key;

// Supporting modules
pub mod frontend;
pub mod semantic_model;
pub mod type_system;
pub mod utils;

/// Compiler version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
