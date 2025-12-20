#[test]
fn address_code_length_uses_contract_getcontract_syscall() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract CodeLenHarness {
        function len(address account) public view returns (uint256) {
            return account.code.length;
        }
    }
    "#;

    let metadata = analyse_source(source).expect("analysis failed");
    let module = ir::Module::from_contract(&metadata).expect("IR lowering failed");
    let len_fn = module
        .functions
        .iter()
        .find(|function| function.name == "len")
        .expect("expected len function");

    let instrs = &len_fn.basic_blocks[0].instructions;
    let mut saw_getcontract = false;
    let mut saw_null = false;

    for instr in instrs {
        match instr {
            ir::Instruction::CallBuiltin {
                builtin: ir::BuiltinCall::Syscall(name),
                ..
            } if name == "System.Contract.GetContract" => saw_getcontract = true,
            ir::Instruction::PushLiteral(ir::LiteralValue::Null) => saw_null = true,
            _ => {}
        }
    }

    assert!(
        saw_getcontract,
        "expected code.length to call System.Contract.GetContract"
    );
    assert!(saw_null, "expected code.length to compare against NULL");
}

#[test]
fn iterator_currentkey_and_value_extract_storage_pair_elements() {
    use num_bigint::BigInt;
    use num_traits::{One, Zero};

    let source = r#"
    pragma solidity ^0.8.19;

    contract IteratorHarness {
        function read(bytes memory prefix) public returns (bytes memory key, bytes memory value) {
            bytes memory it = Storage.find(prefix);
            if (it.next()) {
                key = it.currentKey;
                value = it.value();
            }
        }
    }
    "#;

    let metadata = analyse_source(source).expect("analysis failed");
    let module = ir::Module::from_contract(&metadata).expect("IR lowering failed");
    let read_fn = module
        .functions
        .iter()
        .find(|function| function.name == "read")
        .expect("expected read function");
    let instrs = &read_fn.basic_blocks[0].instructions;

    let iterator_value_calls = instrs
        .iter()
        .filter(|instr| {
            matches!(
                instr,
                ir::Instruction::CallBuiltin { builtin: ir::BuiltinCall::Syscall(name), .. }
                    if name == "System.Iterator.Value"
            )
        })
        .count();
    assert!(
        iterator_value_calls >= 2,
        "expected iterator.currentKey and iterator.value() to use System.Iterator.Value"
    );

    let mut saw_key_extract = false;
    let mut saw_value_extract = false;
    for window in instrs.windows(2) {
        match window {
            [ir::Instruction::PushLiteral(ir::LiteralValue::Integer(n)), ir::Instruction::ArrayGet]
                if n.is_zero() =>
            {
                saw_key_extract = true
            }
            [ir::Instruction::PushLiteral(ir::LiteralValue::Integer(n)), ir::Instruction::ArrayGet]
                if n == &BigInt::one() =>
            {
                saw_value_extract = true
            }
            _ => {}
        }
    }

    assert!(saw_key_extract, "expected PICKITEM(0) for currentKey");
    assert!(saw_value_extract, "expected PICKITEM(1) for value()");
}
