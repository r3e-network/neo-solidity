use sha2::{Digest, Sha256};

/// Build a NEF (Neo Executable Format) file from raw NeoVM bytecode.
///
/// The implementation follows the Neo N3 specification: a 44-byte header
/// (`NEF3` magic, compiler identifier, packed version, script length),
/// followed by the method token table (empty for now), the script bytes, and
/// finally the checksum (first four bytes of the double SHA-256 hash of the
/// preceding payload).
pub fn build_nef(script: &[u8], compiler: &str, version: (u32, u32, u32, u32)) -> Vec<u8> {
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

    // Method token table (empty for now)
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
