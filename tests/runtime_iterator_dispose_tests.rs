use neo_solidity::runtime::execution::ExecutionContext;
use neo_solidity::runtime::RuntimeConfig;
use sha2::{Digest, Sha256};

fn syscall_id(name: &str) -> [u8; 4] {
    let mut h = Sha256::new();
    h.update(name.as_bytes());
    let d = h.finalize();
    [d[0], d[1], d[2], d[3]]
}

#[test]
fn iterator_dispose_removes_handle() {
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("ctx");
    let find = syscall_id("System.Storage.Find");
    let get_context = syscall_id("System.Storage.GetContext");
    let dispose = syscall_id("System.Iterator.Dispose");
    let istype_marker = [0x80u8]; // iterator type code

    // GetContext -> push empty prefix -> Find -> DUP token -> Dispose -> DROP bool
    // push type marker 0x80 -> ISTYPE (should be false) -> RET
    let script = vec![
        0x41, get_context[0], get_context[1], get_context[2], get_context[3],
        0x0C, 0x00,
        0x41, find[0], find[1], find[2], find[3],
        0x4A,
        0x41, dispose[0], dispose[1], dispose[2], dispose[3],
        0x45,
        0x0C, 0x01, istype_marker[0],
        0xD9,
        0x40,
    ];

    ctx.initialize(&script, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    assert_eq!(ctx.return_data(), vec![0u8]);
}
