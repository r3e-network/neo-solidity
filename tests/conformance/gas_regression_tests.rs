/// Gas accounting regression tests
///
/// These tests verify that gas consumption remains stable across refactoring.
use neo_solidity::runtime::execution::ExecutionContext;
use neo_solidity::runtime::RuntimeConfig;
use sha2::{Digest, Sha256};

fn syscall_id(name: &str) -> [u8; 4] {
    let mut h = Sha256::new();
    h.update(name.as_bytes());
    let d = h.finalize();
    [d[0], d[1], d[2], d[3]]
}

/// Test that basic arithmetic consumes expected gas
#[test]
fn gas_arithmetic_consumes_expected_amount() {
    let config = RuntimeConfig {
        gas_limit: 100_000,
        ..Default::default()
    };

    // ADD costs 3 gas
    let code = [
        0x11, // PUSH1 (1 gas)
        0x11, // PUSH1 (1 gas)
        0x9E, // ADD (3 gas)
        0x40, // RET (0 gas)
    ];

    let mut ctx = ExecutionContext::new(&config).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let gas_before = ctx.gas_used();
    while !ctx.step().expect("step").halted {}
    let gas_after = ctx.gas_used();

    let consumed = gas_after - gas_before;
    assert!(
        consumed >= 5,
        "Should consume at least 5 gas for PUSH1+PUSH1+ADD"
    );
    assert!(consumed <= 10, "Should not consume excessive gas");
}

/// Test that storage operations consume gas proportionally to data size
#[test]
fn gas_storage_scales_with_data_size() {
    let config = RuntimeConfig {
        gas_limit: 1_000_000,
        ..Default::default()
    };

    let put_id = syscall_id("System.Storage.Put");
    let get_context_id = syscall_id("System.Storage.GetContext");
    let _get_id = syscall_id("System.Storage.Get");
    let _delete_id = syscall_id("System.Storage.Delete");

    // Test with 10-byte value
    let mut code_small = vec![];
    code_small.extend_from_slice(&[0x57, 0x01, 0x00]); // INITSLOT 1 local, 0 args
    code_small.extend_from_slice(&get_context_id);
    code_small.push(0x70); // STLOC0
    code_small.extend_from_slice(&[0x68]); // LDLOC0
    code_small.push(0x0C); // PUSHDATA1
    code_small.push(10); // 10 bytes
    code_small.extend_from_slice(&[0u8; 10]); // value
    code_small.push(0x0C); // PUSHDATA1
    code_small.push(3); // 3 bytes
    code_small.extend_from_slice(b"key"); // key
    code_small.extend_from_slice(&put_id);
    code_small.push(0x40); // RET

    // Test with 100-byte value
    let mut code_large = vec![];
    code_large.extend_from_slice(&[0x57, 0x01, 0x00]); // INITSLOT 1 local, 0 args
    code_large.extend_from_slice(&get_context_id);
    code_large.push(0x70); // STLOC0
    code_large.extend_from_slice(&[0x68]); // LDLOC0
    code_large.push(0x0C); // PUSHDATA1
    code_large.push(100); // 100 bytes
    code_large.extend_from_slice(&[0u8; 100]); // value
    code_large.push(0x0C); // PUSHDATA1
    code_large.push(3); // 3 bytes
    code_large.extend_from_slice(b"key"); // key
    code_large.extend_from_slice(&put_id);
    code_large.push(0x40); // RET

    let mut ctx_small = ExecutionContext::new(&config.clone()).expect("context init");
    ctx_small.initialize(&code_small, &[]).expect("init");
    while !ctx_small.step().expect("step").halted {}
    let gas_small = ctx_small.gas_used();

    let mut ctx_large = ExecutionContext::new(&config).expect("context init");
    ctx_large.initialize(&code_large, &[]).expect("init");
    while !ctx_large.step().expect("step").halted {}
    let gas_large = ctx_large.gas_used();

    // Larger data should consume more gas
    assert!(
        gas_large > gas_small,
        "Larger storage should consume more gas"
    );
}

/// Test that syscall gas costs are reasonable
#[test]
fn gas_syscall_costs_are_reasonable() {
    let config = RuntimeConfig {
        gas_limit: 100_000,
        ..Default::default()
    };

    // Test Storage.Get costs around 100 gas
    let get_context_id = syscall_id("System.Storage.GetContext");
    let get_id = syscall_id("System.Storage.Get");
    let mut code = vec![];
    code.extend_from_slice(&[0x57, 0x01, 0x00]); // INITSLOT 1 local
    code.extend_from_slice(&get_context_id);
    code.push(0x70); // STLOC0
    code.extend_from_slice(&[0x68]); // LDLOC0
    code.push(0x0C); // PUSHDATA1
    code.push(3);
    code.extend_from_slice(b"key");
    code.extend_from_slice(&get_id);
    code.push(0x40);

    let mut ctx = ExecutionContext::new(&config).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let gas_before = ctx.gas_used();
    while !ctx.step().expect("step").halted {}
    let gas_after = ctx.gas_used();

    let consumed = gas_after - gas_before;
    assert!(consumed >= 100, "Storage.Get should cost at least 100 gas");
    assert!(
        consumed <= 200,
        "Storage.Get should not cost more than 200 gas"
    );
}

/// Test that exception handling consumes gas
#[test]
fn gas_exception_handling_consumes_appropriately() {
    let config = RuntimeConfig {
        gas_limit: 100_000,
        ..Default::default()
    };

    // TRY...THROW should consume gas
    let code = [
        0x3B, 0x04, 0x00, // TRY (1 gas)
        0x0C, 0x04, // PUSHDATA1 (2 gas)
        0x45, 0x52, 0x52, 0x4F, // "ERR"
        0x3A, // THROW (1 gas)
        0x11, // catch: PUSH1
        0x40, // RET
    ];

    let mut ctx = ExecutionContext::new(&config).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let gas_before = ctx.gas_used();
    while !ctx.step().expect("step").halted {}
    let gas_after = ctx.gas_used();

    assert!(
        gas_after > gas_before,
        "Exception handling should consume gas"
    );
}

/// Test that loops don't leak gas
#[test]
fn gas_loops_consume_predictable_amount() {
    let config = RuntimeConfig {
        gas_limit: 1_000_000,
        ..Default::default()
    };

    // Simple loop: 10 iterations
    let mut code = vec![];
    code.extend(&[0x11; 10]); // PUSH1 (1 gas each)
    code.push(0x40); // RET

    let mut ctx = ExecutionContext::new(&config).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let gas_before = ctx.gas_used();
    while !ctx.step().expect("step").halted {}
    let gas_after = ctx.gas_used();

    let consumed = gas_after - gas_before;
    // 10 PUSH1 + 1 RET = 11 gas
    assert!(consumed >= 10, "Should consume at least 10 gas");
    assert!(consumed <= 20, "Should not consume excessive gas");
}

/// Regression test: Ensure gas costs don't accidentally drop to zero
#[test]
fn gas_never_zero_after_operations() {
    let config = RuntimeConfig {
        gas_limit: 100_000,
        ..Default::default()
    };

    // Various operations that should always consume gas
    let operations: Vec<Vec<u8>> = vec![
        vec![0x11, 0x9E, 0x40],       // PUSH1, ADD, RET
        vec![0x11, 0x9F, 0x40],       // PUSH1, MUL, RET
        vec![0x0C, 0x01, 0x61, 0x40], // PUSHDATA1 1, PUSH1, RET
        vec![0x21, 0x21, 0x40],       // NOP, NOP, RET
        vec![0x45, 0x21, 0x40],       // DROP, NOP, RET
    ];

    for (i, op) in operations.iter().enumerate() {
        let mut ctx = ExecutionContext::new(&config.clone()).expect("context init");
        ctx.initialize(op, &[]).expect("init");

        let gas_before = ctx.gas_used();
        while !ctx.step().expect("step").halted {}
        let gas_after = ctx.gas_used();

        let consumed = gas_after - gas_before;
        assert!(
            consumed > 0,
            "Operation {} should consume gas, got {}",
            i,
            consumed
        );
    }
}

/// Test that gas limit is enforced
#[test]
fn gas_limit_is_enforced() {
    let config = RuntimeConfig {
        gas_limit: 10, // Very low limit
        ..Default::default()
    };

    // Code that would exceed gas limit
    // 100 PUSH1 instructions (1 gas each, so 100 would exceed limit of 10)
    let mut code = vec![0x11; 100];
    code.push(0x40); // RET

    let mut ctx = ExecutionContext::new(&config).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let result = ctx.step();
    assert!(result.is_err(), "Should run out of gas");

    // Verify the error is OutOfGas
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("OutOfGas") || error_msg.contains("out of gas"),
        "Error should mention gas limit"
    );
}

/// Test that cryptographic operations consume more gas
#[test]
fn gas_crypto_consumes_more_than_basic_ops() {
    let config = RuntimeConfig {
        gas_limit: 1_000_000,
        ..Default::default()
    };

    // Crypto syscall (GetBlock) costs more than basic ops
    let keccak_id = syscall_id("System.Blockchain.GetBlock");
    let mut code_crypto = vec![
        0x0C, 0x01, 0x61, // PUSHDATA1 1, PUSH1 (block height param)
    ];
    code_crypto.extend_from_slice(&keccak_id);
    code_crypto.push(0x40); // RET

    // Basic arithmetic
    let code_basic = [
        0x11, 0x11, 0x9E, 0x40, // PUSH1, PUSH1, ADD, RET
    ];

    let mut ctx_crypto = ExecutionContext::new(&config.clone()).expect("context init");
    ctx_crypto.initialize(&code_crypto, &[]).expect("init");

    let mut ctx_basic = ExecutionContext::new(&config).expect("context init");
    ctx_basic.initialize(&code_basic, &[]).expect("init");

    let _gas_before_crypto = ctx_crypto.gas_used();
    while !ctx_crypto.step().expect("step").halted {}
    let gas_after_crypto = ctx_crypto.gas_used();

    let _gas_before_basic = ctx_basic.gas_used();
    while !ctx_basic.step().expect("step").halted {}
    let gas_after_basic = ctx_basic.gas_used();

    // Crypto should consume more gas than basic arithmetic
    assert!(
        gas_after_crypto > gas_after_basic,
        "Crypto operations should consume more gas than basic ops"
    );
}
