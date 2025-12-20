//! Neo Runtime Module
//!
//! Complete runtime integration layer providing EVM compatibility on NeoVM,
//! state management, storage operations, and execution environment.

pub mod bridge;
pub mod execution;
pub mod spec;
pub mod state;
pub mod storage;
pub mod types;

use hex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

include!("runtime_parts/runtime_types.rs");
include!("runtime_parts/runtime_impl.rs");

#[cfg(test)]
mod tests;
