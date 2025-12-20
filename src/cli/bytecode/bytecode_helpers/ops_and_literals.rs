fn emit_binary_op(bytecode: &mut Vec<u8>, operator: ir::BinaryOperator) {
    let opcode = match operator {
        ir::BinaryOperator::Add => 0x9E,
        ir::BinaryOperator::Sub => 0x9F,
        ir::BinaryOperator::Mul => 0xA0,
        ir::BinaryOperator::Div => 0xA1,
        ir::BinaryOperator::Mod => 0xA2,
        ir::BinaryOperator::BitAnd => 0x91,
        ir::BinaryOperator::BitOr => 0x92,
        ir::BinaryOperator::BitXor => 0x93,
        ir::BinaryOperator::Shl => 0xA8,
        ir::BinaryOperator::Shr => 0xA9,
        ir::BinaryOperator::Lt => 0xB5,
        ir::BinaryOperator::Le => 0xB6,
        ir::BinaryOperator::Gt => 0xB7,
        ir::BinaryOperator::Ge => 0xB8,
        ir::BinaryOperator::Eq => 0x97,
        ir::BinaryOperator::Ne => 0x98,
    };
    bytecode.push(opcode);
}

fn push_literal_value(bytecode: &mut Vec<u8>, literal: &LiteralValue) {
    match literal {
        LiteralValue::Integer(value) => push_integer_bigint(bytecode, value),
        // Use NeoVM boolean opcodes so returned values match the manifest ABI type.
        LiteralValue::Boolean(true) => bytecode.push(0x08),  // PUSHT
        LiteralValue::Boolean(false) => bytecode.push(0x09), // PUSHF
        LiteralValue::String(bytes) => push_data(bytecode, bytes),
        LiteralValue::ByteArray(bytes) => push_data(bytecode, bytes),
        LiteralValue::Address(bytes) => push_data(bytecode, bytes),
        LiteralValue::Null => bytecode.push(0x0B),
    }
}

fn push_integer_bigint(bytecode: &mut Vec<u8>, value: &BigInt) {
    if value.is_zero() {
        bytecode.push(0x10);
        return;
    }

    if *value == BigInt::from(-1) {
        bytecode.push(0x0F);
        return;
    }

    if value.is_positive() {
        if let Some(n) = value.to_u8() {
            if n <= 16 {
                bytecode.push(0x10 + n);
                return;
            }
        }
    }

    // NeoVM integer literals should be pushed as Integer stack items so that operations like
    // `EQUAL` behave correctly for Solidity numeric types. Use the dedicated PUSHINT opcodes
    // whenever the value fits in their width, falling back to raw bytes only when needed.
    if let Some(n) = value.to_i64() {
        if (i8::MIN as i64..=i8::MAX as i64).contains(&n) {
            bytecode.push(0x00); // PUSHINT8
            bytecode.push(n as i8 as u8);
            return;
        }
        if (i16::MIN as i64..=i16::MAX as i64).contains(&n) {
            bytecode.push(0x01); // PUSHINT16
            bytecode.extend_from_slice(&(n as i16).to_le_bytes());
            return;
        }
        if (i32::MIN as i64..=i32::MAX as i64).contains(&n) {
            bytecode.push(0x02); // PUSHINT32
            bytecode.extend_from_slice(&(n as i32).to_le_bytes());
            return;
        }
        bytecode.push(0x03); // PUSHINT64
        bytecode.extend_from_slice(&n.to_le_bytes());
        return;
    }

    let bytes = value.to_signed_bytes_le();
    if bytes.len() <= 16 {
        bytecode.push(0x04); // PUSHINT128
        let fill = if value.is_negative() { 0xFF } else { 0x00 };
        let mut buf = [fill; 16];
        buf[..bytes.len()].copy_from_slice(&bytes);
        bytecode.extend_from_slice(&buf);
        return;
    }
    if bytes.len() <= 32 {
        bytecode.push(0x05); // PUSHINT256
        let fill = if value.is_negative() { 0xFF } else { 0x00 };
        let mut buf = [fill; 32];
        buf[..bytes.len()].copy_from_slice(&bytes);
        bytecode.extend_from_slice(&buf);
        return;
    }

    // Fallback: out-of-range literal (bigger than 256-bit signed). NeoVM integer literals are
    // signed, so we cannot encode values like `2^256-1` (Solidity `uint256.max`) with PUSHINT256
    // without changing their numeric meaning. Push the signed little-endian bytes and coerce them
    // into an Integer stack item via `value + 0`.
    push_data(bytecode, &bytes);
    bytecode.push(0x10); // PUSH0
    bytecode.push(0x9E); // ADD
}
