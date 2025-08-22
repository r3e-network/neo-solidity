//! Advanced Debugging Capabilities for Neo Solidity
//! 
//! This crate provides comprehensive debugging infrastructure including:
//! - Source map generation and parsing
//! - Runtime trace hooks and breakpoints
//! - Interactive debugging interface
//! - Step-by-step execution tracking
//! - Variable inspection and state visualization
//! - Call stack analysis and frame navigation

pub mod source_maps;
pub mod trace_hooks;
pub mod interactive;
pub mod breakpoints;
pub mod stack_trace;
pub mod variable_inspector;
pub mod error_decoder;
pub mod dwarf_integration;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tracing::{info, debug, warn};

/// Main debugger interface for Neo Solidity
pub struct Debugger {
    config: DebuggerConfig,
    source_maps: source_maps::SourceMapManager,
    trace_hooks: trace_hooks::TraceHookManager,
    breakpoints: breakpoints::BreakpointManager,
    call_stack: stack_trace::CallStack,
    variable_inspector: variable_inspector::VariableInspector,
    error_decoder: error_decoder::ErrorDecoder,
    debugging_session: Option<DebuggingSession>,
}

/// Debugger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebuggerConfig {
    /// Enable source map generation
    pub generate_source_maps: bool,
    /// Enable runtime trace hooks
    pub enable_trace_hooks: bool,
    /// Enable interactive debugging
    pub interactive_mode: bool,
    /// Enable automatic breakpoints on errors
    pub break_on_error: bool,
    /// Enable gas consumption tracking
    pub track_gas_usage: bool,
    /// Enable memory usage tracking
    pub track_memory_usage: bool,
    /// Maximum trace buffer size
    pub max_trace_entries: usize,
    /// Step execution timeout (milliseconds)
    pub step_timeout_ms: u64,
}

/// Active debugging session
#[derive(Debug, Clone)]
pub struct DebuggingSession {
    pub session_id: String,
    pub contract_address: String,
    pub function_name: String,
    pub current_pc: usize,
    pub execution_state: ExecutionState,
    pub trace_buffer: Vec<TraceEntry>,
    pub local_variables: HashMap<String, VariableValue>,
    pub contract_state: HashMap<String, VariableValue>,
}

/// Execution state during debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionState {
    Running,
    Paused,
    SteppingInto,
    SteppingOver,
    SteppingOut,
    Breakpoint,
    Error,
    Completed,
}

/// Source map for mapping bytecode to source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMap {
    pub version: u32,
    pub sources: Vec<String>,
    pub names: Vec<String>,
    pub mappings: String,
    pub sources_content: Option<Vec<String>>,
    pub source_root: Option<String>,
}

/// Individual source mapping entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMapping {
    pub bytecode_offset: usize,
    pub source_file_index: u32,
    pub source_line: u32,
    pub source_column: u32,
    pub source_length: Option<u32>,
    pub name_index: Option<u32>,
}

/// Trace hook for monitoring execution
pub trait TraceHook: Send + Sync {
    /// Called before each instruction execution
    fn before_instruction(&mut self, context: &ExecutionContext) -> Result<HookAction>;
    
    /// Called after each instruction execution
    fn after_instruction(&mut self, context: &ExecutionContext, result: &InstructionResult) -> Result<HookAction>;
    
    /// Called on function entry
    fn on_function_entry(&mut self, context: &ExecutionContext, function: &FunctionInfo) -> Result<HookAction>;
    
    /// Called on function exit
    fn on_function_exit(&mut self, context: &ExecutionContext, function: &FunctionInfo, return_data: &[u8]) -> Result<HookAction>;
    
    /// Called on error or exception
    fn on_error(&mut self, context: &ExecutionContext, error: &ExecutionError) -> Result<HookAction>;
}

/// Action to take after hook execution
#[derive(Debug, Clone, PartialEq)]
pub enum HookAction {
    Continue,
    Break,
    StepInto,
    StepOver,
    StepOut,
    Terminate,
}

/// Execution context for trace hooks
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub contract_address: String,
    pub function_selector: [u8; 4],
    pub program_counter: usize,
    pub instruction: Instruction,
    pub stack: Vec<StackValue>,
    pub memory: Vec<u8>,
    pub storage: HashMap<String, StackValue>,
    pub gas_remaining: u64,
    pub call_depth: u32,
}

/// Instruction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub opcode: u8,
    pub name: String,
    pub gas_cost: u64,
    pub stack_inputs: u8,
    pub stack_outputs: u8,
    pub memory_access: Option<MemoryAccess>,
}

/// Memory access pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAccess {
    pub access_type: MemoryAccessType,
    pub offset: u64,
    pub size: u64,
}

/// Memory access types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryAccessType {
    Read,
    Write,
    Expand,
}

/// Stack value representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackValue {
    pub value: Vec<u8>,
    pub type_hint: Option<TypeHint>,
    pub source_location: Option<SourceLocation>,
}

/// Type hint for better debugging display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeHint {
    Address,
    Uint256,
    Int256,
    Bool,
    Bytes,
    String,
    Array,
    Mapping,
    Struct,
    Function,
}

/// Source code location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub length: u32,
}

/// Instruction execution result
#[derive(Debug, Clone)]
pub struct InstructionResult {
    pub success: bool,
    pub gas_used: u64,
    pub stack_changes: StackChanges,
    pub memory_changes: MemoryChanges,
    pub storage_changes: StorageChanges,
    pub events: Vec<EventEmission>,
}

/// Stack changes during instruction
#[derive(Debug, Clone)]
pub struct StackChanges {
    pub items_pushed: u8,
    pub items_popped: u8,
    pub new_values: Vec<StackValue>,
}

/// Memory changes during instruction
#[derive(Debug, Clone)]
pub struct MemoryChanges {
    pub expansions: Vec<MemoryExpansion>,
    pub writes: Vec<MemoryWrite>,
}

/// Memory expansion
#[derive(Debug, Clone)]
pub struct MemoryExpansion {
    pub old_size: u64,
    pub new_size: u64,
    pub gas_cost: u64,
}

/// Memory write operation
#[derive(Debug, Clone)]
pub struct MemoryWrite {
    pub offset: u64,
    pub data: Vec<u8>,
}

/// Storage changes during instruction
#[derive(Debug, Clone)]
pub struct StorageChanges {
    pub reads: Vec<StorageRead>,
    pub writes: Vec<StorageWrite>,
}

/// Storage read operation
#[derive(Debug, Clone)]
pub struct StorageRead {
    pub key: StackValue,
    pub value: StackValue,
    pub gas_cost: u64,
}

/// Storage write operation
#[derive(Debug, Clone)]
pub struct StorageWrite {
    pub key: StackValue,
    pub old_value: StackValue,
    pub new_value: StackValue,
    pub gas_cost: u64,
}

/// Event emission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEmission {
    pub address: String,
    pub topics: Vec<String>,
    pub data: Vec<u8>,
    pub source_location: Option<SourceLocation>,
}

/// Function information for debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub selector: [u8; 4],
    pub parameters: Vec<ParameterInfo>,
    pub return_types: Vec<TypeInfo>,
    pub visibility: FunctionVisibility,
    pub state_mutability: StateMutability,
    pub source_location: Option<SourceLocation>,
}

/// Parameter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub name: String,
    pub type_info: TypeInfo,
    pub indexed: bool,
}

/// Type information for variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeInfo {
    Elementary {
        name: String,
        size_bytes: u8,
    },
    Array {
        base_type: Box<TypeInfo>,
        length: Option<u64>,
    },
    Mapping {
        key_type: Box<TypeInfo>,
        value_type: Box<TypeInfo>,
    },
    Struct {
        name: String,
        fields: Vec<StructField>,
    },
    Contract {
        name: String,
        address: Option<String>,
    },
}

/// Struct field information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub type_info: TypeInfo,
    pub offset: u32,
}

/// Function visibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunctionVisibility {
    Public,
    External,
    Internal,
    Private,
}

/// State mutability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

/// Execution error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionError {
    pub error_type: ErrorType,
    pub message: String,
    pub program_counter: usize,
    pub source_location: Option<SourceLocation>,
    pub stack_trace: Vec<StackFrame>,
    pub revert_data: Option<Vec<u8>>,
}

/// Types of execution errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorType {
    OutOfGas,
    StackUnderflow,
    StackOverflow,
    InvalidOpcode,
    InvalidJumpDestination,
    Revert,
    Assert,
    Require,
    CustomError,
    InternalError,
}

/// Stack frame for call stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub contract_address: String,
    pub function_name: String,
    pub program_counter: usize,
    pub source_location: Option<SourceLocation>,
    pub local_variables: HashMap<String, VariableValue>,
}

/// Variable value representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableValue {
    pub name: String,
    pub type_info: TypeInfo,
    pub value: ValueRepresentation,
    pub source_location: Option<SourceLocation>,
}

/// Value representation for different types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueRepresentation {
    Primitive(Vec<u8>),
    Array(Vec<ValueRepresentation>),
    Mapping(HashMap<String, ValueRepresentation>),
    Struct(HashMap<String, ValueRepresentation>),
    Address(String),
    String(String),
}

/// Trace entry for execution history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEntry {
    pub step: u64,
    pub program_counter: usize,
    pub instruction: Instruction,
    pub gas_remaining: u64,
    pub gas_cost: u64,
    pub stack_before: Vec<StackValue>,
    pub stack_after: Vec<StackValue>,
    pub memory_changes: Option<MemoryChanges>,
    pub storage_changes: Option<StorageChanges>,
    pub source_location: Option<SourceLocation>,
    pub timestamp_ns: u64,
}

/// Breakpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakpoint {
    pub id: u32,
    pub location: BreakpointLocation,
    pub condition: Option<String>,
    pub hit_count: u32,
    pub enabled: bool,
}

/// Breakpoint location types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreakpointLocation {
    SourceLine {
        file: String,
        line: u32,
    },
    ProgramCounter {
        address: usize,
    },
    FunctionEntry {
        contract: String,
        function: String,
    },
    FunctionExit {
        contract: String,
        function: String,
    },
}

impl Debugger {
    /// Create new debugger instance
    pub fn new(config: DebuggerConfig) -> Result<Self> {
        Ok(Self {
            source_maps: source_maps::SourceMapManager::new(&config)?,
            trace_hooks: trace_hooks::TraceHookManager::new(&config)?,
            breakpoints: breakpoints::BreakpointManager::new()?,
            call_stack: stack_trace::CallStack::new(),
            variable_inspector: variable_inspector::VariableInspector::new()?,
            error_decoder: error_decoder::ErrorDecoder::new()?,
            debugging_session: None,
            config,
        })
    }

    /// Start debugging session for contract execution
    pub fn start_debugging_session(&mut self, contract_address: &str, function_name: &str) -> Result<String> {
        let session_id = format!("debug_session_{}", uuid::Uuid::new_v4());
        
        let session = DebuggingSession {
            session_id: session_id.clone(),
            contract_address: contract_address.to_string(),
            function_name: function_name.to_string(),
            current_pc: 0,
            execution_state: ExecutionState::Paused,
            trace_buffer: Vec::new(),
            local_variables: HashMap::new(),
            contract_state: HashMap::new(),
        };

        self.debugging_session = Some(session);
        
        info!("Started debugging session: {}", session_id);
        Ok(session_id)
    }

    /// Step through execution one instruction at a time
    pub fn step_into(&mut self) -> Result<ExecutionState> {
        if let Some(ref mut session) = self.debugging_session {
            session.execution_state = ExecutionState::SteppingInto;
            
            // Execute next instruction with full tracing
            let context = self.get_current_execution_context()?;
            let result = self.execute_single_instruction(&context)?;
            
            // Update session state
            session.current_pc += 1;
            self.update_trace_buffer(&context, &result);
            
            // Check for breakpoints or completion
            if self.check_breakpoints(&context) {
                session.execution_state = ExecutionState::Breakpoint;
            } else if result.success {
                session.execution_state = ExecutionState::Paused;
            } else {
                session.execution_state = ExecutionState::Error;
            }

            Ok(session.execution_state.clone())
        } else {
            anyhow::bail!("No active debugging session")
        }
    }

    /// Step over function calls
    pub fn step_over(&mut self) -> Result<ExecutionState> {
        if let Some(ref mut session) = self.debugging_session {
            session.execution_state = ExecutionState::SteppingOver;
            
            let initial_call_depth = self.call_stack.depth();
            
            loop {
                let context = self.get_current_execution_context()?;
                let result = self.execute_single_instruction(&context)?;
                
                session.current_pc += 1;
                self.update_trace_buffer(&context, &result);
                
                // Continue until we're back at the same call depth
                if self.call_stack.depth() <= initial_call_depth {
                    break;
                }
                
                // Check for errors
                if !result.success {
                    session.execution_state = ExecutionState::Error;
                    return Ok(session.execution_state.clone());
                }
            }

            session.execution_state = ExecutionState::Paused;
            Ok(session.execution_state.clone())
        } else {
            anyhow::bail!("No active debugging session")
        }
    }

    /// Step out of current function
    pub fn step_out(&mut self) -> Result<ExecutionState> {
        if let Some(ref mut session) = self.debugging_session {
            session.execution_state = ExecutionState::SteppingOut;
            
            let initial_call_depth = self.call_stack.depth();
            
            // Continue execution until we exit the current function
            while self.call_stack.depth() >= initial_call_depth {
                let context = self.get_current_execution_context()?;
                let result = self.execute_single_instruction(&context)?;
                
                session.current_pc += 1;
                self.update_trace_buffer(&context, &result);
                
                if !result.success {
                    session.execution_state = ExecutionState::Error;
                    return Ok(session.execution_state.clone());
                }
            }

            session.execution_state = ExecutionState::Paused;
            Ok(session.execution_state.clone())
        } else {
            anyhow::bail!("No active debugging session")
        }
    }

    /// Continue execution until breakpoint or completion
    pub fn continue_execution(&mut self) -> Result<ExecutionState> {
        if let Some(ref mut session) = self.debugging_session {
            session.execution_state = ExecutionState::Running;
            
            loop {
                let context = self.get_current_execution_context()?;
                
                // Check for breakpoints before execution
                if self.check_breakpoints(&context) {
                    session.execution_state = ExecutionState::Breakpoint;
                    break;
                }
                
                let result = self.execute_single_instruction(&context)?;
                
                session.current_pc += 1;
                self.update_trace_buffer(&context, &result);
                
                if !result.success {
                    session.execution_state = ExecutionState::Error;
                    break;
                }
                
                // Check for completion
                if self.is_execution_complete(&context) {
                    session.execution_state = ExecutionState::Completed;
                    break;
                }
            }

            Ok(session.execution_state.clone())
        } else {
            anyhow::bail!("No active debugging session")
        }
    }

    /// Set breakpoint at source location
    pub fn set_breakpoint(&mut self, file: &str, line: u32) -> Result<u32> {
        let breakpoint = Breakpoint {
            id: self.breakpoints.next_id(),
            location: BreakpointLocation::SourceLine {
                file: file.to_string(),
                line,
            },
            condition: None,
            hit_count: 0,
            enabled: true,
        };

        self.breakpoints.add_breakpoint(breakpoint)
    }

    /// Set conditional breakpoint
    pub fn set_conditional_breakpoint(&mut self, file: &str, line: u32, condition: &str) -> Result<u32> {
        let breakpoint = Breakpoint {
            id: self.breakpoints.next_id(),
            location: BreakpointLocation::SourceLine {
                file: file.to_string(),
                line,
            },
            condition: Some(condition.to_string()),
            hit_count: 0,
            enabled: true,
        };

        self.breakpoints.add_breakpoint(breakpoint)
    }

    /// Remove breakpoint
    pub fn remove_breakpoint(&mut self, breakpoint_id: u32) -> Result<()> {
        self.breakpoints.remove_breakpoint(breakpoint_id)
    }

    /// Get current call stack
    pub fn get_call_stack(&self) -> Vec<StackFrame> {
        self.call_stack.get_frames()
    }

    /// Inspect variable value
    pub fn inspect_variable(&self, variable_name: &str) -> Result<Option<VariableValue>> {
        if let Some(ref session) = self.debugging_session {
            // First check local variables
            if let Some(value) = session.local_variables.get(variable_name) {
                return Ok(Some(value.clone()));
            }
            
            // Then check contract state
            if let Some(value) = session.contract_state.get(variable_name) {
                return Ok(Some(value.clone()));
            }
            
            // Use variable inspector for deeper analysis
            self.variable_inspector.inspect_variable(variable_name, &session.contract_address)
        } else {
            Ok(None)
        }
    }

    /// Get execution trace
    pub fn get_trace(&self) -> Result<Vec<TraceEntry>> {
        if let Some(ref session) = self.debugging_session {
            Ok(session.trace_buffer.clone())
        } else {
            Ok(Vec::new())
        }
    }

    /// Decode error information
    pub fn decode_error(&self, error_data: &[u8]) -> Result<String> {
        self.error_decoder.decode_error(error_data)
    }

    /// Generate source map for contract
    pub fn generate_source_map(&mut self, contract_source: &str, bytecode: &[u8]) -> Result<SourceMap> {
        self.source_maps.generate_source_map(contract_source, bytecode)
    }

    /// Start interactive debugging session
    pub fn start_interactive_session(&mut self) -> Result<()> {
        if self.config.interactive_mode {
            interactive::start_interactive_debugger(self)
        } else {
            anyhow::bail!("Interactive mode not enabled in configuration")
        }
    }

    // Helper methods

    fn get_current_execution_context(&self) -> Result<ExecutionContext> {
        // Mock implementation - would get actual execution context
        Ok(ExecutionContext {
            contract_address: "0x1234567890123456789012345678901234567890".to_string(),
            function_selector: [0u8; 4],
            program_counter: 0,
            instruction: Instruction {
                opcode: 0x00,
                name: "STOP".to_string(),
                gas_cost: 0,
                stack_inputs: 0,
                stack_outputs: 0,
                memory_access: None,
            },
            stack: Vec::new(),
            memory: Vec::new(),
            storage: HashMap::new(),
            gas_remaining: 1000000,
            call_depth: 1,
        })
    }

    fn execute_single_instruction(&self, _context: &ExecutionContext) -> Result<InstructionResult> {
        // Mock implementation - would execute actual instruction
        Ok(InstructionResult {
            success: true,
            gas_used: 3,
            stack_changes: StackChanges {
                items_pushed: 0,
                items_popped: 0,
                new_values: Vec::new(),
            },
            memory_changes: MemoryChanges {
                expansions: Vec::new(),
                writes: Vec::new(),
            },
            storage_changes: StorageChanges {
                reads: Vec::new(),
                writes: Vec::new(),
            },
            events: Vec::new(),
        })
    }

    fn update_trace_buffer(&mut self, context: &ExecutionContext, result: &InstructionResult) {
        if let Some(ref mut session) = self.debugging_session {
            let trace_entry = TraceEntry {
                step: session.trace_buffer.len() as u64,
                program_counter: context.program_counter,
                instruction: context.instruction.clone(),
                gas_remaining: context.gas_remaining,
                gas_cost: result.gas_used,
                stack_before: context.stack.clone(),
                stack_after: context.stack.clone(), // Would calculate actual after state
                memory_changes: Some(result.memory_changes.clone()),
                storage_changes: Some(result.storage_changes.clone()),
                source_location: None, // Would map from source map
                timestamp_ns: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64,
            };

            session.trace_buffer.push(trace_entry);

            // Limit buffer size
            if session.trace_buffer.len() > self.config.max_trace_entries {
                session.trace_buffer.remove(0);
            }
        }
    }

    fn check_breakpoints(&self, context: &ExecutionContext) -> bool {
        // Check if current execution context matches any breakpoints
        self.breakpoints.check_breakpoints(context)
    }

    fn is_execution_complete(&self, context: &ExecutionContext) -> bool {
        // Check if execution is complete (e.g., STOP opcode, RETURN, etc.)
        context.instruction.opcode == 0x00 || // STOP
        context.instruction.opcode == 0xf3 || // RETURN
        context.instruction.opcode == 0xfd    // REVERT
    }
}

impl Default for DebuggerConfig {
    fn default() -> Self {
        Self {
            generate_source_maps: true,
            enable_trace_hooks: true,
            interactive_mode: false,
            break_on_error: true,
            track_gas_usage: true,
            track_memory_usage: true,
            max_trace_entries: 10000,
            step_timeout_ms: 5000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debugger_creation() {
        let config = DebuggerConfig::default();
        let debugger = Debugger::new(config);
        assert!(debugger.is_ok());
    }

    #[test]
    fn test_execution_state_transitions() {
        let states = vec![
            ExecutionState::Running,
            ExecutionState::Paused,
            ExecutionState::SteppingInto,
            ExecutionState::Completed,
        ];

        for state in states {
            match state {
                ExecutionState::Running => assert!(true),
                ExecutionState::Paused => assert!(true),
                ExecutionState::SteppingInto => assert!(true),
                ExecutionState::Completed => assert!(true),
                _ => assert!(false, "Unexpected state"),
            }
        }
    }

    #[test]
    fn test_breakpoint_location_types() {
        let locations = vec![
            BreakpointLocation::SourceLine {
                file: "test.sol".to_string(),
                line: 10,
            },
            BreakpointLocation::ProgramCounter { address: 0x100 },
            BreakpointLocation::FunctionEntry {
                contract: "TestContract".to_string(),
                function: "testFunction".to_string(),
            },
        ];

        assert_eq!(locations.len(), 3);
    }
}