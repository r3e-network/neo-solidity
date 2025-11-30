use neo_solidity::runtime::execution::ExecutionContext;
use neo_solidity::runtime::RuntimeConfig;

#[test]
fn try_catches_throw_and_jumps_to_handler() {
    // TRY (catch at 10, no finally) -> body THROW -> catch: PUSH1 RET
    let code = [
        0x3B, 0x0A, 0x00, 0x00, 0x00, // catch offset = 10
        0x00, 0x00, 0x00, 0x00, // finally offset = 0
        0x3A, // THROW
        0x11, // catch block starts here: PUSH1
        0x40, // RET
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("ctx init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}

    let expected = (1i64).to_le_bytes().to_vec();
    assert_eq!(ctx.return_data(), expected);
}

#[test]
fn endtry_jumps_to_finally_on_success() {
    // TRY with finally at 12, body PUSH2 ENDTRY, finally PUSH3 RET -> result should be 3
    let code = [
        0x3B, 0x0C, 0x00, 0x00, 0x00, // catch offset (unused) = 12
        0x0C, 0x00, 0x00, 0x00, // finally offset = 12
        0x12, // body: PUSH2
        0x3D, // ENDTRY -> should jump to finally
        0x40, // (would RET if no finally)
        0x13, // finally: PUSH3
        0x40, // RET
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("ctx init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}

    let expected = (3i64).to_le_bytes().to_vec();
    assert_eq!(ctx.return_data(), expected);
}

#[test]
fn finally_rethrows_when_no_catch() {
    // TRY with no catch (0), finally at 10, body THROW -> finally runs then rethrows
    let code = [
        0x3B, 0x00, 0x00, 0x00, 0x00, // catch offset = 0 (none)
        0x0B, 0x00, 0x00, 0x00, // finally offset = 11 (NOP)
        0x3A, // body: THROW
        0x40, // RET (won't reach)
        0x21, // finally at 10: NOP
        0x3F, // ENDFINALLY -> should rethrow
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("ctx init");
    ctx.initialize(&code, &[]).expect("init");

    let err = loop {
        match ctx.step() {
            Ok(step) if step.halted => break None,
            Ok(_) => continue,
            Err(e) => break Some(e),
        }
    };
    assert!(err.is_some(), "should rethrow after finally when no catch");
}

#[test]
fn nested_try_finally_rethrows_outer() {
    // Outer TRY (no catch) finally at 21, body has inner TRY with catch at 19 finally at 0 -> inner catch clears, outer must rethrow
    let code = [
        0x3B, 0x00, 0x00, 0x00, 0x00, // outer catch = 0
        0x15, 0x00, 0x00, 0x00, // outer finally = 21 (NOP)
        // inner try
        0x3B, 0x13, 0x00, 0x00, 0x00, // inner catch at 19
        0x00, 0x00, 0x00, 0x00, // inner finally = 0
        0x3A, // THROW (inner)
        // inner catch
        0x11, // PUSH1
        0x3D, // ENDTRY (no finally)
        // outer finally
        0x21, // NOP at 18
        0x3F, // ENDFINALLY -> should rethrow outer error
    ];

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("ctx init");
    ctx.initialize(&code, &[]).expect("init");

    let err = loop {
        match ctx.step() {
            Ok(step) if step.halted => break None,
            Ok(_) => continue,
            Err(e) => break Some(e),
        }
    };
    assert!(err.is_some(), "outer should rethrow even if inner caught");
}
