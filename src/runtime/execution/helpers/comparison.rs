use super::*;

impl ExecutionContext {
    pub(crate) fn stack_items_equal(
        &self,
        a: &StackItem,
        b: &StackItem,
    ) -> Result<bool, RuntimeError> {
        // Task #30 slice 1 Part C: for wide ByteArray operands (e.g. uint256 values
        // pushed via PUSHINT256/PUSHDATA1), route through BigInt so the overflow
        // guards emitted by the compiler compare at full 256-bit width instead of
        // truncating to the low 8 bytes.
        if self.cmp_needs_bigint_path(a, b) {
            let x = self
                .coerce_item_to_bigint(a)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            let y = self
                .coerce_item_to_bigint(b)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            return Ok(x == y);
        }
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

    pub(crate) fn less_than(&self, a: &StackItem, b: &StackItem) -> Result<bool, RuntimeError> {
        // Task #30 slice 1 Part C: see stack_items_equal.
        if self.cmp_needs_bigint_path(a, b) {
            let x = self
                .coerce_item_to_bigint(a)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            let y = self
                .coerce_item_to_bigint(b)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            return Ok(x < y);
        }

        let use_unsigned = matches!(a, StackItem::UnsignedInteger(_))
            || matches!(b, StackItem::UnsignedInteger(_));

        if use_unsigned {
            let x = self
                .coerce_item_to_u64(a)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            let y = self
                .coerce_item_to_u64(b)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            Ok(x < y)
        } else {
            let x = self
                .coerce_item_to_i64(a)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            let y = self
                .coerce_item_to_i64(b)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            Ok(x < y)
        }
    }

    pub(crate) fn greater_than(&self, a: &StackItem, b: &StackItem) -> Result<bool, RuntimeError> {
        // Task #30 slice 1 Part C: see stack_items_equal.
        if self.cmp_needs_bigint_path(a, b) {
            let x = self
                .coerce_item_to_bigint(a)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            let y = self
                .coerce_item_to_bigint(b)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            return Ok(x > y);
        }

        let use_unsigned = matches!(a, StackItem::UnsignedInteger(_))
            || matches!(b, StackItem::UnsignedInteger(_));

        if use_unsigned {
            let x = self
                .coerce_item_to_u64(a)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            let y = self
                .coerce_item_to_u64(b)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            Ok(x > y)
        } else {
            let x = self
                .coerce_item_to_i64(a)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            let y = self
                .coerce_item_to_i64(b)
                .ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for comparison".to_string(),
                })?;
            Ok(x > y)
        }
    }
}
