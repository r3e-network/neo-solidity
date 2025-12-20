// ==================== Identifier Tests ====================

#[test]
fn test_simple_identifier() {
    let mut lexer = Lexer::new("foo");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Identifier);
    assert_eq!(tokens[0].value, "foo");
}

#[test]
fn test_identifier_with_underscore() {
    let mut lexer = Lexer::new("_bar");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Identifier);
    assert_eq!(tokens[0].value, "_bar");
}

#[test]
fn test_identifier_with_numbers() {
    let mut lexer = Lexer::new("baz123");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Identifier);
    assert_eq!(tokens[0].value, "baz123");
}

// ==================== Number Literal Tests ====================

#[test]
fn test_decimal_number() {
    let mut lexer = Lexer::new("42");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Literal);
    assert_eq!(tokens[0].value, "42");
}

#[test]
fn test_zero() {
    let mut lexer = Lexer::new("0");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Literal);
    assert_eq!(tokens[0].value, "0");
}

#[test]
fn test_hex_number_lowercase() {
    let mut lexer = Lexer::new("0xdeadbeef");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Literal);
    assert_eq!(tokens[0].value, "0xdeadbeef");
}

#[test]
fn test_hex_number_uppercase() {
    let mut lexer = Lexer::new("0XABCDEF");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Literal);
    assert_eq!(tokens[0].value, "0xABCDEF");
}

#[test]
fn test_hex_number_mixed_case() {
    let mut lexer = Lexer::new("0x123AbC");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Literal);
    assert_eq!(tokens[0].value, "0x123AbC");
}

// ==================== String Literal Tests ====================

#[test]
fn test_simple_string() {
    let mut lexer = Lexer::new("\"hello\"");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Literal);
    assert_eq!(tokens[0].value, "\"hello\"");
}

#[test]
fn test_empty_string() {
    let mut lexer = Lexer::new("\"\"");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Literal);
    assert_eq!(tokens[0].value, "\"\"");
}

#[test]
fn test_string_with_escape() {
    let mut lexer = Lexer::new("\"hello\\nworld\"");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Literal);
    assert_eq!(tokens[0].value, "\"hello\\nworld\"");
}

#[test]
fn test_unterminated_string() {
    let mut lexer = Lexer::new("\"hello");
    let result = lexer.tokenize();
    assert!(result.is_err());
}

