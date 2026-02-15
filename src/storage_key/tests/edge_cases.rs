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

    let slot1 = derive_mapping_slot(&base1, std::slice::from_ref(&key));
    let slot2 = derive_mapping_slot(&base2, std::slice::from_ref(&key));

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
    let debug_str = format!("{frag:?}");
    assert!(debug_str.contains("Boolean"));
    assert!(debug_str.contains("true"));
}

