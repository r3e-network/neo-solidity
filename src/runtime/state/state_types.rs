/// State manager for account and contract states
#[derive(Debug, Clone, Default)]
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

