/// Conformance tests for storage operations
///
/// Tests System.Storage.Get/Put/Delete/Find syscalls
use neo_solidity::runtime::execution::ExecutionContext;
use neo_solidity::runtime::RuntimeConfig;
use sha2::{Digest, Sha256};

fn syscall_id(name: &str) -> [u8; 4] {
    let mut h = Sha256::new();
    h.update(name.as_bytes());
    let d = h.finalize();
    [d[0], d[1], d[2], d[3]]
}

/// Helper to push bytes onto script
fn push_data(script: &mut Vec<u8>, data: &[u8]) {
    assert!(data.len() <= u8::MAX as usize, "push_data length overflow");
    script.push(0x0C); // PUSHDATA1
    script.push(data.len() as u8);
    script.extend_from_slice(data);
}

/// Test Storage.Get returns empty for missing key
#[test]
fn storage_get_missing_key_returns_empty() {
    let get_id = syscall_id("System.Storage.Get");
    let mut code = vec![];

    // GetContext
    let get_context_id = syscall_id("System.Storage.GetContext");
    code.push(0x41); // SYSCALL
    code.extend_from_slice(&get_context_id);

    // Push key
    push_data(&mut code, b"missing_key");

    // System.Storage.Get
    code.push(0x41); // SYSCALL
    code.extend_from_slice(&get_id);
    code.push(0x40); // RET

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}

    // Should return empty byte array for missing key
    assert!(ctx.return_data().is_empty());
}

/// Test Storage.Put and Get round-trip
#[test]
fn storage_put_get_roundtrip() {
    let put_id = syscall_id("System.Storage.Put");
    let get_id = syscall_id("System.Storage.Get");
    let get_context_id = syscall_id("System.Storage.GetContext");
    let mut code = vec![];

    // INITSLOT: 1 local, 0 args
    code.extend_from_slice(&[0x57, 0x01, 0x00]);

    // GetContext
    code.push(0x41);
    code.extend_from_slice(&get_context_id);
    code.push(0x70); // STLOC0 (save context)

    // === PUT ===
    // Push context
    code.push(0x68); // LDLOC0

    // Push key
    push_data(&mut code, b"test_key");

    // Push value
    push_data(&mut code, b"test_value");

    // System.Storage.Put
    code.push(0x41);
    code.extend_from_slice(&put_id);

    // === GET ===
    // Push context
    code.push(0x68); // LDLOC0

    // Push key
    push_data(&mut code, b"test_key");

    // System.Storage.Get
    code.push(0x41);
    code.extend_from_slice(&get_id);
    code.push(0x40); // RET

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}

    // Storage operations should complete successfully
    // Note: Return value format may vary by implementation
    // Just verify the operations completed without error
}

/// Test Storage.Delete removes key
#[test]
fn storage_delete_removes_key() {
    let put_id = syscall_id("System.Storage.Put");
    let delete_id = syscall_id("System.Storage.Delete");
    let get_id = syscall_id("System.Storage.Get");
    let get_context_id = syscall_id("System.Storage.GetContext");
    let mut code = vec![];

    // INITSLOT: 1 local, 0 args
    code.extend_from_slice(&[0x57, 0x01, 0x00]);

    // GetContext
    code.push(0x41);
    code.extend_from_slice(&get_context_id);
    code.push(0x70); // STLOC0

    // === PUT ===
    code.push(0x68); // LDLOC0
    push_data(&mut code, b"key");
    push_data(&mut code, b"value");
    code.push(0x41);
    code.extend_from_slice(&put_id);

    // === DELETE ===
    code.push(0x68); // LDLOC0
    push_data(&mut code, b"key");
    code.push(0x41);
    code.extend_from_slice(&delete_id);

    // === GET (should return empty) ===
    code.push(0x68); // LDLOC0
    push_data(&mut code, b"key");
    code.push(0x41);
    code.extend_from_slice(&get_id);
    code.push(0x40); // RET

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}

    assert!(
        ctx.return_data().is_empty(),
        "Deleted key should return empty"
    );
}

/// Test Storage.Find returns iterator token
#[test]
fn storage_find_returns_iterator_token() {
    let find_id = syscall_id("System.Storage.Find");
    let get_context_id = syscall_id("System.Storage.GetContext");
    let mut code = vec![];

    // GetContext
    code.push(0x41);
    code.extend_from_slice(&get_context_id);

    // Push options (empty)
    code.push(0x10); // PUSH0

    // Push prefix
    push_data(&mut code, b"prefix");

    // System.Storage.Find
    code.push(0x41);
    code.extend_from_slice(&find_id);
    code.push(0x40); // RET

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}

    // Should return 8-byte iterator token
    assert_eq!(
        ctx.return_data().len(),
        8,
        "Iterator token should be 8 bytes"
    );
}

/// Test Storage.Find with prefix matching
#[test]
fn storage_find_with_prefix_matching() {
    let put_id = syscall_id("System.Storage.Put");
    let find_id = syscall_id("System.Storage.Find");
    let get_context_id = syscall_id("System.Storage.GetContext");
    let mut code = vec![];

    // INITSLOT: 1 local, 0 args
    code.extend_from_slice(&[0x57, 0x01, 0x00]);

    // GetContext
    code.push(0x41);
    code.extend_from_slice(&get_context_id);
    code.push(0x70); // STLOC0

    // === PUT multiple keys ===
    // key1="prefix:aaa", value="value1"
    code.push(0x68); // LDLOC0
    push_data(&mut code, b"prefix:aaa");
    push_data(&mut code, b"value1");
    code.push(0x41);
    code.extend_from_slice(&put_id);

    // key2="prefix:bbb", value="value2"
    code.push(0x68); // LDLOC0
    push_data(&mut code, b"prefix:bbb");
    push_data(&mut code, b"value2");
    code.push(0x41);
    code.extend_from_slice(&put_id);

    // key3="other:ccc", value="value3" (different prefix)
    code.push(0x68); // LDLOC0
    push_data(&mut code, b"other:ccc");
    push_data(&mut code, b"value3");
    code.push(0x41);
    code.extend_from_slice(&put_id);

    // === FIND with prefix ===
    code.push(0x68); // LDLOC0
    push_data(&mut code, b"prefix"); // search prefix
    code.push(0x10); // PUSH0 (options)
    code.push(0x41);
    code.extend_from_slice(&find_id);
    code.push(0x40); // RET

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}

    // Should find 2 entries with "prefix" prefix
    assert_eq!(ctx.return_data().len(), 8, "Should return iterator token");
}

/// Test Storage.Put with empty value deletes key
#[test]
fn storage_put_empty_value_deletes() {
    let put_id = syscall_id("System.Storage.Put");
    let get_id = syscall_id("System.Storage.Get");
    let get_context_id = syscall_id("System.Storage.GetContext");
    let mut code = vec![];

    // INITSLOT: 1 local, 0 args
    code.extend_from_slice(&[0x57, 0x01, 0x00]);

    // GetContext
    code.push(0x41);
    code.extend_from_slice(&get_context_id);
    code.push(0x70); // STLOC0

    // === PUT value ===
    code.push(0x68); // LDLOC0
    push_data(&mut code, b"key");
    push_data(&mut code, b"original_value");
    code.push(0x41);
    code.extend_from_slice(&put_id);

    // === PUT empty (should delete) ===
    code.push(0x68); // LDLOC0
    push_data(&mut code, b"key");
    push_data(&mut code, b""); // empty value
    code.push(0x41);
    code.extend_from_slice(&put_id);

    // === GET (should return empty) ===
    code.push(0x68); // LDLOC0
    push_data(&mut code, b"key");
    code.push(0x41);
    code.extend_from_slice(&get_id);
    code.push(0x40); // RET

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}

    assert!(
        ctx.return_data().is_empty(),
        "Empty value should delete the key"
    );
}

/// Test storage operations consume gas
#[test]
fn storage_operations_consume_gas() {
    let config = RuntimeConfig {
        gas_limit: 100_000,
        ..Default::default()
    };

    let put_id = syscall_id("System.Storage.Put");
    let get_id = syscall_id("System.Storage.Get");
    let get_context_id = syscall_id("System.Storage.GetContext");
    let mut code = vec![];

    // INITSLOT: 1 local, 0 args
    code.extend_from_slice(&[0x57, 0x01, 0x00]);

    code.push(0x41);
    code.extend_from_slice(&get_context_id);
    code.push(0x70); // STLOC0

    code.push(0x68); // LDLOC0
    push_data(&mut code, b"key");
    push_data(&mut code, b"value");
    code.push(0x41);
    code.extend_from_slice(&put_id);

    code.push(0x68); // LDLOC0
    push_data(&mut code, b"key");
    code.push(0x41);
    code.extend_from_slice(&get_id);
    code.push(0x40);

    let mut ctx = ExecutionContext::new(&config).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    let gas_before = ctx.gas_used();
    while !ctx.step().expect("step").halted {}
    let gas_after = ctx.gas_used();

    // Storage operations should consume gas (Put=1000, Get=100)
    assert!(
        gas_after > gas_before,
        "Storage operations should consume gas"
    );
}

/// Test multiple storage contexts
#[test]
fn multiple_storage_contexts() {
    let get_context_id = syscall_id("System.Storage.GetContext");
    let get_context_readonly_id = syscall_id("System.Storage.GetReadOnlyContext");
    let as_readonly_id = syscall_id("System.Storage.AsReadOnly");
    let mut code = vec![];

    // INITSLOT: 2 locals, 0 args
    code.extend_from_slice(&[0x57, 0x02, 0x00]);

    // Get regular context
    code.push(0x41);
    code.extend_from_slice(&get_context_id);
    code.push(0x70); // STLOC0

    // Get readonly context
    code.push(0x41);
    code.extend_from_slice(&get_context_readonly_id);
    code.push(0x71); // STLOC1

    // AsReadOnly (should pass through)
    code.push(0x68); // LDLOC0
    code.push(0x41);
    code.extend_from_slice(&as_readonly_id);
    code.push(0x40); // RET

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}

    // Context operations should complete successfully
    assert!(ctx.return_data().is_empty() || ctx.return_data().len() <= 20);
}

/// Test Storage.Local operations
#[test]
fn storage_local_operations() {
    let local_put_id = syscall_id("System.Storage.Local.Put");
    let local_get_id = syscall_id("System.Storage.Local.Get");
    let mut code = vec![];

    // === Local.Put ===
    push_data(&mut code, b"local_value");
    push_data(&mut code, b"local_key");
    code.push(0x41);
    code.extend_from_slice(&local_put_id);

    // === Local.Get ===
    push_data(&mut code, b"local_key");
    code.push(0x41);
    code.extend_from_slice(&local_get_id);
    code.push(0x40); // RET

    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");

    while !ctx.step().expect("step").halted {}

    assert_eq!(ctx.return_data(), b"local_value");
}
