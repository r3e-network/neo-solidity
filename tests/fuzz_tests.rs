//! Comprehensive Fuzz Tests for Neo Solidity
//!
//! Split into submodules for maintainability. See `tests/fuzz_tests/` for
//! per-category sources.

#![allow(clippy::uninlined_format_args)]
#![allow(clippy::single_match)]
#![allow(clippy::partialeq_to_none)]

#[path = "fuzz_tests/common.rs"]
mod common;
#[path = "fuzz_tests/storage_props.rs"]
mod storage_props;
#[path = "fuzz_tests/compiler_props.rs"]
mod compiler_props;
#[path = "fuzz_tests/optimizer_props.rs"]
mod optimizer_props;
#[path = "fuzz_tests/arithmetic_props.rs"]
mod arithmetic_props;
#[path = "fuzz_tests/baseline_tests.rs"]
mod baseline_tests;
#[path = "fuzz_tests/batches_18_30.rs"]
mod batches_18_30;
#[path = "fuzz_tests/batches_31_45.rs"]
mod batches_31_45;
#[path = "fuzz_tests/batches_46_64.rs"]
mod batches_46_64;
#[path = "fuzz_tests/batches_66_80.rs"]
mod batches_66_80;
#[path = "fuzz_tests/batches_81_90.rs"]
mod batches_81_90;
#[path = "fuzz_tests/batches_91_100.rs"]
mod batches_91_100;
#[path = "fuzz_tests/task107_catch_panic_tests.rs"]
mod task107_catch_panic_tests;
