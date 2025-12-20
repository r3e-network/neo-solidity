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

    fn add_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        let use_unsigned =
            matches!(a, StackItem::UnsignedInteger(_)) || matches!(b, StackItem::UnsignedInteger(_));

        if use_unsigned {
            let x = self.coerce_item_to_u64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for ADD".to_string(),
            })?;
            let y = self.coerce_item_to_u64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for ADD".to_string(),
            })?;
            if self.strict_arithmetic {
                x.checked_add(y)
                    .map(StackItem::UnsignedInteger)
                    .ok_or_else(|| RuntimeError::ExecutionError {
                        message: format!("Unsigned integer overflow in ADD: {} + {}", x, y),
                    })
            } else {
                Ok(StackItem::UnsignedInteger(x.wrapping_add(y)))
            }
        } else {
            let x = self.coerce_item_to_i64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for ADD".to_string(),
            })?;
            let y = self.coerce_item_to_i64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for ADD".to_string(),
            })?;
            if self.strict_arithmetic {
                x.checked_add(y).map(StackItem::Integer).ok_or_else(|| {
                    RuntimeError::ExecutionError {
                        message: format!("Integer overflow in ADD: {} + {}", x, y),
                    }
                })
            } else {
                Ok(StackItem::Integer(x.wrapping_add(y)))
            }
        }
    }

    fn sub_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        let use_unsigned =
            matches!(a, StackItem::UnsignedInteger(_)) || matches!(b, StackItem::UnsignedInteger(_));

        if use_unsigned {
            let x = self.coerce_item_to_u64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for SUB".to_string(),
            })?;
            let y = self.coerce_item_to_u64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for SUB".to_string(),
            })?;
            if self.strict_arithmetic {
                x.checked_sub(y)
                    .map(StackItem::UnsignedInteger)
                    .ok_or_else(|| RuntimeError::ExecutionError {
                        message: format!("Unsigned integer underflow in SUB: {} - {}", x, y),
                    })
            } else {
                Ok(StackItem::UnsignedInteger(x.wrapping_sub(y)))
            }
        } else {
            let x = self.coerce_item_to_i64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for SUB".to_string(),
            })?;
            let y = self.coerce_item_to_i64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for SUB".to_string(),
            })?;
            if self.strict_arithmetic {
                x.checked_sub(y).map(StackItem::Integer).ok_or_else(|| {
                    RuntimeError::ExecutionError {
                        message: format!("Integer underflow in SUB: {} - {}", x, y),
                    }
                })
            } else {
                Ok(StackItem::Integer(x.wrapping_sub(y)))
            }
        }
    }

    fn mul_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        let use_unsigned =
            matches!(a, StackItem::UnsignedInteger(_)) || matches!(b, StackItem::UnsignedInteger(_));

        if use_unsigned {
            let x = self.coerce_item_to_u64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for MUL".to_string(),
            })?;
            let y = self.coerce_item_to_u64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for MUL".to_string(),
            })?;
            if self.strict_arithmetic {
                x.checked_mul(y)
                    .map(StackItem::UnsignedInteger)
                    .ok_or_else(|| RuntimeError::ExecutionError {
                        message: format!("Unsigned integer overflow in MUL: {} * {}", x, y),
                    })
            } else {
                Ok(StackItem::UnsignedInteger(x.wrapping_mul(y)))
            }
        } else {
            let x = self.coerce_item_to_i64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for MUL".to_string(),
            })?;
            let y = self.coerce_item_to_i64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for MUL".to_string(),
            })?;
            if self.strict_arithmetic {
                x.checked_mul(y).map(StackItem::Integer).ok_or_else(|| {
                    RuntimeError::ExecutionError {
                        message: format!("Integer overflow in MUL: {} * {}", x, y),
                    }
                })
            } else {
                Ok(StackItem::Integer(x.wrapping_mul(y)))
            }
        }
    }

    fn div_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        let use_unsigned =
            matches!(a, StackItem::UnsignedInteger(_)) || matches!(b, StackItem::UnsignedInteger(_));

        if use_unsigned {
            let x = self.coerce_item_to_u64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for DIV".to_string(),
            })?;
            let y = self.coerce_item_to_u64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for DIV".to_string(),
            })?;
            if y == 0 {
                return Err(RuntimeError::ExecutionError {
                    message: "Division by zero".to_string(),
                });
            }
            Ok(StackItem::UnsignedInteger(x / y))
        } else {
            let x = self.coerce_item_to_i64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for DIV".to_string(),
            })?;
            let y = self.coerce_item_to_i64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for DIV".to_string(),
            })?;
            if y == 0 {
                return Err(RuntimeError::ExecutionError {
                    message: "Division by zero".to_string(),
                });
            }
            Ok(StackItem::Integer(x / y))
        }
    }

    fn mod_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        let use_unsigned =
            matches!(a, StackItem::UnsignedInteger(_)) || matches!(b, StackItem::UnsignedInteger(_));

        if use_unsigned {
            let x = self.coerce_item_to_u64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for MOD".to_string(),
            })?;
            let y = self.coerce_item_to_u64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for MOD".to_string(),
            })?;
            if y == 0 {
                return Err(RuntimeError::ExecutionError {
                    message: "Modulo by zero".to_string(),
                });
            }
            Ok(StackItem::UnsignedInteger(x % y))
        } else {
            let x = self.coerce_item_to_i64(&a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for MOD".to_string(),
            })?;
            let y = self.coerce_item_to_i64(&b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for MOD".to_string(),
            })?;
            if y == 0 {
                return Err(RuntimeError::ExecutionError {
                    message: "Modulo by zero".to_string(),
                });
            }
            Ok(StackItem::Integer(x % y))
        }
    }
}
