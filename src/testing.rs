//! Testing Utilities Module
//!
//! Helpers for testing the compiler.

/// Test case result
#[derive(Debug)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub duration_ms: u64,
    pub message: Option<String>,
}

impl TestResult {
    pub fn pass(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            passed: true,
            duration_ms: 0,
            message: None,
        }
    }

    pub fn fail(name: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            passed: false,
            duration_ms: 0,
            message: Some(msg.into()),
        }
    }
}

/// Test suite
#[derive(Default)]
pub struct TestSuite {
    results: Vec<TestResult>,
}

impl TestSuite {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, result: TestResult) {
        self.results.push(result);
    }

    pub fn passed(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    pub fn failed(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    pub fn total(&self) -> usize {
        self.results.len()
    }
}
