use neo_solidity::runtime::execution::ExecutionContext;
use neo_solidity::runtime::RuntimeConfig;

#[test]
fn logical_not_toggles_truthiness() {
    // PUSH1 -> NOT -> RET (return should be false/0)
    let code = [0x11, 0xAA, 0x40];
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    assert_eq!(ctx.return_data(), vec![0]);

    // PUSH0 -> NOT -> RET (return should be true/1)
    let code = [0x10, 0xAA, 0x40];
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    assert_eq!(ctx.return_data(), vec![1]);
}

#[test]
fn booland_boolor_follow_truthiness() {
    // true && false -> false
    let code = [0x11, 0x10, 0xAB, 0x40];
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    assert_eq!(ctx.return_data(), vec![0]);

    // false || true -> true
    let code = [0x10, 0x11, 0xAC, 0x40];
    let mut ctx2 = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx2.initialize(&code, &[]).expect("init");
    while !ctx2.step().expect("step").halted {}
    assert_eq!(ctx2.return_data(), vec![1]);
}

#[test]
fn bool_alias_opcodes_match_originals() {
    // PUSHT constant should integrate with NOT (0xAA)
    let code = [0x08, 0xAA, 0x40];
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    assert_eq!(
        ctx.return_data(),
        vec![0],
        "PUSHT followed by NOT should negate truthiness"
    );

    // bitwise NOT 0x90 should flip integer bits
    let code = [0x11, 0x90, 0x40]; // INVERT 1 -> !1 (i64) == -2 (two's complement)
    let mut ctx3 = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx3.initialize(&code, &[]).expect("init");
    while !ctx3.step().expect("step").halted {}
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&ctx3.return_data()[..8]);
    let value = i64::from_le_bytes(buf);
    assert_eq!(value, !1i64, "bitwise INVERT should use numeric complement");
}

#[test]
fn convert_coerces_to_boolean_and_bytearray() {
    // Integer 1 -> CONVERT to Boolean (0x20) should yield true.
    // NeoVM CONVERT takes a 1-byte immediate operand for the target StackItemType.
    let code = [0x11, 0xDB, 0x20, 0x40];
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    assert_eq!(ctx.return_data(), vec![1]);

    // Boolean false -> CONVERT to ByteArray (0x28) should yield [0]
    let code2 = [0x09, 0xDB, 0x28, 0x40];
    let mut ctx2 = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx2.initialize(&code2, &[]).expect("init");
    while !ctx2.step().expect("step").halted {}
    assert_eq!(ctx2.return_data(), vec![0]);
}
