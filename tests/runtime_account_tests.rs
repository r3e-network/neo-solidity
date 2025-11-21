use neo_solidity::runtime::execution::ExecutionContext;
use neo_solidity::runtime::RuntimeConfig;

#[test]
fn normalize_account_accepts_prefixed() {
    let cfg = RuntimeConfig::default();
    let mut ctx = ExecutionContext::new(&cfg).expect("ctx");
    // override caller with prefixed hash should succeed
    ctx.override_caller_account("0x0123456789abcdef0123456789abcdef01234567")
        .expect("valid address");
}

#[test]
fn normalize_account_rejects_short_address() {
    let cfg = RuntimeConfig::default();
    let mut ctx = ExecutionContext::new(&cfg).expect("ctx");
    let result = ctx.override_caller_account("0x1234");
    assert!(result.is_err(), "expected invalid address to error");
}
