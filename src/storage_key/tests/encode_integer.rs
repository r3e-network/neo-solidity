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

// Wave-#22 mutation testing surfaced a test-quality gap on the
// `if raw.len() < min_bytes` boundary at storage_key.rs:149/158.
// A `<` → `<=` mutation would silently prepend an extra padding byte
// when the value's natural BE representation is already exactly
// `min_bytes` long — corrupting mapping-slot derivation for keys
// like `uint8(0xFF)`, `int64(i64::MIN)`. These tests pin the exact
// boundary so any future regression at that comparison fails
// immediately.

#[test]
fn test_encode_unsigned_at_exact_min_bytes_no_extra_pad() {
    // u8 max — to_bytes_be() = [0xFF], min_bytes = 1. With `<` the
    // padding branch is skipped; with `<=` an extra 0x00 would be
    // prepended and length would be 2.
    let value = BigInt::from(255u32);
    let encoded = encode_integer(&value, 8, false);
    assert_eq!(
        encoded,
        vec![0xFF],
        "uint8(0xFF) must encode to exactly [0xFF] (1 byte) — a \
         `<` → `<=` mutation at storage_key.rs:158 would prepend \
         0x00 here, silently changing the mapping-slot hash"
    );

    // u32 max — exactly 4 bytes natively.
    let value = BigInt::from(u32::MAX);
    let encoded = encode_integer(&value, 32, false);
    assert_eq!(encoded.len(), 4, "uint32::MAX must be exactly 4 bytes");
    assert_eq!(encoded, vec![0xFF, 0xFF, 0xFF, 0xFF]);

    // u64 max — exactly 8 bytes.
    let value = BigInt::from(u64::MAX);
    let encoded = encode_integer(&value, 64, false);
    assert_eq!(encoded.len(), 8);
    assert!(encoded.iter().all(|&b| b == 0xFF));
}

#[test]
fn test_encode_signed_at_exact_min_bytes_no_extra_pad() {
    // int8 min (-128) — to_signed_bytes_be() = [0x80], min_bytes = 1.
    let value = BigInt::from(-128);
    let encoded = encode_integer(&value, 8, true);
    assert_eq!(
        encoded,
        vec![0x80],
        "int8(-128) must encode to exactly [0x80] (1 byte) — a \
         `<` → `<=` mutation at storage_key.rs:149 would prepend an \
         0xFF padding byte here, silently changing the mapping-slot hash"
    );

    // int8 max (127) — to_signed_bytes_be() = [0x7F].
    let value = BigInt::from(127);
    let encoded = encode_integer(&value, 8, true);
    assert_eq!(encoded, vec![0x7F]);

    // int64 min — exactly 8 bytes natively.
    let value = BigInt::from(i64::MIN);
    let encoded = encode_integer(&value, 64, true);
    assert_eq!(encoded.len(), 8, "int64::MIN must be exactly 8 bytes");
    assert_eq!(
        encoded,
        vec![0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    );
}

#[test]
fn test_type_name_returns_correct_label() {
    // Mutation gap: a stub returning "" or any other constant for all
    // arms slipped through the prior tests. Pin the per-variant label.
    assert_eq!(
        KeyFragment::Integer {
            value: BigInt::from(1),
            bits: 256,
            signed: true
        }
        .type_name(),
        "int"
    );
    assert_eq!(
        KeyFragment::Integer {
            value: BigInt::from(1),
            bits: 256,
            signed: false
        }
        .type_name(),
        "uint"
    );
    assert_eq!(KeyFragment::Boolean(true).type_name(), "bool");
    assert_eq!(KeyFragment::Address(vec![0u8; 20]).type_name(), "address");
    assert_eq!(KeyFragment::Bytes(vec![]).type_name(), "bytes");
    assert_eq!(KeyFragment::String(String::new()).type_name(), "string");
}

