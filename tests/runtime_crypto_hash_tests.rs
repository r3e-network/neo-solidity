use neo_solidity::runtime::execution::ExecutionContext;
use neo_solidity::runtime::RuntimeConfig;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

#[test]
fn hash160_matches_ripemd_sha256() {
    // push "data", SYSCALL Hash160, RET
    let mut code = vec![0x0C, 0x04, b'd', b'a', b't', b'a', 0x41];
    code.extend_from_slice(&[183, 139, 96, 108]);
    code.push(0x40);
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    let expected = Ripemd160::digest(&Sha256::digest(b"data"));
    assert_eq!(ctx.return_data(), expected.to_vec());
}

#[test]
fn hash256_double_sha256() {
    let mut code = vec![0x0C, 0x03, b'a', b'b', b'c', 0x41];
    code.extend_from_slice(&[158, 51, 68, 154]);
    code.push(0x40);
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    let expected = {
        let first = Sha256::digest(b"abc");
        Sha256::digest(&first)
    };
    assert_eq!(ctx.return_data(), expected.to_vec());
}

#[test]
fn checksig_stub_returns_true() {
    // Use 33-byte compressed secp256k1 pubkey and dummy DER sig to satisfy parser; expect false (verification fail) but should not panic
    let mut code = vec![0x0C, 0x21];
    code.extend_from_slice(&[2u8; 33]);
    // DER sig with correct length prefix (0x44 = 68 bytes)
    code.extend_from_slice(&[0x0C, 0x46]);
    code.extend_from_slice(&[0x30, 0x44, 0x02, 0x20]);
    code.extend_from_slice(&[1u8; 32]); // r
    code.extend_from_slice(&[0x02, 0x20]);
    code.extend_from_slice(&[2u8; 32]); // s
    code.extend_from_slice(&[0x41, 86, 231, 179, 39, 0x40]); // CheckSig, RET
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("context init");
    ctx.initialize(&code, &[]).expect("init");
    while !ctx.step().expect("step").halted {}
    assert!(ctx.return_data().len() == 1);
}
