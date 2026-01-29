//! Strength Reduction Optimization
//!
//! Replaces expensive operations with cheaper equivalents.

use crate::parser::{AstNode, AstNodeType};

/// Strength reduction patterns
pub struct StrengthReducer;

impl StrengthReducer {
    /// Try to reduce multiplication by power of 2 to shift
    pub fn reduce_mul_pow2(value: u64) -> Option<u32> {
        if value.is_power_of_two() {
            Some(value.trailing_zeros())
        } else {
            None
        }
    }

    /// Try to reduce division by power of 2 to shift
    pub fn reduce_div_pow2(value: u64) -> Option<u32> {
        if value.is_power_of_two() {
            Some(value.trailing_zeros())
        } else {
            None
        }
    }

    /// Try to reduce modulo by power of 2 to AND
    pub fn reduce_mod_pow2(value: u64) -> Option<u64> {
        if value.is_power_of_two() {
            Some(value - 1) // x % 2^n = x & (2^n - 1)
        } else {
            None
        }
    }
}
