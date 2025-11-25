use neo_solidity::runtime::execution::ExecutionContext;
use neo_solidity::runtime::RuntimeConfig;

#[test]
fn gas_used_increments_per_instruction() {
    let config = RuntimeConfig {
        gas_limit: 10,
        ..RuntimeConfig::default()
    };
    let mut ctx = ExecutionContext::new(&config).expect("context init");
    // Two cheap instructions: PUSH0, NOP
    ctx.initialize(&[0x10, 0x21], &[]).expect("init");
    let start_gas = ctx.gas_used();
    ctx.step().expect("push");
    let after_push = ctx.gas_used();
    ctx.step().expect("nop");
    let after_nop = ctx.gas_used();
    assert!(after_push > start_gas, "gas should increment after push");
    assert!(after_nop > after_push, "gas should increment after nop");
}

#[test]
fn get_gas_left_reflects_usage() {
    let config = RuntimeConfig {
        gas_limit: 20,
        ..RuntimeConfig::default()
    };
    let mut ctx = ExecutionContext::new(&config).expect("context init");
    // PUSH0, GETGASLEFT, RET
    let code = vec![0x10, 0x41, 150, 39, 78, 22, 0x40];
    ctx.initialize(&code, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    let data = ctx.return_data();
    let mut buf = [0u8; 8];
    let copy = data.len().min(8);
    buf[..copy].copy_from_slice(&data[..copy]);
    let gas_left = u64::from_le_bytes(buf);
    assert!(gas_left < config.gas_limit, "gas left should be reduced");
}
