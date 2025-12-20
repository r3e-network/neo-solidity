type StorageEntries = Vec<(Vec<u8>, Vec<u8>)>;

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
    Create,   // New key-value pair
    Update,   // Update existing value
    Delete,   // Delete key-value pair
    NoChange, // Value unchanged
}

/// Storage gas costs (Neo N3 compatible)
#[derive(Debug, Clone)]
pub struct StorageGasCosts {
    pub sstore_set: u64,        // Store new value
    pub sstore_reset: u64,      // Update existing value
    pub sstore_clear: u64,      // Clear existing value
    pub sload: u64,             // Load value
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

