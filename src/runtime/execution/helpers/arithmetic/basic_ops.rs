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
/// ```text
/// arithmetic_op!(fn_name, op_name, op_sym, checked_fn, wrapping_fn, error_kind);
/// ```
///
/// # Example
/// ```text
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
/// Checks for division by zero AND the signed overflow case `i64::MIN / -1`
/// (which panics in Rust debug mode and wraps in release mode).
macro_rules! divmod_op {
    ($fn_name:ident, $op_name:literal, $checked_fn:ident, $error_msg:literal) => {
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
                x.$checked_fn(y)
                    .map(StackItem::UnsignedInteger)
                    .ok_or_else(|| RuntimeError::ExecutionError {
                        message: format!(
                            "Unsigned integer overflow in {}: {} op {}",
                            $op_name, x, y
                        ),
                    })
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
                x.$checked_fn(y)
                    .map(StackItem::Integer)
                    .ok_or_else(|| RuntimeError::ExecutionError {
                        message: format!(
                            "Signed integer overflow in {}: {} / -1",
                            $op_name, x
                        ),
                    })
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
                if bytes.is_empty() {
                    return Some(0);
                }
                // NeoVM integers are arbitrary-precision little-endian byte arrays.
                // This runtime uses i64 internally; truncation to the low 8 bytes
                // is intentional — the compiler emits masking ops (AND, SHL, etc.)
                // to handle width reduction at the Solidity level.
                let len = bytes.len().min(8);
                let mut buf = [0u8; 8];
                buf[..len].copy_from_slice(&bytes[..len]);
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
                if bytes.is_empty() {
                    return Some(0);
                }
                // See coerce_item_to_i64 comment — truncation to low 8 bytes
                // is the correct NeoVM runtime behavior.
                let len = bytes.len().min(8);
                let mut buf = [0u8; 8];
                buf[..len].copy_from_slice(&bytes[..len]);
                Some(u64::from_le_bytes(buf))
            }
            _ => None,
        }
    }

    // Generate all arithmetic operations using the macros
    arithmetic_op!(add_stack_items, "ADD", "+", checked_add, wrapping_add, "overflow");
    arithmetic_op!(sub_stack_items, "SUB", "-", checked_sub, wrapping_sub, "underflow");
    arithmetic_op!(mul_stack_items, "MUL", "*", checked_mul, wrapping_mul, "overflow");
    divmod_op!(div_stack_items, "DIV", checked_div, "Division by zero");
    divmod_op!(mod_stack_items, "MOD", checked_rem, "Modulo by zero");
}
