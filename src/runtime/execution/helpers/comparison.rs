impl ExecutionContext {
    fn stack_items_equal(&self, a: &StackItem, b: &StackItem) -> Result<bool, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(x == y),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => Ok(x == y),
            (StackItem::Boolean(x), StackItem::Boolean(y)) => Ok(x == y),
            (StackItem::ByteArray(x), StackItem::ByteArray(y)) => Ok(x == y),
            (StackItem::Array(x), StackItem::Array(y)) => Ok(x == y),
            (StackItem::Null, StackItem::Null) => Ok(true),
            // Cross-type comparisons
            (StackItem::Integer(x), StackItem::UnsignedInteger(y)) => {
                if *x < 0 {
                    Ok(false)
                } else {
                    Ok(*x as u64 == *y)
                }
            }
            (StackItem::UnsignedInteger(x), StackItem::Integer(y)) => {
                if *y < 0 {
                    Ok(false)
                } else {
                    Ok(*x == *y as u64)
                }
            }
            _ => Ok(false),
        }
    }

    fn less_than(&self, a: &StackItem, b: &StackItem) -> Result<bool, RuntimeError> {
        let use_unsigned =
            matches!(a, StackItem::UnsignedInteger(_)) || matches!(b, StackItem::UnsignedInteger(_));

        if use_unsigned {
            let x = self.coerce_item_to_u64(a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for comparison".to_string(),
            })?;
            let y = self.coerce_item_to_u64(b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for comparison".to_string(),
            })?;
            Ok(x < y)
        } else {
            let x = self.coerce_item_to_i64(a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for comparison".to_string(),
            })?;
            let y = self.coerce_item_to_i64(b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for comparison".to_string(),
            })?;
            Ok(x < y)
        }
    }

    fn greater_than(&self, a: &StackItem, b: &StackItem) -> Result<bool, RuntimeError> {
        let use_unsigned =
            matches!(a, StackItem::UnsignedInteger(_)) || matches!(b, StackItem::UnsignedInteger(_));

        if use_unsigned {
            let x = self.coerce_item_to_u64(a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for comparison".to_string(),
            })?;
            let y = self.coerce_item_to_u64(b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for comparison".to_string(),
            })?;
            Ok(x > y)
        } else {
            let x = self.coerce_item_to_i64(a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for comparison".to_string(),
            })?;
            let y = self.coerce_item_to_i64(b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for comparison".to_string(),
            })?;
            Ok(x > y)
        }
    }
}
