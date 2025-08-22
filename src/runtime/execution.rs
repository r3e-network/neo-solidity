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
            0x00 => { // PUSHINT8
                if self.instruction_pointer + 1 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT8: insufficient bytecode".to_string(),
                    });
                }
                let value = self.bytecode[self.instruction_pointer as usize + 1] as i8 as i64;
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 2;
            },
            0x01 => { // PUSHINT16
                if self.instruction_pointer + 2 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT16: insufficient bytecode".to_string(),
                    });
                }
                let bytes = &self.bytecode[self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 3];
                let value = i16::from_le_bytes([bytes[0], bytes[1]]) as i64;
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 3;
            },
            0x02 => { // PUSHINT32
                if self.instruction_pointer + 4 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT32: insufficient bytecode".to_string(),
                    });
                }
                let bytes = &self.bytecode[self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 5];
                let value = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64;
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 5;
            },
            0x03 => { // PUSHINT64
                if self.instruction_pointer + 8 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT64: insufficient bytecode".to_string(),
                    });
                }
                let bytes = &self.bytecode[self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 9];
                let mut array = [0u8; 8];
                array.copy_from_slice(bytes);
                let value = i64::from_le_bytes(array);
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 9;
            },
            0x0C => { // PUSHDATA1
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
                let data = self.bytecode[self.instruction_pointer as usize + 2..self.instruction_pointer as usize + 2 + length].to_vec();
                self.push_stack(StackItem::ByteArray(data))?;
                self.instruction_pointer += 2 + length as u32;
            },
            0x10 => { // PUSHM1
                self.push_stack(StackItem::Integer(-1))?;
                self.instruction_pointer += 1;
            },
            0x11 => { // PUSH0
                self.push_stack(StackItem::Integer(0))?;
                self.instruction_pointer += 1;
            },
            0x12..=0x20 => { // PUSH1-PUSH16
                let value = (opcode - 0x11) as i64;
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 1;
            },
            
            // Flow control operations (0x21-0x38)
            0x21 => { // NOP
                self.instruction_pointer += 1;
            },
            0x22 => { // JMP
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
            },
            0x23 => { // JMPIF
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
            },
            
            // Stack operations (0x39-0x4F)
            0x39 => { // DEPTH
                self.push_stack(StackItem::Integer(self.stack.len() as i64))?;
                self.instruction_pointer += 1;
            },
            0x3A => { // DROP
                self.pop_stack()?;
                self.instruction_pointer += 1;
            },
            0x3E => { // DUP
                let top = self.peek_stack()?.clone();
                self.push_stack(top)?;
                self.instruction_pointer += 1;
            },
            0x42 => { // SWAP
                let top = self.pop_stack()?;
                let second = self.pop_stack()?;
                self.push_stack(top)?;
                self.push_stack(second)?;
                self.instruction_pointer += 1;
            },
            
            // Arithmetic (0x90-0x9F)
            0x90 => { // ADD
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.add_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            },
            0x91 => { // SUB
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.sub_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            },
            0x92 => { // MUL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.mul_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            },
            0x93 => { // DIV
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.div_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            },
            0x94 => { // MOD
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.mod_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
            },
            
            // Comparison operations
            0x87 => { // EQUAL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
            },
            0x9F => { // LT
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.less_than(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
            },
            0xA1 => { // GT
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.greater_than(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
            },
            
            // Control flow
            0x40 => { // RET
                if self.call_stack.is_empty() {
                    // End of execution
                    self.instruction_pointer = self.bytecode.len() as u32;
                } else {
                    self.return_from_function()?;
                }
            },
            0x66 => { // THROW
                return Err(RuntimeError::ExecutionError {
                    message: "THROW instruction executed".to_string(),
                });
            },
            0x67 => { // ABORT
                return Err(RuntimeError::ExecutionError {
                    message: "ABORT instruction executed".to_string(),
                });
            },
            
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
    
    fn mul_stack_items(&self, a: StackItem, b: StackItem) -> Result<StackItem, RuntimeError> {
        match (a, b) {
            (StackItem::Integer(x), StackItem::Integer(y)) => {
                Ok(StackItem::Integer(x.wrapping_mul(y)))
            },
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                Ok(StackItem::UnsignedInteger(x.wrapping_mul(y)))
            },
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
            },
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                if y == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "Division by zero".to_string(),
                    });
                }
                Ok(StackItem::UnsignedInteger(x / y))
            },
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
            },
            (StackItem::UnsignedInteger(x), StackItem::UnsignedInteger(y)) => {
                if y == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "Modulo by zero".to_string(),
                    });
                }
                Ok(StackItem::UnsignedInteger(x % y))
            },
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for MOD".to_string(),
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
            },
            (StackItem::UnsignedInteger(x), StackItem::Integer(y)) => {
                if *y < 0 {
                    Ok(false)
                } else {
                    Ok(*x == *y as u64)
                }
            },
            _ => Ok(false),
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
            },
            (StackItem::UnsignedInteger(x), StackItem::Integer(y)) => {
                if *y < 0 {
                    Ok(false)
                } else {
                    Ok(*x < (*y as u64))
                }
            },
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
            },
            (StackItem::UnsignedInteger(x), StackItem::Integer(y)) => {
                if *y < 0 {
                    Ok(true)
                } else {
                    Ok(*x > (*y as u64))
                }
            },
            _ => Err(RuntimeError::ExecutionError {
                message: "Invalid operands for comparison".to_string(),
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
            // Push operations
            0x00..=0x0F => 1,     // PUSHINT variants
            0x0C..=0x0D => 2,     // PUSHDATA variants (base cost)
            0x10..=0x20 => 1,     // PUSH0-PUSH16, PUSHM1
            
            // Flow control
            0x21 => 1,            // NOP
            0x22..=0x28 => 2,     // Jump instructions
            
            // Stack operations
            0x39..=0x47 => 2,     // Stack manipulation
            
            // Arithmetic
            0x90..=0x94 => 4,     // Basic arithmetic
            0x95 => 8,            // POW (expensive)
            0x96 => 6,            // SQRT
            
            // Comparison operations
            0x87..=0x88 => 3,     // EQUAL, NOTEQUAL
            0x9F..=0xA2 => 3,     // LT, LE, GT, GE
            
            // Control flow
            0x40 => 0,            // RET
            0x66..=0x67 => 1,     // THROW, ABORT
            
            _ => 1,               // Default cost
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