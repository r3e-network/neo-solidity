// ==================== Builtin Function Tests ====================

#[test]
fn test_arithmetic_builtins() {
    let builtins = ["add", "sub", "mul", "div", "mod"];
    for builtin in builtins {
        let mut lexer = Lexer::new(builtin);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
        assert_eq!(tokens[0].value, builtin);
    }
}

#[test]
fn test_comparison_builtins() {
    let builtins = ["eq", "lt", "gt", "iszero"];
    for builtin in builtins {
        let mut lexer = Lexer::new(builtin);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
    }
}

#[test]
fn test_bitwise_builtins() {
    let builtins = ["and", "or", "xor", "not", "shl", "shr", "sar"];
    for builtin in builtins {
        let mut lexer = Lexer::new(builtin);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
    }
}

#[test]
fn test_memory_builtins() {
    let builtins = ["mload", "mstore"];
    for builtin in builtins {
        let mut lexer = Lexer::new(builtin);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
    }
}

#[test]
fn test_storage_builtins() {
    let builtins = ["sload", "sstore"];
    for builtin in builtins {
        let mut lexer = Lexer::new(builtin);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
    }
}

#[test]
fn test_crypto_builtins() {
    let builtins = ["keccak256", "sha256", "ripemd160", "ecrecover"];
    for builtin in builtins {
        let mut lexer = Lexer::new(builtin);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
    }
}

#[test]
fn test_context_builtins() {
    let builtins = ["caller", "callvalue", "gas", "origin", "address", "balance"];
    for builtin in builtins {
        let mut lexer = Lexer::new(builtin);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
    }
}

#[test]
fn test_block_builtins() {
    let builtins = ["timestamp", "number", "blockhash", "coinbase"];
    for builtin in builtins {
        let mut lexer = Lexer::new(builtin);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
    }
}

#[test]
fn test_log_builtins() {
    let builtins = ["log0", "log1", "log2", "log3", "log4"];
    for builtin in builtins {
        let mut lexer = Lexer::new(builtin);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
    }
}

#[test]
fn test_call_builtins() {
    let builtins = ["call", "callcode", "delegatecall", "staticcall"];
    for builtin in builtins {
        let mut lexer = Lexer::new(builtin);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::BuiltinFunction);
    }
}

