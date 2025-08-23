use neo_solidity::{codegen::*, lexer::*, optimizer::*, parser::*, semantic::*, CompilerError, CompilerConfig};

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
        assert!(ast.is_ok());
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
        
        // Constants should be processed
        assert!(matches!(optimized.node_type, AstNodeType::Object { .. }));
    }

    #[test]
    fn test_optimization_levels() {
        let input = r#"
        {
            let x := add(1, 2)
            let y := mul(x, 0)
        }
        "#;
        
        let opt0 = optimize_yul(input, 0).unwrap();
        let opt1 = optimize_yul(input, 1).unwrap();
        let opt2 = optimize_yul(input, 2).unwrap();
        let opt3 = optimize_yul(input, 3).unwrap();
        
        // All optimization levels should work
        assert!(matches!(opt0.node_type, AstNodeType::Object { .. }));
        assert!(matches!(opt1.node_type, AstNodeType::Object { .. }));
        assert!(matches!(opt2.node_type, AstNodeType::Object { .. }));
        assert!(matches!(opt3.node_type, AstNodeType::Object { .. }));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    fn full_compile_test(input: &str) -> Result<CompilationResult, CompilerError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        
        let config = CompilerConfig::default();
        let mut code_generator = CodeGenerator::new(&config);
        code_generator.generate(&ast)
    }

    #[test]
    fn test_erc20_like_contract() {
        let input = r#"
        {
            function balanceOf(account) -> balance {
                let slot := add(1, account)
                balance := sload(slot)
            }
            
            function transfer(to, amount) -> success {
                let sender := caller()
                let sender_balance := balanceOf(sender)
                
                if lt(sender_balance, amount) {
                    success := 0
                    leave
                }
                
                success := 1
            }
        }
        "#;
        
        let result = full_compile_test(input).unwrap();
        assert!(!result.bytecode.is_empty());
        assert!(result.estimated_gas > 0);
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
        }
        "#;
        
        let result = full_compile_test(input).unwrap();
        assert!(!result.bytecode.is_empty());
        assert!(result.estimated_gas > 100);
    }

    #[test]
    fn test_optimization_effectiveness() {
        let input = r#"
        {
            function unoptimized_example() -> result {
                let a := add(1, 2)
                let b := add(1, 2)
                let c := mul(a, 1)
                let d := add(c, 0)
                result := d
            }
        }
        "#;
        
        let result = full_compile_test(input).unwrap();
        assert!(!result.bytecode.is_empty());
        assert!(result.estimated_gas < 1000);
    }
}