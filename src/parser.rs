//! Yul Parser Module
//!
//! Parses Yul tokens into an Abstract Syntax Tree (AST). The parser handles
//! Yul language constructs including functions, control flow, and expressions.
//!
//! # AST Structure
//!
//! - [`AstNode`] - Base AST node with type and location
//! - [`AstNodeType`] - Enum of all possible node types
//!
//! # Supported Constructs
//!
//! - Object definitions and code blocks
//! - Function definitions with parameters and return values
//! - Variable declarations (`let x := ...`)
//! - Control flow: `if`, `for`, `switch/case`
//! - Function calls and built-in operations

use crate::error::CompilerError;
use crate::lexer::{Token, TokenType};

include!("parser/parser_types.rs");
include!("parser/parser_impl.rs");
