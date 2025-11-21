use super::*;
use neo_solidity::solidity::analyse_source;

#[test]
fn mapping_code_generation_emits_storage_ops() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract MappingExample {
        mapping(address => uint256) public balances;

        function setBalance(address owner, uint256 amount) public {
            balances[owner] = amount;
        }

        function getBalance(address owner) public view returns (uint256) {
            return balances[owner];
        }
    }
    "#;

    let mut metadata = analyse_source(source).expect("analysis failed");
    let ir_module = ir::Module::from_contract(&metadata).expect("IR lowering failed");
    let bytecode = generate_contract_bytecode(&mut metadata, &ir_module, false);

    assert!(!bytecode.is_empty());

    let sha_id = interop_id_bytes("System.Crypto.SHA256");
    assert!(bytecode.windows(4).any(|window| window == sha_id));

    let put_id = interop_id_bytes("System.Storage.Put");
    assert!(bytecode.windows(4).any(|window| window == put_id));
    let get_id = interop_id_bytes("System.Storage.Get");
    assert!(bytecode.windows(4).any(|window| window == get_id));
}

#[test]
fn event_emission_places_name_first_in_payload() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract EventOrder {
        event Ping(uint256 a, uint256 b);

        function fire() public {
            emit Ping(1, 2);
        }
    }
    "#;

    let mut metadata = analyse_source(source).expect("analysis failed");
    let ir_module = ir::Module::from_contract(&metadata).expect("IR lowering failed");
    let bytecode = generate_contract_bytecode(&mut metadata, &ir_module, false);

    let notify_id = interop_id_bytes("System.Runtime.Notify");
    let notify_sequence: Vec<u8> = std::iter::once(0x41u8).chain(notify_id).collect();
    assert!(
        bytecode
            .windows(notify_sequence.len())
            .any(|window| window == notify_sequence),
        "expected Runtime.Notify syscall"
    );

    let mut expected_sequence = vec![0x0C, 0x04];
    expected_sequence.extend_from_slice(b"Ping");
    expected_sequence.extend_from_slice(&[0x11, 0x12, 0x13, 0xC0]);

    assert!(
        bytecode
            .windows(expected_sequence.len())
            .any(|window| window == expected_sequence),
        "expected event name to be pushed before args and packed"
    );
}
