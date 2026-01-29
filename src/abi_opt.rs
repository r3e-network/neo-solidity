//! ABI Encoding Optimization
//!
//! Optimizes ABI encoding/decoding operations.

/// ABI optimization hints
#[derive(Debug, Clone, Copy)]
pub enum AbiHint {
    /// Fixed-size type, can use direct copy
    FixedSize(usize),
    /// Dynamic type, needs length prefix
    Dynamic,
    /// Packed encoding
    Packed,
}

/// Check if type has fixed ABI size
pub fn is_fixed_size(type_name: &str) -> Option<usize> {
    match type_name {
        "uint8" | "int8" | "bool" => Some(1),
        "uint16" | "int16" => Some(2),
        "uint32" | "int32" => Some(4),
        "uint64" | "int64" => Some(8),
        "uint128" | "int128" => Some(16),
        "uint256" | "int256" => Some(32),
        "address" => Some(20),
        "bytes32" => Some(32),
        _ => None,
    }
}
