//! Code generator unit tests.
//!
//! Tests code generation through compilation.

use neo_solidity::cli::compile_contracts;

#[test]
fn test_simple_arithmetic() {
    let source = r#"
    // SPDX-License-Identifier: MIT
    pragma solidity ^0.8.0;
    contract Arith {
        function add(uint256 a, uint256 b) public pure returns (uint256) {
            return a + b;
        }
    }
    "#;
    assert!(compile_contracts(source, false, 2).is_ok());
}

#[test]
fn test_function_compilation() {
    let source = r#"
    // SPDX-License-Identifier: MIT
    pragma solidity ^0.8.0;
    contract Funcs {
        function addOne(uint256 x) public pure returns (uint256) {
            return x + 1;
        }
    }
    "#;
    assert!(compile_contracts(source, false, 2).is_ok());
}
