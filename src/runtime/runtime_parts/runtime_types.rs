/// Neo runtime for executing compiled Yul contracts
#[derive(Debug)]
pub struct NeoRuntime {
    execution_context: execution::ExecutionContext,
    state_manager: state::StateManager,
    storage_manager: storage::StorageManager,
    vm_bridge: bridge::VMBridge,
    gas_tracker: execution::GasTracker,
}

/// Runtime execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub return_data: Vec<u8>,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub exception: Option<RuntimeException>,
    pub state_changes: Vec<StateChange>,
    pub logs: Vec<LogEntry>,
    pub stack_trace: Option<Vec<StackFrame>>,
    pub metadata: ExecutionMetadata,
}

/// Runtime exception information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeException {
    pub exception_type: ExceptionType,
    pub message: String,
    pub instruction_pointer: Option<u32>,
    pub stack_trace: Vec<StackFrame>,
}

/// Types of runtime exceptions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExceptionType {
    OutOfGas,
    StackOverflow,
    StackUnderflow,
    InvalidOpcode,
    InvalidJump,
    RevertExecution,
    Fault,
    Halt,
}

/// State change record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    pub change_type: StateChangeType,
    pub account: String,
    pub key: Option<Vec<u8>>,
    pub old_value: Option<Vec<u8>>,
    pub new_value: Vec<u8>,
}

/// Types of state changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateChangeType {
    BalanceChange,
    StorageChange,
    CodeChange,
    NonceChange,
    AccountCreation,
    AccountDeletion,
}

/// Log entry for events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub address: String,
    pub topics: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

/// Stack frame for debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub function_name: Option<String>,
    pub instruction_pointer: u32,
    pub opcode: String,
    pub stack_items: Vec<types::StackItem>,
    pub local_variables: HashMap<String, types::StackItem>,
}

/// Optional metadata overrides for a single execution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionOverrides {
    pub block_height: Option<u64>,
    pub timestamp: Option<u64>,
    pub caller_account: Option<String>,
}

/// Metadata captured from the execution environment
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionMetadata {
    pub block_height: Option<u64>,
    pub timestamp: Option<u64>,
    pub caller_account: Option<String>,
}

/// Runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub gas_limit: u64,
    pub call_stack_limit: u32,
    pub memory_limit: usize,
    pub storage_limit: usize,
    pub network_magic: u32,
    pub enable_debugging: bool,
    pub enable_tracing: bool,
    pub strict_mode: bool,
    pub neo_version: String,
    pub contract_account: String,
    pub default_block_height: u64,
    pub default_timestamp: u64,
}

/// Runtime errors
#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Execution failed: {message}")]
    ExecutionError { message: String },

    #[error("Out of gas: used {used}, limit {limit}")]
    OutOfGas { used: u64, limit: u64 },

    #[error("Stack overflow at depth {depth}")]
    StackOverflow { depth: u32 },

    #[error("Invalid operation: {operation}")]
    InvalidOperation { operation: String },

    #[error("State error: {message}")]
    StateError { message: String },

    #[error("Storage error: {message}")]
    StorageError { message: String },

    #[error("Bridge error: {message}")]
    BridgeError { message: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
}

/// Runtime performance statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuntimeStatistics {
    pub total_gas_used: u64,
    pub total_instructions_executed: u64,
    pub max_stack_depth: u32,
    pub storage_reads: u64,
    pub storage_writes: u64,
    pub state_changes: u64,
}

impl RuntimeStatistics {
    /// Get total storage operations
    pub fn total_storage_ops(&self) -> u64 {
        self.storage_reads + self.storage_writes
    }

    /// Get average gas per instruction
    pub fn avg_gas_per_instruction(&self) -> f64 {
        if self.total_instructions_executed == 0 {
            return 0.0;
        }
        self.total_gas_used as f64 / self.total_instructions_executed as f64
    }

    /// Merge with another statistics instance
    pub fn merge(&mut self, other: &RuntimeStatistics) {
        self.total_gas_used += other.total_gas_used;
        self.total_instructions_executed += other.total_instructions_executed;
        self.max_stack_depth = self.max_stack_depth.max(other.max_stack_depth);
        self.storage_reads += other.storage_reads;
        self.storage_writes += other.storage_writes;
        self.state_changes += other.state_changes;
    }
}

