impl ExecutionContext {
    fn bitwise_not(&self, value: StackItem) -> Result<StackItem, RuntimeError> {
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(!v)),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(!v)),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operand for bitwise NOT".to_string(),
            }),
        }
    }

    fn bitwise_and(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x & y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x & y))
            }
            // Handle Boolean types - convert to integers for bitwise operations
            (StackItem::Boolean(x), StackItem::Boolean(y)) => {
                Ok(StackItem::Boolean(x && y))
            }
            (StackItem::Boolean(x), StackItem::Integer(y)) => {
                let x_int = if x { 1i64 } else { 0i64 };
                Ok(StackItem::Integer(x_int & y))
            }
            (StackItem::Integer(x), StackItem::Boolean(y)) => {
                let y_int = if y { 1i64 } else { 0i64 };
                Ok(StackItem::Integer(x & y_int))
            }
            (StackItem::Boolean(x), StackItem::UnsignedInteger(y)) => {
                let x_int = if x { 1u64 } else { 0u64 };
                Ok(StackItem::UnsignedInteger(x_int & y))
            }
            (StackItem::UnsignedInteger(x), StackItem::Boolean(y)) => {
                let y_int = if y { 1u64 } else { 0u64 };
                Ok(StackItem::UnsignedInteger(x & y_int))
            }
            // Handle Null as zero/false
            (StackItem::Null, StackItem::Integer(_)) => Ok(StackItem::Integer(0)),
            (StackItem::Integer(_), StackItem::Null) => Ok(StackItem::Integer(0)),
            (StackItem::Null, StackItem::UnsignedInteger(_)) => Ok(StackItem::UnsignedInteger(0)),
            (StackItem::UnsignedInteger(_), StackItem::Null) => Ok(StackItem::UnsignedInteger(0)),
            (StackItem::Null, StackItem::Boolean(_)) => Ok(StackItem::Boolean(false)),
            (StackItem::Boolean(_), StackItem::Null) => Ok(StackItem::Boolean(false)),
            (StackItem::Null, StackItem::Null) => Ok(StackItem::Integer(0)),
            // Handle ByteArray - convert to integer (little-endian)
            (StackItem::ByteArray(ref x), StackItem::Integer(y)) => {
                let x_bytes = x.borrow();
                let x_int = Self::bytes_to_i64_le(&x_bytes);
                Ok(StackItem::Integer(x_int & y))
            }
            (StackItem::Integer(x), StackItem::ByteArray(ref y)) => {
                let y_bytes = y.borrow();
                let y_int = Self::bytes_to_i64_le(&y_bytes);
                Ok(StackItem::Integer(x & y_int))
            }
            (StackItem::ByteArray(ref x), StackItem::ByteArray(ref y)) => {
                let x_bytes = x.borrow();
                let y_bytes = y.borrow();
                let x_int = Self::bytes_to_i64_le(&x_bytes);
                let y_int = Self::bytes_to_i64_le(&y_bytes);
                Ok(StackItem::Integer(x_int & y_int))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise AND".to_string(),
            }),
        }
    }

    fn bitwise_or(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x | y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x | y))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise OR".to_string(),
            }),
        }
    }

    fn bitwise_xor(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x ^ y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x ^ y))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise XOR".to_string(),
            }),
        }
    }

    fn shift_left(&self, value: StackItem, shift: StackItem) -> Result<StackItem, RuntimeError> {
        let amount = self.extract_shift_amount(shift)?;
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(v.wrapping_shl(amount))),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(v.wrapping_shl(amount))),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for shift left".to_string(),
            }),
        }
    }

    fn shift_right(&self, value: StackItem, shift: StackItem) -> Result<StackItem, RuntimeError> {
        let amount = self.extract_shift_amount(shift)?;
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(v.wrapping_shr(amount))),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(v.wrapping_shr(amount))),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for shift right".to_string(),
            }),
        }
    }

    fn extract_shift_amount(&self, item: StackItem) -> Result<u32, RuntimeError> {
        match item {
            StackItem::Integer(v) => {
                if v < 0 {
                    Err(RuntimeError::ExecutionError {
                        message: "Shift amount must be non-negative".to_string(),
                    })
                } else {
                    Ok((v as u64).min(63) as u32)
                }
            }
            StackItem::UnsignedInteger(v) => Ok((v.min(63)) as u32),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid shift amount".to_string(),
            }),
        }
    }

    /// Convert a byte array to i64 using little-endian encoding
    /// Empty arrays return 0, arrays longer than 8 bytes are truncated
    fn bytes_to_i64_le(bytes: &[u8]) -> i64 {
        if bytes.is_empty() {
            return 0;
        }
        let mut result: i64 = 0;
        for (i, &byte) in bytes.iter().take(8).enumerate() {
            result |= (byte as i64) << (i * 8);
        }
        // Sign extend if the original value was negative (high bit set in last byte)
        if bytes.len() <= 8 && !bytes.is_empty() {
            let last_byte = bytes[bytes.len().min(8) - 1];
            if last_byte & 0x80 != 0 && bytes.len() < 8 {
                // Sign extend
                for i in bytes.len()..8 {
                    result |= 0xFFi64 << (i * 8);
                }
            }
        }
        result
    }
}
