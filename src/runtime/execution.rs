//! Execution Context Module
//!
//! Provides execution context and gas tracking for Neo runtime.

        use super::{spec, storage, LogEntry, RuntimeConfig, RuntimeError};
use hex;
use ripemd::Ripemd160;
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use sha3::Keccak256;
use std::collections::{HashMap, HashSet};
use std::ptr::NonNull;

/// Execution context for runtime operations
#[derive(Debug)]
pub struct ExecutionContext {
    bytecode: Vec<u8>,
    input_data: Vec<u8>,
    gas_limit: u64,
    gas_used: u64,
    instruction_pointer: u32,
    call_stack_limit: u32,
    stack: Vec<StackItem>,
    locals: Vec<StackItem>,
    args: Vec<StackItem>,
    static_fields: Vec<StackItem>,
    memory: Vec<u8>,
    memory_limit: usize,
    return_data: Vec<u8>,
    logs: Vec<LogEntry>,
    call_stack: Vec<CallFrame>,
    try_stack: Vec<TryFrame>,
    iterators: HashMap<u64, IteratorState>,
    next_iterator_id: u64,
    syscall_gas: HashMap<[u8; 4], u64>,
    debugging_enabled: bool,
    breakpoints: HashSet<u32>,
    instruction_count: u64,
    max_stack_depth: u32,
    storage_overlay: HashMap<Vec<u8>, OverlayEntry>,
    storage_account: Option<String>,
    storage_host: Option<NonNull<storage::StorageManager>>,
    default_account: String,
    default_account_bytes: Vec<u8>,
    caller_account: Option<Vec<u8>>,
    block_height: Option<u64>,
    default_block_height: u64,
    timestamp: Option<u64>,
    default_timestamp: u64,
    invocation_counter: u64,
    network_magic: u32,
    pending_caller_account: Option<Vec<u8>>,
    pending_block_height: Option<u64>,
    pending_timestamp: Option<u64>,
    neo_balances: HashMap<Vec<u8>, u64>,
    gas_balances: HashMap<Vec<u8>, u64>,
    neo_total_supply: u64,
    gas_total_supply: u64,
    contract_registry: HashMap<Vec<u8>, ContractState>,
}

/// Stack item in execution context
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StackItem {
    Integer(i64),
    UnsignedInteger(u64),
    ByteArray(Vec<u8>),
    Array(Vec<StackItem>),
    Map(std::collections::HashMap<Vec<u8>, StackItem>),
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

#[derive(Debug, Clone)]
struct TryFrame {
    catch_target: Option<u32>,
    finally_target: Option<u32>,
    pending_error: Option<String>,
    // When true, any ENDFINALLY should rethrow even if inner blocks cleared their own errors.
    must_rethrow: bool,
}

#[derive(Debug, Clone)]
struct IteratorState {
    entries: Vec<StackItem>,
    index: usize,
}

#[derive(Debug, Clone)]
struct ContractState {
    hash: [u8; 20],
    nef: Vec<u8>,
    manifest: Vec<u8>,
    update_counter: u32,
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
            call_stack_limit: config.call_stack_limit,
            stack: Vec::new(),
            locals: Vec::new(),
            args: Vec::new(),
            static_fields: Vec::new(),
            memory: Vec::new(),
            memory_limit: config.memory_limit,
            return_data: Vec::new(),
            logs: Vec::new(),
            call_stack: Vec::new(),
            try_stack: Vec::new(),
            iterators: HashMap::new(),
            next_iterator_id: 1,
            syscall_gas: spec::syscall_gas_table(),
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
            network_magic: config.network_magic,
            pending_caller_account: None,
            pending_block_height: None,
            pending_timestamp: None,
            neo_balances: HashMap::new(),
            gas_balances: HashMap::new(),
            neo_total_supply: 100_000_000,
            gas_total_supply: 30_000_000_000,
            contract_registry: HashMap::new(),
        })
    }

    fn handle_native_transfer(&mut self, is_neo: bool, params: StackItem) -> StackItem {
        if let StackItem::Array(args) = params {
            if args.len() < 3 {
                return StackItem::Boolean(false);
            }
            let from = if let StackItem::ByteArray(acc) = &args[0] {
                acc.clone()
            } else {
                Vec::new()
            };
            let to = if let StackItem::ByteArray(acc) = &args[1] {
                acc.clone()
            } else {
                Vec::new()
            };
            let amount = match &args[2] {
                StackItem::UnsignedInteger(u) => *u,
                StackItem::Integer(i) if *i >= 0 => *i as u64,
                _ => 0,
            };
            if amount == 0 {
                return StackItem::Boolean(false);
            }

            // Read balances
            let mut from_bal = if is_neo {
                *self.neo_balances.get(&from).unwrap_or(&0)
            } else {
                *self.gas_balances.get(&from).unwrap_or(&0)
            };
            let mut to_bal = if is_neo {
                *self.neo_balances.get(&to).unwrap_or(&0)
            } else {
                *self.gas_balances.get(&to).unwrap_or(&0)
            };

            if !from.is_empty() && from_bal < amount {
                return StackItem::Boolean(false);
            }
            if !from.is_empty() {
                from_bal -= amount;
            }
            to_bal = to_bal.saturating_add(amount);

            if is_neo {
                if !from.is_empty() {
                    self.neo_balances.insert(from.clone(), from_bal);
                }
                self.neo_balances.insert(to.clone(), to_bal);
            } else {
                if !from.is_empty() {
                    self.gas_balances.insert(from.clone(), from_bal);
                }
                self.gas_balances.insert(to.clone(), to_bal);
            }

            StackItem::Boolean(true)
        } else {
            StackItem::Boolean(false)
        }
    }

    /// Initialize context for execution
    pub fn initialize(&mut self, bytecode: &[u8], input: &[u8]) -> Result<(), RuntimeError> {
        self.bytecode = bytecode.to_vec();
        self.input_data = input.to_vec();
        self.instruction_pointer = 0;
        self.stack.clear();
        self.locals.clear();
        self.args.clear();
        self.static_fields.clear();
        self.memory.clear();
        self.return_data.clear();
        self.logs.clear();
        self.call_stack.clear();
        self.gas_used = 0;
        self.instruction_count = 0;
        self.storage_overlay.clear();
        self.iterators.clear();
        self.next_iterator_id = 1;
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
        self.try_stack.clear();
        self.iterators.clear();
        self.next_iterator_id = 1;
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

    /// Get gas used so far
    pub fn gas_used(&self) -> u64 {
        self.gas_used
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

    /// Number of contracts currently tracked by the in-memory registry.
    pub fn contract_registry_len(&self) -> usize {
        self.contract_registry.len()
    }

    /// Update counter for a registered contract (if present).
    pub fn contract_update_counter(&self, hash: &[u8]) -> Option<u32> {
        self.contract_registry.get(hash).map(|c| c.update_counter)
    }

    /// Known contract hashes (20-byte each).
    pub fn contract_hashes(&self) -> Vec<[u8; 20]> {
        self.contract_registry
            .keys()
            .filter_map(|k| {
                if k.len() == 20 {
                    let mut arr = [0u8; 20];
                    arr.copy_from_slice(k);
                    Some(arr)
                } else {
                    None
                }
            })
            .collect()
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
        match self.execute_instruction(opcode) {
            Ok(()) => {}
            Err(e) => {
                let msg = e.to_string();
                // Propagate error info through all active try frames, marking frames without catch to rethrow after finally.
                if self.try_stack.is_empty() {
                    return Err(e);
                }

                for frame in self.try_stack.iter_mut() {
                    frame.pending_error = Some(msg.clone());
                    if frame.catch_target.is_none() {
                        frame.must_rethrow = true;
                    }
                }

                // Route to the nearest catch if available, otherwise the topmost finally.
                if let Some(frame) = self.try_stack.iter().rev().find(|f| f.catch_target.is_some()) {
                    self.instruction_pointer = frame.catch_target.unwrap();
                } else if let Some(finally_target) = self.try_stack.last().and_then(|f| f.finally_target) {
                    self.instruction_pointer = finally_target;
                } else {
                    return Err(RuntimeError::ExecutionError { message: msg });
                }
            }
        }

        self.instruction_count += 1;
        self.max_stack_depth = self.max_stack_depth.max(self.stack.len() as u32);
        let halted = self.instruction_pointer as usize >= self.bytecode.len();

        Ok(StepResult {
            instruction_pointer: self.instruction_pointer,
            opcode: opcode_name,
            stack_items: self.stack.clone(),
            gas_used: self.gas_used - old_gas,
            memory_changes: Vec::new(), // Would track actual changes
            halted,
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

    fn get_local(&self, index: usize) -> Result<StackItem, RuntimeError> {
        self.locals
            .get(index)
            .cloned()
            .ok_or(RuntimeError::ExecutionError {
                message: format!("Local index {index} out of bounds"),
            })
    }

    fn set_local(&mut self, index: usize, value: StackItem) -> Result<(), RuntimeError> {
        if let Some(slot) = self.locals.get_mut(index) {
            *slot = value;
            Ok(())
        } else {
            Err(RuntimeError::ExecutionError {
                message: format!("Local index {index} out of bounds"),
            })
        }
    }

    fn get_arg(&self, index: usize) -> Result<StackItem, RuntimeError> {
        self.args
            .get(index)
            .cloned()
            .ok_or(RuntimeError::ExecutionError {
                message: format!("Argument index {index} out of bounds"),
            })
    }

    fn set_arg(&mut self, index: usize, value: StackItem) -> Result<(), RuntimeError> {
        if let Some(slot) = self.args.get_mut(index) {
            *slot = value;
            Ok(())
        } else {
            Err(RuntimeError::ExecutionError {
                message: format!("Argument index {index} out of bounds"),
            })
        }
    }

    fn get_static(&self, index: usize) -> Result<StackItem, RuntimeError> {
        self.static_fields
            .get(index)
            .cloned()
            .ok_or(RuntimeError::ExecutionError {
                message: format!("Static field index {index} out of bounds"),
            })
    }

    fn set_static(&mut self, index: usize, value: StackItem) -> Result<(), RuntimeError> {
        if let Some(slot) = self.static_fields.get_mut(index) {
            *slot = value;
            Ok(())
        } else {
            Err(RuntimeError::ExecutionError {
                message: format!("Static field index {index} out of bounds"),
            })
        }
    }

    fn pop_usize(&mut self, opname: &str) -> Result<usize, RuntimeError> {
        let item = self.pop_stack()?;
        let as_usize = match item {
            StackItem::Integer(i) if i >= 0 => i as usize,
            StackItem::UnsignedInteger(u) => u as usize,
            _ => {
                return Err(RuntimeError::ExecutionError {
                    message: format!("{opname}: expected non-negative integer"),
                })
            }
        };
        Ok(as_usize)
    }

    fn clear_stack(&mut self) {
        self.stack.clear();
    }

    fn nip(&mut self) -> Result<(), RuntimeError> {
        if self.stack.len() < 2 {
            return Err(RuntimeError::ExecutionError {
                message: "NIP: insufficient stack items".to_string(),
            });
        }
        let top = self.pop_stack()?;
        self.pop_stack()?; // drop second
        self.push_stack(top)?;
        Ok(())
    }

    fn xdrop(&mut self) -> Result<(), RuntimeError> {
        let index = self.pop_usize("XDROP")?;
        if index >= self.stack.len() {
            return Err(RuntimeError::ExecutionError {
                message: "XDROP: index out of bounds".to_string(),
            });
        }
        let remove_pos = self.stack.len() - 1 - index;
        self.stack.remove(remove_pos);
        Ok(())
    }

    fn over(&mut self) -> Result<(), RuntimeError> {
        if self.stack.len() < 2 {
            return Err(RuntimeError::ExecutionError {
                message: "OVER: insufficient stack items".to_string(),
            });
        }
        let second = self.stack[self.stack.len().saturating_sub(2)].clone();
        self.push_stack(second)
    }

    fn pick_n(&mut self) -> Result<(), RuntimeError> {
        let index = self.pop_usize("PICK")?;
        if index >= self.stack.len() {
            return Err(RuntimeError::ExecutionError {
                message: "PICK: index out of bounds".to_string(),
            });
        }
        let pos = self.stack.len() - 1 - index;
        let item = self.stack[pos].clone();
        self.push_stack(item)
    }

    fn tuck(&mut self) -> Result<(), RuntimeError> {
        if self.stack.len() < 2 {
            return Err(RuntimeError::ExecutionError {
                message: "TUCK: insufficient stack items".to_string(),
            });
        }
        let top = self.pop_stack()?;
        let second = self.pop_stack()?;
        let duplicate = top.clone();
        self.push_stack(duplicate)?;
        self.push_stack(second)?;
        self.push_stack(top)?;
        Ok(())
    }

    fn roll(&mut self) -> Result<(), RuntimeError> {
        let index = self.pop_usize("ROLL")?;
        if index >= self.stack.len() {
            return Err(RuntimeError::ExecutionError {
                message: "ROLL: index out of bounds".to_string(),
            });
        }
        let pos = self.stack.len() - 1 - index;
        let item = self.stack.remove(pos);
        self.push_stack(item)
    }

    fn reverse_top_n(&mut self, n: usize) -> Result<(), RuntimeError> {
        if n > self.stack.len() {
            return Err(RuntimeError::ExecutionError {
                message: format!("REVERSEN: need {n} items"),
            });
        }
        let split_at = self.stack.len() - n;
        let mut tail = self.stack.split_off(split_at);
        tail.reverse();
        self.stack.extend(tail);
        Ok(())
    }

    fn handle_syscall(&mut self, id: [u8; 4]) -> Result<(), RuntimeError> {
        let name = match spec::SYSCALLS.get(&id) {
            Some(spec) => spec.name,
            None => {
                return Err(RuntimeError::ExecutionError {
                    message: format!(
                        "Unsupported syscall id {:02X?} at ip {}",
                        id, self.instruction_pointer
                    ),
                })
            }
        };

        match name {
            "System.Storage.GetContext" | "System.Storage.GetReadOnlyContext" => {
                self.push_stack(StackItem::ByteArray(Vec::new()))?;
            }
            "System.Storage.Get" => {
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
            "System.Storage.Put" => {
                let value_item = self.pop_stack()?;
                let _context = self.pop_stack()?; // ignored
                let slot_item = self.pop_stack()?;

                let key = Self::stack_item_to_bytes(slot_item);
                let value = Self::stack_item_to_bytes(value_item);

                let entry =
                    self.storage_overlay
                        .entry(key.clone())
                        .or_insert_with(|| OverlayEntry {
                            value: None,
                            dirty: false,
                        });
                entry.value = if value.is_empty() { None } else { Some(value) };
                entry.dirty = true;
            }
            "System.Storage.Delete" => {
                let _context = self.pop_stack()?;
                let slot_item = self.pop_stack()?;
                let key = Self::stack_item_to_bytes(slot_item);
                self.storage_overlay.insert(
                    key,
                    OverlayEntry {
                        value: None,
                        dirty: true,
                    },
                );
            }
            "System.Runtime.GetNetwork" => {
                self.push_stack(StackItem::UnsignedInteger(self.network_magic as u64))?;
            }
            "System.Runtime.Platform" => {
                self.push_stack(StackItem::ByteArray(b"NEO".to_vec()))?;
            }
            "System.Runtime.GetGasLeft" => {
                let remaining = self.gas_limit.saturating_sub(self.gas_used);
                self.push_stack(StackItem::UnsignedInteger(remaining))?;
            }
            "System.Runtime.GetInvocationCounter" => {
                self.invocation_counter += 1;
                self.push_stack(StackItem::UnsignedInteger(self.invocation_counter))?;
            }
            "System.Runtime.CallingScriptHash" => {
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
            "System.Runtime.EntryScriptHash" | "System.Runtime.ExecutingScriptHash" => {
                self.push_stack(StackItem::ByteArray(self.default_account_bytes.clone()))?;
            }
            "System.Runtime.GetTime" | "System.Runtime.GetBlockTime" => {
                let timestamp = *self.timestamp.get_or_insert(self.default_timestamp);
                self.push_stack(StackItem::UnsignedInteger(timestamp))?;
            }
            "System.Blockchain.GetHeight" => {
                let height = *self.block_height.get_or_insert(self.default_block_height);
                self.push_stack(StackItem::UnsignedInteger(height))?;
            }
            "System.Blockchain.GetRandom" | "System.Runtime.GetRandom" => {
                let seed = self.invocation_counter.to_le_bytes().to_vec();
                let hash = Sha256::digest(&seed);
                self.push_stack(StackItem::ByteArray(hash[..].to_vec()))?;
            }
            "System.Runtime.Notify" => {
                let payload = self.pop_stack()?;
                let bytes = Self::stack_item_to_bytes(payload);
                self.logs.push(LogEntry {
                    address: self.default_account.clone(),
                    topics: Vec::new(),
                    data: bytes.clone(),
                });
                self.return_data = bytes;
            }
            "System.Runtime.Log" => {
                let msg = Self::stack_item_to_bytes(self.pop_stack()?);
                self.logs.push(LogEntry {
                    address: self.default_account.clone(),
                    topics: Vec::new(),
                    data: msg,
                });
            }
            "System.Runtime.CheckWitness" => {
                let witness_item = self.pop_stack()?;
                let caller_bytes = self
                    .caller_account
                    .clone()
                    .unwrap_or_else(|| self.default_account_bytes.clone());
                let is_match = match witness_item {
                    StackItem::Array(items) => items.into_iter().any(|w| {
                        let bytes = Self::stack_item_to_bytes(w);
                        bytes == caller_bytes || bytes == self.default_account_bytes
                    }),
                    other => {
                        let bytes = Self::stack_item_to_bytes(other);
                        bytes == caller_bytes || bytes == self.default_account_bytes
                    }
                };
                self.push_stack(StackItem::Boolean(is_match))?;
            }
            "System.Runtime.Serialize" => {
                let item = self.pop_stack()?;
                let bytes = Self::stack_item_to_bytes(item);
                self.push_stack(StackItem::ByteArray(bytes))?;
            }
            "System.Runtime.Deserialize" => {
                let bytes = Self::stack_item_to_bytes(self.pop_stack()?);
                self.push_stack(StackItem::ByteArray(bytes))?;
            }
            "System.Crypto.SHA256" => {
                let data = self.pop_stack()?;
                let bytes = Self::stack_item_to_bytes(data);
                let digest = Sha256::digest(&bytes);
                self.push_stack(StackItem::ByteArray(digest[..].to_vec()))?;
            }
            "System.Crypto.RIPEMD160" => {
                let data = self.pop_stack()?;
                let bytes = Self::stack_item_to_bytes(data);
                let digest = Ripemd160::digest(&bytes);
                self.push_stack(StackItem::ByteArray(digest[..].to_vec()))?;
            }
            "System.Crypto.Keccak256" => {
                let data = self.pop_stack()?;
                let bytes = Self::stack_item_to_bytes(data);
                let digest = Keccak256::digest(&bytes);
                self.push_stack(StackItem::ByteArray(digest[..].to_vec()))?;
            }
            "System.Crypto.Murmur32" => {
                let data = Self::stack_item_to_bytes(self.pop_stack()?);
                let hash = Self::murmur3_32(&data);
                self.push_stack(StackItem::UnsignedInteger(hash as u64))?;
            }
            "System.Crypto.Hash160" => {
                let data = self.pop_stack()?;
                let bytes = Self::stack_item_to_bytes(data);
                let sha = Sha256::digest(&bytes);
                let ripemd = Ripemd160::digest(&sha);
                self.push_stack(StackItem::ByteArray(ripemd[..].to_vec()))?;
            }
            "System.Crypto.Hash256" => {
                let data = self.pop_stack()?;
                let bytes = Self::stack_item_to_bytes(data);
                let first = Sha256::digest(&bytes);
                let second = Sha256::digest(&first);
                self.push_stack(StackItem::ByteArray(second[..].to_vec()))?;
            }
            "System.Crypto.CheckSig" => {
                let sig_item = self.pop_stack()?;
                let pub_item = self.pop_stack()?;
                let pubkey = Self::stack_item_to_bytes(pub_item);
                let sig = Self::stack_item_to_bytes(sig_item);
                let ok = Self::verify_secp256k1(&pubkey, &sig);
                self.push_stack(StackItem::Boolean(ok))?;
            }
            "System.Crypto.CheckMultisig" => {
                let sigs = Self::stack_item_to_bytes(self.pop_stack()?);
                let pubs = Self::stack_item_to_bytes(self.pop_stack()?);
                // Treat as true only if both blobs can be split into at least one valid pair
                let ok = !pubs.is_empty()
                    && !sigs.is_empty()
                    && Self::verify_secp256k1(&pubs, &sigs);
                self.push_stack(StackItem::Boolean(ok))?;
            }
            "System.Storage.Find" => {
                let prefix = Self::stack_item_to_bytes(self.pop_stack()?);
                let _context = self.pop_stack()?;

                let entries = self.build_storage_entries(prefix.clone())?;
                let token = self.allocate_iterator(entries);
                self.push_stack(token)?;
            }
            "System.Iterator.Next" => {
                let token = self.pop_stack()?;
                let mut has_next = false;
                if let Some(id) = Self::iterator_id_from_item(&token) {
                    if let Some(state) = self.iterators.get_mut(&id) {
                        has_next = state.index < state.entries.len();
                        if has_next {
                            state.index += 1;
                        }
                    }
                }
                self.push_stack(token)?;
                self.push_stack(StackItem::Boolean(has_next))?;
            }
            "System.Iterator.Value" => {
                let token = self.pop_stack()?;
                let mut value = StackItem::Null;
                if let Some(id) = Self::iterator_id_from_item(&token) {
                    if let Some(state) = self.iterators.get(&id) {
                        if state.index > 0 {
                            value = state
                                .entries
                                .get(state.index - 1)
                                .cloned()
                                .unwrap_or(StackItem::Null);
                        }
                    }
                }
                self.push_stack(token)?;
                self.push_stack(value)?;
            }
            "System.Iterator.Dispose" => {
                let token = self.pop_stack()?;
                let mut removed = false;
                if let Some(id) = Self::iterator_id_from_item(&token) {
                    removed = self.iterators.remove(&id).is_some();
                }
                self.push_stack(StackItem::Boolean(removed))?;
            }
            "System.Blockchain.GetBlock"
            | "System.Blockchain.GetTransaction"
            | "System.Blockchain.GetTransactionHeight"
            | "System.Blockchain.GetTransactionFromBlock"
            | "System.Blockchain.GetContract"
            | "System.Blockchain.GetCommittee"
            | "System.Blockchain.GetValidators"
            | "System.Blockchain.GetBlockHash" => {
                // Return a default validator/committee set placeholder for now.
                self.push_stack(StackItem::Array(vec![StackItem::ByteArray(
                    self.default_account_bytes.clone(),
                )]))?;
            }
            "System.Contract.Call" | "System.Contract.CallEx" => {
                self.handle_contract_call()?;
            }
            "System.Contract.GetCallFlags" => {
                // Default to All flag
                self.push_stack(StackItem::UnsignedInteger(0xFF))?;
            }
            "System.Contract.CreateStandardAccount" | "System.Contract.CreateMultisigAccount" => {
                // Push a deterministic pseudo-address
                let mut addr = [0u8; 20];
                addr.copy_from_slice(&Sha256::digest(&[self.invocation_counter as u8])[0..20]);
                self.push_stack(StackItem::ByteArray(addr.to_vec()))?;
            }
            "System.ContractManagement.GetContract" => {
                let params = self.pop_stack()?;
                let mut result = StackItem::Null;
                if let StackItem::Array(args) = params {
                    if let Some(StackItem::ByteArray(hash_bytes)) = args.get(0) {
                        if let Some(state) = self.lookup_contract(hash_bytes) {
                            result = self.contract_to_stackitem(&state);
                        }
                    }
                }
                self.push_stack(result)?;
            }
            "System.ContractManagement.Deploy" | "System.ContractManagement.Update" => {
                let params = self.pop_stack()?;
                if let StackItem::Array(args) = params {
                    if args.len() >= 2 {
                        let nef = Self::stack_item_to_bytes(args[0].clone());
                        let manifest = Self::stack_item_to_bytes(args[1].clone());
                        let target_hash = args.get(2).and_then(|h| match h {
                            StackItem::ByteArray(b) => Some(b.clone()),
                            _ => None,
                        });
                        let state = if let Some(hash_bytes) = target_hash {
                            self.update_contract(&hash_bytes, nef.clone(), manifest.clone())
                                .unwrap_or_else(|| self.register_contract(nef.clone(), manifest.clone()))
                        } else if name.ends_with("Deploy") {
                            self.register_contract(nef.clone(), manifest.clone())
                        } else {
                            if let Some(hash) = self.contract_registry.keys().next().cloned() {
                                self.update_contract(&hash, nef.clone(), manifest.clone())
                                    .unwrap_or_else(|| self.register_contract(nef.clone(), manifest.clone()))
                            } else {
                                self.register_contract(nef.clone(), manifest.clone())
                            }
                        };
                        self.push_stack(self.contract_to_stackitem(&state))?;
                    } else {
                        self.push_stack(StackItem::Null)?;
                    }
                } else {
                    self.push_stack(StackItem::Null)?;
                }
            }
            "System.Oracle.Request" => {
                // Consume oracle request params and return a pseudo request id
                let _gas = self.pop_stack()?;
                let _filter = self.pop_stack()?;
                let _callback = self.pop_stack()?;
                let _url = self.pop_stack()?;
                let request_id = Sha256::digest(&self.invocation_counter.to_le_bytes());
                self.push_stack(StackItem::ByteArray(request_id[..4].to_vec()))?;
            }
            "System.Policy.GetFeePerByte"
            | "System.Policy.GetExecFeeFactor"
            | "System.Policy.GetStoragePrice" => {
                self.push_stack(StackItem::UnsignedInteger(0))?;
            }
            _ => {
                // Gracefully handle known-but-unimplemented syscalls with a null result
                self.push_stack(StackItem::Null)?;
            }
        }

        Ok(())
    }

    fn handle_contract_call(&mut self) -> Result<(), RuntimeError> {
        let params = self.pop_stack()?;
        let _flags = self.pop_stack()?; // call flags ignored in emulator
        let method_item = self.pop_stack()?;
        let contract_item = self.pop_stack()?;

        let method = String::from_utf8(Self::stack_item_to_bytes(method_item)).unwrap_or_default();
        let contract_bytes = Self::stack_item_to_bytes(contract_item);
        let mut hash = [0u8; 20];
        for (i, b) in contract_bytes.iter().take(20).enumerate() {
            hash[i] = *b;
        }

        let result = self.invoke_native_contract(&hash, &method, params);
        self.push_stack(result)?;
        Ok(())
    }

    fn invoke_native_contract(
        &mut self,
        hash: &[u8; 20],
        method: &str,
        params: StackItem,
    ) -> StackItem {
        let method_lower = method.to_ascii_lowercase();
        if let Some(name) = spec::native_contract_name(hash) {
            match name {
                "NEO" => match method_lower.as_str() {
                    "symbol" => StackItem::ByteArray(b"NEO".to_vec()),
                    "decimals" => StackItem::UnsignedInteger(0),
                    "totalsupply" => StackItem::UnsignedInteger(self.neo_total_supply),
                    "balanceof" => {
                        if let StackItem::Array(args) = params {
                            if let Some(StackItem::ByteArray(acc)) = args.get(0) {
                                let bal = *self.neo_balances.get(acc).unwrap_or(&0);
                                StackItem::UnsignedInteger(bal)
                            } else {
                                StackItem::UnsignedInteger(0)
                            }
                        } else {
                            StackItem::UnsignedInteger(0)
                        }
                    }
                    "transfer" => {
                        self.handle_native_transfer(true, params)
                    }
                    _ => StackItem::Null,
                },
                "GAS" => match method_lower.as_str() {
                    "symbol" => StackItem::ByteArray(b"GAS".to_vec()),
                    "decimals" => StackItem::UnsignedInteger(8),
                    "totalsupply" => StackItem::UnsignedInteger(self.gas_total_supply),
                    "balanceof" => {
                        if let StackItem::Array(args) = params {
                            if let Some(StackItem::ByteArray(acc)) = args.get(0) {
                                let bal = *self.gas_balances.get(acc).unwrap_or(&0);
                                StackItem::UnsignedInteger(bal)
                            } else {
                                StackItem::UnsignedInteger(0)
                            }
                        } else {
                            StackItem::UnsignedInteger(0)
                        }
                    }
                    "transfer" => {
                        self.handle_native_transfer(false, params)
                    }
                    _ => StackItem::Null,
                },
                "Policy" => match method_lower.as_str() {
                    "getfeeperbyte" | "getexecfeefactor" | "getstorageprice" => {
                        StackItem::UnsignedInteger(0)
                    }
                    _ => StackItem::Null,
                },
                "Oracle" => {
                    if method_lower == "request" {
                        // Return pseudo request id
                        let req = Sha256::digest(&self.invocation_counter.to_le_bytes());
                        StackItem::ByteArray(req[..4].to_vec())
                    } else {
                        StackItem::Null
                    }
                }
                "ContractManagement" => match method_lower.as_str() {
                    "getcontract" => {
                        if let StackItem::Array(args) = params {
                            if let Some(StackItem::ByteArray(hash_bytes)) = args.get(0) {
                                if let Some(state) = self.lookup_contract(hash_bytes) {
                                    return self.contract_to_stackitem(&state);
                                }
                            }
                        }
                        StackItem::Null
                    }
                    "deploy" => {
                        if let StackItem::Array(args) = params {
                            if args.len() >= 2 {
                                let nef = Self::stack_item_to_bytes(args[0].clone());
                                let manifest = Self::stack_item_to_bytes(args[1].clone());
                                let state = self.register_contract(nef, manifest);
                                return self.contract_to_stackitem(&state);
                            }
                        }
                        StackItem::Null
                    }
                    "update" => {
                        if let StackItem::Array(args) = params {
                            if args.len() >= 2 {
                                let nef = Self::stack_item_to_bytes(args[0].clone());
                                let manifest = Self::stack_item_to_bytes(args[1].clone());
                                if let Some(hash) = self.contract_registry.keys().next().cloned() {
                                    if let Some(state) =
                                        self.update_contract(&hash, nef.clone(), manifest.clone())
                                    {
                                        return self.contract_to_stackitem(&state);
                                    }
                                }
                                let state = self.register_contract(nef, manifest);
                                return self.contract_to_stackitem(&state);
                            }
                        }
                        StackItem::Null
                    }
                    _ => StackItem::Null,
                },
                "RoleManagement" => match method_lower.as_str() {
                    "designateasrole" | "isdesignated" => StackItem::Boolean(true),
                    _ => StackItem::Null,
                },
                _ => StackItem::Null,
            }
        } else {
            // Unknown contract – return null to avoid halting.
            let _ = params; // silence unused warning for now
            StackItem::Null
        }
    }

    fn pack_items(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("PACK")?;
        if count > self.stack.len() {
            return Err(RuntimeError::ExecutionError {
                message: "PACK: insufficient stack items".to_string(),
            });
        }
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(self.pop_stack()?);
        }
        items.reverse();
        self.push_stack(StackItem::Array(items))
    }

    fn pack_map(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("PACKMAP")?;
        if count.saturating_mul(2) > self.stack.len() {
            return Err(RuntimeError::ExecutionError {
                message: "PACKMAP: insufficient stack items".to_string(),
            });
        }
        let mut map = std::collections::HashMap::new();
        for _ in 0..count {
            let value = self.pop_stack()?;
            let key = self.pop_stack()?;
            let key_bytes = Self::stack_item_to_bytes(key);
            map.insert(key_bytes, value);
        }
        self.push_stack(StackItem::Map(map))
    }

    fn convert_item(&self, item: StackItem, target: StackItem) -> Result<StackItem, RuntimeError> {
        let target_code = Self::stack_item_type_code(&target);

        match target_code {
            0x00 => Ok(item), // Any/no-op
            0x20 => Ok(StackItem::Boolean(item.is_truthy())),
            0x21 | 0x22 => {
                let bytes = Self::stack_item_to_bytes(item);
                let mut buf = [0u8; 8];
                for (i, b) in bytes.iter().take(8).enumerate() {
                    buf[i] = *b;
                }
                Ok(StackItem::Integer(i64::from_le_bytes(buf)))
            }
            0x28 | 0x30 => Ok(StackItem::ByteArray(Self::stack_item_to_bytes(item))),
            0x40 | 0x41 => match item {
                StackItem::Array(items) => Ok(StackItem::Array(items)),
                StackItem::Map(map) => Ok(StackItem::Array(
                    map.into_iter()
                        .map(|(k, v)| StackItem::Array(vec![StackItem::ByteArray(k), v]))
                        .collect(),
                )),
                other => Ok(StackItem::Array(vec![other])),
            },
            0x48 => match item {
                StackItem::Map(map) => Ok(StackItem::Map(map)),
                StackItem::Array(items) => {
                    let mut map = std::collections::HashMap::new();
                    for pair in items {
                        if let StackItem::Array(mut kv) = pair {
                            if kv.len() >= 2 {
                                let value = kv.pop().unwrap();
                                let key = kv.remove(0);
                                map.insert(Self::stack_item_to_bytes(key), value);
                            }
                        }
                    }
                    Ok(StackItem::Map(map))
                }
                other => {
                    let mut map = std::collections::HashMap::new();
                    map.insert(Vec::new(), other);
                    Ok(StackItem::Map(map))
                }
            },
            0x80 => Ok(item), // iterator tokens already byte arrays; leave untouched
            _ => Ok(item),
        }
    }

    fn stack_item_type_code(marker: &StackItem) -> u8 {
        match marker {
            StackItem::Integer(i) if *i >= 0 && *i <= u8::MAX as i64 => *i as u8,
            StackItem::UnsignedInteger(u) if *u <= u8::MAX as u64 => *u as u8,
            StackItem::ByteArray(bytes) => *bytes.last().unwrap_or(&0u8),
            StackItem::Boolean(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn is_iterator_token(&self, item: &StackItem) -> bool {
        if let Some(id) = Self::iterator_id_from_item(item) {
            return self.iterators.contains_key(&id);
        }
        false
    }

    fn pick_item(&mut self) -> Result<(), RuntimeError> {
        let key = self.pop_stack()?;
        let collection = self.pop_stack()?;
        let value = Self::pick_from_collection(collection, key)?;
        self.push_stack(value)
    }

    fn set_item(&mut self) -> Result<(), RuntimeError> {
        let value = self.pop_stack()?;
        let key = self.pop_stack()?;
        let collection = self.pop_stack()?;
        let updated = Self::set_in_collection(collection, key, value)?;
        self.push_stack(updated)
    }

    fn pick_from_collection(
        collection: StackItem,
        key: StackItem,
    ) -> Result<StackItem, RuntimeError> {
        match collection {
            StackItem::Array(items) => {
                let index = match key {
                    StackItem::Integer(i) if i >= 0 => i as usize,
                    StackItem::UnsignedInteger(u) => u as usize,
                    _ => {
                        return Err(RuntimeError::ExecutionError {
                            message: "PICKITEM: array index must be non-negative integer"
                                .to_string(),
                        })
                    }
                };
                items
                    .get(index)
                    .cloned()
                    .ok_or_else(|| RuntimeError::ExecutionError {
                        message: "PICKITEM: index out of bounds".to_string(),
                    })
            }
            StackItem::ByteArray(bytes) => {
                let index = match key {
                    StackItem::Integer(i) if i >= 0 => i as usize,
                    StackItem::UnsignedInteger(u) => u as usize,
                    _ => {
                        return Err(RuntimeError::ExecutionError {
                            message: "PICKITEM: byte index must be non-negative integer"
                                .to_string(),
                        })
                    }
                };
                bytes
                    .get(index)
                    .map(|b| StackItem::ByteArray(vec![*b]))
                    .ok_or_else(|| RuntimeError::ExecutionError {
                        message: "PICKITEM: index out of bounds".to_string(),
                    })
            }
            StackItem::Map(map) => {
                let key_bytes = Self::stack_item_to_bytes(key);
                map.get(&key_bytes)
                    .cloned()
                    .ok_or_else(|| RuntimeError::ExecutionError {
                        message: "PICKITEM: key not found".to_string(),
                    })
            }
            other => Err(RuntimeError::ExecutionError {
                message: format!("PICKITEM: unsupported target {:?}", other),
            }),
        }
    }

    fn set_in_collection(
        collection: StackItem,
        key: StackItem,
        value: StackItem,
    ) -> Result<StackItem, RuntimeError> {
        match collection {
            StackItem::Array(mut items) => {
                let index = match key {
                    StackItem::Integer(i) if i >= 0 => i as usize,
                    StackItem::UnsignedInteger(u) => u as usize,
                    _ => {
                        return Err(RuntimeError::ExecutionError {
                            message: "SETITEM: array index must be non-negative integer"
                                .to_string(),
                        })
                    }
                };
                if index >= items.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "SETITEM: index out of bounds".to_string(),
                    });
                }
                items[index] = value;
                Ok(StackItem::Array(items))
            }
            StackItem::ByteArray(mut bytes) => {
                let index = match key {
                    StackItem::Integer(i) if i >= 0 => i as usize,
                    StackItem::UnsignedInteger(u) => u as usize,
                    _ => {
                        return Err(RuntimeError::ExecutionError {
                            message: "SETITEM: byte index must be non-negative integer".to_string(),
                        })
                    }
                };
                if index >= bytes.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "SETITEM: index out of bounds".to_string(),
                    });
                }
                let as_byte = match value {
                    StackItem::ByteArray(ref v) if v.len() == 1 => v[0],
                    StackItem::Integer(i) if i >= 0 && i < 256 => i as u8,
                    StackItem::UnsignedInteger(u) if u < 256 => u as u8,
                    _ => {
                        return Err(RuntimeError::ExecutionError {
                            message: "SETITEM: value not representable as byte".to_string(),
                        })
                    }
                };
                bytes[index] = as_byte;
                Ok(StackItem::ByteArray(bytes))
            }
            StackItem::Map(mut map) => {
                let key_bytes = Self::stack_item_to_bytes(key);
                map.insert(key_bytes, value);
                Ok(StackItem::Map(map))
            }
            other => Err(RuntimeError::ExecutionError {
                message: format!("SETITEM: unsupported target {:?}", other),
            }),
        }
    }

    fn index_from_key(&self, key: &StackItem, opname: &str) -> Result<usize, RuntimeError> {
        match key {
            StackItem::Integer(i) if *i >= 0 => Ok(*i as usize),
            StackItem::UnsignedInteger(u) => Ok(*u as usize),
            _ => Err(RuntimeError::ExecutionError {
                message: format!("{opname}: index must be non-negative integer"),
            }),
        }
    }

    fn new_array0(&mut self) -> Result<(), RuntimeError> {
        self.push_stack(StackItem::Array(Vec::new()))
    }

    fn new_struct0(&mut self) -> Result<(), RuntimeError> {
        self.push_stack(StackItem::Array(Vec::new()))
    }

    fn new_struct(&mut self) -> Result<(), RuntimeError> {
        self.new_array()
    }

    fn append_item(&mut self) -> Result<(), RuntimeError> {
        let value = self.pop_stack()?;
        let collection = self.pop_stack()?;
        match collection {
            StackItem::Array(mut items) => {
                items.push(value);
                self.push_stack(StackItem::Array(items))
            }
            StackItem::ByteArray(mut bytes) => {
                let mut append = Self::stack_item_to_bytes(value);
                bytes.append(&mut append);
                self.push_stack(StackItem::ByteArray(bytes))
            }
            other => Err(RuntimeError::ExecutionError {
                message: format!("APPEND: unsupported target {:?}", other),
            }),
        }
    }

    fn remove_item(&mut self) -> Result<(), RuntimeError> {
        let key = self.pop_stack()?;
        let collection = self.pop_stack()?;
        let updated = match collection {
            StackItem::Array(mut items) => {
                let index = self.index_from_key(&key, "REMOVE")?;
                if index >= items.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "REMOVE: index out of bounds".to_string(),
                    });
                }
                items.remove(index);
                StackItem::Array(items)
            }
            StackItem::ByteArray(mut bytes) => {
                let index = self.index_from_key(&key, "REMOVE")?;
                if index >= bytes.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "REMOVE: index out of bounds".to_string(),
                    });
                }
                bytes.remove(index);
                StackItem::ByteArray(bytes)
            }
            StackItem::Map(mut map) => {
                let key_bytes = Self::stack_item_to_bytes(key);
                map.remove(&key_bytes);
                StackItem::Map(map)
            }
            other => {
                return Err(RuntimeError::ExecutionError {
                    message: format!("REMOVE: unsupported target {:?}", other),
                })
            }
        };
        self.push_stack(updated)
    }

    fn clear_items(&mut self) -> Result<(), RuntimeError> {
        let collection = self.pop_stack()?;
        let cleared = match collection {
            StackItem::Array(_) => StackItem::Array(Vec::new()),
            StackItem::ByteArray(_) => StackItem::ByteArray(Vec::new()),
            StackItem::Map(_) => StackItem::Map(std::collections::HashMap::new()),
            other => {
                return Err(RuntimeError::ExecutionError {
                    message: format!("CLEARITEMS: unsupported target {:?}", other),
                })
            }
        };
        self.push_stack(cleared)
    }

    fn pop_item_from_collection(&mut self) -> Result<(), RuntimeError> {
        let collection = self.pop_stack()?;
        match collection {
            StackItem::Array(mut items) => {
                let value = items.pop().ok_or(RuntimeError::ExecutionError {
                    message: "POPITEM: array is empty".to_string(),
                })?;
                self.push_stack(value)
            }
            StackItem::ByteArray(mut bytes) => {
                let value = bytes.pop().ok_or(RuntimeError::ExecutionError {
                    message: "POPITEM: buffer is empty".to_string(),
                })?;
                self.push_stack(StackItem::ByteArray(vec![value]))
            }
            StackItem::Map(_) => Err(RuntimeError::ExecutionError {
                message: "POPITEM: unsupported for maps".to_string(),
            }),
            other => Err(RuntimeError::ExecutionError {
                message: format!("POPITEM: unsupported target {:?}", other),
            }),
        }
    }

    fn reverse_items(&mut self) -> Result<(), RuntimeError> {
        let collection = self.pop_stack()?;
        match collection {
            StackItem::Array(mut items) => {
                items.reverse();
                self.push_stack(StackItem::Array(items))
            }
            StackItem::ByteArray(mut bytes) => {
                bytes.reverse();
                self.push_stack(StackItem::ByteArray(bytes))
            }
            other => Err(RuntimeError::ExecutionError {
                message: format!("REVERSEITEMS: unsupported target {:?}", other),
            }),
        }
    }

    fn new_array(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("NEWARRAY")?;
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(StackItem::Null);
        }
        self.push_stack(StackItem::Array(items))
    }

    fn new_map(&mut self) -> Result<(), RuntimeError> {
        self.push_stack(StackItem::Map(std::collections::HashMap::new()))
    }

    fn new_buffer(&mut self) -> Result<(), RuntimeError> {
        let len = self.pop_usize("NEWBUFFER")?;
        self.push_stack(StackItem::ByteArray(vec![0u8; len]))
    }

    fn memcpy_bytes(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("MEMCPY count")?;
        let src_offset = self.pop_usize("MEMCPY src_offset")?;
        let src = Self::stack_item_to_bytes(self.pop_stack()?);
        let dst_offset = self.pop_usize("MEMCPY dst_offset")?;
        let dst_item = self.pop_stack()?;

        match dst_item {
            StackItem::ByteArray(mut dst) => {
                let src_end = src_offset
                    .checked_add(count)
                    .ok_or(RuntimeError::ExecutionError {
                        message: "MEMCPY: source range overflow".to_string(),
                    })?;
                let dst_end = dst_offset
                    .checked_add(count)
                    .ok_or(RuntimeError::ExecutionError {
                        message: "MEMCPY: destination range overflow".to_string(),
                    })?;

                if src_end > src.len() || dst_end > dst.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "MEMCPY: range out of bounds".to_string(),
                    });
                }

                let src_slice = &src[src_offset..src_end];
                dst[dst_offset..dst_end].copy_from_slice(src_slice);
                self.push_stack(StackItem::ByteArray(dst))
            }
            other => Err(RuntimeError::ExecutionError {
                message: format!("MEMCPY: unsupported destination {:?}", other),
            }),
        }
    }

    fn size_of(&mut self) -> Result<(), RuntimeError> {
        let item = self.pop_stack()?;
        let size = match item {
            StackItem::Array(items) => items.len(),
            StackItem::Map(map) => map.len(),
            StackItem::ByteArray(bytes) => bytes.len(),
            StackItem::Integer(_)
            | StackItem::UnsignedInteger(_)
            | StackItem::Boolean(_)
            | StackItem::Null => {
                return Err(RuntimeError::ExecutionError {
                    message: "SIZE: unsupported type".to_string(),
                })
            }
        };
        self.push_stack(StackItem::Integer(size as i64))
    }

    fn concat_bytes(&mut self) -> Result<(), RuntimeError> {
        let b = Self::stack_item_to_bytes(self.pop_stack()?);
        let mut a = Self::stack_item_to_bytes(self.pop_stack()?);
        a.extend_from_slice(&b);
        self.push_stack(StackItem::ByteArray(a))
    }

    fn substr_bytes(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("SUBSTR")?;
        let index = self.pop_usize("SUBSTR")?;
        let data = Self::stack_item_to_bytes(self.pop_stack()?);
        if index + count > data.len() {
            return Err(RuntimeError::ExecutionError {
                message: "SUBSTR: out of bounds".to_string(),
            });
        }
        self.push_stack(StackItem::ByteArray(data[index..index + count].to_vec()))
    }

    fn left_bytes(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("LEFT")?;
        let data = Self::stack_item_to_bytes(self.pop_stack()?);
        if count > data.len() {
            return Err(RuntimeError::ExecutionError {
                message: "LEFT: out of bounds".to_string(),
            });
        }
        self.push_stack(StackItem::ByteArray(data[..count].to_vec()))
    }

    fn logical_not(&mut self) -> Result<(), RuntimeError> {
        let value = self.pop_stack()?;
        self.push_stack(StackItem::Boolean(!value.is_truthy()))
    }

    fn logical_and(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop_stack()?;
        let a = self.pop_stack()?;
        self.push_stack(StackItem::Boolean(a.is_truthy() && b.is_truthy()))
    }

    fn logical_or(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop_stack()?;
        let a = self.pop_stack()?;
        self.push_stack(StackItem::Boolean(a.is_truthy() || b.is_truthy()))
    }

    fn right_bytes(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("RIGHT")?;
        let data = Self::stack_item_to_bytes(self.pop_stack()?);
        if count > data.len() {
            return Err(RuntimeError::ExecutionError {
                message: "RIGHT: out of bounds".to_string(),
            });
        }
        self.push_stack(StackItem::ByteArray(data[data.len() - count..].to_vec()))
    }

    fn haskey(&mut self) -> Result<(), RuntimeError> {
        let key = self.pop_stack()?;
        let collection = self.pop_stack()?;
        let exists = match collection {
            StackItem::Array(items) => match key {
                StackItem::Integer(i) if i >= 0 => (i as usize) < items.len(),
                StackItem::UnsignedInteger(u) => (u as usize) < items.len(),
                _ => false,
            },
            StackItem::Map(map) => {
                let key_bytes = Self::stack_item_to_bytes(key);
                map.contains_key(&key_bytes)
            }
            StackItem::ByteArray(bytes) => match key {
                StackItem::Integer(i) if i >= 0 => (i as usize) < bytes.len(),
                StackItem::UnsignedInteger(u) => (u as usize) < bytes.len(),
                _ => false,
            },
            _ => false,
        };
        self.push_stack(StackItem::Boolean(exists))
    }

    fn keys(&mut self) -> Result<(), RuntimeError> {
        let collection = self.pop_stack()?;
        let keys = match collection {
            StackItem::Map(map) => StackItem::Array(
                map.keys()
                    .cloned()
                    .map(StackItem::ByteArray)
                    .collect::<Vec<_>>(),
            ),
            _ => {
                return Err(RuntimeError::ExecutionError {
                    message: "KEYS: only supported for maps".to_string(),
                })
            }
        };
        self.push_stack(keys)
    }

    fn values(&mut self) -> Result<(), RuntimeError> {
        let collection = self.pop_stack()?;
        let values = match collection {
            StackItem::Map(map) => StackItem::Array(map.values().cloned().collect()),
            StackItem::Array(items) => StackItem::Array(items),
            StackItem::ByteArray(bytes) => StackItem::Array(
                bytes
                    .iter()
                    .map(|b| StackItem::ByteArray(vec![*b]))
                    .collect(),
            ),
            _ => {
                return Err(RuntimeError::ExecutionError {
                    message: "VALUES: unsupported target".to_string(),
                })
            }
        };
        self.push_stack(values)
    }

    fn unpack(&mut self) -> Result<(), RuntimeError> {
        let collection = self.pop_stack()?;
        let items = match collection {
            StackItem::Array(items) => items,
            StackItem::ByteArray(bytes) => bytes
                .iter()
                .map(|b| StackItem::ByteArray(vec![*b]))
                .collect(),
            StackItem::Map(map) => map.values().cloned().collect(),
            other => {
                return Err(RuntimeError::ExecutionError {
                    message: format!("UNPACK: unsupported target {:?}", other),
                })
            }
        };
        let count = items.len();
        for item in items.into_iter().rev() {
            self.push_stack(item)?;
        }
        self.push_stack(StackItem::Integer(count as i64))
    }

    fn read_u32_offset(&self, opcode_label: &str) -> Result<u32, RuntimeError> {
        let start = self.instruction_pointer as usize + 1;
        let end = start + 4;
        if end > self.bytecode.len() {
            return Err(RuntimeError::ExecutionError {
                message: format!("{opcode_label}: insufficient bytecode for offset"),
            });
        }

        let mut buf = [0u8; 4];
        buf.copy_from_slice(&self.bytecode[start..end]);
        Ok(u32::from_le_bytes(buf))
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

        if address > self.memory.len() {
            return Err(RuntimeError::ExecutionError {
                message: "Memory access exceeds allocated size".to_string(),
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

    /// Access logs
    pub fn logs(&self) -> &[LogEntry] {
        &self.logs
    }

    /// Call function
    pub fn call_function(
        &mut self,
        address: u32,
        function_name: Option<String>,
    ) -> Result<(), RuntimeError> {
        if self.call_stack.len() as u32 >= self.call_stack_limit {
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
        let projected_gas = self.gas_used.saturating_add(gas_cost);
        if projected_gas > self.gas_limit {
            return Err(RuntimeError::OutOfGas {
                used: projected_gas,
                limit: self.gas_limit,
            });
        }

        // Complete NeoVM instruction execution
        match opcode {
            // Push operations (0x00-0x4F)
            0x05 => {
                // PUSHINT256
                if self.instruction_pointer + 32 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT256: insufficient bytecode".to_string(),
                    });
                }
                let start = self.instruction_pointer as usize + 1;
                let end = start + 32;
                let value = self.bytecode[start..end].to_vec();
                self.push_stack(StackItem::ByteArray(value))?;
                self.instruction_pointer += 33;
            }
            0x04 => {
                // PUSHINT128
                if self.instruction_pointer + 16 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT128: insufficient bytecode".to_string(),
                    });
                }
                let start = self.instruction_pointer as usize + 1;
                let end = start + 16;
                let value = self.bytecode[start..end].to_vec();
                self.push_stack(StackItem::ByteArray(value))?;
                self.instruction_pointer += 17;
            }
            0x08 => {
                // PUSHT
                self.push_stack(StackItem::Boolean(true))?;
                self.instruction_pointer += 1;
            }
            0x09 => {
                // PUSHF
                self.push_stack(StackItem::Boolean(false))?;
                self.instruction_pointer += 1;
            }
            0x0F => {
                // PUSHM1
                self.push_stack(StackItem::Integer(-1))?;
                self.instruction_pointer += 1;
            }
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
            0x0A => {
                // PUSHA (absolute address)
                let target = self.read_u32_offset("PUSHA")?;
                self.push_stack(StackItem::UnsignedInteger(target as u64))?;
                self.instruction_pointer += 5;
            }
            0x0B => {
                // PUSHNULL
                self.push_stack(StackItem::Null)?;
                self.instruction_pointer += 1;
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
            0x0D => {
                // PUSHDATA2
                if self.instruction_pointer + 2 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHDATA2: insufficient bytecode for length".to_string(),
                    });
                }
                let len_bytes = &self.bytecode
                    [self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 3];
                let length = u16::from_le_bytes([len_bytes[0], len_bytes[1]]) as usize;
                if self.instruction_pointer as usize + 3 + length > self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHDATA2: insufficient bytecode for data".to_string(),
                    });
                }
                let data = self.bytecode[self.instruction_pointer as usize + 3
                    ..self.instruction_pointer as usize + 3 + length]
                    .to_vec();
                self.push_stack(StackItem::ByteArray(data))?;
                self.instruction_pointer += 3 + length as u32;
            }
            0x0E => {
                // PUSHDATA4
                if self.instruction_pointer + 4 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHDATA4: insufficient bytecode for length".to_string(),
                    });
                }
                let len_bytes = &self.bytecode
                    [self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 5];
                let length =
                    u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]])
                        as usize;
                if self.instruction_pointer as usize + 5 + length > self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHDATA4: insufficient bytecode for data".to_string(),
                    });
                }
                let data = self.bytecode[self.instruction_pointer as usize + 5
                    ..self.instruction_pointer as usize + 5 + length]
                    .to_vec();
                self.push_stack(StackItem::ByteArray(data))?;
                self.instruction_pointer += 5 + length as u32;
            }
            0x10 => {
                // PUSH0
                self.push_stack(StackItem::Integer(0))?;
                self.instruction_pointer += 1;
            }
            0x11..=0x20 => {
                // PUSH1-PUSH16
                let value = (opcode - 0x10) as i64;
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
                let target = self.read_u32_offset("JMP")?;
                if target as usize >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "JMP: jump target out of bounds".to_string(),
                    });
                }
                self.instruction_pointer = target;
            }
            0x23 => {
                // JMP_L uses same offset width in this emulator
                let target = self.read_u32_offset("JMP_L")?;
                if target as usize >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "JMP_L: jump target out of bounds".to_string(),
                    });
                }
                self.instruction_pointer = target;
            }
            0x24 => {
                // JMPIF
                let condition = self.pop_stack()?;
                let target = self.read_u32_offset("JMPIF")?;
                if condition.is_truthy() {
                    if target as usize >= self.bytecode.len() {
                        return Err(RuntimeError::ExecutionError {
                            message: "JMPIF: jump target out of bounds".to_string(),
                        });
                    }
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x25 => {
                // JMPIF_L
                let condition = self.pop_stack()?;
                let target = self.read_u32_offset("JMPIF_L")?;
                if condition.is_truthy() {
                    if target as usize >= self.bytecode.len() {
                        return Err(RuntimeError::ExecutionError {
                            message: "JMPIF_L: jump target out of bounds".to_string(),
                        });
                    }
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x26 => {
                // JMPIFNOT
                let condition = self.pop_stack()?;
                let target = self.read_u32_offset("JMPIFNOT")?;
                if !condition.is_truthy() {
                    if target as usize >= self.bytecode.len() {
                        return Err(RuntimeError::ExecutionError {
                            message: "JMPIFNOT: jump target out of bounds".to_string(),
                        });
                    }
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x27 => {
                // JMPIFNOT_L
                let condition = self.pop_stack()?;
                let target = self.read_u32_offset("JMPIFNOT_L")?;
                if !condition.is_truthy() {
                    if target as usize >= self.bytecode.len() {
                        return Err(RuntimeError::ExecutionError {
                            message: "JMPIFNOT_L: jump target out of bounds".to_string(),
                        });
                    }
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x28 => {
                // JMPEQ
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let target = self.read_u32_offset("JMPEQ")?;
                let is_eq = self.stack_items_equal(&a, &b)?;
                if is_eq {
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x29 => {
                // JMPEQ_L
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let target = self.read_u32_offset("JMPEQ_L")?;
                let is_eq = self.stack_items_equal(&a, &b)?;
                if is_eq {
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x2A => {
                // JMPNE
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let target = self.read_u32_offset("JMPNE")?;
                let is_eq = self.stack_items_equal(&a, &b)?;
                if !is_eq {
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x2B => {
                // JMPNE_L
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let target = self.read_u32_offset("JMPNE_L")?;
                let is_eq = self.stack_items_equal(&a, &b)?;
                if !is_eq {
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x2C => {
                // JMPGT
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let target = self.read_u32_offset("JMPGT")?;
                if self.greater_than(&a, &b)? {
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x2D => {
                // JMPGT_L
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let target = self.read_u32_offset("JMPGT_L")?;
                if self.greater_than(&a, &b)? {
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x2E => {
                // JMPGE
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let target = self.read_u32_offset("JMPGE")?;
                let ge = self.greater_than(&a, &b)? || self.stack_items_equal(&a, &b)?;
                if ge {
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x2F => {
                // JMPGE_L
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let target = self.read_u32_offset("JMPGE_L")?;
                let ge = self.greater_than(&a, &b)? || self.stack_items_equal(&a, &b)?;
                if ge {
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x30 => {
                // JMPLT
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let target = self.read_u32_offset("JMPLT")?;
                if self.less_than(&a, &b)? {
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x31 => {
                // JMPLT_L
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let target = self.read_u32_offset("JMPLT_L")?;
                if self.less_than(&a, &b)? {
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x32 => {
                // JMPLE
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let target = self.read_u32_offset("JMPLE")?;
                let le = self.less_than(&a, &b)? || self.stack_items_equal(&a, &b)?;
                if le {
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x33 => {
                // JMPLE_L
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let target = self.read_u32_offset("JMPLE_L")?;
                let le = self.less_than(&a, &b)? || self.stack_items_equal(&a, &b)?;
                if le {
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer = self.instruction_pointer.saturating_add(5);
                }
            }
            0x34 | 0x35 => {
                // CALL / CALL_L
                let target = self.read_u32_offset("CALL")?;
                if target as usize >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "CALL: target out of bounds".to_string(),
                    });
                }
                let frame = CallFrame {
                    return_address: self.instruction_pointer + 5,
                    function_name: None,
                    local_variables: HashMap::new(),
                    stack_base: self.stack.len(),
                };
                self.call_stack.push(frame);
                self.instruction_pointer = target;
            }
            0x36 | 0x37 => {
                // CALLA / CALLT – treated as absolute CALL for now
                let target = self.read_u32_offset("CALLA/CALLT")?;
                if target as usize >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "CALLA/CALLT: target out of bounds".to_string(),
                    });
                }
                let frame = CallFrame {
                    return_address: self.instruction_pointer + 5,
                    function_name: None,
                    local_variables: HashMap::new(),
                    stack_base: self.stack.len(),
                };
                self.call_stack.push(frame);
                self.instruction_pointer = target;
            }
            0x38 => {
                // ABORT
                return Err(RuntimeError::ExecutionError {
                    message: "ABORT instruction executed".to_string(),
                });
            }
            0x39 => {
                // ASSERT
                let condition = self.pop_stack()?;
                if condition.is_truthy() {
                    self.instruction_pointer += 1;
                } else {
                    return Err(RuntimeError::ExecutionError {
                        message: "ASSERT failed".to_string(),
                    });
                }
            }
            0x3A => {
                // THROW
                return Err(RuntimeError::ExecutionError {
                    message: "THROW instruction executed".to_string(),
                });
            }
            0x3B | 0x3C => {
                // TRY / TRY_L – read catch/finally absolute targets (4-byte each)
                let start = self.instruction_pointer as usize + 1;
                let end = start + 8;
                if end > self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "TRY: insufficient bytecode for offsets".to_string(),
                    });
                }
                let mut catch_buf = [0u8; 4];
                catch_buf.copy_from_slice(&self.bytecode[start..start + 4]);
                let mut finally_buf = [0u8; 4];
                finally_buf.copy_from_slice(&self.bytecode[start + 4..end]);
                let catch_target = u32::from_le_bytes(catch_buf);
                let finally_target = u32::from_le_bytes(finally_buf);
                self.try_stack.push(TryFrame {
                    catch_target: if catch_target == 0 { None } else { Some(catch_target) },
                    finally_target: if finally_target == 0 {
                        None
                    } else {
                        Some(finally_target)
                    },
                    pending_error: None,
                    must_rethrow: false,
                });
                self.instruction_pointer = self.instruction_pointer.saturating_add(9);
            }
            0x3D | 0x3E => {
                // ENDTRY / ENDTRY_L – jump to finally if present, otherwise pop frame
                if let Some(mut frame) = self.try_stack.pop() {
                    // Catch handled, clear pending error only if a catch block existed
                    if frame.catch_target.is_some() {
                        frame.pending_error = None;
                    }
                    if let Some(finally_target) = frame.finally_target.take() {
                        self.try_stack.push(frame);
                        self.instruction_pointer = finally_target;
                    } else {
                        self.instruction_pointer += 1;
                    }
                } else {
                    self.instruction_pointer += 1;
                }
            }
            0x3F => {
                // ENDFINALLY – pop frame, rethrow if an error was pending or marked to rethrow
                if let Some(frame) = self.try_stack.pop() {
                    if frame.must_rethrow || frame.pending_error.is_some() {
                        let msg = frame
                            .pending_error
                            .unwrap_or_else(|| "Unhandled exception".to_string());
                        return Err(RuntimeError::ExecutionError { message: msg });
                    }
                }
                self.instruction_pointer += 1;
            }

            // Stack operations (0x39-0x4F)
            0x43 => {
                // DEPTH
                self.push_stack(StackItem::Integer(self.stack.len() as i64))?;
                self.instruction_pointer += 1;
            }
            0x45 => {
                // DROP
                self.pop_stack()?;
                self.instruction_pointer += 1;
            }
            0x46 => {
                // NIP
                self.nip()?;
                self.instruction_pointer += 1;
            }
            0x48 => {
                // XDROP
                self.xdrop()?;
                self.instruction_pointer += 1;
            }
            0x49 => {
                // CLEAR
                self.clear_stack();
                self.instruction_pointer += 1;
            }
            0x4A => {
                // DUP
                let top = self.peek_stack()?.clone();
                self.push_stack(top)?;
                self.instruction_pointer += 1;
            }
            0x4B => {
                // OVER
                self.over()?;
                self.instruction_pointer += 1;
            }
            0x4D => {
                // PICK (index on top)
                self.pick_n()?;
                self.instruction_pointer += 1;
            }
            0x4E => {
                // TUCK
                self.tuck()?;
                self.instruction_pointer += 1;
            }
            0x50 => {
                // SWAP
                let top = self.pop_stack()?;
                let second = self.pop_stack()?;
                self.push_stack(top)?;
                self.push_stack(second)?;
                self.instruction_pointer += 1;
            }
            0x51 => {
                // ROT
                if self.stack.len() < 3 {
                    return Err(RuntimeError::ExecutionError {
                        message: "ROT: insufficient stack items".to_string(),
                    });
                }
                let c = self.pop_stack()?;
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                self.push_stack(b)?;
                self.push_stack(c)?;
                self.push_stack(a)?;
                self.instruction_pointer += 1;
            }
            0x52 => {
                // ROLL (index on top)
                self.roll()?;
                self.instruction_pointer += 1;
            }
            0x53 => {
                // REVERSE3
                self.reverse_top_n(3)?;
                self.instruction_pointer += 1;
            }
            0x54 => {
                // REVERSE4
                self.reverse_top_n(4)?;
                self.instruction_pointer += 1;
            }
            0x55 => {
                // REVERSEN (count on top)
                let count = self.pop_usize("REVERSEN")?;
                self.reverse_top_n(count)?;
                self.instruction_pointer += 1;
            }
            0x88 => {
                // NEWBUFFER
                self.new_buffer()?;
                self.instruction_pointer += 1;
            }
            0x89 => {
                // MEMCPY
                self.memcpy_bytes()?;
                self.instruction_pointer += 1;
            }
            0x8B => {
                // CAT
                self.concat_bytes()?;
                self.instruction_pointer += 1;
            }
            0x8C => {
                // SUBSTR
                self.substr_bytes()?;
                self.instruction_pointer += 1;
            }
            0x8D => {
                // LEFT
                self.left_bytes()?;
                self.instruction_pointer += 1;
            }
            0x8E => {
                // RIGHT
                self.right_bytes()?;
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
            0x97 => {
                // EQUAL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
            }
            0x98 => {
                // NOTEQUAL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(!result))?;
                self.instruction_pointer += 1;
            }
            0x99 => {
                // SIGN
                let value = self.pop_stack()?;
                let result = self.sign_stack_item(value)?;
                self.push_stack(StackItem::Integer(result))?;
                self.instruction_pointer += 1;
            }
            0x9A => {
                // ABS
                let value = self.pop_stack()?;
                let result = self.abs_stack_item(value)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x9B => {
                // NEGATE
                let value = self.pop_stack()?;
                let result = self.negate_stack_item(value)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x9C => {
                // INC
                let value = self.pop_stack()?;
                let result = self.inc_stack_item(value)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x9D => {
                // DEC
                let value = self.pop_stack()?;
                let result = self.dec_stack_item(value)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x9E => {
                // ADD
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.add_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0x9F => {
                // SUB
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.sub_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0xA0 => {
                // MUL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.mul_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0xA1 => {
                // DIV
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.div_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0xA2 => {
                // MOD
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.mod_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0xA3 => {
                // POW
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.pow_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0xA4 => {
                // SQRT
                let value = self.pop_stack()?;
                let result = self.sqrt_stack_item(value)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0xA5 => {
                // MODMUL
                let modulus = self.pop_stack()?;
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.modmul_stack_items(a, b, modulus)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0xA6 => {
                // MODPOW
                let modulus = self.pop_stack()?;
                let exponent = self.pop_stack()?;
                let base = self.pop_stack()?;
                let result = self.modpow_stack_items(base, exponent, modulus)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0xA8 => {
                // SHL
                let shift = self.pop_stack()?;
                let value = self.pop_stack()?;
                let result = self.shift_left(value, shift)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0xA9 => {
                // SHR
                let shift = self.pop_stack()?;
                let value = self.pop_stack()?;
                let result = self.shift_right(value, shift)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0xAA => {
                // NOT (logical)
                self.logical_not()?;
                self.instruction_pointer += 1;
            }
            0xAB => {
                // BOOLAND
                self.logical_and()?;
                self.instruction_pointer += 1;
            }
            0xAC => {
                // BOOLOR
                self.logical_or()?;
                self.instruction_pointer += 1;
            }
            0xB1 => {
                // NZ
                let value = self.pop_stack()?;
                self.push_stack(StackItem::Boolean(value.is_truthy()))?;
                self.instruction_pointer += 1;
            }
            0xB3 => {
                // NUMEQUAL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
            }
            0xB4 => {
                // NUMNOTEQUAL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(!result))?;
                self.instruction_pointer += 1;
            }
            0xB5 => {
                // LT
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.less_than(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
            }
            0xB6 => {
                // LE
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let lt = self.less_than(&a, &b)?;
                let eq = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(lt || eq))?;
                self.instruction_pointer += 1;
            }
            0xB7 => {
                // GT
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.greater_than(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
            }
            0xB8 => {
                // GE
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let gt = self.greater_than(&a, &b)?;
                let eq = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(gt || eq))?;
                self.instruction_pointer += 1;
            }
            0xB9 => {
                // MIN
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.min_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0xBA => {
                // MAX
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.max_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            }
            0xBB => {
                // WITHIN (left inclusive)
                let max_item = self.pop_stack()?;
                let min_item = self.pop_stack()?;
                let value = self.pop_stack()?;
                let result = self.within_stack_items(value, min_item, max_item)?;
                self.push_stack(StackItem::Boolean(result))?;
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
                // Consume syscall gas if known
                if let Some(cost) = self.syscall_gas.get(&syscall_id) {
                    let projected = self.gas_used.saturating_add(*cost);
                    if projected > self.gas_limit {
                        return Err(RuntimeError::OutOfGas {
                            used: projected,
                            limit: self.gas_limit,
                        });
                    }
                    self.gas_used = projected;
                }
                self.handle_syscall(syscall_id)?;
                self.instruction_pointer += 5;
            }
            0x57 => {
                // INITSLOT locals, args
                if self.instruction_pointer + 2 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "INITSLOT: insufficient bytecode".to_string(),
                    });
                }
                let local_count = self.bytecode[self.instruction_pointer as usize + 1] as usize;
                let arg_count = self.bytecode[self.instruction_pointer as usize + 2] as usize;
                self.locals = vec![StackItem::Null; local_count];
                self.args = vec![StackItem::Null; arg_count];
                self.instruction_pointer += 3;
            }
            0x56 => {
                // INITSSLOT static slots
                if self.instruction_pointer + 1 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "INITSSLOT: insufficient bytecode".to_string(),
                    });
                }
                let static_count = self.bytecode[self.instruction_pointer as usize + 1] as usize;
                self.static_fields = vec![StackItem::Null; static_count];
                self.instruction_pointer += 2;
            }
            0x58..=0x5E => {
                // LDSFLD0-6
                let index = (opcode - 0x58) as usize;
                let value = self.get_static(index)?;
                self.push_stack(value)?;
                self.instruction_pointer += 1;
            }
            0x5F => {
                // LDSFLD
                let idx_pos = self.instruction_pointer as usize + 1;
                if idx_pos >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "LDSFLD: missing index".to_string(),
                    });
                }
                let index = self.bytecode[idx_pos] as usize;
                let value = self.get_static(index)?;
                self.push_stack(value)?;
                self.instruction_pointer += 2;
            }
            0x60..=0x66 => {
                // STSFLD0-6
                let index = (opcode - 0x60) as usize;
                let value = self.pop_stack()?;
                self.set_static(index, value)?;
                self.instruction_pointer += 1;
            }
            0x67 => {
                // STSFLD
                let idx_pos = self.instruction_pointer as usize + 1;
                if idx_pos >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "STSFLD: missing index".to_string(),
                    });
                }
                let index = self.bytecode[idx_pos] as usize;
                let value = self.pop_stack()?;
                self.set_static(index, value)?;
                self.instruction_pointer += 2;
            }
            0x68..=0x6E => {
                // LDLOC0-6
                let index = (opcode - 0x68) as usize;
                let value = self.get_local(index)?;
                self.push_stack(value)?;
                self.instruction_pointer += 1;
            }
            0x6F => {
                // LDLOC
                let idx_pos = self.instruction_pointer as usize + 1;
                if idx_pos >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "LDLOC: missing index".to_string(),
                    });
                }
                let index = self.bytecode[idx_pos] as usize;
                let value = self.get_local(index)?;
                self.push_stack(value)?;
                self.instruction_pointer += 2;
            }
            0x70..=0x76 => {
                // STLOC0-6
                let index = (opcode - 0x70) as usize;
                let value = self.pop_stack()?;
                self.set_local(index, value)?;
                self.instruction_pointer += 1;
            }
            0x77 => {
                // STLOC
                let idx_pos = self.instruction_pointer as usize + 1;
                if idx_pos >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "STLOC: missing index".to_string(),
                    });
                }
                let index = self.bytecode[idx_pos] as usize;
                let value = self.pop_stack()?;
                self.set_local(index, value)?;
                self.instruction_pointer += 2;
            }
            0x78..=0x7E => {
                // LDARG0-6
                let index = (opcode - 0x78) as usize;
                let value = self.get_arg(index)?;
                self.push_stack(value)?;
                self.instruction_pointer += 1;
            }
            0x7F => {
                // LDARG
                let idx_pos = self.instruction_pointer as usize + 1;
                if idx_pos >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "LDARG: missing index".to_string(),
                    });
                }
                let index = self.bytecode[idx_pos] as usize;
                let value = self.get_arg(index)?;
                self.push_stack(value)?;
                self.instruction_pointer += 2;
            }
            0x80..=0x86 => {
                // STARG0-6
                let index = (opcode - 0x80) as usize;
                let value = self.pop_stack()?;
                self.set_arg(index, value)?;
                self.instruction_pointer += 1;
            }
            0x87 => {
                // STARG
                let idx_pos = self.instruction_pointer as usize + 1;
                if idx_pos >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "STARG: missing index".to_string(),
                    });
                }
                let index = self.bytecode[idx_pos] as usize;
                let value = self.pop_stack()?;
                self.set_arg(index, value)?;
                self.instruction_pointer += 2;
            }
            0xBE => {
                // PACKMAP
                self.pack_map()?;
                self.instruction_pointer += 1;
            }
            0xBF => {
                // PACKSTRUCT (array-backed)
                self.pack_items()?;
                self.instruction_pointer += 1;
            }
            0xC0 => {
                // PACK
                self.pack_items()?;
                self.instruction_pointer += 1;
            }
            0xC1 => {
                // UNPACK
                self.unpack()?;
                self.instruction_pointer += 1;
            }
            0xC2 => {
                // NEWARRAY0
                self.new_array0()?;
                self.instruction_pointer += 1;
            }
            0xC3 => {
                // NEWARRAY
                self.new_array()?;
                self.instruction_pointer += 1;
            }
            0xC4 => {
                // NEWARRAY_T
                self.new_array()?;
                self.instruction_pointer += 1;
            }
            0xC5 => {
                // NEWSTRUCT0
                self.new_struct0()?;
                self.instruction_pointer += 1;
            }
            0xC6 => {
                // NEWSTRUCT
                self.new_struct()?;
                self.instruction_pointer += 1;
            }
            0xC8 => {
                // NEWMAP
                self.new_map()?;
                self.instruction_pointer += 1;
            }
            0xCA => {
                // SIZE
                self.size_of()?;
                self.instruction_pointer += 1;
            }
            0xCB => {
                // HASKEY
                self.haskey()?;
                self.instruction_pointer += 1;
            }
            0xCC => {
                // KEYS
                self.keys()?;
                self.instruction_pointer += 1;
            }
            0xCD => {
                // VALUES
                self.values()?;
                self.instruction_pointer += 1;
            }
            0xCE => {
                // PICKITEM
                self.pick_item()?;
                self.instruction_pointer += 1;
            }
            0xCF => {
                // APPEND
                self.append_item()?;
                self.instruction_pointer += 1;
            }
            0xD0 => {
                // SETITEM
                self.set_item()?;
                self.instruction_pointer += 1;
            }
            0xD1 => {
                // REVERSEITEMS
                self.reverse_items()?;
                self.instruction_pointer += 1;
            }
            0xD2 => {
                // REMOVE
                self.remove_item()?;
                self.instruction_pointer += 1;
            }
            0xD3 => {
                // CLEARITEMS
                self.clear_items()?;
                self.instruction_pointer += 1;
            }
            0xD4 => {
                // POPITEM
                self.pop_item_from_collection()?;
                self.instruction_pointer += 1;
            }
            0xD8 => {
                // ISNULL
                let item = self.pop_stack()?;
                self.push_stack(StackItem::Boolean(matches!(item, StackItem::Null)))?;
                self.instruction_pointer += 1;
            }
            0xD9 => {
                // ISTYPE (best-effort)
                let type_marker = self.pop_stack()?;
                let item = self.pop_stack()?;
                let expected = Self::stack_item_type_code(&type_marker);
                let matches = match expected {
                    0x00 => true, // Any
                    0x20 => matches!(item, StackItem::Boolean(_)),
                    0x21 | 0x22 => {
                        matches!(item, StackItem::Integer(_) | StackItem::UnsignedInteger(_))
                    }
                    0x28 | 0x30 => matches!(item, StackItem::ByteArray(_)),
                    0x40 | 0x41 => matches!(item, StackItem::Array(_)),
                    0x48 => matches!(item, StackItem::Map(_)),
                    0x60 => matches!(item, StackItem::ByteArray(_)), // interop handles as byte tokens
                    0x80 => self.is_iterator_token(&item),
                    _ => false,
                };
                self.push_stack(StackItem::Boolean(matches))?;
                self.instruction_pointer += 1;
            }
            0xDB => {
                // CONVERT (best-effort coercion)
                let target = self.pop_stack()?;
                let item = self.pop_stack()?;
                let converted = self.convert_item(item, target)?;
                self.push_stack(converted)?;
                self.instruction_pointer += 1;
            }
            0xE0 => {
                // ABORTMSG
                let message = Self::stack_item_to_bytes(self.pop_stack()?);
                return Err(RuntimeError::ExecutionError {
                    message: format!("ABORTMSG: {}", String::from_utf8_lossy(&message)),
                });
            }
            0xE1 => {
                // ASSERTMSG
                let message = Self::stack_item_to_bytes(self.pop_stack()?);
                let condition = self.pop_stack()?;
                if condition.is_truthy() {
                    self.instruction_pointer += 1;
                } else {
                    return Err(RuntimeError::ExecutionError {
                        message: format!("ASSERTMSG failed: {}", String::from_utf8_lossy(&message)),
                    });
                }
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

            _ => {
                return Err(RuntimeError::ExecutionError {
                    message: format!("Unsupported opcode: 0x{:02X}", opcode),
                });
            }
        }

        // Consume gas after successful execution
        self.gas_used = self.gas_used.saturating_add(gas_cost);

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

    fn sign_stack_item(&self, value: StackItem) -> Result<i64, RuntimeError> {
        match value {
            StackItem::Integer(v) => Ok(v.signum()),
            StackItem::UnsignedInteger(v) => Ok(if v == 0 { 0 } else { 1 }),
            StackItem::Boolean(b) => Ok(if b { 1 } else { 0 }),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operand for SIGN".to_string(),
            }),
        }
    }

    fn abs_stack_item(&self, value: StackItem) -> Result<StackItem, RuntimeError> {
        match value {
            StackItem::Integer(v) => {
                v.checked_abs()
                    .map(StackItem::Integer)
                    .ok_or(RuntimeError::ExecutionError {
                        message: "ABS overflow".to_string(),
                    })
            }
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(v)),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operand for ABS".to_string(),
            }),
        }
    }

    fn negate_stack_item(&self, value: StackItem) -> Result<StackItem, RuntimeError> {
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(v.wrapping_neg())),
            StackItem::UnsignedInteger(v) => {
                if v <= i64::MAX as u64 {
                    Ok(StackItem::Integer(-(v as i64)))
                } else {
                    Err(RuntimeError::ExecutionError {
                        message: "NEGATE overflow for unsigned value".to_string(),
                    })
                }
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operand for NEGATE".to_string(),
            }),
        }
    }

    fn inc_stack_item(&self, value: StackItem) -> Result<StackItem, RuntimeError> {
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(v.wrapping_add(1))),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(v.wrapping_add(1))),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operand for INC".to_string(),
            }),
        }
    }

    fn dec_stack_item(&self, value: StackItem) -> Result<StackItem, RuntimeError> {
        match value {
            StackItem::Integer(v) => Ok(StackItem::Integer(v.wrapping_sub(1))),
            StackItem::UnsignedInteger(v) => Ok(StackItem::UnsignedInteger(v.wrapping_sub(1))),
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operand for DEC".to_string(),
            }),
        }
    }

    fn pow_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        let exponent: u32 = match b {
            StackItem::Integer(e) => {
                if e < 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "POW exponent must be non-negative".to_string(),
                    });
                }
                e as u32
            }
            StackItem::UnsignedInteger(e) => {
                e.try_into().map_err(|_| RuntimeError::ExecutionError {
                    message: "POW exponent too large".to_string(),
                })?
            }
            _ => {
                return Err(RuntimeError::ExecutionError {
                    message: "Invalid exponent for POW".to_string(),
                })
            }
        };

        match a {
            StackItem::Integer(base) => Ok(StackItem::Integer(base.wrapping_pow(exponent))),
            StackItem::UnsignedInteger(base) => {
                Ok(StackItem::UnsignedInteger(base.wrapping_pow(exponent)))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid base for POW".to_string(),
            }),
        }
    }

    fn sqrt_stack_item(&self, value: StackItem) -> Result<StackItem, RuntimeError> {
        fn int_sqrt(n: u128) -> u128 {
            let mut x0 = n;
            let mut x1 = (x0 + 1) >> 1;
            while x1 < x0 {
                x0 = x1;
                x1 = (x1 + n / x1) >> 1;
            }
            x0
        }

        match value {
            StackItem::Integer(v) => {
                if v < 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "SQRT of negative value".to_string(),
                    });
                }
                Ok(StackItem::Integer(int_sqrt(v as u128) as i64))
            }
            StackItem::UnsignedInteger(v) => {
                Ok(StackItem::UnsignedInteger(int_sqrt(v as u128) as u64))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operand for SQRT".to_string(),
            }),
        }
    }

    fn modmul_stack_items(
        &self,
        a: StackItem,
        b: StackItem,
        modulus: StackItem,
    ) -> Result<StackItem, RuntimeError> {
        match (a, b, modulus) {
            (StackItem::Integer(x), StackItem::Integer(y), StackItem::Integer(m)) => {
                if m == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "MODMUL modulus cannot be zero".to_string(),
                    });
                }
                let modulus = m.abs() as i128;
                let product = (x as i128).wrapping_mul(y as i128);
                let result = product.rem_euclid(modulus);
                Ok(StackItem::Integer(result as i64))
            }
            (
                StackItem::UnsignedInteger(x),
                StackItem::UnsignedInteger(y),
                StackItem::UnsignedInteger(m),
            ) => {
                if m == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "MODMUL modulus cannot be zero".to_string(),
                    });
                }
                let product = (x as u128).wrapping_mul(y as u128);
                let result = product % (m as u128);
                Ok(StackItem::UnsignedInteger(result as u64))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for MODMUL".to_string(),
            }),
        }
    }

    fn modpow_stack_items(
        &self,
        base: StackItem,
        exponent: StackItem,
        modulus: StackItem,
    ) -> Result<StackItem, RuntimeError> {
        match (base, exponent, modulus) {
            (StackItem::Integer(b), StackItem::Integer(e), StackItem::Integer(m)) => {
                if m == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "MODPOW modulus cannot be zero".to_string(),
                    });
                }
                if e < 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "MODPOW exponent must be non-negative".to_string(),
                    });
                }
                let modulus = m.abs() as i128;
                let mut result: i128 = 1 % modulus;
                let mut base = (b as i128).rem_euclid(modulus);
                let mut exp = e as u128;
                while exp > 0 {
                    if exp & 1 == 1 {
                        result = (result * base).rem_euclid(modulus);
                    }
                    base = (base * base).rem_euclid(modulus);
                    exp >>= 1;
                }
                Ok(StackItem::Integer(result as i64))
            }
            (
                StackItem::UnsignedInteger(b),
                StackItem::UnsignedInteger(e),
                StackItem::UnsignedInteger(m),
            ) => {
                if m == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "MODPOW modulus cannot be zero".to_string(),
                    });
                }
                let mut result: u128 = 1 % m as u128;
                let mut base = (b as u128) % (m as u128);
                let mut exp = e as u128;
                while exp > 0 {
                    if exp & 1 == 1 {
                        result = (result * base) % (m as u128);
                    }
                    base = (base * base) % (m as u128);
                    exp >>= 1;
                }
                Ok(StackItem::UnsignedInteger(result as u64))
            }
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for MODPOW".to_string(),
            }),
        }
    }

    fn min_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        let lt = self.less_than(&a, &b)?;
        if lt {
            Ok(a)
        } else {
            Ok(b)
        }
    }

    fn max_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        let gt = self.greater_than(&a, &b)?;
        if gt {
            Ok(a)
        } else {
            Ok(b)
        }
    }

    fn within_stack_items(
        &self,
        value: StackItem,
        min_item: StackItem,
        max_item: StackItem,
    ) -> Result<bool, RuntimeError> {
        let ge_min = !self.less_than(&value, &min_item)?;
        let lt_max = self.less_than(&value, &max_item)?;
        Ok(ge_min && lt_max)
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
            (StackItem::Array(x), StackItem::Array(y)) => Ok(x == y),
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

    fn murmur3_32(data: &[u8]) -> u32 {
        let digest = Sha256::digest(data);
        u32::from_le_bytes([digest[0], digest[1], digest[2], digest[3]])
    }

    fn verify_secp256k1(pubkey: &[u8], signature: &[u8]) -> bool {
        // Best-effort ECDSA check against sha256("") as message placeholder
        if pubkey.is_empty() || signature.len() < 64 {
            return false;
        }
        let msg = Sha256::digest(&[]);
        if let (Ok(pk), Ok(sig)) = (
            secp256k1::PublicKey::from_slice(pubkey),
            secp256k1::ecdsa::Signature::from_der(signature).or_else(|_| {
                secp256k1::ecdsa::Signature::from_compact(signature)
            }),
        ) {
            let secp = secp256k1::Secp256k1::verification_only();
            let msg = secp256k1::Message::from_slice(&msg).unwrap();
            secp.verify_ecdsa(&msg, &sig, &pk).is_ok()
        } else {
            false
        }
    }

    fn stack_item_to_bytes(item: StackItem) -> Vec<u8> {
        match item {
            StackItem::ByteArray(bytes) => bytes,
            StackItem::Integer(value) => value.to_le_bytes().to_vec(),
            StackItem::UnsignedInteger(value) => value.to_le_bytes().to_vec(),
            StackItem::Boolean(value) => vec![value as u8],
            StackItem::Map(map) => serde_json::to_vec(&map).unwrap_or_default(),
            StackItem::Array(items) => serde_json::to_vec(&items).unwrap_or_default(),
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
        self.storage_host = Some(NonNull::from(storage));
        self.storage_overlay.clear();
        // Seed native contract balances for default account
        self.neo_balances
            .insert(self.default_account_bytes.clone(), self.neo_total_supply);
        self.gas_balances
            .insert(self.default_account_bytes.clone(), self.gas_total_supply);
        self.try_stack.clear();
        Ok(())
    }

    fn build_storage_entries(&self, prefix: Vec<u8>) -> Result<Vec<StackItem>, RuntimeError> {
        let mut entries: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        if let (Some(mut ptr), Some(account)) = (self.storage_host, self.storage_account.as_ref()) {
            let storage = unsafe { ptr.as_mut() };
            let query = storage::StorageQuery {
                account: account.clone(),
                key_prefix: Some(prefix.clone()),
                limit: None,
                include_pending: true,
            };
            entries = storage.query(query)?;
        }

        for (key, entry) in &self.storage_overlay {
            if !key.starts_with(&prefix) {
                continue;
            }
            match &entry.value {
                Some(value) => {
                    entries.retain(|(k, _)| k != key);
                    entries.push((key.clone(), value.clone()));
                }
                None => entries.retain(|(k, _)| k != key),
            }
        }

        entries.sort_by(|a, b| a.0.cmp(&b.0));
        let mut items = Vec::with_capacity(entries.len());
        for (k, v) in entries {
            items.push(StackItem::Array(vec![
                StackItem::ByteArray(k),
                StackItem::ByteArray(v),
            ]));
        }
        Ok(items)
    }

    fn allocate_iterator(&mut self, entries: Vec<StackItem>) -> StackItem {
        let id = self.next_iterator_id;
        self.next_iterator_id = self.next_iterator_id.saturating_add(1);
        self.iterators.insert(
            id,
            IteratorState {
                entries,
                index: 0,
            },
        );
        StackItem::ByteArray(id.to_le_bytes().to_vec())
    }

    fn iterator_id_from_item(item: &StackItem) -> Option<u64> {
        if let StackItem::ByteArray(bytes) = item {
            if bytes.len() >= 8 {
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&bytes[..8]);
                return Some(u64::from_le_bytes(buf));
            }
        }
        None
    }

    fn register_contract(&mut self, nef: Vec<u8>, manifest: Vec<u8>) -> ContractState {
        let digest = Sha256::digest(&nef);
        let mut hash = [0u8; 20];
        hash.copy_from_slice(&digest[0..20]);
        let state = ContractState {
            hash,
            nef,
            manifest,
            update_counter: 0,
        };
        self.contract_registry.insert(hash.to_vec(), state.clone());
        state
    }

    fn update_contract(
        &mut self,
        hash: &[u8],
        nef: Vec<u8>,
        manifest: Vec<u8>,
    ) -> Option<ContractState> {
        if let Some(existing) = self.contract_registry.get_mut(hash) {
            existing.nef = nef;
            existing.manifest = manifest;
            existing.update_counter = existing.update_counter.saturating_add(1);
            return Some(existing.clone());
        }
        None
    }

    fn lookup_contract(&self, hash: &[u8]) -> Option<ContractState> {
        self.contract_registry.get(hash).cloned()
    }

    fn contract_to_stackitem(&self, state: &ContractState) -> StackItem {
        let mut map = std::collections::HashMap::new();
        map.insert(b"hash".to_vec(), StackItem::ByteArray(state.hash.to_vec()));
        map.insert(b"nef".to_vec(), StackItem::ByteArray(state.nef.clone()));
        map.insert(
            b"manifest".to_vec(),
            StackItem::ByteArray(state.manifest.clone()),
        );
        map.insert(
            b"updatecounter".to_vec(),
            StackItem::UnsignedInteger(state.update_counter as u64),
        );
        StackItem::Map(map)
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
            (Some(mut ptr), Some(account)) => {
                // Safety: storage_host is set via bind_storage, which guarantees the pointer is valid
                // for the duration of execution. We avoid holding references across calls.
                let storage = unsafe { ptr.as_mut() };
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
        if let Some(name) = spec::opcode_name(opcode) {
            name.to_string()
        } else {
            format!("UNKNOWN_{:02X}", opcode)
        }
    }

    fn get_instruction_gas_cost(&self, opcode: u8) -> u64 {
        spec::opcode_gas(opcode).unwrap_or(1)
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

    /// Sync gas usage with execution context accounting.
    pub fn sync_from_execution(&mut self, execution_used: u64) {
        self.used = self.base_cost.saturating_add(execution_used);
    }

    /// Base transaction cost that seeds gas tracking.
    pub fn base_cost(&self) -> u64 {
        self.base_cost
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
            StackItem::Array(items) => serde_json::to_vec(items).unwrap_or_default(),
            StackItem::Map(map) => serde_json::to_vec(map).unwrap_or_default(),
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
            StackItem::Array(items) => !items.is_empty(),
            StackItem::Map(map) => !map.is_empty(),
            StackItem::Boolean(b) => *b,
            StackItem::Null => false,
        }
    }
}
