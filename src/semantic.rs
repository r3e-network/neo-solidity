//! Yul Semantic Analysis Module
//!
//! Performs semantic analysis on Yul AST, including type checking, scope
//! validation, and code quality metrics.
//!
//! # Analysis Results
//!
//! - [`SemanticResult`] - Complete analysis output
//! - [`ComplexityMetrics`] - Code complexity measurements
//! - [`SecurityIssue`] - Potential security concerns
//! - [`PerformanceMetrics`] - Gas and performance estimates
//!
//! # Checks Performed
//!
//! - Variable scope and shadowing detection
//! - Function signature validation
//! - Unreachable code detection
//! - Security pattern analysis

use crate::error::CompilerError;
use crate::parser::{AstNode, AstNodeType};

include!("semantic/types.rs");
include!("semantic/analyzer.rs");
include!("semantic/visitor.rs");
include!("semantic/builtins.rs");
