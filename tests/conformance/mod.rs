//! NeoVM Conformance Test Suite
//!
//! This module provides infrastructure for validating the neo-solidity compiler
//! against NeoVM specification test vectors.

pub mod crypto_signature_tests;
pub mod exception_handling_tests;
pub mod gas_regression_tests;
pub mod storage_tests;

use neo_solidity::runtime::{NeoRuntime, RuntimeConfig};
use serde::{Deserialize, Serialize};

/// A single conformance test vector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestVector {
    /// Unique name for this test
    pub name: String,
    /// Description of what this test validates
    pub description: String,
    /// Solidity source code to compile
    pub source: String,
    /// Expected return value as i64
    #[serde(default)]
    pub expected_return: Option<i64>,
    /// Whether execution should succeed
    #[serde(default = "default_true")]
    pub expected_success: bool,
}

fn default_true() -> bool {
    true
}

/// Result of running a single test vector
#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

impl TestResult {
    pub fn pass(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passed: true,
            message: "PASS".to_string(),
        }
    }

    pub fn fail(name: &str, message: &str) -> Self {
        Self {
            name: name.to_string(),
            passed: false,
            message: message.to_string(),
        }
    }
}

/// Conformance test runner
pub struct ConformanceRunner {
    vectors: Vec<TestVector>,
}

impl ConformanceRunner {
    /// Create a new conformance runner with the given test vectors
    pub fn new(vectors: Vec<TestVector>) -> Self {
        Self { vectors }
    }

    /// Run all test vectors and return results
    pub fn run_all(&self) -> Vec<TestResult> {
        self.vectors.iter().map(|v| self.run_single(v)).collect()
    }

    /// Run a single test vector
    pub fn run_single(&self, vector: &TestVector) -> TestResult {
        // Compile the source using the CLI compile function
        let compile_result = neo_solidity::cli::compile_contracts(&vector.source, false, 0);

        let artifacts = match compile_result {
            Ok(artifacts) => artifacts,
            Err(e) => {
                if !vector.expected_success {
                    return TestResult::pass(&vector.name);
                }
                return TestResult::fail(&vector.name, &format!("Compilation failed: {:?}", e));
            }
        };

        // Use the last contract (usually the main one)
        let artifact = match artifacts.last() {
            Some(a) => a,
            None => {
                return TestResult::fail(&vector.name, "No contract artifact found");
            }
        };

        // Execute the bytecode
        let mut runtime = match NeoRuntime::new(RuntimeConfig::default()) {
            Ok(r) => r,
            Err(e) => {
                return TestResult::fail(
                    &vector.name,
                    &format!("Runtime creation failed: {:?}", e),
                );
            }
        };

        let execution_result = match runtime.execute(&artifact.bytecode, &[]) {
            Ok(r) => r,
            Err(e) => {
                return TestResult::fail(&vector.name, &format!("Execution failed: {:?}", e));
            }
        };

        // Check results
        if vector.expected_success {
            if !execution_result.is_success() {
                let msg = execution_result
                    .exception
                    .as_ref()
                    .map(|e| e.message.as_str())
                    .unwrap_or("unknown error");
                return TestResult::fail(
                    &vector.name,
                    &format!("Expected success but got: {}", msg),
                );
            }

            // Check return value if specified
            if let Some(expected) = vector.expected_return {
                let expected_bytes = expected.to_le_bytes().to_vec();
                if execution_result.return_data != expected_bytes {
                    return TestResult::fail(
                        &vector.name,
                        &format!(
                            "Return value mismatch: expected {}, got {:?}",
                            expected, execution_result.return_data
                        ),
                    );
                }
            }

            TestResult::pass(&vector.name)
        } else {
            if execution_result.is_success() {
                return TestResult::fail(&vector.name, "Expected failure but execution succeeded");
            }
            TestResult::pass(&vector.name)
        }
    }

    /// Get summary statistics from results
    pub fn summary(results: &[TestResult]) -> ConformanceSummary {
        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let failed = total - passed;
        let pass_rate = if total > 0 {
            (passed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        ConformanceSummary {
            total,
            passed,
            failed,
            pass_rate,
        }
    }
}

/// Summary of conformance test results
#[derive(Debug)]
pub struct ConformanceSummary {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub pass_rate: f64,
}

impl std::fmt::Display for ConformanceSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Conformance Tests: {}/{} passed ({:.1}%)",
            self.passed, self.total, self.pass_rate
        )
    }
}

/// Built-in test vectors for basic validation
pub fn basic_test_vectors() -> Vec<TestVector> {
    vec![
        TestVector {
            name: "simple_return".to_string(),
            description: "Simple function that returns a constant".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract SimpleReturn {
                    function run() public pure returns (uint256) {
                        return 42;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        TestVector {
            name: "arithmetic_add".to_string(),
            description: "Basic addition operation".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract ArithmeticAdd {
                    function run() public pure returns (uint256) {
                        return 10 + 32;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        TestVector {
            name: "arithmetic_mul".to_string(),
            description: "Basic multiplication operation".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract ArithmeticMul {
                    function run() public pure returns (uint256) {
                        return 6 * 7;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(42),
            expected_success: true,
        },
        TestVector {
            name: "boolean_true".to_string(),
            description: "Boolean true returns 1".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract BooleanTrue {
                    function run() public pure returns (uint256) {
                        bool b = true;
                        if (b) return 1;
                        return 0;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(1),
            expected_success: true,
        },
        TestVector {
            name: "boolean_false".to_string(),
            description: "Boolean false returns 0".to_string(),
            source: r#"
                pragma solidity ^0.8.20;
                contract BooleanFalse {
                    function run() public pure returns (uint256) {
                        bool b = false;
                        if (b) return 1;
                        return 0;
                    }
                }
            "#
            .to_string(),
            expected_return: Some(0),
            expected_success: true,
        },
    ]
}
