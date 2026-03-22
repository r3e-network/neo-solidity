//! Yul AST Optimizer Module
//!
//! Performs optimization passes on the Yul AST to improve code quality and
//! reduce bytecode size.
//!
//! # Optimization Levels
//!
//! - **Level 0**: No optimization
//! - **Level 1**: Constant folding only
//! - **Level 2**: Constant folding + dead code elimination
//! - **Level 3**: All optimizations including function inlining
//!
//! # Optimization Passes
//!
//! - [`Optimizer::constant_folding`] - Evaluates constant expressions at compile time
//! - [`Optimizer::dead_code_elimination`] - Removes unreachable code after returns
//! - [`Optimizer::function_inlining`] - Inlines small functions to reduce call overhead

mod types;
mod dispatch;
mod constant_folding;
mod dead_code;
mod inlining;
mod strength;
mod cse;
mod cse_pass;

pub use types::*;
pub use strength::*;
pub use cse::{ExprHash, CseOptimizer};
