impl VMBridge {
    fn shift_left_value(value: StackItem, shift: StackItem) -> Result<StackItem, VMBridgeError> {
        let amount = Self::extract_shift_amount(shift)?;
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(v.wrapping_shl(amount))),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(v.wrapping_shl(amount))),
            _ => Err(VMBridgeError::BridgeError {
                message: "Invalid operands for SHL".to_string(),
            }),
        }
    }

    fn shift_right_value(value: StackItem, shift: StackItem) -> Result<StackItem, VMBridgeError> {
        let amount = Self::extract_shift_amount(shift)?;
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(v.wrapping_shr(amount))),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(v.wrapping_shr(amount))),
            _ => Err(VMBridgeError::BridgeError {
                message: "Invalid operands for SHR".to_string(),
            }),
        }
    }

    fn extract_shift_amount(item: StackItem) -> Result<u32, VMBridgeError> {
        match item {
            StackItem::Integer(v) => {
                if v < 0 {
                    Err(VMBridgeError::BridgeError {
                        message: "Shift amount must be non-negative".to_string(),
                    })
                } else {
                    Ok((v as u64).min(63) as u32)
                }
            }
            StackItem::UnsignedInteger(v) => Ok((v.min(63)) as u32),
            _ => Err(VMBridgeError::BridgeError {
                message: "Invalid shift amount".to_string(),
            }),
        }
    }
}
