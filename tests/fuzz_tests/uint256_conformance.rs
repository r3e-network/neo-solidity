//! End-to-end conformance tests for `uint256` values >= 2^255.
//!
//! NeoVM integers are signed two's-complement, capped at 32 bytes, so a Solidity
//! `uint256` value in `[2^255, 2^256-1]` (e.g. `type(uint256).max`) is stored as
//! its 32-byte two's-complement (which "looks negative"). These tests assert the
//! observable Solidity semantics hold for that range: literal round-trips,
//! UNSIGNED ordered comparison (native NeoVM compare is signed), equality, and
//! checked/unchecked arithmetic with overflow detection.
//!
//! Each contract exposes a single `f()` and is driven through
//! `compile_and_execute` / `observe`.

use super::common::{compile_and_execute, observe, ObservedBehavior};
use num_bigint::BigUint;

fn u256_max() -> BigUint {
    (BigUint::from(1u8) << 256u32) - BigUint::from(1u8)
}

fn returns_bool(body: &str) -> bool {
    let source = format!(
        r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {{ function f() external pure returns (bool) {{ {body} }} }}"#
    );
    match observe(&compile_and_execute(&source)) {
        ObservedBehavior::Returned(v) => v != BigUint::from(0u8),
        other => panic!("expected bool return, got {other:?} for body: {body}"),
    }
}

fn returns_uint(body: &str) -> BigUint {
    let source = format!(
        r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {{ function f() external pure returns (uint256) {{ {body} }} }}"#
    );
    match observe(&compile_and_execute(&source)) {
        ObservedBehavior::Returned(v) => v,
        other => panic!("expected uint return, got {other:?} for body: {body}"),
    }
}

// ---------------- literal round-trip ----------------

#[test]
fn literal_max_returns_unsigned() {
    assert_eq!(returns_uint("return type(uint256).max;"), u256_max());
}

#[test]
fn literal_one_shl_255() {
    assert_eq!(returns_uint("return 1 << 255;"), BigUint::from(1u8) << 255u32);
}

// ---------------- unsigned ordered comparison ----------------

#[test]
fn max_is_greater_than_small() {
    assert!(returns_bool("return type(uint256).max >= 5;"));
    assert!(returns_bool("return type(uint256).max > 5;"));
    assert!(!returns_bool("return type(uint256).max < 5;"));
    assert!(!returns_bool("return type(uint256).max <= 5;"));
}

#[test]
fn small_is_less_than_max() {
    assert!(returns_bool("return uint256(5) < type(uint256).max;"));
    assert!(returns_bool("return uint256(5) <= type(uint256).max;"));
    assert!(!returns_bool("return uint256(5) > type(uint256).max;"));
}

#[test]
fn straddle_2_255_boundary() {
    // 2^255 vs 2^255 - 1: the exact point where signed and unsigned diverge.
    // Uses variables (the realistic form); a bare literal-vs-literal compare is
    // a separate constant-folding concern.
    assert!(returns_bool(
        "uint256 a = uint256(1) << 255; uint256 b = a - 1; return a > b;"
    ));
    assert!(returns_bool(
        "uint256 a = uint256(1) << 255; uint256 b = a - 1; return b < a;"
    ));
}

#[test]
fn equality_on_max() {
    assert!(returns_bool("return type(uint256).max == type(uint256).max;"));
    assert!(!returns_bool("return type(uint256).max == 0;"));
    assert!(returns_bool("return type(uint256).max != 0;"));
}

// ---------------- checked arithmetic (overflow/underflow -> Panic 0x11) ----------------

fn panics(body: &str) -> bool {
    let source = format!(
        r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {{ function f() external pure returns (uint256) {{ {body} }} }}"#
    );
    matches!(
        observe(&compile_and_execute(&source)),
        ObservedBehavior::Panicked(0x11)
    )
}

#[test]
fn checked_add_overflow_panics() {
    assert!(panics("return type(uint256).max + 1;"));
    assert!(panics("uint256 a = type(uint256).max; uint256 b = 5; return a + b;"));
    // In range: no panic, correct value.
    assert_eq!(returns_uint("return type(uint256).max - 1 + 1;"), u256_max());
}

#[test]
fn checked_sub_underflow_panics() {
    assert!(panics("uint256 a = 3; uint256 b = 5; return a - b;"));
    assert!(panics("return uint256(0) - 1;"));
    // 2^255 - 1 crosses the sign boundary but does not underflow.
    assert_eq!(returns_uint("return (uint256(1) << 255) - 1;"), (BigUint::from(1u8) << 255u32) - BigUint::from(1u8));
}

#[test]
fn checked_mul_overflow_panics() {
    assert!(panics("return (uint256(1) << 255) * 2;"));
    assert!(panics("return type(uint256).max * 2;"));
    // (2^255 - 1) * 2 = 2^256 - 2 fits.
    assert_eq!(
        returns_uint("return ((uint256(1) << 255) - 1) * 2;"),
        (BigUint::from(1u8) << 256u32) - BigUint::from(2u8)
    );
}

#[test]
fn unchecked_arithmetic_wraps() {
    assert_eq!(returns_uint("unchecked { return type(uint256).max + 1; }"), BigUint::from(0u8));
    assert_eq!(
        returns_uint("unchecked { return uint256(0) - 1; }"),
        u256_max()
    );
    assert_eq!(
        returns_uint("unchecked { return type(uint256).max * type(uint256).max; }"),
        BigUint::from(1u8)
    );
}

// ---------------- unsigned division / modulo ----------------

#[test]
fn unsigned_div_mod_including_large() {
    // max / 2 = 2^255 - 1 (signed DIV would give 0).
    assert_eq!(
        returns_uint("return type(uint256).max / 2;"),
        (BigUint::from(1u8) << 255u32) - BigUint::from(1u8)
    );
    assert_eq!(returns_uint("return type(uint256).max % 2;"), BigUint::from(1u8));
    // max / max = 1 ; max % max = 0.
    assert_eq!(returns_uint("return type(uint256).max / type(uint256).max;"), BigUint::from(1u8));
    assert_eq!(returns_uint("return type(uint256).max % type(uint256).max;"), BigUint::from(0u8));
    // large dividend, small divisor (the DeFi `balance / 1e18` shape).
    assert_eq!(
        returns_uint("return type(uint256).max / 1000000000000000000;"),
        ((BigUint::from(1u8) << 256u32) - BigUint::from(1u8)) / BigUint::from(1_000_000_000_000_000_000u64)
    );
    // small / large (big divisor branch) -> 0, remainder = dividend.
    assert_eq!(returns_uint("return uint256(5) / (uint256(1) << 255);"), BigUint::from(0u8));
    assert_eq!(returns_uint("return uint256(5) % (uint256(1) << 255);"), BigUint::from(5u8));
    // ordinary small values still correct.
    assert_eq!(returns_uint("return uint256(100) / 7;"), BigUint::from(14u8));
    assert_eq!(returns_uint("return uint256(100) % 7;"), BigUint::from(2u8));
}

#[test]
fn arith_scope_div_then_mul_overflow() {
    // (max/2 + 1) * 2 = 2^256 -> overflow Panic(0x11). Exercises div + checked mul.
    assert!(panics("return (type(uint256).max / 2 + 1) * 2;"));
}

// ---------------- the ERC-20 infinite-approval pattern ----------------

#[test]
fn erc20_infinite_approval_check() {
    // `require(allowance >= amount)` with allowance == type(uint256).max.
    assert!(returns_bool(
        "uint256 allowance = type(uint256).max; uint256 amount = 1000; return allowance >= amount;"
    ));
}


#[test]
fn unsigned_power_overflow_and_wrap() {
    // 2^200 and 2^255 fit in uint256.
    assert_eq!(returns_uint("uint256 b = 2; return b ** 200;"), BigUint::from(1u8) << 200u32);
    assert_eq!(returns_uint("uint256 b = 2; return b ** 255;"), BigUint::from(1u8) << 255u32);
    // 2^256 overflows uint256 -> Panic(0x11).
    assert!(panics("uint256 b = 2; return b ** 256;"));
    // unchecked wraps mod 2^256: 2^256 -> 0.
    assert_eq!(returns_uint("uint256 b = 2; unchecked { return b ** 256; }"), BigUint::from(0u8));
}

#[test]
fn unchecked_compound_uint256_wraps() {
    assert_eq!(returns_uint("uint256 x = type(uint256).max; unchecked { x += 5; } return x;"), BigUint::from(4u8));
    assert_eq!(returns_uint("uint256 x = 0; unchecked { x -= 1; } return x;"), u256_max());
    // loop counter reused as index still works (the old Buffer-index regression).
    assert_eq!(returns_uint("uint256[] memory a = new uint256[](3); for (uint256 i = 0; i < 3; i++) { a[i] = i; } return a[2];"), BigUint::from(2u8));
}
