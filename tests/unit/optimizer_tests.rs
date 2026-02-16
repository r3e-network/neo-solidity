//! Optimizer unit tests.
//!
//! Tests optimization passes through compilation and direct AST manipulation.

use neo_solidity::cli::compile_contracts;
use neo_solidity::optimizer::Optimizer;
use neo_solidity::parser::{AstNode, AstNodeType};

// === Helper constructors ===

fn lit(v: &str) -> AstNode {
    AstNode::new(AstNodeType::Literal { value: v.to_string() }, 0, 0)
}

fn ident(n: &str) -> AstNode {
    AstNode::new(AstNodeType::Identifier { name: n.to_string() }, 0, 0)
}

fn call(name: &str, args: Vec<AstNode>) -> AstNode {
    AstNode::new(
        AstNodeType::FunctionCall { name: name.to_string(), arguments: args },
        0, 0,
    )
}

fn block(stmts: Vec<AstNode>) -> AstNode {
    AstNode::new(AstNodeType::Block { statements: stmts }, 0, 0)
}

fn object(stmts: Vec<AstNode>) -> AstNode {
    AstNode::new(AstNodeType::Object { statements: stmts }, 0, 0)
}

// === Integration tests via compile_contracts ===

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

// === Direct optimizer unit tests ===

#[test]
fn test_constant_folding_add() {
    let mut opt = Optimizer::new(1);
    // add(3, 4) → 7
    let ast = object(vec![call("add", vec![lit("3"), lit("4")])]);
    let result = opt.optimize(ast).unwrap();
    if let AstNodeType::Object { statements } = &result.node_type {
        assert_eq!(statements.len(), 1);
        if let AstNodeType::Literal { value } = &statements[0].node_type {
            assert_eq!(value, "7");
        } else {
            panic!("expected literal, got {:?}", statements[0].node_type);
        }
    } else {
        panic!("expected object");
    }
}

#[test]
fn test_constant_folding_nested() {
    let mut opt = Optimizer::new(1);
    // mul(add(2, 3), 4) → 20
    let ast = object(vec![call("mul", vec![
        call("add", vec![lit("2"), lit("3")]),
        lit("4"),
    ])]);
    let result = opt.optimize(ast).unwrap();
    if let AstNodeType::Object { statements } = &result.node_type {
        if let AstNodeType::Literal { value } = &statements[0].node_type {
            assert_eq!(value, "20");
        } else {
            panic!("expected literal after nested fold");
        }
    } else {
        panic!("expected object");
    }
}

#[test]
fn test_identity_mul_by_one() {
    let mut opt = Optimizer::new(1);
    // mul(x, 1) → x
    let ast = object(vec![call("mul", vec![ident("x"), lit("1")])]);
    let result = opt.optimize(ast).unwrap();
    if let AstNodeType::Object { statements } = &result.node_type {
        if let AstNodeType::Identifier { name } = &statements[0].node_type {
            assert_eq!(name, "x");
        } else {
            panic!("expected identifier 'x', got {:?}", statements[0].node_type);
        }
    } else {
        panic!("expected object");
    }
}

#[test]
fn test_identity_add_zero() {
    let mut opt = Optimizer::new(1);
    // add(0, y) → y
    let ast = object(vec![call("add", vec![lit("0"), ident("y")])]);
    let result = opt.optimize(ast).unwrap();
    if let AstNodeType::Object { statements } = &result.node_type {
        if let AstNodeType::Identifier { name } = &statements[0].node_type {
            assert_eq!(name, "y");
        } else {
            panic!("expected identifier 'y', got {:?}", statements[0].node_type);
        }
    } else {
        panic!("expected object");
    }
}

#[test]
fn test_identity_xor_self() {
    let mut opt = Optimizer::new(1);
    // xor(x, x) → 0
    let ast = object(vec![call("xor", vec![ident("x"), ident("x")])]);
    let result = opt.optimize(ast).unwrap();
    if let AstNodeType::Object { statements } = &result.node_type {
        if let AstNodeType::Literal { value } = &statements[0].node_type {
            assert_eq!(value, "0");
        } else {
            panic!("expected literal '0'");
        }
    } else {
        panic!("expected object");
    }
}

#[test]
fn test_strength_reduction_mul_pow2() {
    let mut opt = Optimizer::new(1);
    // mul(x, 8) → shl(3, x)
    let ast = object(vec![call("mul", vec![ident("x"), lit("8")])]);
    let result = opt.optimize(ast).unwrap();
    if let AstNodeType::Object { statements } = &result.node_type {
        if let AstNodeType::FunctionCall { name, arguments } = &statements[0].node_type {
            assert_eq!(name, "shl");
            assert_eq!(arguments.len(), 2);
            if let AstNodeType::Literal { value } = &arguments[0].node_type {
                assert_eq!(value, "3"); // log2(8) = 3
            } else {
                panic!("expected shift amount literal");
            }
            if let AstNodeType::Identifier { name } = &arguments[1].node_type {
                assert_eq!(name, "x");
            } else {
                panic!("expected identifier 'x'");
            }
        } else {
            panic!("expected shl call, got {:?}", statements[0].node_type);
        }
    } else {
        panic!("expected object");
    }
}

#[test]
fn test_strength_reduction_div_pow2() {
    let mut opt = Optimizer::new(1);
    // div(x, 16) → shr(4, x)
    let ast = object(vec![call("div", vec![ident("x"), lit("16")])]);
    let result = opt.optimize(ast).unwrap();
    if let AstNodeType::Object { statements } = &result.node_type {
        if let AstNodeType::FunctionCall { name, arguments } = &statements[0].node_type {
            assert_eq!(name, "shr");
            if let AstNodeType::Literal { value } = &arguments[0].node_type {
                assert_eq!(value, "4"); // log2(16) = 4
            } else {
                panic!("expected shift amount");
            }
        } else {
            panic!("expected shr call, got {:?}", statements[0].node_type);
        }
    } else {
        panic!("expected object");
    }
}

#[test]
fn test_strength_reduction_mod_pow2() {
    let mut opt = Optimizer::new(1);
    // mod(x, 4) → and(x, 3)
    let ast = object(vec![call("mod", vec![ident("x"), lit("4")])]);
    let result = opt.optimize(ast).unwrap();
    if let AstNodeType::Object { statements } = &result.node_type {
        if let AstNodeType::FunctionCall { name, arguments } = &statements[0].node_type {
            assert_eq!(name, "and");
            // Second arg should be mask = 4-1 = 3
            if let AstNodeType::Literal { value } = &arguments[1].node_type {
                assert_eq!(value, "3");
            } else {
                panic!("expected mask literal");
            }
        } else {
            panic!("expected and call, got {:?}", statements[0].node_type);
        }
    } else {
        panic!("expected object");
    }
}

#[test]
fn test_strength_reduction_non_pow2_unchanged() {
    let mut opt = Optimizer::new(1);
    // mul(x, 7) should NOT be strength-reduced (7 is not power of 2)
    let ast = object(vec![call("mul", vec![ident("x"), lit("7")])]);
    let result = opt.optimize(ast).unwrap();
    if let AstNodeType::Object { statements } = &result.node_type {
        if let AstNodeType::FunctionCall { name, .. } = &statements[0].node_type {
            assert_eq!(name, "mul"); // should remain mul
        } else {
            panic!("expected mul call to remain unchanged");
        }
    } else {
        panic!("expected object");
    }
}

#[test]
fn test_dead_code_after_return() {
    let mut opt = Optimizer::new(2);
    // block { leave; add(1,2) } → block { leave }
    let ast = object(vec![block(vec![
        AstNode::new(AstNodeType::Leave, 0, 0),
        call("add", vec![lit("1"), lit("2")]),
    ])]);
    let result = opt.optimize(ast).unwrap();
    if let AstNodeType::Object { statements } = &result.node_type {
        if let AstNodeType::Block { statements: inner } = &statements[0].node_type {
            assert_eq!(inner.len(), 1, "dead code after leave should be eliminated");
        } else {
            panic!("expected block");
        }
    } else {
        panic!("expected object");
    }
}

#[test]
fn test_cse_duplicate_expression() {
    let mut opt = Optimizer::new(3); // CSE enabled at O3
    // block { add(x, y); add(x, y) } → second should become identifier ref
    let ast = object(vec![block(vec![
        call("add", vec![ident("x"), ident("y")]),
        call("add", vec![ident("x"), ident("y")]),
    ])]);
    let result = opt.optimize(ast).unwrap();
    if let AstNodeType::Object { statements } = &result.node_type {
        if let AstNodeType::Block { statements: inner } = &statements[0].node_type {
            assert_eq!(inner.len(), 2);
            // Second expression should be replaced with a CSE temp variable
            assert!(
                matches!(&inner[1].node_type, AstNodeType::Identifier { name } if name.starts_with("_cse_")),
                "expected CSE temp variable for duplicate expression, got {:?}",
                inner[1].node_type
            );
        } else {
            panic!("expected block");
        }
    } else {
        panic!("expected object");
    }
}

#[test]
fn test_cse_different_expressions_unchanged() {
    let mut opt = Optimizer::new(3);
    // block { add(x, y); mul(x, y) } → both should remain as calls
    let ast = object(vec![block(vec![
        call("add", vec![ident("x"), ident("y")]),
        call("mul", vec![ident("x"), ident("y")]),
    ])]);
    let result = opt.optimize(ast).unwrap();
    if let AstNodeType::Object { statements } = &result.node_type {
        if let AstNodeType::Block { statements: inner } = &statements[0].node_type {
            assert!(matches!(&inner[0].node_type, AstNodeType::FunctionCall { name, .. } if name == "add"));
            assert!(matches!(&inner[1].node_type, AstNodeType::FunctionCall { name, .. } if name == "mul"));
        } else {
            panic!("expected block");
        }
    } else {
        panic!("expected object");
    }
}

#[test]
fn test_o0_no_optimization() {
    let mut opt = Optimizer::new(0);
    let ast = object(vec![call("add", vec![lit("3"), lit("4")])]);
    let result = opt.optimize(ast).unwrap();
    // At O0, add(3,4) should NOT be folded
    if let AstNodeType::Object { statements } = &result.node_type {
        assert!(matches!(&statements[0].node_type, AstNodeType::FunctionCall { name, .. } if name == "add"));
    } else {
        panic!("expected object");
    }
}

#[test]
fn test_optimizer_stats() {
    let mut opt = Optimizer::new(3);
    let ast = object(vec![
        call("add", vec![lit("1"), lit("2")]),
        call("mul", vec![ident("x"), lit("0")]),
    ]);
    let _ = opt.optimize(ast).unwrap();
    let stats = opt.get_stats();
    assert!(stats.passes_run > 0, "should have run at least one pass");
    assert!(stats.folded_constants > 0, "should have folded at least one constant");
}
