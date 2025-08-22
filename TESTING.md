# Neo Solidity Testing Framework

Comprehensive testing and quality assurance framework for the Neo Solidity compiler and runtime, providing extensive testing capabilities including unit tests, differential testing, security analysis, conformance testing, fuzzing, performance benchmarking, and debugging tools.

## Overview

The Neo Solidity testing framework is designed to ensure the highest quality and security standards for bridging Solidity smart contracts to the Neo blockchain. It provides multiple layers of testing and analysis:

- **Unit Testing**: Runtime primitives and core functionality
- **Differential Testing**: EVM vs NeoVM compatibility validation  
- **Security Analysis**: Comprehensive vulnerability detection
- **Fuzzing**: Boundary condition and edge case testing
- **Conformance Testing**: Standards compliance validation
- **Performance Benchmarking**: Execution and gas efficiency analysis
- **Interactive Debugging**: Source-level debugging with trace capabilities

## Quick Start

### Running All Tests

```bash
# Run comprehensive test suite
cargo run --bin neo-sol-test test

# Run specific test categories
cargo run --bin neo-sol-test test --suites unit,differential,security

# Run tests in parallel with custom timeout
cargo run --bin neo-sol-test test --parallel --timeout 600
```

### Security Analysis

```bash
# Analyze a single contract
cargo run --bin neo-sol-test security contract.sol --report

# Analyze all contracts in a directory
cargo run --bin neo-sol-test security contracts/ --static-analysis --crypto-analysis

# Set minimum severity threshold
cargo run --bin neo-sol-test security contract.sol --min-severity critical
```

### Fuzzing Campaign

```bash
# Run fuzzing campaign
cargo run --bin neo-sol-test fuzz contracts/ --max-cases 50000 --duration 1800

# Enable differential fuzzing (EVM vs NeoVM)
cargo run --bin neo-sol-test fuzz contract.sol --differential --crash-dir ./crashes
```

### Performance Benchmarking

```bash
# Run all benchmarks
cargo run --bin neo-sol-test benchmark --report

# Run specific benchmark categories
cargo run --bin neo-sol-test benchmark --categories execution,gas,memory --iterations 1000
```

### Interactive Debugging

```bash
# Start debugging session
cargo run --bin neo-sol-test debug contract.bytecode --source-maps --trace-hooks

# Debug with breakpoints
cargo run --bin neo-sol-test debug contract.bytecode --breakpoints breakpoints.txt
```

## Framework Architecture

### Core Components

```
neo-solidity/
├── crates/
│   ├── testing/           # Core testing framework
│   ├── security/          # Security analysis
│   ├── fuzzing/           # Fuzzing harnesses
│   ├── conformance/       # Standards conformance
│   └── debugger/          # Debugging tools
├── src/
│   └── bin/
│       └── test_runner.rs # Main test execution binary
└── TESTING.md            # This documentation
```

### Testing Framework (`crates/testing/`)

The core testing framework provides:

- **Test Runner**: Orchestrates all test execution
- **Unit Tests**: Runtime primitive testing
- **Differential Testing**: EVM/NeoVM comparison
- **Property-Based Testing**: Generated input validation
- **Integration Testing**: Cross-component validation
- **Performance Testing**: Benchmarking integration

#### Key Files

- `src/lib.rs` - Main testing framework API
- `src/unit_tests.rs` - Runtime primitive unit tests
- `src/differential.rs` - EVM vs NeoVM differential testing
- `src/benchmarks.rs` - Performance benchmarking infrastructure

#### Test Configuration

```rust
use neo_solidity::testing::{TestConfig, TestSuite, TestType};

let config = TestConfig {
    parallel_execution: true,
    max_execution_time_ms: 300000,
    enable_tracing: true,
    output_format: OutputFormat::Json,
    test_suites: vec![
        TestSuite {
            name: "runtime_primitives".to_string(),
            test_type: TestType::Unit,
            enabled: true,
            timeout_ms: 30000,
            retry_count: 0,
        },
        // ... more suites
    ],
};
```

### Security Analysis (`crates/security/`)

Comprehensive security analysis framework including:

- **Static Analysis**: Code pattern analysis
- **Reentrancy Detection**: Cross-function reentrancy analysis  
- **Crypto Analysis**: Cryptographic operation validation
- **Storage Analysis**: Storage collision detection
- **Runtime Monitoring**: Dynamic security validation

#### Key Features

```rust
use neo_solidity::security::{SecurityAnalyzer, SecurityConfig, Severity};

let mut analyzer = SecurityAnalyzer::new(SecurityConfig {
    enable_static_analysis: true,
    enable_reentrancy_detection: true,
    enable_crypto_validation: true,
    enable_storage_analysis: true,
    severity_threshold: Severity::Medium,
    ..Default::default()
})?;

let report = analyzer.analyze_contract(contract_source, "MyContract").await?;
```

#### Vulnerability Categories

- **Reentrancy**: Single-function, cross-function, read-only reentrancy
- **Cryptographic**: Weak randomness, crypto misuse
- **Storage**: Storage collisions, unprotected storage
- **Access Control**: Missing controls, privilege escalation
- **Integer**: Overflow/underflow vulnerabilities
- **Gas**: Gas limit issues, griefing attacks
- **Logic**: General logic errors
- **Neo-specific**: NeoVM stack overflow, invalid Neo calls

### Fuzzing Framework (`crates/fuzzing/`)

Advanced fuzzing capabilities for boundary testing:

- **ABI Fuzzing**: Encoding/decoding boundary testing
- **Transaction Fuzzing**: Transaction data validation
- **Contract Fuzzing**: Contract function call testing
- **Differential Fuzzing**: Comparative fuzzing across VMs

#### Fuzzing Configuration

```rust
use neo_solidity::fuzzing::{FuzzingCoordinator, FuzzingConfig};

let mut coordinator = FuzzingCoordinator::new(FuzzingConfig {
    enable_abi_fuzzing: true,
    enable_transaction_fuzzing: true,
    enable_contract_fuzzing: true,
    max_test_cases: 10000,
    target_coverage: 85.0,
    crash_detection: CrashDetectionConfig {
        detect_buffer_overflows: true,
        detect_integer_overflows: true,
        execution_timeout_ms: 10000,
        ..Default::default()
    },
    ..Default::default()
})?;

let statistics = coordinator.run_fuzzing_campaign().await?;
```

### Conformance Testing (`crates/conformance/`)

Standards compliance validation framework:

- **EVM Compatibility**: EVM specification compliance
- **Solidity Compliance**: Language specification adherence
- **Neo Integration**: Neo blockchain integration testing
- **Ethereum Test Vectors**: Official test suite execution

#### Conformance Levels

- **Basic**: Core features only (50%+ compatibility)
- **Standard**: Most common features (75%+ compatibility)
- **Full**: All specified features (85%+ compatibility)
- **Extended**: Neo-specific extensions (95%+ compatibility)

```rust
use neo_solidity::conformance::{ConformanceTestSuite, ConformanceLevel};

let mut suite = ConformanceTestSuite::new(ConformanceConfig {
    enable_evm_tests: true,
    enable_solidity_tests: true,
    enable_neo_tests: true,
    required_conformance_level: ConformanceLevel::Standard,
    ..Default::default()
})?;

let results = suite.run_conformance_tests().await?;
```

### Performance Benchmarking (`crates/testing/benchmarks.rs`)

Comprehensive performance analysis:

- **Execution Benchmarks**: Contract execution performance
- **Compilation Benchmarks**: Compilation time analysis
- **Gas Benchmarks**: Gas consumption analysis
- **Memory Benchmarks**: Memory usage profiling
- **Differential Benchmarks**: EVM vs NeoVM performance

#### Benchmark Categories

```rust
use neo_solidity::testing::benchmarks::{PerformanceBenchmarker, BenchmarkConfig};

let mut benchmarker = PerformanceBenchmarker::new(BenchmarkConfig {
    warm_up_iterations: 100,
    measurement_iterations: 1000,
    sample_size: 100,
    confidence_level: 0.95,
    enable_memory_profiling: true,
    enable_gas_profiling: true,
    ..Default::default()
});

// Execute different benchmark types
let execution_benchmark = benchmarker.benchmark_contract_execution("contract", &bytecode)?;
let gas_benchmark = benchmarker.benchmark_gas_consumption("operation")?;
let memory_benchmark = benchmarker.benchmark_memory_usage("operation")?;
```

### Interactive Debugging (`crates/debugger/`)

Advanced debugging capabilities:

- **Source Maps**: Bytecode to source mapping
- **Trace Hooks**: Runtime execution monitoring
- **Breakpoints**: Source-level debugging
- **Variable Inspection**: State analysis
- **Call Stack**: Execution flow analysis
- **Error Decoding**: Human-readable error messages

#### Debugging Features

```rust
use neo_solidity::debugger::{Debugger, DebuggerConfig, ExecutionState};

let mut debugger = Debugger::new(DebuggerConfig {
    generate_source_maps: true,
    enable_trace_hooks: true,
    interactive_mode: true,
    break_on_error: true,
    track_gas_usage: true,
    ..Default::default()
})?;

// Start debugging session
let session_id = debugger.start_debugging_session(contract_address, "function_name")?;

// Set breakpoints
let breakpoint_id = debugger.set_breakpoint("contract.sol", 42)?;

// Step execution
let state = debugger.step_into()?;
let call_stack = debugger.get_call_stack();
let variable_value = debugger.inspect_variable("myVariable")?;
```

## Test Categories

### 1. Unit Tests

Test individual runtime primitives and core functionality:

```bash
# Run runtime primitive unit tests
cargo test --lib unit_tests

# Test specific primitive categories
cargo test test_uint256_operations
cargo test test_storage_operations
cargo test test_neo_primitive_tests
```

**Covers:**
- Data type operations (uint256, address, bytes, strings, booleans)
- Arithmetic operations with overflow/underflow detection
- Memory management and bounds checking
- Storage operations and optimization
- Neo-specific primitive operations
- Type conversion between EVM and Neo formats
- Error handling mechanisms

### 2. Differential Testing

Validate compatibility between EVM and NeoVM execution:

```bash
# Run differential tests
cargo test differential_tests

# Test specific categories
cargo test test_arithmetic_operations_diff
cargo test test_gas_consumption_diff
```

**Validates:**
- Identical execution results between EVM and NeoVM
- Gas consumption differences within tolerance
- State change consistency
- Event emission compatibility
- Error handling equivalence

### 3. Security Analysis

Comprehensive vulnerability detection:

```bash
# Run security analysis
cargo run --bin neo-sol-test security contract.sol

# Enable all security checks
cargo run --bin neo-sol-test security contract.sol \
  --static-analysis \
  --crypto-analysis \
  --min-severity low \
  --report
```

**Detects:**
- Reentrancy vulnerabilities (single-function, cross-function)
- Integer overflow/underflow issues
- Access control problems
- Cryptographic misuse
- Storage collision risks
- Gas limit vulnerabilities
- Logic errors and edge cases

### 4. Fuzzing

Boundary condition and edge case testing:

```bash
# Run fuzzing campaign
cargo run --bin neo-sol-test fuzz contracts/ \
  --max-cases 100000 \
  --duration 3600 \
  --differential

# Target specific vulnerability categories
cargo fuzz run fuzz_abi_decode
cargo fuzz run fuzz_transaction_data
```

**Tests:**
- ABI encoding/decoding boundaries
- Transaction data validation
- Contract function calls with random inputs
- Memory corruption detection
- Integer boundary conditions
- Buffer overflow protection

### 5. Conformance Testing

Standards compliance validation:

```bash
# Run conformance tests
cargo run --bin neo-sol-test conformance \
  --ethereum-tests \
  --solidity-tests \
  --level full

# Use custom test vectors
cargo run --bin neo-sol-test conformance \
  --sources custom_tests/ \
  --level standard
```

**Validates:**
- EVM specification compliance
- Solidity language specification adherence
- Ethereum test vector compatibility
- Neo blockchain integration standards
- Cross-chain interoperability

### 6. Performance Benchmarking

Execution and efficiency analysis:

```bash
# Run performance benchmarks
cargo run --bin neo-sol-test benchmark \
  --categories execution,gas,memory \
  --iterations 1000 \
  --report

# Compare EVM vs NeoVM performance
cargo run --bin neo-sol-test benchmark \
  --categories differential \
  --report
```

**Measures:**
- Contract execution performance
- Gas consumption efficiency
- Memory usage patterns
- Compilation performance
- Differential performance (EVM vs NeoVM)

## Configuration

### Test Configuration Files

Create `neo-solidity.toml` for project-specific settings:

```toml
[testing]
parallel_execution = true
max_execution_time_ms = 600000
enable_tracing = true
output_format = "json"

[security]
enable_static_analysis = true
enable_crypto_validation = true
enable_storage_analysis = true
enable_reentrancy_detection = true
severity_threshold = "medium"

[fuzzing]
max_test_cases = 50000
max_generation_time = 1800
enable_differential_benchmarking = true
target_coverage = 85.0

[conformance]
enable_evm_tests = true
enable_solidity_tests = true
enable_ethereum_tests = true
required_conformance_level = "standard"

[debugging]
generate_source_maps = true
enable_trace_hooks = true
break_on_error = true
track_gas_usage = true
```

### Environment Variables

```bash
# Set logging level
export RUST_LOG=debug

# Configure test parallelism
export NEO_SOLIDITY_TEST_THREADS=8

# Set fuzzing timeout
export NEO_SOLIDITY_FUZZ_TIMEOUT=3600

# Configure conformance test sources
export NEO_SOLIDITY_ETHEREUM_TESTS=./ethereum-tests
```

## Integration with CI/CD

### GitHub Actions Example

```yaml
name: Neo Solidity Testing

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    
    - name: Unit Tests
      run: cargo run --bin neo-sol-test test --suites unit
    
    - name: Security Analysis
      run: cargo run --bin neo-sol-test security contracts/ --report
    
    - name: Differential Testing
      run: cargo run --bin neo-sol-test test --suites differential
    
    - name: Conformance Testing
      run: cargo run --bin neo-sol-test conformance --level standard
    
    - name: Performance Benchmarks
      run: cargo run --bin neo-sol-test benchmark --report
    
    - name: Upload Reports
      uses: actions/upload-artifact@v3
      with:
        name: test-reports
        path: |
          *_report.json
          *_benchmark.json
```

### Pre-commit Hooks

Create `.pre-commit-config.yaml`:

```yaml
repos:
- repo: local
  hooks:
  - id: neo-solidity-security
    name: Neo Solidity Security Analysis
    entry: cargo run --bin neo-sol-test security
    language: system
    files: '\.sol$'
    
  - id: neo-solidity-unit-tests
    name: Neo Solidity Unit Tests
    entry: cargo test --lib
    language: system
    pass_filenames: false
```

## Advanced Usage

### Custom Test Suites

Implement custom test suites by extending the `TestSuite` trait:

```rust
use neo_solidity::testing::{TestSuite, TestResult, SuiteResult};
use async_trait::async_trait;

pub struct CustomTestSuite {
    name: String,
}

#[async_trait]
impl TestSuite for CustomTestSuite {
    fn name(&self) -> &str { &self.name }
    fn description(&self) -> &str { "Custom test implementation" }
    
    async fn initialize(&mut self) -> Result<()> {
        // Setup test environment
        Ok(())
    }
    
    async fn discover_tests(&self) -> Result<Vec<TestDescriptor>> {
        // Discover available tests
        Ok(vec![])
    }
    
    async fn execute_test(&self, test: &TestDescriptor) -> Result<TestResult> {
        // Execute individual test
        Ok(TestResult::Passed {
            name: test.name.clone(),
            duration_ms: 0,
            metrics: None,
        })
    }
    
    async fn cleanup(&mut self) -> Result<()> {
        // Cleanup test environment
        Ok(())
    }
}
```

### Custom Security Rules

Add custom security analysis rules:

```rust
use neo_solidity::security::{SecurityRule, Severity, VulnerabilityCategory};

let custom_rule = SecurityRule {
    id: "CUSTOM_001".to_string(),
    name: "Custom Vulnerability Check".to_string(),
    description: "Checks for custom vulnerability pattern".to_string(),
    severity: Severity::High,
    category: VulnerabilityCategory::Logic,
    pattern: r"dangerous_pattern\s*\(".to_string(),
    enabled: true,
};

security_config.custom_rules.push(custom_rule);
```

### Custom Fuzzing Harnesses

Create specialized fuzzing targets:

```rust
use neo_solidity::fuzzing::{FuzzableValue, FuzzableFunction};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(fuzzable_data) = FuzzableValue::arbitrary(&mut Unstructured::new(data)) {
        // Test your specific functionality
        test_custom_functionality(&fuzzable_data);
    }
});
```

## Troubleshooting

### Common Issues

1. **Test Timeouts**: Increase timeout values in configuration
2. **Memory Issues**: Reduce parallel execution or increase system memory
3. **Conformance Failures**: Check test vector sources and network connectivity
4. **Security False Positives**: Add custom rules or adjust severity thresholds
5. **Fuzzing Hangs**: Set appropriate execution timeouts and monitor resources

### Debug Information

Enable detailed logging:

```bash
RUST_LOG=debug cargo run --bin neo-sol-test test --verbose
```

### Performance Issues

For slow test execution:

1. Enable parallel execution: `--parallel`
2. Reduce test scope: `--suites unit,differential`
3. Increase timeout values: `--timeout 1200`
4. Use faster storage (SSD) for fuzzing corpus
5. Monitor system resources during execution

## Contributing

### Adding New Tests

1. Create test files in appropriate crate (`crates/testing/src/`)
2. Implement test functions following naming convention: `test_*`
3. Add test registration to suite discovery
4. Update documentation and examples
5. Add integration to CI/CD pipeline

### Security Rule Development

1. Research vulnerability patterns
2. Implement detection logic in `crates/security/src/`
3. Add test cases for both positive and negative scenarios
4. Document rule purpose and implementation
5. Add to default security configuration

### Performance Optimization

1. Profile test execution to identify bottlenecks
2. Implement caching for expensive operations
3. Optimize parallel execution strategies
4. Add performance regression tests
5. Document performance expectations

## Reference

### Command-Line Interface

```
USAGE:
    neo-sol-test <COMMAND>

COMMANDS:
    test         Run comprehensive test suite
    security     Run security analysis
    fuzz         Run fuzzing campaign
    conformance  Run conformance testing
    debug        Interactive debugging session
    benchmark    Performance benchmarking
    report       Generate comprehensive report
    help         Print help information

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
    -v, --verbose    Enable verbose logging
    --config <FILE>  Configuration file path
    --output-format <FORMAT>  Output format [json|human|xml|tap|junit]
```

### API Documentation

Full API documentation is available at:
- [docs.rs/neo-solidity](https://docs.rs/neo-solidity)
- Local documentation: `cargo doc --open`

### Test Result Formats

The framework supports multiple output formats:

- **Human**: Readable console output
- **JSON**: Machine-readable structured data
- **XML**: XML format for integration tools
- **TAP**: Test Anything Protocol
- **JUnit**: JUnit XML for CI/CD integration

### Performance Expectations

**Typical execution times on standard hardware:**
- Unit tests: 1-5 minutes
- Differential tests: 5-15 minutes  
- Security analysis: 2-10 minutes per contract
- Fuzzing campaign: 5 minutes to several hours
- Conformance tests: 10-30 minutes
- Performance benchmarks: 5-20 minutes

**Resource requirements:**
- RAM: 2-8 GB depending on test scope
- Storage: 1-10 GB for fuzzing corpus and test vectors
- CPU: Multi-core recommended for parallel execution

This comprehensive testing framework ensures the highest quality and security standards for the Neo Solidity implementation, providing developers and users with confidence in the reliability and compatibility of their smart contracts.