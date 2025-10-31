use neo_solidity::runtime::execution::{ExecutionContext, StackItem};
use neo_solidity::runtime::RuntimeConfig;
use sha2::{Digest, Sha256};

fn syscall_bytes(name: &str) -> [u8; 4] {
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    let digest = hasher.finalize();
    [digest[0], digest[1], digest[2], digest[3]]
}

fn build_script(syscalls: &[&str]) -> Vec<u8> {
    let mut bytecode = Vec::with_capacity(syscalls.len() * 5 + 1);
    for name in syscalls {
        bytecode.push(0x41);
        bytecode.extend_from_slice(&syscall_bytes(name));
    }
    bytecode.push(0x40); // RET
    bytecode
}

#[test]
fn execution_context_reports_default_metadata() {
    let config = RuntimeConfig {
        default_block_height: 1_234,
        default_timestamp: 9_876,
        contract_account: "0x1122334455667788990011223344556677889900".to_string(),
        ..RuntimeConfig::default()
    };

    let mut context = ExecutionContext::new(&config).expect("context creation");
    let script = build_script(&[
        "System.Blockchain.GetHeight",
        "System.Runtime.GetTime",
        "System.Runtime.CallingScriptHash",
        "System.Runtime.GetInvocationCounter",
        "System.Runtime.GetInvocationCounter",
    ]);

    context
        .initialize(&script, &[])
        .expect("context initialization");

    loop {
        let step = context.step().expect("execution step");
        if step.halted {
            break;
        }
    }

    match context.pop_stack().expect("second invocation counter") {
        StackItem::UnsignedInteger(value) => assert_eq!(value, 2),
        other => panic!("expected second invocation counter, got {:?}", other),
    }

    match context.pop_stack().expect("first invocation counter") {
        StackItem::UnsignedInteger(value) => assert_eq!(value, 1),
        other => panic!("expected first invocation counter, got {:?}", other),
    }

    match context.pop_stack().expect("calling script hash") {
        StackItem::ByteArray(bytes) => {
            let expected =
                hex::decode("1122334455667788990011223344556677889900").expect("valid hex");
            assert_eq!(bytes, expected);
        }
        other => panic!("expected calling script hash bytes, got {:?}", other),
    }

    match context.pop_stack().expect("timestamp") {
        StackItem::UnsignedInteger(value) => assert_eq!(value, config.default_timestamp),
        other => panic!("expected timestamp, got {:?}", other),
    }

    match context.pop_stack().expect("block height") {
        StackItem::UnsignedInteger(value) => assert_eq!(value, config.default_block_height),
        other => panic!("expected block height, got {:?}", other),
    }

    assert_eq!(context.stack_depth(), 0);
}

#[test]
fn execution_context_applies_metadata_overrides() {
    let mut context = ExecutionContext::new(&RuntimeConfig::default()).expect("context creation");
    context.override_block_height(77);
    context.override_timestamp(1_111);
    context
        .override_caller_account("0x0102030405060708090a0102030405060708090a")
        .expect("caller override");

    let script = build_script(&[
        "System.Blockchain.GetHeight",
        "System.Runtime.GetTime",
        "System.Runtime.CallingScriptHash",
    ]);

    context
        .initialize(&script, &[])
        .expect("context initialization");

    loop {
        let step = context.step().expect("execution step");
        if step.halted {
            break;
        }
    }

    match context.pop_stack().expect("caller hash") {
        StackItem::ByteArray(bytes) => {
            let expected =
                hex::decode("0102030405060708090a0102030405060708090a").expect("valid hex");
            assert_eq!(bytes, expected);
        }
        other => panic!("expected caller script hash, got {:?}", other),
    }

    match context.pop_stack().expect("timestamp") {
        StackItem::UnsignedInteger(value) => assert_eq!(value, 1_111),
        other => panic!("expected override timestamp, got {:?}", other),
    }

    match context.pop_stack().expect("block height") {
        StackItem::UnsignedInteger(value) => assert_eq!(value, 77),
        other => panic!("expected override block height, got {:?}", other),
    }

    assert_eq!(context.stack_depth(), 0);
}
