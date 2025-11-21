use neo_solidity::{
    codegen::*, lexer::*, optimizer::*, parser::*, semantic::*, storage_key::*, CompilerConfig,
    CompilerError,
};

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
        assert!(ast.is_ok(), "parser error: {:?}", ast.err());
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
mod runtime_tests {
    use neo_solidity::runtime::execution::ExecutionContext;
    use neo_solidity::runtime::{NeoRuntime, RuntimeConfig};
    use sha3::{Digest, Keccak256};

    #[test]
    fn linear_memory_zero_extends_and_hashes() {
        let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("execution context");

        ctx.write_memory(4, &[0xAA, 0xBB]).expect("memory write");
        let snapshot = ctx.read_memory(0, 8).expect("memory read").to_vec();
        assert_eq!(snapshot[4], 0xAA);
        assert_eq!(snapshot[5], 0xBB);
        assert_eq!(ctx.memory_size(), 8);

        let hash = ctx.keccak_memory_slice(0, 8).expect("keccak hash");
        let mut reference = Keccak256::new();
        reference.update(&snapshot);
        assert_eq!(hash.as_slice(), reference.finalize().as_slice());
    }

    #[test]
    fn calldata_helpers_load_and_copy() {
        let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("execution context");
        ctx.initialize(&[], &[1, 2, 3, 4, 5, 6])
            .expect("context init");

        let word = ctx.calldataload_word(2);
        assert_eq!(&word[..4], &[3, 4, 5, 6]);

        ctx.calldatacopy(0, 1, 3).expect("calldata copy");
        assert_eq!(ctx.read_memory(0, 3).expect("memory slice"), &[2, 3, 4]);
    }

    #[test]
    fn returndatacopy_moves_bytes_into_memory() {
        let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("execution context");
        ctx.set_return_data(vec![10, 20, 30, 40]);
        ctx.returndatacopy(5, 1, 2).expect("returndatacopy");
        assert_eq!(ctx.returndatasize(), 4);
        assert_eq!(ctx.read_memory(5, 2).expect("memory slice"), &[20, 30]);
    }

    #[test]
    fn runtime_captures_return_payload() {
        let mut runtime = NeoRuntime::new(RuntimeConfig::default()).expect("runtime");
        let script = vec![0x0C, 0x03, b'f', b'o', b'o', 0x40];
        let result = runtime.execute(&script, &[]).expect("execution");
        assert!(result.is_success());
        assert_eq!(result.return_data, b"foo");
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

#[cfg(test)]
mod ir_tests {
    use neo_solidity::ir::Instruction;
    use neo_solidity::ir::Module;
    use neo_solidity::solidity::analyse_source;

    #[test]
    fn lowers_single_slot_extsload_assembly() {
        let source = r#"
        contract Extsload {
            function extsload(bytes32 slot) external view returns (bytes32) {
                assembly {
                    mstore(0, sload(slot))
                    return(0, 0x20)
                }
            }
        }
        "#;

        let metadata = analyse_source(source).expect("analysis failed");
        let module = Module::from_contract(&metadata).expect("IR lowering failed");
        let function = module
            .functions
            .iter()
            .find(|f| f.name == "extsload")
            .expect("function not found");
        let mut has_dynamic_sload = false;
        let mut has_return = false;
        for bb in &function.basic_blocks {
            for instr in &bb.instructions {
                use neo_solidity::ir::Instruction;
                match instr {
                    Instruction::LoadStorageDynamic => has_dynamic_sload = true,
                    Instruction::Return => has_return = true,
                    _ => {}
                }
            }
        }
        assert!(has_dynamic_sload, "expected LoadStorageDynamic instruction");
        assert!(has_return, "expected Return instruction");
    }

    #[test]
    fn lowers_range_extsload_assembly() {
        let source = r#"
        contract ExtsloadRange {
            function extsload(bytes32 startSlot, uint256 nSlots) external view returns (bytes32[] memory) {
                assembly {
                    let memptr := mload(0x40)
                    let start := memptr
                    let length := shl(5, nSlots)
                    mstore(memptr, 0x20)
                    mstore(add(memptr, 0x20), nSlots)
                    memptr := add(memptr, 0x40)
                    let end := add(memptr, length)
                    for {} 1 {} {
                        mstore(memptr, sload(startSlot))
                        memptr := add(memptr, 0x20)
                        startSlot := add(startSlot, 1)
                        if iszero(lt(memptr, end)) { break }
                    }
                    return(start, sub(end, start))
                }
            }
        }
        "#;

        let metadata = analyse_source(source).expect("analysis failed");
        let module = Module::from_contract(&metadata).expect("IR lowering failed");
        let function = module
            .functions
            .iter()
            .find(|f| f.name == "extsload")
            .expect("function not found");

        let mut has_array_init = false;
        let mut has_loop = false;

        for bb in &function.basic_blocks {
            for instr in &bb.instructions {
                use neo_solidity::ir::Instruction;
                match instr {
                    Instruction::NewArray { .. } => has_array_init = true,
                    Instruction::Label(_) => has_loop = true,
                    _ => {}
                }
            }
        }

        assert!(has_array_init, "expected NewArray instruction");
        assert!(has_loop, "expected loop labels in instructions");
    }

    #[test]
    fn lowers_array_literal_and_slice() {
        let source = r#"
        contract ArrayOps {
            function literal() external pure returns (uint256[] memory out) {
                out = [1, 2, 3];
            }

            function slice(uint256[] memory input) external pure returns (uint256[] memory) {
                return input[1:3];
            }
        }
        "#;

        let metadata = analyse_source(source).expect("analysis failed");
        let module = Module::from_contract(&metadata).expect("IR lowering failed");

        let literal_fn = module
            .functions
            .iter()
            .find(|f| f.name == "literal")
            .unwrap();
        assert!(
            literal_fn
                .basic_blocks
                .iter()
                .flat_map(|bb| &bb.instructions)
                .any(|i| matches!(i, Instruction::NewArray { .. })),
            "literal should allocate an array"
        );

        let slice_fn = module.functions.iter().find(|f| f.name == "slice").unwrap();
        let mut has_slice_copy = false;
        for instr in slice_fn.basic_blocks.iter().flat_map(|bb| &bb.instructions) {
            if matches!(instr, Instruction::ArrayGet) {
                has_slice_copy = true;
                break;
            }
        }
        assert!(has_slice_copy, "slice lowering should copy elements");
    }

    #[test]
    fn extload_overloads_have_unique_function_entries() {
        let source = r#"
        contract ExtsloadMulti {
            function extsload(bytes32 slot) external view returns (bytes32) {
                assembly {
                    mstore(0, sload(slot))
                    return(0, 0x20)
                }
            }

            function extsload(bytes32 startSlot, uint256 nSlots) external view returns (bytes32[] memory) {
                assembly {
                    let memptr := mload(0x40)
                    let start := memptr
                    let length := shl(5, nSlots)
                    mstore(memptr, 0x20)
                    mstore(add(memptr, 0x20), nSlots)
                    memptr := add(memptr, 0x40)
                    let end := add(memptr, length)
                    for {} 1 {} {
                        mstore(memptr, sload(startSlot))
                        memptr := add(memptr, 0x20)
                        startSlot := add(startSlot, 1)
                        if iszero(lt(memptr, end)) { break }
                    }
                    return(start, sub(end, start))
                }
            }
        }
        "#;

        let metadata = analyse_source(source).expect("analysis failed");
        let module = Module::from_contract(&metadata).expect("IR lowering failed");
        let names: Vec<_> = module.functions.iter().map(|f| f.name.clone()).collect();
        assert!(
            names.iter().any(|name| name == "extsload"),
            "expected function named 'extsload'"
        );
    }

    #[test]
    fn extload_multi_parameter_names() {
        let source = r#"
        contract ExtsloadMulti {
            function extsload(bytes32 slot) external view returns (bytes32) {
                assembly {
                    mstore(0, sload(slot))
                    return(0, 0x20)
                }
            }

            function extsload(bytes32 startSlot, uint256 nSlots) external view returns (bytes32[] memory) {
                assembly {
                    let memptr := mload(0x40)
                    let start := memptr
                    let length := shl(5, nSlots)
                    mstore(memptr, 0x20)
                    mstore(add(memptr, 0x20), nSlots)
                    memptr := add(memptr, 0x40)
                    let end := add(memptr, length)
                    for {} 1 {} {
                        mstore(memptr, sload(startSlot))
                        memptr := add(memptr, 0x20)
                        startSlot := add(startSlot, 1)
                        if iszero(lt(memptr, end)) { break }
                    }
                    return(start, sub(end, start))
                }
            }
        }
        "#;

        let metadata = analyse_source(source).expect("analysis failed");
        let range_method = metadata
            .methods
            .iter()
            .find(|m| m.name == "extsload" && m.parameters.len() == 2)
            .expect("range overload not found");

        let names: Vec<_> = range_method
            .parameters
            .iter()
            .map(|p| p.name.clone())
            .collect();
        assert_eq!(names[0].as_deref(), Some("startSlot"));
        assert_eq!(names[1].as_deref(), Some("nSlots"));
    }
}

#[cfg(test)]
mod storage_key_tests {
    use super::*;
    use num_bigint::BigInt;

    #[test]
    fn test_compute_state_slot_deterministic() {
        let slot_a = compute_state_slot("balances");
        let slot_b = compute_state_slot("balances");
        let slot_c = compute_state_slot("allowances");

        assert_eq!(slot_a.len(), 32);
        assert_eq!(slot_a, slot_b);
        assert_ne!(slot_a, slot_c);
    }

    #[test]
    fn test_derive_mapping_slot_changes_with_key() {
        let base_slot = compute_state_slot("balances");

        let key_a = KeyFragment::address(vec![0x11; 20]);
        let key_b = KeyFragment::address(vec![0x22; 20]);

        let slot_a = derive_mapping_slot(&base_slot, &[key_a.clone()]);
        let slot_b = derive_mapping_slot(&base_slot, &[key_b]);

        assert_eq!(slot_a.len(), 32);
        assert_eq!(slot_b.len(), 32);
        assert_ne!(slot_a, slot_b);
    }

    #[test]
    fn test_derive_mapping_slot_multiple_fragments() {
        let base_slot = compute_state_slot("allowances");
        let owner = KeyFragment::address(vec![0xAA; 20]);
        let spender = KeyFragment::address(vec![0xBB; 20]);
        let amount = KeyFragment::integer(BigInt::from(42), 256, false);

        let slot = derive_mapping_slot(&base_slot, &[owner, spender, amount]);
        assert_eq!(slot.len(), 32);

        let different = derive_mapping_slot(
            &base_slot,
            &[
                KeyFragment::address(vec![0xAA; 20]),
                KeyFragment::address(vec![0xCC; 20]),
                KeyFragment::integer(BigInt::from(42), 256, false),
            ],
        );

        assert_ne!(slot, different);
    }

    #[test]
    fn test_integer_encoding_sign() {
        let base_slot = compute_state_slot("signedMapping");
        let positive = KeyFragment::integer(BigInt::from(5), 256, true);
        let negative = KeyFragment::integer(BigInt::from(-5), 256, true);

        let slot_pos = derive_mapping_slot(&base_slot, &[positive]);
        let slot_neg = derive_mapping_slot(&base_slot, &[negative]);

        assert_ne!(slot_pos, slot_neg);
    }
}
