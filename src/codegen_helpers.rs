//! Code Generation Helpers
//!
//! Utility functions for bytecode generation.

/// Encode a small integer efficiently
pub fn encode_small_int(n: i64) -> Vec<u8> {
    match n {
        -1 => vec![0x0F],      // PUSHM1
        0..=16 => vec![0x10 + n as u8], // PUSH0-PUSH16
        _ => encode_varint(n),
    }
}

fn encode_varint(n: i64) -> Vec<u8> {
    let bytes = n.to_le_bytes();
    let len = minimal_bytes(n);
    let mut result = vec![0x00 + len as u8];
    result.extend_from_slice(&bytes[..len]);
    result
}

fn minimal_bytes(n: i64) -> usize {
    if n >= -128 && n <= 127 { 1 }
    else if n >= -32768 && n <= 32767 { 2 }
    else if n >= -2147483648 && n <= 2147483647 { 4 }
    else { 8 }
}
