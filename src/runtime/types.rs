//! Runtime Types Module
//! 
//! Common types and utilities for Neo runtime operations.

pub use super::execution::StackItem;
use serde::{Deserialize, Serialize};

/// Re-export commonly used types
pub use super::{
    ExecutionResult, RuntimeException, ExceptionType, StateChange, StateChangeType,
    LogEntry, StackFrame, RuntimeConfig, RuntimeError, RuntimeStatistics
};

/// Value type for runtime operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RuntimeValue {
    Null,
    Boolean(bool),
    Integer(i64),
    UnsignedInteger(u64),
    ByteString(Vec<u8>),
    Array(Vec<RuntimeValue>),
    Map(std::collections::HashMap<String, RuntimeValue>),
}

/// Address type for account identification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Address(pub String);

/// Transaction hash type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionHash(pub String);

/// Block hash type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BlockHash(pub String);

/// Gas amount type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gas(pub u64);

/// Balance amount type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Balance(pub u64);

/// Block number type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockNumber(pub u64);

/// Timestamp type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(pub u64);

impl RuntimeValue {
    /// Convert to bytes representation
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            RuntimeValue::Null => vec![0],
            RuntimeValue::Boolean(b) => vec![if *b { 1 } else { 0 }],
            RuntimeValue::Integer(i) => i.to_le_bytes().to_vec(),
            RuntimeValue::UnsignedInteger(u) => u.to_le_bytes().to_vec(),
            RuntimeValue::ByteString(bytes) => bytes.clone(),
            RuntimeValue::Array(_) => {
                // Serialize as JSON for now
                serde_json::to_vec(self).unwrap_or_default()
            },
            RuntimeValue::Map(_) => {
                // Serialize as JSON for now
                serde_json::to_vec(self).unwrap_or_default()
            },
        }
    }

    /// Create from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return RuntimeValue::Null;
        }

        if bytes.len() == 8 {
            if let Ok(array) = bytes.try_into() {
                return RuntimeValue::UnsignedInteger(u64::from_le_bytes(array));
            }
        }

        RuntimeValue::ByteString(bytes.to_vec())
    }

    /// Check if value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            RuntimeValue::Null => false,
            RuntimeValue::Boolean(b) => *b,
            RuntimeValue::Integer(i) => *i != 0,
            RuntimeValue::UnsignedInteger(u) => *u != 0,
            RuntimeValue::ByteString(bytes) => !bytes.is_empty() && bytes.iter().any(|&b| b != 0),
            RuntimeValue::Array(arr) => !arr.is_empty(),
            RuntimeValue::Map(map) => !map.is_empty(),
        }
    }

    /// Get type name
    pub fn type_name(&self) -> &'static str {
        match self {
            RuntimeValue::Null => "null",
            RuntimeValue::Boolean(_) => "boolean",
            RuntimeValue::Integer(_) => "integer",
            RuntimeValue::UnsignedInteger(_) => "unsigned_integer",
            RuntimeValue::ByteString(_) => "byte_string",
            RuntimeValue::Array(_) => "array",
            RuntimeValue::Map(_) => "map",
        }
    }

    /// Convert to stack item
    pub fn to_stack_item(&self) -> StackItem {
        match self {
            RuntimeValue::Null => StackItem::Null,
            RuntimeValue::Boolean(b) => StackItem::Boolean(*b),
            RuntimeValue::Integer(i) => StackItem::Integer(*i),
            RuntimeValue::UnsignedInteger(u) => StackItem::UnsignedInteger(*u),
            RuntimeValue::ByteString(bytes) => StackItem::ByteArray(bytes.clone()),
            RuntimeValue::Array(_) => StackItem::ByteArray(self.to_bytes()),
            RuntimeValue::Map(_) => StackItem::ByteArray(self.to_bytes()),
        }
    }

    /// Create from stack item
    pub fn from_stack_item(item: &StackItem) -> Self {
        match item {
            StackItem::Null => RuntimeValue::Null,
            StackItem::Boolean(b) => RuntimeValue::Boolean(*b),
            StackItem::Integer(i) => RuntimeValue::Integer(*i),
            StackItem::UnsignedInteger(u) => RuntimeValue::UnsignedInteger(*u),
            StackItem::ByteArray(bytes) => RuntimeValue::ByteString(bytes.clone()),
        }
    }
}

impl Address {
    /// Create new address
    pub fn new(address: String) -> Result<Self, &'static str> {
        if address.len() == 42 && address.starts_with("0x") {
            Ok(Address(address))
        } else if address.len() == 40 {
            Ok(Address(format!("0x{}", address)))
        } else {
            Err("Invalid address format")
        }
    }

    /// Get address as string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get address bytes (20 bytes)
    pub fn to_bytes(&self) -> Vec<u8> {
        if self.0.starts_with("0x") {
            hex::decode(&self.0[2..]).unwrap_or_default()
        } else {
            hex::decode(&self.0).unwrap_or_default()
        }
    }

    /// Create from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Address(format!("0x{}", hex::encode(bytes)))
    }

    /// Check if address is zero
    pub fn is_zero(&self) -> bool {
        self.0 == "0x0000000000000000000000000000000000000000"
    }
}

impl TransactionHash {
    /// Create new transaction hash
    pub fn new(hash: String) -> Self {
        TransactionHash(hash)
    }

    /// Get hash as string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get hash bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        if self.0.starts_with("0x") {
            hex::decode(&self.0[2..]).unwrap_or_default()
        } else {
            hex::decode(&self.0).unwrap_or_default()
        }
    }

    /// Create from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        TransactionHash(format!("0x{}", hex::encode(bytes)))
    }
}

impl BlockHash {
    /// Create new block hash
    pub fn new(hash: String) -> Self {
        BlockHash(hash)
    }

    /// Get hash as string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get hash bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        if self.0.starts_with("0x") {
            hex::decode(&self.0[2..]).unwrap_or_default()
        } else {
            hex::decode(&self.0).unwrap_or_default()
        }
    }

    /// Create from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        BlockHash(format!("0x{}", hex::encode(bytes)))
    }
}

impl Gas {
    /// Create new gas amount
    pub fn new(amount: u64) -> Self {
        Gas(amount)
    }

    /// Get gas amount
    pub fn amount(&self) -> u64 {
        self.0
    }

    /// Add gas amounts
    pub fn add(self, other: Gas) -> Gas {
        Gas(self.0.saturating_add(other.0))
    }

    /// Subtract gas amounts
    pub fn sub(self, other: Gas) -> Gas {
        Gas(self.0.saturating_sub(other.0))
    }

    /// Check if sufficient gas
    pub fn sufficient_for(&self, required: Gas) -> bool {
        self.0 >= required.0
    }
}

impl Balance {
    /// Create new balance
    pub fn new(amount: u64) -> Self {
        Balance(amount)
    }

    /// Get balance amount
    pub fn amount(&self) -> u64 {
        self.0
    }

    /// Add balances
    pub fn add(self, other: Balance) -> Balance {
        Balance(self.0.saturating_add(other.0))
    }

    /// Subtract balances
    pub fn sub(self, other: Balance) -> Result<Balance, &'static str> {
        if self.0 >= other.0 {
            Ok(Balance(self.0 - other.0))
        } else {
            Err("Insufficient balance")
        }
    }

    /// Check if sufficient balance
    pub fn sufficient_for(&self, required: Balance) -> bool {
        self.0 >= required.0
    }

    /// Is zero balance
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl BlockNumber {
    /// Create new block number
    pub fn new(number: u64) -> Self {
        BlockNumber(number)
    }

    /// Get block number
    pub fn number(&self) -> u64 {
        self.0
    }

    /// Get next block number
    pub fn next(self) -> BlockNumber {
        BlockNumber(self.0 + 1)
    }

    /// Get previous block number
    pub fn prev(self) -> Option<BlockNumber> {
        if self.0 > 0 {
            Some(BlockNumber(self.0 - 1))
        } else {
            None
        }
    }
}

impl Timestamp {
    /// Create new timestamp
    pub fn new(timestamp: u64) -> Self {
        Timestamp(timestamp)
    }

    /// Get timestamp
    pub fn timestamp(&self) -> u64 {
        self.0
    }

    /// Get current timestamp
    pub fn now() -> Self {
        Timestamp(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        )
    }

    /// Add duration in seconds
    pub fn add_seconds(self, seconds: u64) -> Timestamp {
        Timestamp(self.0 + seconds)
    }

    /// Subtract duration in seconds
    pub fn sub_seconds(self, seconds: u64) -> Timestamp {
        Timestamp(self.0.saturating_sub(seconds))
    }
}

// Standard trait implementations

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for TransactionHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for BlockHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Gas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} gas", self.0)
    }
}

impl std::fmt::Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} wei", self.0)
    }
}

impl std::fmt::Display for BlockNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for Gas {
    fn from(amount: u64) -> Self {
        Gas(amount)
    }
}

impl From<Gas> for u64 {
    fn from(gas: Gas) -> Self {
        gas.0
    }
}

impl From<u64> for Balance {
    fn from(amount: u64) -> Self {
        Balance(amount)
    }
}

impl From<Balance> for u64 {
    fn from(balance: Balance) -> Self {
        balance.0
    }
}

impl From<u64> for BlockNumber {
    fn from(number: u64) -> Self {
        BlockNumber(number)
    }
}

impl From<BlockNumber> for u64 {
    fn from(block_number: BlockNumber) -> Self {
        block_number.0
    }
}

impl From<u64> for Timestamp {
    fn from(timestamp: u64) -> Self {
        Timestamp(timestamp)
    }
}

impl From<Timestamp> for u64 {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_value_conversion() {
        let value = RuntimeValue::UnsignedInteger(42);
        let bytes = value.to_bytes();
        let restored = RuntimeValue::from_bytes(&bytes);
        
        assert_eq!(value, restored);
        assert!(value.is_truthy());
        assert_eq!(value.type_name(), "unsigned_integer");
    }

    #[test]
    fn test_address_creation() {
        let addr1 = Address::new("0x1234567890123456789012345678901234567890".to_string()).unwrap();
        let addr2 = Address::new("1234567890123456789012345678901234567890".to_string()).unwrap();
        
        assert_eq!(addr1.as_str(), addr2.as_str());
        assert!(!addr1.is_zero());
        
        let zero_addr = Address::new("0x0000000000000000000000000000000000000000".to_string()).unwrap();
        assert!(zero_addr.is_zero());
    }

    #[test]
    fn test_gas_operations() {
        let gas1 = Gas::new(1000);
        let gas2 = Gas::new(500);
        
        let sum = gas1.add(gas2);
        assert_eq!(sum.amount(), 1500);
        
        let diff = gas1.sub(gas2);
        assert_eq!(diff.amount(), 500);
        
        assert!(gas1.sufficient_for(gas2));
        assert!(!gas2.sufficient_for(gas1));
    }

    #[test]
    fn test_balance_operations() {
        let balance1 = Balance::new(1000);
        let balance2 = Balance::new(300);
        
        let sum = balance1.add(balance2);
        assert_eq!(sum.amount(), 1300);
        
        let diff = balance1.sub(balance2).unwrap();
        assert_eq!(diff.amount(), 700);
        
        assert!(balance2.sub(balance1).is_err());
        
        assert!(balance1.sufficient_for(balance2));
        assert!(!Balance::new(0).is_zero() == false);
    }

    #[test]
    fn test_block_number_operations() {
        let block = BlockNumber::new(100);
        let next = block.next();
        let prev = next.prev().unwrap();
        
        assert_eq!(next.number(), 101);
        assert_eq!(prev.number(), 100);
        
        let genesis = BlockNumber::new(0);
        assert!(genesis.prev().is_none());
    }

    #[test]
    fn test_timestamp_operations() {
        let now = Timestamp::now();
        let future = now.add_seconds(3600);
        let past = now.sub_seconds(1800);
        
        assert!(future.timestamp() > now.timestamp());
        assert!(past.timestamp() < now.timestamp());
    }

    #[test]
    fn test_runtime_value_truthy() {
        assert!(!RuntimeValue::Null.is_truthy());
        assert!(!RuntimeValue::Boolean(false).is_truthy());
        assert!(RuntimeValue::Boolean(true).is_truthy());
        assert!(!RuntimeValue::Integer(0).is_truthy());
        assert!(RuntimeValue::Integer(42).is_truthy());
        assert!(!RuntimeValue::UnsignedInteger(0).is_truthy());
        assert!(RuntimeValue::UnsignedInteger(42).is_truthy());
        assert!(!RuntimeValue::ByteString(vec![]).is_truthy());
        assert!(RuntimeValue::ByteString(vec![1, 2, 3]).is_truthy());
    }

    #[test]
    fn test_stack_item_conversion() {
        let value = RuntimeValue::Integer(42);
        let stack_item = value.to_stack_item();
        let restored = RuntimeValue::from_stack_item(&stack_item);
        
        assert_eq!(value, restored);
    }

    #[test]
    fn test_hash_types() {
        let tx_hash = TransactionHash::new("0xabcdef1234567890".to_string());
        let block_hash = BlockHash::new("0xfedcba0987654321".to_string());
        
        assert_eq!(tx_hash.as_str(), "0xabcdef1234567890");
        assert_eq!(block_hash.as_str(), "0xfedcba0987654321");
        
        let bytes = tx_hash.to_bytes();
        let restored = TransactionHash::from_bytes(&bytes);
        assert_eq!(tx_hash.as_str(), restored.as_str());
    }
}