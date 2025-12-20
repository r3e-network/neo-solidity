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

