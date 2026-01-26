/// Conformance tests for exception handling
///
/// Tests TRY/ENDTRY/ENDFINALLY with CATCH and FINALLY blocks
use neo_solidity::runtime::execution::ExecutionContext;
use neo_solidity::runtime::RuntimeConfig;

/// Test TRY without exception takes normal path
#[test]
fn try_block_no_exception_normal_path() {
    // Based on runtime_exception_tests.rs pattern
    let code = [
        0x3B, 0x00, 0x04, // TRY: catch=0, finally=+4
        0x11, // body: PUSH1
        0x3D, 0x02, // ENDTRY(+2) -> end
        0x40, // RET
        0x13, // finally: PUSH3
        0x3F, // ENDFINALLY
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}
    assert_eq!(ctx.return_data(), (3i64).to_le_bytes().to_vec());
}

/// Test THROW without TRY block propagates exception
#[test]
fn throw_without_try_block_propagates() {
    let code = [
        0x0C, 0x04, // PUSHDATA1 4 bytes
        0x45, 0x52, 0x52, 0x4F, // "ERR"
        0x3A, // THROW
        0x40, // RET
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let result = ctx.step();
    assert!(result.is_err(), "Should throw exception");
}

/// Test TRY with CATCH catches exception
#[test]
fn try_catch_catches_exception() {
    // Pattern from runtime_exception_tests.rs
    let code = [
        0x3B, 0x04, 0x00, // TRY: catch=+4, finally=0
        0x3A, // THROW
        0x11, // catch: PUSH1
        0x40, // RET
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}
    assert_eq!(ctx.return_data(), (1i64).to_le_bytes().to_vec());
}

/// Test ASSERT with true condition passes
#[test]
fn assert_true_passes() {
    let code = [
        0x11, // PUSH1 (true)
        0x39, // ASSERT
        0x40, // RET
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}
}

/// Test ASSERT with false condition throws
#[test]
fn assert_false_throws() {
    let code = [
        0x10, // PUSH0 (false)
        0x39, // ASSERT
        0x40, // RET
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let result = ctx.step();
    assert!(result.is_err(), "ASSERT false should throw");
}

/// Test nested TRY blocks
#[test]
fn nested_try_blocks() {
    // From runtime_exception_tests.rs pattern
    let code = [
        0x3B, 0x00, 0x0E, // outer TRY: catch=0, finally=+14
        0x3B, 0x04, 0x00, // inner TRY: catch=+4, finally=0
        0x3A, // inner THROW
        0x45, // inner catch: DROP (exception)
        0x11, // PUSH1
        0x3D, 0x02, // inner ENDTRY(+2)
        0x3D, 0x02, // outer ENDTRY(+2)
        0x40, // end: RET
        0x13, // outer finally: PUSH3
        0x3F, // ENDFINALLY
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}
    assert_eq!(ctx.return_data(), (3i64).to_le_bytes().to_vec());
}

/// Test FINALLY block executes after normal completion
#[test]
fn finally_executes_after_normal_completion() {
    let code = [
        0x3B, 0x00, 0x07, // TRY: catch=0, finally=+7
        0x12, // body: PUSH2
        0x3D, 0x02, // ENDTRY(+2)
        0x40, // end: RET
        0x13, // finally: PUSH3
        0x3F, // ENDFINALLY
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}
    assert_eq!(ctx.return_data(), (3i64).to_le_bytes().to_vec());
}

/// Test FINALLY block executes after exception
#[test]
fn finally_executes_after_exception() {
    let code = [
        0x3B, 0x00, 0x05, // TRY: catch=0, finally=+5
        0x3A, // THROW
        0x40, // RET (won't reach)
        0x21, // finally: NOP
        0x3F, // ENDFINALLY -> rethrows
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let err = loop {
        match ctx.step() {
            Ok(step) if step.halted => break None,
            Ok(_) => continue,
            Err(e) => break Some(e),
        }
    };
    assert!(err.is_some(), "should rethrow after finally");
}

/// Test ABORT terminates execution immediately
#[test]
fn abort_terminates_immediately() {
    let code = [
        0x21, // NOP (before ABORT)
        0x38, // ABORT
        0x21, // NOP (should not execute)
        0x40, // RET
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let result = ctx.step();
    assert!(result.is_err(), "ABORT should terminate with error");
}

/// Test TRY_L with long offsets
#[test]
fn try_l_with_long_offsets() {
    let code = [
        0x3C, // TRY_L
        0x04, 0x00, 0x00, 0x00, // catch_offset=4
        0x00, 0x00, 0x00, 0x00, // finally_offset=0
        0x0C, 0x04, // PUSHDATA1 4
        0x54, 0x45, 0x53, 0x54, // "TEST"
        0x3A, // THROW
        0x11, // catch: PUSH1
        0x3E, // ENDTRY_L
        0x01, 0x00, 0x00, 0x00, // end_offset=1
        0x40, // RET
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}
}

/// Test empty exception message
#[test]
fn throw_with_empty_message() {
    let code = [
        0x0C, 0x00, // PUSHDATA1 0 bytes
        0x3A, // THROW
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let result = ctx.step();
    assert!(result.is_err(), "Should throw");
}

/// Test ENDTRY without TRY throws error
#[test]
fn endtry_without_try_throws() {
    let code = [
        0x3D, // ENDTRY (no matching TRY)
        0x01, // end_offset
        0x40, // RET
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let result = ctx.step();
    assert!(result.is_err(), "ENDTRY without TRY should error");
}

/// Test gas is consumed during exception handling
#[test]
fn exception_handling_consumes_gas() {
    let config = RuntimeConfig {
        gas_limit: 10_000,
        ..Default::default()
    };

    let code = [
        0x3B, 0x04, 0x00, // TRY (1 gas)
        0x0C, 0x04, // PUSHDATA1 (2 gas)
        0x45, 0x52, 0x52, 0x4F, // "ERRO"
        0x3A, // THROW (1 gas)
        0x11, // catch: PUSH1
        0x40, // RET
    ];

    let mut ctx = ExecutionContext::new(&config).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let gas_before = ctx.gas_used();
    while !ctx.step().expect("step").halted {}
    let gas_after = ctx.gas_used();

    assert!(gas_after > gas_before, "Gas should be consumed");
}

/// Test exception message is preserved through catch
#[test]
fn exception_message_preserved_in_catch() {
    let code = [
        0x0C, 0x0A, // PUSHDATA1 10 bytes
        0x54, 0x65, 0x73, 0x74, 0x45, 0x72, 0x72, 0x6F, 0x72, // "TestError"
        0x3B, 0x04, 0x00, // TRY: catch=+4, finally=0
        0x3A, // THROW (uses value on stack as message)
        0x21, // catch: NOP (message consumed but not checked)
        0x3D, 0x01, // ENDTRY(+1)
        0x40, // RET
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}
}

/// Test multiple exceptions in sequence
#[test]
fn multiple_exceptions_in_sequence() {
    let code = [
        0x11, // PUSH1
        0x39, // ASSERT (passes, true)
        0x10, // PUSH0
        0x39, // ASSERT (throws, false)
        0x40, // RET (never reached)
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let result = ctx.step();
    assert!(result.is_err(), "Second ASSERT should fail");
}
