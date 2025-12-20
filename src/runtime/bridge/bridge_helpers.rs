fn extract_bytes(item: &StackItem) -> Result<Vec<u8>, VMBridgeError> {
    Ok(match item {
        StackItem::ByteArray(bytes) => bytes.borrow().clone(),
        StackItem::UnsignedInteger(value) => value.to_be_bytes().to_vec(),
        StackItem::Integer(value) => {
            if *value < 0 {
                return Err(VMBridgeError::StackOperationFailed {
                    message: "Negative integers not supported for byte extraction".to_string(),
                });
            }
            (*value as u64).to_be_bytes().to_vec()
        }
        StackItem::Array(_) => {
            return Err(VMBridgeError::StackOperationFailed {
                message: "Cannot extract bytes from array stack item".to_string(),
            })
        }
        StackItem::Map(_) => {
            return Err(VMBridgeError::StackOperationFailed {
                message: "Cannot extract bytes from map".to_string(),
            })
        }
        StackItem::Boolean(flag) => vec![if *flag { 1 } else { 0 }],
        StackItem::Null => Vec::new(),
    })
}

fn extract_integer(item: &StackItem) -> Result<u128, VMBridgeError> {
    Ok(match item {
        StackItem::UnsignedInteger(value) => *value as u128,
        StackItem::Integer(value) => {
            if *value < 0 {
                return Err(VMBridgeError::StackOperationFailed {
                    message: "Negative integers not supported here".to_string(),
                });
            }
            *value as u128
        }
        StackItem::ByteArray(bytes) => {
            let bytes = bytes.borrow();
            if bytes.is_empty() {
                0
            } else {
                let mut padded = [0u8; 16];
                let copy_len = bytes.len().min(16);
                padded[16 - copy_len..].copy_from_slice(&bytes[bytes.len() - copy_len..]);
                u128::from_be_bytes(padded)
            }
        }
        StackItem::Array(_) => {
            return Err(VMBridgeError::StackOperationFailed {
                message: "Arrays not supported for integer extraction".to_string(),
            });
        }
        StackItem::Map(_) => {
            return Err(VMBridgeError::StackOperationFailed {
                message: "Maps not supported for integer extraction".to_string(),
            });
        }
        StackItem::Boolean(flag) => {
            if *flag {
                1
            } else {
                0
            }
        }
        StackItem::Null => 0,
    })
}
