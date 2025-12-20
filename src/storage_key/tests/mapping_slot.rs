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
    let slot1 = derive_mapping_slot(&base, std::slice::from_ref(&key));
    let slot2 = derive_mapping_slot(&base, std::slice::from_ref(&key));
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
    let expected = Sha256::digest(base);
    assert_eq!(slot[..], expected[..]);
}

