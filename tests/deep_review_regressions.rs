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

/// #4 — `abi.encode` of a `bytesN` value must produce a correct fixed-width,
/// left-aligned, big-endian 32-byte slot regardless of how the value is backed.
/// Fixed: an Integer-backed `bytesN` static arg (hex literal / named constant)
/// is now resolved to its big-endian bytes and emitted as a left-aligned slot,
/// instead of the byte-reversed (N==32) / `SIZE`-faulting (N<32) encoder path;
/// ByteArray-backed values (keccak output) keep their correct path. (A bytesN
/// constant as a struct FIELD, or alongside a dynamic arg via the head/tail
/// path, still uses the encoder — rarer, see the abi_encode.rs note.)
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

/// deep-review bytesN indexing — `b[i]` on an Integer-backed `bytesN` (a hex
/// literal / named constant) indexed the little-endian byte span, returning the
/// byte from the wrong end. Solidity `b[0]` is the MOST-significant byte. Fixed
/// by canonicalizing the constant to big-endian bytes before PICKITEM.
#[test]
fn bytesn_constant_indexing_uses_solidity_byte_order() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    bytes32 constant ROLE = 0x00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff;
    function first() external pure returns (bytes1) { return ROLE[0]; }
    function last()  external pure returns (bytes1) { return ROLE[31]; }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    let call0 = |m: &str| -> Vec<u8> {
        let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
        let r = rt
            .call_method(&art.bytecode, &art.tokens, &art.manifest, m, &[])
            .expect("call");
        assert!(r.success, "{m} faulted: {:?}", r.exception.as_ref().map(|e| &e.message));
        r.return_data
    };
    // ROLE[0] is the most-significant byte 0x00; ROLE[31] is the least, 0xff.
    assert_eq!(call0("first"), vec![0x00u8], "ROLE[0] must be the MSB 0x00");
    assert_eq!(call0("last"), vec![0xffu8], "ROLE[31] must be the LSB 0xff");
}

/// deep-review bytesN bitwise — `&`/`|`/`^` between a runtime bytesN value
/// (big-endian) and an Integer-backed constant (little-endian) combined bytes
/// from opposite ends, producing a wrong mask. Fixed by canonicalizing the
/// constant operand to big-endian bytes (reducing it to the correct
/// runtime-op-runtime case).
#[test]
fn bytesn_bitwise_with_constant_mask_is_correct() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    bytes32 constant MASK = 0x00000000000000000000000000000000000000000000000000000000000000ff;
    function withConst(bytes32 x) external pure returns (bytes32) { return x & MASK; }
    function withRuntime(bytes32 x, bytes32 m) external pure returns (bytes32) { return x & m; }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    let x: Vec<u8> = (0..32u8).map(|i| 0xA0u8.wrapping_add(i)).collect(); // distinct bytes, BE
    let mut mask = vec![0u8; 32]; mask[31] = 0xff;
    let mut expected = vec![0u8; 32]; expected[31] = x[31]; // only last byte survives
    let runc = |m: &str, args: Vec<StackItem>| -> (bool, Vec<u8>) {
        let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
        let r = rt.call_method(&art.bytecode, &art.tokens, &art.manifest, m, &args).expect("call");
        (r.success, r.return_data)
    };
    let (rs, rr) = runc("withRuntime", vec![StackItem::byte_array(x.clone()), StackItem::byte_array(mask.clone())]);
    let mut rr32 = rr.clone(); rr32.resize(32, 0);
    assert!(rs && rr32 == expected, "runtime & runtime must mask correctly: got {}", hex::encode(&rr32));
    let (cs, cr) = runc("withConst", vec![StackItem::byte_array(x.clone())]);
    let mut cr32 = cr.clone(); cr32.resize(32, 0);
    assert!(cs && cr32 == expected, "runtime & CONSTANT must mask correctly (not byte-reversed): got {}", hex::encode(&cr32));
}

/// deep-review #2 (CRITICAL) — a modifier with an epilogue (e.g. a
/// `nonReentrant` guard's `locked = 0;` reset) applied to a VOID function must
/// still run the epilogue on an early `return;`. Previously the redirect was
/// only installed for functions with declared returns, so a bare `return;` in a
/// void guarded body emitted ReturnVoid directly and SKIPPED the epilogue —
/// leaving the guard engaged and bricking the contract.
#[test]
fn modifier_epilogue_runs_on_early_return_in_void_function() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    uint256 locked;
    uint256 public count;
    modifier guard() { require(locked == 0, "REENTRANT"); locked = 1; _; locked = 0; }
    function doit(bool early) external guard {
        count += 1;
        if (early) return;
        count += 10;
    }
    function lockedVal() external view returns (uint256) { return locked; }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
    // First call with an early return — the guard epilogue must reset `locked`.
    let r1 = rt
        .call_method(&art.bytecode, &art.tokens, &art.manifest, "doit", &[StackItem::Boolean(true)])
        .expect("call doit#1");
    assert!(r1.success, "doit(true)#1 must succeed: {:?}", r1.exception.as_ref().map(|e| &e.message));
    let lv = rt
        .call_method(&art.bytecode, &art.tokens, &art.manifest, "lockedVal", &[])
        .expect("call lockedVal");
    assert!(lv.return_data.iter().all(|&b| b == 0), "locked must be reset to 0 after early return");
    // Second call must NOT revert on require(locked == 0) — proves the guard reset.
    let r2 = rt
        .call_method(&art.bytecode, &art.tokens, &art.manifest, "doit", &[StackItem::Boolean(true)])
        .expect("call doit#2");
    assert!(r2.success, "doit(true)#2 must succeed (guard was reset), got fault: {:?}", r2.exception.as_ref().map(|e| &e.message));
}

/// deep-review #7 — a bare hex literal implicitly returned as `bytesN` (in a
/// multi-return tuple) must be LEFT-aligned in its 32-byte ABI slot (bytesN),
/// not RIGHT-aligned (uint-style). Previously the literal was encoded as a
/// uint, corrupting the returned tuple for any ABI decoder.
#[test]
fn multi_return_bytesn_literal_is_left_aligned() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    function f() external pure returns (bool, bytes4) { return (true, 0x11223344); }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
    let r = rt
        .call_method(&art.bytecode, &art.tokens, &art.manifest, "f", &[])
        .expect("call");
    assert!(r.success, "f faulted: {:?}", r.exception.as_ref().map(|e| &e.message));
    assert_eq!(r.return_data.len(), 64, "(bool, bytes4) ABI-encodes to 64 bytes");
    // slot 0: bool true, right-aligned.
    let mut expect_bool = vec![0u8; 32]; expect_bool[31] = 1;
    assert_eq!(&r.return_data[0..32], expect_bool.as_slice(), "bool slot");
    // slot 1: bytes4 0x11223344 LEFT-aligned (not 0x00..0011223344).
    let mut expect_b4 = vec![0u8; 32]; expect_b4[..4].copy_from_slice(&[0x11, 0x22, 0x33, 0x44]);
    assert_eq!(&r.return_data[32..64], expect_b4.as_slice(), "bytes4 must be left-aligned");
}

/// Regression for finding #7 (indexed-event variant): a bare hex-number literal
/// emitted as an `indexed bytesN` topic was ABI-encoded RIGHT-aligned (uint
/// style) by the generic static-slot encoder, producing a corrupt EVM topic.
/// `bytesN` topics must be LEFT-aligned. Verifies the fix in
/// `lower_emit_evm_shape`.
#[test]
fn indexed_event_bytesn_literal_topic_is_left_aligned() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    event Tag(bytes4 indexed t);
    function f() external { emit Tag(0x11223344); }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
    let r = rt
        .call_method(&art.bytecode, &art.tokens, &art.manifest, "f", &[])
        .expect("call");
    assert!(r.success, "f faulted: {:?}", r.exception.as_ref().map(|e| &e.message));
    let log = r.logs.first().expect("an event must be emitted");
    // topics[0] = keccak(signature); topics[1] = the indexed bytes4 arg.
    assert!(log.topics.len() >= 2, "expected topic0 + 1 indexed topic, got {}", log.topics.len());
    let mut expect = vec![0u8; 32]; expect[..4].copy_from_slice(&[0x11, 0x22, 0x33, 0x44]);
    assert_eq!(log.topics[1], expect, "indexed bytes4 topic must be left-aligned");
}

/// A `returns (bytes memory)` method returns the byte payload directly (the
/// NeoVM ByteString), so the returned `bytes` IS the value under test.
fn run_returns_bytes(src: &str) -> Vec<u8> {
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
    let r = rt
        .call_method(&art.bytecode, &art.tokens, &art.manifest, "f", &[])
        .expect("call");
    assert!(r.success, "f faulted: {:?}", r.exception.as_ref().map(|e| &e.message));
    r.return_data
}

/// #7b (struct-field variant): an integer-backed `bytesN` (hex literal) as a
/// STRUCT FIELD must be ABI-encoded LEFT-aligned. For N<32 the old encoder
/// FAULTed (SIZE on an Integer stack item); for N==32 it byte-reversed the
/// literal. Both are fixed by canonicalizing the literal to its big-endian
/// ByteArray at struct construction.
#[test]
fn struct_field_bytes4_literal_is_left_aligned() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    struct S { bytes4 b; uint256 x; }
    function f() external pure returns (bytes memory) { return abi.encode(S(0x01020304, 1)); }
}"#;
    let inner = run_returns_bytes(src);
    assert_eq!(inner.len(), 64, "struct (bytes4, uint256) encodes to 64 bytes");
    let mut expect_b = vec![0u8; 32]; expect_b[..4].copy_from_slice(&[0x01, 0x02, 0x03, 0x04]);
    assert_eq!(&inner[0..32], expect_b.as_slice(), "bytes4 field must be left-aligned");
    let mut expect_x = vec![0u8; 32]; expect_x[31] = 1;
    assert_eq!(&inner[32..64], expect_x.as_slice(), "uint256 field");
}

#[test]
fn struct_field_bytes32_literal_is_not_reversed() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    struct S { bytes32 b; uint256 x; }
    function f() external pure returns (bytes memory) {
        return abi.encode(S(0x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20, 1));
    }
}"#;
    let inner = run_returns_bytes(src);
    assert_eq!(inner.len(), 64);
    let expect_b: Vec<u8> = (1u8..=0x20).collect();
    assert_eq!(&inner[0..32], expect_b.as_slice(), "bytes32 field must be verbatim, not reversed");
}

/// #7c (array-element variant): an integer-backed `bytesN` assigned to a
/// dynamic-array element must encode LEFT-aligned, not fault.
#[test]
fn array_element_bytes4_literal_is_left_aligned() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    function f() external pure returns (bytes memory) {
        bytes4[] memory a = new bytes4[](1);
        a[0] = 0x01020304;
        return abi.encode(a);
    }
}"#;
    let inner = run_returns_bytes(src);
    // abi.encode(bytes4[]) = [offset 0x20][count 1][elem0 left-aligned 32B]
    assert!(inner.len() >= 96, "got {}", inner.len());
    let mut expect_e = vec![0u8; 32]; expect_e[..4].copy_from_slice(&[0x01, 0x02, 0x03, 0x04]);
    assert_eq!(&inner[64..96], expect_e.as_slice(), "array bytes4 element must be left-aligned");
}

/// #7d (encodePacked variant): a `bytesN` constant packed via
/// `abi.encodePacked` must emit exactly its N big-endian bytes, not the
/// little-endian integer backing (which was 8 reversed bytes).
#[test]
fn encode_packed_bytes4_constant_is_exact_width_be() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    bytes4 constant SEL = 0x01020304;
    function f() external pure returns (bytes memory) { return abi.encodePacked(SEL); }
}"#;
    let inner = run_returns_bytes(src);
    assert_eq!(inner, vec![0x01, 0x02, 0x03, 0x04], "packed bytes4 must be exactly 4 BE bytes");
}

/// #8 — `addmod(a, b, m)` must reduce the TRUE (up to 257-bit) sum mod m. The
/// old lowering used a native NeoVM Add, which adds the two 32-byte words as
/// signed two's-complement and discards the carry out of bit 255, so any sum
/// reaching 2^256 produced a wrong residue.
#[test]
fn addmod_reduces_true_sum_not_truncated_sum() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    function f(uint256 a, uint256 b, uint256 m) external pure returns (uint256) {
        return addmod(a, b, m);
    }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    // uint256 max = 2^256-1 — the 32-byte all-ones two's-complement word.
    let max = StackItem::byte_array(vec![0xFFu8; 32]);
    let call = |a: StackItem, b: StackItem, m: StackItem| -> Vec<u8> {
        let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
        let r = rt
            .call_method(&art.bytecode, &art.tokens, &art.manifest, "f", &[a, b, m])
            .expect("call");
        assert!(r.success, "addmod faulted: {:?}", r.exception.as_ref().map(|e| &e.message));
        r.return_data
    };
    // (2^256-1)+(2^256-1) = 2^257-2 ≡ 0 (mod 5). Truncated sum would give 4.
    let r = call(max.clone(), max.clone(), StackItem::Integer(5));
    assert!(r.iter().all(|&x| x == 0), "addmod(max,max,5) must be 0, got {r:?}");
    // (2^256-1)+1 = 2^256 ≡ 2 (mod 7). Truncated sum would give 0.
    let r = call(max.clone(), StackItem::Integer(1), StackItem::Integer(7));
    assert_eq!(r.first().copied(), Some(2), "addmod(max,1,7) must be 2, got {r:?}");
    // Non-overflowing control: 30 mod 7 == 2.
    let r = call(StackItem::Integer(10), StackItem::Integer(20), StackItem::Integer(7));
    assert_eq!(r.first().copied(), Some(2), "addmod(10,20,7) must be 2, got {r:?}");
    // m == 0 → 0 (Solidity addmod-by-zero returns 0, no panic).
    let r = call(StackItem::Integer(5), StackItem::Integer(5), StackItem::Integer(0));
    assert!(r.iter().all(|&x| x == 0), "addmod(_,_,0) must be 0, got {r:?}");
}

/// #10 — `intN(bytesN)` must reverse big-endian↔little-endian like `uintN(bytesN)`
/// does. The signed-cast branch omitted the reversal, so `int256(bytes32(1))`
/// decoded as 2^248 and diverged from `uint256(bytes32(1))`.
#[test]
fn int_cast_of_bytesn_reverses_like_uint_cast() {
    // int256(b) must equal uint256(b) reinterpreted, and both must equal 1 for
    // b = bytes32(uint256(1)).
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    function asInt(bytes32 b) external pure returns (int256) { return int256(b); }
    function asUint(bytes32 b) external pure returns (uint256) { return uint256(b); }
    function narrow(bytes4 b) external pure returns (int32) { return int32(b); }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    let call = |m: &str, arg: StackItem| -> (bool, Vec<u8>) {
        let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
        let r = rt
            .call_method(&art.bytecode, &art.tokens, &art.manifest, m, &[arg])
            .expect("call");
        (r.success, r.return_data)
    };
    // bytes32(uint256(1)) big-endian = 31 zero bytes then 0x01.
    let mut b32 = vec![0u8; 32];
    b32[31] = 1;
    let (ok, rd) = call("asInt", StackItem::byte_array(b32.clone()));
    assert!(ok, "int256(bytes32) must succeed");
    assert_eq!(le_u128(&rd), 1, "int256(bytes32(1)) must be 1, not 2^248");
    let (ok, rd) = call("asUint", StackItem::byte_array(b32));
    assert!(ok);
    assert_eq!(le_u128(&rd), 1, "uint256(bytes32(1)) must be 1 (control)");
    // bytes4 0xFFFFFFFF as int32 == -1.
    let (ok, rd) = call("narrow", StackItem::byte_array(vec![0xFFu8; 4]));
    assert!(ok, "int32(bytes4) must succeed");
    // -1 little-endian minimal form is 0xFF (one byte); interpret signed.
    let signed = if rd.is_empty() { 0i128 } else {
        let mut v = 0i128;
        for (i, &b) in rd.iter().take(16).enumerate() { v |= (b as i128) << (8 * i); }
        // sign-extend from the top set byte
        let bits = rd.len().min(16) * 8;
        if bits < 128 && (v >> (bits - 1)) & 1 == 1 { v |= -1i128 << bits; }
        v
    };
    assert_eq!(signed, -1, "int32(bytes4(0xFFFFFFFF)) must be -1, got {rd:?}");
}

/// #15 — a bare integer-backed `bytesN` literal/constant returned DIRECTLY
/// (`return 0x01020304;`) must come back as a big-endian ByteString matching the
/// manifest's `ByteArray` return type — not the little-endian Integer form.
/// (Real-node verified: pre-fix `return 0x01020304` returned Integer 16909060.)
#[test]
fn bare_bytesn_literal_return_is_bytestring() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    function b4() external pure returns (bytes4) { return 0x01020304; }
    bytes4 constant SEL = 0xaabbccdd;
    function sel() external pure returns (bytes4) { return SEL; }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    let call = |m: &str| -> Vec<u8> {
        let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
        let r = rt
            .call_method(&art.bytecode, &art.tokens, &art.manifest, m, &[])
            .expect("call");
        assert!(r.success, "{m} faulted: {:?}", r.exception.as_ref().map(|e| &e.message));
        r.return_data
    };
    // Big-endian ByteString [01,02,03,04] — NOT the LE Integer form [04,03,02,01].
    assert_eq!(call("b4"), vec![0x01, 0x02, 0x03, 0x04], "bytes4 literal return must be BE ByteString");
    assert_eq!(call("sel"), vec![0xaa, 0xbb, 0xcc, 0xdd], "bytes4 constant return must be BE ByteString");
}

/// #14 — try/catch must keep the NeoVM TRY frame balanced: a matched
/// non-fallback catch now exits via ENDTRY (not a JMP past it), and a `return`
/// inside a catch body pops the active frame before the RET. The in-repo
/// simulator tolerates an unbalanced frame, so these assert correct BEHAVIOR
/// (value + no fault) across all three catch exit shapes — the balance fix is
/// reasoned + structural (real C# NeoVM faults at RET with a live try-stack).
#[test]
fn try_catch_frame_balanced_across_exit_shapes() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    function willPanic() public pure returns (uint256) { uint256 a = 1; uint256 b = 0; return a / b; }
    function ok() public pure returns (uint256) { return 7; }
    // matched non-fallback catch with NO return (falls through, then returns).
    function fallThrough() external returns (uint256) {
        uint256 total = 0;
        try this.willPanic() returns (uint256 r) { total += r; }
        catch Panic(uint256 code) { total += code; }
        catch (bytes memory) { total += 100; }
        return total;
    }
    // matched non-fallback catch that RETURNS (return-in-catch).
    function returnInCatch() external returns (uint256) {
        try this.willPanic() returns (uint256 r) { return r; }
        catch Panic(uint256 code) { return code + 1; }
        catch (bytes memory) { return 100; }
    }
    // success path: handler returns (frame already popped on the success edge).
    function successReturns() external returns (uint256) {
        try this.ok() returns (uint256 r) { return r + 1; }
        catch { return 0; }
    }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = arts.iter().find(|a| a.metadata.name == "C").expect("C artifact");
    let call = |m: &str| -> (bool, Vec<u8>) {
        let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
        let r = rt
            .call_method(&art.bytecode, &art.tokens, &art.manifest, m, &[])
            .expect("call");
        (r.success, r.return_data)
    };
    // div-by-zero → Panic 0x12 (=18). fallThrough: total += 18 → 18.
    let (ok, rd) = call("fallThrough");
    assert!(ok, "fallThrough must succeed");
    assert_eq!(rd.first().copied(), Some(18), "fallThrough total must be 0x12 (got {rd:?})");
    // returnInCatch: code(18) + 1 = 19, returned from inside the catch.
    let (ok, rd) = call("returnInCatch");
    assert!(ok, "returnInCatch must succeed");
    assert_eq!(rd.first().copied(), Some(19), "returnInCatch must return code+1 (got {rd:?})");
    // successReturns: ok() returns 7 → handler returns 8.
    let (ok, rd) = call("successReturns");
    assert!(ok, "successReturns must succeed");
    assert_eq!(rd.first().copied(), Some(8), "successReturns must return 8 (got {rd:?})");
}

/// #13 — a side-effecting MODIFIER argument must be evaluated exactly ONCE.
/// Parameter substitution cloned the argument expression at every parameter
/// use, so `check(tick())` ran `tick()` once per `v` reference (counter ended
/// at 2 instead of 1).
#[test]
fn modifier_argument_evaluated_once() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    uint256 public counter;
    function tick() internal returns (uint256) { counter += 1; return counter; }
    modifier check(uint256 v) { require(v < 1000000, "x"); require(v < 1000000, "y"); _; }
    function run() public check(tick()) returns (uint256) { return counter; }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = arts.iter().find(|a| a.metadata.name == "C").expect("C artifact");
    let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
    let r = rt
        .call_method(&art.bytecode, &art.tokens, &art.manifest, "run", &[])
        .expect("call");
    assert!(r.success, "run faulted: {:?}", r.exception.as_ref().map(|e| &e.message));
    assert_eq!(
        r.return_data.first().copied(),
        Some(1),
        "modifier arg tick() must run exactly once (counter==1); got {:?}",
        r.return_data
    );
}

/// #13b — a side-effecting BASE-CONSTRUCTOR argument must be evaluated exactly
/// ONCE. `Base(side())` ran `side()` once per use of the base parameter (p=1
/// but q=2 instead of q=1).
#[test]
fn base_constructor_argument_evaluated_once() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract Base { uint256 public p; uint256 public q; constructor(uint256 v) { p = v; q = v; } }
contract Derived is Base {
    uint256 public counter;
    function side() internal returns (uint256) { counter += 1; return counter; }
    constructor() Base(side()) {}
    function pv() external view returns (uint256) { return p; }
    function qv() external view returns (uint256) { return q; }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = arts.iter().find(|a| a.metadata.name == "Derived").expect("Derived artifact");
    // The `_deploy` constructor auto-fires on each call (fresh rt → runs once),
    // setting p and q. With single-eval, side() runs once so p == q == 1; with
    // the bug, side() ran twice so q == 2.
    let getv = |m: &str| -> u8 {
        let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
        let r = rt
            .call_method(&art.bytecode, &art.tokens, &art.manifest, m, &[])
            .expect("call");
        assert!(r.success, "{m} faulted: {:?}", r.exception.as_ref().map(|e| &e.message));
        r.return_data.first().copied().unwrap_or(0)
    };
    assert_eq!(getv("pv"), 1, "p must be 1");
    assert_eq!(getv("qv"), 1, "q must be 1 — base ctor arg side() must run once, not per use");
}

/// #12 — storage `bytes` `.push(b)` / `.pop()` previously compiled to a SILENT
/// no-op (the lowering required `ValueType::Array`, bailed for `bytes`, and the
/// fallback dropped the args and wrote nothing). They must mutate the stored
/// ByteString and revert Panic(0x31) on empty-pop.
#[test]
fn storage_bytes_push_pop_mutates_and_underflows() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    bytes public data;
    function add(bytes1 b) external { data.push(b); }
    function popOne() external { data.pop(); }
    function len() external view returns (uint256) { return data.length; }
    function at(uint256 i) external view returns (bytes1) { return data[i]; }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = arts.iter().find(|a| a.metadata.name == "C").expect("C artifact");
    let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
    let call = |rt: &mut NeoRuntime, m: &str, args: &[StackItem]| -> (bool, Vec<u8>) {
        let r = rt
            .call_method(&art.bytecode, &art.tokens, &art.manifest, m, args)
            .expect("call");
        (r.success, r.return_data)
    };
    // push 0xAB → len 1, data[0] == 0xAB
    let (ok, _) = call(&mut rt, "add", &[StackItem::byte_array(vec![0xAB])]);
    assert!(ok, "push 0xAB must succeed");
    let (_, rd) = call(&mut rt, "len", &[]);
    assert_eq!(rd.first().copied(), Some(1), "len after one push must be 1 (was a silent no-op)");
    let (_, rd) = call(&mut rt, "at", &[StackItem::Integer(0)]);
    assert_eq!(rd, vec![0xAB], "data[0] must be the pushed byte");
    // push 0xCD → len 2, data[1] == 0xCD
    call(&mut rt, "add", &[StackItem::byte_array(vec![0xCD])]);
    let (_, rd) = call(&mut rt, "len", &[]);
    assert_eq!(rd.first().copied(), Some(2), "len after two pushes must be 2");
    let (_, rd) = call(&mut rt, "at", &[StackItem::Integer(1)]);
    assert_eq!(rd, vec![0xCD], "data[1] must be the second pushed byte");
    // pop → len 1
    let (ok, _) = call(&mut rt, "popOne", &[]);
    assert!(ok, "pop must succeed");
    let (_, rd) = call(&mut rt, "len", &[]);
    assert_eq!(rd.first().copied(), Some(1), "len after pop must be 1");
    // pop last → len 0
    call(&mut rt, "popOne", &[]);
    let (_, rd) = call(&mut rt, "len", &[]);
    assert!(rd.iter().all(|&x| x == 0), "len after popping all must be 0");
    // pop on empty → Panic(0x31) revert
    let (ok, _) = call(&mut rt, "popOne", &[]);
    assert!(!ok, "pop on empty bytes must revert Panic(0x31)");
}

/// #11 — a single-arg interface method named `transfer`/`send` (e.g.
/// `IToken(t).transfer(100)`) was hijacked into a GAS native value transfer
/// because interface handles infer as `Address`. It must lower to a
/// cross-contract CALL of the interface method instead. A plain
/// `payable(t).send(x)` must STILL be a GAS transfer.
#[test]
fn interface_send_is_contract_call_not_gas_native_send() {
    const GAS_LE: [u8; 20] = [
        0xcf, 0x76, 0xe2, 0x8b, 0xd0, 0x06, 0x2c, 0x4a, 0x47, 0x8e, 0xe3, 0x55, 0x61, 0x01, 0x13,
        0x19, 0xf3, 0xcf, 0xa4, 0xd2,
    ];
    let references_gas = |src: &str| -> bool {
        let arts = compile_contracts(src, false, 2).expect("compile");
        let art = arts.iter().find(|a| a.metadata.name == "C").expect("C artifact");
        art.bytecode.windows(20).any(|w| w == GAS_LE)
            || art.tokens.iter().any(|t| t.hash == GAS_LE)
    };
    let interface_form = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
interface IMail { function send(uint256 x) external returns (bool); }
contract C { function run(address t) external returns (bool) { return IMail(t).send(100); } }"#;
    let plain_form = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C { function run(address t) external returns (bool) { return payable(t).send(100); } }"#;
    assert!(
        !references_gas(interface_form),
        "IMail(t).send(x) must be a cross-contract call, not a GAS native transfer"
    );
    assert!(
        references_gas(plain_form),
        "payable(t).send(x) must remain a GAS native transfer (control)"
    );
}

/// Read `return_data` as a little-endian unsigned integer (for moderate values).
fn le_u128(rd: &[u8]) -> u128 {
    let mut v = 0u128;
    for (i, &b) in rd.iter().take(16).enumerate() {
        v |= (b as u128) << (8 * i);
    }
    v
}

/// #9 (int256 `**`): previously `int256` exponentiation fell through the
/// type-gating with NO overflow check (checked) and NO mod-2^256 wrap
/// (unchecked) — only uint256 and narrow widths were handled. Checked overflow
/// must Panic(0x11); unchecked must wrap; in-range powers must be exact.
#[test]
fn int256_pow_checked_panics_unchecked_wraps_inrange_exact() {
    let checked = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C { function f(int256 a, uint256 b) external pure returns (int256) { return a ** b; } }"#;
    let unchecked = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C { function f(int256 a, uint256 b) external pure returns (int256) { unchecked { return a ** b; } } }"#;
    let run = |src: &str, a: i64, b: i64| -> (bool, Vec<u8>) {
        let arts = compile_contracts(src, false, 2).expect("compile");
        let art = &arts[0];
        let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
        let r = rt
            .call_method(&art.bytecode, &art.tokens, &art.manifest, "f",
                &[StackItem::Integer(a), StackItem::Integer(b)])
            .expect("call");
        (r.success, r.return_data)
    };
    // in-range positive: 2**10 = 1024
    let (ok, rd) = run(checked, 2, 10);
    assert!(ok, "2**10 must succeed");
    assert_eq!(le_u128(&rd), 1024, "2**10 == 1024");
    // in-range negative base: (-2)**3 = -8 (must not fault)
    let (ok, _) = run(checked, -2, 3);
    assert!(ok, "(-2)**3 must succeed");
    // checked overflow: 2**255 > int256 max (2^255-1) -> Panic
    let (ok, _) = run(checked, 2, 255);
    assert!(!ok, "checked int256 2**255 must Panic (overflow), not return");
    // unchecked overflow: 2**255 wraps to int256.min, must NOT fault
    let (ok, _) = run(unchecked, 2, 255);
    assert!(ok, "unchecked int256 2**255 must wrap without faulting");
}

/// #16 — uint256 `**` now uses the soft-arith 256-bit multiply in the loop so
/// an overflowing intermediate never materializes a >32-byte integer (which
/// faults uncatchably on a real node). Validates the in-range result and the
/// unchecked mod-2^256 wrap (the catchable-Panic-vs-fault distinction is only
/// observable on a real node — see examples/test_neoxp_arith_smoke.sh).
#[test]
fn uint256_pow_soft_mul_in_range_and_unchecked_wrap() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    function p10() external pure returns (uint256) { uint256 b = 2; return b ** 10; }
    function wrap() external pure returns (uint256) { unchecked { uint256 b = 2; return b ** 256; } }
    function p1e18() external pure returns (uint256) { uint256 b = 10; return b ** 18; }
}"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    let call = |m: &str| -> Vec<u8> {
        let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
        let r = rt
            .call_method(&art.bytecode, &art.tokens, &art.manifest, m, &[])
            .expect("call");
        assert!(r.success, "{m} faulted: {:?}", r.exception.as_ref().map(|e| &e.message));
        r.return_data
    };
    assert_eq!(le_u128(&call("p10")), 1024, "2**10 == 1024 via soft-arith mul");
    assert!(call("wrap").iter().all(|&x| x == 0), "unchecked 2**256 wraps to 0");
    assert_eq!(le_u128(&call("p1e18")), 1_000_000_000_000_000_000, "10**18");
}

/// #9b (square-and-multiply restructure): the squaring-skip fix that avoids the
/// overflowing final squaring must not change any result value.
#[test]
fn pow_values_preserved_after_squaring_skip() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C { function f(uint256 b, uint256 e) external pure returns (uint256) { return b ** e; } }"#;
    let arts = compile_contracts(src, false, 2).expect("compile");
    let art = &arts[0];
    let run = |b: i64, e: i64| -> u128 {
        let mut rt = NeoRuntime::new(RuntimeConfig::default()).expect("rt");
        let r = rt
            .call_method(&art.bytecode, &art.tokens, &art.manifest, "f",
                &[StackItem::Integer(b), StackItem::Integer(e)])
            .expect("call");
        assert!(r.success, "{b}**{e} faulted: {:?}", r.exception.as_ref().map(|x| &x.message));
        le_u128(&r.return_data)
    };
    assert_eq!(run(2, 0), 1, "b**0 == 1");
    assert_eq!(run(7, 1), 7, "b**1 == b");
    assert_eq!(run(2, 10), 1024);
    assert_eq!(run(3, 5), 243);
    assert_eq!(run(2, 64), 1u128 << 64, "2**64");
    assert_eq!(run(10, 18), 10u128.pow(18), "10**18 (1e18, common token scale)");
}

/// Control: a `bytesN` local assigned from a hex literal, then `abi.encode`d.
/// Confirms whether local-variable binding already canonicalizes (informs the
/// scope of the fix).
#[test]
fn local_bytes4_literal_abi_encode_is_left_aligned() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract C {
    function f() external pure returns (bytes memory) {
        bytes4 x = 0x01020304;
        return abi.encode(x);
    }
}"#;
    let inner = run_returns_bytes(src);
    assert_eq!(inner.len(), 32, "abi.encode(bytes4) is one 32-byte slot");
    let mut expect = vec![0u8; 32]; expect[..4].copy_from_slice(&[0x01, 0x02, 0x03, 0x04]);
    assert_eq!(inner, expect, "local bytes4 must be left-aligned");
}
