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

#[test]
fn multiple_deploys_create_separate_contracts() {
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("ctx");

    let deploy_id = syscall_id("System.ContractManagement.Deploy");

    // Deploy first contract
    let deploy1 = vec![
        0x0C, 0x02, 0x01, 0x01, // nef bytes
        0x0C, 0x02, 0x02, 0x02, // manifest bytes
        0x12, 0xC0,
        0x41, deploy_id[0], deploy_id[1], deploy_id[2], deploy_id[3],
        0x40,
    ];
    ctx.initialize(&deploy1, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    assert_eq!(ctx.contract_registry_len(), 1);

    // Deploy second contract with different bytes
    let deploy2 = vec![
        0x0C, 0x02, 0x03, 0x03, // different nef bytes
        0x0C, 0x02, 0x04, 0x04, // different manifest bytes
        0x12, 0xC0,
        0x41, deploy_id[0], deploy_id[1], deploy_id[2], deploy_id[3],
        0x40,
    ];
    ctx.initialize(&deploy2, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    assert_eq!(ctx.contract_registry_len(), 2, "should have two contracts");

    // Verify they have different hashes
    let hashes = ctx.contract_hashes();
    assert_eq!(hashes.len(), 2);
    assert_ne!(hashes[0], hashes[1], "contracts should have different hashes");
}

#[test]
fn update_increments_counter_multiple_times() {
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("ctx");

    // Deploy
    let deploy_id = syscall_id("System.ContractManagement.Deploy");
    let deploy = vec![
        0x0C, 0x02, 0xAA, 0xAB,
        0x0C, 0x02, 0xBA, 0xBB,
        0x12, 0xC0,
        0x41, deploy_id[0], deploy_id[1], deploy_id[2], deploy_id[3],
        0x40,
    ];
    ctx.initialize(&deploy, &[]).expect("init");
    while !ctx.step().expect("step").halted {}

    let hashes = ctx.contract_hashes();
    let hash = hashes.first().expect("hash");
    assert_eq!(ctx.contract_update_counter(hash), Some(0));

    // Update multiple times
    let update_id = syscall_id("System.ContractManagement.Update");
    for i in 1..=3 {
        let update = vec![
            0x0C, 0x02, 0xCC, i as u8,
            0x0C, 0x02, 0xDC, i as u8,
            0x12, 0xC0,
            0x41, update_id[0], update_id[1], update_id[2], update_id[3],
            0x40,
        ];
        ctx.initialize(&update, &[]).expect("init");
        while !ctx.step().expect("step").halted {}
        assert_eq!(
            ctx.contract_update_counter(hash),
            Some(i),
            "update counter should be {} after {} updates",
            i,
            i
        );
    }
}
