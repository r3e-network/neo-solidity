//! VM Bridge Module
//!
//! Provides bridge between EVM semantics and NeoVM execution environment.

use super::types::StackItem;
use super::{
    execution, state, storage, ExceptionType, ExecutionMetadata, ExecutionResult, RuntimeConfig,
    RuntimeError, RuntimeException, StackFrame, StateChange,
};
use std::collections::HashMap;
use thiserror::Error;

mod bridge_helpers;
mod bridge_impl_arithmetic;
mod bridge_impl_core;
mod bridge_impl_stack_items;
mod bridge_impl_syscalls;
mod bridge_types;
mod logic;
mod stack;

pub(crate) use bridge_helpers::*;
pub use bridge_types::*;
