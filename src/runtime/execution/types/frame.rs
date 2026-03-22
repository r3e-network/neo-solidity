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
    /// Saved caller locals — restored when returning from this frame.
    pub saved_locals: Vec<StackItem>,
    /// Saved caller args — restored when returning from this frame.
    pub saved_args: Vec<StackItem>,
}

impl CallFrame {
    /// Create a new call frame
    pub fn new(return_address: u32, stack_base: usize) -> Self {
        Self {
            return_address,
            function_name: None,
            local_variables: HashMap::new(),
            stack_base,
            saved_locals: Vec::new(),
            saved_args: Vec::new(),
        }
    }

    /// Check if this frame has local variables
    pub fn has_locals(&self) -> bool {
        !self.local_variables.is_empty()
    }
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

impl TryFrameState {
    /// Check if in error handling state
    pub fn is_handling_error(&self) -> bool {
        matches!(self, Self::Catch | Self::Finally)
    }

    /// Get state as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Try => "try",
            Self::Catch => "catch",
            Self::Finally => "finally",
        }
    }
}
