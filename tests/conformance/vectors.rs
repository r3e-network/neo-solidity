//! Built-in conformance test vectors.
//!
//! Provides predefined test vectors for basic compiler validation.

use super::infrastructure::TestVector;

/// Built-in test vectors for basic validation
pub fn basic_test_vectors() -> Vec<TestVector> {
    vec![
        TestVector {
            name: "simple_return".to_string(),
            description: "Simple function that returns a constant".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract SimpleReturn {
                    function run() public pure returns (uint256) {
                        return 42;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        TestVector {
            name: "arithmetic_add".to_string(),
            description: "Basic addition operation".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract ArithmeticAdd {
                    function run() public pure returns (uint256) {
                        return 10 + 32;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        TestVector {
            name: "arithmetic_mul".to_string(),
            description: "Basic multiplication operation".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract ArithmeticMul {
                    function run() public pure returns (uint256) {
                        return 6 * 7;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        TestVector {
            name: "boolean_true".to_string(),
            description: "Boolean true returns 1".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract BooleanTrue {
                    function run() public pure returns (uint256) {
                        bool b = true;
                        if (b) return 1;
                        return 0;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(1),
            expected_success: true,
        },
        TestVector {
            name: "boolean_false".to_string(),
            description: "Boolean false returns 0".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract BooleanFalse {
                    function run() public pure returns (uint256) {
                        bool b = false;
                        if (b) return 1;
                        return 0;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(0),
            expected_success: true,
        },
    ]
}
