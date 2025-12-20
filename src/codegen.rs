//! Yul Code Generation Module
//!
//! Generates NeoVM bytecode from Yul AST. This module handles the translation
//! of high-level Yul constructs to low-level NeoVM instructions.
//!
//! # Output
//!
//! - [`CompilationResult`] - Complete compilation output including bytecode,
//!   ABI, manifest, and debug information
//!
//! # Code Generation Pipeline
//!
//! 1. AST traversal and instruction emission
//! 2. Label resolution and jump patching
//! 3. Optimization passes
//! 4. Final bytecode assembly

use crate::error::CompilerError;
use crate::parser::{AstNode, AstNodeType};
use crate::types::CompilerConfig;
use sha2::{Digest, Sha256};
use std::collections::HashSet;

include!("codegen/codegen_types.rs");
include!("codegen/codegen_impl.rs");
include!("codegen/interop.rs");
