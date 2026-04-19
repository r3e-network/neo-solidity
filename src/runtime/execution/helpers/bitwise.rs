impl ExecutionContext {
    /// Task #50: mask a BigInt bitwise result to 256 bits and encode as a
    /// fixed-width (up to 32-byte) little-endian ByteArray. Returns narrow
    /// Integer for values that fit in i64 to preserve existing narrow
    /// comparison paths; wider values return a signed-LE payload so that
    /// subsequent decode via `coerce_item_to_bigint` (which uses
    /// `BigInt::from_signed_bytes_le`) round-trips correctly. Without the
    /// sign-padding byte, a positive magnitude whose MSB byte has its high
    /// bit set (e.g. `0x80 * 2^240` has magnitude-MSB `0x80`) would decode
    /// as a negative value — the Task #118 fuzz harness
    /// `batch51_aa4_bytes_to_bytes32_bitwise_shl_or_assembly` hits this case
    /// when the input byte is `>= 0x80` and the shift places it at bit
    /// position ≡ 7 (mod 8).
    fn u256_bigint_to_stack_item(value: num_bigint::BigInt) -> StackItem {
        use num_bigint::{BigInt, Sign};
        let one: BigInt = BigInt::from(1);
        let mask: BigInt = (one << 256u32) - BigInt::from(1);
        let masked: BigInt = value & mask;
        // Narrow-path: fits in i64 → return Integer to preserve existing
        // narrow comparison semantics.
        if let Ok(n) = i64::try_from(masked.clone()) {
            return StackItem::Integer(n);
        }
        // Wide path: `masked` is non-negative after masking. Emit MAGNITUDE
        // bytes and append a `0x00` sign-extension byte iff the MSB has its
        // high bit set, so `from_signed_bytes_le` decodes it as the positive
        // magnitude rather than a sign-extended negative. Task #118.
        let (sign, mut bytes) = masked.to_bytes_le();
        debug_assert!(sign != Sign::Minus, "mask produced non-negative value");
        if bytes.last().is_some_and(|b| b & 0x80 != 0) && bytes.len() < 32 {
            bytes.push(0x00);
        }
        if bytes.len() > 32 {
            bytes.truncate(32);
        }
        StackItem::byte_array(bytes)
    }

    fn bitwise_not(&self, value: StackItem) -> Result<StackItem, RuntimeError> {
        // Task #50: narrow scalars preserve existing i64/u64 semantics; wide
        // ByteArray operands route through BigInt and mask to 256 bits so
        // `~uint256(x)` returns `u256::MAX - x` instead of `!(x as u64)`.
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(!v)),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(!v)),
            StackItem::ByteArray(_) => {
                let x = self.coerce_item_to_bigint(&value).ok_or_else(|| {
                    RuntimeError::ExecutionError {
                        message: "Invalid operand for bitwise NOT".to_string(),
                    }
                })?;
                Ok(Self::u256_bigint_to_stack_item(!x))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operand for bitwise NOT".to_string(),
            }),
        }
    }

    fn bitwise_and(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        // Task #50: wide ByteArray pair routes through BigInt; other cases
        // keep their pre-existing narrow semantics for compat with the 137
        // passing fuzz tests.
        if self.cmp_needs_bigint_path(&a, &b) {
            let x = self.coerce_item_to_bigint(&a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise AND".to_string(),
            })?;
            let y = self.coerce_item_to_bigint(&b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise AND".to_string(),
            })?;
            return Ok(Self::u256_bigint_to_stack_item(x & y));
        }
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x & y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x & y))
            }
            // Task #170: mixed signed/unsigned narrow-integer arms. This case
            // arises when a `uint32(block.number)` cast (emits `value & 0xFFFFFFFF`)
            // runs against a runtime-sourced `UnsignedInteger` (block.number)
            // with a compiler-pushed `Integer` mask — without these arms the
            // packed-struct push (mapping → dynamic-array of struct) faults
            // with "Invalid operands for bitwise AND" mid-push. `i64 as u64`
            // preserves the bit pattern (two's complement view), which is the
            // correct semantics for a bitwise operation.
            (StackItem::UnsignedInteger(x), StackItem::Integer(y)) => {
                Ok(StackItem::UnsignedInteger(x & (y as u64)))
            }
            (StackItem::Integer(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger((x as u64) & y))
            }
            // Handle Boolean types - convert to integers for bitwise operations
            (StackItem::Boolean(x), StackItem::Boolean(y)) => {
                Ok(StackItem::Boolean(x && y))
            }
            (StackItem::Boolean(x), StackItem::Integer(y)) => {
                let x_int = if x { 1i64 } else { 0i64 };
                Ok(StackItem::Integer(x_int & y))
            }
            (StackItem::Integer(x), StackItem::Boolean(y)) => {
                let y_int = if y { 1i64 } else { 0i64 };
                Ok(StackItem::Integer(x & y_int))
            }
            (StackItem::Boolean(x), StackItem::UnsignedInteger(y)) => {
                let x_int = if x { 1u64 } else { 0u64 };
                Ok(StackItem::UnsignedInteger(x_int & y))
            }
            (StackItem::UnsignedInteger(x), StackItem::Boolean(y)) => {
                let y_int = if y { 1u64 } else { 0u64 };
                Ok(StackItem::UnsignedInteger(x & y_int))
            }
            // Handle Null as zero/false
            (StackItem::Null, StackItem::Integer(_)) => Ok(StackItem::Integer(0)),
            (StackItem::Integer(_), StackItem::Null) => Ok(StackItem::Integer(0)),
            (StackItem::Null, StackItem::UnsignedInteger(_)) => Ok(StackItem::UnsignedInteger(0)),
            (StackItem::UnsignedInteger(_), StackItem::Null) => Ok(StackItem::UnsignedInteger(0)),
            (StackItem::Null, StackItem::Boolean(_)) => Ok(StackItem::Boolean(false)),
            (StackItem::Boolean(_), StackItem::Null) => Ok(StackItem::Boolean(false)),
            (StackItem::Null, StackItem::Null) => Ok(StackItem::Integer(0)),
            // Handle ByteArray - convert to integer (little-endian)
            (StackItem::ByteArray(ref x), StackItem::Integer(y)) => {
                let x_bytes = x.borrow();
                let x_int = Self::bytes_to_i64_le(&x_bytes);
                Ok(StackItem::Integer(x_int & y))
            }
            (StackItem::Integer(x), StackItem::ByteArray(ref y)) => {
                let y_bytes = y.borrow();
                let y_int = Self::bytes_to_i64_le(&y_bytes);
                Ok(StackItem::Integer(x & y_int))
            }
            // Task #170: narrow-ByteArray × UnsignedInteger symmetric arms.
            // Mirrors the (ByteArray, Integer) / (Integer, ByteArray) pair
            // above — required when a mask is staged as a UnsignedInteger
            // (e.g. when it matches a PUSHINT that landed in the unsigned
            // narrow path) alongside a narrow serialized-slot read.
            (StackItem::ByteArray(ref x), StackItem::UnsignedInteger(y)) => {
                let x_bytes = x.borrow();
                let x_u = Self::bytes_to_i64_le(&x_bytes) as u64;
                Ok(StackItem::UnsignedInteger(x_u & y))
            }
            (StackItem::UnsignedInteger(x), StackItem::ByteArray(ref y)) => {
                let y_bytes = y.borrow();
                let y_u = Self::bytes_to_i64_le(&y_bytes) as u64;
                Ok(StackItem::UnsignedInteger(x & y_u))
            }
            (StackItem::ByteArray(ref x), StackItem::ByteArray(ref y)) => {
                let x_bytes = x.borrow();
                let y_bytes = y.borrow();
                let x_int = Self::bytes_to_i64_le(&x_bytes);
                let y_int = Self::bytes_to_i64_le(&y_bytes);
                Ok(StackItem::Integer(x_int & y_int))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise AND".to_string(),
            }),
        }
    }

    fn bitwise_or(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        // Task #50: accept ByteArray operands by routing through BigInt with a
        // 256-bit mask, so `uint256(2^63) | uint256(1)` no longer panics.
        if matches!(a, StackItem::ByteArray(_)) || matches!(b, StackItem::ByteArray(_)) {
            let x = self.coerce_item_to_bigint(&a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise OR".to_string(),
            })?;
            let y = self.coerce_item_to_bigint(&b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise OR".to_string(),
            })?;
            return Ok(Self::u256_bigint_to_stack_item(x | y));
        }
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x | y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x | y))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise OR".to_string(),
            }),
        }
    }

    fn bitwise_xor(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        // Task #50: accept ByteArray operands via the BigInt path with a
        // 256-bit mask, matching the new OR/NOT semantics.
        if matches!(a, StackItem::ByteArray(_)) || matches!(b, StackItem::ByteArray(_)) {
            let x = self.coerce_item_to_bigint(&a).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise XOR".to_string(),
            })?;
            let y = self.coerce_item_to_bigint(&b).ok_or_else(|| RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise XOR".to_string(),
            })?;
            return Ok(Self::u256_bigint_to_stack_item(x ^ y));
        }
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x ^ y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x ^ y))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise XOR".to_string(),
            }),
        }
    }

    /// True when the LHS is a wide ByteArray (>8 bytes) that would otherwise
    /// be truncated by the narrow i64/u64 shift path. Task #H4.
    fn shift_lhs_is_wide(item: &StackItem) -> bool {
        matches!(item, StackItem::ByteArray(bytes) if bytes.borrow().len() > 8)
    }

    fn shift_left(&self, value: StackItem, shift: StackItem) -> Result<StackItem, RuntimeError> {
        let amount = self.extract_shift_amount(shift)?;
        // Task #H4: route through BigInt when either (a) LHS is a wide
        // ByteArray (would truncate to low 8 bytes), or (b) amount >= 64
        // (result needs more than 64 bits even for narrow 1-bit LHS like
        // `uint256(1) << 128`). Reuses the Task #50 BigInt infrastructure
        // so results wider than i64 round-trip as ByteArray (u256 LE) and
        // `decode_uint_le` on return reads back the full value.
        if amount >= 256 {
            // EIP-145: shift >= operand width silently produces 0.
            return Ok(Self::u256_bigint_to_stack_item(num_bigint::BigInt::from(0)));
        }
        if Self::shift_lhs_is_wide(&value) || amount >= 64 {
            let x =
                self.coerce_item_to_bigint(&value).ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for shift left".to_string(),
                })?;
            return Ok(Self::u256_bigint_to_stack_item(x << amount));
        }
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(v.wrapping_shl(amount))),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(v.wrapping_shl(amount))),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for shift left".to_string(),
            }),
        }
    }

    fn shift_right(&self, value: StackItem, shift: StackItem) -> Result<StackItem, RuntimeError> {
        let amount = self.extract_shift_amount(shift)?;
        // Task #H4: route through BigInt when LHS is a wide ByteArray so
        // `uint256 x = (1 << 65) - 1; x >> 64` produces 1 instead of
        // faulting. Narrow scalars with `amount >= 64` already correctly
        // produce 0 for i64 (all bits shifted out), so keep the fast path.
        if amount >= 256 {
            return Ok(Self::u256_bigint_to_stack_item(num_bigint::BigInt::from(0)));
        }
        if Self::shift_lhs_is_wide(&value) {
            let x =
                self.coerce_item_to_bigint(&value).ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Invalid operands for shift right".to_string(),
                })?;
            return Ok(Self::u256_bigint_to_stack_item(x >> amount));
        }
        if amount >= 64 {
            return match value {
                StackItem::Integer(_) => Ok(StackItem::Integer(0)),
                StackItem::UnsignedInteger(_) => Ok(StackItem::UnsignedInteger(0)),
                _ => Err(RuntimeError::ExecutionError {
                    message: "Invalid operands for shift right".to_string(),
                }),
            };
        }
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(v.wrapping_shr(amount))),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(v.wrapping_shr(amount))),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for shift right".to_string(),
            }),
        }
    }

    fn extract_shift_amount(&self, item: StackItem) -> Result<u32, RuntimeError> {
        // Per EIP-145 (adopted by Solidity 0.8.x), a shift amount >= the
        // operand's bit-width silently produces 0 rather than faulting.
        // Neo-Solidity's widest scalar is uint256, so we clamp any
        // out-of-range non-negative shift amount to 256. Callers
        // (`shift_left`/`shift_right`) already short-circuit to 0 once the
        // amount exceeds their operand width, so the sentinel flows through
        // unchanged. Task #33.
        match item {
            StackItem::Integer(v) => {
                if v < 0 {
                    Err(RuntimeError::ExecutionError {
                        message: "Shift amount must be non-negative".to_string(),
                    })
                } else if v > 255 {
                    Ok(256)
                } else {
                    Ok(v as u32)
                }
            }
            StackItem::UnsignedInteger(v) => {
                if v > 255 {
                    Ok(256)
                } else {
                    Ok(v as u32)
                }
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid shift amount".to_string(),
            }),
        }
    }

    /// Convert a byte array to i64 using little-endian encoding
    /// Empty arrays return 0, arrays longer than 8 bytes are truncated
    fn bytes_to_i64_le(bytes: &[u8]) -> i64 {
        if bytes.is_empty() {
            return 0;
        }
        let mut result: i64 = 0;
        for (i, &byte) in bytes.iter().take(8).enumerate() {
            result |= (byte as i64) << (i * 8);
        }
        // Sign extend if the original value was negative (high bit set in last byte)
        if bytes.len() <= 8 && !bytes.is_empty() {
            let last_byte = bytes[bytes.len().min(8) - 1];
            if last_byte & 0x80 != 0 && bytes.len() < 8 {
                // Sign extend
                for i in bytes.len()..8 {
                    result |= 0xFFi64 << (i * 8);
                }
            }
        }
        result
    }
}
