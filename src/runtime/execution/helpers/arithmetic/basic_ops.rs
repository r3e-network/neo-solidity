/// Arithmetic operation helpers with overflow/underflow checking.
///
/// All arithmetic operations follow the same pattern:
/// 1. Determine if unsigned arithmetic should be used
/// 2. Coerce both operands to appropriate type (i64/u64)
/// 3. Perform operation with overflow checking (if strict_arithmetic is enabled)
/// 4. Return result as StackItem
///
/// The macros below generate the repetitive boilerplate for each operation.
/// Macro to generate arithmetic operation functions (ADD, SUB, MUL).
///
/// # Syntax
/// ```ignore
/// arithmetic_op!(fn_name, op_name, op_sym, checked_fn, wrapping_fn, error_kind);
/// ```
///
/// # Example
/// ```ignore
/// arithmetic_op!(add_stack_items, "ADD", "+", checked_add, wrapping_add, "overflow");
/// ```
macro_rules! arithmetic_op {
    ($fn_name:ident, $op_name:literal, $op_sym:literal, $checked:ident, $wrapping:ident, $error_kind:literal) => {
        fn $fn_name(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
            let use_unsigned =
                matches!(a, StackItem::UnsignedInteger(_)) || matches!(b, StackItem::UnsignedInteger(_));

            if use_unsigned {
                let x = self.coerce_item_to_u64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                    message: concat!("Invalid operands for ", $op_name).to_string(),
                })?;
                let y = self.coerce_item_to_u64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                    message: concat!("Invalid operands for ", $op_name).to_string(),
                })?;
                if self.strict_arithmetic {
                    x.$checked(y)
                        .map(StackItem::UnsignedInteger)
                        .ok_or_else(|| RuntimeError::ExecutionError {
                            message: format!(
                                "Unsigned integer {} in {}: {} {} {}",
                                $error_kind, $op_name, x, $op_sym, y
                            ),
                        })
                } else {
                    Ok(StackItem::UnsignedInteger(x.$wrapping(y)))
                }
            } else {
                let x = self.coerce_item_to_i64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                    message: concat!("Invalid operands for ", $op_name).to_string(),
                })?;
                let y = self.coerce_item_to_i64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                    message: concat!("Invalid operands for ", $op_name).to_string(),
                })?;
                if self.strict_arithmetic {
                    x.$checked(y)
                        .map(StackItem::Integer)
                        .ok_or_else(|| RuntimeError::ExecutionError {
                            message: format!(
                                "Integer {} in {}: {} {} {}",
                                $error_kind, $op_name, x, $op_sym, y
                            ),
                        })
                } else {
                    Ok(StackItem::Integer(x.$wrapping(y)))
                }
            }
        }
    };
}

/// Macro to generate division/modulo operation functions (DIV, MOD).
///
/// Unlike basic arithmetic, these operations don't overflow but check for division by zero.
macro_rules! divmod_op {
    ($fn_name:ident, $op_name:literal, $op_fn:ident, $error_msg:literal) => {
        fn $fn_name(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
            let use_unsigned =
                matches!(a, StackItem::UnsignedInteger(_)) || matches!(b, StackItem::UnsignedInteger(_));

            if use_unsigned {
                let x = self.coerce_item_to_u64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                    message: concat!("Invalid operands for ", $op_name).to_string(),
                })?;
                let y = self.coerce_item_to_u64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                    message: concat!("Invalid operands for ", $op_name).to_string(),
                })?;
                if y == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: $error_msg.to_string(),
                    });
                }
                Ok(StackItem::UnsignedInteger(x.$op_fn(y)))
            } else {
                let x = self.coerce_item_to_i64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                    message: concat!("Invalid operands for ", $op_name).to_string(),
                })?;
                let y = self.coerce_item_to_i64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                    message: concat!("Invalid operands for ", $op_name).to_string(),
                })?;
                if y == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: $error_msg.to_string(),
                    });
                }
                Ok(StackItem::Integer(x.$op_fn(y)))
            }
        }
    };
}

impl ExecutionContext {
    fn coerce_item_to_i64(&self, item: &StackItem) -> Option<i64> {
        match item {
            StackItem::Integer(value) => Some(*value),
            StackItem::UnsignedInteger(value) => i64::try_from(*value).ok(),
            StackItem::Boolean(value) => Some(if *value { 1 } else { 0 }),
            StackItem::Null => Some(0),
            StackItem::ByteArray(bytes) => {
                let bytes = bytes.borrow();
                let mut buf = [0u8; 8];
                for (idx, byte) in bytes.iter().take(8).enumerate() {
                    buf[idx] = *byte;
                }
                Some(i64::from_le_bytes(buf))
            }
            _ => None,
        }
    }

    fn coerce_item_to_u64(&self, item: &StackItem) -> Option<u64> {
        match item {
            StackItem::UnsignedInteger(value) => Some(*value),
            StackItem::Integer(value) => {
                if *value < 0 {
                    None
                } else {
                    Some(*value as u64)
                }
            }
            StackItem::Boolean(value) => Some(if *value { 1 } else { 0 }),
            StackItem::Null => Some(0),
            StackItem::ByteArray(bytes) => {
                let bytes = bytes.borrow();
                let mut buf = [0u8; 8];
                for (idx, byte) in bytes.iter().take(8).enumerate() {
                    buf[idx] = *byte;
                }
                Some(u64::from_le_bytes(buf))
            }
            _ => None,
        }
    }

    // Generate all arithmetic operations using the macros
    arithmetic_op!(add_stack_items, "ADD", "+", checked_add, wrapping_add, "overflow");
    arithmetic_op!(sub_stack_items, "SUB", "-", checked_sub, wrapping_sub, "underflow");
    arithmetic_op!(mul_stack_items, "MUL", "*", checked_mul, wrapping_mul, "overflow");
    divmod_op!(div_stack_items, "DIV", div, "Division by zero");
    divmod_op!(mod_stack_items, "MOD", rem, "Modulo by zero");
}
