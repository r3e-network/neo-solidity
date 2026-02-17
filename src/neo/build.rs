use super::constants::{MAX_CALL_FLAGS, MAX_METHOD_TOKENS, MAX_TOKEN_METHOD_LENGTH};
use super::encoding::{
    calculate_checksum, write_fixed_string, write_varbytes, write_varint, write_varstring,
};
use super::method_token::MethodToken;
use super::source::clamp_nef_source;

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
pub fn build_nef(script: &[u8], compiler: &str) -> Result<Vec<u8>, String> {
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
) -> Result<Vec<u8>, String> {
    if script.is_empty() {
        return Err("NEF script payload cannot be empty".to_string());
    }
    let source = clamp_nef_source(source);
    if tokens.len() > MAX_METHOD_TOKENS {
        return Err(format!(
            "NEF method token table exceeds {MAX_METHOD_TOKENS} entries"
        ));
    }

    for token in tokens {
        let method_len = token.method.len();
        if method_len > MAX_TOKEN_METHOD_LENGTH {
            return Err(format!(
                "method token '{}' exceeds {MAX_TOKEN_METHOD_LENGTH} bytes",
                token.method
            ));
        }
        if token.method.starts_with('_') {
            return Err(format!(
                "method token '{}' must not start with '_'",
                token.method
            ));
        }
        if token.call_flags & !MAX_CALL_FLAGS != 0 {
            return Err(format!(
                "method token '{}' has invalid call flags {:#x}",
                token.method, token.call_flags
            ));
        }
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

    Ok(buffer)
}
