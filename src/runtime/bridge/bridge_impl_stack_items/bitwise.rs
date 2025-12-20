impl VMBridge {
    fn and_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x & y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x & y))
            }
            _ => Err(VMBridgeError::BridgeError {
                message: "Invalid operands for AND".to_string(),
            }),
        }
    }

    fn or_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x | y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x | y))
            }
            _ => Err(VMBridgeError::BridgeError {
                message: "Invalid operands for OR".to_string(),
            }),
        }
    }

    fn xor_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x ^ y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x ^ y))
            }
            _ => Err(VMBridgeError::BridgeError {
                message: "Invalid operands for XOR".to_string(),
            }),
        }
    }
}
