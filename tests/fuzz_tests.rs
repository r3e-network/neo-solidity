//! Comprehensive Fuzz Tests for Neo Solidity
//!
//! Split into submodules for maintainability. See `tests/fuzz_tests/` for
//! per-category sources.

#![allow(clippy::uninlined_format_args)]
#![allow(clippy::single_match)]
#![allow(clippy::partialeq_to_none)]
#![allow(clippy::err_expect)]
#![allow(clippy::implicit_clone)]
#![allow(clippy::len_zero)]
#![allow(clippy::manual_contains)]
#![allow(clippy::manual_div_ceil)]
#![allow(clippy::manual_repeat_n)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_return)]
#![allow(clippy::op_ref)]
#![allow(clippy::type_complexity)]
#![allow(clippy::useless_format)]
#![allow(clippy::useless_vec)]

#[path = "fuzz_tests/abi_roundtrip_props.rs"]
mod abi_roundtrip_props;
#[path = "fuzz_tests/arithmetic_helpers_props.rs"]
mod arithmetic_helpers_props;
#[path = "fuzz_tests/arithmetic_props.rs"]
mod arithmetic_props;
#[path = "fuzz_tests/baseline_tests.rs"]
mod baseline_tests;
#[path = "fuzz_tests/batches_100_105.rs"]
mod batches_100_105;
#[path = "fuzz_tests/batches_106_110.rs"]
mod batches_106_110;
#[path = "fuzz_tests/batches_111_115.rs"]
mod batches_111_115;
#[path = "fuzz_tests/batches_116_120.rs"]
mod batches_116_120;
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
#[path = "fuzz_tests/common.rs"]
mod common;
#[path = "fuzz_tests/compile_runtime_roundtrip.rs"]
mod compile_runtime_roundtrip;
#[path = "fuzz_tests/compiler_props.rs"]
mod compiler_props;
#[path = "fuzz_tests/conditional_jumps.rs"]
mod conditional_jumps;
#[path = "fuzz_tests/constant_immutable_gap_props.rs"]
mod constant_immutable_gap_props;
#[path = "fuzz_tests/constructor_lifecycle_props.rs"]
mod constructor_lifecycle_props;
#[path = "fuzz_tests/contract_upgrade_props.rs"]
mod contract_upgrade_props;
#[path = "fuzz_tests/convergence_props.rs"]
mod convergence_props;
#[path = "fuzz_tests/custom_error_envelope_props.rs"]
mod custom_error_envelope_props;
#[path = "fuzz_tests/determinism_props.rs"]
mod determinism_props;
#[path = "fuzz_tests/devpack_props.rs"]
mod devpack_props;
#[path = "fuzz_tests/diagnostic_stability_props.rs"]
mod diagnostic_stability_props;
#[path = "fuzz_tests/differential.rs"]
mod differential;
#[path = "fuzz_tests/disasm_stability_props.rs"]
mod disasm_stability_props;
#[path = "fuzz_tests/erc1155_proxy_props.rs"]
mod erc1155_proxy_props;
#[path = "fuzz_tests/examples_smoke_props.rs"]
mod examples_smoke_props;
#[path = "fuzz_tests/fallback_receive_props.rs"]
mod fallback_receive_props;
#[path = "fuzz_tests/in_contract_array_return_props.rs"]
mod in_contract_array_return_props;
#[path = "fuzz_tests/library_deployment_props.rs"]
mod library_deployment_props;
#[path = "fuzz_tests/modifier_rewrite_props.rs"]
mod modifier_rewrite_props;
#[path = "fuzz_tests/multi_source_compile_props.rs"]
mod multi_source_compile_props;
#[path = "fuzz_tests/native_contract_props.rs"]
mod native_contract_props;
#[path = "fuzz_tests/native_resolver_props.rs"]
mod native_resolver_props;
#[path = "fuzz_tests/openzeppelin_patterns_props.rs"]
mod openzeppelin_patterns_props;
#[path = "fuzz_tests/optimizer_props.rs"]
mod optimizer_props;
#[path = "fuzz_tests/pathological_corpus_smoke.rs"]
mod pathological_corpus_smoke;
#[path = "fuzz_tests/performance_regression.rs"]
mod performance_regression;
#[path = "fuzz_tests/reentrancy_props.rs"]
mod reentrancy_props;
#[path = "fuzz_tests/stdlib_native_props.rs"]
mod stdlib_native_props;
#[path = "fuzz_tests/storage_iterator_stress.rs"]
mod storage_iterator_stress;
#[path = "fuzz_tests/storage_props.rs"]
mod storage_props;
#[path = "fuzz_tests/storage_state_machine.rs"]
mod storage_state_machine;
#[path = "fuzz_tests/task107_catch_panic_tests.rs"]
mod task107_catch_panic_tests;
