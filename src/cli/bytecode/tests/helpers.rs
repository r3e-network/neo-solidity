// ==================== Helper Function Unit Tests ====================

#[test]
fn emit_load_local_uses_short_form_for_small_indices() {
    let mut bytecode = Vec::new();
    emit_load_local(&mut bytecode, 0);
    assert_eq!(bytecode, vec![0x68], "LDLOC0");

    bytecode.clear();
    emit_load_local(&mut bytecode, 6);
    assert_eq!(bytecode, vec![0x6E], "LDLOC6");

    bytecode.clear();
    emit_load_local(&mut bytecode, 7);
    assert_eq!(bytecode, vec![0x6F, 0x07], "LDLOC with index");
}

#[test]
fn emit_store_local_uses_short_form_for_small_indices() {
    let mut bytecode = Vec::new();
    emit_store_local(&mut bytecode, 0);
    assert_eq!(bytecode, vec![0x70], "STLOC0");

    bytecode.clear();
    emit_store_local(&mut bytecode, 6);
    assert_eq!(bytecode, vec![0x76], "STLOC6");

    bytecode.clear();
    emit_store_local(&mut bytecode, 7);
    assert_eq!(bytecode, vec![0x77, 0x07], "STLOC with index");
}

#[test]
fn emit_binary_op_emits_correct_opcodes() {
    let test_cases = [
        (ir::BinaryOperator::Add, 0x9E),
        (ir::BinaryOperator::Sub, 0x9F),
        (ir::BinaryOperator::Mul, 0xA0),
        (ir::BinaryOperator::Div, 0xA1),
        (ir::BinaryOperator::Mod, 0xA2),
        (ir::BinaryOperator::BitAnd, 0x91),
        (ir::BinaryOperator::BitOr, 0x92),
        (ir::BinaryOperator::BitXor, 0x93),
        (ir::BinaryOperator::Lt, 0xB5),
        (ir::BinaryOperator::Eq, 0x97),
        (ir::BinaryOperator::Ne, 0x98),
    ];

    for (op, expected_opcode) in test_cases {
        let mut bytecode = Vec::new();
        emit_binary_op(&mut bytecode, op);
        assert_eq!(
            bytecode,
            vec![expected_opcode],
            "operator {op:?} should emit opcode 0x{expected_opcode:02X}"
        );
    }
}

#[test]
fn push_literal_value_handles_all_types() {
    // Boolean true
    let mut bytecode = Vec::new();
    push_literal_value(&mut bytecode, &LiteralValue::Boolean(true));
    assert_eq!(bytecode, vec![0x08], "PUSHT for true");

    // Boolean false
    bytecode.clear();
    push_literal_value(&mut bytecode, &LiteralValue::Boolean(false));
    assert_eq!(bytecode, vec![0x09], "PUSHF for false");

    // Small integer
    bytecode.clear();
    push_literal_value(&mut bytecode, &LiteralValue::Integer(BigInt::from(5)));
    assert_eq!(bytecode, vec![0x15], "PUSH5");

    // Zero
    bytecode.clear();
    push_literal_value(&mut bytecode, &LiteralValue::Integer(BigInt::from(0)));
    assert_eq!(bytecode, vec![0x10], "PUSH0");

    // Empty string
    bytecode.clear();
    push_literal_value(&mut bytecode, &LiteralValue::String(vec![]));
    assert_eq!(bytecode, vec![0x0C, 0x00], "PUSHDATA1 with empty data");

    // Short byte array
    bytecode.clear();
    push_literal_value(&mut bytecode, &LiteralValue::ByteArray(vec![0xAB, 0xCD]));
    assert_eq!(
        bytecode,
        vec![0x0C, 0x02, 0xAB, 0xCD],
        "PUSHDATA1 with bytes"
    );
}

#[test]
fn push_integer_bigint_handles_special_values() {
    // Zero
    let mut bytecode = Vec::new();
    push_integer_bigint(&mut bytecode, &BigInt::from(0));
    assert_eq!(bytecode, vec![0x10], "PUSH0");

    // -1
    bytecode.clear();
    push_integer_bigint(&mut bytecode, &BigInt::from(-1));
    assert_eq!(bytecode, vec![0x0F], "PUSHM1");

    // Small positive (1-16)
    for i in 1..=16u8 {
        bytecode.clear();
        push_integer_bigint(&mut bytecode, &BigInt::from(i));
        assert_eq!(bytecode, vec![0x10 + i], "PUSH{i}");
    }

    // Larger positive
    bytecode.clear();
    push_integer_bigint(&mut bytecode, &BigInt::from(255));
    assert!(bytecode.len() > 1, "larger integers use PUSHDATA");
}

#[test]
fn push_integer_bigint_coerces_out_of_range_unsigned_literals() {
    use num_traits::One;

    // 2^256 - 1 does not fit in a signed 256-bit integer literal, but is a valid Solidity uint256.
    let mut value = BigInt::one();
    value <<= 256usize;
    value -= BigInt::one();

    let mut bytecode = Vec::new();
    push_integer_bigint(&mut bytecode, &value);

    assert!(
        bytecode.ends_with(&[0x10, 0x9E]),
        "expected large integer literal lowering to coerce via PUSH0 + ADD"
    );
}

#[test]
fn append_default_value_handles_all_types() {
    // Integer
    let mut bytecode = Vec::new();
    append_default_value(
        &mut bytecode,
        &ValueType::Integer {
            signed: false,
            bits: 256,
        },
    );
    assert_eq!(bytecode, vec![0x10], "default integer is 0");

    // Boolean
    bytecode.clear();
    append_default_value(&mut bytecode, &ValueType::Boolean);
    assert_eq!(bytecode, vec![0x10], "default boolean is false (0)");

    // Array
    bytecode.clear();
    append_default_value(
        &mut bytecode,
        &ValueType::Array(Box::new(ValueType::Boolean)),
    );
    assert_eq!(bytecode, vec![0xC2], "NEWARRAY0");

    // Mapping
    bytecode.clear();
    append_default_value(
        &mut bytecode,
        &ValueType::Mapping {
            key: Box::new(ValueType::Address),
            value: Box::new(ValueType::Integer {
                signed: false,
                bits: 256,
            }),
        },
    );
    assert_eq!(bytecode, vec![0xC8], "NEWMAP");

    // Any
    bytecode.clear();
    append_default_value(&mut bytecode, &ValueType::Any);
    assert_eq!(bytecode, vec![0x0B], "PUSHNULL");
}

#[test]
fn interop_id_bytes_produces_consistent_hashes() {
    let id1 = interop_id_bytes("System.Storage.Get");
    let id2 = interop_id_bytes("System.Storage.Get");
    assert_eq!(id1, id2, "same syscall should produce same ID");

    let id3 = interop_id_bytes("System.Storage.Put");
    assert_ne!(id1, id3, "different syscalls should produce different IDs");

    assert_eq!(id1.len(), 4, "interop ID should be 4 bytes");
}
