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

