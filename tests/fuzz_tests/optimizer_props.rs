//! Optimizer-level property tests.
//!
//! Reserved placeholder module for optimizer-focused proptest blocks. The
//! pre-split `tests/fuzz_tests.rs` did not carry any optimizer-only top-level
//! proptest block (optimizer coverage lives inside Batch #13 / #14 which remain
//! grouped under `baseline_tests.rs`). This file exists so future optimizer
//! harnesses have a natural home without reshuffling the split layout.

#![allow(unused_imports)]

use super::common::*;
use neo_solidity::cli::compile_contracts;
use neo_solidity::runtime::{NeoRuntime, RuntimeConfig};
use proptest::prelude::*;
