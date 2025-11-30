//! Mock implementations for Neo Solidity testing
//!
//! Provides mock storage, execution context, and other components

use std::collections::HashMap;

/// Mock storage implementation
#[derive(Debug, Clone, Default)]
pub struct MockStorage {
    data: HashMap<Vec<u8>, Vec<u8>>,
}

impl MockStorage {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.data.get(key).cloned()
    }

    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.data.insert(key, value);
    }

    pub fn delete(&mut self, key: &[u8]) {
        self.data.remove(key);
    }

    pub fn contains(&self, key: &[u8]) -> bool {
        self.data.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

/// Mock execution context
#[derive(Debug, Clone)]
pub struct MockExecutionContext {
    pub caller: [u8; 20],
    pub origin: [u8; 20],
    pub contract_address: [u8; 20],
    pub value: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub block_number: u64,
    pub timestamp: u64,
}

impl Default for MockExecutionContext {
    fn default() -> Self {
        Self {
            caller: [0; 20],
            origin: [0; 20],
            contract_address: [0; 20],
            value: 0,
            gas_limit: 10_000_000,
            gas_used: 0,
            block_number: 1,
            timestamp: 1700000000,
        }
    }
}

impl MockExecutionContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_caller(mut self, caller: [u8; 20]) -> Self {
        self.caller = caller;
        self
    }

    pub fn with_value(mut self, value: u64) -> Self {
        self.value = value;
        self
    }

    pub fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = gas_limit;
        self
    }

    pub fn consume_gas(&mut self, amount: u64) -> Result<(), &'static str> {
        if self.gas_used + amount > self.gas_limit {
            return Err("Out of gas");
        }
        self.gas_used += amount;
        Ok(())
    }

    pub fn gas_remaining(&self) -> u64 {
        self.gas_limit.saturating_sub(self.gas_used)
    }
}

/// Mock stack for testing
#[derive(Debug, Clone, Default)]
pub struct MockStack {
    items: Vec<Vec<u8>>,
}

impl MockStack {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, item: Vec<u8>) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<Vec<u8>> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&Vec<u8>> {
        self.items.last()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn as_slice(&self) -> &[Vec<u8>] {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_storage() {
        let mut storage = MockStorage::new();

        storage.put(vec![1, 2, 3], vec![4, 5, 6]);
        assert_eq!(storage.get(&[1, 2, 3]), Some(vec![4, 5, 6]));
        assert!(storage.contains(&[1, 2, 3]));

        storage.delete(&[1, 2, 3]);
        assert_eq!(storage.get(&[1, 2, 3]), None);
    }

    #[test]
    fn test_mock_execution_context() {
        let mut ctx = MockExecutionContext::new()
            .with_gas_limit(1000);

        assert!(ctx.consume_gas(500).is_ok());
        assert_eq!(ctx.gas_remaining(), 500);

        assert!(ctx.consume_gas(600).is_err());
    }

    #[test]
    fn test_mock_stack() {
        let mut stack = MockStack::new();

        stack.push(vec![1]);
        stack.push(vec![2]);

        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop(), Some(vec![2]));
        assert_eq!(stack.peek(), Some(&vec![1]));
    }
}
