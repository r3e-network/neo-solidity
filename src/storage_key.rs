use num_bigint::BigInt;
use num_traits::Signed;
use sha2::{Digest, Sha256};

/// Represents a single mapping key fragment used to derive a storage slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyFragment {
    Integer {
        value: BigInt,
        bits: u16,
        signed: bool,
    },
    Boolean(bool),
    Address(Vec<u8>),
    Bytes(Vec<u8>),
    String(String),
}

impl KeyFragment {
    pub fn integer(value: BigInt, bits: u16, signed: bool) -> Self {
        Self::Integer {
            value,
            bits,
            signed,
        }
    }

    pub fn boolean(value: bool) -> Self {
        Self::Boolean(value)
    }

    pub fn address(bytes: impl Into<Vec<u8>>) -> Self {
        Self::Address(bytes.into())
    }

    pub fn bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::Bytes(bytes.into())
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }
}

/// Compute the canonical storage slot hash for a state variable name.
///
/// The returned value is the 32-byte SHA-256 digest of the UTF-8 name.
pub fn compute_state_slot(name: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

/// Derive the storage slot for a mapping entry given the base slot and key fragments.
///
/// Each key fragment is serialised with a 4-byte little-endian length prefix to avoid
/// ambiguity between `{a, bc}` and `{ab, c}` style encodings.
pub fn derive_mapping_slot(base_slot: &[u8], fragments: &[KeyFragment]) -> [u8; 32] {
    let mut buffer = Vec::new();
    for fragment in fragments {
        let encoded = encode_fragment(fragment);
        buffer.extend_from_slice(&(encoded.len() as u32).to_le_bytes());
        buffer.extend_from_slice(&encoded);
    }
    buffer.extend_from_slice(base_slot);

    let digest = Sha256::digest(buffer);
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

fn encode_fragment(fragment: &KeyFragment) -> Vec<u8> {
    match fragment {
        KeyFragment::Integer {
            value,
            bits,
            signed,
        } => encode_integer(value, *bits, *signed),
        KeyFragment::Boolean(value) => vec![if *value { 1 } else { 0 }],
        KeyFragment::Address(bytes) => bytes.clone(),
        KeyFragment::Bytes(bytes) => bytes.clone(),
        KeyFragment::String(value) => value.as_bytes().to_vec(),
    }
}

fn encode_integer(value: &BigInt, bits: u16, signed: bool) -> Vec<u8> {
    let min_bytes = ((bits.max(8) as usize) + 7) / 8;
    if signed {
        let mut raw = value.to_signed_bytes_be();
        let needs_negative_padding = value.is_negative();
        let pad_byte = if needs_negative_padding { 0xFF } else { 0x00 };
        if raw.len() < min_bytes {
            let mut padded = vec![pad_byte; min_bytes];
            padded[min_bytes - raw.len()..].copy_from_slice(&raw);
            raw = padded;
        }
        raw
    } else {
        let (_, mut raw) = value.to_bytes_be();
        let pad_byte = 0x00;
        if raw.len() < min_bytes {
            let mut padded = vec![pad_byte; min_bytes];
            padded[min_bytes - raw.len()..].copy_from_slice(&raw);
            raw = padded;
        }
        raw
    }
}
