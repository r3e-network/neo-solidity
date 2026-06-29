//! Regression tests for the deep-review correctness fixes.
//!
//! Each test pins a behavior that previously miscompiled or faulted on a real
//! Neo node while passing in the in-tree simulator (which masked the bug).

use neo_devpack_solidity::cli::compile_contracts;
use neo_devpack_solidity::runtime::types::StackItem;
use neo_devpack_solidity::runtime::{NeoRuntime, RuntimeConfig};

/// #1 — An external function taking a struct parameter must reserve ONE arg
/// slot (the struct arrives as a single `Array` StackItem, per the manifest's
/// single `Array` parameter). Previously INITSLOT over-counted the flattened
/// field count, so a conformant single-`Array` call underflowed the frame and
/// faulted. Here the call passes one Array and must succeed with the right sum.
#[test]
fn external_struct_param_is_callable_with_single_array_arg() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    struct P { uint256 a; uint256 b; }
    function f(P memory p) external pure returns (uint256) { return p.a + p.b; }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
    let r = rt
        .call_method(
            &art.bytecode,
            &art.tokens,
            &art.manifest,
            "f",
            &[StackItem::array(vec![
                StackItem::Integer(11),
                StackItem::Integer(31),
            ])],
        )
        .expect("call");
    assert!(
        r.success,
        "struct-param external fn faulted: {:?}",
        r.exception.as_ref().map(|e| &e.message)
    );
    // 11 + 31 = 42.
    assert_eq!(r.return_data.first().copied(), Some(42u8), "sum must be 42");
}

/// #3 — A contract function declared without an explicit visibility specifier
/// is a hard error in Solidity 0.5.0+. Previously it silently defaulted to
/// `internal` and vanished from the ABI, producing a contract that "compiles"
/// but exposes none of its intended entrypoints.
#[test]
fn missing_visibility_specifier_is_a_hard_error() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract Tok {
    mapping(address => uint256) bal;
    function balanceOf(address a) returns (uint256) { return bal[a]; }
}"#;
    let err = compile_contracts(src, false, 2).expect_err("missing visibility must be rejected");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("visibility") || msg.contains("NO_VISIBILITY_SPECIFIER"),
        "diagnostic should mention the missing visibility; got: {msg}"
    );
}

/// #3 — explicit visibility still compiles and the method appears in the ABI.
#[test]
fn explicit_visibility_keeps_method_in_abi() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract Tok {
    mapping(address => uint256) bal;
    function balanceOf(address a) public view returns (uint256) { return bal[a]; }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let methods = arts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods");
    assert!(
        methods
            .iter()
            .any(|m| m["name"].as_str() == Some("balanceOf")),
        "balanceOf must appear in the ABI"
    );
}

/// #5 — `int256.min / -1` overflows. Native NeoVM DIV would fault UNCATCHABLY
/// (the quotient 2^255 needs 33 bytes); the compiler now pre-checks the pair.
/// In an `unchecked` block the result wraps to `int256.min` instead of faulting.
#[test]
fn int256_min_div_minus_one_unchecked_wraps_without_faulting() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    function f(int256 a, int256 b) external pure returns (int256) {
        unchecked { return a / b; }
    }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    // int256.min = -2^255 -> 32-byte little-endian two's complement: 0x80 in the
    // most-significant (last LE) byte, all others zero.
    let mut min_le = vec![0u8; 32];
    min_le[31] = 0x80;
    let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
    let r = rt
        .call_method(
            &art.bytecode,
            &art.tokens,
            &art.manifest,
            "f",
            &[StackItem::byte_array(min_le.clone()), StackItem::Integer(-1)],
        )
        .expect("call");
    assert!(
        r.success,
        "unchecked int256.min / -1 must not fault: {:?}",
        r.exception.as_ref().map(|e| &e.message)
    );
    // Result wraps to int256.min, whose 32-byte little-endian form is `min_le`.
    let mut got = r.return_data.clone();
    got.resize(32, 0);
    assert_eq!(got, min_le, "unchecked int256.min / -1 must wrap to int256.min");
}

/// #4 — `abi.encode` of a `bytesN` value should produce a correct fixed-width,
/// left-aligned, big-endian 32-byte slot regardless of how the value is backed.
/// KNOWN-FAILING (deep-review #4): Integer-backed `bytesN` (hex
/// literals/constants) are byte-reversed for N==32 and fault on `SIZE` for
/// N<32, while ByteArray-backed values (keccak output) are correct. Fixing it
/// needs the lowered value's backing tracked — see the limitation note in
/// `abi_encode.rs`. This test pins the TARGET behavior; un-ignore it once the
/// backing is tracked.
#[ignore = "deep-review #4: bytesN abi.encode reversal depends on Integer-vs-ByteArray backing"]
#[test]
fn abi_encode_bytesn_produces_correct_fixed_width_slot() {
    // `z` is a runtime arg so the multi-arg static-slot path is exercised
    // (and not constant-folded away); the `bytesN` constant flows through the
    // bytesN ABI-slot encoder under test.
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    bytes32 constant FULL  = 0x00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff;
    bytes32 constant TRAIL = 0xaabbccdd00000000000000000000000000000000000000000000000000000000;
    bytes4  constant SEL   = 0x01ffc9a7;
    function encFull(uint8 z)  external pure returns (bytes memory) { return abi.encode(z, FULL); }
    function encTrail(uint8 z) external pure returns (bytes memory) { return abi.encode(z, TRAIL); }
    function encSel(uint8 z)   external pure returns (bytes memory) { return abi.encode(z, SEL); }
    // keccak output is ByteArray-backed (already big-endian) — must round-trip
    // unchanged (no reversal).
    function encKeccak(bytes memory d) external pure returns (bytes memory) {
        return abi.encode(uint8(0), keccak256(d));
    }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];

    let call = |method: &str, arg: StackItem| -> Vec<u8> {
        let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
        let r = rt
            .call_method(&art.bytecode, &art.tokens, &art.manifest, method, &[arg])
            .expect("call");
        assert!(
            r.success,
            "{method} faulted: {:?}",
            r.exception.as_ref().map(|e| &e.message)
        );
        r.return_data
    };

    // Helper: bytes32 slot is bytes [32..64] of `abi.encode(uint8, bytes32)`.
    let hexb = |s: &str| hex::decode(s).unwrap();

    let full = call("encFull", StackItem::Integer(0));
    assert_eq!(full.len(), 64, "abi.encode(uint8,bytes32) is 64 bytes");
    assert_eq!(
        &full[32..64],
        hexb("00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff").as_slice(),
        "bytes32 (full) slot must be the value, big-endian, left-aligned"
    );

    let trail = call("encTrail", StackItem::Integer(0));
    assert_eq!(
        &trail[32..64],
        hexb("aabbccdd00000000000000000000000000000000000000000000000000000000").as_slice(),
        "bytes32 with trailing zeros must keep its full 32-byte width (was truncated)"
    );

    let sel = call("encSel", StackItem::Integer(0));
    let mut expected_sel = vec![0u8; 32];
    expected_sel[..4].copy_from_slice(&hexb("01ffc9a7"));
    assert_eq!(
        &sel[32..64],
        expected_sel.as_slice(),
        "bytes4 must be left-aligned + zero-padded to 32 bytes (was a SIZE fault)"
    );

    // keccak256("") = c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
    let kec = call("encKeccak", StackItem::byte_array(Vec::new()));
    assert_eq!(
        &kec[32..64],
        hexb("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470").as_slice(),
        "keccak256 (ByteArray-backed) slot must round-trip unchanged (no reversal)"
    );
}

/// #8 — uintN multiply (N >= 128) must not fault uncatchably: the full product
/// exceeds NeoVM's 32-byte integer. Checked overflow → catchable Panic(0x11);
/// unchecked → wraps mod 2^N. Verified for uint128 via runtime execution.
#[test]
fn uint128_mul_checked_panics_and_unchecked_wraps_without_faulting() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    function mc(uint128 a, uint128 b) external pure returns (uint128) { return a * b; }
    function mu(uint128 a, uint128 b) external pure returns (uint128) { unchecked { return a * b; } }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    // 2^e (e < 128) as a 16-byte little-endian uint128 argument; for e <= 100
    // the high bit is clear, so it's an unambiguous positive value.
    let pow2_u128 = |e: usize| -> StackItem {
        let mut le = vec![0u8; 16];
        le[e / 8] = 1u8 << (e % 8);
        StackItem::byte_array(le)
    };
    let runc = |m: &str, a: StackItem, b: StackItem| {
        let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
        rt.call_method(&art.bytecode, &art.tokens, &art.manifest, m, &[a, b])
            .expect("call")
    };
    // checked: 2^100 * 2^100 = 2^200 > 2^128-1 -> overflow Panic (not success).
    let r = runc("mc", pow2_u128(100), pow2_u128(100));
    assert!(!r.success, "checked uint128 overflow must Panic, not succeed");
    // checked: 3 * 5 = 15 fits.
    let r = runc("mc", StackItem::Integer(3), StackItem::Integer(5));
    assert!(
        r.success && r.return_data.first().copied() == Some(15),
        "checked 3*5 must be 15"
    );
    // unchecked: 2^100 * 2^100 = 2^200 -> mod 2^128 = 0, must NOT fault.
    let r = runc("mu", pow2_u128(100), pow2_u128(100));
    assert!(
        r.success,
        "unchecked uint128 mul must not fault: {:?}",
        r.exception.as_ref().map(|e| &e.message)
    );
    assert!(
        r.return_data.iter().all(|&x| x == 0),
        "2^100 * 2^100 mod 2^128 == 0"
    );
}

/// deep-review #4-delete — `delete` of a struct-element storage array must clear
/// the per-field element slots, not just the length. Previously only the length
/// slot was zeroed, so `ps[i].field` (which reads via the per-field slot and
/// skips the array bounds guard) resurrected the pre-delete value.
#[test]
fn delete_struct_element_array_clears_field_slots() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    struct P { uint256 a; uint256 b; }
    P[] ps;
    function test() external returns (uint256) {
        ps.push(P(7, 9));   // ps[0].a = 7
        delete ps;          // must clear the ps[0].a field slot too
        return ps[0].a;     // length 0; reads the field slot directly
    }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
    let r = rt
        .call_method(&art.bytecode, &art.tokens, &art.manifest, "test", &[])
        .expect("call");
    assert!(
        r.success,
        "test() faulted: {:?}",
        r.exception.as_ref().map(|e| &e.message)
    );
    // After `delete ps`, ps[0].a must read as the default 0, not the stale 7.
    assert!(
        r.return_data.iter().all(|&x| x == 0),
        "deleted struct-element field must read as 0, got {:?}",
        r.return_data
    );
}

/// deep-review #1-itoa (simulator) — `StdLib.itoa` must format the full
/// arbitrary-precision value. The old i64 decode truncated wide (>64-bit)
/// integers to their low 8 bytes, formatting the wrong number and giving false
/// confidence to any test asserting itoa output for large values.
#[test]
fn stdlib_itoa_formats_full_wide_integer() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
import "devpack/contracts/Syscalls.sol";
contract C {
    function f(int256 v) external view returns (bytes memory) { return bytes(Syscalls.itoa(v)); }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = arts
        .iter()
        .find(|a| a.metadata.name == "C")
        .expect("contract C");
    // 2^100 as a 32-byte little-endian int256 (positive: bit 100, high bit clear).
    let mut le = vec![0u8; 32];
    le[12] = 0x10; // bit 100 = byte 12, bit 4
    let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
    let r = rt
        .call_method(&art.bytecode, &art.tokens, &art.manifest, "f", &[StackItem::byte_array(le)])
        .expect("call");
    assert!(r.success, "itoa faulted: {:?}", r.exception.as_ref().map(|e| &e.message));
    assert_eq!(
        String::from_utf8_lossy(&r.return_data),
        "1267650600228229401496703205376", // 2^100 in decimal
        "itoa must format the full 2^100, not a low-8-byte truncation"
    );
}

/// deep-review bytesN comparison — a `bytesN` named constant is Integer-backed
/// (pushed little-endian) while a ByteArray-backed value (keccak/cast/storage/
/// param) is big-endian, so a raw EQUAL compared them byte-reversed and equal
/// values tested UNEQUAL (e.g. `require(role == ADMIN_ROLE)` silently failing).
/// Fixed for the common `runtime_value == CONSTANT` shape by canonicalizing the
/// constant to big-endian bytes when the other operand is not integer-backed.
/// (Indexing `b[i]` and bitwise on Integer-backed bytesN remain — see the
/// bytesN-backing note in `binary.rs` / the v0.25 memory.)
#[test]
fn bytesn_constant_compares_equal_to_runtime_value() {
    // A bytes32 param (ByteArray-backed) compared against a bytes32 constant
    // with the same value must be equal.
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    bytes32 constant ROLE  = 0x00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff;
    bytes32 constant ROLE2 = 0x00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff;
    function eq(bytes32 x) external pure returns (bool) { return x == ROLE; }
    function ne(bytes32 x) external pure returns (bool) { return x != ROLE; }
    function constEq() external pure returns (bool) { return ROLE == ROLE2; }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    let role_be =
        hex::decode("00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff").unwrap();
    let call1 = |m: &str, x: Vec<u8>| -> Vec<u8> {
        let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
        let r = rt
            .call_method(&art.bytecode, &art.tokens, &art.manifest, m, &[StackItem::byte_array(x)])
            .expect("call");
        assert!(r.success, "{m} faulted: {:?}", r.exception.as_ref().map(|e| &e.message));
        r.return_data
    };
    // runtime (ByteArray, big-endian) == CONSTANT (Integer-backed) for equal values.
    assert_eq!(call1("eq", role_be.clone()).first().copied(), Some(1u8), "x == ROLE must be true");
    // != is the logical negation.
    assert_eq!(call1("ne", role_be.clone()).first().copied(), Some(0u8), "x != ROLE must be false");
    // a DIFFERENT value must not be equal.
    let mut other = role_be.clone();
    other[0] ^= 0xff;
    assert_eq!(call1("eq", other).first().copied(), Some(0u8), "x == ROLE must be false for a different value");
    // CONSTANT == CONSTANT (both Integer-backed) must remain correct.
    let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
    let r = rt.call_method(&art.bytecode, &art.tokens, &art.manifest, "constEq", &[]).expect("call");
    assert!(r.success && r.return_data.first().copied() == Some(1u8), "ROLE == ROLE2 must stay true");
}
