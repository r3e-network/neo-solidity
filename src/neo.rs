use sha2::{Digest, Sha256};
use std::borrow::Cow;

/// Method token for cross-contract calls in NEF format.
///
/// Method tokens are used to optimize calls to other contracts by caching
/// the target contract hash and method information in the NEF header.
#[derive(Debug, Clone)]
pub struct MethodToken {
    /// Target contract hash (20 bytes, Script Hash)
    pub hash: [u8; 20],
    /// Method name to call
    pub method: String,
    /// Number of parameters the method accepts
    pub parameters_count: u16,
    /// Whether the method returns a value
    pub has_return_value: bool,
    /// Call flags (None=0, AllowStates=1, AllowModifyStates=2, AllowCall=4, All=7)
    pub call_flags: u8,
}

impl MethodToken {
    /// Create a new method token
    pub fn new(hash: [u8; 20], method: &str, params: u16, has_return: bool, flags: u8) -> Self {
        Self {
            hash,
            method: method.to_string(),
            parameters_count: params,
            has_return_value: has_return,
            call_flags: flags,
        }
    }

    /// Serialize the method token to bytes
    fn serialize(&self, buffer: &mut Vec<u8>) {
        // Contract hash (20 bytes)
        buffer.extend_from_slice(&self.hash);

        // Method name (length-prefixed string, max 32 bytes per Neo spec)
        let bytes = self.method.as_bytes();
        assert!(
            bytes.len() <= MAX_TOKEN_METHOD_LENGTH,
            "method token '{}' exceeds {MAX_TOKEN_METHOD_LENGTH} bytes",
            self.method
        );
        write_varbytes(buffer, bytes);

        // Parameters count (2 bytes, little-endian)
        buffer.extend_from_slice(&self.parameters_count.to_le_bytes());

        // Has return value (1 byte)
        buffer.push(if self.has_return_value { 1 } else { 0 });

        // Call flags (1 byte)
        buffer.push(self.call_flags);
    }
}

/// Build a NEF (Neo Executable Format) file from raw NeoVM bytecode.
///
/// The implementation follows the Neo N3 specification:
///
/// - Magic header `NEF3`
/// - Compiler identifier (fixed 64 bytes, UTF-8, padded with zeros)
/// - Source URL (varstring, max 256 bytes)
/// - Reserved byte (must be 0)
/// - Method token table (varint count + entries, max 128 entries)
/// - Reserved 2 bytes (must be 0)
/// - Script payload (varbytes)
/// - Checksum (first four bytes of double SHA256 over all previous bytes)
///
/// # Arguments
/// * `script` - The NeoVM bytecode
/// * `compiler` - Compiler identifier string (max 64 bytes)
///
/// # Returns
/// Complete NEF file as byte vector
pub fn build_nef(script: &[u8], compiler: &str) -> Vec<u8> {
    build_nef_with_tokens(script, compiler, "", &[])
}

/// Build a NEF file with method tokens for cross-contract calls.
///
/// # Arguments
/// * `script` - The NeoVM bytecode
/// * `compiler` - Compiler identifier string (max 64 bytes)
/// * `source` - Optional source URL (max 256 bytes)
/// * `tokens` - Array of method tokens for cross-contract calls
///
/// # Returns
/// Complete NEF file as byte vector
pub fn build_nef_with_tokens(
    script: &[u8],
    compiler: &str,
    source: &str,
    tokens: &[MethodToken],
) -> Vec<u8> {
    assert!(!script.is_empty(), "NEF script payload cannot be empty");
    let source = clamp_nef_source(source);
    assert!(
        tokens.len() <= MAX_METHOD_TOKENS,
        "NEF method token table exceeds {MAX_METHOD_TOKENS} entries"
    );

    for token in tokens {
        let method_len = token.method.len();
        assert!(
            method_len <= MAX_TOKEN_METHOD_LENGTH,
            "method token '{}' exceeds {MAX_TOKEN_METHOD_LENGTH} bytes",
            token.method
        );
        assert!(
            !token.method.starts_with('_'),
            "method token '{}' must not start with '_'",
            token.method
        );
        assert!(
            token.call_flags & !MAX_CALL_FLAGS == 0,
            "method token '{}' has invalid call flags {:#x}",
            token.method,
            token.call_flags
        );
    }

    // Rough capacity hint: header (magic + compiler + empty source + reserves) plus script and tokens.
    let token_size: usize = tokens.iter().map(|t| 20 + t.method.len() + 10).sum();
    let mut buffer = Vec::with_capacity(80 + token_size + script.len());

    // Magic (4 bytes)
    buffer.extend_from_slice(b"NEF3");

    // Compiler identifier (64 bytes, zero padded)
    write_fixed_string(&mut buffer, compiler, 64);

    // Source URL (varstring, max 256 bytes)
    write_varstring(&mut buffer, &source);

    // Reserved byte must be zero
    buffer.push(0u8);

    // Method token table
    write_varint(&mut buffer, tokens.len() as u64);
    for token in tokens {
        token.serialize(&mut buffer);
    }

    // Reserved bytes (2 bytes, must be 0)
    buffer.extend_from_slice(&[0u8; 2]);

    // Script payload (length-prefixed)
    write_varbytes(&mut buffer, script);

    // Checksum (first 4 bytes of double SHA-256)
    let checksum = calculate_checksum(&buffer);
    buffer.extend_from_slice(&checksum.to_le_bytes());

    buffer
}

fn write_fixed_string(buffer: &mut Vec<u8>, value: &str, width: usize) {
    let mut padded = vec![0u8; width];
    let bytes = value.as_bytes();
    let copy_len = bytes.len().min(width);
    padded[..copy_len].copy_from_slice(&bytes[..copy_len]);
    buffer.extend_from_slice(&padded);
}

fn write_varstring(buffer: &mut Vec<u8>, value: &str) {
    write_varbytes(buffer, value.as_bytes());
}

fn write_varbytes(buffer: &mut Vec<u8>, data: &[u8]) {
    write_varint(buffer, data.len() as u64);
    buffer.extend_from_slice(data);
}

fn write_varint(buffer: &mut Vec<u8>, value: u64) {
    match value {
        0x0000_0000..=0x0000_00FC => buffer.push(value as u8),
        0x0000_00FD..=0x0000_FFFF => {
            buffer.push(0xFD);
            buffer.extend_from_slice(&(value as u16).to_le_bytes());
        }
        0x0001_0000..=0xFFFF_FFFF => {
            buffer.push(0xFE);
            buffer.extend_from_slice(&(value as u32).to_le_bytes());
        }
        _ => {
            buffer.push(0xFF);
            buffer.extend_from_slice(&value.to_le_bytes());
        }
    }
}

fn calculate_checksum(payload: &[u8]) -> u32 {
    let first = Sha256::digest(payload);
    let second = Sha256::digest(first);
    u32::from_le_bytes(second[..4].try_into().expect("checksum slice"))
}

const MAX_SOURCE_LENGTH: usize = 256;
const MAX_METHOD_TOKENS: usize = 128;
const MAX_TOKEN_METHOD_LENGTH: usize = 32;
const MAX_CALL_FLAGS: u8 = 0x07;
/// Publicly exposed maximum byte length for the NEF `source` field.
pub const NEF_SOURCE_MAX_BYTES: usize = MAX_SOURCE_LENGTH;

fn clamp_utf8<'a>(value: &'a str, max_len: usize) -> Cow<'a, str> {
    if value.len() <= max_len {
        return Cow::Borrowed(value);
    }

    let mut end = max_len.min(value.len());
    while end > 0 && !value.is_char_boundary(end) {
        end -= 1;
    }
    Cow::Owned(value[..end].to_string())
}

/// Clamp a NEF source string to the maximum allowed byte length, preserving
/// UTF-8 boundaries.
pub fn clamp_nef_source<'a>(value: &'a str) -> Cow<'a, str> {
    clamp_utf8(value, MAX_SOURCE_LENGTH)
}

/// Clamp a NEF source string and report whether truncation occurred.
pub fn clamp_nef_source_with_flag<'a>(value: &'a str) -> (Cow<'a, str>, bool) {
    let clamped = clamp_nef_source(value);
    let truncated = clamped.len() < value.len();
    (clamped, truncated)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_varint(data: &[u8], offset: &mut usize) -> u64 {
        match data[*offset] {
            0xFD => {
                let start = *offset + 1;
                *offset += 3;
                u16::from_le_bytes(data[start..start + 2].try_into().unwrap()) as u64
            }
            0xFE => {
                let start = *offset + 1;
                *offset += 5;
                u32::from_le_bytes(data[start..start + 4].try_into().unwrap()) as u64
            }
            0xFF => {
                let start = *offset + 1;
                *offset += 9;
                u64::from_le_bytes(data[start..start + 8].try_into().unwrap())
            }
            value => {
                *offset += 1;
                value as u64
            }
        }
    }

    #[test]
    fn builds_nef_payload_matching_spec() {
        let script = vec![0x0C, 0x01, 0x2A, 0x40];
        let token = MethodToken::new([0x11; 20], "transfer", 2, true, 0x07);
        let source = "https://example.com/src.sol";
        let compiler = "neo-solidity-test";

        let nef = build_nef_with_tokens(&script, compiler, source, &[token]);
        let mut offset = 0usize;

        // Magic
        assert_eq!(&nef[offset..offset + 4], b"NEF3");
        offset += 4;

        // Compiler (padded to 64 bytes)
        let compiler_bytes = &nef[offset..offset + 64];
        assert_eq!(
            &compiler_bytes[..compiler.len()],
            compiler.as_bytes(),
            "compiler prefix should match"
        );
        offset += 64;

        // Source
        let source_len = read_varint(&nef, &mut offset) as usize;
        let source_bytes = &nef[offset..offset + source_len];
        assert_eq!(std::str::from_utf8(source_bytes).unwrap(), source);
        offset += source_len;

        // Reserved byte
        assert_eq!(nef[offset], 0);
        offset += 1;

        // Tokens
        let token_count = read_varint(&nef, &mut offset);
        assert_eq!(token_count, 1);
        assert_eq!(&nef[offset..offset + 20], &[0x11u8; 20]);
        offset += 20;

        let method_len = read_varint(&nef, &mut offset) as usize;
        let method = &nef[offset..offset + method_len];
        assert_eq!(std::str::from_utf8(method).unwrap(), "transfer");
        offset += method_len;

        let params = u16::from_le_bytes(nef[offset..offset + 2].try_into().expect("param bytes"));
        offset += 2;
        let has_return = nef[offset] != 0;
        offset += 1;
        let call_flags = nef[offset];
        offset += 1;

        assert_eq!(params, 2);
        assert!(has_return);
        assert_eq!(call_flags, 0x07);

        // Reserved 2 bytes
        assert_eq!(&nef[offset..offset + 2], &[0u8; 2]);
        offset += 2;

        // Script payload
        let script_len = read_varint(&nef, &mut offset) as usize;
        assert_eq!(script_len, script.len());
        assert_eq!(&nef[offset..offset + script_len], script.as_slice());
        offset += script_len;

        // Checksum
        let checksum = u32::from_le_bytes(nef[offset..offset + 4].try_into().unwrap());
        let expected_checksum = calculate_checksum(&nef[..offset]);
        assert_eq!(checksum, expected_checksum);
        offset += 4;

        assert_eq!(offset, nef.len(), "parsed all bytes");
    }

    #[test]
    #[should_panic(expected = "call flags")]
    fn rejects_invalid_call_flags() {
        let script = vec![0x40];
        let bad_token = MethodToken::new([0x22; 20], "x", 0, false, 0x80);
        let _ = build_nef_with_tokens(&script, "compiler", "", &[bad_token]);
    }

    #[test]
    fn clamps_long_source_field() {
        let script = vec![0x40];
        let long_source = "a".repeat(MAX_SOURCE_LENGTH + 20);
        let nef = build_nef_with_tokens(&script, "compiler", &long_source, &[]);
        let mut offset = 4 + 64; // magic + compiler
        let len = read_varint(&nef, &mut offset) as usize;
        assert_eq!(len, MAX_SOURCE_LENGTH);
        let source = std::str::from_utf8(&nef[offset..offset + len]).unwrap();
        assert!(source.len() <= MAX_SOURCE_LENGTH);
    }
}
