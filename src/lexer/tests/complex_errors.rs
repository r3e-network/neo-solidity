// ==================== Complex Expression Tests ====================

#[test]
fn test_variable_declaration() {
    let mut lexer = Lexer::new("let x := 42");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::Let);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Assignment);
    assert_eq!(tokens[3].token_type, TokenType::Literal);
}

#[test]
fn test_function_call() {
    let mut lexer = Lexer::new("add(1, 2)");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
    assert_eq!(tokens[1].token_type, TokenType::LeftParen);
    assert_eq!(tokens[2].token_type, TokenType::Literal);
    assert_eq!(tokens[3].token_type, TokenType::Comma);
    assert_eq!(tokens[4].token_type, TokenType::Literal);
    assert_eq!(tokens[5].token_type, TokenType::RightParen);
}

#[test]
fn test_function_definition() {
    // function foo(a, b) -> result { }
    // tokens: function, foo, (, a, ,, b, ), ->, result, {, }
    let mut lexer = Lexer::new("function foo(a, b) -> result { }");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 11);
    assert_eq!(tokens[0].token_type, TokenType::Function);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[7].token_type, TokenType::Arrow);
}

#[test]
fn test_if_statement() {
    let mut lexer = Lexer::new("if eq(x, 0) { leave }");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens[0].token_type, TokenType::If);
    assert_eq!(tokens[1].token_type, TokenType::BuiltinFunction);
    assert!(tokens.iter().any(|t| t.token_type == TokenType::Leave));
}

#[test]
fn test_for_loop() {
    let mut lexer = Lexer::new("for { let i := 0 } lt(i, 10) { i := add(i, 1) } { }");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens[0].token_type, TokenType::For);
    assert!(tokens.iter().any(|t| t.token_type == TokenType::Let));
    assert!(tokens
        .iter()
        .any(|t| t.token_type == TokenType::BuiltinFunction && t.value == "lt"));
}

#[test]
fn test_switch_statement() {
    let mut lexer = Lexer::new("switch x case 0 { } case 1 { } default { }");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens[0].token_type, TokenType::Switch);
    assert_eq!(
        tokens
            .iter()
            .filter(|t| t.token_type == TokenType::Case)
            .count(),
        2
    );
    assert!(tokens.iter().any(|t| t.token_type == TokenType::Default));
}

// ==================== Error Handling Tests ====================

#[test]
fn test_unexpected_character() {
    let mut lexer = Lexer::new("@");
    let result = lexer.tokenize();
    assert!(result.is_err());
}

#[test]
fn test_error_position() {
    let mut lexer = Lexer::new("foo @");
    let result = lexer.tokenize();
    assert!(result.is_err());
    if let Err(e) = result {
        let msg = format!("{e}");
        assert!(msg.contains("line"));
        assert!(msg.contains("column"));
    }
}

