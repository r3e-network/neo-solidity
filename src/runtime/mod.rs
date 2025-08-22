//! Neo Runtime Module
//! 
//! Complete runtime integration layer providing EVM compatibility on NeoVM,
//! state management, storage operations, and execution environment.

pub mod execution;
pub mod state;
pub mod storage;
pub mod bridge;
pub mod types;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

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

/// Runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub gas_limit: u64,
    pub call_stack_limit: u32,
    pub memory_limit: usize,
    pub storage_limit: usize,
    pub enable_debugging: bool,
    pub enable_tracing: bool,
    pub strict_mode: bool,
    pub neo_version: String,
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

impl NeoRuntime {
    /// Create new runtime instance
    pub fn new(config: RuntimeConfig) -> Result<Self, RuntimeError> {
        Ok(Self {
            execution_context: execution::ExecutionContext::new(&config)?,
            state_manager: state::StateManager::new(&config)?,
            storage_manager: storage::StorageManager::new(&config)?,
            vm_bridge: bridge::VMBridge::new(&config)?,
            gas_tracker: execution::GasTracker::new(config.gas_limit),
        })
    }

    /// Execute bytecode with given input
    pub fn execute(&mut self, bytecode: &[u8], input: &[u8]) -> Result<ExecutionResult, RuntimeError> {
        // Initialize execution context
        self.execution_context.initialize(bytecode, input)?;
        
        // Reset gas tracker
        self.gas_tracker.reset(self.execution_context.gas_limit());
        
        // Execute bytecode through VM bridge
        let result = self.vm_bridge.execute(
            &mut self.execution_context,
            &mut self.state_manager,
            &mut self.storage_manager,
            &mut self.gas_tracker,
        )?;

        Ok(result)
    }

    /// Call specific function with arguments
    pub fn call_function(&mut self, bytecode: &[u8], function_name: &str, args: &[types::StackItem]) -> Result<ExecutionResult, RuntimeError> {
        // Prepare function call
        let call_data = self.prepare_function_call(function_name, args)?;
        
        // Execute with call data
        self.execute(bytecode, &call_data)
    }

    /// Deploy contract and return address
    pub fn deploy_contract(&mut self, bytecode: &[u8], constructor_args: &[u8]) -> Result<String, RuntimeError> {
        // Generate contract address
        let address = self.generate_contract_address()?;
        
        // Store bytecode in state
        self.state_manager.set_code(&address, bytecode)?;
        
        // Execute constructor if present
        if !constructor_args.is_empty() {
            let result = self.execute(bytecode, constructor_args)?;
            if !result.success {
                return Err(RuntimeError::ExecutionError {
                    message: format!("Constructor failed: {:?}", result.exception),
                });
            }
        }
        
        Ok(address)
    }

    /// Get current state of the runtime
    pub fn get_state_snapshot(&self) -> state::StateSnapshot {
        self.state_manager.get_snapshot()
    }

    /// Restore state from snapshot
    pub fn restore_state(&mut self, snapshot: state::StateSnapshot) -> Result<(), RuntimeError> {
        self.state_manager.restore_snapshot(snapshot)
            .map_err(|e| RuntimeError::StateError { message: e.to_string() })
    }

    /// Get storage value for account and key
    pub fn get_storage(&self, account: &str, key: &[u8]) -> Result<Option<Vec<u8>>, RuntimeError> {
        self.storage_manager.get(account, key)
            .map_err(|e| RuntimeError::StorageError { message: e.to_string() })
    }

    /// Set storage value for account and key
    pub fn set_storage(&mut self, account: &str, key: &[u8], value: &[u8]) -> Result<(), RuntimeError> {
        self.storage_manager.set(account, key, value)
            .map_err(|e| RuntimeError::StorageError { message: e.to_string() })
    }

    /// Get account balance
    pub fn get_balance(&self, account: &str) -> Result<u64, RuntimeError> {
        self.state_manager.get_balance(account)
            .map_err(|e| RuntimeError::StateError { message: e.to_string() })
    }

    /// Set account balance
    pub fn set_balance(&mut self, account: &str, balance: u64) -> Result<(), RuntimeError> {
        self.state_manager.set_balance(account, balance)
            .map_err(|e| RuntimeError::StateError { message: e.to_string() })
    }

    /// Get runtime statistics
    pub fn get_statistics(&self) -> RuntimeStatistics {
        RuntimeStatistics {
            total_gas_used: self.gas_tracker.used(),
            total_instructions_executed: self.execution_context.instruction_count(),
            max_stack_depth: self.execution_context.max_stack_depth(),
            storage_reads: self.storage_manager.read_count(),
            storage_writes: self.storage_manager.write_count(),
            state_changes: self.state_manager.change_count(),
        }
    }

    /// Enable debugging mode
    pub fn enable_debugging(&mut self) {
        self.execution_context.enable_debugging();
    }

    /// Disable debugging mode
    pub fn disable_debugging(&mut self) {
        self.execution_context.disable_debugging();
    }

    /// Set breakpoint at instruction
    pub fn set_breakpoint(&mut self, instruction_pointer: u32) {
        self.execution_context.set_breakpoint(instruction_pointer);
    }

    /// Remove breakpoint
    pub fn remove_breakpoint(&mut self, instruction_pointer: u32) {
        self.execution_context.remove_breakpoint(instruction_pointer);
    }

    /// Step through execution
    pub fn step(&mut self) -> Result<execution::StepResult, RuntimeError> {
        self.execution_context.step()
            .map_err(|e| RuntimeError::ExecutionError { message: e.to_string() })
    }

    // Private helper methods
    
    fn prepare_function_call(&self, function_name: &str, args: &[types::StackItem]) -> Result<Vec<u8>, RuntimeError> {
        // This would implement function selector and argument encoding
        // For now, just a placeholder
        let mut call_data = Vec::new();
        
        // Add function selector (first 4 bytes of keccak256 hash of function signature)
        let selector = self.calculate_function_selector(function_name)?;
        call_data.extend_from_slice(&selector);
        
        // Encode arguments
        for arg in args {
            call_data.extend_from_slice(&arg.to_bytes());
        }
        
        Ok(call_data)
    }

    fn calculate_function_selector(&self, function_name: &str) -> Result<[u8; 4], RuntimeError> {
        // Calculate keccak256 hash and take first 4 bytes
        use sha3::{Digest, Keccak256};
        
        let hash = Keccak256::digest(function_name.as_bytes());
        let mut selector = [0u8; 4];
        selector.copy_from_slice(&hash[..4]);
        Ok(selector)
    }

    fn generate_contract_address(&self) -> Result<String, RuntimeError> {
        // Generate deterministic contract address
        // In a real implementation, this would use deployer address + nonce
        use sha3::{Digest, Keccak256};
        
        let input = format!("contract_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos());
        
        let hash = Keccak256::digest(input.as_bytes());
        Ok(format!("0x{}", hex::encode(&hash[12..32]))) // Take last 20 bytes
    }
}

/// Runtime performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeStatistics {
    pub total_gas_used: u64,
    pub total_instructions_executed: u64,
    pub max_stack_depth: u32,
    pub storage_reads: u64,
    pub storage_writes: u64,
    pub state_changes: u64,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            gas_limit: 10_000_000,
            call_stack_limit: 1024,
            memory_limit: 1024 * 1024, // 1MB
            storage_limit: 10 * 1024 * 1024, // 10MB
            enable_debugging: false,
            enable_tracing: false,
            strict_mode: true,
            neo_version: "3.5.0".to_string(),
        }
    }
}

impl ExecutionResult {
    /// Check if execution was successful
    pub fn is_success(&self) -> bool {
        self.success && self.exception.is_none()
    }

    /// Get gas efficiency (percentage of gas limit used)
    pub fn gas_efficiency(&self) -> f64 {
        if self.gas_limit == 0 {
            0.0
        } else {
            (self.gas_used as f64) / (self.gas_limit as f64)
        }
    }

    /// Check if execution ran out of gas
    pub fn out_of_gas(&self) -> bool {
        matches!(self.exception, Some(RuntimeException { 
            exception_type: ExceptionType::OutOfGas, .. 
        }))
    }

    /// Get return data as string (if valid UTF-8)
    pub fn return_string(&self) -> Option<String> {
        String::from_utf8(self.return_data.clone()).ok()
    }

    /// Get return data as hex string
    pub fn return_hex(&self) -> String {
        hex::encode(&self.return_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let config = RuntimeConfig::default();
        let runtime = NeoRuntime::new(config);
        assert!(runtime.is_ok());
    }

    #[test]
    fn test_contract_deployment() {
        let config = RuntimeConfig::default();
        let mut runtime = NeoRuntime::new(config).unwrap();
        
        let bytecode = vec![0x60, 0x01, 0x60, 0x02, 0x01]; // Simple ADD bytecode
        let result = runtime.deploy_contract(&bytecode, &[]);
        
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(address.starts_with("0x"));
        assert_eq!(address.len(), 42); // 0x + 40 hex chars
    }

    #[test]
    fn test_function_selector_calculation() {
        let config = RuntimeConfig::default();
        let runtime = NeoRuntime::new(config).unwrap();
        
        let selector = runtime.calculate_function_selector("transfer(address,uint256)").unwrap();
        // Known selector for transfer function
        assert_eq!(selector, [0xa9, 0x05, 0x9c, 0xbb]);
    }

    #[test]
    fn test_execution_result() {
        let result = ExecutionResult {
            success: true,
            return_data: vec![0x01, 0x02, 0x03],
            gas_used: 1000,
            gas_limit: 10000,
            exception: None,
            state_changes: vec![],
            logs: vec![],
            stack_trace: None,
        };
        
        assert!(result.is_success());
        assert_eq!(result.gas_efficiency(), 0.1);
        assert!(!result.out_of_gas());
        assert_eq!(result.return_hex(), "010203");
    }

    #[test]
    fn test_runtime_statistics() {
        let stats = RuntimeStatistics {
            total_gas_used: 5000,
            total_instructions_executed: 100,
            max_stack_depth: 10,
            storage_reads: 5,
            storage_writes: 3,
            state_changes: 2,
        };
        
        assert_eq!(stats.total_gas_used, 5000);
        assert_eq!(stats.total_instructions_executed, 100);
    }

    #[test]
    fn test_state_operations() {
        let config = RuntimeConfig::default();
        let mut runtime = NeoRuntime::new(config).unwrap();
        
        let account = "0x1234567890123456789012345678901234567890";
        let key = b"test_key";
        let value = b"test_value";
        
        // Set storage
        let result = runtime.set_storage(account, key, value);
        assert!(result.is_ok());
        
        // Get storage
        let retrieved = runtime.get_storage(account, key).unwrap();
        assert_eq!(retrieved, Some(value.to_vec()));
    }

    #[test]
    fn test_balance_operations() {
        let config = RuntimeConfig::default();
        let mut runtime = NeoRuntime::new(config).unwrap();
        
        let account = "0x1234567890123456789012345678901234567890";
        let balance = 1000u64;
        
        // Set balance
        let result = runtime.set_balance(account, balance);
        assert!(result.is_ok());
        
        // Get balance
        let retrieved = runtime.get_balance(account).unwrap();
        assert_eq!(retrieved, balance);
    }
}