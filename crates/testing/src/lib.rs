//! Comprehensive Testing Framework for Neo Solidity
//! 
//! This crate provides a complete testing infrastructure including:
//! - Unit tests for runtime primitives
//! - Differential testing between EVM and NeoVM
//! - Property-based testing
//! - Performance benchmarking
//! - Integration test suites

pub mod unit_tests;
pub mod differential;
pub mod property_based;
pub mod integration;
pub mod benchmarks;
pub mod test_data;
pub mod assertions;
pub mod mocks;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Core test runner for Neo Solidity
pub struct TestRunner {
    config: TestConfig,
    results: TestResults,
}

/// Configuration for test execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub parallel_execution: bool,
    pub max_execution_time_ms: u64,
    pub enable_tracing: bool,
    pub output_format: OutputFormat,
    pub test_suites: Vec<TestSuite>,
}

/// Test suite definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub name: String,
    pub description: String,
    pub test_type: TestType,
    pub enabled: bool,
    pub timeout_ms: u64,
    pub retry_count: u32,
}

/// Types of tests supported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    Unit,
    Integration,
    Differential,
    PropertyBased,
    Performance,
    Security,
    Conformance,
}

/// Test execution results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    pub total_tests: u32,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub duration_ms: u64,
    pub suite_results: HashMap<String, SuiteResult>,
}

/// Results for a specific test suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteResult {
    pub suite_name: String,
    pub tests_run: u32,
    pub passed: u32,
    pub failed: u32,
    pub duration_ms: u64,
    pub failures: Vec<TestFailure>,
    pub performance_metrics: Option<PerformanceMetrics>,
}

/// Individual test failure information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestFailure {
    pub test_name: String,
    pub error_message: String,
    pub stack_trace: Option<String>,
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

/// Performance metrics for benchmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub avg_execution_time_ns: u64,
    pub min_execution_time_ns: u64,
    pub max_execution_time_ns: u64,
    pub throughput_ops_per_sec: f64,
    pub memory_usage_bytes: u64,
    pub gas_consumption: u64,
}

/// Output formats for test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Human,
    Json,
    Xml,
    Tap,
    JUnit,
}

/// Individual test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestResult {
    Passed {
        name: String,
        duration_ms: u64,
        metrics: Option<PerformanceMetrics>,
    },
    Failed {
        name: String,
        duration_ms: u64,
        error: TestFailure,
    },
    Skipped {
        name: String,
        reason: String,
    },
}

impl TestRunner {
    /// Create a new test runner with configuration
    pub fn new(config: TestConfig) -> Self {
        Self {
            config,
            results: TestResults::default(),
        }
    }

    /// Run all configured test suites
    pub async fn run_all(&mut self) -> Result<TestResults> {
        tracing::info!("Starting comprehensive test suite execution");
        
        let start_time = std::time::Instant::now();
        let mut total_tests = 0;
        let mut passed = 0;
        let mut failed = 0;
        let mut skipped = 0;

        for suite in &self.config.test_suites {
            if !suite.enabled {
                tracing::debug!("Skipping disabled test suite: {}", suite.name);
                continue;
            }

            let suite_result = self.run_suite(suite).await?;
            total_tests += suite_result.tests_run;
            passed += suite_result.passed;
            failed += suite_result.failed;

            self.results.suite_results.insert(
                suite.name.clone(),
                suite_result,
            );
        }

        self.results.total_tests = total_tests;
        self.results.passed = passed;
        self.results.failed = failed;
        self.results.skipped = skipped;
        self.results.duration_ms = start_time.elapsed().as_millis() as u64;

        tracing::info!(
            "Test execution completed: {}/{} passed in {}ms",
            passed,
            total_tests,
            self.results.duration_ms
        );

        Ok(self.results.clone())
    }

    /// Run a specific test suite
    async fn run_suite(&self, suite: &TestSuite) -> Result<SuiteResult> {
        tracing::info!("Running test suite: {}", suite.name);
        
        let start_time = std::time::Instant::now();
        
        let suite_result = match suite.test_type {
            TestType::Unit => {
                unit_tests::run_unit_tests(&suite.name).await
            }
            TestType::Integration => {
                integration::run_integration_tests(&suite.name).await
            }
            TestType::Differential => {
                differential::run_differential_tests(&suite.name).await
            }
            TestType::PropertyBased => {
                property_based::run_property_tests(&suite.name).await
            }
            TestType::Performance => {
                benchmarks::run_benchmarks(&suite.name).await
            }
            TestType::Security => {
                // Security tests handled by security crate
                todo!("Security test integration")
            }
            TestType::Conformance => {
                // Conformance tests handled by conformance crate
                todo!("Conformance test integration")
            }
        };

        let mut result = suite_result?;
        result.duration_ms = start_time.elapsed().as_millis() as u64;
        
        Ok(result)
    }

    /// Generate test report in specified format
    pub fn generate_report(&self) -> Result<String> {
        match self.config.output_format {
            OutputFormat::Json => {
                serde_json::to_string_pretty(&self.results)
                    .map_err(|e| anyhow::anyhow!("Failed to serialize results: {}", e))
            }
            OutputFormat::Human => self.generate_human_report(),
            OutputFormat::JUnit => self.generate_junit_report(),
            _ => todo!("Additional output formats"),
        }
    }

    fn generate_human_report(&self) -> Result<String> {
        let mut report = String::new();
        
        report.push_str(&format!(
            "Neo Solidity Test Results\n{}\n\n",
            "=".repeat(50)
        ));

        report.push_str(&format!(
            "Summary: {}/{} tests passed ({:.1}%)\n",
            self.results.passed,
            self.results.total_tests,
            (self.results.passed as f64 / self.results.total_tests as f64) * 100.0
        ));

        report.push_str(&format!(
            "Duration: {}ms\n\n",
            self.results.duration_ms
        ));

        for (suite_name, suite_result) in &self.results.suite_results {
            report.push_str(&format!(
                "Suite: {} - {}/{} passed\n",
                suite_name,
                suite_result.passed,
                suite_result.tests_run
            ));

            if !suite_result.failures.is_empty() {
                report.push_str("Failures:\n");
                for failure in &suite_result.failures {
                    report.push_str(&format!(
                        "  - {}: {}\n",
                        failure.test_name,
                        failure.error_message
                    ));
                }
            }

            report.push('\n');
        }

        Ok(report)
    }

    fn generate_junit_report(&self) -> Result<String> {
        // JUnit XML format implementation
        todo!("JUnit XML report generation")
    }
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            parallel_execution: true,
            max_execution_time_ms: 300000, // 5 minutes
            enable_tracing: true,
            output_format: OutputFormat::Human,
            test_suites: vec![
                TestSuite {
                    name: "runtime_primitives".to_string(),
                    description: "Runtime primitive unit tests".to_string(),
                    test_type: TestType::Unit,
                    enabled: true,
                    timeout_ms: 30000,
                    retry_count: 0,
                },
                TestSuite {
                    name: "differential_evm_neo".to_string(),
                    description: "Differential testing between EVM and NeoVM".to_string(),
                    test_type: TestType::Differential,
                    enabled: true,
                    timeout_ms: 60000,
                    retry_count: 1,
                },
                TestSuite {
                    name: "property_based".to_string(),
                    description: "Property-based testing with random inputs".to_string(),
                    test_type: TestType::PropertyBased,
                    enabled: true,
                    timeout_ms: 120000,
                    retry_count: 0,
                },
            ],
        }
    }
}

impl Default for TestResults {
    fn default() -> Self {
        Self {
            total_tests: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            duration_ms: 0,
            suite_results: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_runner_creation() {
        let config = TestConfig::default();
        let runner = TestRunner::new(config);
        assert_eq!(runner.results.total_tests, 0);
    }

    #[test]
    fn test_config_serialization() {
        let config = TestConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: TestConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.parallel_execution, deserialized.parallel_execution);
    }
}