use super::*;

impl VMBridge {
    pub(crate) fn lt_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        let result = match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => x < y,
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => x < y,
            _ => false,
        };
        Ok(StackItem::Boolean(result))
    }

    pub(crate) fn gt_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        let result = match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => x > y,
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => x > y,
            _ => false,
        };
        Ok(StackItem::Boolean(result))
    }

    pub(crate) fn eq_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        let result = match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => x == y,
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => x == y,
            (StackItem::Boolean(x), StackItem::Boolean(y)) => x == y,
            (StackItem::ByteArray { data: x, type_tag: tx }, StackItem::ByteArray { data: y, type_tag: ty }) => tx == ty && x == y,
            (StackItem::Null, StackItem::Null) => true,
            _ => false,
        };
        Ok(StackItem::Boolean(result))
    }
}
