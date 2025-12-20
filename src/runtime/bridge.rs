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

include!("bridge/bridge_types.rs");
include!("bridge/bridge_impl_core.rs");
include!("bridge/bridge_impl_arithmetic.rs");
include!("bridge/bridge_impl_logic.rs");
include!("bridge/bridge_impl_stack.rs");
include!("bridge/bridge_impl_syscalls.rs");
include!("bridge/bridge_impl_stack_items.rs");
include!("bridge/bridge_helpers.rs");
