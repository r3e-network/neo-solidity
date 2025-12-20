/// VM Bridge for EVM-to-NeoVM translation
#[derive(Debug)]
pub struct VMBridge {
    config: RuntimeConfig,
    instruction_mapping: HashMap<u8, InstructionHandler>,
    system_calls: HashMap<String, SystemCall>,
    contract_account: String,
}

/// Instruction handler function type
type InstructionHandler = fn(
    &mut VMBridge,
    &mut execution::ExecutionContext,
    &mut state::StateManager,
    &mut storage::StorageManager,
    &mut execution::GasTracker,
) -> Result<(), VMBridgeError>;

/// System call function type
type SystemCall = fn(&mut VMBridge, &[StackItem]) -> Result<Vec<StackItem>, VMBridgeError>;

/// VM Bridge errors
#[derive(Debug, Error)]
pub enum VMBridgeError {
    #[error("Instruction not supported: {opcode:#04x}")]
    UnsupportedInstruction { opcode: u8 },

    #[error("System call failed: {name} - {message}")]
    SystemCallFailed { name: String, message: String },

    #[error("Stack operation failed: {message}")]
    StackOperationFailed { message: String },

    #[error("Memory operation failed: {message}")]
    MemoryOperationFailed { message: String },

    #[error("Storage operation failed: {message}")]
    StorageOperationFailed { message: String },

    #[error("State operation failed: {message}")]
    StateOperationFailed { message: String },

    #[error("Bridge error: {message}")]
    BridgeError { message: String },

    #[error("Invalid argument count: expected {expected}, got {got}")]
    InvalidArguments { expected: usize, got: usize },
}

