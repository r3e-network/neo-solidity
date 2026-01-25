//! Integration tests for full compilation pipeline.
//!
//! Tests end-to-end compilation from Yul source to bytecode.

use neo_solidity::{codegen::*, lexer::*, optimizer::*, parser::*, semantic::*, storage_key::*, CompilerConfig, CompilerError};

#[cfg(test)]
mod integration_tests {
    use super::*;

    fn full_compile_test(input: &str) -> Result<CompilationResult, CompilerError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        let config = CompilerConfig::default();
        let mut code_generator = CodeGenerator::new(&config);
        code_generator.generate(&ast)
    }

    #[test]
    fn test_erc20_like_contract() {
        let input = r#"
        {
            function balanceOf(account) -> balance {
                let slot := add(1, account)
                balance := sload(slot)
            }

            function transfer(to, amount) -> success {
                let sender := caller()
                let sender_balance := balanceOf(sender)

                if lt(sender_balance, amount) {
                    success := 0
                    leave
                }

                success := 1
            }
        }
        "#;

        let result = full_compile_test(input).unwrap();
        assert!(!result.bytecode.is_empty());
        assert!(result.estimated_gas > 0);
    }

    #[test]
    fn test_complex_control_flow() {
        let input = r#"
        {
            function fibonacci(n) -> result {
                if lt(n, 2) {
                    result := n
                    leave
                }

                let a := 0
                let b := 1
                for { let i := 2 } lt(i, add(n, 1)) { i := add(i, 1) } {
                    let temp := add(a, b)
                    a := b
                    b := temp
                }
                result := b
            }
        }
        "#;

        let result = full_compile_test(input).unwrap();
        assert!(!result.bytecode.is_empty());
        assert!(result.estimated_gas > 100);
    }

    #[test]
    fn test_optimization_effectiveness() {
        let input = r#"
        {
            function unoptimized_example() -> result {
                let a := add(1, 2)
                let b := add(1, 2)
                let c := mul(a, 1)
                let d := add(c, 0)
                result := d
            }
        }
        "#;

        let result = full_compile_test(input).unwrap();
        assert!(!result.bytecode.is_empty());
        assert!(result.estimated_gas < 1000);
    }
}
