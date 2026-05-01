use neo_devpack_solidity::runtime::execution::ExecutionContext;
use neo_devpack_solidity::runtime::RuntimeConfig;

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
    let code = vec![0x10, 0x41, 20, 136, 216, 206, 0x40];
    ctx.initialize(&code, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    let data = ctx.return_data();
    let mut buf = [0u8; 8];
    let copy = data.len().min(8);
    buf[..copy].copy_from_slice(&data[..copy]);
    let gas_left = u64::from_le_bytes(buf);
    assert!(gas_left < config.gas_limit, "gas left should be reduced");
}

#[test]
fn gas_exhaustion_halts_execution() {
    let config = RuntimeConfig {
        gas_limit: 2, // Very low limit
        ..RuntimeConfig::default()
    };
    let mut ctx = ExecutionContext::new(&config).expect("context init");
    // Multiple instructions that should exhaust gas
    ctx.initialize(&[0x10, 0x10, 0x10, 0x10, 0x10, 0x40], &[])
        .expect("init");

    let mut halted = false;
    let mut steps = 0;
    while steps < 10 {
        match ctx.step() {
            Ok(result) => {
                if result.halted {
                    halted = true;
                    break;
                }
            }
            Err(_) => {
                halted = true;
                break;
            }
        }
        steps += 1;
    }
    assert!(halted, "execution should halt due to gas exhaustion");
}

#[test]
fn syscall_consumes_additional_gas() {
    let config = RuntimeConfig {
        gas_limit: 1000,
        ..RuntimeConfig::default()
    };
    let mut ctx = ExecutionContext::new(&config).expect("context init");

    // Simple code with syscall: PUSH0, RET
    let simple_code = vec![0x10, 0x40];
    ctx.initialize(&simple_code, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    let simple_gas = ctx.gas_used();

    // Reset and run code with syscall
    let mut ctx2 = ExecutionContext::new(&config).expect("context init");
    // SYSCALL System.Runtime.GetTime, RET
    let syscall_code = vec![
        0x41, 183, 195, 136, 3,    // SYSCALL System.Runtime.GetTime
        0x40, // RET
    ];
    ctx2.initialize(&syscall_code, &[]).expect("init");
    while !ctx2.step().expect("step").halted {}
    let syscall_gas = ctx2.gas_used();

    assert!(
        syscall_gas > simple_gas,
        "syscall should consume more gas than simple instructions"
    );
}

#[test]
fn storage_operations_consume_significant_gas() {
    use sha2::{Digest, Sha256};

    fn syscall_id(name: &str) -> [u8; 4] {
        let hash = Sha256::digest(name.as_bytes());
        [hash[0], hash[1], hash[2], hash[3]]
    }

    let config = RuntimeConfig {
        gas_limit: 10000,
        ..RuntimeConfig::default()
    };
    let mut ctx = ExecutionContext::new(&config).expect("context init");

    let get_ctx_id = syscall_id("System.Storage.GetContext");
    let put_id = syscall_id("System.Storage.Put");

    // Storage.GetContext, PUSH key, PUSH value, Storage.Put, RET
    let code = vec![
        0x41,
        get_ctx_id[0],
        get_ctx_id[1],
        get_ctx_id[2],
        get_ctx_id[3], // GetContext
        0x0C,
        0x03,
        0x6B,
        0x65,
        0x79, // PUSHDATA1 "key"
        0x0C,
        0x05,
        0x76,
        0x61,
        0x6C,
        0x75,
        0x65, // PUSHDATA1 "value"
        0x41,
        put_id[0],
        put_id[1],
        put_id[2],
        put_id[3], // Storage.Put
        0x40,      // RET
    ];

    ctx.initialize(&code, &[]).expect("init");
    while !ctx.step().expect("step").halted {}

    let gas_used = ctx.gas_used();
    assert!(
        gas_used > 100,
        "storage operations should consume significant gas (used: {gas_used})"
    );
}
