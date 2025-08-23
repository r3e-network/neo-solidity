//! Complete Neo Solidity Compiler Example
//!
//! This example demonstrates the full functionality of the Neo Solidity compiler,
//! including compilation, optimization, runtime execution, and debugging features.

use neo_solidity::{
    CompilerOptions, DiagnosticLevel, NeoRuntime, NeoVMVersion, OutputFormat, RuntimeConfig,
    SolidityCompiler,
};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Neo Solidity Compiler - Complete Example");
    println!("===========================================\n");

    // Example 1: Basic Compilation
    println!("📝 Example 1: Basic Compilation");
    basic_compilation_example()?;

    // Example 2: Advanced Compilation with Optimization
    println!("\n🔧 Example 2: Advanced Compilation with Optimization");
    advanced_compilation_example()?;

    // Example 3: Runtime Execution
    println!("\n⚡ Example 3: Runtime Execution");
    runtime_execution_example()?;

    // Example 4: Complex Smart Contract
    println!("\n🏗️  Example 4: Complex Smart Contract");
    complex_contract_example()?;

    // Example 5: Error Handling and Diagnostics
    println!("\n🔍 Example 5: Error Handling and Diagnostics");
    error_handling_example()?;

    // Example 6: Performance Analysis
    println!("\n📊 Example 6: Performance Analysis");
    performance_analysis_example()?;

    println!("\n✅ All examples completed successfully!");
    Ok(())
}

/// Example 1: Basic compilation of simple Yul code
fn basic_compilation_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut compiler = SolidityCompiler::new();

    let source = r#"
        {
            // Simple arithmetic operations
            let x := 10
            let y := 20
            let sum := add(x, y)
            let product := mul(sum, 2)
        }
    "#;

    println!("Source code:");
    println!("{}", source);

    let result = compiler.compile(source)?;

    println!("Compilation Results:");
    println!("  ✅ Success: {}", result.is_success());
    println!("  📦 Bytecode size: {} bytes", result.bytecode.len());
    println!("  ⛽ Gas estimate: {:?}", result.gas_estimate);
    println!("  🎯 Target: {:?}", result.metadata.neo_version);
    println!(
        "  🔧 Optimization level: {}",
        result.metadata.optimization_level
    );

    // Show bytecode in hex format
    println!(
        "  📋 Bytecode (hex): {}",
        hex::encode(&result.bytecode[..std::cmp::min(32, result.bytecode.len())])
    );
    if result.bytecode.len() > 32 {
        println!("    ... ({} more bytes)", result.bytecode.len() - 32);
    }

    Ok(())
}

/// Example 2: Advanced compilation with different options
fn advanced_compilation_example() -> Result<(), Box<dyn std::error::Error>> {
    let source = r#"
        function fibonacci(n) -> result {
            if lt(n, 2) {
                result := n
            }
            if iszero(lt(n, 2)) {
                let a := fibonacci(sub(n, 1))
                let b := fibonacci(sub(n, 2))
                result := add(a, b)
            }
        }
        
        function main() {
            let fib10 := fibonacci(10)
        }
    "#;

    println!("Complex source with functions:");
    println!("{}", source);

    // Test different optimization levels
    for level in 0..=3 {
        let options = CompilerOptions {
            optimization_level: level,
            debug: level == 0, // Enable debug for unoptimized build
            source_maps: level == 0,
            target_version: NeoVMVersion::Latest,
            ..Default::default()
        };

        let mut compiler = SolidityCompiler::with_options(options);
        let result = compiler.compile(source)?;

        println!("\nOptimization Level {}:", level);
        println!("  📦 Bytecode size: {} bytes", result.bytecode.len());
        println!("  ⛽ Gas estimate: {:?}", result.gas_estimate);
        println!("  📄 ABI entries: {}", result.abi.len());
        println!("  ⚠️  Warnings: {}", result.metadata.security_warnings);

        if !result.abi.is_empty() {
            println!("  🔧 Functions:");
            for abi_entry in &result.abi {
                println!(
                    "    - {} ({} inputs, {} outputs)",
                    abi_entry.name,
                    abi_entry.inputs.len(),
                    abi_entry.outputs.len()
                );
            }
        }
    }

    Ok(())
}

/// Example 3: Runtime execution of compiled bytecode
fn runtime_execution_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut compiler = SolidityCompiler::new();

    let source = r#"
        function square(x) -> result {
            result := mul(x, x)
        }
        
        {
            let number := 7
            let squared := square(number)
        }
    "#;

    println!("Compiling for runtime execution:");
    let compiled = compiler.compile(source)?;
    println!(
        "  ✅ Compiled successfully: {} bytes",
        compiled.bytecode.len()
    );

    // Create runtime with custom configuration
    let config = RuntimeConfig {
        gas_limit: 1_000_000,
        enable_debugging: true,
        enable_tracing: true,
        ..Default::default()
    };

    let mut runtime = NeoRuntime::new(config)?;
    println!("  🔧 Runtime created with 1M gas limit");

    // Deploy contract
    let contract_address = runtime.deploy_contract(&compiled.bytecode, &[])?;
    println!("  📍 Contract deployed at: {}", contract_address);

    // Execute contract
    let execution_result = runtime.execute(&compiled.bytecode, &[])?;

    println!("\nExecution Results:");
    println!("  ✅ Success: {}", execution_result.is_success());
    println!(
        "  ⛽ Gas used: {} / {}",
        execution_result.gas_used, execution_result.gas_limit
    );
    println!(
        "  📊 Gas efficiency: {:.2}%",
        execution_result.gas_efficiency() * 100.0
    );
    println!(
        "  📤 Return data: {} bytes",
        execution_result.return_data.len()
    );
    if !execution_result.return_data.is_empty() {
        println!("    Hex: {}", execution_result.return_hex());
    }

    // Show runtime statistics
    let stats = runtime.get_statistics();
    println!("  📈 Runtime stats:");
    println!(
        "    Instructions executed: {}",
        stats.total_instructions_executed
    );
    println!("    Max stack depth: {}", stats.max_stack_depth);
    println!("    Storage reads: {}", stats.storage_reads);
    println!("    Storage writes: {}", stats.storage_writes);

    Ok(())
}

/// Example 4: Complex smart contract with multiple features
fn complex_contract_example() -> Result<(), Box<dyn std::error::Error>> {
    let source = r#"
        object "TokenContract" {
            code {
                // Constructor
                let owner := caller()
                sstore(0, owner)
                
                // Initial supply
                let totalSupply := 1000000
                sstore(1, totalSupply)
                sstore(keccak256(owner, 2), totalSupply)
            }
            
            data "metadata" "0x1234567890abcdef"
        }
        
        function transfer(to, amount) -> success {
            let sender := caller()
            let senderBalance := sload(keccak256(sender, 2))
            
            if lt(senderBalance, amount) {
                success := 0
                leave
            }
            
            // Update balances
            sstore(keccak256(sender, 2), sub(senderBalance, amount))
            let toBalance := sload(keccak256(to, 2))
            sstore(keccak256(to, 2), add(toBalance, amount))
            
            // Emit transfer event (simplified)
            log3(0, 0, keccak256("Transfer(address,address,uint256)"), sender, to)
            
            success := 1
        }
        
        function balanceOf(account) -> balance {
            balance := sload(keccak256(account, 2))
        }
        
        function totalSupply() -> supply {
            supply := sload(1)
        }
    "#;

    println!("Complex smart contract source:");
    println!("{}", &source[..std::cmp::min(500, source.len())]);
    if source.len() > 500 {
        println!("... ({} more characters)", source.len() - 500);
    }

    let options = CompilerOptions {
        optimization_level: 2,
        security_checks: true,
        debug: true,
        source_maps: true,
        ..Default::default()
    };

    let mut compiler = SolidityCompiler::with_options(options);
    let result = compiler.compile(source)?;

    println!("\nComplex Contract Results:");
    println!("  ✅ Compilation successful");
    println!("  📦 Bytecode size: {} bytes", result.bytecode.len());
    println!("  📄 Functions in ABI: {}", result.abi.len());
    println!("  ⛽ Estimated gas: {:?}", result.gas_estimate);
    println!(
        "  🛡️  Security warnings: {}",
        result.metadata.security_warnings
    );
    println!("  📊 Diagnostics: {}", result.diagnostics.len());

    // Show function signatures
    if !result.abi.is_empty() {
        println!("  🔧 Function signatures:");
        for (i, func) in result.abi.iter().enumerate() {
            println!(
                "    {}. {}({} inputs) -> {} outputs",
                i + 1,
                func.name,
                func.inputs.len(),
                func.outputs.len()
            );
        }
    }

    // Show diagnostics
    if !result.diagnostics.is_empty() {
        println!("  📋 Diagnostics:");
        for (i, diag) in result.diagnostics.iter().take(3).enumerate() {
            let level = match diag.level {
                DiagnosticLevel::Error => "❌",
                DiagnosticLevel::Warning => "⚠️",
                DiagnosticLevel::Info => "ℹ️",
                DiagnosticLevel::Hint => "💡",
            };
            println!("    {}. {} {}", i + 1, level, diag.message);
        }
        if result.diagnostics.len() > 3 {
            println!("    ... and {} more", result.diagnostics.len() - 3);
        }
    }

    Ok(())
}

/// Example 5: Error handling and diagnostics
fn error_handling_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut compiler = SolidityCompiler::new();

    println!("Testing error handling with invalid source code:");

    // Test various error conditions
    let test_cases = vec![
        (
            "Syntax Error",
            r#"
            {
                let x := 
            }
        "#,
        ),
        (
            "Invalid Function",
            r#"
            function broken( {
                let x := 42
            }
        "#,
        ),
        (
            "Unknown Function Call",
            r#"
            {
                let result := unknownFunction(42)
            }
        "#,
        ),
        (
            "Unmatched Braces",
            r#"
            {
                let x := 42
            
        "#,
        ),
    ];

    for (test_name, source) in test_cases {
        println!("\n🧪 Testing: {}", test_name);

        match compiler.compile(source) {
            Ok(result) => {
                println!("  ⚠️  Compiled with warnings:");
                for diag in result.diagnostics.iter().take(2) {
                    println!(
                        "    - {}: {}",
                        match diag.level {
                            DiagnosticLevel::Warning => "Warning",
                            DiagnosticLevel::Error => "Error",
                            DiagnosticLevel::Info => "Info",
                            DiagnosticLevel::Hint => "Hint",
                        },
                        diag.message
                    );
                }
            }
            Err(error) => {
                println!("  ❌ Compilation failed: {}", error);
            }
        }

        // Test validation without compilation
        match compiler.validate(source) {
            Ok(diagnostics) => {
                println!("  📋 Validation found {} issues", diagnostics.len());
            }
            Err(error) => {
                println!("  🔍 Validation error: {}", error);
            }
        }
    }

    Ok(())
}

/// Example 6: Performance analysis and benchmarking
fn performance_analysis_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("Performance analysis of different code patterns:");

    let test_cases = vec![
        (
            "Simple arithmetic",
            r#"
            {
                let result := add(mul(2, 3), 4)
            }
        "#,
        ),
        (
            "Loop with 10 iterations",
            r#"
            {
                let sum := 0
                for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                    sum := add(sum, i)
                }
            }
        "#,
        ),
        (
            "Nested loops",
            r#"
            {
                let result := 0
                for { let i := 0 } lt(i, 5) { i := add(i, 1) } {
                    for { let j := 0 } lt(j, 5) { j := add(j, 1) } {
                        result := add(result, mul(i, j))
                    }
                }
            }
        "#,
        ),
        (
            "Function calls",
            r#"
            function helper(x) -> result {
                result := mul(x, x)
            }
            
            {
                let a := helper(5)
                let b := helper(10)
                let sum := add(a, b)
            }
        "#,
        ),
    ];

    println!("\n📊 Performance Comparison:");
    println!(
        "{:<20} | {:>12} | {:>12} | {:>15}",
        "Pattern", "Bytecode (B)", "Gas Est.", "Compile Time"
    );
    println!("{:-<20}-+-{:-<12}-+-{:-<12}-+-{:-<15}", "", "", "", "");

    for (name, source) in test_cases {
        let mut compiler = SolidityCompiler::new();

        let start_time = std::time::Instant::now();
        match compiler.compile(source) {
            Ok(result) => {
                let compile_time = start_time.elapsed();

                println!(
                    "{:<20} | {:>12} | {:>12} | {:>13}ms",
                    name,
                    result.bytecode.len(),
                    result
                        .gas_estimate
                        .map(|g| g.to_string())
                        .unwrap_or("N/A".to_string()),
                    compile_time.as_millis()
                );
            }
            Err(_) => {
                println!(
                    "{:<20} | {:>12} | {:>12} | {:>15}",
                    name, "ERROR", "ERROR", "ERROR"
                );
            }
        }
    }

    // Memory usage test
    println!("\n🧠 Memory Usage Analysis:");
    let config = RuntimeConfig {
        gas_limit: 10_000_000,
        memory_limit: 1024 * 1024, // 1MB
        ..Default::default()
    };
    let runtime = NeoRuntime::new(config)?;

    println!("  📊 Runtime memory limit: 1MB");
    println!("  ⛽ Runtime gas limit: 10M");

    // Test with different bytecode sizes
    let sizes = [100, 1000, 10000];
    for size in sizes {
        let bytecode = vec![0x60, 0x01]; // Simple PUSH1 1
        let mut extended_bytecode = bytecode;
        for _ in 0..size / 2 {
            extended_bytecode.extend(&[0x60, 0x01]);
        }

        println!(
            "  📦 Bytecode size {}: {} bytes",
            size,
            extended_bytecode.len()
        );
    }

    Ok(())
}

/// Utility function to create a simple contract for testing
fn create_test_contract() -> &'static str {
    r#"
        function add(a, b) -> result {
            result := add(a, b)
        }
        
        function multiply(a, b) -> result {
            result := mul(a, b)
        }
        
        function power(base, exp) -> result {
            result := 1
            for { let i := 0 } lt(i, exp) { i := add(i, 1) } {
                result := mul(result, base)
            }
        }
        
        {
            let x := 5
            let y := 3
            let sum := add(x, y)
            let product := multiply(x, y)
            let power_result := power(x, y)
        }
    "#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        basic_compilation_example().expect("Basic example should work");
    }

    #[test]
    fn test_advanced_example() {
        advanced_compilation_example().expect("Advanced example should work");
    }

    #[test]
    fn test_runtime_example() {
        runtime_execution_example().expect("Runtime example should work");
    }
}
