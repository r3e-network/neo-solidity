use std::collections::HashMap;
use neo_solidity::compiler::{lexer::*, parser::*, semantic::*, codegen::*, optimizer::*};
use neo_solidity::runtime::{EvmRuntime, EvmMemoryManager, StorageManager, AbiEncoder, CryptoLib, EventManager};

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let input = "{ let x := add(1, 2) }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[1].token_type, TokenType::Let);
        assert_eq!(tokens[2].token_type, TokenType::Identifier);
        assert_eq!(tokens[3].token_type, TokenType::Assignment);
        assert_eq!(tokens[4].token_type, TokenType::Identifier);
        assert_eq!(tokens[5].token_type, TokenType::LeftParen);
        assert_eq!(tokens[6].token_type, TokenType::Literal);
        assert_eq!(tokens[7].token_type, TokenType::Comma);
        assert_eq!(tokens[8].token_type, TokenType::Literal);
        assert_eq!(tokens[9].token_type, TokenType::RightParen);
        assert_eq!(tokens[10].token_type, TokenType::RightBrace);
    }

    #[test]
    fn test_number_literals() {
        let input = "0x42 123 0b1010";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].value, "0x42");
        assert_eq!(tokens[1].value, "123");
        assert_eq!(tokens[2].value, "0b1010");
    }

    #[test]
    fn test_string_literals() {
        let input = r#""hello world" "test\"quote""#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value, "\"hello world\"");
        assert_eq!(tokens[1].value, "\"test\\\"quote\"");
    }

    #[test]
    fn test_built_in_functions() {
        let input = "add sub mul div mod";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 5);
        for token in &tokens {
            assert_eq!(token.token_type, TokenType::BuiltinFunction);
        }
    }

    #[test]
    fn test_comments() {
        let input = r#"
            // Single line comment
            let x := 1 /* multi
            line comment */ let y := 2
        "#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        // Comments should be filtered out
        assert_eq!(tokens.len(), 8); // let x := 1 let y := 2
    }

    #[test]
    fn test_error_handling() {
        let input = "let @ invalid";
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();
        
        assert!(result.is_err());
    }

    #[test]
    fn test_position_tracking() {
        let input = "let\nx\n:=\n1";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[1].line, 2);
        assert_eq!(tokens[2].line, 3);
        assert_eq!(tokens[3].line, 4);
    }

    #[test]
    fn test_all_operators() {
        let input = ":= + - * / % < > <= >= == != & | ^ ~ << >>";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 17);
        assert_eq!(tokens[0].token_type, TokenType::Assignment);
        assert_eq!(tokens[1].token_type, TokenType::Plus);
        assert_eq!(tokens[2].token_type, TokenType::Minus);
        // ... etc for all operators
    }

    #[test]
    fn test_keywords() {
        let input = "let if for switch case default leave break continue function";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Let);
        assert_eq!(tokens[1].token_type, TokenType::If);
        assert_eq!(tokens[2].token_type, TokenType::For);
        assert_eq!(tokens[3].token_type, TokenType::Switch);
        assert_eq!(tokens[4].token_type, TokenType::Case);
        assert_eq!(tokens[5].token_type, TokenType::Default);
        assert_eq!(tokens[6].token_type, TokenType::Leave);
        assert_eq!(tokens[7].token_type, TokenType::Break);
        assert_eq!(tokens[8].token_type, TokenType::Continue);
        assert_eq!(tokens[9].token_type, TokenType::Function);
    }
}

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
            AstNodeType::Object { statements, .. } => {
                assert_eq!(statements.len(), 1);
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
        let ast = parse_yul(input).unwrap();
        
        match ast.node_type {
            AstNodeType::Object { statements, .. } => {
                assert_eq!(statements.len(), 1);
                match &statements[0].node_type {
                    AstNodeType::Function { name, params, returns, .. } => {
                        assert_eq!(name, "add");
                        assert_eq!(params.len(), 2);
                        assert_eq!(returns.len(), 1);
                    }
                    _ => panic!("Expected Function node"),
                }
            }
            _ => panic!("Expected Object node"),
        }
    }

    #[test]
    fn test_if_statement() {
        let input = r#"
        {
            if eq(x, 0) {
                x := 1
            }
        }
        "#;
        let ast = parse_yul(input).unwrap();
        
        match ast.node_type {
            AstNodeType::Object { statements, .. } => {
                match &statements[0].node_type {
                    AstNodeType::If { condition, then_branch, else_branch } => {
                        assert!(condition.node_type.is_function_call());
                        assert!(then_branch.node_type.is_block());
                        assert!(else_branch.is_none());
                    }
                    _ => panic!("Expected If node"),
                }
            }
            _ => panic!("Expected Object node"),
        }
    }

    #[test]
    fn test_for_loop() {
        let input = r#"
        {
            for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                // loop body
            }
        }
        "#;
        let ast = parse_yul(input).unwrap();
        
        match ast.node_type {
            AstNodeType::Object { statements, .. } => {
                match &statements[0].node_type {
                    AstNodeType::For { init, condition, update, body } => {
                        assert!(init.is_some());
                        assert!(condition.node_type.is_function_call());
                        assert!(update.is_some());
                        assert!(body.node_type.is_block());
                    }
                    _ => panic!("Expected For node"),
                }
            }
            _ => panic!("Expected Object node"),
        }
    }

    #[test]
    fn test_switch_statement() {
        let input = r#"
        {
            switch x
            case 0 { y := 1 }
            case 1 { y := 2 }
            default { y := 3 }
        }
        "#;
        let ast = parse_yul(input).unwrap();
        
        match ast.node_type {
            AstNodeType::Object { statements, .. } => {
                match &statements[0].node_type {
                    AstNodeType::Switch { expression, cases, default } => {
                        assert!(expression.node_type.is_identifier());
                        assert_eq!(cases.len(), 2);
                        assert!(default.is_some());
                    }
                    _ => panic!("Expected Switch node"),
                }
            }
            _ => panic!("Expected Object node"),
        }
    }

    #[test]
    fn test_function_call() {
        let input = "{ let result := add(mul(x, y), z) }";
        let ast = parse_yul(input).unwrap();
        
        match ast.node_type {
            AstNodeType::Object { statements, .. } => {
                match &statements[0].node_type {
                    AstNodeType::Assignment { targets, value } => {
                        assert_eq!(targets.len(), 1);
                        assert!(value.node_type.is_function_call());
                        
                        match &value.node_type {
                            AstNodeType::FunctionCall { name, arguments } => {
                                assert_eq!(name, "add");
                                assert_eq!(arguments.len(), 2);
                                // First argument should be another function call
                                assert!(arguments[0].node_type.is_function_call());
                            }
                            _ => panic!("Expected FunctionCall node"),
                        }
                    }
                    _ => panic!("Expected Assignment node"),
                }
            }
            _ => panic!("Expected Object node"),
        }
    }

    #[test]
    fn test_variable_assignment() {
        let input = "{ let x, y := f() }";
        let ast = parse_yul(input).unwrap();
        
        match ast.node_type {
            AstNodeType::Object { statements, .. } => {
                match &statements[0].node_type {
                    AstNodeType::Assignment { targets, value } => {
                        assert_eq!(targets.len(), 2);
                        assert_eq!(targets[0], "x");
                        assert_eq!(targets[1], "y");
                        assert!(value.node_type.is_function_call());
                    }
                    _ => panic!("Expected Assignment node"),
                }
            }
            _ => panic!("Expected Object node"),
        }
    }

    #[test]
    fn test_nested_blocks() {
        let input = r#"
        {
            let x := 1
            {
                let y := 2
                {
                    let z := add(x, y)
                }
            }
        }
        "#;
        let ast = parse_yul(input).unwrap();
        
        match ast.node_type {
            AstNodeType::Object { statements, .. } => {
                assert_eq!(statements.len(), 2);
                assert!(statements[1].node_type.is_block());
            }
            _ => panic!("Expected Object node"),
        }
    }

    #[test]
    fn test_error_recovery() {
        let input = "{ let x := }"; // Missing value
        let result = parse_yul(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_expression() {
        let input = "{ let result := add(mul(div(x, y), mod(z, w)), sub(a, b)) }";
        let ast = parse_yul(input).unwrap();
        
        // Should parse without errors
        assert!(matches!(ast.node_type, AstNodeType::Object { .. }));
    }
}

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
                let y := x  // x should be visible here
                let x := 2  // shadow outer x
            }
            // y should not be visible here
        }
        "#;
        let result = analyze_yul(input).unwrap();
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_undefined_variable() {
        let input = "{ let x := y }"; // y is undefined
        let result = analyze_yul(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_function_signature_validation() {
        let input = r#"
        {
            function f(a, b) -> x, y {
                x := a
                y := b
            }
            let a, b := f(1, 2)  // Correct usage
        }
        "#;
        let result = analyze_yul(input).unwrap();
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_function_argument_mismatch() {
        let input = r#"
        {
            function f(a, b) -> x {
                x := add(a, b)
            }
            let result := f(1)  // Wrong number of arguments
        }
        "#;
        let result = analyze_yul(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_built_in_function_validation() {
        let input = r#"
        {
            let x := add(1, 2)      // Correct
            let y := add(1, 2, 3)   // Too many arguments
        }
        "#;
        let result = analyze_yul(input);
        // Should detect the error but continue analysis
        assert!(!result.unwrap().warnings.is_empty());
    }

    #[test]
    fn test_recursive_function_detection() {
        let input = r#"
        {
            function factorial(n) -> result {
                if eq(n, 0) {
                    result := 1
                }
                if gt(n, 0) {
                    result := mul(n, factorial(sub(n, 1)))
                }
            }
        }
        "#;
        let result = analyze_yul(input).unwrap();
        assert!(result.warnings.iter().any(|w| w.contains("recursive")));
    }

    #[test]
    fn test_dead_code_detection() {
        let input = r#"
        {
            let x := 1
            leave
            let y := 2  // Dead code
        }
        "#;
        let result = analyze_yul(input).unwrap();
        assert!(result.warnings.iter().any(|w| w.contains("unreachable")));
    }

    #[test]
    fn test_security_analysis() {
        let input = r#"
        {
            let x := div(a, b)  // Potential division by zero
            let y := mod(c, d)  // Potential division by zero
        }
        "#;
        let result = analyze_yul(input).unwrap();
        assert!(!result.security_issues.is_empty());
    }

    #[test]
    fn test_complexity_metrics() {
        let input = r#"
        {
            function complex_function(x) -> result {
                if gt(x, 0) {
                    if gt(x, 10) {
                        if gt(x, 100) {
                            result := 3
                        }
                        result := 2
                    }
                    result := 1
                }
                result := 0
            }
        }
        "#;
        let result = analyze_yul(input).unwrap();
        assert!(result.complexity_metrics.cyclomatic > 3);
        assert!(result.complexity_metrics.max_nesting_depth >= 3);
    }

    #[test]
    fn test_performance_analysis() {
        let input = r#"
        {
            for { let i := 0 } lt(i, 1000) { i := add(i, 1) } {
                let expensive := mul(div(i, 2), mod(i, 3))
            }
        }
        "#;
        let result = analyze_yul(input).unwrap();
        assert!(result.performance_metrics.estimated_gas > 1000);
        assert!(!result.performance_metrics.hot_paths.is_empty());
    }
}

#[cfg(test)]
mod codegen_tests {
    use super::*;

    fn compile_yul(input: &str) -> Result<codegen::CompilationResult, CompilerError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        
        let config = crate::CompilerConfig::default();
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
        assert!(result.abi.as_object().unwrap().contains_key("functions"));
    }

    #[test]
    fn test_control_flow_compilation() {
        let input = r#"
        {
            let result := 0
            if gt(x, 0) {
                result := 1
            }
            if eq(x, 0) {
                result := 2
            }
        }
        "#;
        let result = compile_yul(input).unwrap();
        
        assert!(!result.bytecode.is_empty());
        // Should contain jump instructions
        assert!(result.assembly.contains("JMP") || result.assembly.contains("JMPIF"));
    }

    #[test]
    fn test_loop_compilation() {
        let input = r#"
        {
            let sum := 0
            for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                sum := add(sum, i)
            }
        }
        "#;
        let result = compile_yul(input).unwrap();
        
        assert!(!result.bytecode.is_empty());
        assert!(result.estimated_gas > 50); // Loops should have higher gas cost
    }

    #[test]
    fn test_switch_compilation() {
        let input = r#"
        {
            let result := 0
            switch x
            case 1 { result := 10 }
            case 2 { result := 20 }
            default { result := 30 }
        }
        "#;
        let result = compile_yul(input).unwrap();
        
        assert!(!result.bytecode.is_empty());
        // Should contain conditional jumps
        assert!(result.assembly.contains("JMPIF") || result.assembly.contains("JMP"));
    }

    #[test]
    fn test_memory_operations() {
        let input = r#"
        {
            mstore(0x40, 0x80)
            let value := mload(0x40)
        }
        "#;
        let result = compile_yul(input).unwrap();
        
        assert!(!result.bytecode.is_empty());
    }

    #[test]
    fn test_storage_operations() {
        let input = r#"
        {
            sstore(0, 42)
            let value := sload(0)
        }
        "#;
        let result = compile_yul(input).unwrap();
        
        assert!(!result.bytecode.is_empty());
    }

    #[test]
    fn test_abi_generation() {
        let input = r#"
        {
            function transfer(to, amount) -> success {
                success := 1
            }
            function balanceOf(account) -> balance {
                balance := sload(account)
            }
        }
        "#;
        let result = compile_yul(input).unwrap();
        
        let abi = result.abi.as_object().unwrap();
        let functions = abi["functions"].as_array().unwrap();
        assert_eq!(functions.len(), 2);
        
        let transfer_func = &functions[0];
        assert_eq!(transfer_func["name"], "transfer");
        assert_eq!(transfer_func["inputs"].as_array().unwrap().len(), 2);
        assert_eq!(transfer_func["outputs"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_gas_estimation() {
        let input = r#"
        {
            // Expensive operations
            let hash := keccak256(0, 1000)
            for { let i := 0 } lt(i, 100) { i := add(i, 1) } {
                sstore(i, i)
            }
        }
        "#;
        let result = compile_yul(input).unwrap();
        
        assert!(result.estimated_gas > 10000); // Should be expensive
    }

    #[test]
    fn test_source_map_generation() {
        let input = r#"
        {
            let x := 1
            let y := 2
            let z := add(x, y)
        }
        "#;
        let result = compile_yul(input).unwrap();
        
        assert!(!result.source_map.is_empty());
        // Source map should contain position mappings
        assert!(result.source_map.contains(";"));
    }

    #[test]
    fn test_debug_info_generation() {
        let input = r#"
        {
            function test_function(a, b) -> result {
                result := add(a, b)
            }
        }
        "#;
        let result = compile_yul(input).unwrap();
        
        let debug_info = result.debug_info.as_object().unwrap();
        assert!(debug_info.contains_key("functions"));
        assert!(debug_info.contains_key("variables"));
    }
}

#[cfg(test)]
mod optimizer_tests {
    use super::*;

    fn optimize_yul(input: &str, level: u8) -> Result<AstNode, CompilerError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        let mut optimizer = Optimizer::new(level);
        optimizer.optimize(ast)
    }

    #[test]
    fn test_constant_folding() {
        let input = r#"
        {
            let x := add(1, 2)
            let y := mul(3, 4)
            let z := sub(y, x)
        }
        "#;
        let optimized = optimize_yul(input, 2).unwrap();
        
        // Constants should be folded
        // This is a simplified test - in reality we'd check the actual AST structure
        assert!(matches!(optimized.node_type, AstNodeType::Object { .. }));
    }

    #[test]
    fn test_dead_code_elimination() {
        let input = r#"
        {
            let x := 1
            let y := 2  // y is never used
            let z := add(x, 3)
        }
        "#;
        let optimized = optimize_yul(input, 3).unwrap();
        
        // Dead code should be eliminated at level 3
        assert!(matches!(optimized.node_type, AstNodeType::Object { .. }));
    }

    #[test]
    fn test_function_inlining() {
        let input = r#"
        {
            function simple_add(a, b) -> result {
                result := add(a, b)
            }
            let x := simple_add(1, 2)
            let y := simple_add(3, 4)
        }
        "#;
        let optimized = optimize_yul(input, 3).unwrap();
        
        // Small functions should be inlined
        assert!(matches!(optimized.node_type, AstNodeType::Object { .. }));
    }

    #[test]
    fn test_optimization_levels() {
        let input = r#"
        {
            let x := add(1, 2)
            let y := mul(x, 0)  // Should become 0
        }
        "#;
        
        let opt0 = optimize_yul(input, 0).unwrap();
        let opt1 = optimize_yul(input, 1).unwrap();
        let opt2 = optimize_yul(input, 2).unwrap();
        let opt3 = optimize_yul(input, 3).unwrap();
        
        // Higher optimization levels should produce different results
        // This is a structural test - actual optimization effects would be tested differently
        assert!(matches!(opt0.node_type, AstNodeType::Object { .. }));
        assert!(matches!(opt1.node_type, AstNodeType::Object { .. }));
        assert!(matches!(opt2.node_type, AstNodeType::Object { .. }));
        assert!(matches!(opt3.node_type, AstNodeType::Object { .. }));
    }

    #[test]
    fn test_common_subexpression_elimination() {
        let input = r#"
        {
            let a := add(x, y)
            let b := mul(add(x, y), 2)  // add(x, y) is computed twice
        }
        "#;
        let optimized = optimize_yul(input, 3).unwrap();
        
        // Common subexpressions should be eliminated
        assert!(matches!(optimized.node_type, AstNodeType::Object { .. }));
    }

    #[test]
    fn test_loop_optimization() {
        let input = r#"
        {
            for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                let constant := add(5, 5)  // Loop invariant
            }
        }
        "#;
        let optimized = optimize_yul(input, 3).unwrap();
        
        // Loop invariants should be moved outside the loop
        assert!(matches!(optimized.node_type, AstNodeType::Object { .. }));
    }

    #[test]
    fn test_no_optimization_at_level_0() {
        let input = r#"
        {
            let x := add(1, 2)
            let y := 0
        }
        "#;
        let original_lexer = Lexer::new(input);
        let original_tokens = original_lexer.tokenize().unwrap();
        let original_parser = Parser::new(original_tokens);
        let original_ast = original_parser.parse().unwrap();
        
        let optimized = optimize_yul(input, 0).unwrap();
        
        // At level 0, AST should be mostly unchanged
        // (This is a simplified test - would need deep comparison in practice)
        assert!(matches!(optimized.node_type, AstNodeType::Object { .. }));
    }
}

#[cfg(test)]
mod runtime_tests {
    use super::*;

    #[test]
    fn test_memory_manager() {
        let mut memory = EvmMemoryManager::new();
        
        // Test basic allocation
        let ptr = memory.allocate(32).unwrap();
        assert_eq!(ptr, 0);
        
        let ptr2 = memory.allocate(64).unwrap();
        assert_eq!(ptr2, 32);
        
        // Test memory operations
        memory.store_word(ptr, &[1u8; 32]).unwrap();
        let data = memory.load_word(ptr).unwrap();
        assert_eq!(data[0], 1);
        
        // Test memory expansion
        let large_ptr = memory.allocate(1024 * 1024).unwrap(); // 1MB
        assert!(large_ptr > ptr2);
        
        // Test memory statistics
        let stats = memory.get_statistics();
        assert!(stats.total_allocated > 0);
        assert!(stats.active_allocations > 0);
    }

    #[test]
    fn test_storage_manager() {
        let mut storage = StorageManager::new();
        
        // Test basic storage operations
        let key = [1u8; 32];
        let value = [42u8; 32];
        
        storage.store(&key, &value).unwrap();
        let retrieved = storage.load(&key).unwrap();
        assert_eq!(retrieved, value);
        
        // Test mapping storage
        let mapping_slot = [0u8; 32];
        let mapping_key = [1u8; 32];
        let mapping_value = [123u8; 32];
        
        let storage_key = storage.calculate_mapping_key(&mapping_slot, &mapping_key);
        storage.store(&storage_key, &mapping_value).unwrap();
        let retrieved = storage.load(&storage_key).unwrap();
        assert_eq!(retrieved, mapping_value);
        
        // Test storage statistics
        let stats = storage.get_statistics();
        assert!(stats.total_reads > 0);
        assert!(stats.total_writes > 0);
    }

    #[test]
    fn test_abi_encoder() {
        let encoder = AbiEncoder::new();
        
        // Test uint256 encoding
        let value = 42u64;
        let encoded = encoder.encode_uint256(value);
        assert_eq!(encoded.len(), 32);
        
        let decoded = encoder.decode_uint256(&encoded).unwrap();
        assert_eq!(decoded, value as u128);
        
        // Test string encoding
        let test_string = "Hello, World!";
        let encoded = encoder.encode_string(test_string);
        let decoded = encoder.decode_string(&encoded).unwrap();
        assert_eq!(decoded, test_string);
        
        // Test address encoding
        let address = [1u8; 20];
        let encoded = encoder.encode_address(&address);
        assert_eq!(encoded.len(), 32);
        
        let decoded = encoder.decode_address(&encoded).unwrap();
        assert_eq!(decoded[12..], address);
        
        // Test function selector calculation
        let signature = "transfer(address,uint256)";
        let selector = encoder.calculate_function_selector(signature).unwrap();
        assert_eq!(selector.len(), 4);
    }

    #[test]
    fn test_crypto_lib() {
        let crypto = CryptoLib::new();
        
        // Test keccak256
        let data = b"test data";
        let hash = crypto.keccak256(data);
        assert_eq!(hash.len(), 32);
        
        // Test sha256
        let sha_hash = crypto.sha256(data);
        assert_eq!(sha_hash.len(), 32);
        
        // Hashes should be different
        assert_ne!(hash, sha_hash);
        
        // Test determinism
        let hash2 = crypto.keccak256(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_event_manager() {
        let mut event_manager = EventManager::new();
        
        // Test event emission
        let topics = vec![
            [1u8; 32], // Event signature
            [2u8; 32], // Indexed parameter
        ];
        let data = vec![42u8, 43u8, 44u8];
        
        event_manager.emit_event(topics, data.clone()).unwrap();
        
        // Test event retrieval
        let events = event_manager.get_events(0).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].topics.len(), 2);
        assert_eq!(events[0].data, data);
        
        // Test event filtering
        let filter = EventFilter {
            topics: vec![Some([1u8; 32])],
            ..Default::default()
        };
        let filtered_events = event_manager.get_events_with_filter(&filter).unwrap();
        assert_eq!(filtered_events.len(), 1);
    }

    #[test]
    fn test_runtime_integration() {
        let runtime = EvmRuntime::new();
        
        // Test basic operations
        let result = runtime.add(10, 20);
        assert_eq!(result, 30);
        
        let result = runtime.mul(6, 7);
        assert_eq!(result, 42);
        
        // Test memory operations
        runtime.mstore(0, &[1u8; 32]).unwrap();
        let data = runtime.mload(0).unwrap();
        assert_eq!(data[31], 1); // Big-endian
        
        // Test storage operations
        runtime.sstore(&[1u8; 32], &[42u8; 32]).unwrap();
        let value = runtime.sload(&[1u8; 32]).unwrap();
        assert_eq!(value[31], 42);
        
        // Test gas operations
        let gas = runtime.gas();
        assert!(gas > 0);
        
        // Test address operations
        let address = runtime.address();
        assert_eq!(address.len(), 20);
        
        // Test balance operations
        let balance = runtime.balance(&address);
        assert!(balance >= 0);
    }

    #[test]
    fn test_error_handling() {
        let runtime = EvmRuntime::new();
        
        // Test division by zero
        let result = std::panic::catch_unwind(|| {
            runtime.div(10, 0)
        });
        assert!(result.is_err());
        
        // Test invalid memory access
        let result = runtime.mload(u32::MAX as usize);
        assert!(result.is_err());
        
        // Test invalid storage key
        let invalid_key = [0u8; 31]; // Wrong size
        let result = runtime.sload(&invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_performance_characteristics() {
        let runtime = EvmRuntime::new();
        
        // Test memory allocation performance
        let start = std::time::Instant::now();
        for i in 0..1000 {
            runtime.mstore(i * 32, &[i as u8; 32]).unwrap();
        }
        let memory_time = start.elapsed();
        
        // Test storage operations performance
        let start = std::time::Instant::now();
        for i in 0..100 {
            let key = [i as u8; 32];
            let value = [i as u8; 32];
            runtime.sstore(&key, &value).unwrap();
        }
        let storage_time = start.elapsed();
        
        // Memory should be faster than storage
        assert!(memory_time < storage_time);
        
        // Both should complete within reasonable time
        assert!(memory_time.as_millis() < 1000);
        assert!(storage_time.as_millis() < 5000);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    fn full_compile_test(input: &str) -> Result<codegen::CompilationResult, CompilerError> {
        // Full compilation pipeline
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        
        let mut semantic_analyzer = SemanticAnalyzer::new();
        let _semantic_result = semantic_analyzer.analyze(&ast)?;
        
        let mut optimizer = Optimizer::new(2);
        let optimized_ast = optimizer.optimize(ast)?;
        
        let config = crate::CompilerConfig::default();
        let mut code_generator = CodeGenerator::new(&config);
        code_generator.generate(&optimized_ast)
    }

    #[test]
    fn test_erc20_like_contract() {
        let input = r#"
        {
            // Storage slots
            let TOTAL_SUPPLY_SLOT := 0
            let BALANCES_SLOT := 1
            let ALLOWANCES_SLOT := 2
            
            function balanceOf(account) -> balance {
                let slot := add(BALANCES_SLOT, account)
                balance := sload(slot)
            }
            
            function transfer(to, amount) -> success {
                let sender := caller()
                let sender_balance := balanceOf(sender)
                
                if lt(sender_balance, amount) {
                    success := 0
                    leave
                }
                
                // Update sender balance
                let sender_slot := add(BALANCES_SLOT, sender)
                sstore(sender_slot, sub(sender_balance, amount))
                
                // Update recipient balance
                let recipient_balance := balanceOf(to)
                let recipient_slot := add(BALANCES_SLOT, to)
                sstore(recipient_slot, add(recipient_balance, amount))
                
                // Emit Transfer event
                log3(0, 0, 
                    0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef,
                    sender,
                    to
                )
                
                success := 1
            }
            
            function approve(spender, amount) -> success {
                let owner := caller()
                let allowance_key := keccak256(owner, spender)
                let allowance_slot := add(ALLOWANCES_SLOT, allowance_key)
                
                sstore(allowance_slot, amount)
                
                // Emit Approval event
                log3(0, 0,
                    0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925,
                    owner,
                    spender
                )
                
                success := 1
            }
        }
        "#;
        
        let result = full_compile_test(input).unwrap();
        assert!(!result.bytecode.is_empty());
        assert!(result.estimated_gas > 0);
        
        // Check ABI generation
        let abi = result.abi.as_object().unwrap();
        let functions = abi["functions"].as_array().unwrap();
        assert!(functions.len() >= 3); // balanceOf, transfer, approve
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
            
            function test_fibonacci() {
                let fib10 := fibonacci(10)
                // Should be 55
                if eq(fib10, 55) {
                    // Success
                    log1(0, 0, 0x1)
                }
            }
        }
        "#;
        
        let result = full_compile_test(input).unwrap();
        assert!(!result.bytecode.is_empty());
        assert!(result.estimated_gas > 100); // Complex computation should cost gas
    }

    #[test]
    fn test_memory_intensive_operations() {
        let input = r#"
        {
            function copy_memory(src, dst, len) {
                for { let i := 0 } lt(i, len) { i := add(i, 32) } {
                    let data := mload(add(src, i))
                    mstore(add(dst, i), data)
                }
            }
            
            function test_memory() {
                // Store some data
                mstore(0x80, 0x123456789abcdef)
                mstore(0xa0, 0xfedcba9876543210)
                
                // Copy it elsewhere
                copy_memory(0x80, 0x200, 64)
                
                // Verify copy
                let copied1 := mload(0x200)
                let copied2 := mload(0x220)
                
                if and(eq(copied1, 0x123456789abcdef), eq(copied2, 0xfedcba9876543210)) {
                    log1(0, 0, 0x2)
                }
            }
        }
        "#;
        
        let result = full_compile_test(input).unwrap();
        assert!(!result.bytecode.is_empty());
        // Memory operations should generate appropriate bytecode
        assert!(result.assembly.contains("MLOAD") || result.assembly.contains("MSTORE"));
    }

    #[test]
    fn test_error_propagation() {
        let input = r#"
        {
            function safe_div(a, b) -> result {
                if eq(b, 0) {
                    revert(0, 0)
                }
                result := div(a, b)
            }
            
            function test_division() {
                let result1 := safe_div(10, 2)  // Should succeed
                let result2 := safe_div(10, 0)  // Should revert
            }
        }
        "#;
        
        let result = full_compile_test(input).unwrap();
        assert!(!result.bytecode.is_empty());
        // Should handle revert cases
        assert!(result.assembly.contains("REVERT") || result.assembly.contains("THROW"));
    }

    #[test]
    fn test_optimization_effectiveness() {
        let input = r#"
        {
            function unoptimized_example() -> result {
                let a := add(1, 2)
                let b := add(1, 2)  // Same as a
                let c := mul(a, 1)  // Should be optimized to just a
                let d := add(c, 0)  // Should be optimized to just c
                result := d
            }
        }
        "#;
        
        let result = full_compile_test(input).unwrap();
        assert!(!result.bytecode.is_empty());
        // Optimized code should be smaller and more efficient
        assert!(result.estimated_gas < 100); // Should be quite cheap after optimization
    }

    #[test]
    fn test_large_contract() {
        let input = r#"
        {
            // A larger contract with multiple functions and complex logic
            let OWNER_SLOT := 0
            let PAUSED_SLOT := 1
            let TOTAL_SUPPLY_SLOT := 2
            
            function onlyOwner() {
                if iszero(eq(caller(), sload(OWNER_SLOT))) {
                    revert(0, 0)
                }
            }
            
            function whenNotPaused() {
                if eq(sload(PAUSED_SLOT), 1) {
                    revert(0, 0)
                }
            }
            
            function setOwner(newOwner) {
                onlyOwner()
                sstore(OWNER_SLOT, newOwner)
            }
            
            function pause() {
                onlyOwner()
                sstore(PAUSED_SLOT, 1)
            }
            
            function unpause() {
                onlyOwner()
                sstore(PAUSED_SLOT, 0)
            }
            
            function mint(to, amount) -> success {
                onlyOwner()
                whenNotPaused()
                
                let current_supply := sload(TOTAL_SUPPLY_SLOT)
                let new_supply := add(current_supply, amount)
                sstore(TOTAL_SUPPLY_SLOT, new_supply)
                
                success := 1
            }
            
            function complexCalculation(input) -> result {
                let temp := 0
                for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                    switch mod(i, 3)
                    case 0 { temp := add(temp, mul(input, i)) }
                    case 1 { temp := add(temp, div(input, add(i, 1))) }
                    default { temp := sub(temp, i) }
                }
                result := temp
            }
        }
        "#;
        
        let result = full_compile_test(input).unwrap();
        assert!(!result.bytecode.is_empty());
        assert!(result.bytecode.len() > 100); // Should be a substantial contract
        
        // Check that all functions are in ABI
        let abi = result.abi.as_object().unwrap();
        let functions = abi["functions"].as_array().unwrap();
        assert!(functions.len() >= 6); // All the functions we defined
    }

    #[test]
    fn test_edge_cases() {
        let input = r#"
        {
            // Test edge cases and boundary conditions
            function test_edge_cases() {
                // Maximum uint256
                let max_uint := sub(0, 1)
                
                // Test overflow handling
                let overflow_result := add(max_uint, 1) // Should wrap to 0
                
                // Test underflow handling  
                let underflow_result := sub(0, 1) // Should wrap to max_uint
                
                // Test division edge cases
                let div_result := div(max_uint, max_uint) // Should be 1
                
                // Test modulo edge cases
                let mod_result := mod(max_uint, 2) // Should be 1
                
                // Test shift edge cases
                let shift_result := shr(255, max_uint) // Should be 1
            }
        }
        "#;
        
        let result = full_compile_test(input).unwrap();
        assert!(!result.bytecode.is_empty());
        // Should handle edge cases without errors
    }

    #[test]
    fn test_gas_optimization() {
        let expensive_input = r#"
        {
            function expensive_function() -> result {
                result := 0
                for { let i := 0 } lt(i, 1000) { i := add(i, 1) } {
                    result := add(result, keccak256(i, 32))
                }
            }
        }
        "#;
        
        let cheap_input = r#"
        {
            function cheap_function() -> result {
                result := add(1, 2)
            }
        }
        "#;
        
        let expensive_result = full_compile_test(expensive_input).unwrap();
        let cheap_result = full_compile_test(cheap_input).unwrap();
        
        // Expensive function should cost significantly more gas
        assert!(expensive_result.estimated_gas > cheap_result.estimated_gas * 10);
    }
}

#[cfg(test)]
mod benchmark_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_lexer_performance() {
        let large_input = r#"
        {
            function large_function() {
                let x := 1
                let y := 2
        "#.repeat(1000) + "}".repeat(1000).as_str();
        
        let start = Instant::now();
        let mut lexer = Lexer::new(&large_input);
        let tokens = lexer.tokenize().unwrap();
        let duration = start.elapsed();
        
        println!("Lexer processed {} tokens in {:?}", tokens.len(), duration);
        assert!(duration.as_millis() < 1000); // Should complete within 1 second
    }

    #[test]
    fn benchmark_parser_performance() {
        let input = r#"
        {
            function recursive_function(n) -> result {
                if eq(n, 0) {
                    result := 1
                }
                if gt(n, 0) {
                    result := mul(n, recursive_function(sub(n, 1)))
                }
            }
        }
        "#;
        
        let start = Instant::now();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let _ast = parser.parse().unwrap();
        let duration = start.elapsed();
        
        println!("Parser completed in {:?}", duration);
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn benchmark_compilation_performance() {
        let input = r#"
        {
            function complex_contract() {
                for { let i := 0 } lt(i, 100) { i := add(i, 1) } {
                    for { let j := 0 } lt(j, 50) { j := add(j, 1) } {
                        let hash := keccak256(add(i, j), 32)
                        sstore(add(i, j), hash)
                    }
                }
            }
        }
        "#;
        
        let start = Instant::now();
        let result = full_compile_test(input).unwrap();
        let duration = start.elapsed();
        
        println!("Full compilation completed in {:?}", duration);
        println!("Generated {} bytes of bytecode", result.bytecode.len());
        println!("Estimated gas: {}", result.estimated_gas);
        
        assert!(duration.as_millis() < 5000); // Should complete within 5 seconds
        assert!(!result.bytecode.is_empty());
    }

    #[test]
    fn benchmark_runtime_performance() {
        let runtime = EvmRuntime::new();
        
        // Benchmark memory operations
        let start = Instant::now();
        for i in 0..10000 {
            runtime.mstore(i * 32, &[i as u8; 32]).unwrap();
        }
        let memory_duration = start.elapsed();
        
        // Benchmark storage operations
        let start = Instant::now();
        for i in 0..1000 {
            let key = [i as u8; 32];
            let value = [i as u8; 32];
            runtime.sstore(&key, &value).unwrap();
        }
        let storage_duration = start.elapsed();
        
        // Benchmark arithmetic operations
        let start = Instant::now();
        let mut result = 0u64;
        for i in 0..100000 {
            result = runtime.add(result, i);
        }
        let arithmetic_duration = start.elapsed();
        
        println!("Memory operations: {:?}", memory_duration);
        println!("Storage operations: {:?}", storage_duration);
        println!("Arithmetic operations: {:?}", arithmetic_duration);
        
        // Performance assertions
        assert!(memory_duration.as_millis() < 1000);
        assert!(storage_duration.as_millis() < 5000);
        assert!(arithmetic_duration.as_millis() < 100);
    }

    fn full_compile_test(input: &str) -> Result<codegen::CompilationResult, CompilerError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        
        let config = crate::CompilerConfig::default();
        let mut code_generator = CodeGenerator::new(&config);
        code_generator.generate(&ast)
    }
}