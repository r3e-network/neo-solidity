use super::*;

#[test]
fn callt_emits_method_tokens_for_native_calls() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract CallTHarness {
        function supply() public view returns (uint256) {
            return NativeCalls.gasTotalSupply();
        }
    }
    "#;

    let artifacts = compile_contracts_with_options(
        source,
        false,
        CompileOptions {
            optimizer_level: 2,
            use_callt: true,
            deny_wildcard_permissions: false,
            deny_wildcard_contracts: false,
            deny_wildcard_methods: false,
            manifest_permissions: None,
        },
    )
    .expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let artifact = &artifacts[0];
    assert_eq!(
        artifact.tokens.len(),
        1,
        "expected exactly one method token for a single native call"
    );

    let token = &artifact.tokens[0];
    let expected_hash =
        super::bytecode::native_contract_hash(neo_solidity::ir::NativeContract::Gas);
    assert_eq!(token.hash, expected_hash);
    assert_eq!(token.method, "totalSupply");
    assert_eq!(token.parameters_count, 0);
    assert!(token.has_return_value);
    assert_eq!(token.call_flags, 0x05);

    assert!(
        artifact
            .bytecode
            .windows(3)
            .any(|w| w == [0x37, 0x00, 0x00]),
        "expected CALLT opcode with token index 0"
    );
}
