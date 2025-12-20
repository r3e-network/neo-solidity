// ==================== Basic Token Tests ====================

#[test]
fn test_empty_input() {
    let mut lexer = Lexer::new("");
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.is_empty());
}

#[test]
fn test_whitespace_only() {
    let mut lexer = Lexer::new("   \t\n  ");
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.is_empty());
}

#[test]
fn test_braces() {
    let mut lexer = Lexer::new("{ }");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::LeftBrace);
    assert_eq!(tokens[1].token_type, TokenType::RightBrace);
}

#[test]
fn test_parentheses() {
    let mut lexer = Lexer::new("( )");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::RightParen);
}

#[test]
fn test_comma() {
    let mut lexer = Lexer::new(",");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Comma);
}

// ==================== Operator Tests ====================

#[test]
fn test_plus_operator() {
    let mut lexer = Lexer::new("+");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Plus);
}

#[test]
fn test_minus_operator() {
    let mut lexer = Lexer::new("-");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Minus);
}

#[test]
fn test_arrow_operator() {
    let mut lexer = Lexer::new("->");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Arrow);
    assert_eq!(tokens[0].value, "->");
}

#[test]
fn test_assignment_operator() {
    let mut lexer = Lexer::new(":=");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Assignment);
    assert_eq!(tokens[0].value, ":=");
}

// ==================== Keyword Tests ====================

#[test]
fn test_let_keyword() {
    let mut lexer = Lexer::new("let");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Let);
}

#[test]
fn test_if_keyword() {
    let mut lexer = Lexer::new("if");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::If);
}

#[test]
fn test_else_keyword() {
    let mut lexer = Lexer::new("else");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Else);
}

#[test]
fn test_for_keyword() {
    let mut lexer = Lexer::new("for");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::For);
}

#[test]
fn test_switch_keyword() {
    let mut lexer = Lexer::new("switch");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Switch);
}

#[test]
fn test_case_keyword() {
    let mut lexer = Lexer::new("case");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Case);
}

#[test]
fn test_default_keyword() {
    let mut lexer = Lexer::new("default");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Default);
}

#[test]
fn test_leave_keyword() {
    let mut lexer = Lexer::new("leave");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Leave);
}

#[test]
fn test_break_keyword() {
    let mut lexer = Lexer::new("break");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Break);
}

#[test]
fn test_continue_keyword() {
    let mut lexer = Lexer::new("continue");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Continue);
}

#[test]
fn test_function_keyword() {
    let mut lexer = Lexer::new("function");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Function);
}

