//! State Management Module
//! 
//! Manages account states, balances, and transaction state for Neo runtime.

use super::{RuntimeConfig, RuntimeError, StateChange, StateChangeType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// State manager for account and contract states
#[derive(Debug, Clone)]
pub struct StateManager {
    accounts: HashMap<String, AccountState>,
    snapshots: Vec<StateSnapshot>,
    change_log: Vec<StateChange>,
    change_count: u64,
}

/// Account state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountState {
    pub address: String,
    pub balance: u64,
    pub nonce: u64,
    pub code: Option<Vec<u8>>,
    pub code_hash: Option<String>,
    pub storage_root: Option<String>,
    pub created_at: u64,
}

/// State snapshot for rollback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub id: u64,
    pub timestamp: u64,
    pub accounts: HashMap<String, AccountState>,
    pub description: String,
}

/// State query interface
#[derive(Debug)]
pub struct StateQuery {
    pub account: Option<String>,
    pub include_code: bool,
    pub include_storage: bool,
    pub block_number: Option<u64>,
}

/// State update batch
#[derive(Debug)]
pub struct StateBatch {
    pub changes: Vec<StateChange>,
    pub atomic: bool,
}

impl StateManager {
    /// Create new state manager
    pub fn new(_config: &RuntimeConfig) -> Result<Self, RuntimeError> {
        Ok(Self {
            accounts: HashMap::new(),
            snapshots: Vec::new(),
            change_log: Vec::new(),
            change_count: 0,
        })
    }

    /// Get account state
    pub fn get_account(&self, address: &str) -> Option<&AccountState> {
        self.accounts.get(address)
    }

    /// Get account balance
    pub fn get_balance(&self, address: &str) -> Result<u64, RuntimeError> {
        Ok(self.accounts.get(address)
            .map(|account| account.balance)
            .unwrap_or(0))
    }

    /// Set account balance
    pub fn set_balance(&mut self, address: &str, balance: u64) -> Result<(), RuntimeError> {
        let old_balance = self.get_balance(address)?;
        
        let account = self.accounts.entry(address.to_string()).or_insert_with(|| {
            AccountState {
                address: address.to_string(),
                balance: 0,
                nonce: 0,
                code: None,
                code_hash: None,
                storage_root: None,
                created_at: self.current_timestamp(),
            }
        });

        account.balance = balance;

        // Record state change
        self.record_change(StateChange {
            change_type: StateChangeType::BalanceChange,
            account: address.to_string(),
            key: None,
            old_value: Some(old_balance.to_le_bytes().to_vec()),
            new_value: balance.to_le_bytes().to_vec(),
        });

        Ok(())
    }

    /// Get account nonce
    pub fn get_nonce(&self, address: &str) -> Result<u64, RuntimeError> {
        Ok(self.accounts.get(address)
            .map(|account| account.nonce)
            .unwrap_or(0))
    }

    /// Set account nonce
    pub fn set_nonce(&mut self, address: &str, nonce: u64) -> Result<(), RuntimeError> {
        let old_nonce = self.get_nonce(address)?;
        
        let account = self.accounts.entry(address.to_string()).or_insert_with(|| {
            AccountState {
                address: address.to_string(),
                balance: 0,
                nonce: 0,
                code: None,
                code_hash: None,
                storage_root: None,
                created_at: self.current_timestamp(),
            }
        });

        account.nonce = nonce;

        // Record state change
        self.record_change(StateChange {
            change_type: StateChangeType::NonceChange,
            account: address.to_string(),
            key: None,
            old_value: Some(old_nonce.to_le_bytes().to_vec()),
            new_value: nonce.to_le_bytes().to_vec(),
        });

        Ok(())
    }

    /// Get contract code
    pub fn get_code(&self, address: &str) -> Option<&[u8]> {
        self.accounts.get(address)
            .and_then(|account| account.code.as_ref())
            .map(|code| code.as_slice())
    }

    /// Set contract code
    pub fn set_code(&mut self, address: &str, code: &[u8]) -> Result<(), RuntimeError> {
        let old_code = self.get_code(address).map(|c| c.to_vec());
        
        let account = self.accounts.entry(address.to_string()).or_insert_with(|| {
            AccountState {
                address: address.to_string(),
                balance: 0,
                nonce: 0,
                code: None,
                code_hash: None,
                storage_root: None,
                created_at: self.current_timestamp(),
            }
        });

        // Calculate code hash
        let code_hash = self.calculate_hash(code);
        account.code = Some(code.to_vec());
        account.code_hash = Some(code_hash);

        // Record state change
        self.record_change(StateChange {
            change_type: StateChangeType::CodeChange,
            account: address.to_string(),
            key: None,
            old_value: old_code,
            new_value: code.to_vec(),
        });

        Ok(())
    }

    /// Create new account
    pub fn create_account(&mut self, address: &str, initial_balance: u64) -> Result<(), RuntimeError> {
        if self.accounts.contains_key(address) {
            return Err(RuntimeError::StateError {
                message: format!("Account {} already exists", address),
            });
        }

        let account = AccountState {
            address: address.to_string(),
            balance: initial_balance,
            nonce: 0,
            code: None,
            code_hash: None,
            storage_root: None,
            created_at: self.current_timestamp(),
        };

        self.accounts.insert(address.to_string(), account);

        // Record state change
        self.record_change(StateChange {
            change_type: StateChangeType::AccountCreation,
            account: address.to_string(),
            key: None,
            old_value: None,
            new_value: initial_balance.to_le_bytes().to_vec(),
        });

        Ok(())
    }

    /// Delete account
    pub fn delete_account(&mut self, address: &str) -> Result<(), RuntimeError> {
        if let Some(account) = self.accounts.remove(address) {
            // Record state change
            self.record_change(StateChange {
                change_type: StateChangeType::AccountDeletion,
                account: address.to_string(),
                key: None,
                old_value: Some(account.balance.to_le_bytes().to_vec()),
                new_value: vec![],
            });
        }

        Ok(())
    }

    /// Check if account exists
    pub fn account_exists(&self, address: &str) -> bool {
        self.accounts.contains_key(address)
    }

    /// Transfer balance between accounts
    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), RuntimeError> {
        let from_balance = self.get_balance(from)?;
        let to_balance = self.get_balance(to)?;

        if from_balance < amount {
            return Err(RuntimeError::StateError {
                message: "Insufficient balance".to_string(),
            });
        }

        self.set_balance(from, from_balance - amount)?;
        self.set_balance(to, to_balance + amount)?;

        Ok(())
    }

    /// Create state snapshot
    pub fn create_snapshot(&mut self, description: String) -> u64 {
        let snapshot_id = self.snapshots.len() as u64;
        let snapshot = StateSnapshot {
            id: snapshot_id,
            timestamp: self.current_timestamp(),
            accounts: self.accounts.clone(),
            description,
        };

        self.snapshots.push(snapshot);
        snapshot_id
    }

    /// Get state snapshot
    pub fn get_snapshot(&self) -> StateSnapshot {
        StateSnapshot {
            id: u64::MAX,
            timestamp: self.current_timestamp(),
            accounts: self.accounts.clone(),
            description: "Current state".to_string(),
        }
    }

    /// Restore from snapshot
    pub fn restore_snapshot(&mut self, snapshot: StateSnapshot) -> Result<(), RuntimeError> {
        // Record changes for each account
        for (address, account) in &snapshot.accounts {
            if let Some(current_account) = self.accounts.get(address) {
                if current_account.balance != account.balance {
                    self.record_change(StateChange {
                        change_type: StateChangeType::BalanceChange,
                        account: address.clone(),
                        key: None,
                        old_value: Some(current_account.balance.to_le_bytes().to_vec()),
                        new_value: account.balance.to_le_bytes().to_vec(),
                    });
                }
            }
        }

        self.accounts = snapshot.accounts;
        Ok(())
    }

    /// Get change count
    pub fn change_count(&self) -> u64 {
        self.change_count
    }

    /// Get recent changes
    pub fn get_recent_changes(&self, count: usize) -> &[StateChange] {
        let start = self.change_log.len().saturating_sub(count);
        &self.change_log[start..]
    }

    /// Clear change log
    pub fn clear_change_log(&mut self) {
        self.change_log.clear();
    }

    /// Execute state batch
    pub fn execute_batch(&mut self, batch: StateBatch) -> Result<(), RuntimeError> {
        if batch.atomic {
            // Create snapshot for rollback
            let snapshot_id = self.create_snapshot("Batch execution".to_string());
            
            // Execute all changes
            for change in &batch.changes {
                match self.apply_change(change) {
                    Ok(_) => {},
                    Err(e) => {
                        // Rollback on error
                        if let Some(snapshot) = self.snapshots.get(snapshot_id as usize) {
                            self.restore_snapshot(snapshot.clone())?;
                        }
                        return Err(e);
                    }
                }
            }
        } else {
            // Execute changes non-atomically
            for change in &batch.changes {
                // Continue on error for non-atomic batch
                let _ = self.apply_change(change);
            }
        }

        Ok(())
    }

    /// Query state with filters
    pub fn query_state(&self, query: StateQuery) -> Vec<&AccountState> {
        self.accounts
            .values()
            .filter(|account| {
                if let Some(ref addr) = query.account {
                    account.address == *addr
                } else {
                    true
                }
            })
            .collect()
    }

    /// Get state statistics
    pub fn get_statistics(&self) -> StateStatistics {
        let total_accounts = self.accounts.len();
        let total_balance: u64 = self.accounts.values().map(|a| a.balance).sum();
        let contracts = self.accounts.values().filter(|a| a.code.is_some()).count();

        StateStatistics {
            total_accounts,
            total_balance,
            contract_accounts: contracts,
            external_accounts: total_accounts - contracts,
            total_changes: self.change_count,
            snapshots_count: self.snapshots.len(),
        }
    }

    // Private helper methods

    fn record_change(&mut self, change: StateChange) {
        self.change_log.push(change);
        self.change_count += 1;
    }

    fn apply_change(&mut self, change: &StateChange) -> Result<(), RuntimeError> {
        match change.change_type {
            StateChangeType::BalanceChange => {
                let new_balance = u64::from_le_bytes(
                    change.new_value.as_slice().try_into()
                        .map_err(|_| RuntimeError::StateError {
                            message: "Invalid balance format".to_string(),
                        })?
                );
                self.set_balance(&change.account, new_balance)
            },
            StateChangeType::NonceChange => {
                let new_nonce = u64::from_le_bytes(
                    change.new_value.as_slice().try_into()
                        .map_err(|_| RuntimeError::StateError {
                            message: "Invalid nonce format".to_string(),
                        })?
                );
                self.set_nonce(&change.account, new_nonce)
            },
            StateChangeType::CodeChange => {
                self.set_code(&change.account, &change.new_value)
            },
            StateChangeType::AccountCreation => {
                let initial_balance = u64::from_le_bytes(
                    change.new_value.as_slice().try_into()
                        .map_err(|_| RuntimeError::StateError {
                            message: "Invalid balance format".to_string(),
                        })?
                );
                self.create_account(&change.account, initial_balance)
            },
            StateChangeType::AccountDeletion => {
                self.delete_account(&change.account)
            },
            StateChangeType::StorageChange => {
                // Storage changes are handled by storage manager
                Ok(())
            },
        }
    }

    fn calculate_hash(&self, data: &[u8]) -> String {
        use sha3::{Digest, Keccak256};
        let hash = Keccak256::digest(data);
        hex::encode(hash)
    }

    fn current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// State statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateStatistics {
    pub total_accounts: usize,
    pub total_balance: u64,
    pub contract_accounts: usize,
    pub external_accounts: usize,
    pub total_changes: u64,
    pub snapshots_count: usize,
}

impl Default for StateManager {
    fn default() -> Self {
        Self {
            accounts: HashMap::new(),
            snapshots: Vec::new(),
            change_log: Vec::new(),
            change_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeConfig;

    #[test]
    fn test_account_creation() {
        let config = RuntimeConfig::default();
        let mut state_manager = StateManager::new(&config).unwrap();
        
        let address = "0x1234567890123456789012345678901234567890";
        let balance = 1000u64;
        
        assert!(!state_manager.account_exists(address));
        
        let result = state_manager.create_account(address, balance);
        assert!(result.is_ok());
        assert!(state_manager.account_exists(address));
        assert_eq!(state_manager.get_balance(address).unwrap(), balance);
    }

    #[test]
    fn test_balance_operations() {
        let config = RuntimeConfig::default();
        let mut state_manager = StateManager::new(&config).unwrap();
        
        let address = "0x1234567890123456789012345678901234567890";
        
        // Set balance for non-existent account (should create it)
        let result = state_manager.set_balance(address, 500);
        assert!(result.is_ok());
        assert_eq!(state_manager.get_balance(address).unwrap(), 500);
        
        // Update balance
        let result = state_manager.set_balance(address, 1000);
        assert!(result.is_ok());
        assert_eq!(state_manager.get_balance(address).unwrap(), 1000);
    }

    #[test]
    fn test_transfer() {
        let config = RuntimeConfig::default();
        let mut state_manager = StateManager::new(&config).unwrap();
        
        let from = "0x1111111111111111111111111111111111111111";
        let to = "0x2222222222222222222222222222222222222222";
        
        // Set up accounts
        state_manager.create_account(from, 1000).unwrap();
        state_manager.create_account(to, 0).unwrap();
        
        // Transfer
        let result = state_manager.transfer(from, to, 300);
        assert!(result.is_ok());
        
        assert_eq!(state_manager.get_balance(from).unwrap(), 700);
        assert_eq!(state_manager.get_balance(to).unwrap(), 300);
    }

    #[test]
    fn test_insufficient_balance_transfer() {
        let config = RuntimeConfig::default();
        let mut state_manager = StateManager::new(&config).unwrap();
        
        let from = "0x1111111111111111111111111111111111111111";
        let to = "0x2222222222222222222222222222222222222222";
        
        state_manager.create_account(from, 100).unwrap();
        state_manager.create_account(to, 0).unwrap();
        
        // Try to transfer more than balance
        let result = state_manager.transfer(from, to, 200);
        assert!(result.is_err());
        
        // Balances should remain unchanged
        assert_eq!(state_manager.get_balance(from).unwrap(), 100);
        assert_eq!(state_manager.get_balance(to).unwrap(), 0);
    }

    #[test]
    fn test_snapshots() {
        let config = RuntimeConfig::default();
        let mut state_manager = StateManager::new(&config).unwrap();
        
        let address = "0x1234567890123456789012345678901234567890";
        
        // Initial state
        state_manager.create_account(address, 1000).unwrap();
        
        // Create snapshot
        let snapshot_id = state_manager.create_snapshot("Initial state".to_string());
        
        // Modify state
        state_manager.set_balance(address, 2000).unwrap();
        assert_eq!(state_manager.get_balance(address).unwrap(), 2000);
        
        // Restore snapshot
        if let Some(snapshot) = state_manager.snapshots.get(snapshot_id as usize) {
            let result = state_manager.restore_snapshot(snapshot.clone());
            assert!(result.is_ok());
        }
        
        // State should be restored
        assert_eq!(state_manager.get_balance(address).unwrap(), 1000);
    }

    #[test]
    fn test_code_operations() {
        let config = RuntimeConfig::default();
        let mut state_manager = StateManager::new(&config).unwrap();
        
        let address = "0x1234567890123456789012345678901234567890";
        let code = vec![0x60, 0x01, 0x60, 0x02, 0x01]; // Simple bytecode
        
        // Set code
        let result = state_manager.set_code(address, &code);
        assert!(result.is_ok());
        
        // Get code
        let retrieved_code = state_manager.get_code(address);
        assert_eq!(retrieved_code, Some(code.as_slice()));
        
        // Check account was created
        assert!(state_manager.account_exists(address));
        let account = state_manager.get_account(address).unwrap();
        assert!(account.code_hash.is_some());
    }

    #[test]
    fn test_state_statistics() {
        let config = RuntimeConfig::default();
        let mut state_manager = StateManager::new(&config).unwrap();
        
        // Create some accounts
        state_manager.create_account("0x1111111111111111111111111111111111111111", 1000).unwrap();
        state_manager.create_account("0x2222222222222222222222222222222222222222", 2000).unwrap();
        
        // Set code for one account (make it a contract)
        state_manager.set_code("0x1111111111111111111111111111111111111111", &[0x60, 0x01]).unwrap();
        
        let stats = state_manager.get_statistics();
        assert_eq!(stats.total_accounts, 2);
        assert_eq!(stats.total_balance, 3000);
        assert_eq!(stats.contract_accounts, 1);
        assert_eq!(stats.external_accounts, 1);
        assert!(stats.total_changes > 0);
    }
}