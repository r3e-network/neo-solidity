//! Call and exception handling frames for execution context.
//!
//! Provides structures for managing function call frames and exception handling.

use super::stack::StackItem;
use std::collections::HashMap;

/// Call frame for function calls
#[derive(Debug, Clone)]
pub struct CallFrame {
    pub return_address: u32,
    pub function_name: Option<String>,
    pub local_variables: HashMap<String, StackItem>,
    pub stack_base: usize,
}

/// Exception handling frame for try-catch-finally blocks
#[derive(Debug, Clone)]
pub struct TryFrame {
    pub(crate) catch_target: Option<u32>,
    pub(crate) finally_target: Option<u32>,
    pub(crate) end_target: Option<u32>,
    pub(crate) state: TryFrameState,
}

/// State of a try-catch-finally frame
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFrameState {
    Try,
    Catch,
    Finally,
}
