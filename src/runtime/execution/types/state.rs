//! State management types for execution context.
//!
//! Provides structures for managing iterator state, contract state,
//! storage overlays, and isolated storage keys.

use super::stack::StackItem;

/// Iterator state for SYSCALL streaming operations
#[derive(Debug, Clone)]
pub struct IteratorState {
    pub(crate) entries: Vec<StackItem>,
    pub(crate) index: usize,
}

/// Contract state for deployed contracts
#[derive(Debug, Clone)]
pub struct ContractState {
    pub(crate) id: u32,
    pub(crate) hash: [u8; 20],
    pub(crate) nef: Vec<u8>,
    pub(crate) manifest: Vec<u8>,
    pub(crate) update_counter: u32,
}

/// Storage overlay entry for transactional storage operations
#[derive(Debug, Clone)]
pub struct OverlayEntry {
    pub(crate) value: Option<Vec<u8>>,
    pub(crate) dirty: bool,
}

/// Storage key that includes account for isolation
/// Used for cross-contract storage isolation to prevent storage collisions
#[allow(dead_code)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IsolatedStorageKey {
    /// Account/contract that owns this storage
    account: String,
    /// The actual storage key
    key: Vec<u8>,
}

#[allow(dead_code)]
impl IsolatedStorageKey {
    fn new(account: &str, key: Vec<u8>) -> Self {
        Self {
            account: account.to_string(),
            key,
        }
    }
}

/// Type alias for storage overlay entries
pub type StorageOverlayEntries = Vec<(Vec<u8>, Option<Vec<u8>>)>;
