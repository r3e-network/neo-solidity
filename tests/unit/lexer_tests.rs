//! Lexer unit tests.
//!
//! Tests tokenization of Yul source code into tokens.

use neo_solidity::{codegen::*, lexer::*, optimizer::*, parser::*, semantic::*, storage_key::*, CompilerConfig, CompilerError};

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let input = "{ let x := add(1, 2) }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert!(!tokens.is_empty());
        // Basic validation that tokenization works
    }

    #[test]
    fn test_number_literals() {
        let input = "0x42 123 0b1010";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert!(tokens.len() >= 3);
        // Basic validation that number parsing works
    }

    #[test]
    fn test_string_literals() {
        let input = r#""hello world" "test\"quote""#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert!(tokens.len() >= 2);
        // Basic validation that string parsing works
    }
}
