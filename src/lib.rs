//! Neo DevPack for Solidity Library
//!
//! A Solidity to NeoVM bytecode compiler for Neo N3 smart contracts.
//!
//! # Public API
//!
//! - [`cli`] — compiler entry point (`compile_contracts`, `disassemble_neovm_bytecode`)
//! - [`neo`] — NEF/manifest building and parsing utilities
//!
//! All other modules are `#[doc(hidden)]` — internal/test-facing, not part
//! of the documented public contract.
//!
//! Author: Jimmy <jimmy@r3e.network>

// Allow referring to this crate by name (`neo_devpack_solidity::...`) internally.
extern crate self as neo_devpack_solidity;

// === Public API (documented) ===

/// Compiler CLI: `compile_contracts`, `disassemble_neovm_bytecode`, standard-JSON.
pub mod cli;

/// NEF/manifest building, parsing, method tokens, contract hashes.
pub mod neo;

// === Internal / test-facing (hidden from docs) ===

#[doc(hidden)]
pub mod frontend;
#[doc(hidden)]
pub mod interop;
#[doc(hidden)]
pub mod ir;
#[doc(hidden)]
pub mod runtime;
#[doc(hidden)]
pub mod semantic_model;
#[doc(hidden)]
pub mod solidity;
#[doc(hidden)]
pub mod storage_key;
#[doc(hidden)]
pub mod type_system;
#[doc(hidden)]
pub mod utils;

/// Compiler version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
