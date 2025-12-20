//! Storage Management Module
//!
//! Provides persistent storage for smart contracts with Neo blockchain compatibility.

use super::{RuntimeConfig, RuntimeError};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

include!("storage/storage_types.rs");
include!("storage/storage_impl.rs");

#[cfg(test)]
mod tests;
