//! Custom assertions for Neo Solidity testing
//!
//! Provides specialized assertion macros and functions

/// Assert that two bytecode sequences are equivalent
pub fn assert_bytecode_eq(expected: &[u8], actual: &[u8]) {
    if expected != actual {
        panic!(
            "Bytecode mismatch:\nExpected: {:02x?}\nActual:   {:02x?}",
            expected, actual
        );
    }
}

/// Assert that gas usage is within expected range
pub fn assert_gas_in_range(actual: u64, min: u64, max: u64) {
    if actual < min || actual > max {
        panic!(
            "Gas usage {} not in expected range [{}, {}]",
            actual, min, max
        );
    }
}

/// Assert that execution succeeded
pub fn assert_execution_success(result: &Result<Vec<u8>, String>) {
    if let Err(e) = result {
        panic!("Execution failed: {}", e);
    }
}

/// Assert that execution failed with expected error
pub fn assert_execution_error(result: &Result<Vec<u8>, String>, expected_error: &str) {
    match result {
        Ok(_) => panic!("Expected execution to fail with: {}", expected_error),
        Err(e) => {
            if !e.contains(expected_error) {
                panic!(
                    "Expected error containing '{}', got: {}",
                    expected_error, e
                );
            }
        }
    }
}

/// Assert stack state matches expected
pub fn assert_stack_eq(expected: &[Vec<u8>], actual: &[Vec<u8>]) {
    if expected.len() != actual.len() {
        panic!(
            "Stack size mismatch: expected {}, got {}",
            expected.len(),
            actual.len()
        );
    }
    for (i, (exp, act)) in expected.iter().zip(actual.iter()).enumerate() {
        if exp != act {
            panic!(
                "Stack mismatch at index {}:\nExpected: {:02x?}\nActual:   {:02x?}",
                i, exp, act
            );
        }
    }
}

/// Assert storage value matches expected
pub fn assert_storage_eq(key: &[u8], expected: &[u8], actual: &[u8]) {
    if expected != actual {
        panic!(
            "Storage mismatch for key {:02x?}:\nExpected: {:02x?}\nActual:   {:02x?}",
            key, expected, actual
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytecode_eq_pass() {
        assert_bytecode_eq(&[0x01, 0x02], &[0x01, 0x02]);
    }

    #[test]
    #[should_panic(expected = "Bytecode mismatch")]
    fn test_bytecode_eq_fail() {
        assert_bytecode_eq(&[0x01, 0x02], &[0x01, 0x03]);
    }

    #[test]
    fn test_gas_in_range_pass() {
        assert_gas_in_range(100, 50, 150);
    }

    #[test]
    #[should_panic(expected = "not in expected range")]
    fn test_gas_in_range_fail() {
        assert_gas_in_range(200, 50, 150);
    }
}
