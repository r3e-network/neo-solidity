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
    let min_bytes = (bits.max(8) as usize).div_ceil(8);
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

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== KeyFragment Constructor Tests ====================

    #[test]
    fn test_key_fragment_integer() {
        let frag = KeyFragment::integer(BigInt::from(42), 256, false);
        match frag {
            KeyFragment::Integer {
                value,
                bits,
                signed,
            } => {
                assert_eq!(value, BigInt::from(42));
                assert_eq!(bits, 256);
                assert!(!signed);
            }
            _ => panic!("Expected Integer variant"),
        }
    }

    #[test]
    fn test_key_fragment_signed_integer() {
        let frag = KeyFragment::integer(BigInt::from(-100), 128, true);
        match frag {
            KeyFragment::Integer {
                value,
                bits,
                signed,
            } => {
                assert_eq!(value, BigInt::from(-100));
                assert_eq!(bits, 128);
                assert!(signed);
            }
            _ => panic!("Expected Integer variant"),
        }
    }

    #[test]
    fn test_key_fragment_boolean_true() {
        let frag = KeyFragment::boolean(true);
        assert_eq!(frag, KeyFragment::Boolean(true));
    }

    #[test]
    fn test_key_fragment_boolean_false() {
        let frag = KeyFragment::boolean(false);
        assert_eq!(frag, KeyFragment::Boolean(false));
    }

    #[test]
    fn test_key_fragment_address() {
        let addr = vec![0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0];
        let frag = KeyFragment::address(addr.clone());
        assert_eq!(frag, KeyFragment::Address(addr));
    }

    #[test]
    fn test_key_fragment_bytes() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let frag = KeyFragment::bytes(data.clone());
        assert_eq!(frag, KeyFragment::Bytes(data));
    }

    #[test]
    fn test_key_fragment_string() {
        let frag = KeyFragment::string("hello");
        assert_eq!(frag, KeyFragment::String("hello".to_string()));
    }

    // ==================== compute_state_slot Tests ====================

    #[test]
    fn test_compute_state_slot_deterministic() {
        let slot1 = compute_state_slot("balance");
        let slot2 = compute_state_slot("balance");
        assert_eq!(slot1, slot2);
    }

    #[test]
    fn test_compute_state_slot_different_names() {
        let slot1 = compute_state_slot("balance");
        let slot2 = compute_state_slot("owner");
        assert_ne!(slot1, slot2);
    }

    #[test]
    fn test_compute_state_slot_length() {
        let slot = compute_state_slot("test");
        assert_eq!(slot.len(), 32);
    }

    #[test]
    fn test_compute_state_slot_empty_name() {
        let slot = compute_state_slot("");
        assert_eq!(slot.len(), 32);
        // SHA-256 of empty string is a known value
        let expected = Sha256::digest(b"");
        assert_eq!(slot[..], expected[..]);
    }

    #[test]
    fn test_compute_state_slot_unicode() {
        let slot = compute_state_slot("余额");
        assert_eq!(slot.len(), 32);
        // Should be deterministic
        let slot2 = compute_state_slot("余额");
        assert_eq!(slot, slot2);
    }

    // ==================== derive_mapping_slot Tests ====================

    #[test]
    fn test_derive_mapping_slot_single_key() {
        let base = compute_state_slot("balances");
        let key = KeyFragment::address(vec![0x01; 20]);
        let slot = derive_mapping_slot(&base, &[key]);
        assert_eq!(slot.len(), 32);
    }

    #[test]
    fn test_derive_mapping_slot_deterministic() {
        let base = compute_state_slot("balances");
        let key = KeyFragment::address(vec![0x01; 20]);
        let slot1 = derive_mapping_slot(&base, &[key.clone()]);
        let slot2 = derive_mapping_slot(&base, &[key]);
        assert_eq!(slot1, slot2);
    }

    #[test]
    fn test_derive_mapping_slot_different_keys() {
        let base = compute_state_slot("balances");
        let key1 = KeyFragment::address(vec![0x01; 20]);
        let key2 = KeyFragment::address(vec![0x02; 20]);
        let slot1 = derive_mapping_slot(&base, &[key1]);
        let slot2 = derive_mapping_slot(&base, &[key2]);
        assert_ne!(slot1, slot2);
    }

    #[test]
    fn test_derive_mapping_slot_nested_mapping() {
        // mapping(address => mapping(address => uint256))
        let base = compute_state_slot("allowances");
        let owner = KeyFragment::address(vec![0x01; 20]);
        let spender = KeyFragment::address(vec![0x02; 20]);
        let slot = derive_mapping_slot(&base, &[owner, spender]);
        assert_eq!(slot.len(), 32);
    }

    #[test]
    fn test_derive_mapping_slot_integer_key() {
        let base = compute_state_slot("data");
        let key = KeyFragment::integer(BigInt::from(42), 256, false);
        let slot = derive_mapping_slot(&base, &[key]);
        assert_eq!(slot.len(), 32);
    }

    #[test]
    fn test_derive_mapping_slot_string_key() {
        let base = compute_state_slot("names");
        let key = KeyFragment::string("alice");
        let slot = derive_mapping_slot(&base, &[key]);
        assert_eq!(slot.len(), 32);
    }

    #[test]
    fn test_derive_mapping_slot_boolean_key() {
        let base = compute_state_slot("flags");
        let key_true = KeyFragment::boolean(true);
        let key_false = KeyFragment::boolean(false);
        let slot_true = derive_mapping_slot(&base, &[key_true]);
        let slot_false = derive_mapping_slot(&base, &[key_false]);
        assert_ne!(slot_true, slot_false);
    }

    #[test]
    fn test_derive_mapping_slot_empty_fragments() {
        let base = compute_state_slot("data");
        let slot = derive_mapping_slot(&base, &[]);
        // With no fragments, should just hash the base slot
        let expected = Sha256::digest(&base);
        assert_eq!(slot[..], expected[..]);
    }

    // ==================== encode_fragment Tests ====================

    #[test]
    fn test_encode_fragment_boolean_true() {
        let frag = KeyFragment::Boolean(true);
        let encoded = encode_fragment(&frag);
        assert_eq!(encoded, vec![1]);
    }

    #[test]
    fn test_encode_fragment_boolean_false() {
        let frag = KeyFragment::Boolean(false);
        let encoded = encode_fragment(&frag);
        assert_eq!(encoded, vec![0]);
    }

    #[test]
    fn test_encode_fragment_address() {
        let addr = vec![0xde, 0xad, 0xbe, 0xef];
        let frag = KeyFragment::Address(addr.clone());
        let encoded = encode_fragment(&frag);
        assert_eq!(encoded, addr);
    }

    #[test]
    fn test_encode_fragment_bytes() {
        let data = vec![0x01, 0x02, 0x03];
        let frag = KeyFragment::Bytes(data.clone());
        let encoded = encode_fragment(&frag);
        assert_eq!(encoded, data);
    }

    #[test]
    fn test_encode_fragment_string() {
        let frag = KeyFragment::String("test".to_string());
        let encoded = encode_fragment(&frag);
        assert_eq!(encoded, b"test".to_vec());
    }

    // ==================== encode_integer Tests ====================

    #[test]
    fn test_encode_integer_unsigned_small() {
        let value = BigInt::from(42u32);
        let encoded = encode_integer(&value, 8, false);
        assert_eq!(encoded, vec![42]);
    }

    #[test]
    fn test_encode_integer_unsigned_256bit() {
        let value = BigInt::from(1u32);
        let encoded = encode_integer(&value, 256, false);
        assert_eq!(encoded.len(), 32);
        assert_eq!(encoded[31], 1);
        assert!(encoded[..31].iter().all(|&b| b == 0));
    }

    #[test]
    fn test_encode_integer_signed_positive() {
        let value = BigInt::from(127);
        let encoded = encode_integer(&value, 8, true);
        assert_eq!(encoded, vec![127]);
    }

    #[test]
    fn test_encode_integer_signed_negative() {
        let value = BigInt::from(-1);
        let encoded = encode_integer(&value, 8, true);
        assert_eq!(encoded, vec![0xFF]);
    }

    #[test]
    fn test_encode_integer_signed_negative_128bit() {
        let value = BigInt::from(-1);
        let encoded = encode_integer(&value, 128, true);
        assert_eq!(encoded.len(), 16);
        assert!(encoded.iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn test_encode_integer_zero() {
        let value = BigInt::from(0);
        let encoded = encode_integer(&value, 256, false);
        assert_eq!(encoded.len(), 32);
        assert!(encoded.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_encode_integer_max_uint256() {
        let max_uint256 = (BigInt::from(1) << 256) - 1;
        let encoded = encode_integer(&max_uint256, 256, false);
        assert_eq!(encoded.len(), 32);
        assert!(encoded.iter().all(|&b| b == 0xFF));
    }

    // ==================== Edge Cases and Integration Tests ====================

    #[test]
    fn test_mapping_slot_order_matters() {
        let base = compute_state_slot("data");
        let key1 = KeyFragment::integer(BigInt::from(1), 256, false);
        let key2 = KeyFragment::integer(BigInt::from(2), 256, false);

        let slot_12 = derive_mapping_slot(&base, &[key1.clone(), key2.clone()]);
        let slot_21 = derive_mapping_slot(&base, &[key2, key1]);

        assert_ne!(slot_12, slot_21);
    }

    #[test]
    fn test_different_base_slots_different_results() {
        let base1 = compute_state_slot("balances");
        let base2 = compute_state_slot("allowances");
        let key = KeyFragment::address(vec![0x01; 20]);

        let slot1 = derive_mapping_slot(&base1, &[key.clone()]);
        let slot2 = derive_mapping_slot(&base2, &[key]);

        assert_ne!(slot1, slot2);
    }

    #[test]
    fn test_length_prefix_prevents_ambiguity() {
        // Test that {a, bc} and {ab, c} produce different slots
        let base = compute_state_slot("data");

        let key_a = KeyFragment::bytes(vec![0x61]); // "a"
        let key_bc = KeyFragment::bytes(vec![0x62, 0x63]); // "bc"
        let key_ab = KeyFragment::bytes(vec![0x61, 0x62]); // "ab"
        let key_c = KeyFragment::bytes(vec![0x63]); // "c"

        let slot_a_bc = derive_mapping_slot(&base, &[key_a, key_bc]);
        let slot_ab_c = derive_mapping_slot(&base, &[key_ab, key_c]);

        assert_ne!(slot_a_bc, slot_ab_c);
    }

    #[test]
    fn test_key_fragment_clone() {
        let frag = KeyFragment::integer(BigInt::from(42), 256, false);
        let cloned = frag.clone();
        assert_eq!(frag, cloned);
    }

    #[test]
    fn test_key_fragment_debug() {
        let frag = KeyFragment::boolean(true);
        let debug_str = format!("{:?}", frag);
        assert!(debug_str.contains("Boolean"));
        assert!(debug_str.contains("true"));
    }
}
