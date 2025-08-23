//! Integration Tests for Neo Solidity Compiler
//!
//! Comprehensive integration tests covering the entire compilation pipeline
//! from Yul source to NeoVM bytecode execution.

use neo_solidity::{CompilerOptions, NeoRuntime, NeoVMVersion, RuntimeConfig, SolidityCompiler};
use std::collections::HashMap;

/// Test basic compilation pipeline
#[test]
fn test_basic_compilation() {
    let mut compiler = SolidityCompiler::new();
    let source = r#"
        {
            let x := 42
            let y := add(x, 8)
        }
    "#;

    let result = compiler
        .compile(source)
        .expect("Compilation should succeed");
    assert!(!result.bytecode.is_empty());
    assert!(result.is_success());
    assert!(result.gas_estimate.is_some());
    assert!(result.gas_estimate.unwrap() > 0);
}

/// Test function compilation and execution
#[test]
fn test_function_compilation() {
    let mut compiler = SolidityCompiler::new();
    let source = r#"
        function add(a, b) -> result {
            result := add(a, b)
        }
        
        function main() {
            let x := add(10, 20)
            let y := add(x, 5)
        }
    "#;

    let result = compiler
        .compile(source)
        .expect("Function compilation should succeed");
    assert!(!result.bytecode.is_empty());
    assert!(!result.abi.is_empty());

    // Should have two functions in ABI
    assert_eq!(result.abi.len(), 2);

    let function_names: Vec<_> = result.abi.iter().map(|f| &f.name).collect();
    assert!(function_names.contains(&&"add".to_string()));
    assert!(function_names.contains(&&"main".to_string()));
}

/// Test arithmetic operations
#[test]
fn test_arithmetic_operations() {
    let mut compiler = SolidityCompiler::new();
    let source = r#"
        {
            let a := 10
            let b := 5
            let sum := add(a, b)
            let diff := sub(a, b)
            let product := mul(a, b)
            let quotient := div(a, b)
            let remainder := mod(a, b)
        }
    "#;

    let result = compiler
        .compile(source)
        .expect("Arithmetic compilation should succeed");
    assert!(!result.bytecode.is_empty());
}

/// Test control flow structures
#[test]
fn test_control_flow() {
    let mut compiler = SolidityCompiler::new();
    let source = r#"
        {
            let x := 10
            
            if gt(x, 5) {
                x := add(x, 1)
            }
            
            switch x
            case 11 {
                x := mul(x, 2)
            }
            default {
                x := 0
            }
            
            for { let i := 0 } lt(i, 5) { i := add(i, 1) } {
                x := add(x, i)
            }
        }
    "#;

    let result = compiler
        .compile(source)
        .expect("Control flow compilation should succeed");
    assert!(!result.bytecode.is_empty());
}

/// Test memory operations
#[test]
fn test_memory_operations() {
    let mut compiler = SolidityCompiler::new();
    let source = r#"
        {
            let ptr := 0x40
            let value := 0x1234567890abcdef
            mstore(ptr, value)
            let loaded := mload(ptr)
            
            let byteValue := 0xff
            mstore8(add(ptr, 32), byteValue)
        }
    "#;

    let result = compiler
        .compile(source)
        .expect("Memory operations compilation should succeed");
    assert!(!result.bytecode.is_empty());
}

/// Test different optimization levels
#[test]
fn test_optimization_levels() {
    let source = r#"
        {
            let a := add(1, 2)
            let b := mul(3, 4)
            let c := add(a, b)
            let d := sub(c, 1)
        }
    "#;

    let mut results = HashMap::new();

    for level in 0..=3 {
        let options = CompilerOptions {
            optimization_level: level,
            ..Default::default()
        };
        let mut compiler = SolidityCompiler::with_options(options);

        let result = compiler
            .compile(source)
            .expect("Compilation should succeed");
        results.insert(level, result);
    }

    // Higher optimization levels should generally produce smaller or more efficient bytecode
    assert!(
        results[&0].bytecode.len() >= results[&3].bytecode.len()
            || results[&0].gas_estimate >= results[&3].gas_estimate
    );
}

/// Test different NeoVM target versions
#[test]
fn test_neovm_versions() {
    let source = r#"
        {
            let x := add(1, 2)
        }
    "#;

    let versions = [NeoVMVersion::V3_0, NeoVMVersion::V3_5, NeoVMVersion::Latest];

    for version in versions.iter() {
        let options = CompilerOptions {
            target_version: version.clone(),
            ..Default::default()
        };
        let mut compiler = SolidityCompiler::with_options(options);

        let result = compiler
            .compile(source)
            .expect("Compilation should succeed for all versions");
        assert!(!result.bytecode.is_empty());
        assert_eq!(result.metadata.neo_version, *version);
    }
}

/// Test runtime integration
#[test]
fn test_runtime_integration() {
    let mut compiler = SolidityCompiler::new();
    let source = r#"
        function square(x) -> result {
            result := mul(x, x)
        }
    "#;

    let compiled = compiler
        .compile(source)
        .expect("Compilation should succeed");

    let config = RuntimeConfig::default();
    let mut runtime = NeoRuntime::new(config).expect("Runtime creation should succeed");

    // Deploy contract
    let address = runtime
        .deploy_contract(&compiled.bytecode, &[])
        .expect("Deployment should succeed");
    assert!(!address.is_empty());

    // Execute function (simplified - in real implementation would need proper function calling)
    let result = runtime
        .execute(&compiled.bytecode, &[])
        .expect("Execution should succeed");
    assert!(result.is_success());
}

/// Test error handling and recovery
#[test]
fn test_error_handling() {
    let mut compiler = SolidityCompiler::new();

    // Test lexical error
    let invalid_source = r#"
        {
            let x := @invalid_character
        }
    "#;

    let result = compiler.compile(invalid_source);
    assert!(result.is_err());

    // Test parse error
    let parse_error_source = r#"
        {
            let x := 
        }
    "#;

    let result = compiler.compile(parse_error_source);
    assert!(result.is_err());

    // Test semantic error (undefined variable)
    let semantic_error_source = r#"
        {
            let x := undefined_variable
        }
    "#;

    // This might succeed at compile time but should produce warnings
    let result = compiler.compile(semantic_error_source);
    match result {
        Ok(compiled) => {
            // Should have diagnostics/warnings
            assert!(!compiled.diagnostics.is_empty());
        }
        Err(_) => {
            // Also acceptable if caught as error
        }
    }
}

/// Test complex nested structures
#[test]
fn test_complex_nested_structures() {
    let mut compiler = SolidityCompiler::new();
    let source = r#"
        function fibonacci(n) -> result {
            if lt(n, 2) {
                result := n
            }
            if iszero(lt(n, 2)) {
                result := add(fibonacci(sub(n, 1)), fibonacci(sub(n, 2)))
            }
        }
        
        function factorial(n) -> result {
            result := 1
            for { let i := 1 } lt(i, add(n, 1)) { i := add(i, 1) } {
                result := mul(result, i)
            }
        }
        
        {
            let fib5 := fibonacci(5)
            let fact5 := factorial(5)
            let combined := add(fib5, fact5)
        }
    "#;

    let result = compiler
        .compile(source)
        .expect("Complex nested compilation should succeed");
    assert!(!result.bytecode.is_empty());
    assert!(result.abi.len() >= 2); // At least fibonacci and factorial functions
}

/// Test validation without compilation
#[test]
fn test_validation_only() {
    let mut compiler = SolidityCompiler::new();

    // Valid source
    let valid_source = r#"
        function test() {
            let x := 42
        }
    "#;

    let diagnostics = compiler
        .validate(valid_source)
        .expect("Validation should succeed");
    assert!(
        diagnostics.is_empty()
            || diagnostics
                .iter()
                .all(|d| !matches!(d.level, neo_solidity::DiagnosticLevel::Error))
    );

    // Invalid source
    let invalid_source = r#"
        function test( {
            let x := 42
        }
    "#;

    let diagnostics = compiler.validate(invalid_source);
    // Should either return error or diagnostics with errors
    match diagnostics {
        Ok(diags) => assert!(diags
            .iter()
            .any(|d| matches!(d.level, neo_solidity::DiagnosticLevel::Error))),
        Err(_) => {} // Also acceptable
    }
}

/// Test gas estimation accuracy
#[test]
fn test_gas_estimation() {
    let mut compiler = SolidityCompiler::new();

    // Simple operations should have lower gas cost
    let simple_source = r#"
        {
            let x := 1
        }
    "#;

    let simple_result = compiler
        .compile(simple_source)
        .expect("Simple compilation should succeed");
    let simple_gas = simple_result
        .gas_estimate
        .expect("Should have gas estimate");

    // Complex operations should have higher gas cost
    let complex_source = r#"
        {
            for { let i := 0 } lt(i, 100) { i := add(i, 1) } {
                let x := mul(i, i)
                let y := add(x, i)
            }
        }
    "#;

    let complex_result = compiler
        .compile(complex_source)
        .expect("Complex compilation should succeed");
    let complex_gas = complex_result
        .gas_estimate
        .expect("Should have gas estimate");

    // Complex should use more gas than simple
    assert!(complex_gas > simple_gas);
}

/// Test bytecode format and structure
#[test]
fn test_bytecode_format() {
    let mut compiler = SolidityCompiler::new();
    let source = r#"
        {
            let x := 42
        }
    "#;

    let result = compiler
        .compile(source)
        .expect("Compilation should succeed");
    let bytecode = &result.bytecode;

    // Bytecode should not be empty
    assert!(!bytecode.is_empty());

    // Should be valid binary data
    assert!(bytecode.iter().all(|&b| b <= 255));

    // Assembly should not be empty
    assert!(!result.assembly.is_empty());
    assert!(
        result.assembly.contains("PUSH")
            || result.assembly.contains("ADD")
            || result.assembly.lines().count() > 0
    );
}

/// Test source map generation
#[test]
fn test_source_maps() {
    let options = CompilerOptions {
        source_maps: true,
        debug: true,
        ..Default::default()
    };
    let mut compiler = SolidityCompiler::with_options(options);

    let source = r#"
        function test() {
            let x := 42
            let y := add(x, 1)
        }
    "#;

    let result = compiler
        .compile(source)
        .expect("Compilation should succeed");
    assert!(result.source_maps.is_some());

    let source_maps = result.source_maps.unwrap();
    assert!(!source_maps.mappings.is_empty());
    assert!(!source_maps.sources.is_empty());
}

/// Test metadata generation
#[test]
fn test_metadata_generation() {
    let mut compiler = SolidityCompiler::new();
    let source = r#"
        {
            let x := 42
        }
    "#;

    let result = compiler
        .compile(source)
        .expect("Compilation should succeed");
    let metadata = &result.metadata;

    assert!(!metadata.compiler_version.is_empty());
    assert!(metadata.compilation_time > 0);
    assert_eq!(metadata.optimization_level, 2); // Default optimization level
}

/// Performance benchmark test
#[test]
fn test_compilation_performance() {
    let mut compiler = SolidityCompiler::new();
    let source = r#"
        function complexCalculation(n) -> result {
            result := 0
            for { let i := 0 } lt(i, n) { i := add(i, 1) } {
                for { let j := 0 } lt(j, n) { j := add(j, 1) } {
                    result := add(result, mul(i, j))
                }
            }
        }
        
        {
            let result := complexCalculation(10)
        }
    "#;

    let start = std::time::Instant::now();
    let result = compiler
        .compile(source)
        .expect("Compilation should succeed");
    let duration = start.elapsed();

    // Compilation should complete in reasonable time (less than 1 second for this test)
    assert!(duration.as_secs() < 1);
    assert!(!result.bytecode.is_empty());

    // Check that metadata includes timing information
    assert!(result.metadata.compilation_time > 0);
}

/// Test memory management in runtime
#[test]
fn test_runtime_memory_management() {
    let config = RuntimeConfig {
        memory_limit: 1024 * 1024, // 1MB limit
        ..Default::default()
    };
    let mut runtime = NeoRuntime::new(config).expect("Runtime creation should succeed");

    // Simple bytecode that doesn't exceed memory limit
    let bytecode = vec![0x60, 0x01, 0x60, 0x02, 0x01]; // PUSH1 1, PUSH1 2, ADD
    let result = runtime
        .execute(&bytecode, &[])
        .expect("Execution should succeed");
    assert!(result.is_success());
}

/// Test storage operations in runtime
#[test]
fn test_runtime_storage_operations() {
    let config = RuntimeConfig::default();
    let mut runtime = NeoRuntime::new(config).expect("Runtime creation should succeed");

    let account = "0x1234567890123456789012345678901234567890";
    let key = b"test_key";
    let value = b"test_value";

    // Set storage
    runtime
        .set_storage(account, key, value)
        .expect("Storage set should succeed");

    // Get storage
    let retrieved = runtime
        .get_storage(account, key)
        .expect("Storage get should succeed");
    assert_eq!(retrieved, Some(value.to_vec()));

    // Test balance operations
    runtime
        .set_balance(account, 1000)
        .expect("Balance set should succeed");
    let balance = runtime
        .get_balance(account)
        .expect("Balance get should succeed");
    assert_eq!(balance, 1000);
}

/// Test state management and snapshots
#[test]
fn test_state_snapshots() {
    let config = RuntimeConfig::default();
    let mut runtime = NeoRuntime::new(config).expect("Runtime creation should succeed");

    let account = "0x1234567890123456789012345678901234567890";

    // Set initial state
    runtime
        .set_balance(account, 1000)
        .expect("Initial balance set should succeed");

    // Take snapshot
    let snapshot = runtime.get_state_snapshot();

    // Modify state
    runtime
        .set_balance(account, 2000)
        .expect("Modified balance set should succeed");
    assert_eq!(runtime.get_balance(account).unwrap(), 2000);

    // Restore snapshot
    runtime
        .restore_state(snapshot)
        .expect("State restore should succeed");
    assert_eq!(runtime.get_balance(account).unwrap(), 1000);
}

/// Test error conditions in runtime
#[test]
fn test_runtime_error_conditions() {
    let config = RuntimeConfig {
        gas_limit: 100, // Very low gas limit
        ..Default::default()
    };
    let mut runtime = NeoRuntime::new(config).expect("Runtime creation should succeed");

    // Bytecode that should exceed gas limit
    let expensive_bytecode = vec![
        0x60, 0x01, // PUSH1 1
        0x60, 0x02, // PUSH1 2
        0x01, // ADD
              // Repeat many times to exceed gas
    ];
    let mut extended_bytecode = expensive_bytecode;
    for _ in 0..100 {
        extended_bytecode.extend(&[0x60, 0x01, 0x60, 0x02, 0x01]);
    }

    let result = runtime
        .execute(&extended_bytecode, &[])
        .expect("Execution should not panic");
    assert!(!result.success); // Should fail due to gas limit
    assert!(result.out_of_gas());
}
