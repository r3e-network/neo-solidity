//! Storage Key Tests
//!
//! Tests for storage key computation and derivation.

use neo_devpack_solidity::storage_key::{compute_state_slot, derive_mapping_slot, KeyFragment};
use num_bigint::BigInt;

#[test]
fn test_compute_state_slot_basic() {
    let slot = compute_state_slot("balance");
    assert_eq!(slot.len(), 32, "Slot should be 32 bytes");
}

#[test]
fn test_compute_state_slot_deterministic() {
    let slot1 = compute_state_slot("myVar");
    let slot2 = compute_state_slot("myVar");
    assert_eq!(slot1, slot2, "Same name should produce same slot");
}

#[test]
fn test_compute_state_slot_different_names() {
    let slot1 = compute_state_slot("balance");
    let slot2 = compute_state_slot("allowance");
    assert_ne!(
        slot1, slot2,
        "Different names should produce different slots"
    );
}

#[test]
fn test_derive_mapping_slot_basic() {
    let base = compute_state_slot("balances");
    let key = KeyFragment::address(vec![1u8; 20]);
    let slot = derive_mapping_slot(&base, &[key]);
    assert_eq!(slot.len(), 32);
}

#[test]
fn test_derive_mapping_slot_different_keys() {
    let base = compute_state_slot("balances");
    let key1 = KeyFragment::address(vec![1u8; 20]);
    let key2 = KeyFragment::address(vec![2u8; 20]);
    let slot1 = derive_mapping_slot(&base, &[key1]);
    let slot2 = derive_mapping_slot(&base, &[key2]);
    assert_ne!(slot1, slot2);
}

#[test]
fn test_derive_mapping_slot_nested() {
    let base = compute_state_slot("allowances");
    let owner = KeyFragment::address(vec![1u8; 20]);
    let spender = KeyFragment::address(vec![2u8; 20]);
    let slot = derive_mapping_slot(&base, &[owner, spender]);
    assert_eq!(slot.len(), 32);
}

#[test]
fn test_key_fragment_integer() {
    let base = compute_state_slot("data");
    let key = KeyFragment::uint256(BigInt::from(42));
    let slot = derive_mapping_slot(&base, &[key]);
    assert_eq!(slot.len(), 32);
}

#[test]
fn test_key_fragment_boolean() {
    let base = compute_state_slot("flags");
    let key = KeyFragment::boolean(true);
    let slot = derive_mapping_slot(&base, &[key]);
    assert_eq!(slot.len(), 32);
}

#[test]
fn test_key_fragment_string() {
    let base = compute_state_slot("names");
    let key = KeyFragment::string("alice");
    let slot = derive_mapping_slot(&base, &[key]);
    assert_eq!(slot.len(), 32);
}
