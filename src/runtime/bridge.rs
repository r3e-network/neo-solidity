//! VM Bridge Module
//! 
//! Provides bridge between EVM semantics and NeoVM execution environment.

use super::{RuntimeConfig, RuntimeError, ExecutionResult, RuntimeException, ExceptionType,
           StateChange, LogEntry, StackFrame, execution, state, storage};
use super::types::StackItem;
use std::collections::HashMap;
use thiserror::Error;

/// VM Bridge for EVM-to-NeoVM translation
#[derive(Debug)]
pub struct VMBridge {
    config: RuntimeConfig,
    instruction_mapping: HashMap<u8, InstructionHandler>,
    system_calls: HashMap<String, SystemCall>,
}

/// Instruction handler function type
type InstructionHandler = fn(&mut VMBridge, &mut execution::ExecutionContext, 
                           &mut state::StateManager, &mut storage::StorageManager,
                           &mut execution::GasTracker) -> Result<(), VMBridgeError>;

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
}

impl VMBridge {
    /// Create new VM bridge
    pub fn new(config: &RuntimeConfig) -> Result<Self, RuntimeError> {
        let mut bridge = Self {
            config: config.clone(),
            instruction_mapping: HashMap::new(),
            system_calls: HashMap::new(),
        };
        
        bridge.initialize_instruction_mapping();
        bridge.initialize_system_calls();
        Ok(bridge)
    }

    /// Execute bytecode through the bridge
    pub fn execute(
        &mut self,
        context: &mut execution::ExecutionContext,
        state: &mut state::StateManager,
        storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<ExecutionResult, RuntimeError> {
        let mut state_changes = Vec::new();
        let mut logs = Vec::new();
        let mut stack_trace = Vec::new();

        loop {
            // Check gas limit
            if gas.out_of_gas() {
                return Ok(ExecutionResult {
                    success: false,
                    return_data: Vec::new(),
                    gas_used: gas.used(),
                    gas_limit: gas.limit(),
                    exception: Some(RuntimeException {
                        exception_type: ExceptionType::OutOfGas,
                        message: "Execution ran out of gas".to_string(),
                        instruction_pointer: Some(context.instruction_count() as u32),
                        stack_trace: stack_trace.clone(),
                    }),
                    state_changes,
                    logs,
                    stack_trace: Some(stack_trace),
                });
            }

            // Execute single step
            match context.step() {
                Ok(step_result) => {
                    if step_result.halted {
                        // Commit pending storage changes
                        for account in self.get_modified_accounts(storage) {
                            let changes = storage.commit(&account)?;
                            for change in changes {
                                state_changes.push(StateChange {
                                    change_type: super::StateChangeType::StorageChange,
                                    account: account.clone(),
                                    key: Some(change.key),
                                    old_value: change.old_value,
                                    new_value: change.new_value.unwrap_or_default(),
                                });
                            }
                        }

                        return Ok(ExecutionResult {
                            success: true,
                            return_data: self.extract_return_data(context)?,
                            gas_used: gas.used(),
                            gas_limit: gas.limit(),
                            exception: None,
                            state_changes,
                            logs,
                            stack_trace: None,
                        });
                    }

                    // Add to stack trace if debugging enabled
                    if self.config.enable_debugging {
                        stack_trace.push(StackFrame {
                            function_name: None,
                            instruction_pointer: step_result.instruction_pointer,
                            opcode: step_result.opcode,
                            stack_items: step_result.stack_items,
                            local_variables: HashMap::new(),
                        });
                    }
                },
                Err(e) => {
                    // Rollback pending changes on error
                    for account in self.get_modified_accounts(storage) {
                        storage.rollback(&account)?;
                    }

                    return Ok(ExecutionResult {
                        success: false,
                        return_data: Vec::new(),
                        gas_used: gas.used(),
                        gas_limit: gas.limit(),
                        exception: Some(RuntimeException {
                            exception_type: ExceptionType::Fault,
                            message: e.to_string(),
                            instruction_pointer: Some(context.instruction_count() as u32),
                            stack_trace: stack_trace.clone(),
                        }),
                        state_changes,
                        logs,
                        stack_trace: Some(stack_trace),
                    });
                }
            }
        }
    }

    /// Handle EVM instruction in NeoVM context
    pub fn handle_instruction(
        &mut self,
        opcode: u8,
        context: &mut execution::ExecutionContext,
        state: &mut state::StateManager,
        storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        if let Some(handler) = self.instruction_mapping.get(&opcode) {
            handler(self, context, state, storage, gas)
        } else {
            Err(VMBridgeError::UnsupportedInstruction { opcode })
        }
    }

    /// Call system function
    pub fn call_system_function(&mut self, name: &str, args: &[StackItem]) -> Result<Vec<StackItem>, VMBridgeError> {
        if let Some(syscall) = self.system_calls.get(name) {
            syscall(self, args)
        } else {
            Err(VMBridgeError::SystemCallFailed {
                name: name.to_string(),
                message: "System call not found".to_string(),
            })
        }
    }

    // Private helper methods

    fn initialize_instruction_mapping(&mut self) {
        // Arithmetic instructions
        self.instruction_mapping.insert(0x95, Self::handle_add);
        self.instruction_mapping.insert(0x96, Self::handle_sub);
        self.instruction_mapping.insert(0x97, Self::handle_mul);
        self.instruction_mapping.insert(0x98, Self::handle_div);
        self.instruction_mapping.insert(0x99, Self::handle_mod);

        // Comparison instructions
        self.instruction_mapping.insert(0xA5, Self::handle_lt);
        self.instruction_mapping.insert(0xA7, Self::handle_gt);
        self.instruction_mapping.insert(0xA3, Self::handle_eq);

        // Stack instructions
        self.instruction_mapping.insert(0x10, Self::handle_push0);
        self.instruction_mapping.insert(0x11, Self::handle_push1);
        self.instruction_mapping.insert(0x45, Self::handle_drop);
        self.instruction_mapping.insert(0x4A, Self::handle_dup);
        self.instruction_mapping.insert(0x50, Self::handle_swap);

        // Control flow
        self.instruction_mapping.insert(0x40, Self::handle_ret);
        self.instruction_mapping.insert(0x22, Self::handle_jmp);
        self.instruction_mapping.insert(0x23, Self::handle_jmpif);
        self.instruction_mapping.insert(0x24, Self::handle_jmpifnot);

        // Memory operations (EVM compatibility)
        self.instruction_mapping.insert(0x51, Self::handle_mload);
        self.instruction_mapping.insert(0x52, Self::handle_mstore);
        self.instruction_mapping.insert(0x53, Self::handle_mstore8);

        // Storage operations (EVM compatibility)
        self.instruction_mapping.insert(0x54, Self::handle_sload);
        self.instruction_mapping.insert(0x55, Self::handle_sstore);
    }

    fn initialize_system_calls(&mut self) {
        self.system_calls.insert("keccak256".to_string(), Self::syscall_keccak256);
        self.system_calls.insert("sha256".to_string(), Self::syscall_sha256);
        self.system_calls.insert("ecrecover".to_string(), Self::syscall_ecrecover);
        self.system_calls.insert("verify".to_string(), Self::syscall_verify);
    }

    // Instruction handlers

    fn handle_add(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("ADD", None).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let b = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;
        let a = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        let result = Self::add_stack_items(a, b)?;
        context.push_stack(result).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_sub(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("SUB", None).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let b = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;
        let a = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        let result = Self::sub_stack_items(a, b)?;
        context.push_stack(result).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_mul(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("MUL", None).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let b = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;
        let a = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        let result = Self::mul_stack_items(a, b)?;
        context.push_stack(result).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_div(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("DIV", None).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let b = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;
        let a = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        let result = Self::div_stack_items(a, b)?;
        context.push_stack(result).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_mod(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("MOD", None).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let b = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;
        let a = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        let result = Self::mod_stack_items(a, b)?;
        context.push_stack(result).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_lt(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("LT", None).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let b = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;
        let a = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        let result = Self::lt_stack_items(a, b)?;
        context.push_stack(result).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_gt(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("GT", None).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let b = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;
        let a = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        let result = Self::gt_stack_items(a, b)?;
        context.push_stack(result).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_eq(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("EQ", None).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let b = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;
        let a = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        let result = Self::eq_stack_items(a, b)?;
        context.push_stack(result).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_push0(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("PUSH", Some(1)).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        context.push_stack(StackItem::UnsignedInteger(0)).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_push1(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("PUSH", Some(1)).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        context.push_stack(StackItem::UnsignedInteger(1)).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_drop(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("DROP", Some(2)).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_dup(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("DUP", Some(2)).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let item = context.peek_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?.clone();

        context.push_stack(item).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_swap(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("SWAP", Some(2)).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let a = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;
        let b = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        context.push_stack(a).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;
        context.push_stack(b).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_ret(
        _bridge: &mut VMBridge,
        _context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        _gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        // RET instruction - handled by execution context
        Ok(())
    }

    fn handle_jmp(
        _bridge: &mut VMBridge,
        _context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        _gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        // JMP instruction - would need address from bytecode
        Ok(())
    }

    fn handle_jmpif(
        _bridge: &mut VMBridge,
        _context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        _gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        // JMPIF instruction - conditional jump
        Ok(())
    }

    fn handle_jmpifnot(
        _bridge: &mut VMBridge,
        _context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        _gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        // JMPIFNOT instruction - conditional jump
        Ok(())
    }

    fn handle_mload(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("MLOAD", Some(3)).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let address = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        // Convert address to usize
        let addr = match address {
            StackItem::UnsignedInteger(a) => a as usize,
            StackItem::Integer(a) => a as usize,
            _ => return Err(VMBridgeError::MemoryOperationFailed {
                message: "Invalid memory address".to_string(),
            }),
        };

        // Read 32 bytes from memory
        let data = context.read_memory(addr, 32).map_err(|e| VMBridgeError::MemoryOperationFailed {
            message: e.to_string(),
        })?;

        context.push_stack(StackItem::ByteArray(data.to_vec())).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_mstore(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("MSTORE", Some(3)).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let value = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;
        let address = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        let addr = match address {
            StackItem::UnsignedInteger(a) => a as usize,
            StackItem::Integer(a) => a as usize,
            _ => return Err(VMBridgeError::MemoryOperationFailed {
                message: "Invalid memory address".to_string(),
            }),
        };

        let data = value.to_bytes();
        let mut padded_data = data;
        padded_data.resize(32, 0); // Pad to 32 bytes

        context.write_memory(addr, &padded_data).map_err(|e| VMBridgeError::MemoryOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_mstore8(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("MSTORE8", Some(3)).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let value = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;
        let address = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        let addr = match address {
            StackItem::UnsignedInteger(a) => a as usize,
            StackItem::Integer(a) => a as usize,
            _ => return Err(VMBridgeError::MemoryOperationFailed {
                message: "Invalid memory address".to_string(),
            }),
        };

        let byte_value = match value {
            StackItem::UnsignedInteger(v) => (v & 0xFF) as u8,
            StackItem::Integer(v) => (v & 0xFF) as u8,
            StackItem::ByteArray(bytes) => bytes.get(0).copied().unwrap_or(0),
            _ => 0,
        };

        context.write_memory(addr, &[byte_value]).map_err(|e| VMBridgeError::MemoryOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_sload(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("SLOAD", None).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let key = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        // For now, use a default account address
        // In a real implementation, this would come from the execution context
        let account = "0x0000000000000000000000000000000000000000";
        let key_bytes = key.to_bytes();

        let value = storage.get(account, &key_bytes).map_err(|e| VMBridgeError::StorageOperationFailed {
            message: e.to_string(),
        })?;

        let result = match value {
            Some(data) => StackItem::ByteArray(data),
            None => StackItem::UnsignedInteger(0),
        };

        context.push_stack(result).map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn handle_sstore(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("SSTORE", None).map_err(|e| VMBridgeError::BridgeError {
            message: e.to_string(),
        })?;

        let value = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;
        let key = context.pop_stack().map_err(|e| VMBridgeError::StackOperationFailed {
            message: e.to_string(),
        })?;

        let account = "0x0000000000000000000000000000000000000000";
        let key_bytes = key.to_bytes();
        let value_bytes = value.to_bytes();

        storage.set(account, &key_bytes, &value_bytes).map_err(|e| VMBridgeError::StorageOperationFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    // System call handlers

    fn syscall_keccak256(_bridge: &mut VMBridge, args: &[StackItem]) -> Result<Vec<StackItem>, VMBridgeError> {
        if args.len() != 1 {
            return Err(VMBridgeError::SystemCallFailed {
                name: "keccak256".to_string(),
                message: "Expected 1 argument".to_string(),
            });
        }

        use sha3::{Digest, Keccak256};
        
        let input = args[0].to_bytes();
        let hash = Keccak256::digest(&input);
        
        Ok(vec![StackItem::ByteArray(hash.to_vec())])
    }

    fn syscall_sha256(_bridge: &mut VMBridge, args: &[StackItem]) -> Result<Vec<StackItem>, VMBridgeError> {
        if args.len() != 1 {
            return Err(VMBridgeError::SystemCallFailed {
                name: "sha256".to_string(),
                message: "Expected 1 argument".to_string(),
            });
        }

        use sha2::{Digest, Sha256};
        
        let input = args[0].to_bytes();
        let hash = Sha256::digest(&input);
        
        Ok(vec![StackItem::ByteArray(hash.to_vec())])
    }

    fn syscall_ecrecover(_bridge: &mut VMBridge, _args: &[StackItem]) -> Result<Vec<StackItem>, VMBridgeError> {
        // Placeholder implementation
        Err(VMBridgeError::SystemCallFailed {
            name: "ecrecover".to_string(),
            message: "Not implemented".to_string(),
        })
    }

    fn syscall_verify(_bridge: &mut VMBridge, _args: &[StackItem]) -> Result<Vec<StackItem>, VMBridgeError> {
        // Placeholder implementation
        Err(VMBridgeError::SystemCallFailed {
            name: "verify".to_string(),
            message: "Not implemented".to_string(),
        })
    }

    // Arithmetic operations on stack items

    fn add_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_add(y)))
            },
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_add(y)))
            },
            (StackItem::Integer(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::Integer(x.wrapping_add(y as i64)))
            },
            (StackItem::UnsignedInteger(x), StackItem::Integer(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_add(y as u64)))
            },
            _ => Err(VMBridgeError::StackOperationFailed {
                message: "Invalid operands for ADD".to_string(),
            }),
        }
    }

    fn sub_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_sub(y)))
            },
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_sub(y)))
            },
            _ => Err(VMBridgeError::StackOperationFailed {
                message: "Invalid operands for SUB".to_string(),
            }),
        }
    }

    fn mul_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_mul(y)))
            },
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_mul(y)))
            },
            _ => Err(VMBridgeError::StackOperationFailed {
                message: "Invalid operands for MUL".to_string(),
            }),
        }
    }

    fn div_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                if y == 0 {
                    Ok(StackItem::Integer(0)) // Division by zero returns 0 in EVM
                } else {
                    Ok(StackItem::Integer(x / y))
                }
            },
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                if y == 0 {
                    Ok(StackItem::UnsignedInteger(0))
                } else {
                    Ok(StackItem::UnsignedInteger(x / y))
                }
            },
            _ => Err(VMBridgeError::StackOperationFailed {
                message: "Invalid operands for DIV".to_string(),
            }),
        }
    }

    fn mod_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                if y == 0 {
                    Ok(StackItem::Integer(0))
                } else {
                    Ok(StackItem::Integer(x % y))
                }
            },
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                if y == 0 {
                    Ok(StackItem::UnsignedInteger(0))
                } else {
                    Ok(StackItem::UnsignedInteger(x % y))
                }
            },
            _ => Err(VMBridgeError::StackOperationFailed {
                message: "Invalid operands for MOD".to_string(),
            }),
        }
    }

    fn lt_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        let result = match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => x < y,
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => x < y,
            _ => false,
        };
        Ok(StackItem::Boolean(result))
    }

    fn gt_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        let result = match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => x > y,
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => x > y,
            _ => false,
        };
        Ok(StackItem::Boolean(result))
    }

    fn eq_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        let result = match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => x == y,
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => x == y,
            (StackItem::Boolean(x), StackItem::Boolean(y)) => x == y,
            (StackItem::ByteArray(x), StackItem::ByteArray(y)) => x == y,
            (StackItem::Null, StackItem::Null) => true,
            _ => false,
        };
        Ok(StackItem::Boolean(result))
    }

    fn get_modified_accounts(&self, storage: &storage::StorageManager) -> Vec<String> {
        // Get all accounts with pending changes
        // This is a simplified implementation
        vec!["0x0000000000000000000000000000000000000000".to_string()]
    }

    fn extract_return_data(&self, _context: &execution::ExecutionContext) -> Result<Vec<u8>, RuntimeError> {
        // Extract return data from execution context
        // For now, return empty data
        Ok(Vec::new())
    }
}