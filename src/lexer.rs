//! Yul Lexer Module
//!
//! Lexical analyzer for the Yul intermediate language. Converts Yul source code
//! into a stream of tokens for parsing.
//!
//! # Supported Tokens
//!
//! - Keywords: `let`, `if`, `else`, `for`, `switch`, `case`, `default`, `function`, etc.
//! - Operators: `:=`, `->`, `+`, `-`
//! - Literals: decimal numbers, hex numbers (`0x...`), strings
//! - Identifiers: variable and function names
//! - Built-in functions: `add`, `sub`, `mul`, `div`, `mload`, `sstore`, etc.
//!
//! # Example
//!
//! ```text
//! use neo_devpack_solidity::lexer::Lexer;
//!
//! let mut lexer = Lexer::new("let x := add(1, 2)");
//! let tokens = lexer.tokenize()?;
//! ```

use crate::error::CompilerError;

include!("lexer/lexer_types.rs");
include!("lexer/lexer_impl.rs");

#[cfg(test)]
mod tests;
