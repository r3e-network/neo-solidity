use super::*;
use neo_solidity::ir;
use neo_solidity::runtime::{NeoRuntime, RuntimeConfig};
use neo_solidity::solidity::analyse_source;
use serde_json::Value;

fn execute_bytecode(bytecode: &[u8]) -> neo_solidity::runtime::ExecutionResult {
    let mut runtime = NeoRuntime::new(RuntimeConfig::default()).expect("runtime");
    runtime.execute(bytecode, &[]).expect("execution")
}

fn execute_bytecode_with_tokens(
    bytecode: &[u8],
    tokens: &[neo_solidity::neo::MethodToken],
) -> neo_solidity::runtime::ExecutionResult {
    let mut runtime = NeoRuntime::new(RuntimeConfig::default()).expect("runtime");
    runtime
        .execute_with_tokens(bytecode, &[], tokens)
        .expect("execution")
}

include!("semantics/state_mutability.rs");
include!("semantics/control_flow.rs");
include!("semantics/return_values.rs");
include!("semantics/native_calls.rs");
include!("semantics/storage_layout.rs");
