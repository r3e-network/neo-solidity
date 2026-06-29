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
