use neo_solidity::runtime::execution::ExecutionContext;
use neo_solidity::runtime::RuntimeConfig;

#[test]
fn jmp_out_of_bounds_produces_error() {
    // JMP with offset that would jump past bytecode end
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&[0x22, 0x7F], &[]).expect("init");

    // First step executes JMP and should error
    let result = ctx.step();
    assert!(result.is_err(), "expected JMP out of bounds to error");
    let err = format!("{}", result.unwrap_err());
    assert!(err.contains("jump target out of bounds"), "unexpected error: {err}");
}

#[test]
fn jmpif_skips_when_condition_false() {
    // PUSH0 (false), JMPIF +2, PUSH1, RET
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    let code = [0x11, 0x23, 0x02, 0x12, 0x40];
    ctx.initialize(&code, &[]).expect("init");

    ctx.step().expect("push0");
    let step = ctx.step().expect("jmpif");
    assert_eq!(step.instruction_pointer as usize, 3, "JMPIF should skip two bytes when false");

    // Next instruction should be PUSH1
    let next = ctx.step().expect("push1");
    assert_eq!(next.instruction_pointer as usize, 4);
}

#[test]
fn jmpif_jumps_forward_when_true() {
    // PUSH1 (true), JMPIF +1 (to RET), NOP, RET
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    let code = [0x12, 0x23, 0x01, 0x21, 0x40];
    ctx.initialize(&code, &[]).expect("init");

    ctx.step().expect("push1");
    let step = ctx.step().expect("jmpif");
    assert_eq!(step.instruction_pointer as usize, 4, "JMPIF should jump to RET target");

    let ret = ctx.step().expect("ret");
    assert!(ret.halted, "RET should halt execution");
}
