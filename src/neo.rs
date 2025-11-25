use sha2::{Digest, Sha256};

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

        // Method name (length-prefixed string)
        let method_bytes = self.method.as_bytes();
        write_varint(buffer, method_bytes.len() as u32);
        buffer.extend_from_slice(method_bytes);

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
/// The implementation follows the Neo N3 specification: a 44-byte header
/// (`NEF3` magic, compiler identifier, packed version, script length),
/// followed by the method token table (empty), the script bytes, and finally
/// the checksum (first four bytes of the double SHA-256 hash of the
/// preceding payload).
///
/// # Arguments
/// * `script` - The NeoVM bytecode
/// * `compiler` - Compiler identifier string (max 32 bytes)
/// * `version` - Version tuple (major, minor, build, revision)
///
/// # Returns
/// Complete NEF file as byte vector
pub fn build_nef(script: &[u8], compiler: &str, version: (u32, u32, u32, u32)) -> Vec<u8> {
    // Use legacy format for backward compatibility
    build_nef_legacy(script, compiler, version)
}

/// Build a NEF file with method tokens for cross-contract calls.
///
/// # Arguments
/// * `script` - The NeoVM bytecode
/// * `compiler` - Compiler identifier string (max 32 bytes)
/// * `version` - Version tuple (major, minor, build, revision)
/// * `tokens` - Array of method tokens for cross-contract calls
///
/// # Returns
/// Complete NEF file as byte vector
pub fn build_nef_with_tokens(
    script: &[u8],
    compiler: &str,
    version: (u32, u32, u32, u32),
    tokens: &[MethodToken],
) -> Vec<u8> {
    // Estimate buffer size
    let token_size: usize = tokens.iter().map(|t| 20 + t.method.len() + 10).sum();
    let mut buffer = Vec::with_capacity(56 + token_size + script.len());

    // Magic (4 bytes)
    buffer.extend_from_slice(b"NEF3");

    // Compiler identifier (32 bytes, zero padded)
    let mut compiler_bytes = [0u8; 32];
    let compiler_utf8 = compiler.as_bytes();
    let copy_len = compiler_utf8.len().min(compiler_bytes.len());
    compiler_bytes[..copy_len].copy_from_slice(&compiler_utf8[..copy_len]);
    buffer.extend_from_slice(&compiler_bytes);

    // Version packed into a single u32 (one byte per component)
    let packed_version = pack_version(version);
    buffer.extend_from_slice(&packed_version.to_le_bytes());

    // Source (empty string for now, length-prefixed)
    // Neo N3 NEF format includes a source field after compiler
    write_varint(&mut buffer, 0); // Empty source string

    // Method token table
    write_varint(&mut buffer, tokens.len() as u32);
    for token in tokens {
        token.serialize(&mut buffer);
    }

    // Reserved bytes (2 bytes, must be 0)
    buffer.extend_from_slice(&[0u8; 2]);

    // Script payload (length-prefixed)
    write_varint(&mut buffer, script.len() as u32);
    buffer.extend_from_slice(script);

    // Checksum (first 4 bytes of double SHA-256)
    let checksum = calculate_checksum(&buffer);
    buffer.extend_from_slice(&checksum.to_le_bytes());

    buffer
}

/// Build NEF using legacy format (for backward compatibility)
/// This format omits the source field and reserved bytes
pub fn build_nef_legacy(script: &[u8], compiler: &str, version: (u32, u32, u32, u32)) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(48 + script.len());

    // Magic
    buffer.extend_from_slice(b"NEF3");

    // Compiler identifier (32 bytes, zero padded)
    let mut compiler_bytes = [0u8; 32];
    let compiler_utf8 = compiler.as_bytes();
    let copy_len = compiler_utf8.len().min(compiler_bytes.len());
    compiler_bytes[..copy_len].copy_from_slice(&compiler_utf8[..copy_len]);
    buffer.extend_from_slice(&compiler_bytes);

    // Version packed into a single u32 (one byte per component)
    let packed_version = pack_version(version);
    buffer.extend_from_slice(&packed_version.to_le_bytes());

    // Script length
    buffer.extend_from_slice(&(script.len() as u32).to_le_bytes());

    // Method token table (empty)
    write_varint(&mut buffer, 0);

    // Script payload
    buffer.extend_from_slice(script);

    // Checksum (first 4 bytes of double SHA-256)
    let checksum = calculate_checksum(&buffer);
    buffer.extend_from_slice(&checksum.to_le_bytes());

    buffer
}

fn pack_version((major, minor, build, revision): (u32, u32, u32, u32)) -> u32 {
    ((major & 0xFF) << 24) | ((minor & 0xFF) << 16) | ((build & 0xFF) << 8) | (revision & 0xFF)
}

fn write_varint(buffer: &mut Vec<u8>, value: u32) {
    match value {
        0x0000_0000..=0x0000_00FC => buffer.push(value as u8),
        0x0000_00FD..=0x0000_FFFF => {
            buffer.push(0xFD);
            buffer.extend_from_slice(&(value as u16).to_le_bytes());
        }
        _ => {
            buffer.push(0xFE);
            buffer.extend_from_slice(&value.to_le_bytes());
        }
    }
}

fn calculate_checksum(payload: &[u8]) -> u32 {
    let first = Sha256::digest(payload);
    let second = Sha256::digest(first);
    u32::from_le_bytes(second[..4].try_into().expect("checksum slice"))
}
