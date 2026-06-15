//! Regression test for the compile-time diagnostic on uint256 literals that
//! exceed NeoVM's 32-byte signed-integer limit (#12).
//!
//! NeoVM integers are signed and capped at 32 bytes, so Solidity `uint256`
//! values in [2^255, 2^256-1] — notably `type(uint256).max` — currently emit a
//! 33-byte push that faults on a real node once the value is coerced into an
//! integer operation. The compiler now surfaces this as a WARNING at compile
//! time (instead of a silent on-chain fault) so developers learn of the
//! limitation before deploying. (Full support requires unsigned-aware uint256
//! lowering — see claudedocs/review-findings.md.)

use neo_devpack_solidity::cli::compile_contracts;

fn warns_about_neovm_limit(src: &str) -> bool {
    let arts = compile_contracts(src, false, 2).expect("compile");
    arts.iter().any(|a| {
        a.warnings
            .iter()
            .any(|w| w.message.contains("32-byte signed-integer limit"))
    })
}

#[test]
fn type_uint256_max_emits_compile_time_warning() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
contract C { function f() public pure returns (uint256) { return type(uint256).max; } }"#;
    assert!(
        warns_about_neovm_limit(src),
        "type(uint256).max must emit a compile-time NeoVM-integer-limit warning"
    );
}

#[test]
fn large_hex_literal_emits_compile_time_warning() {
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
contract C {
    function f() public pure returns (uint256) {
        return 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
    }
}"#;
    assert!(
        warns_about_neovm_limit(src),
        "a 2^256-1 hex literal must emit the NeoVM-integer-limit warning"
    );
}

#[test]
fn normal_uint256_values_do_not_warn() {
    // Values that fit a 32-byte signed integer (< 2^255) must NOT warn:
    // `type(uint128).max` (2^128-1) and an ordinary uint256 constant.
    let src = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
contract C {
    function f() public pure returns (uint128) { return type(uint128).max; }
    function g() public pure returns (uint256) { return 123456789; }
}"#;
    assert!(
        !warns_about_neovm_limit(src),
        "uint128.max / ordinary values fit 32 bytes and must not warn"
    );
}
