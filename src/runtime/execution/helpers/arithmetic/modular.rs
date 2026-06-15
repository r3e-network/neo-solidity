use super::*;

impl ExecutionContext {
    pub(crate) fn modmul_stack_items(
        &self,
        a: StackItem,
        b: StackItem,
        modulus: StackItem,
    ) -> Result<StackItem, RuntimeError> {
        match (a, b, modulus) {
            (StackItem::Integer(x), StackItem::Integer(y), StackItem::Integer(m)) => {
                if m == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "MODMUL modulus cannot be zero".to_string(),
                    });
                }
                // NeoVM MODMUL is `x1 * x2 % modulus` (C# `%`), the TRUNCATED
                // remainder whose sign follows the product — not the Euclidean
                // (always-non-negative) remainder. Use `%` so the signed path
                // matches real NeoVM for negative products.
                let modulus = m.abs() as i128;
                let product = (x as i128).wrapping_mul(y as i128);
                let result = product % modulus;
                Ok(StackItem::Integer(result as i64))
            }
            (
                StackItem::UnsignedInteger(x),
                StackItem::UnsignedInteger(y),
                StackItem::UnsignedInteger(m),
            ) => {
                if m == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "MODMUL modulus cannot be zero".to_string(),
                    });
                }
                let product = (x as u128).wrapping_mul(y as u128);
                let result = product % (m as u128);
                Ok(StackItem::UnsignedInteger(result as u64))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for MODMUL".to_string(),
            }),
        }
    }

    pub(crate) fn modpow_stack_items(
        &self,
        base: StackItem,
        exponent: StackItem,
        modulus: StackItem,
    ) -> Result<StackItem, RuntimeError> {
        match (base, exponent, modulus) {
            (StackItem::Integer(b), StackItem::Integer(e), StackItem::Integer(m)) => {
                if m == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "MODPOW modulus cannot be zero".to_string(),
                    });
                }
                if e < 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "MODPOW exponent must be non-negative".to_string(),
                    });
                }
                // NeoVM MODPOW mirrors C# `BigInteger.ModPow`, which uses
                // truncated (`%`) remainders throughout — the result's sign
                // follows `base^exp`, not the Euclidean non-negative form.
                let modulus = m.abs() as i128;
                let mut result: i128 = 1 % modulus;
                let mut base = (b as i128) % modulus;
                let mut exp = e as u128;
                while exp > 0 {
                    if exp & 1 == 1 {
                        result = (result * base) % modulus;
                    }
                    base = (base * base) % modulus;
                    exp >>= 1;
                }
                Ok(StackItem::Integer(result as i64))
            }
            (
                StackItem::UnsignedInteger(b),
                StackItem::UnsignedInteger(e),
                StackItem::UnsignedInteger(m),
            ) => {
                if m == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "MODPOW modulus cannot be zero".to_string(),
                    });
                }
                let mut result: u128 = 1 % m as u128;
                let mut base = (b as u128) % (m as u128);
                let mut exp = e as u128;
                while exp > 0 {
                    if exp & 1 == 1 {
                        result = (result * base) % (m as u128);
                    }
                    base = (base * base) % (m as u128);
                    exp >>= 1;
                }
                Ok(StackItem::UnsignedInteger(result as u64))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for MODPOW".to_string(),
            }),
        }
    }
}
