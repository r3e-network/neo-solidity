impl VMBridge {
    fn lt_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        let result = match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => x < y,
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => x < y,
            _ => false,
        };
        Ok(StackItem::Boolean(result))
    }

    fn gt_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        let result = match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => x > y,
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => x > y,
            _ => false,
        };
        Ok(StackItem::Boolean(result))
    }

    fn eq_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        let result = match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => x == y,
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => x == y,
            (StackItem::Boolean(x), StackItem::Boolean(y)) => x == y,
            (StackItem::ByteArray(x), StackItem::ByteArray(y)) => x == y,
            (StackItem::Null, StackItem::Null) => true,
            _ => false,
        };
        Ok(StackItem::Boolean(result))
    }
}
