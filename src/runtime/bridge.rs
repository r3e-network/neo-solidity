//! VM Bridge Module
//!
//! Provides bridge between EVM semantics and NeoVM execution environment.

use super::types::StackItem;
use super::{
    execution, state, storage, ExceptionType, ExecutionMetadata, ExecutionResult, RuntimeConfig,
    RuntimeError, RuntimeException, StackFrame, StateChange,
};
use std::collections::HashMap;
use thiserror::Error;

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

impl VMBridge {
    /// Create new VM bridge
    pub fn new(config: &RuntimeConfig) -> Result<Self, RuntimeError> {
        let mut bridge = Self {
            config: config.clone(),
            instruction_mapping: HashMap::new(),
            system_calls: HashMap::new(),
            contract_account: config.contract_account.clone(),
        };

        bridge.initialize_instruction_mapping();
        bridge.initialize_system_calls();
        Ok(bridge)
    }

    /// Execute bytecode through the bridge
    pub fn execute(
        &mut self,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<ExecutionResult, RuntimeError> {
        context.bind_storage(&self.contract_account, storage)?;

        let mut state_changes = Vec::new();
        let mut stack_trace = Vec::new();

        loop {
            gas.sync_from_execution(context.gas_used());

            // Check gas limit using execution context accounting
            if gas.out_of_gas() {
                let result = ExecutionResult {
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
                    logs: context.logs().to_vec(),
                    stack_trace: Some(stack_trace),
                    metadata: ExecutionMetadata::default(),
                };
                context.unbind_storage();
                return Ok(result);
            }

            // Execute single step
            match context.step() {
                Ok(step_result) => {
                    gas.sync_from_execution(context.gas_used());

                    if step_result.halted {
                        let modified_accounts = self.apply_storage_overlay(context, storage)?;
                        for account in modified_accounts {
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

                        gas.sync_from_execution(context.gas_used());

                        let result = ExecutionResult {
                            success: true,
                            return_data: self.extract_return_data(context)?,
                            gas_used: gas.used(),
                            gas_limit: gas.limit(),
                            exception: None,
                            state_changes,
                            logs: context.logs().to_vec(),
                            stack_trace: None,
                            metadata: ExecutionMetadata::default(),
                        };
                        context.unbind_storage();
                        return Ok(result);
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
                }
                Err(RuntimeError::OutOfGas { .. }) => {
                    gas.sync_from_execution(context.gas_used());

                    let result = ExecutionResult {
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
                        logs: context.logs().to_vec(),
                        stack_trace: Some(stack_trace),
                        metadata: ExecutionMetadata::default(),
                    };
                    context.unbind_storage();
                    return Ok(result);
                }
                Err(e) => {
                    gas.sync_from_execution(context.gas_used());

                    let result = ExecutionResult {
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
                        logs: context.logs().to_vec(),
                        stack_trace: Some(stack_trace),
                        metadata: ExecutionMetadata::default(),
                    };
                    context.unbind_storage();
                    return Ok(result);
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
    pub fn call_system_function(
        &mut self,
        name: &str,
        args: &[StackItem],
    ) -> Result<Vec<StackItem>, VMBridgeError> {
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
        // Arithmetic instructions (aligned to spec)
        self.instruction_mapping.insert(0x9E, Self::handle_add);
        self.instruction_mapping.insert(0x9F, Self::handle_sub);
        self.instruction_mapping.insert(0xA0, Self::handle_mul);
        self.instruction_mapping.insert(0xA1, Self::handle_div);
        self.instruction_mapping.insert(0xA2, Self::handle_mod);
        self.instruction_mapping.insert(0xA5, Self::handle_modmul);
        self.instruction_mapping.insert(0xA6, Self::handle_modpow);
        self.instruction_mapping.insert(0xA8, Self::handle_shl);
        self.instruction_mapping.insert(0xA9, Self::handle_shr);
        self.instruction_mapping.insert(0x91, Self::handle_and);
        self.instruction_mapping.insert(0x92, Self::handle_or);
        self.instruction_mapping.insert(0x93, Self::handle_xor);

        // Comparison instructions
        self.instruction_mapping.insert(0xB5, Self::handle_lt);
        self.instruction_mapping.insert(0xB7, Self::handle_gt);
        // Support both deep equality and numeric equality opcodes
        self.instruction_mapping.insert(0x97, Self::handle_eq);
        self.instruction_mapping.insert(0x98, Self::handle_ne);
        self.instruction_mapping.insert(0xB3, Self::handle_eq);
        self.instruction_mapping.insert(0xB4, Self::handle_ne);

        // Stack instructions
        self.instruction_mapping.insert(0x10, Self::handle_push0);
        self.instruction_mapping.insert(0x11, Self::handle_push1);
        self.instruction_mapping.insert(0x45, Self::handle_drop);
        self.instruction_mapping.insert(0x4A, Self::handle_dup);
        self.instruction_mapping.insert(0x50, Self::handle_swap);

        // Control flow
        self.instruction_mapping.insert(0x40, Self::handle_ret);
        self.instruction_mapping.insert(0x22, Self::handle_jmp);
        self.instruction_mapping.insert(0x24, Self::handle_jmpif);
        self.instruction_mapping.insert(0x26, Self::handle_jmpifnot);
        self.instruction_mapping.insert(0x25, Self::handle_jmpif); // long variant
        self.instruction_mapping.insert(0x27, Self::handle_jmpifnot); // long variant

        // Memory operations (EVM compatibility)
        self.instruction_mapping.insert(0x51, Self::handle_mload);
        self.instruction_mapping.insert(0x52, Self::handle_mstore);
        self.instruction_mapping.insert(0x53, Self::handle_mstore8);

        // Storage operations (EVM compatibility)
        self.instruction_mapping.insert(0x54, Self::handle_sload);
        self.instruction_mapping.insert(0x55, Self::handle_sstore);
    }

    fn initialize_system_calls(&mut self) {
        self.system_calls
            .insert("keccak256".to_string(), Self::syscall_keccak256);
        self.system_calls
            .insert("sha256".to_string(), Self::syscall_sha256);
        self.system_calls
            .insert("ecrecover".to_string(), Self::syscall_ecrecover);
        self.system_calls
            .insert("verify".to_string(), Self::syscall_verify);
    }

    // Instruction handlers

    fn apply_storage_overlay(
        &self,
        context: &mut execution::ExecutionContext,
        storage: &mut storage::StorageManager,
    ) -> Result<Vec<String>, RuntimeError> {
        if let Some((account, entries)) = context.drain_dirty_storage_overlay() {
            for (key, value) in entries {
                match value {
                    Some(bytes) => storage.set(&account, &key, &bytes)?,
                    None => storage.delete(&account, &key)?,
                }
            }
            Ok(vec![account])
        } else {
            Ok(Vec::new())
        }
    }

    fn handle_add(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("ADD", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::add_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("SUB", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::sub_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("MUL", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::mul_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("DIV", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::div_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("MOD", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::mod_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }

    fn handle_modmul(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("MODMUL", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let modulus = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::modmul_stack_items(a, b, modulus)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        Ok(())
    }

    fn handle_modpow(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("MODPOW", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let modulus = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let exponent = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let base = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::modpow_stack_items(base, exponent, modulus)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("LT", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::lt_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("GT", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::gt_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }

    fn handle_and(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("AND", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::and_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        Ok(())
    }

    fn handle_or(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("OR", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::or_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        Ok(())
    }

    fn handle_xor(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("XOR", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::xor_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("EQ", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::eq_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }

    fn handle_ne(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("NE", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::eq_stack_items(a, b)?;
        let inverted = match result {
            StackItem::Boolean(flag) => StackItem::Boolean(!flag),
            other => other,
        };
        context
            .push_stack(inverted)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }

    fn handle_shl(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("SHL", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let shift = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let value = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::shift_left_value(value, shift)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        Ok(())
    }

    fn handle_shr(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("SHR", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let shift = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let value = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::shift_right_value(value, shift)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("PUSH", Some(1))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        context
            .push_stack(StackItem::UnsignedInteger(0))
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("PUSH", Some(1))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        context
            .push_stack(StackItem::UnsignedInteger(1))
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("DROP", Some(2))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("DUP", Some(2))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let item = context
            .peek_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?
            .clone();

        context
            .push_stack(item)
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("SWAP", Some(2))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        context
            .push_stack(a)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        context
            .push_stack(b)
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("MLOAD", Some(3))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let address = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        // Convert address to usize
        let addr = match address {
            StackItem::UnsignedInteger(a) => a as usize,
            StackItem::Integer(a) => a as usize,
            _ => {
                return Err(VMBridgeError::MemoryOperationFailed {
                    message: "Invalid memory address".to_string(),
                })
            }
        };

        // Read 32 bytes from memory
        let data = {
            let slice = context.read_memory(addr, 32).map_err(|e| {
                VMBridgeError::MemoryOperationFailed {
                    message: e.to_string(),
                }
            })?;
            slice.to_vec()
        };
        context
            .push_stack(StackItem::ByteArray(data))
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("MSTORE", Some(3))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let value = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let address = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let addr = match address {
            StackItem::UnsignedInteger(a) => a as usize,
            StackItem::Integer(a) => a as usize,
            _ => {
                return Err(VMBridgeError::MemoryOperationFailed {
                    message: "Invalid memory address".to_string(),
                })
            }
        };

        let data = value.to_bytes();
        let mut padded_data = data;
        padded_data.resize(32, 0); // Pad to 32 bytes

        context.write_memory(addr, &padded_data).map_err(|e| {
            VMBridgeError::MemoryOperationFailed {
                message: e.to_string(),
            }
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
        gas.consume_gas("MSTORE8", Some(3))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let value = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let address = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let addr = match address {
            StackItem::UnsignedInteger(a) => a as usize,
            StackItem::Integer(a) => a as usize,
            _ => {
                return Err(VMBridgeError::MemoryOperationFailed {
                    message: "Invalid memory address".to_string(),
                })
            }
        };

        let byte_value = match value {
            StackItem::UnsignedInteger(v) => (v & 0xFF) as u8,
            StackItem::Integer(v) => (v & 0xFF) as u8,
            StackItem::ByteArray(bytes) => bytes.first().copied().unwrap_or(0),
            _ => 0,
        };

        context.write_memory(addr, &[byte_value]).map_err(|e| {
            VMBridgeError::MemoryOperationFailed {
                message: e.to_string(),
            }
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
        gas.consume_gas("SLOAD", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let key = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        // Use default account address for storage operations
        // Note: Override via ExecutionContext.set_storage_account() for custom accounts
        let account = "0x0000000000000000000000000000000000000000";
        let key_bytes = key.to_bytes();

        let value = storage.get(account, &key_bytes).map_err(|e| {
            VMBridgeError::StorageOperationFailed {
                message: e.to_string(),
            }
        })?;

        let result = match value {
            Some(data) => StackItem::ByteArray(data),
            None => StackItem::UnsignedInteger(0),
        };

        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
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
        gas.consume_gas("SSTORE", None)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let value = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let key = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let account = "0x0000000000000000000000000000000000000000";
        let key_bytes = key.to_bytes();
        let value_bytes = value.to_bytes();

        storage
            .set(account, &key_bytes, &value_bytes)
            .map_err(|e| VMBridgeError::StorageOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }

    // System call handlers

    fn syscall_keccak256(
        bridge: &mut VMBridge,
        args: &[StackItem],
    ) -> Result<Vec<StackItem>, VMBridgeError> {
        if args.len() != 1 {
            return Err(VMBridgeError::InvalidArguments {
                expected: 1,
                got: args.len(),
            });
        }

        let input = extract_bytes(&args[0])?;
        let hash = bridge.keccak256(&input);

        Ok(vec![StackItem::ByteArray(hash.to_vec())])
    }

    fn syscall_sha256(
        _bridge: &mut VMBridge,
        args: &[StackItem],
    ) -> Result<Vec<StackItem>, VMBridgeError> {
        if args.len() != 1 {
            return Err(VMBridgeError::InvalidArguments {
                expected: 1,
                got: args.len(),
            });
        }

        use sha2::{Digest, Sha256};

        let input = extract_bytes(&args[0])?;
        let hash = Sha256::digest(&input);

        Ok(vec![StackItem::ByteArray(hash.to_vec())])
    }

    fn syscall_ecrecover(
        bridge: &mut VMBridge,
        args: &[StackItem],
    ) -> Result<Vec<StackItem>, VMBridgeError> {
        // Complete ECDSA signature recovery implementation
        if args.len() != 4 {
            return Err(VMBridgeError::InvalidArguments {
                expected: 4,
                got: args.len(),
            });
        }

        let hash = extract_bytes(&args[0])?;
        let v = extract_integer(&args[1])? as u8;
        let r = extract_bytes(&args[2])?;
        let s = extract_bytes(&args[3])?;

        if hash.len() < 32 || r.len() != 32 || s.len() != 32 {
            return Err(VMBridgeError::SystemCallFailed {
                name: "ecrecover".to_string(),
                message: "Invalid hash or signature length".to_string(),
            });
        }

        use secp256k1::ecdsa::{RecoverableSignature, RecoveryId};
        use secp256k1::{Message, Secp256k1};

        let secp = Secp256k1::new();
        let message =
            Message::from_slice(&hash[..32]).map_err(|e| VMBridgeError::SystemCallFailed {
                name: "ecrecover".to_string(),
                message: format!("Invalid hash: {}", e),
            })?;

        let rec_id = match v {
            27 | 28 => RecoveryId::from_i32((v - 27) as i32).map_err(|e| {
                VMBridgeError::SystemCallFailed {
                    name: "ecrecover".to_string(),
                    message: format!("Invalid recovery id: {}", e),
                }
            })?,
            _ => {
                return Err(VMBridgeError::SystemCallFailed {
                    name: "ecrecover".to_string(),
                    message: "Recovery id must be 27 or 28".to_string(),
                })
            }
        };

        let mut sig_bytes = [0u8; 64];
        sig_bytes[..32].copy_from_slice(&r[..32]);
        sig_bytes[32..].copy_from_slice(&s[..32]);

        let signature = RecoverableSignature::from_compact(&sig_bytes, rec_id).map_err(|e| {
            VMBridgeError::SystemCallFailed {
                name: "ecrecover".to_string(),
                message: format!("Invalid signature: {}", e),
            }
        })?;

        let public_key = secp.recover_ecdsa(&message, &signature).map_err(|e| {
            VMBridgeError::SystemCallFailed {
                name: "ecrecover".to_string(),
                message: format!("Recovery failed: {}", e),
            }
        })?;

        let pub_bytes = public_key.serialize_uncompressed();
        let hash = bridge.keccak256(&pub_bytes[1..]); // remove prefix byte
        let address = hash[12..].to_vec(); // last 20 bytes

        Ok(vec![StackItem::ByteArray(address)])
    }

    fn syscall_verify(
        _bridge: &mut VMBridge,
        args: &[StackItem],
    ) -> Result<Vec<StackItem>, VMBridgeError> {
        if args.len() != 3 {
            return Err(VMBridgeError::InvalidArguments {
                expected: 3,
                got: args.len(),
            });
        }

        let hash = extract_bytes(&args[0])?;
        let signature = extract_bytes(&args[1])?;
        let public_key = extract_bytes(&args[2])?;

        if hash.len() < 32 || signature.len() != 64 {
            return Err(VMBridgeError::SystemCallFailed {
                name: "verify".to_string(),
                message: "Invalid hash or signature length".to_string(),
            });
        }

        use secp256k1::ecdsa::Signature;
        use secp256k1::{Message, PublicKey, Secp256k1};

        let secp = Secp256k1::new();

        // Create message from hash
        let message =
            Message::from_slice(&hash[..32]).map_err(|e| VMBridgeError::SystemCallFailed {
                name: "verify".to_string(),
                message: format!("Invalid hash: {}", e),
            })?;

        // Parse signature
        let sig = Signature::from_compact(&signature[..64]).map_err(|e| {
            VMBridgeError::SystemCallFailed {
                name: "verify".to_string(),
                message: format!("Invalid signature: {}", e),
            }
        })?;

        // Parse public key
        let pubkey =
            PublicKey::from_slice(&public_key).map_err(|e| VMBridgeError::SystemCallFailed {
                name: "verify".to_string(),
                message: format!("Invalid public key: {}", e),
            })?;

        // Verify signature
        let verification_result = secp.verify_ecdsa(&message, &sig, &pubkey).is_ok();

        Ok(vec![StackItem::Boolean(verification_result)])
    }

    fn keccak256(&self, data: &[u8]) -> [u8; 32] {
        use sha3::{Digest, Keccak256};
        let hash = Keccak256::digest(data);
        let mut out = [0u8; 32];
        out.copy_from_slice(&hash);
        out
    }

    // Arithmetic operations on stack items

    fn add_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_add(y)))
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_add(y)))
            }
            (StackItem::Integer(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::Integer(x.wrapping_add(y as i64)))
            }
            (StackItem::UnsignedInteger(x), StackItem::Integer(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_add(y as u64)))
            }
            _ => Err(VMBridgeError::StackOperationFailed {
                message: "Invalid operands for ADD".to_string(),
            }),
        }
    }

    fn sub_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_sub(y)))
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_sub(y)))
            }
            _ => Err(VMBridgeError::StackOperationFailed {
                message: "Invalid operands for SUB".to_string(),
            }),
        }
    }

    fn mul_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_mul(y)))
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_mul(y)))
            }
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
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                if y == 0 {
                    Ok(StackItem::UnsignedInteger(0))
                } else {
                    Ok(StackItem::UnsignedInteger(x / y))
                }
            }
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
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                if y == 0 {
                    Ok(StackItem::UnsignedInteger(0))
                } else {
                    Ok(StackItem::UnsignedInteger(x % y))
                }
            }
            _ => Err(VMBridgeError::StackOperationFailed {
                message: "Invalid operands for MOD".to_string(),
            }),
        }
    }

    fn modmul_stack_items(
        a: StackItem,
        b: StackItem,
        modulus: StackItem,
    ) -> Result<StackItem, VMBridgeError> {
        let (a_int, b_int, m_int) = match (a, b, modulus) {
            (
                StackItem::UnsignedInteger(x),
                StackItem::UnsignedInteger(y),
                StackItem::UnsignedInteger(m),
            ) => (x as u128, y as u128, m as u128),
            (StackItem::Integer(x), StackItem::Integer(y), StackItem::Integer(m)) => {
                if x < 0 || y < 0 || m <= 0 {
                    return Err(VMBridgeError::StackOperationFailed {
                        message: "MODMUL expects non-negative operands".to_string(),
                    });
                }
                (x as u128, y as u128, m as u128)
            }
            _ => {
                return Err(VMBridgeError::StackOperationFailed {
                    message: "Invalid operands for MODMUL".to_string(),
                })
            }
        };

        if m_int == 0 {
            return Ok(StackItem::UnsignedInteger(0));
        }
        let result = ((a_int % m_int) * (b_int % m_int)) % m_int;
        Ok(StackItem::UnsignedInteger(result as u64))
    }

    fn modpow_stack_items(
        base: StackItem,
        exponent: StackItem,
        modulus: StackItem,
    ) -> Result<StackItem, VMBridgeError> {
        let (mut base, exp, modulus) = match (base, exponent, modulus) {
            (
                StackItem::UnsignedInteger(b),
                StackItem::UnsignedInteger(e),
                StackItem::UnsignedInteger(m),
            ) => (b as u128, e as u128, m as u128),
            (StackItem::Integer(b), StackItem::Integer(e), StackItem::Integer(m)) => {
                if b < 0 || e < 0 || m <= 0 {
                    return Err(VMBridgeError::StackOperationFailed {
                        message: "MODPOW expects non-negative operands".to_string(),
                    });
                }
                (b as u128, e as u128, m as u128)
            }
            _ => {
                return Err(VMBridgeError::StackOperationFailed {
                    message: "Invalid operands for MODPOW".to_string(),
                })
            }
        };

        if modulus == 0 {
            return Ok(StackItem::UnsignedInteger(0));
        }

        base %= modulus;
        let mut result: u128 = 1;
        let mut exp_mut = exp;
        while exp_mut > 0 {
            if exp_mut & 1 == 1 {
                result = (result * base) % modulus;
            }
            base = (base * base) % modulus;
            exp_mut >>= 1;
        }

        Ok(StackItem::UnsignedInteger(result as u64))
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

    fn and_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x & y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x & y))
            }
            _ => Err(VMBridgeError::BridgeError {
                message: "Invalid operands for AND".to_string(),
            }),
        }
    }

    fn or_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x | y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x | y))
            }
            _ => Err(VMBridgeError::BridgeError {
                message: "Invalid operands for OR".to_string(),
            }),
        }
    }

    fn xor_stack_items(a: StackItem, b: StackItem) -> Result<StackItem, VMBridgeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x ^ y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x ^ y))
            }
            _ => Err(VMBridgeError::BridgeError {
                message: "Invalid operands for XOR".to_string(),
            }),
        }
    }

    fn shift_left_value(value: StackItem, shift: StackItem) -> Result<StackItem, VMBridgeError> {
        let amount = Self::extract_shift_amount(shift)?;
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(v.wrapping_shl(amount))),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(v.wrapping_shl(amount))),
            _ => Err(VMBridgeError::BridgeError {
                message: "Invalid operands for SHL".to_string(),
            }),
        }
    }

    fn shift_right_value(value: StackItem, shift: StackItem) -> Result<StackItem, VMBridgeError> {
        let amount = Self::extract_shift_amount(shift)?;
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(v.wrapping_shr(amount))),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(v.wrapping_shr(amount))),
            _ => Err(VMBridgeError::BridgeError {
                message: "Invalid operands for SHR".to_string(),
            }),
        }
    }

    fn extract_shift_amount(item: StackItem) -> Result<u32, VMBridgeError> {
        match item {
            StackItem::Integer(v) => {
                if v < 0 {
                    Err(VMBridgeError::BridgeError {
                        message: "Shift amount must be non-negative".to_string(),
                    })
                } else {
                    Ok((v as u64).min(63) as u32)
                }
            }
            StackItem::UnsignedInteger(v) => Ok((v.min(63)) as u32),
            _ => Err(VMBridgeError::BridgeError {
                message: "Invalid shift amount".to_string(),
            }),
        }
    }

    fn extract_return_data(
        &self,
        context: &execution::ExecutionContext,
    ) -> Result<Vec<u8>, RuntimeError> {
        Ok(context.return_data().to_vec())
    }
}

fn extract_bytes(item: &StackItem) -> Result<Vec<u8>, VMBridgeError> {
    Ok(match item {
        StackItem::ByteArray(bytes) => bytes.clone(),
        StackItem::UnsignedInteger(value) => value.to_be_bytes().to_vec(),
        StackItem::Integer(value) => {
            if *value < 0 {
                return Err(VMBridgeError::StackOperationFailed {
                    message: "Negative integers not supported for byte extraction".to_string(),
                });
            }
            (*value as u64).to_be_bytes().to_vec()
        }
        StackItem::Array(_) => {
            return Err(VMBridgeError::StackOperationFailed {
                message: "Cannot extract bytes from array stack item".to_string(),
            })
        }
        StackItem::Map(_) => {
            return Err(VMBridgeError::StackOperationFailed {
                message: "Cannot extract bytes from map".to_string(),
            })
        }
        StackItem::Boolean(flag) => vec![if *flag { 1 } else { 0 }],
        StackItem::Null => Vec::new(),
    })
}

fn extract_integer(item: &StackItem) -> Result<u128, VMBridgeError> {
    Ok(match item {
        StackItem::UnsignedInteger(value) => *value as u128,
        StackItem::Integer(value) => {
            if *value < 0 {
                return Err(VMBridgeError::StackOperationFailed {
                    message: "Negative integers not supported here".to_string(),
                });
            }
            *value as u128
        }
        StackItem::ByteArray(bytes) => {
            if bytes.is_empty() {
                0
            } else {
                let mut padded = [0u8; 16];
                let copy_len = bytes.len().min(16);
                padded[16 - copy_len..].copy_from_slice(&bytes[bytes.len() - copy_len..]);
                u128::from_be_bytes(padded)
            }
        }
        StackItem::Array(_) => {
            return Err(VMBridgeError::StackOperationFailed {
                message: "Arrays not supported for integer extraction".to_string(),
            });
        }
        StackItem::Map(_) => {
            return Err(VMBridgeError::StackOperationFailed {
                message: "Maps not supported for integer extraction".to_string(),
            });
        }
        StackItem::Boolean(flag) => {
            if *flag {
                1
            } else {
                0
            }
        }
        StackItem::Null => 0,
    })
}
