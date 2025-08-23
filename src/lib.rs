//! Neo Solidity Compiler Library
//! Author: Jimmy <jimmy@r3e.network>

pub mod codegen;
pub mod compiler;
pub mod error;
pub mod lexer;
pub mod optimizer;
pub mod parser;
pub mod semantic;
pub mod types;

pub use error::*;
pub use types::*;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
