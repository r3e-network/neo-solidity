//! State Management Module
//!
//! Manages account states, balances, and transaction state for Neo runtime.

use super::{RuntimeConfig, RuntimeError, StateChange, StateChangeType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

include!("state/state_types.rs");
include!("state/state_impl.rs");

#[cfg(test)]
mod tests;
