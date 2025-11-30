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
fn deploy_and_update_contract_tracks_state() {
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("ctx");

    // Deploy: params [nef, manifest]
    let deploy_id = syscall_id("System.ContractManagement.Deploy");
    let deploy = vec![
        0x0C,
        0x02,
        0xAA,
        0xAB, // nef bytes
        0x0C,
        0x02,
        0xBA,
        0xBB, // manifest bytes
        0x12, // count=2
        0xC0, // PACK -> params array
        0x41,
        deploy_id[0],
        deploy_id[1],
        deploy_id[2],
        deploy_id[3],
        0x40, // RET
    ];

    ctx.initialize(&deploy, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    assert_eq!(ctx.contract_registry_len(), 1, "one contract registered");
    let hashes = ctx.contract_hashes();
    let hash = hashes.first().expect("hash");
    assert_eq!(ctx.contract_update_counter(hash), Some(0));

    // Update with new nef/manifest
    let update_id = syscall_id("System.ContractManagement.Update");
    let update = vec![
        0x0C,
        0x02,
        0xCC,
        0xCD, // nef bytes
        0x0C,
        0x02,
        0xDC,
        0xDD, // manifest bytes
        0x12, // count=2
        0xC0, // PACK
        0x41,
        update_id[0],
        update_id[1],
        update_id[2],
        update_id[3],
        0x40,
    ];
    ctx.initialize(&update, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    assert_eq!(ctx.contract_registry_len(), 1);
    assert_eq!(ctx.contract_update_counter(hash), Some(1));
}
