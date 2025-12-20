const MAX_SOURCE_LENGTH: usize = 256;
/// Maximum number of method tokens allowed in a single NEF file.
pub const MAX_METHOD_TOKENS: usize = 128;
/// Maximum byte length of a method name stored in a NEF method token.
pub const MAX_TOKEN_METHOD_LENGTH: usize = 32;
/// Bitmask of valid CallFlags values (`CallFlags.All`).
pub const MAX_CALL_FLAGS: u8 = 0x0F;
/// Publicly exposed maximum byte length for the NEF `source` field.
pub const NEF_SOURCE_MAX_BYTES: usize = MAX_SOURCE_LENGTH;
