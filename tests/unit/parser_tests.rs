//! Parser unit tests.
//!
//! Tests parsing of Yul source code into AST nodes.

use neo_solidity::{codegen::*, lexer::*, optimizer::*, parser::*, semantic::*, storage_key::*, CompilerConfig, CompilerError};

#[cfg(test)]
mod parser_tests {
    use super::*;

    fn parse_yul(input: &str) -> Result<AstNode, CompilerError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_simple_block() {
        let input = "{ let x := 1 }";
        let ast = parse_yul(input).unwrap();

        match ast.node_type {
            AstNodeType::Object { statements: _ } => {
                // Object parsed successfully
            }
            _ => panic!("Expected Object node"),
        }
    }

    #[test]
    fn test_function_definition() {
        let input = r#"
        {
            function add(a, b) -> result {
                result := add(a, b)
            }
        }
        "#;
        let ast = parse_yul(input);
        assert!(ast.is_ok(), "parser error: {:?}", ast.err());
    }
}
