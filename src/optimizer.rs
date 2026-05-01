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
//! - `Optimizer::constant_folding` - Evaluates constant expressions at compile time
//! - `Optimizer::dead_code_elimination` - Removes unreachable code after returns
//! - `Optimizer::function_inlining` - Inlines small functions to reduce call overhead

mod constant_folding;
mod cse;
mod cse_pass;
mod dead_code;
mod dispatch;
mod inlining;
mod strength;
mod types;

pub use cse::{CseOptimizer, ExprHash};
pub use strength::*;
pub use types::*;
