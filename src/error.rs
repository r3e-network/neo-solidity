//! Compiler Error Types
//!
//! Defines error types used throughout the Neo Solidity compiler pipeline.
//! All errors implement the standard `Error` trait for easy integration.
//!
//! # Error Categories
//!
//! - [`CompilerError::IoError`] - File system and I/O errors
//! - [`CompilerError::ParseError`] - Syntax and parsing errors
//! - [`CompilerError::SemanticError`] - Type checking and semantic errors
//! - [`CompilerError::CodegenError`] - Code generation errors

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("IO error: {0}")]
    IoError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Semantic error: {0}")]
    SemanticError(String),

    #[error("Codegen error: {0}")]
    CodegenError(String),
}
