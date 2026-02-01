//! Optimizer unit tests.
//!
//! Tests optimization passes through compilation.

use neo_solidity::cli::compile_contracts;

#[test]
fn test_constant_folding() {
    let source = r#"
    // SPDX-License-Identifier: MIT
    pragma solidity ^0.8.0;
    contract Constants {
        uint256 constant X = 1 + 2;
        uint256 constant Y = 3 * 4;
    }
    "#;
    assert!(compile_contracts(source, false, 2).is_ok());
}

#[test]
fn test_optimization_levels() {
    let source = r#"
    // SPDX-License-Identifier: MIT
    pragma solidity ^0.8.0;
    contract Simple { uint256 x; }
    "#;

    for level in 0..=3 {
        assert!(compile_contracts(source, false, level).is_ok());
    }
}
