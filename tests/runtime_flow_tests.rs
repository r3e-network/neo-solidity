use neo_solidity::runtime::execution::ExecutionContext;
use neo_solidity::runtime::RuntimeConfig;

#[test]
fn jmp_out_of_bounds_produces_error() {
    // JMP with offset that would jump past bytecode end
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&[0x22, 0x0A, 0x00, 0x00, 0x00], &[])
        .expect("init");

    // First step executes JMP and should error
    let result = ctx.step();
    assert!(result.is_err(), "expected JMP out of bounds to error");
    let err = format!("{}", result.unwrap_err());
    assert!(
        err.contains("jump target out of bounds"),
        "unexpected error: {err}"
    );
}

#[test]
fn jmpif_skips_when_condition_false() {
    // PUSH0 (false), JMPIF -> RET (target 7), PUSH1, RET
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    let code = [0x10, 0x24, 0x07, 0x00, 0x00, 0x00, 0x11, 0x40];
    ctx.initialize(&code, &[]).expect("init");

    ctx.step().expect("push0");
    let step = ctx.step().expect("jmpif");
    assert_eq!(
        step.instruction_pointer as usize, 6,
        "JMPIF should advance past instruction when condition is false"
    );

    // Next instruction should be PUSH1
    let next = ctx.step().expect("push1");
    assert_eq!(next.instruction_pointer as usize, 7);
}

#[test]
fn jmpif_jumps_forward_when_true() {
    // PUSH1 (true), JMPIF -> RET (target 7), NOP, RET
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    let code = [0x11, 0x24, 0x07, 0x00, 0x00, 0x00, 0x21, 0x40];
    ctx.initialize(&code, &[]).expect("init");

    ctx.step().expect("push1");
    let step = ctx.step().expect("jmpif");
    assert_eq!(
        step.instruction_pointer as usize, 7,
        "JMPIF should jump to RET target"
    );

    let ret = ctx.step().expect("ret");
    assert!(ret.halted, "RET should halt execution");
}

#[test]
fn jmpifnot_jumps_when_false() {
    // PUSH0 (false), JMPIFNOT -> RET (target 7)
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    let code = [0x10, 0x26, 0x07, 0x00, 0x00, 0x00, 0x21, 0x40];
    ctx.initialize(&code, &[]).expect("init");

    ctx.step().expect("push0");
    let step = ctx.step().expect("jmpifnot");
    assert_eq!(
        step.instruction_pointer as usize, 7,
        "JMPIFNOT should skip to RET when condition is false"
    );
    let ret = ctx.step().expect("ret");
    assert!(ret.halted, "RET should halt execution");
}

#[test]
fn jmp_accepts_wide_offsets() {
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    let target: u32 = 250;
    let mut code = vec![0x22];
    code.extend_from_slice(&target.to_le_bytes());
    let padding = target as usize - 5;
    code.extend(std::iter::repeat(0x21).take(padding)); // fill with NOPs
    code.push(0x40); // RET at jump target

    ctx.initialize(&code, &[]).expect("init");

    let step = ctx.step().expect("jmp");
    assert_eq!(
        step.instruction_pointer, target,
        "JMP should support 4-byte absolute offsets"
    );
}
