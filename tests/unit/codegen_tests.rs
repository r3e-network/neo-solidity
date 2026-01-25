//! Code generator unit tests.
//!
//! Tests code generation from Yul AST to bytecode.

use neo_solidity::{codegen::*, lexer::*, optimizer::*, parser::*, semantic::*, storage_key::*, CompilerConfig, CompilerError};

#[cfg(test)]
mod codegen_tests {
    use super::*;

    fn compile_yul(input: &str) -> Result<CompilationResult, CompilerError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        let config = CompilerConfig::default();
        let mut codegen = CodeGenerator::new(&config);
        codegen.generate(&ast)
    }

    #[test]
    fn test_simple_arithmetic() {
        let input = "{ let x := add(1, 2) }";
        let result = compile_yul(input).unwrap();

        assert!(!result.bytecode.is_empty());
        assert!(result.estimated_gas > 0);
    }

    #[test]
    fn test_function_compilation() {
        let input = r#"
        {
            function add_one(x) -> result {
                result := add(x, 1)
            }
            let y := add_one(5)
        }
        "#;
        let result = compile_yul(input).unwrap();

        assert!(!result.bytecode.is_empty());
        assert!(!result.assembly.is_empty());
    }
}
