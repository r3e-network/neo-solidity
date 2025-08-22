//! Execution Context Module
//! 
//! Provides execution context and gas tracking for Neo runtime.

use super::{RuntimeConfig, RuntimeError};
use serde::{Deserialize, Serialize};
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
    call_stack: Vec<CallFrame>,
    debugging_enabled: bool,
    breakpoints: HashSet<u32>,
    instruction_count: u64,
    max_stack_depth: u32,
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

impl ExecutionContext {
    /// Create new execution context
    pub fn new(config: &RuntimeConfig) -> Result<Self, RuntimeError> {
        Ok(Self {
            bytecode: Vec::new(),
            input_data: Vec::new(),
            gas_limit: config.gas_limit,
            gas_used: 0,
            instruction_pointer: 0,
            stack: Vec::new(),
            memory: vec![0; 1024], // Initial memory size
            call_stack: Vec::new(),
            debugging_enabled: config.enable_debugging,
            breakpoints: HashSet::new(),
            instruction_count: 0,
            max_stack_depth: 0,
        })
    }

    /// Initialize context for execution
    pub fn initialize(&mut self, bytecode: &[u8], input: &[u8]) -> Result<(), RuntimeError> {
        self.bytecode = bytecode.to_vec();
        self.input_data = input.to_vec();
        self.instruction_pointer = 0;
        self.stack.clear();
        self.call_stack.clear();
        self.gas_used = 0;
        self.instruction_count = 0;
        Ok(())
    }

    /// Get gas limit
    pub fn gas_limit(&self) -> u64 {
        self.gas_limit
    }

    /// Get instruction count
    pub fn instruction_count(&self) -> u64 {
        self.instruction_count
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
        if self.stack.len() >= 2048 { // NeoVM stack limit
            return Err(RuntimeError::StackOverflow { depth: self.stack.len() as u32 });
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

    /// Read from memory
    pub fn read_memory(&self, address: usize, length: usize) -> Result<&[u8], RuntimeError> {
        if address + length > self.memory.len() {
            return Err(RuntimeError::ExecutionError {
                message: "Memory access out of bounds".to_string(),
            });
        }
        Ok(&self.memory[address..address + length])
    }

    /// Write to memory
    pub fn write_memory(&mut self, address: usize, data: &[u8]) -> Result<(), RuntimeError> {
        if address + data.len() > self.memory.len() {
            // Expand memory if needed
            self.memory.resize(address + data.len(), 0);
        }
        self.memory[address..address + data.len()].copy_from_slice(data);
        Ok(())
    }

    /// Call function
    pub fn call_function(&mut self, address: u32, function_name: Option<String>) -> Result<(), RuntimeError> {
        if self.call_stack.len() >= 1024 { // Call stack limit
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
        // Simplified instruction execution
        match opcode {
            0x10 => { // PUSH0
                self.push_stack(StackItem::Integer(0))?;
                self.instruction_pointer += 1;
            },
            0x11 => { // PUSH1
                self.push_stack(StackItem::Integer(1))?;
                self.instruction_pointer += 1;
            },
            0x95 => { // ADD
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.add_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            },
            0x96 => { // SUB
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.sub_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            },
            0x40 => { // RET
                if self.call_stack.is_empty() {
                    // End of execution
                    self.instruction_pointer = self.bytecode.len() as u32;
                } else {
                    self.return_from_function()?;
                }
            },
            _ => {
                self.instruction_pointer += 1;
            }
        }

        // Consume gas
        self.gas_used += self.get_instruction_gas_cost(opcode);
        
        Ok(())
    }

    fn add_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_add(y)))
            },
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_add(y)))
            },
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for ADD".to_string(),
            }),
        }
    }

    fn sub_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_sub(y)))
            },
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_sub(y)))
            },
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for SUB".to_string(),
            }),
        }
    }

    fn get_opcode_name(&self, opcode: u8) -> String {
        match opcode {
            0x10 => "PUSH0".to_string(),
            0x11 => "PUSH1".to_string(),
            0x95 => "ADD".to_string(),
            0x96 => "SUB".to_string(),
            0x40 => "RET".to_string(),
            _ => format!("UNKNOWN_{:02X}", opcode),
        }
    }

    fn get_instruction_gas_cost(&self, opcode: u8) -> u64 {
        match opcode {
            0x10..=0x20 => 1, // PUSH instructions
            0x95..=0x99 => 3, // Arithmetic instructions
            0x40 => 0,        // RET
            _ => 1,           // Default cost
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
    pub fn consume_gas(&mut self, operation: &str, amount: Option<u64>) -> Result<(), RuntimeError> {
        let cost = amount.unwrap_or_else(|| {
            *self.operation_costs.get(operation).unwrap_or(&1)
        });

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