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
        // ========== Arithmetic extensions ==========
        TestVector {
            name: "arithmetic_sub".to_string(),
            description: "Subtraction operation".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract ArithmeticSub {
                    function run() public pure returns (uint256) {
                        return 100 - 58;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        TestVector {
            name: "arithmetic_div".to_string(),
            description: "Integer division".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract ArithmeticDiv {
                    function run() public pure returns (uint256) {
                        return 84 / 2;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        TestVector {
            name: "arithmetic_mod".to_string(),
            description: "Modulo operation".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract ArithmeticMod {
                    function run() public pure returns (uint256) {
                        return 47 % 5;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(2),
            expected_success: true,
        },
        // ========== Conditionals ==========
        TestVector {
            name: "if_else_true_branch".to_string(),
            description: "If-else takes true branch".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract IfElseTrue {
                    function run() public pure returns (uint256) {
                        uint256 x = 10;
                        if (x > 5) {
                            return 1;
                        } else {
                            return 0;
                        }
                    }
                }
            "#
            .to_string(),
            expected_return: Some(1),
            expected_success: true,
        },
        TestVector {
            name: "ternary_operator".to_string(),
            description: "Ternary conditional expression".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract Ternary {
                    function run() public pure returns (uint256) {
                        uint256 x = 3;
                        return x > 2 ? 42 : 0;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        // ========== Loops ==========
        TestVector {
            name: "for_loop_sum".to_string(),
            description: "For loop summing 1..10".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract ForLoop {
                    function run() public pure returns (uint256) {
                        uint256 sum = 0;
                        for (uint256 i = 1; i <= 10; i++) {
                            sum += i;
                        }
                        return sum;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(55),
            expected_success: true,
        },
        TestVector {
            name: "while_loop".to_string(),
            description: "While loop counting down".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract WhileLoop {
                    function run() public pure returns (uint256) {
                        uint256 n = 10;
                        uint256 count = 0;
                        while (n > 0) {
                            n -= 1;
                            count += 1;
                        }
                        return count;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(10),
            expected_success: true,
        },
        TestVector {
            name: "do_while_loop".to_string(),
            description: "Do-while executes at least once".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract DoWhile {
                    function run() public pure returns (uint256) {
                        uint256 x = 0;
                        do {
                            x += 7;
                        } while (x < 5);
                        return x;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(7),
            expected_success: true,
        },
        // ========== Internal function calls ==========
        TestVector {
            name: "internal_call".to_string(),
            description: "Internal function call and return".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract InternalCall {
                    function double(uint256 x) internal pure returns (uint256) {
                        return x * 2;
                    }
                    function run() public pure returns (uint256) {
                        return double(21);
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        TestVector {
            name: "nested_calls".to_string(),
            description: "Nested internal function calls".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract NestedCalls {
                    function add(uint256 a, uint256 b) internal pure returns (uint256) {
                        return a + b;
                    }
                    function triple(uint256 x) internal pure returns (uint256) {
                        return add(x, add(x, x));
                    }
                    function run() public pure returns (uint256) {
                        return triple(14);
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        // ========== Bitwise operations ==========
        TestVector {
            name: "bitwise_and".to_string(),
            description: "Bitwise AND operation".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract BitwiseAnd {
                    function run() public pure returns (uint256) {
                        return 0xFF & 0x0F;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(15),
            expected_success: true,
        },
        TestVector {
            name: "bitwise_or".to_string(),
            description: "Bitwise OR operation".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract BitwiseOr {
                    function run() public pure returns (uint256) {
                        return 0x30 | 0x0A;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(58),
            expected_success: true,
        },
        TestVector {
            name: "bitwise_shift_left".to_string(),
            description: "Left shift operation".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract ShiftLeft {
                    function run() public pure returns (uint256) {
                        return 1 << 8;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(256),
            expected_success: true,
        },
        // ========== Type conversions ==========
        TestVector {
            name: "uint8_to_uint256".to_string(),
            description: "Widen uint8 to uint256".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract Widen {
                    function run() public pure returns (uint256) {
                        uint8 small = 42;
                        return uint256(small);
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        TestVector {
            name: "uint256_to_uint8".to_string(),
            description: "Narrow uint256 to uint8".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract Narrow {
                    function run() public pure returns (uint256) {
                        uint256 big = 42;
                        uint8 small = uint8(big);
                        return uint256(small);
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        TestVector {
            name: "signed_cast".to_string(),
            description: "Cast int256 to uint256".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract SignedCast {
                    function run() public pure returns (uint256) {
                        int256 s = 42;
                        return uint256(s);
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        // ========== Comparison operators ==========
        TestVector {
            name: "comparison_lt".to_string(),
            description: "Less-than comparison".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract CompLt {
                    function run() public pure returns (uint256) {
                        return 3 < 5 ? 1 : 0;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(1),
            expected_success: true,
        },
        TestVector {
            name: "comparison_eq".to_string(),
            description: "Equality comparison".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract CompEq {
                    function run() public pure returns (uint256) {
                        uint256 a = 42;
                        uint256 b = 42;
                        return a == b ? 1 : 0;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(1),
            expected_success: true,
        },
        // ========== Logical operators ==========
        TestVector {
            name: "logical_and".to_string(),
            description: "Logical AND short-circuit".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract LogicalAnd {
                    function run() public pure returns (uint256) {
                        bool a = true;
                        bool b = true;
                        return (a && b) ? 1 : 0;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(1),
            expected_success: true,
        },
        TestVector {
            name: "logical_or".to_string(),
            description: "Logical OR short-circuit".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract LogicalOr {
                    function run() public pure returns (uint256) {
                        bool a = false;
                        bool b = true;
                        return (a || b) ? 1 : 0;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(1),
            expected_success: true,
        },
        TestVector {
            name: "logical_not".to_string(),
            description: "Logical NOT operator".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract LogicalNot {
                    function run() public pure returns (uint256) {
                        bool a = false;
                        return (!a) ? 1 : 0;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(1),
            expected_success: true,
        },
        // ========== Local variables ==========
        TestVector {
            name: "local_variable_reassign".to_string(),
            description: "Local variable reassignment".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract LocalReassign {
                    function run() public pure returns (uint256) {
                        uint256 x = 10;
                        x = x + 20;
                        x = x + 12;
                        return x;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        TestVector {
            name: "nested_expression".to_string(),
            description: "Complex nested arithmetic".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract NestedExpr {
                    function run() public pure returns (uint256) {
                        return (2 + 3) * (4 + 4) + 2;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        // ========== Additional bitwise ==========
        TestVector {
            name: "bitwise_xor".to_string(),
            description: "Bitwise XOR operation".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract BitwiseXor {
                    function run() public pure returns (uint256) {
                        return 0xFF ^ 0xF0;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(15),
            expected_success: true,
        },
        TestVector {
            name: "bitwise_shift_right".to_string(),
            description: "Right shift operation".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract ShiftRight {
                    function run() public pure returns (uint256) {
                        return 256 >> 4;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(16),
            expected_success: true,
        },
        // ========== Compound assignment ==========
        TestVector {
            name: "compound_assignment".to_string(),
            description: "Compound assignment operators".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract CompoundAssign {
                    function run() public pure returns (uint256) {
                        uint256 x = 10;
                        x += 5;
                        x -= 3;
                        x *= 4;
                        return x;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(48),
            expected_success: true,
        },
        // ========== Break / continue ==========
        TestVector {
            name: "break_in_loop".to_string(),
            description: "Break exits loop early".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract BreakLoop {
                    function run() public pure returns (uint256) {
                        uint256 sum = 0;
                        for (uint256 i = 1; i <= 100; i++) {
                            if (i > 5) break;
                            sum += i;
                        }
                        return sum;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(15),
            expected_success: true,
        },
    ]
}
