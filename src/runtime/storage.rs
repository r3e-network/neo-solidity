//! Storage Management Module
//! 
//! Provides persistent storage for smart contracts with Neo blockchain compatibility.

use super::{RuntimeConfig, RuntimeError};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};

/// Storage manager for contract storage
#[derive(Debug)]
pub struct StorageManager {
    storage: HashMap<String, AccountStorage>,
    read_count: u64,
    write_count: u64,
    gas_costs: StorageGasCosts,
}

/// Account-specific storage
#[derive(Debug, Clone)]
pub struct AccountStorage {
    pub account: String,
    pub storage: BTreeMap<Vec<u8>, Vec<u8>>,
    pub pending_changes: HashMap<Vec<u8>, StorageChange>,
}

/// Storage change record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageChange {
    pub key: Vec<u8>,
    pub old_value: Option<Vec<u8>>,
    pub new_value: Option<Vec<u8>>,
    pub change_type: StorageChangeType,
    pub gas_cost: u64,
}

/// Types of storage changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageChangeType {
    Create,     // New key-value pair
    Update,     // Update existing value
    Delete,     // Delete key-value pair
    NoChange,   // Value unchanged
}

/// Storage gas costs (Neo N3 compatible)
#[derive(Debug, Clone)]
pub struct StorageGasCosts {
    pub sstore_set: u64,      // Store new value
    pub sstore_reset: u64,    // Update existing value
    pub sstore_clear: u64,    // Clear existing value
    pub sload: u64,           // Load value
    pub storage_byte_cost: u64, // Cost per byte stored
}

/// Storage query options
#[derive(Debug)]
pub struct StorageQuery {
    pub account: String,
    pub key_prefix: Option<Vec<u8>>,
    pub limit: Option<usize>,
    pub include_pending: bool,
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStatistics {
    pub total_accounts: usize,
    pub total_keys: usize,
    pub total_storage_bytes: usize,
    pub read_operations: u64,
    pub write_operations: u64,
    pub pending_changes: usize,
}

impl StorageManager {
    /// Create new storage manager
    pub fn new(_config: &RuntimeConfig) -> Result<Self, RuntimeError> {
        Ok(Self {
            storage: HashMap::new(),
            read_count: 0,
            write_count: 0,
            gas_costs: StorageGasCosts::default(),
        })
    }

    /// Get storage value
    pub fn get(&mut self, account: &str, key: &[u8]) -> Result<Option<Vec<u8>>, RuntimeError> {
        self.read_count += 1;

        let account_storage = self.storage.get(account);
        
        if let Some(storage) = account_storage {
            // Check pending changes first
            if let Some(change) = storage.pending_changes.get(key) {
                match change.change_type {
                    StorageChangeType::Delete => return Ok(None),
                    _ => {
                        if let Some(ref new_value) = change.new_value {
                            return Ok(Some(new_value.clone()));
                        }
                    }
                }
            }
            
            // Check committed storage
            if let Some(value) = storage.storage.get(key) {
                return Ok(Some(value.clone()));
            }
        }

        Ok(None)
    }

    /// Set storage value
    pub fn set(&mut self, account: &str, key: &[u8], value: &[u8]) -> Result<(), RuntimeError> {
        self.write_count += 1;

        let old_value = self.get(account, key)?;
        
        // Calculate gas cost
        let change_type = match &old_value {
            None => StorageChangeType::Create,
            Some(old) if old == value => StorageChangeType::NoChange,
            Some(_) if value.is_empty() => StorageChangeType::Delete,
            Some(_) => StorageChangeType::Update,
        };

        let gas_cost = self.calculate_storage_gas_cost(&change_type, value.len());

        // Get or create account storage
        let account_storage = self.storage.entry(account.to_string()).or_insert_with(|| {
            AccountStorage {
                account: account.to_string(),
                storage: BTreeMap::new(),
                pending_changes: HashMap::new(),
            }
        });

        // Record pending change
        let change = StorageChange {
            key: key.to_vec(),
            old_value,
            new_value: if value.is_empty() { None } else { Some(value.to_vec()) },
            change_type,
            gas_cost,
        };

        account_storage.pending_changes.insert(key.to_vec(), change);

        Ok(())
    }

    /// Delete storage value
    pub fn delete(&mut self, account: &str, key: &[u8]) -> Result<(), RuntimeError> {
        self.set(account, key, &[])
    }

    /// Check if storage key exists
    pub fn exists(&mut self, account: &str, key: &[u8]) -> Result<bool, RuntimeError> {
        Ok(self.get(account, key)?.is_some())
    }

    /// Get storage size for account
    pub fn get_storage_size(&self, account: &str) -> Result<usize, RuntimeError> {
        if let Some(storage) = self.storage.get(account) {
            let committed_size: usize = storage.storage.iter()
                .map(|(k, v)| k.len() + v.len())
                .sum();
            
            let pending_size: usize = storage.pending_changes.iter()
                .map(|(k, change)| {
                    k.len() + change.new_value.as_ref().map(|v| v.len()).unwrap_or(0)
                })
                .sum();
            
            Ok(committed_size + pending_size)
        } else {
            Ok(0)
        }
    }

    /// Commit pending changes
    pub fn commit(&mut self, account: &str) -> Result<Vec<StorageChange>, RuntimeError> {
        if let Some(account_storage) = self.storage.get_mut(account) {
            let changes: Vec<StorageChange> = account_storage.pending_changes.values().cloned().collect();
            
            // Apply changes to committed storage
            for (key, change) in account_storage.pending_changes.drain() {
                match change.change_type {
                    StorageChangeType::Create | StorageChangeType::Update => {
                        if let Some(new_value) = change.new_value {
                            account_storage.storage.insert(key, new_value);
                        }
                    },
                    StorageChangeType::Delete => {
                        account_storage.storage.remove(&key);
                    },
                    StorageChangeType::NoChange => {
                        // No change needed
                    }
                }
            }
            
            Ok(changes)
        } else {
            Ok(Vec::new())
        }
    }

    /// Rollback pending changes
    pub fn rollback(&mut self, account: &str) -> Result<(), RuntimeError> {
        if let Some(account_storage) = self.storage.get_mut(account) {
            account_storage.pending_changes.clear();
        }
        Ok(())
    }

    /// Get all pending changes for account
    pub fn get_pending_changes(&self, account: &str) -> Vec<&StorageChange> {
        self.storage.get(account)
            .map(|storage| storage.pending_changes.values().collect())
            .unwrap_or_default()
    }

    /// Query storage with filters
    pub fn query(&mut self, query: StorageQuery) -> Result<Vec<(Vec<u8>, Vec<u8>)>, RuntimeError> {
        let mut results = Vec::new();

        if let Some(account_storage) = self.storage.get(&query.account) {
            // Collect from committed storage
            for (key, value) in &account_storage.storage {
                if let Some(ref prefix) = query.key_prefix {
                    if !key.starts_with(prefix) {
                        continue;
                    }
                }
                results.push((key.clone(), value.clone()));
            }

            // Include pending changes if requested
            if query.include_pending {
                for (key, change) in &account_storage.pending_changes {
                    if let Some(ref prefix) = query.key_prefix {
                        if !key.starts_with(prefix) {
                            continue;
                        }
                    }
                    
                    match &change.change_type {
                        StorageChangeType::Delete => {
                            results.retain(|(k, _)| k != key);
                        },
                        _ => {
                            if let Some(ref new_value) = change.new_value {
                                // Remove old entry if exists and add new one
                                results.retain(|(k, _)| k != key);
                                results.push((key.clone(), new_value.clone()));
                            }
                        }
                    }
                }
            }
        }

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        // Sort by key for consistent results
        results.sort_by(|a, b| a.0.cmp(&b.0));

        Ok(results)
    }

    /// Get storage root hash for account (Neo compatibility)
    pub fn get_storage_root(&self, account: &str) -> Result<String, RuntimeError> {
        if let Some(account_storage) = self.storage.get(account) {
            // Calculate Merkle root of storage items
            let mut items: Vec<_> = account_storage.storage.iter().collect();
            items.sort_by_key(|(k, _)| *k);
            
            let root_hash = self.calculate_merkle_root(&items);
            Ok(hex::encode(root_hash))
        } else {
            Ok(hex::encode(self.empty_root_hash()))
        }
    }

    /// Calculate total gas cost for pending changes
    pub fn calculate_pending_gas_cost(&self, account: &str) -> u64 {
        self.storage.get(account)
            .map(|storage| storage.pending_changes.values().map(|c| c.gas_cost).sum())
            .unwrap_or(0)
    }

    /// Get read count
    pub fn read_count(&self) -> u64 {
        self.read_count
    }

    /// Get write count
    pub fn write_count(&self) -> u64 {
        self.write_count
    }

    /// Get storage statistics
    pub fn get_statistics(&self) -> StorageStatistics {
        let total_accounts = self.storage.len();
        let total_keys: usize = self.storage.values()
            .map(|s| s.storage.len())
            .sum();
        let total_storage_bytes: usize = self.storage.values()
            .flat_map(|s| s.storage.iter())
            .map(|(k, v)| k.len() + v.len())
            .sum();
        let pending_changes: usize = self.storage.values()
            .map(|s| s.pending_changes.len())
            .sum();

        StorageStatistics {
            total_accounts,
            total_keys,
            total_storage_bytes,
            read_operations: self.read_count,
            write_operations: self.write_count,
            pending_changes,
        }
    }

    /// Clear all storage (for testing)
    pub fn clear(&mut self) {
        self.storage.clear();
        self.read_count = 0;
        self.write_count = 0;
    }

    // Private helper methods

    fn calculate_storage_gas_cost(&self, change_type: &StorageChangeType, value_size: usize) -> u64 {
        match change_type {
            StorageChangeType::Create => {
                self.gas_costs.sstore_set + (value_size as u64 * self.gas_costs.storage_byte_cost)
            },
            StorageChangeType::Update => {
                self.gas_costs.sstore_reset + (value_size as u64 * self.gas_costs.storage_byte_cost)
            },
            StorageChangeType::Delete => self.gas_costs.sstore_clear,
            StorageChangeType::NoChange => 0,
        }
    }

    fn calculate_merkle_root(&self, items: &[(&Vec<u8>, &Vec<u8>)]) -> Vec<u8> {
        use sha3::{Digest, Keccak256};
        
        if items.is_empty() {
            return self.empty_root_hash();
        }

        // Simple implementation - hash all items together
        // In a real implementation, this would build a proper Merkle tree
        let mut hasher = Keccak256::new();
        for (key, value) in items {
            hasher.update(key);
            hasher.update(value);
        }
        hasher.finalize().to_vec()
    }

    fn empty_root_hash(&self) -> Vec<u8> {
        use sha3::{Digest, Keccak256};
        Keccak256::digest(b"").to_vec()
    }
}

impl Default for StorageGasCosts {
    fn default() -> Self {
        // Neo N3 compatible gas costs
        Self {
            sstore_set: 8000,     // Create new storage entry
            sstore_reset: 8000,   // Update existing storage entry
            sstore_clear: 8000,   // Delete storage entry
            sload: 4000,          // Load from storage
            storage_byte_cost: 400, // Cost per byte stored
        }
    }
}

impl StorageQuery {
    /// Create new storage query
    pub fn new(account: String) -> Self {
        Self {
            account,
            key_prefix: None,
            limit: None,
            include_pending: false,
        }
    }

    /// Set key prefix filter
    pub fn with_prefix(mut self, prefix: Vec<u8>) -> Self {
        self.key_prefix = Some(prefix);
        self
    }

    /// Set result limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Include pending changes in results
    pub fn include_pending(mut self) -> Self {
        self.include_pending = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeConfig;

    #[test]
    fn test_storage_operations() {
        let config = RuntimeConfig::default();
        let mut storage = StorageManager::new(&config).unwrap();
        
        let account = "0x1234567890123456789012345678901234567890";
        let key = b"test_key";
        let value = b"test_value";
        
        // Initially empty
        assert!(storage.get(account, key).unwrap().is_none());
        assert!(!storage.exists(account, key).unwrap());
        
        // Set value
        storage.set(account, key, value).unwrap();
        assert_eq!(storage.get(account, key).unwrap(), Some(value.to_vec()));
        assert!(storage.exists(account, key).unwrap());
        
        // Update value
        let new_value = b"updated_value";
        storage.set(account, key, new_value).unwrap();
        assert_eq!(storage.get(account, key).unwrap(), Some(new_value.to_vec()));
        
        // Delete value
        storage.delete(account, key).unwrap();
        assert!(storage.get(account, key).unwrap().is_none());
    }

    #[test]
    fn test_pending_changes() {
        let config = RuntimeConfig::default();
        let mut storage = StorageManager::new(&config).unwrap();
        
        let account = "0x1234567890123456789012345678901234567890";
        let key = b"test_key";
        let value = b"test_value";
        
        // Set value (creates pending change)
        storage.set(account, key, value).unwrap();
        
        // Should have pending changes
        let pending = storage.get_pending_changes(account);
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].key, key);
        assert_eq!(pending[0].new_value, Some(value.to_vec()));
        
        // Commit changes
        let changes = storage.commit(account).unwrap();
        assert_eq!(changes.len(), 1);
        
        // No more pending changes
        let pending = storage.get_pending_changes(account);
        assert!(pending.is_empty());
        
        // Value still accessible
        assert_eq!(storage.get(account, key).unwrap(), Some(value.to_vec()));
    }

    #[test]
    fn test_rollback() {
        let config = RuntimeConfig::default();
        let mut storage = StorageManager::new(&config).unwrap();
        
        let account = "0x1234567890123456789012345678901234567890";
        let key = b"test_key";
        let value = b"test_value";
        
        // Set initial value and commit
        storage.set(account, key, value).unwrap();
        storage.commit(account).unwrap();
        
        // Update value (pending)
        let new_value = b"updated_value";
        storage.set(account, key, new_value).unwrap();
        assert_eq!(storage.get(account, key).unwrap(), Some(new_value.to_vec()));
        
        // Rollback
        storage.rollback(account).unwrap();
        
        // Should return to committed value
        assert_eq!(storage.get(account, key).unwrap(), Some(value.to_vec()));
        assert!(storage.get_pending_changes(account).is_empty());
    }

    #[test]
    fn test_storage_query() {
        let config = RuntimeConfig::default();
        let mut storage = StorageManager::new(&config).unwrap();
        
        let account = "0x1234567890123456789012345678901234567890";
        
        // Set up test data
        storage.set(account, b"prefix_key1", b"value1").unwrap();
        storage.set(account, b"prefix_key2", b"value2").unwrap();
        storage.set(account, b"other_key", b"value3").unwrap();
        storage.commit(account).unwrap();
        
        // Query with prefix
        let query = StorageQuery::new(account.to_string())
            .with_prefix(b"prefix_".to_vec());
        let results = storage.query(query).unwrap();
        
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|(k, v)| k == b"prefix_key1" && v == b"value1"));
        assert!(results.iter().any(|(k, v)| k == b"prefix_key2" && v == b"value2"));
        
        // Query with limit
        let query = StorageQuery::new(account.to_string())
            .with_limit(2);
        let results = storage.query(query).unwrap();
        
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_gas_cost_calculation() {
        let config = RuntimeConfig::default();
        let mut storage = StorageManager::new(&config).unwrap();
        
        let account = "0x1234567890123456789012345678901234567890";
        let key = b"test_key";
        let value = b"test_value";
        
        // Set value
        storage.set(account, key, value).unwrap();
        
        let gas_cost = storage.calculate_pending_gas_cost(account);
        assert!(gas_cost > 0);
        
        // Cost should include base cost plus per-byte cost
        let expected_cost = storage.gas_costs.sstore_set + 
                           (value.len() as u64 * storage.gas_costs.storage_byte_cost);
        assert_eq!(gas_cost, expected_cost);
    }

    #[test]
    fn test_storage_statistics() {
        let config = RuntimeConfig::default();
        let mut storage = StorageManager::new(&config).unwrap();
        
        let account1 = "0x1111111111111111111111111111111111111111";
        let account2 = "0x2222222222222222222222222222222222222222";
        
        // Add some data
        storage.set(account1, b"key1", b"value1").unwrap();
        storage.set(account1, b"key2", b"value2").unwrap();
        storage.set(account2, b"key3", b"value3").unwrap();
        
        // Some committed, some pending
        storage.commit(account1).unwrap();
        
        let stats = storage.get_statistics();
        assert_eq!(stats.total_accounts, 2);
        assert!(stats.total_keys >= 2); // At least the committed ones
        assert!(stats.total_storage_bytes > 0);
        assert!(stats.read_operations > 0);
        assert!(stats.write_operations > 0);
        assert!(stats.pending_changes > 0);
    }

    #[test]
    fn test_storage_root_hash() {
        let config = RuntimeConfig::default();
        let mut storage = StorageManager::new(&config).unwrap();
        
        let account = "0x1234567890123456789012345678901234567890";
        
        // Empty storage should have empty root
        let empty_root = storage.get_storage_root(account).unwrap();
        assert!(!empty_root.is_empty());
        
        // Add some data and commit
        storage.set(account, b"key1", b"value1").unwrap();
        storage.set(account, b"key2", b"value2").unwrap();
        storage.commit(account).unwrap();
        
        let root_with_data = storage.get_storage_root(account).unwrap();
        assert_ne!(empty_root, root_with_data);
        
        // Same data should produce same root
        let root_again = storage.get_storage_root(account).unwrap();
        assert_eq!(root_with_data, root_again);
    }
}