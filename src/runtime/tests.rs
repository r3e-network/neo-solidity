use super::*;
use hex;
use sha2::{Digest as Sha2Digest, Sha256};
use sha3::Keccak256;

fn metadata_script() -> Vec<u8> {
    let syscalls = [
        "System.Runtime.GetTime",
        "System.Runtime.GetCallingScriptHash",
    ];
    let mut script = Vec::with_capacity(syscalls.len() * 5 + 1);
    for name in syscalls {
        script.push(0x41);
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        let digest = hasher.finalize();
        script.extend_from_slice(&[digest[0], digest[1], digest[2], digest[3]]);
    }
    script.push(0x40); // RET
    script
}

#[test]
fn test_runtime_creation() {
    let config = RuntimeConfig::default();
    let runtime = NeoRuntime::new(config);
    assert!(runtime.is_ok());
}

#[test]
fn test_contract_deployment() {
    let config = RuntimeConfig::default();
    let mut runtime = NeoRuntime::new(config).unwrap();

    let bytecode = vec![0x60, 0x01, 0x60, 0x02, 0x01]; // Simple ADD bytecode
    let result = runtime.deploy_contract(&bytecode, &[]);

    assert!(result.is_ok());
    let address = result.unwrap();
    assert!(address.starts_with("0x"));
    assert_eq!(address.len(), 42); // 0x + 40 hex chars
}

#[test]
fn test_contract_deployment_addresses_are_deterministic_and_unique_per_nonce() {
    let config = RuntimeConfig::default();
    let mut runtime = NeoRuntime::new(config).unwrap();

    let bytecode = vec![0x60, 0x01, 0x60, 0x02, 0x01];

    let first = runtime
        .deploy_contract(&bytecode, &[])
        .expect("first deploy");
    let second = runtime
        .deploy_contract(&bytecode, &[])
        .expect("second deploy");

    assert_ne!(
        first, second,
        "nonce should make subsequent addresses unique"
    );

    let mut input = Vec::with_capacity(20 + std::mem::size_of::<u64>());
    input.extend_from_slice(runtime.execution_context.default_account_bytes());
    input.extend_from_slice(&0u64.to_le_bytes());
    let hash = Keccak256::digest(&input);
    let expected_first = format!("0x{}", hex::encode(&hash[12..32]));
    assert_eq!(first, expected_first);
}

#[test]
fn test_function_selector_calculation() {
    let config = RuntimeConfig::default();
    let runtime = NeoRuntime::new(config).unwrap();

    let selector = runtime
        .calculate_function_selector("transfer(address,uint256)")
        .unwrap();
    // Known selector for transfer function
    assert_eq!(selector, [0xa9, 0x05, 0x9c, 0xbb]);
}

#[test]
fn test_execution_result() {
    let result = ExecutionResult {
        success: true,
        return_data: vec![0x01, 0x02, 0x03],
        gas_used: 1000,
        gas_limit: 10000,
        exception: None,
        state_changes: vec![],
        logs: vec![],
        stack_trace: None,
        metadata: ExecutionMetadata::default(),
    };

    assert!(result.is_success());
    assert_eq!(result.gas_efficiency(), 0.1);
    assert!(!result.out_of_gas());
    assert_eq!(result.return_hex(), "010203");
}

#[test]
fn test_runtime_statistics() {
    let stats = RuntimeStatistics {
        total_gas_used: 5000,
        total_instructions_executed: 100,
        max_stack_depth: 10,
        storage_reads: 5,
        storage_writes: 3,
        state_changes: 2,
    };

    assert_eq!(stats.total_gas_used, 5000);
    assert_eq!(stats.total_instructions_executed, 100);
}

#[test]
fn test_state_operations() {
    let config = RuntimeConfig::default();
    let mut runtime = NeoRuntime::new(config).unwrap();

    let account = "0x1234567890123456789012345678901234567890";
    let key = b"test_key";
    let value = b"test_value";

    // Set storage
    let result = runtime.set_storage(account, key, value);
    assert!(result.is_ok());

    // Get storage
    let retrieved = runtime.get_storage(account, key).unwrap();
    assert_eq!(retrieved, Some(value.to_vec()));
}

#[test]
fn test_balance_operations() {
    let config = RuntimeConfig::default();
    let mut runtime = NeoRuntime::new(config).unwrap();

    let account = "0x1234567890123456789012345678901234567890";
    let balance = 1000u64;

    // Set balance
    let result = runtime.set_balance(account, balance);
    assert!(result.is_ok());

    // Get balance
    let retrieved = runtime.get_balance(account).unwrap();
    assert_eq!(retrieved, balance);
}

#[test]
fn test_runtime_metadata_overrides_apply_once() {
    let mut runtime = NeoRuntime::new(RuntimeConfig::default()).unwrap();
    let overrides = ExecutionOverrides {
        block_height: Some(42),
        timestamp: Some(1_337),
        caller_account: Some("0x0102030405060708090a0102030405060708090a".to_string()),
    };

    let script = metadata_script();
    let result = runtime
        .execute_with_overrides(&script, &[], &overrides)
        .expect("execution");
    assert!(result.success);

    assert_eq!(result.metadata.block_height, Some(42));
    assert_eq!(result.metadata.timestamp, Some(1_337));
    assert_eq!(
        result.metadata.caller_account.as_deref(),
        Some("0x0102030405060708090a0102030405060708090a")
    );

    assert!(runtime.execution_context.pending_block_height().is_none());
    assert!(runtime.execution_context.pending_timestamp().is_none());
    assert!(runtime.execution_context.pending_caller_account().is_none());

    assert_eq!(runtime.execution_context.block_height(), Some(42));
    assert_eq!(runtime.execution_context.timestamp(), Some(1_337));

    let mut expected = hex::decode("0102030405060708090a0102030405060708090a").expect("valid hex");
    expected.reverse();
    assert_eq!(
        runtime.execution_context.caller_account(),
        Some(expected.as_slice())
    );

    // Next execution without overrides should fall back to defaults.
    let second = runtime
        .execute_with_overrides(&metadata_script(), &[], &ExecutionOverrides::default())
        .expect("second execution");

    assert_eq!(runtime.execution_context.block_height(), Some(0));
    assert_eq!(runtime.execution_context.timestamp(), Some(0));
    assert_eq!(
        runtime.execution_context.caller_account(),
        Some(runtime.execution_context.default_account_bytes())
    );

    assert_eq!(second.metadata.block_height, Some(0));
    assert_eq!(second.metadata.timestamp, Some(0));
    let expected_default = format!(
        "0x{}",
        hex::encode(runtime.execution_context.default_account_bytes())
    );
    assert_eq!(
        second.metadata.caller_account.as_deref(),
        Some(expected_default.as_str())
    );
}

#[test]
fn test_override_caller_account_rejects_invalid_hex() {
    let mut runtime = NeoRuntime::new(RuntimeConfig::default()).unwrap();
    let err = runtime
        .override_caller_account("0x123")
        .expect_err("should reject odd-length hex");
    assert!(matches!(err, RuntimeError::ConfigurationError { .. }));
    assert!(runtime.execution_context.pending_caller_account().is_none());
}

#[test]
fn test_execute_with_overrides_rejects_invalid_metadata() {
    let mut runtime = NeoRuntime::new(RuntimeConfig::default()).unwrap();
    let overrides = ExecutionOverrides {
        block_height: Some(77),
        timestamp: Some(555),
        caller_account: Some("0x123".to_string()),
    };

    let err = runtime
        .execute_with_overrides(&[], &[], &overrides)
        .expect_err("invalid caller should error");
    assert!(matches!(err, RuntimeError::ConfigurationError { .. }));

    assert!(runtime.execution_context.pending_block_height().is_none());
    assert!(runtime.execution_context.pending_timestamp().is_none());
    assert!(runtime.execution_context.pending_caller_account().is_none());
}
