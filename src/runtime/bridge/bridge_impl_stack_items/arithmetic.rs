impl VMBridge {
    // Arithmetic operations on stack items

    fn add_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_add(y)))
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_add(y)))
            }
            (StackItem::Integer(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::Integer(x.wrapping_add(y as i64)))
            }
            (StackItem::UnsignedInteger(x), StackItem::Integer(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_add(y as u64)))
            }
            _ => Err(VMBridgeError::StackOperationFailed {
                message: "Invalid operands for ADD".to_string(),
            }),
        }
    }

    fn sub_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_sub(y)))
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_sub(y)))
            }
            _ => Err(VMBridgeError::StackOperationFailed {
                message: "Invalid operands for SUB".to_string(),
            }),
        }
    }

    fn mul_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_mul(y)))
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_mul(y)))
            }
            _ => Err(VMBridgeError::StackOperationFailed {
                message: "Invalid operands for MUL".to_string(),
            }),
        }
    }

    fn div_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                if y == 0 {
                    Ok(StackItem::Integer(0)) // Division by zero returns 0 in EVM
                } else {
                    Ok(StackItem::Integer(x / y))
                }
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                if y == 0 {
                    Ok(StackItem::UnsignedInteger(0))
                } else {
                    Ok(StackItem::UnsignedInteger(x / y))
                }
            }
            _ => Err(VMBridgeError::StackOperationFailed {
                message: "Invalid operands for DIV".to_string(),
            }),
        }
    }

    fn mod_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                if y == 0 {
                    Ok(StackItem::Integer(0))
                } else {
                    Ok(StackItem::Integer(x % y))
                }
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                if y == 0 {
                    Ok(StackItem::UnsignedInteger(0))
                } else {
                    Ok(StackItem::UnsignedInteger(x % y))
                }
            }
            _ => Err(VMBridgeError::StackOperationFailed {
                message: "Invalid operands for MOD".to_string(),
            }),
        }
    }

    fn modmul_stack_items(
        a: StackItem,
        b: StackItem,
        modulus: StackItem,
    ) -> Result<StackItem, VMBridgeError> {
        let (a_int, b_int, m_int) = match (a, b, modulus) {
            (
                StackItem::UnsignedInteger(x),
                StackItem::UnsignedInteger(y),
                StackItem::UnsignedInteger(m),
            ) => (x as u128, y as u128, m as u128),
            (StackItem::Integer(x), StackItem::Integer(y), StackItem::Integer(m)) => {
                if x < 0 || y < 0 || m <= 0 {
                    return Err(VMBridgeError::StackOperationFailed {
                        message: "MODMUL expects non-negative operands".to_string(),
                    });
                }
                (x as u128, y as u128, m as u128)
            }
            _ => {
                return Err(VMBridgeError::StackOperationFailed {
                    message: "Invalid operands for MODMUL".to_string(),
                })
            }
        };

        if m_int == 0 {
            return Ok(StackItem::UnsignedInteger(0));
        }
        let result = ((a_int % m_int) * (b_int % m_int)) % m_int;
        Ok(StackItem::UnsignedInteger(result as u64))
    }

    fn modpow_stack_items(
        base: StackItem,
        exponent: StackItem,
        modulus: StackItem,
    ) -> Result<StackItem, VMBridgeError> {
        let (mut base, exp, modulus) = match (base, exponent, modulus) {
            (
                StackItem::UnsignedInteger(b),
                StackItem::UnsignedInteger(e),
                StackItem::UnsignedInteger(m),
            ) => (b as u128, e as u128, m as u128),
            (StackItem::Integer(b), StackItem::Integer(e), StackItem::Integer(m)) => {
                if b < 0 || e < 0 || m <= 0 {
                    return Err(VMBridgeError::StackOperationFailed {
                        message: "MODPOW expects non-negative operands".to_string(),
                    });
                }
                (b as u128, e as u128, m as u128)
            }
            _ => {
                return Err(VMBridgeError::StackOperationFailed {
                    message: "Invalid operands for MODPOW".to_string(),
                })
            }
        };

        if modulus == 0 {
            return Ok(StackItem::UnsignedInteger(0));
        }

        base %= modulus;
        let mut result: u128 = 1;
        let mut exp_mut = exp;
        while exp_mut > 0 {
            if exp_mut & 1 == 1 {
                result = (result * base) % modulus;
            }
            base = (base * base) % modulus;
            exp_mut >>= 1;
        }

        Ok(StackItem::UnsignedInteger(result as u64))
    }
}
