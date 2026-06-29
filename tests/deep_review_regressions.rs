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
