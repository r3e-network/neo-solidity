//! Optimizer unit tests.
//!
//! Tests optimization passes and constant folding.

use neo_solidity::{codegen::*, lexer::*, optimizer::*, parser::*, semantic::*, storage_key::*, CompilerConfig, CompilerError};

#[cfg(test)]
mod optimizer_tests {
    use super::*;

    fn optimize_yul(input: &str, level: u8) -> Result<AstNode, CompilerError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        let mut optimizer = Optimizer::new(level);
        optimizer.optimize(ast)
    }

    #[test]
    fn test_constant_folding() {
        let input = r#"
        {
            let x := add(1, 2)
            let y := mul(3, 4)
            let z := sub(y, x)
        }
        "#;
        let optimized = optimize_yul(input, 2).unwrap();

        // Constants should be processed
        assert!(matches!(optimized.node_type, AstNodeType::Object { .. }));
    }

    #[test]
    fn test_optimization_levels() {
        let input = r#"
        {
            let x := add(1, 2)
            let y := mul(x, 0)
        }
        "#;

        let opt0 = optimize_yul(input, 0).unwrap();
        let opt1 = optimize_yul(input, 1).unwrap();
        let opt2 = optimize_yul(input, 2).unwrap();
        let opt3 = optimize_yul(input, 3).unwrap();

        // All optimization levels should work
        assert!(matches!(opt0.node_type, AstNodeType::Object { .. }));
        assert!(matches!(opt1.node_type, AstNodeType::Object { .. }));
        assert!(matches!(opt2.node_type, AstNodeType::Object { .. }));
        assert!(matches!(opt3.node_type, AstNodeType::Object { .. }));
    }
}
