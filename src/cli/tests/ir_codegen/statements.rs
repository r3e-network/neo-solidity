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
    let bytecode = generate_contract_bytecode(&mut metadata, &ir_module, false, 2, false)
        .expect("bytecode generation")
        .script;

    assert!(!bytecode.is_empty());

    // Mapping slots are hashed via CryptoLib.keccak256 (native contract) invoked
    // through System.Contract.Call.
    let call_id = interop_id_bytes("System.Contract.Call");
    assert!(bytecode.windows(4).any(|window| window == call_id));

    const CRYPTOLIB_HASH_LE: [u8; 20] = [
        0x1B, 0xF5, 0x75, 0xAB, 0x11, 0x89, 0x68, 0x84, 0x13, 0x61, 0x0A, 0x35, 0xA1, 0x28, 0x86,
        0xCD, 0xE0, 0xB6, 0x6C, 0x72,
    ];
    let mut cryptolib_push = vec![0x0C, 0x14];
    cryptolib_push.extend_from_slice(&CRYPTOLIB_HASH_LE);
    assert!(
        bytecode
            .windows(cryptolib_push.len())
            .any(|window| window == cryptolib_push.as_slice()),
        "expected CryptoLib script hash to be pushed for keccak256"
    );

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
    let bytecode = generate_contract_bytecode(&mut metadata, &ir_module, false, 2, false)
        .expect("bytecode generation")
        .script;

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
    // EmitEvent packs only the arguments, then swaps to place `eventName` on top for Runtime.Notify.
    expected_sequence.extend_from_slice(&[0x11, 0x12, 0x12, 0xC0, 0x4A, 0xD1, 0x50]);

    assert!(
        bytecode
            .windows(expected_sequence.len())
            .any(|window| window == expected_sequence),
        "expected event name to be pushed before args and packed"
    );
}

#[test]
fn modifiers_are_expanded_into_function_body() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract Modifiers {
        modifier onlyPositive(uint256 minValue) {
            require(minValue > 0, "minValue must be > 0");
            _;
        }

        function f(uint256 x) public onlyPositive(x) {
            // empty body
        }
    }
    "#;

    let metadata = analyse_source(source).expect("analysis failed");
    let module = ir::Module::from_contract(&metadata).expect("IR lowering failed");
    let f = module
        .functions
        .iter()
        .find(|function| function.name == "f")
        .expect("expected f function");

    let instrs = &f.basic_blocks[0].instructions;
    assert!(
        instrs
            .iter()
            .any(|instr| matches!(instr, ir::Instruction::Throw)),
        "expected expanded modifier require() to emit a Throw"
    );
}

