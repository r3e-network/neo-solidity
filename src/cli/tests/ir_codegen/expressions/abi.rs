#[test]
fn abi_encode_preserves_argument_order() {
    let source = r#"
    pragma solidity ^0.8.20;

    contract EncodeOrder {
        function encode() public pure returns (bytes memory) {
            return abi.encode(uint256(1), uint256(2));
        }
    }
    "#;

    let mut metadata = analyse_source(source).expect("analysis failed");
    let ir_module = ir::Module::from_contract(&metadata).expect("IR lowering failed");
    let bytecode = generate_contract_bytecode(&mut metadata, &ir_module, false, 2, false)
        .expect("bytecode generation")
        .script;

    // Expect: PUSH1, PUSH2, PUSH2(count), PACK, DUP, REVERSEITEMS
    let expected_sequence = [0x11u8, 0x12u8, 0x12u8, 0xC0u8, 0x4Au8, 0xD1u8];
    assert!(
        bytecode
            .windows(expected_sequence.len())
            .any(|window| window == expected_sequence),
        "expected abi.encode to reverse PACK order via DUP+REVERSEITEMS"
    );
}
