//! Semantic analyzer unit tests.
//!
//! Tests semantic analysis including variable scoping, function validation, and type checking.

use neo_solidity::{codegen::*, lexer::*, optimizer::*, parser::*, semantic::*, storage_key::*, CompilerConfig, CompilerError};

#[cfg(test)]
mod semantic_tests {
    use super::*;

    fn analyze_yul(input: &str) -> Result<SemanticResult, CompilerError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.analyze(&ast)
    }

    #[test]
    fn test_variable_scoping() {
        let input = r#"
        {
            let x := 1
            {
                let y := x
                let x := 2
            }
        }
        "#;
        let result = analyze_yul(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_validation() {
        let input = r#"
        {
            function f(a, b) -> x, y {
                x := a
                y := b
            }
            let a, b := f(1, 2)
        }
        "#;
        let result = analyze_yul(input);
        assert!(result.is_ok());
    }
}
