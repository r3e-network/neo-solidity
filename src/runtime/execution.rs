//! Execution Context Module
//!
//! Provides execution context and gas tracking for Neo runtime.

use super::{storage, RuntimeConfig, RuntimeError};
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sha3::Keccak256;
use std::collections::{HashMap, HashSet};

/// Execution context for runtime operations
#[derive(Debug)]
pub struct ExecutionContext {
    bytecode: Vec<u8>,
    input_data: Vec<u8>,
    gas_limit: u64,
    gas_used: u64,
    instruction_pointer: u32,
    stack: Vec<StackItem>,
    memory: Vec<u8>,
    memory_limit: usize,
    return_data: Vec<u8>,
    call_stack: Vec<CallFrame>,
    debugging_enabled: bool,
    breakpoints: HashSet<u32>,
    instruction_count: u64,
    max_stack_depth: u32,
    storage_overlay: HashMap<Vec<u8>, OverlayEntry>,
    storage_account: Option<String>,
    storage_host: Option<*mut storage::StorageManager>,
    default_account: String,
    default_account_bytes: Vec<u8>,
    caller_account: Option<Vec<u8>>,
    block_height: Option<u64>,
    default_block_height: u64,
    timestamp: Option<u64>,
    default_timestamp: u64,
    invocation_counter: u64,
    pending_caller_account: Option<Vec<u8>>,
    pending_block_height: Option<u64>,
    pending_timestamp: Option<u64>,
}

/// Stack item in execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StackItem {
    Integer(i64),
    UnsignedInteger(u64),
    ByteArray(Vec<u8>),
    Boolean(bool),
    Null,
}

/// Call frame for function calls
#[derive(Debug, Clone)]
pub struct CallFrame {
    pub return_address: u32,
    pub function_name: Option<String>,
    pub local_variables: HashMap<String, StackItem>,
    pub stack_base: usize,
}

/// Gas tracker for execution costs
#[derive(Debug)]
pub struct GasTracker {
    limit: u64,
    used: u64,
    base_cost: u64,
    operation_costs: HashMap<String, u64>,
}

/// Step result for debugging
#[derive(Debug)]
pub struct StepResult {
    pub instruction_pointer: u32,
    pub opcode: String,
    pub stack_items: Vec<StackItem>,
    pub gas_used: u64,
    pub memory_changes: Vec<MemoryChange>,
    pub halted: bool,
}

/// Memory change record
#[derive(Debug, Clone)]
pub struct MemoryChange {
    pub address: usize,
    pub old_value: u8,
    pub new_value: u8,
}

#[derive(Debug, Clone)]
struct OverlayEntry {
    value: Option<Vec<u8>>,
    dirty: bool,
}

const SYS_STORAGE_GET_CONTEXT: [u8; 4] = [155, 246, 103, 206];
const SYS_STORAGE_GET: [u8; 4] = [146, 93, 232, 49];
const SYS_STORAGE_PUT: [u8; 4] = [230, 63, 24, 132];
const SYS_CRYPTO_SHA256: [u8; 4] = [48, 86, 191, 186];
const SYS_BLOCKCHAIN_GET_HEIGHT: [u8; 4] = [126, 245, 114, 31];
const SYS_RUNTIME_GET_TIME: [u8; 4] = [183, 195, 136, 3];
const SYS_RUNTIME_CALLING_SCRIPT_HASH: [u8; 4] = [241, 144, 111, 97];
const SYS_RUNTIME_GET_INVOCATION_COUNTER: [u8; 4] = [132, 39, 17, 67];

type StorageOverlayEntries = Vec<(Vec<u8>, Option<Vec<u8>>)>;

impl ExecutionContext {
    /// Create new execution context
    pub fn new(config: &RuntimeConfig) -> Result<Self, RuntimeError> {
        let default_account = Self::normalize_account(&config.contract_account)?;
        let default_account_bytes = Self::account_string_to_bytes(&default_account)?;
        let default_block_height = config.default_block_height;
        let default_timestamp = config.default_timestamp;

        Ok(Self {
            bytecode: Vec::new(),
            input_data: Vec::new(),
            gas_limit: config.gas_limit,
            gas_used: 0,
            instruction_pointer: 0,
            stack: Vec::new(),
            memory: Vec::new(),
            memory_limit: config.memory_limit,
            return_data: Vec::new(),
            call_stack: Vec::new(),
            debugging_enabled: config.enable_debugging,
            breakpoints: HashSet::new(),
            instruction_count: 0,
            max_stack_depth: 0,
            storage_overlay: HashMap::new(),
            storage_account: Some(default_account.clone()),
            storage_host: None,
            default_account,
            default_account_bytes,
            caller_account: None,
            block_height: Some(default_block_height),
            default_block_height,
            timestamp: Some(default_timestamp),
            default_timestamp,
            invocation_counter: 0,
            pending_caller_account: None,
            pending_block_height: None,
            pending_timestamp: None,
        })
    }

    /// Initialize context for execution
    pub fn initialize(&mut self, bytecode: &[u8], input: &[u8]) -> Result<(), RuntimeError> {
        self.bytecode = bytecode.to_vec();
        self.input_data = input.to_vec();
        self.instruction_pointer = 0;
        self.stack.clear();
        self.memory.clear();
        self.return_data.clear();
        self.call_stack.clear();
        self.gas_used = 0;
        self.instruction_count = 0;
        self.storage_overlay.clear();
        self.storage_account = Some(self.default_account.clone());
        self.storage_host = None;
        self.caller_account = self.pending_caller_account.take();
        self.block_height = self
            .pending_block_height
            .take()
            .or(Some(self.default_block_height));
        self.timestamp = self
            .pending_timestamp
            .take()
            .or(Some(self.default_timestamp));
        self.invocation_counter = 0;
        Ok(())
    }

    /// Override the block height for the next execution.
    pub fn override_block_height(&mut self, height: u64) {
        self.pending_block_height = Some(height);
    }

    /// Override the timestamp for the next execution.
    pub fn override_timestamp(&mut self, timestamp: u64) {
        self.pending_timestamp = Some(timestamp);
    }

    /// Override the calling script hash for the next execution.
    pub fn override_caller_account(&mut self, account: &str) -> Result<(), RuntimeError> {
        let normalized = Self::normalize_account(account)?;
        let bytes = Self::account_string_to_bytes(&normalized)?;
        self.pending_caller_account = Some(bytes);
        Ok(())
    }

    /// Clear any pending metadata overrides before the next execution.
    pub fn clear_pending_overrides(&mut self) {
        self.pending_block_height = None;
        self.pending_timestamp = None;
        self.pending_caller_account = None;
    }

    /// Get gas limit
    pub fn gas_limit(&self) -> u64 {
        self.gas_limit
    }

    /// Get instruction count
    pub fn instruction_count(&self) -> u64 {
        self.instruction_count
    }

    /// Get pending block height override, if any.
    pub fn pending_block_height(&self) -> Option<u64> {
        self.pending_block_height
    }

    /// Get pending timestamp override, if any.
    pub fn pending_timestamp(&self) -> Option<u64> {
        self.pending_timestamp
    }

    /// Get pending caller account override bytes, if any.
    pub fn pending_caller_account(&self) -> Option<&[u8]> {
        self.pending_caller_account.as_deref()
    }

    /// Get the active block height for the current execution.
    pub fn block_height(&self) -> Option<u64> {
        self.block_height
    }

    /// Get the active timestamp for the current execution.
    pub fn timestamp(&self) -> Option<u64> {
        self.timestamp
    }

    /// Get the active caller account for the current execution.
    pub fn caller_account(&self) -> Option<&[u8]> {
        self.caller_account.as_deref()
    }

    /// Get the default account bytes configured for this context.
    pub fn default_account_bytes(&self) -> &[u8] {
        &self.default_account_bytes
    }

    /// Get maximum stack depth
    pub fn max_stack_depth(&self) -> u32 {
        self.max_stack_depth
    }

    /// Enable debugging
    pub fn enable_debugging(&mut self) {
        self.debugging_enabled = true;
    }

    /// Disable debugging
    pub fn disable_debugging(&mut self) {
        self.debugging_enabled = false;
    }

    /// Set breakpoint
    pub fn set_breakpoint(&mut self, address: u32) {
        self.breakpoints.insert(address);
    }

    /// Remove breakpoint
    pub fn remove_breakpoint(&mut self, address: u32) {
        self.breakpoints.remove(&address);
    }

    /// Step through one instruction
    pub fn step(&mut self) -> Result<StepResult, RuntimeError> {
        if self.instruction_pointer as usize >= self.bytecode.len() {
            return Ok(StepResult {
                instruction_pointer: self.instruction_pointer,
                opcode: "HALT".to_string(),
                stack_items: self.stack.clone(),
                gas_used: self.gas_used,
                memory_changes: Vec::new(),
                halted: true,
            });
        }

        let opcode = self.bytecode[self.instruction_pointer as usize];
        let opcode_name = self.get_opcode_name(opcode);
        let old_gas = self.gas_used;

        // Execute instruction (simplified)
        self.execute_instruction(opcode)?;

        self.instruction_count += 1;
        self.max_stack_depth = self.max_stack_depth.max(self.stack.len() as u32);

        Ok(StepResult {
            instruction_pointer: self.instruction_pointer,
            opcode: opcode_name,
            stack_items: self.stack.clone(),
            gas_used: self.gas_used - old_gas,
            memory_changes: Vec::new(), // Would track actual changes
            halted: false,
        })
    }

    /// Push value onto stack
    pub fn push_stack(&mut self, item: StackItem) -> Result<(), RuntimeError> {
        if self.stack.len() >= 2048 {
            // NeoVM stack limit
            return Err(RuntimeError::StackOverflow {
                depth: self.stack.len() as u32,
            });
        }
        self.stack.push(item);
        Ok(())
    }

    /// Pop value from stack
    pub fn pop_stack(&mut self) -> Result<StackItem, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::ExecutionError {
            message: "Stack underflow".to_string(),
        })
    }

    /// Peek at top stack item
    pub fn peek_stack(&self) -> Result<&StackItem, RuntimeError> {
        self.stack.last().ok_or(RuntimeError::ExecutionError {
            message: "Stack is empty".to_string(),
        })
    }

    /// Get stack depth
    pub fn stack_depth(&self) -> usize {
        self.stack.len()
    }

    /// Read from memory, expanding zero-initialized regions as needed
    pub fn read_memory(&mut self, address: usize, length: usize) -> Result<&[u8], RuntimeError> {
        if length == 0 {
            if address > self.memory_limit {
                return Err(RuntimeError::ExecutionError {
                    message: "Memory access exceeds configured limit".to_string(),
                });
            }
            return Ok(&[]);
        }

        let required = address
            .checked_add(length)
            .ok_or(RuntimeError::ExecutionError {
                message: "Memory access overflow".to_string(),
            })?;
        if required > self.memory_limit {
            return Err(RuntimeError::ExecutionError {
                message: "Memory access exceeds configured limit".to_string(),
            });
        }

        if required > self.memory.len() {
            self.memory.resize(required, 0);
        }
        Ok(&self.memory[address..required])
    }

    /// Write to memory
    pub fn write_memory(&mut self, address: usize, data: &[u8]) -> Result<(), RuntimeError> {
        if data.is_empty() {
            return Ok(());
        }

        let required = address
            .checked_add(data.len())
            .ok_or(RuntimeError::ExecutionError {
                message: "Memory write overflow".to_string(),
            })?;
        if required > self.memory_limit {
            return Err(RuntimeError::ExecutionError {
                message: "Memory write exceeds configured limit".to_string(),
            });
        }

        if required > self.memory.len() {
            self.memory.resize(required, 0);
        }
        self.memory[address..required].copy_from_slice(data);
        Ok(())
    }

    /// Return the length of the calldata buffer
    pub fn input_size(&self) -> usize {
        self.input_data.len()
    }

    /// Read a slice of calldata, zero-padding when the requested range exceeds the buffer
    pub fn read_input_slice(&self, offset: usize, length: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; length];
        if offset >= self.input_data.len() {
            return buffer;
        }

        let available = (offset + length).min(self.input_data.len());
        let copy_len = available.saturating_sub(offset);
        buffer[..copy_len].copy_from_slice(&self.input_data[offset..offset + copy_len]);
        buffer
    }

    /// Copy calldata into linear memory, expanding memory as required
    pub fn copy_input_to_memory(
        &mut self,
        memory_offset: usize,
        input_offset: usize,
        length: usize,
    ) -> Result<(), RuntimeError> {
        if length == 0 {
            return Ok(());
        }

        let data = self.read_input_slice(input_offset, length);
        self.write_memory(memory_offset, &data)
    }

    /// Current size of the linear memory buffer
    pub fn memory_size(&self) -> usize {
        self.memory.len()
    }

    /// Size of the calldata buffer
    pub fn calldatasize(&self) -> usize {
        self.input_data.len()
    }

    /// Load a 32-byte word from calldata starting at `offset`
    pub fn calldataload_word(&self, offset: usize) -> [u8; 32] {
        let data = self.read_input_slice(offset, 32);
        let mut word = [0u8; 32];
        word.copy_from_slice(&data);
        word
    }

    /// Copy calldata into linear memory
    pub fn calldatacopy(
        &mut self,
        memory_offset: usize,
        data_offset: usize,
        length: usize,
    ) -> Result<(), RuntimeError> {
        if length == 0 {
            return Ok(());
        }
        let data = self.read_input_slice(data_offset, length);
        self.write_memory(memory_offset, &data)
    }

    /// Compute the Keccak-256 hash over a memory slice
    pub fn keccak_memory_slice(
        &mut self,
        offset: usize,
        length: usize,
    ) -> Result<[u8; 32], RuntimeError> {
        use sha3::Digest as KeccakDigest;

        let slice = self.read_memory(offset, length)?;
        let mut hasher = Keccak256::new();
        KeccakDigest::update(&mut hasher, slice);
        let digest = hasher.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(&digest);
        Ok(out)
    }

    /// Size of the last return buffer
    pub fn returndatasize(&self) -> usize {
        self.return_data.len()
    }

    /// Copy bytes from the last return buffer into memory
    pub fn returndatacopy(
        &mut self,
        memory_offset: usize,
        return_offset: usize,
        length: usize,
    ) -> Result<(), RuntimeError> {
        if length == 0 {
            return Ok(());
        }

        let end = return_offset
            .checked_add(length)
            .ok_or(RuntimeError::ExecutionError {
                message: "Return data copy overflow".to_string(),
            })?;

        if end > self.return_data.len() {
            return Err(RuntimeError::ExecutionError {
                message: "Return data access out of bounds".to_string(),
            });
        }

        let slice = self.return_data[return_offset..end].to_vec();
        self.write_memory(memory_offset, &slice)
    }

    /// Replace the current return buffer
    pub fn set_return_data(&mut self, data: Vec<u8>) {
        self.return_data = data;
    }

    /// Access the return buffer
    pub fn return_data(&self) -> &[u8] {
        &self.return_data
    }

    /// Call function
    pub fn call_function(
        &mut self,
        address: u32,
        function_name: Option<String>,
    ) -> Result<(), RuntimeError> {
        if self.call_stack.len() >= 1024 {
            // Call stack limit
            return Err(RuntimeError::ExecutionError {
                message: "Call stack overflow".to_string(),
            });
        }

        let frame = CallFrame {
            return_address: self.instruction_pointer + 1,
            function_name,
            local_variables: HashMap::new(),
            stack_base: self.stack.len(),
        };

        self.call_stack.push(frame);
        self.instruction_pointer = address;
        Ok(())
    }

    /// Return from function
    pub fn return_from_function(&mut self) -> Result<(), RuntimeError> {
        if let Some(frame) = self.call_stack.pop() {
            self.instruction_pointer = frame.return_address;
            // Restore stack to base level
            self.stack.truncate(frame.stack_base);
            Ok(())
        } else {
            Err(RuntimeError::ExecutionError {
                message: "No function to return from".to_string(),
            })
        }
    }

    // Private helper methods

    fn execute_instruction(&mut self, opcode: u8) -> Result<(), RuntimeError> {
        // Check gas before execution
        let gas_cost = self.get_instruction_gas_cost(opcode);
        if self.gas_used + gas_cost > self.gas_limit {
            return Err(RuntimeError::OutOfGas {
                used: self.gas_used + gas_cost,
                limit: self.gas_limit,
            });
        }

        // Complete NeoVM instruction execution
        match opcode {
            // Push operations (0x00-0x4F)
            0x00 => {
                // PUSHINT8
                if self.instruction_pointer + 1 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT8: insufficient bytecode".to_string(),
                    });
                }
                let value = self.bytecode[self.instruction_pointer as usize + 1] as i8 as i64;
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 2;
            }
            0x01 => {
                // PUSHINT16
                if self.instruction_pointer + 2 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT16: insufficient bytecode".to_string(),
                    });
                }
                let bytes = &self.bytecode
                    [self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 3];
                let value = i16::from_le_bytes([bytes[0], bytes[1]]) as i64;
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 3;
            }
            0x02 => {
                // PUSHINT32
                if self.instruction_pointer + 4 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT32: insufficient bytecode".to_string(),
                    });
                }
                let bytes = &self.bytecode
                    [self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 5];
                let value = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64;
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 5;
            }
            0x03 => {
                // PUSHINT64
                if self.instruction_pointer + 8 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT64: insufficient bytecode".to_string(),
                    });
                }
                let bytes = &self.bytecode
                    [self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 9];
                let mut array = [0u8; 8];
                array.copy_from_slice(bytes);
                let value = i64::from_le_bytes(array);
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 9;
            }
            0x0C => {
                // PUSHDATA1
                if self.instruction_pointer + 1 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHDATA1: insufficient bytecode for length".to_string(),
                    });
                }
                let length = self.bytecode[self.instruction_pointer as usize + 1] as usize;
                if self.instruction_pointer as usize + 2 + length > self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHDATA1: insufficient bytecode for data".to_string(),
                    });
                }
                let data = self.bytecode[self.instruction_pointer as usize + 2
                    ..self.instruction_pointer as usize + 2 + length]
                    .to_vec();
                self.push_stack(StackItem::ByteArray(data))?;
                self.instruction_pointer += 2 + length as u32;
            }
            0x10 => {
                // PUSHM1
                self.push_stack(StackItem::Integer(-1))?;
                self.instruction_pointer += 1;
            }
            0x11 => {
                // PUSH0
                self.push_stack(StackItem::Integer(0))?;
                self.instruction_pointer += 1;
            }
            0x12..=0x20 => {
                // PUSH1-PUSH16
                let value = (opcode - 0x11) as i64;
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 1;
            }

            // Flow control operations (0x21-0x38)
            0x21 => {
                // NOP
                self.instruction_pointer += 1;
            }
            0x22 => {
                // JMP
                if self.instruction_pointer + 1 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "JMP: insufficient bytecode for offset".to_string(),
                    });
                }
                let offset = self.bytecode[self.instruction_pointer as usize + 1] as i8;
                let new_ip = (self.instruction_pointer as i32 + offset as i32 + 2) as u32;
                if new_ip >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "JMP: jump target out of bounds".to_string(),
                    });
                }
                self.instruction_pointer = new_ip;
            }
            0x23 => {
                // JMPIF
                let condition = self.pop_stack()?;
                if self.instruction_pointer + 1 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "JMPIF: insufficient bytecode for offset".to_string(),
                    });
                }
                let offset = self.bytecode[self.instruction_pointer as usize + 1] as i8;
                if condition.is_truthy() {
                    let new_ip = (self.instruction_pointer as i32 + offset as i32 + 2) as u32;
                    if new_ip >= self.bytecode.len() as u32 {
                        return Err(RuntimeError::ExecutionError {
                            message: "JMPIF: jump target out of bounds".to_string(),
                        });
                    }
                    self.instruction_pointer = new_ip;
                } else {
                    self.instruction_pointer += 2;
                }
            }

            // Stack operations (0x39-0x4F)
            0x39 => {
                // DEPTH
                self.push_stack(StackItem::Integer(self.stack.len() as i64))?;
                self.instruction_pointer += 1;
            }
            0x3A => {
                // DROP
                self.pop_stack()?;
                self.instruction_pointer += 1;
            }
            0x3E => {
                // DUP
                let top = self.peek_stack()?.clone();
                self.push_stack(top)?;
                self.instruction_pointer += 1;
            }
            0x90 => {
                // INVERT
                let value = self.pop_stack()?;
                let result = self.bitwise_not(value)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x91 => {
                // AND
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.bitwise_and(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x92 => {
                // OR
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.bitwise_or(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x93 => {
                // XOR
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.bitwise_xor(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x9E => {
                // SHL
                let shift = self.pop_stack()?;
                let value = self.pop_stack()?;
                let result = self.shift_left(value, shift)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x9F => {
                // SHR
                let shift = self.pop_stack()?;
                let value = self.pop_stack()?;
                let result = self.shift_right(value, shift)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x41 => {
                if self.instruction_pointer + 4 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "SYSCALL: insufficient bytecode".to_string(),
                    });
                }

                let mut syscall_id = [0u8; 4];
                syscall_id.copy_from_slice(
                    &self.bytecode[self.instruction_pointer as usize + 1
                        ..self.instruction_pointer as usize + 5],
                );

                match syscall_id {
                    SYS_STORAGE_GET_CONTEXT => {
                        self.push_stack(StackItem::ByteArray(Vec::new()))?;
                    }
                    SYS_STORAGE_GET => {
                        let slot_item = self.pop_stack()?;
                        let _context = self.pop_stack()?; // ignored
                        let key = Self::stack_item_to_bytes(slot_item);

                        let value = if let Some(entry) = self.storage_overlay.get(&key) {
                            entry.value.clone().unwrap_or_default()
                        } else {
                            let fetched = self.fetch_storage_value(&key)?;
                            let bytes = fetched.clone().unwrap_or_default();
                            self.storage_overlay.insert(
                                key.clone(),
                                OverlayEntry {
                                    value: fetched,
                                    dirty: false,
                                },
                            );
                            bytes
                        };

                        self.push_stack(StackItem::ByteArray(value))?;
                    }
                    SYS_STORAGE_PUT => {
                        let value_item = self.pop_stack()?;
                        let _context = self.pop_stack()?; // ignored
                        let slot_item = self.pop_stack()?;

                        let key = Self::stack_item_to_bytes(slot_item);
                        let value = Self::stack_item_to_bytes(value_item);

                        let entry = self.storage_overlay.entry(key.clone()).or_insert_with(|| {
                            OverlayEntry {
                                value: None,
                                dirty: false,
                            }
                        });
                        entry.value = if value.is_empty() { None } else { Some(value) };
                        entry.dirty = true;
                    }
                    SYS_BLOCKCHAIN_GET_HEIGHT => {
                        let height = *self.block_height.get_or_insert(self.default_block_height);
                        self.push_stack(StackItem::UnsignedInteger(height))?;
                    }
                    SYS_RUNTIME_GET_TIME => {
                        let timestamp = *self.timestamp.get_or_insert(self.default_timestamp);
                        self.push_stack(StackItem::UnsignedInteger(timestamp))?;
                    }
                    SYS_RUNTIME_CALLING_SCRIPT_HASH => {
                        if self.caller_account.is_none() {
                            self.caller_account = Some(self.default_account_bytes.clone());
                            self.storage_account = Some(self.default_account.clone());
                        }
                        let bytes = self
                            .caller_account
                            .clone()
                            .unwrap_or_else(|| self.default_account_bytes.clone());
                        self.push_stack(StackItem::ByteArray(bytes))?;
                    }
                    SYS_RUNTIME_GET_INVOCATION_COUNTER => {
                        self.invocation_counter += 1;
                        self.push_stack(StackItem::UnsignedInteger(self.invocation_counter))?;
                    }
                    SYS_CRYPTO_SHA256 => {
                        let data = self.pop_stack()?;
                        let bytes = Self::stack_item_to_bytes(data);
                        let digest = Sha256::digest(&bytes);
                        self.push_stack(StackItem::ByteArray(digest[..].to_vec()))?;
                    }
                    _ => {}
                }

                self.instruction_pointer += 5;
            }
            0x42 => {
                // SWAP
                let top = self.pop_stack()?;
                let second = self.pop_stack()?;
                self.push_stack(top)?;
                self.push_stack(second)?;
                self.instruction_pointer += 1;
            }

            // Arithmetic (0x95-0x99)
            0x95 => {
                // ADD
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.add_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x96 => {
                // SUB
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.sub_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x97 => {
                // MUL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.mul_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x98 => {
                // DIV
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.div_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x99 => {
                // MOD
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.mod_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }

            // Comparison operations
            0xA3 => {
                // EQUAL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
            }
            0xA4 => {
                // NOT EQUAL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(!result))?;
                self.instruction_pointer += 1;
            }
            0xA5 => {
                // LT
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.less_than(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
            }
            0xA6 => {
                // LE
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let lt = self.less_than(&a, &b)?;
                let eq = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(lt || eq))?;
                self.instruction_pointer += 1;
            }
            0xA7 => {
                // GT
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.greater_than(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
            }
            0xA8 => {
                // GE
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let gt = self.greater_than(&a, &b)?;
                let eq = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(gt || eq))?;
                self.instruction_pointer += 1;
            }

            // Control flow
            0x40 => {
                // RET
                if self.call_stack.is_empty() {
                    if let Some(item) = self.stack.last() {
                        self.return_data = Self::stack_item_to_bytes(item.clone());
                    } else {
                        self.return_data.clear();
                    }
                    self.instruction_pointer = self.bytecode.len() as u32;
                } else {
                    self.return_from_function()?;
                }
            }
            0x66 => {
                // THROW
                return Err(RuntimeError::ExecutionError {
                    message: "THROW instruction executed".to_string(),
                });
            }
            0x67 => {
                // ABORT
                return Err(RuntimeError::ExecutionError {
                    message: "ABORT instruction executed".to_string(),
                });
            }

            _ => {
                return Err(RuntimeError::ExecutionError {
                    message: format!("Unsupported opcode: 0x{:02X}", opcode),
                });
            }
        }

        // Consume gas after successful execution
        self.gas_used += gas_cost;

        Ok(())
    }

    fn add_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_add(y)))
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_add(y)))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for ADD".to_string(),
            }),
        }
    }

    fn sub_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_sub(y)))
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_sub(y)))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for SUB".to_string(),
            }),
        }
    }

    fn mul_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_mul(y)))
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_mul(y)))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for MUL".to_string(),
            }),
        }
    }

    fn div_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                if y == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "Division by zero".to_string(),
                    });
                }
                Ok(StackItem::Integer(x / y))
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                if y == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "Division by zero".to_string(),
                    });
                }
                Ok(StackItem::UnsignedInteger(x / y))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for DIV".to_string(),
            }),
        }
    }

    fn mod_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                if y == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "Modulo by zero".to_string(),
                    });
                }
                Ok(StackItem::Integer(x % y))
            }
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                if y == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "Modulo by zero".to_string(),
                    });
                }
                Ok(StackItem::UnsignedInteger(x % y))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for MOD".to_string(),
            }),
        }
    }

    fn bitwise_not(&self, value: StackItem) -> Result<StackItem, RuntimeError> {
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(!v)),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(!v)),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operand for bitwise NOT".to_string(),
            }),
        }
    }

    fn bitwise_and(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x & y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x & y))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise AND".to_string(),
            }),
        }
    }

    fn bitwise_or(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x | y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x | y))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise OR".to_string(),
            }),
        }
    }

    fn bitwise_xor(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(StackItem::Integer(x ^ y)),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x ^ y))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for bitwise XOR".to_string(),
            }),
        }
    }

    fn shift_left(&self, value: StackItem, shift: StackItem) -> Result<StackItem, RuntimeError> {
        let amount = self.extract_shift_amount(shift)?;
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(v.wrapping_shl(amount))),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(v.wrapping_shl(amount))),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for shift left".to_string(),
            }),
        }
    }

    fn shift_right(&self, value: StackItem, shift: StackItem) -> Result<StackItem, RuntimeError> {
        let amount = self.extract_shift_amount(shift)?;
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(v.wrapping_shr(amount))),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(v.wrapping_shr(amount))),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for shift right".to_string(),
            }),
        }
    }

    fn extract_shift_amount(&self, item: StackItem) -> Result<u32, RuntimeError> {
        match item {
            StackItem::Integer(v) => {
                if v < 0 {
                    Err(RuntimeError::ExecutionError {
                        message: "Shift amount must be non-negative".to_string(),
                    })
                } else {
                    Ok((v as u64).min(63) as u32)
                }
            }
            StackItem::UnsignedInteger(v) => Ok((v.min(63)) as u32),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid shift amount".to_string(),
            }),
        }
    }

    fn stack_items_equal(&self, a: &StackItem, b: &StackItem) -> Result<bool, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(x == y),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => Ok(x == y),
            (StackItem::Boolean(x), StackItem::Boolean(y)) => Ok(x == y),
            (StackItem::ByteArray(x), StackItem::ByteArray(y)) => Ok(x == y),
            (StackItem::Null, StackItem::Null) => Ok(true),
            // Cross-type comparisons
            (StackItem::Integer(x), StackItem::UnsignedInteger(y)) => {
                if *x < 0 {
                    Ok(false)
                } else {
                    Ok(*x as u64 == *y)
                }
            }
            (StackItem::UnsignedInteger(x), StackItem::Integer(y)) => {
                if *y < 0 {
                    Ok(false)
                } else {
                    Ok(*x == *y as u64)
                }
            }
            _ => Ok(false),
        }
    }

    fn stack_item_to_bytes(item: StackItem) -> Vec<u8> {
        match item {
            StackItem::ByteArray(bytes) => bytes,
            StackItem::Integer(value) => value.to_le_bytes().to_vec(),
            StackItem::UnsignedInteger(value) => value.to_le_bytes().to_vec(),
            StackItem::Boolean(value) => vec![value as u8],
            StackItem::Null => Vec::new(),
        }
    }

    fn normalize_account(account: &str) -> Result<String, RuntimeError> {
        let trimmed = account.trim();
        let without_prefix = trimmed.strip_prefix("0x").unwrap_or(trimmed);
        if without_prefix.len() % 2 != 0 {
            return Err(RuntimeError::ConfigurationError {
                message: "contract account hex string has odd length".to_string(),
            });
        }
        let lower = without_prefix.to_ascii_lowercase();
        Ok(format!("0x{}", lower))
    }

    fn account_string_to_bytes(account: &str) -> Result<Vec<u8>, RuntimeError> {
        let normalized = Self::normalize_account(account)?;
        let hex_part = normalized.trim_start_matches("0x");
        hex::decode(hex_part).map_err(|e| RuntimeError::ConfigurationError {
            message: format!("invalid contract account: {}", e),
        })
    }

    pub fn bind_storage(
        &mut self,
        account: &str,
        storage: &mut storage::StorageManager,
    ) -> Result<(), RuntimeError> {
        let normalized = Self::normalize_account(account)?;
        self.storage_account = Some(normalized);
        self.storage_host = Some(storage as *mut _);
        self.storage_overlay.clear();
        Ok(())
    }

    pub fn unbind_storage(&mut self) {
        self.storage_host = None;
        self.storage_account = Some(self.default_account.clone());
        self.storage_overlay.clear();
    }

    pub fn drain_dirty_storage_overlay(&mut self) -> Option<(String, StorageOverlayEntries)> {
        let account = self.storage_account.clone()?;
        let mut entries = Vec::new();
        for (key, entry) in self.storage_overlay.drain() {
            if entry.dirty {
                entries.push((key, entry.value));
            }
        }

        if entries.is_empty() {
            None
        } else {
            Some((account, entries))
        }
    }

    fn fetch_storage_value(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, RuntimeError> {
        match (self.storage_host, self.storage_account.as_ref()) {
            (Some(ptr), Some(account)) => {
                let storage = unsafe { &mut *ptr };
                storage.get(account, key)
            }
            _ => Ok(None),
        }
    }

    fn less_than(&self, a: &StackItem, b: &StackItem) -> Result<bool, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(x < y),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => Ok(x < y),
            (StackItem::Integer(x), StackItem::UnsignedInteger(y)) => {
                if *x < 0 {
                    Ok(true)
                } else {
                    Ok((*x as u64) < *y)
                }
            }
            (StackItem::UnsignedInteger(x), StackItem::Integer(y)) => {
                if *y < 0 {
                    Ok(false)
                } else {
                    Ok(*x < (*y as u64))
                }
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for comparison".to_string(),
            }),
        }
    }

    fn greater_than(&self, a: &StackItem, b: &StackItem) -> Result<bool, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => Ok(x > y),
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => Ok(x > y),
            (StackItem::Integer(x), StackItem::UnsignedInteger(y)) => {
                if *x < 0 {
                    Ok(false)
                } else {
                    Ok((*x as u64) > *y)
                }
            }
            (StackItem::UnsignedInteger(x), StackItem::Integer(y)) => {
                if *y < 0 {
                    Ok(true)
                } else {
                    Ok(*x > (*y as u64))
                }
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for comparison".to_string(),
            }),
        }
    }

    fn get_opcode_name(&self, opcode: u8) -> String {
        match opcode {
            0x10 => "PUSH0".to_string(),
            0x11 => "PUSH1".to_string(),
            0x90 => "INVERT".to_string(),
            0x91 => "AND".to_string(),
            0x92 => "OR".to_string(),
            0x93 => "XOR".to_string(),
            0x95 => "ADD".to_string(),
            0x96 => "SUB".to_string(),
            0x9E => "SHL".to_string(),
            0x9F => "SHR".to_string(),
            0x40 => "RET".to_string(),
            _ => format!("UNKNOWN_{:02X}", opcode),
        }
    }

    fn get_instruction_gas_cost(&self, opcode: u8) -> u64 {
        match opcode {
            // Push operations
            0x0C | 0x0D => 2,               // PUSHDATA variants (base cost)
            0x00..=0x0B | 0x0E..=0x0F => 1, // PUSHINT variants
            0x10..=0x20 => 1,               // PUSH0-PUSH16, PUSHM1

            // Flow control
            0x21 => 1,        // NOP
            0x22..=0x28 => 2, // Jump instructions

            // Stack operations
            0x39..=0x3F | 0x41..=0x47 => 2, // Stack manipulation (excluding 0x40)

            // Bitwise operations
            0x90..=0x93 => 3,

            // Arithmetic
            0x94 => 4,        // Reserved/unary operations
            0x95 => 8,        // POW (expensive)
            0x96 => 6,        // SQRT
            0x97..=0x99 => 4, // Remaining arithmetic

            // Shift operations
            0x9E..=0x9F => 3,

            // Comparison operations
            0x87..=0x88 => 3, // EQUAL, NOTEQUAL
            0xA5..=0xA8 => 3, // LT, LE, GT, GE

            // Control flow
            0x40 => 0,        // RET
            0x66..=0x67 => 1, // THROW, ABORT

            _ => 1, // Default cost
        }
    }
}

impl GasTracker {
    /// Create new gas tracker
    pub fn new(limit: u64) -> Self {
        let mut operation_costs = HashMap::new();
        operation_costs.insert("ADD".to_string(), 3);
        operation_costs.insert("SUB".to_string(), 3);
        operation_costs.insert("MUL".to_string(), 5);
        operation_costs.insert("DIV".to_string(), 5);
        operation_costs.insert("PUSH".to_string(), 1);
        operation_costs.insert("POP".to_string(), 2);
        operation_costs.insert("CALL".to_string(), 700);
        operation_costs.insert("SSTORE".to_string(), 20000);
        operation_costs.insert("SLOAD".to_string(), 800);

        Self {
            limit,
            used: 0,
            base_cost: 21000, // Base transaction cost
            operation_costs,
        }
    }

    /// Reset gas tracker
    pub fn reset(&mut self, new_limit: u64) {
        self.limit = new_limit;
        self.used = self.base_cost;
    }

    /// Consume gas for operation
    pub fn consume_gas(
        &mut self,
        operation: &str,
        amount: Option<u64>,
    ) -> Result<(), RuntimeError> {
        let cost = amount.unwrap_or_else(|| *self.operation_costs.get(operation).unwrap_or(&1));

        if self.used + cost > self.limit {
            return Err(RuntimeError::OutOfGas {
                used: self.used + cost,
                limit: self.limit,
            });
        }

        self.used += cost;
        Ok(())
    }

    /// Get remaining gas
    pub fn remaining(&self) -> u64 {
        self.limit.saturating_sub(self.used)
    }

    /// Get used gas
    pub fn used(&self) -> u64 {
        self.used
    }

    /// Get gas limit
    pub fn limit(&self) -> u64 {
        self.limit
    }

    /// Check if out of gas
    pub fn out_of_gas(&self) -> bool {
        self.used >= self.limit
    }
}

impl StackItem {
    /// Convert to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            StackItem::Integer(i) => i.to_le_bytes().to_vec(),
            StackItem::UnsignedInteger(u) => u.to_le_bytes().to_vec(),
            StackItem::ByteArray(bytes) => bytes.clone(),
            StackItem::Boolean(b) => vec![if *b { 1 } else { 0 }],
            StackItem::Null => vec![0],
        }
    }

    /// Create from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return StackItem::Null;
        }

        if bytes.len() == 8 {
            if let Ok(array) = bytes.try_into() {
                return StackItem::UnsignedInteger(u64::from_le_bytes(array));
            }
        }

        StackItem::ByteArray(bytes.to_vec())
    }

    /// Check if truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            StackItem::Integer(i) => *i != 0,
            StackItem::UnsignedInteger(u) => *u != 0,
            StackItem::ByteArray(bytes) => !bytes.is_empty() && bytes.iter().any(|&b| b != 0),
            StackItem::Boolean(b) => *b,
            StackItem::Null => false,
        }
    }
}
