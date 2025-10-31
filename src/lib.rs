//! Neo Solidity Compiler Library
//! Author: Jimmy <jimmy@r3e.network>

pub mod codegen;
pub mod compiler;
pub mod error;
pub mod frontend;
pub mod ir;
pub mod lexer;
pub mod neo;
pub mod optimizer;
pub mod parser;
pub mod semantic;
pub mod semantic_model;
pub mod solidity;
pub mod storage_key;
pub mod runtime;
pub mod type_system;
pub mod types;

pub use error::*;
pub use types::*;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
