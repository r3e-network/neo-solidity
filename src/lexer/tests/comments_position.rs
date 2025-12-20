// ==================== Comment Tests ====================

#[test]
fn test_line_comment() {
    let mut lexer = Lexer::new("// this is a comment\nfoo");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Identifier);
    assert_eq!(tokens[0].value, "foo");
}

#[test]
fn test_comment_at_end() {
    let mut lexer = Lexer::new("foo // comment");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Identifier);
}

// ==================== Position Tracking Tests ====================

#[test]
fn test_line_tracking() {
    let mut lexer = Lexer::new("a\nb\nc");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[1].line, 2);
    assert_eq!(tokens[2].line, 3);
}

#[test]
fn test_column_tracking() {
    let mut lexer = Lexer::new("abc def");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens[0].column, 1);
    assert_eq!(tokens[1].column, 5);
}

