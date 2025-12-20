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

