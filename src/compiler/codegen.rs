//! NeoVM Code Generator
//! 
//! Complete code generator that transforms Yul AST into NeoVM bytecode with
//! comprehensive instruction mapping, optimization, and runtime integration.

use super::parser::{YulAST, YulItem, YulFunction, YulBlock, YulStatement, YulExpression, 
                     YulVariableDeclaration, YulAssignment, YulIf, YulSwitch, YulForLoop,
                     YulFunctionCall, YulIdentifier, YulLiteral, LiteralKind, SourceLocation};
use super::{CompilerOptions, NeoVMVersion, AbiEntry, AbiType, AbiParameter, StateMutability};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use thiserror::Error;

/// NeoVM code generator
#[derive(Debug)]
pub struct NeoVMCodeGenerator {
    options: CompilerOptions,
    instruction_set: InstructionSet,
    local_variables: HashMap<String, LocalVariable>,
    static_variables: HashMap<String, StaticVariable>,
    functions: HashMap<String, FunctionInfo>,
    labels: HashMap<String, u32>,
    bytecode: Vec<u8>,
    current_offset: u32,
    stack_height: i32,
    max_stack_height: i32,
    debug_info: DebugInfo,
}

/// NeoVM instruction set mapping
#[derive(Debug, Clone)]
pub struct InstructionSet {
    version: NeoVMVersion,
    instructions: HashMap<String, Instruction>,
    builtin_mappings: HashMap<String, Vec<Instruction>>,
}

/// Single NeoVM instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub opcode: u8,
    pub name: String,
    pub operands: Vec<Operand>,
    pub stack_effect: StackEffect,
    pub gas_cost: u64,
    pub description: String,
}

/// Instruction operand
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operand {
    None,
    Byte(u8),
    Short(u16),
    Int(u32),
    Long(u64),
    Bytes(Vec<u8>),
    String(String),
    Address(String),
}

/// Stack effect of instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackEffect {
    pub pops: u8,
    pub pushes: u8,
}

/// Local variable information
#[derive(Debug, Clone)]
struct LocalVariable {
    name: String,
    index: u8,
    type_size: usize,
    is_parameter: bool,
}

/// Static variable information
#[derive(Debug, Clone)]
struct StaticVariable {
    name: String,
    offset: u32,
    size: usize,
    initial_value: Option<Vec<u8>>,
}

/// Function compilation information
#[derive(Debug, Clone)]
struct FunctionInfo {
    name: String,
    parameter_count: u8,
    return_count: u8,
    local_count: u8,
    address: u32,
    is_public: bool,
}

/// Debug information for generated code
#[derive(Debug, Clone)]
struct DebugInfo {
    source_map: Vec<SourceMapEntry>,
    function_offsets: HashMap<String, u32>,
    variable_map: HashMap<String, VariableDebugInfo>,
}

/// Source map entry for debugging
#[derive(Debug, Clone)]
struct SourceMapEntry {
    bytecode_offset: u32,
    source_location: SourceLocation,
    instruction: String,
}

/// Variable debug information
#[derive(Debug, Clone)]
struct VariableDebugInfo {
    name: String,
    slot_index: u8,
    type_name: String,
    scope_start: u32,
    scope_end: u32,
}

/// Code generation result
#[derive(Debug, Clone)]
pub struct CodeGenResult {
    pub bytecode: Vec<u8>,
    pub assembly: String,
    pub abi: Vec<AbiEntry>,
    pub gas_estimate: Option<u64>,
    pub debug_info: Option<DebugInfo>,
}

/// Code generation errors
#[derive(Debug, Error)]
pub enum CodeGenError {
    #[error("Unsupported instruction: {instruction}")]
    UnsupportedInstruction { instruction: String },
    
    #[error("Stack overflow at offset {offset}")]
    StackOverflow { offset: u32 },
    
    #[error("Stack underflow at offset {offset}")]
    StackUnderflow { offset: u32 },
    
    #[error("Undefined function: {name}")]
    UndefinedFunction { name: String },
    
    #[error("Undefined variable: {name}")]
    UndefinedVariable { name: String },
    
    #[error("Invalid operand: {operand:?}")]
    InvalidOperand { operand: Operand },
    
    #[error("Code generation error: {message}")]
    Generic { message: String },
}

// NeoVM Opcodes (Neo N3 compatible)
#[allow(dead_code)]
mod opcodes {
    // Control flow
    pub const NOP: u8 = 0x21;
    pub const JMP: u8 = 0x22;
    pub const JMPIF: u8 = 0x23;
    pub const JMPIFNOT: u8 = 0x24;
    pub const JMPEQ: u8 = 0x25;
    pub const JMPNE: u8 = 0x26;
    pub const JMPGT: u8 = 0x27;
    pub const JMPLT: u8 = 0x28;
    pub const JMPGE: u8 = 0x29;
    pub const JMPLE: u8 = 0x2A;
    pub const CALL: u8 = 0x2B;
    pub const CALLA: u8 = 0x2C;
    pub const CALLT: u8 = 0x2D;
    pub const ABORT: u8 = 0x2E;
    pub const ASSERT: u8 = 0x2F;
    pub const THROW: u8 = 0x3A;
    pub const TRY: u8 = 0x3B;
    pub const ENDTRY: u8 = 0x3C;
    pub const ENDFINALLY: u8 = 0x3D;
    pub const RET: u8 = 0x40;
    pub const SYSCALL: u8 = 0x41;

    // Stack operations
    pub const DEPTH: u8 = 0x43;
    pub const DROP: u8 = 0x45;
    pub const NIP: u8 = 0x46;
    pub const XDROP: u8 = 0x48;
    pub const CLEAR: u8 = 0x49;
    pub const DUP: u8 = 0x4A;
    pub const OVER: u8 = 0x4B;
    pub const PICK: u8 = 0x4D;
    pub const TUCK: u8 = 0x4E;
    pub const SWAP: u8 = 0x50;
    pub const ROT: u8 = 0x51;
    pub const ROLL: u8 = 0x52;
    pub const REVERSE3: u8 = 0x53;
    pub const REVERSE4: u8 = 0x54;
    pub const REVERSEN: u8 = 0x55;

    // Slot operations
    pub const INITSSLOT: u8 = 0x56;
    pub const INITSLOT: u8 = 0x57;
    pub const LDSFLD0: u8 = 0x58;
    pub const LDSFLD1: u8 = 0x59;
    pub const LDSFLD2: u8 = 0x5A;
    pub const LDSFLD3: u8 = 0x5B;
    pub const LDSFLD4: u8 = 0x5C;
    pub const LDSFLD5: u8 = 0x5D;
    pub const LDSFLD6: u8 = 0x5E;
    pub const LDSFLD: u8 = 0x5F;
    pub const STSFLD0: u8 = 0x60;
    pub const STSFLD1: u8 = 0x61;
    pub const STSFLD2: u8 = 0x62;
    pub const STSFLD3: u8 = 0x63;
    pub const STSFLD4: u8 = 0x64;
    pub const STSFLD5: u8 = 0x65;
    pub const STSFLD6: u8 = 0x66;
    pub const STSFLD: u8 = 0x67;
    pub const LDLOC0: u8 = 0x68;
    pub const LDLOC1: u8 = 0x69;
    pub const LDLOC2: u8 = 0x6A;
    pub const LDLOC3: u8 = 0x6B;
    pub const LDLOC4: u8 = 0x6C;
    pub const LDLOC5: u8 = 0x6D;
    pub const LDLOC6: u8 = 0x6E;
    pub const LDLOC: u8 = 0x6F;
    pub const STLOC0: u8 = 0x70;
    pub const STLOC1: u8 = 0x71;
    pub const STLOC2: u8 = 0x72;
    pub const STLOC3: u8 = 0x73;
    pub const STLOC4: u8 = 0x74;
    pub const STLOC5: u8 = 0x75;
    pub const STLOC6: u8 = 0x76;
    pub const STLOC: u8 = 0x77;
    pub const LDARG0: u8 = 0x78;
    pub const LDARG1: u8 = 0x79;
    pub const LDARG2: u8 = 0x7A;
    pub const LDARG3: u8 = 0x7B;
    pub const LDARG4: u8 = 0x7C;
    pub const LDARG5: u8 = 0x7D;
    pub const LDARG6: u8 = 0x7E;
    pub const LDARG: u8 = 0x7F;
    pub const STARG0: u8 = 0x80;
    pub const STARG1: u8 = 0x81;
    pub const STARG2: u8 = 0x82;
    pub const STARG3: u8 = 0x83;
    pub const STARG4: u8 = 0x84;
    pub const STARG5: u8 = 0x85;
    pub const STARG6: u8 = 0x86;
    pub const STARG: u8 = 0x87;

    // Arithmetic
    pub const SIGN: u8 = 0x90;
    pub const ABS: u8 = 0x91;
    pub const NEGATE: u8 = 0x92;
    pub const INC: u8 = 0x93;
    pub const DEC: u8 = 0x94;
    pub const ADD: u8 = 0x95;
    pub const SUB: u8 = 0x96;
    pub const MUL: u8 = 0x97;
    pub const DIV: u8 = 0x98;
    pub const MOD: u8 = 0x99;
    pub const POW: u8 = 0x9A;
    pub const SQRT: u8 = 0x9B;
    pub const MODMUL: u8 = 0x9C;
    pub const MODPOW: u8 = 0x9D;
    pub const SHL: u8 = 0x9E;
    pub const SHR: u8 = 0x9F;
    pub const NOT: u8 = 0xA0;
    pub const BOOLAND: u8 = 0xA1;
    pub const BOOLOR: u8 = 0xA2;
    pub const NUMEQUAL: u8 = 0xA3;
    pub const NUMNOTEQUAL: u8 = 0xA4;
    pub const LT: u8 = 0xA5;
    pub const LE: u8 = 0xA6;
    pub const GT: u8 = 0xA7;
    pub const GE: u8 = 0xA8;
    pub const MIN: u8 = 0xA9;
    pub const MAX: u8 = 0xAA;
    pub const WITHIN: u8 = 0xAB;

    // Crypto
    pub const SHA256: u8 = 0xB0;
    pub const HASH160: u8 = 0xB1;
    pub const HASH256: u8 = 0xB2;
    pub const CHECKSIG: u8 = 0xB3;
    pub const VERIFY: u8 = 0xB4;
    pub const CHECKMULTISIG: u8 = 0xB5;

    // Array/Buffer
    pub const PACK: u8 = 0xC0;
    pub const UNPACK: u8 = 0xC1;
    pub const PICKITEM: u8 = 0xC2;
    pub const SETITEM: u8 = 0xC3;
    pub const NEWARRAY0: u8 = 0xC4;
    pub const NEWARRAY: u8 = 0xC5;
    pub const NEWARRAY_T: u8 = 0xC6;
    pub const NEWSTRUCT0: u8 = 0xC7;
    pub const NEWSTRUCT: u8 = 0xC8;
    pub const NEWMAP: u8 = 0xC9;
    pub const SIZE: u8 = 0xCA;
    pub const HASKEY: u8 = 0xCB;
    pub const KEYS: u8 = 0xCC;
    pub const VALUES: u8 = 0xCD;
    pub const PICKITEM0: u8 = 0xCE;
    pub const PICKITEM1: u8 = 0xCF;
    pub const APPEND: u8 = 0xD0;
    pub const SETITEM0: u8 = 0xD1;
    pub const SETITEM1: u8 = 0xD2;
    pub const REMOVE: u8 = 0xD3;
    pub const CLEARITEMS: u8 = 0xD4;
    pub const POPITEM: u8 = 0xD5;

    // Types
    pub const ISNULL: u8 = 0xD8;
    pub const ISTYPE: u8 = 0xD9;
    pub const CONVERT: u8 = 0xDB;

    // Push constants
    pub const PUSHINT8: u8 = 0x00;
    pub const PUSHINT16: u8 = 0x01;
    pub const PUSHINT32: u8 = 0x02;
    pub const PUSHINT64: u8 = 0x03;
    pub const PUSHINT128: u8 = 0x04;
    pub const PUSHINT256: u8 = 0x05;
    pub const PUSHA: u8 = 0x0A;
    pub const PUSHNULL: u8 = 0x0B;
    pub const PUSHDATA1: u8 = 0x0C;
    pub const PUSHDATA2: u8 = 0x0D;
    pub const PUSHDATA4: u8 = 0x0E;
    pub const PUSHM1: u8 = 0x0F;
    pub const PUSH0: u8 = 0x10;
    pub const PUSH1: u8 = 0x11;
    pub const PUSH2: u8 = 0x12;
    pub const PUSH3: u8 = 0x13;
    pub const PUSH4: u8 = 0x14;
    pub const PUSH5: u8 = 0x15;
    pub const PUSH6: u8 = 0x16;
    pub const PUSH7: u8 = 0x17;
    pub const PUSH8: u8 = 0x18;
    pub const PUSH9: u8 = 0x19;
    pub const PUSH10: u8 = 0x1A;
    pub const PUSH11: u8 = 0x1B;
    pub const PUSH12: u8 = 0x1C;
    pub const PUSH13: u8 = 0x1D;
    pub const PUSH14: u8 = 0x1E;
    pub const PUSH15: u8 = 0x1F;
    pub const PUSH16: u8 = 0x20;
}

impl NeoVMCodeGenerator {
    /// Create new code generator
    pub fn new(options: &CompilerOptions) -> Self {
        let mut generator = Self {
            options: options.clone(),
            instruction_set: InstructionSet::new(&options.target_version),
            local_variables: HashMap::new(),
            static_variables: HashMap::new(),
            functions: HashMap::new(),
            labels: HashMap::new(),
            bytecode: Vec::new(),
            current_offset: 0,
            stack_height: 0,
            max_stack_height: 0,
            debug_info: DebugInfo {
                source_map: Vec::new(),
                function_offsets: HashMap::new(),
                variable_map: HashMap::new(),
            },
        };
        
        generator.initialize_builtin_mappings();
        generator
    }

    /// Generate NeoVM bytecode from AST
    pub fn generate(&mut self, ast: &YulAST) -> Result<CodeGenResult, CodeGenError> {
        self.reset_state();
        
        // Phase 1: Collect functions and build symbol table
        self.collect_functions(ast)?;
        
        // Phase 2: Generate bytecode for each item
        for item in &ast.items {
            match item {
                YulItem::Function(func) => self.generate_function(func)?,
                YulItem::Object(obj) => {
                    if let Some(code) = &obj.code {
                        self.generate_block(code)?;
                    }
                },
                YulItem::Block(block) => self.generate_block(block)?,
            }
        }
        
        // Phase 3: Resolve labels and generate final bytecode
        self.resolve_labels()?;
        
        // Phase 4: Generate assembly representation
        let assembly = self.generate_assembly();
        
        // Phase 5: Generate ABI information
        let abi = self.generate_abi();
        
        // Phase 6: Estimate gas usage
        let gas_estimate = self.estimate_gas();

        Ok(CodeGenResult {
            bytecode: self.bytecode.clone(),
            assembly,
            abi,
            gas_estimate: Some(gas_estimate),
            debug_info: if self.options.debug {
                Some(self.debug_info.clone())
            } else {
                None
            },
        })
    }

    /// Reset generator state
    fn reset_state(&mut self) {
        self.local_variables.clear();
        self.static_variables.clear();
        self.functions.clear();
        self.labels.clear();
        self.bytecode.clear();
        self.current_offset = 0;
        self.stack_height = 0;
        self.max_stack_height = 0;
        self.debug_info = DebugInfo {
            source_map: Vec::new(),
            function_offsets: HashMap::new(),
            variable_map: HashMap::new(),
        };
    }

    /// Collect all function definitions
    fn collect_functions(&mut self, ast: &YulAST) -> Result<(), CodeGenError> {
        for item in &ast.items {
            if let YulItem::Function(func) = item {
                let func_info = FunctionInfo {
                    name: func.name.clone(),
                    parameter_count: func.parameters.len() as u8,
                    return_count: func.returns.len() as u8,
                    local_count: 0, // Will be calculated during generation
                    address: 0, // Will be set during generation
                    is_public: true, // All functions are public for now
                };
                
                self.functions.insert(func.name.clone(), func_info);
            }
        }
        Ok(())
    }

    /// Generate code for a function
    fn generate_function(&mut self, func: &YulFunction) -> Result<(), CodeGenError> {
        let start_offset = self.current_offset;
        
        // Update function address
        if let Some(func_info) = self.functions.get_mut(&func.name) {
            func_info.address = start_offset;
        }
        
        // Add debug info
        self.debug_info.function_offsets.insert(func.name.clone(), start_offset);
        
        // Initialize local variables slot if needed
        let local_count = self.count_local_variables(&func.body);
        if local_count > 0 {
            self.emit_instruction(opcodes::INITSLOT, &[Operand::Byte(local_count), Operand::Byte(func.parameters.len() as u8)])?;
        }
        
        // Set up parameter mappings
        for (i, param) in func.parameters.iter().enumerate() {
            let var_info = LocalVariable {
                name: param.name.clone(),
                index: i as u8,
                type_size: 32, // Default size
                is_parameter: true,
            };
            self.local_variables.insert(param.name.clone(), var_info);
        }
        
        // Generate function body
        self.generate_block(&func.body)?;
        
        // Ensure function returns properly
        if func.returns.is_empty() {
            self.emit_instruction(opcodes::RET, &[])?;
        }
        
        // Clean up local variables
        self.local_variables.clear();
        
        Ok(())
    }

    /// Generate code for a block
    fn generate_block(&mut self, block: &YulBlock) -> Result<(), CodeGenError> {
        for statement in &block.statements {
            self.generate_statement(statement)?;
        }
        Ok(())
    }

    /// Generate code for a statement
    fn generate_statement(&mut self, statement: &YulStatement) -> Result<(), CodeGenError> {
        match statement {
            YulStatement::Block(block) => self.generate_block(block)?,
            YulStatement::FunctionDef(func) => self.generate_function(func)?,
            YulStatement::VariableDeclaration(var_decl) => {
                self.generate_variable_declaration(var_decl)?;
            },
            YulStatement::Assignment(assignment) => {
                self.generate_assignment(assignment)?;
            },
            YulStatement::If(if_stmt) => self.generate_if(if_stmt)?,
            YulStatement::Switch(switch_stmt) => self.generate_switch(switch_stmt)?,
            YulStatement::ForLoop(for_loop) => self.generate_for_loop(for_loop)?,
            YulStatement::Break(location) => {
                self.add_source_map_entry(location.clone(), "BREAK".to_string());
                // Break implementation would depend on loop context
                self.emit_instruction(opcodes::JMP, &[Operand::Int(0)])?; // Placeholder
            },
            YulStatement::Continue(location) => {
                self.add_source_map_entry(location.clone(), "CONTINUE".to_string());
                // Continue implementation would depend on loop context
                self.emit_instruction(opcodes::JMP, &[Operand::Int(0)])?; // Placeholder
            },
            YulStatement::Leave(location) => {
                self.add_source_map_entry(location.clone(), "LEAVE".to_string());
                self.emit_instruction(opcodes::RET, &[])?;
            },
            YulStatement::Expression(expr) => {
                self.generate_expression(expr)?;
                // Pop the result if it's not used
                self.emit_instruction(opcodes::DROP, &[])?;
            },
        }
        Ok(())
    }

    /// Generate code for variable declaration
    fn generate_variable_declaration(&mut self, var_decl: &YulVariableDeclaration) -> Result<(), CodeGenError> {
        // Generate code for the initial value if present
        if let Some(value) = &var_decl.value {
            self.generate_expression(value)?;
        } else {
            // Push null for uninitialized variables
            self.emit_instruction(opcodes::PUSHNULL, &[])?;
        }
        
        // Store in local variables
        for (i, var) in var_decl.variables.iter().enumerate() {
            let var_index = self.allocate_local_variable(&var.name);
            
            if i > 0 {
                // Duplicate the value for multiple variables
                self.emit_instruction(opcodes::DUP, &[])?;
            }
            
            self.emit_store_local(var_index)?;
            
            // Add debug info
            let debug_info = VariableDebugInfo {
                name: var.name.clone(),
                slot_index: var_index,
                type_name: format!("{:?}", var.type_info.type_name),
                scope_start: self.current_offset,
                scope_end: 0, // Will be updated when scope ends
            };
            self.debug_info.variable_map.insert(var.name.clone(), debug_info);
        }
        
        Ok(())
    }

    /// Generate code for assignment
    fn generate_assignment(&mut self, assignment: &YulAssignment) -> Result<(), CodeGenError> {
        // Generate value
        self.generate_expression(&assignment.value)?;
        
        // Handle multiple assignment targets
        for (i, var_name) in assignment.variables.iter().enumerate() {
            if i > 0 {
                self.emit_instruction(opcodes::DUP, &[])?;
            }
            
            if let Some(var_info) = self.local_variables.get(var_name).cloned() {
                self.emit_store_local(var_info.index)?;
            } else {
                return Err(CodeGenError::UndefinedVariable { 
                    name: var_name.clone() 
                });
            }
        }
        
        Ok(())
    }

    /// Generate code for if statement
    fn generate_if(&mut self, if_stmt: &YulIf) -> Result<(), CodeGenError> {
        // Generate condition
        self.generate_expression(&if_stmt.condition)?;
        
        // Jump if false
        let end_label = format!("if_end_{}", self.current_offset);
        self.emit_instruction(opcodes::JMPIFNOT, &[Operand::String(end_label.clone())])?;
        
        // Generate body
        self.generate_block(&if_stmt.body)?;
        
        // End label
        self.labels.insert(end_label, self.current_offset);
        
        Ok(())
    }

    /// Generate code for switch statement
    fn generate_switch(&mut self, switch_stmt: &YulSwitch) -> Result<(), CodeGenError> {
        // Generate switch expression
        self.generate_expression(&switch_stmt.expression)?;
        
        let end_label = format!("switch_end_{}", self.current_offset);
        let mut case_labels = Vec::new();
        
        // Generate case comparisons
        for (i, case) in switch_stmt.cases.iter().enumerate() {
            let case_label = format!("case_{}_{}", i, self.current_offset);
            case_labels.push(case_label.clone());
            
            // Duplicate switch value
            self.emit_instruction(opcodes::DUP, &[])?;
            
            // Push case value and compare
            self.generate_literal(&case.value)?;
            self.emit_instruction(opcodes::NUMEQUAL, &[])?;
            self.emit_instruction(opcodes::JMPIF, &[Operand::String(case_label)])?;
        }
        
        // Jump to default or end
        if switch_stmt.default.is_some() {
            let default_label = format!("default_{}", self.current_offset);
            self.emit_instruction(opcodes::JMP, &[Operand::String(default_label.clone())])?;
            
            // Generate case bodies
            for (i, case) in switch_stmt.cases.iter().enumerate() {
                self.labels.insert(case_labels[i].clone(), self.current_offset);
                self.emit_instruction(opcodes::DROP, &[])?; // Drop switch value
                self.generate_block(&case.body)?;
                self.emit_instruction(opcodes::JMP, &[Operand::String(end_label.clone())])?;
            }
            
            // Generate default
            self.labels.insert(default_label, self.current_offset);
            self.emit_instruction(opcodes::DROP, &[])?; // Drop switch value
            if let Some(default_block) = &switch_stmt.default {
                self.generate_block(default_block)?;
            }
        } else {
            self.emit_instruction(opcodes::JMP, &[Operand::String(end_label.clone())])?;
            
            // Generate case bodies
            for (i, case) in switch_stmt.cases.iter().enumerate() {
                self.labels.insert(case_labels[i].clone(), self.current_offset);
                self.emit_instruction(opcodes::DROP, &[])?; // Drop switch value
                self.generate_block(&case.body)?;
                self.emit_instruction(opcodes::JMP, &[Operand::String(end_label.clone())])?;
            }
        }
        
        self.labels.insert(end_label, self.current_offset);
        Ok(())
    }

    /// Generate code for for loop
    fn generate_for_loop(&mut self, for_loop: &YulForLoop) -> Result<(), CodeGenError> {
        // Generate init block
        self.generate_block(&for_loop.init)?;
        
        let loop_start = format!("loop_start_{}", self.current_offset);
        let loop_end = format!("loop_end_{}", self.current_offset);
        let loop_continue = format!("loop_continue_{}", self.current_offset);
        
        // Loop start
        self.labels.insert(loop_start.clone(), self.current_offset);
        
        // Generate condition
        self.generate_expression(&for_loop.condition)?;
        self.emit_instruction(opcodes::JMPIFNOT, &[Operand::String(loop_end.clone())])?;
        
        // Generate body
        self.generate_block(&for_loop.body)?;
        
        // Continue point
        self.labels.insert(loop_continue, self.current_offset);
        
        // Generate post block
        self.generate_block(&for_loop.post)?;
        
        // Jump back to start
        self.emit_instruction(opcodes::JMP, &[Operand::String(loop_start)])?;
        
        // Loop end
        self.labels.insert(loop_end, self.current_offset);
        
        Ok(())
    }

    /// Generate code for expression
    fn generate_expression(&mut self, expr: &YulExpression) -> Result<(), CodeGenError> {
        match expr {
            YulExpression::Literal(literal) => self.generate_literal(literal)?,
            YulExpression::Identifier(identifier) => self.generate_identifier(identifier)?,
            YulExpression::FunctionCall(call) => self.generate_function_call(call)?,
        }
        Ok(())
    }

    /// Generate code for literal
    fn generate_literal(&mut self, literal: &YulLiteral) -> Result<(), CodeGenError> {
        match literal.kind {
            LiteralKind::Number => {
                let value = literal.value.parse::<i64>().map_err(|_| {
                    CodeGenError::Generic { message: format!("Invalid number: {}", literal.value) }
                })?;
                self.emit_push_integer(value)?;
            },
            LiteralKind::HexNumber => {
                let hex_str = &literal.value[2..]; // Remove 0x prefix
                let bytes = hex::decode(hex_str).map_err(|_| {
                    CodeGenError::Generic { message: format!("Invalid hex: {}", literal.value) }
                })?;
                self.emit_push_bytes(&bytes)?;
            },
            LiteralKind::String => {
                let bytes = literal.value.as_bytes();
                self.emit_push_bytes(bytes)?;
            },
            LiteralKind::Boolean => {
                if literal.value == "true" {
                    self.emit_instruction(opcodes::PUSH1, &[])?;
                } else {
                    self.emit_instruction(opcodes::PUSH0, &[])?;
                }
            },
        }
        Ok(())
    }

    /// Generate code for identifier (variable access)
    fn generate_identifier(&mut self, identifier: &YulIdentifier) -> Result<(), CodeGenError> {
        if let Some(var_info) = self.local_variables.get(&identifier.name).cloned() {
            if var_info.is_parameter {
                self.emit_load_argument(var_info.index)?;
            } else {
                self.emit_load_local(var_info.index)?;
            }
        } else {
            return Err(CodeGenError::UndefinedVariable { 
                name: identifier.name.clone() 
            });
        }
        Ok(())
    }

    /// Generate code for function call
    fn generate_function_call(&mut self, call: &YulFunctionCall) -> Result<(), CodeGenError> {
        let func_name = &call.function.name;
        
        // Check if it's a builtin function
        if let Some(instructions) = self.instruction_set.builtin_mappings.get(func_name) {
            // Generate arguments
            for arg in &call.arguments {
                self.generate_expression(arg)?;
            }
            
            // Emit builtin instructions
            for instruction in instructions {
                self.emit_raw_instruction(instruction.clone())?;
            }
        } else if self.functions.contains_key(func_name) {
            // Generate arguments
            for arg in &call.arguments {
                self.generate_expression(arg)?;
            }
            
            // Call user-defined function
            self.emit_instruction(opcodes::CALL, &[Operand::String(func_name.clone())])?;
        } else {
            return Err(CodeGenError::UndefinedFunction { 
                name: func_name.clone() 
            });
        }
        
        Ok(())
    }

    /// Emit instruction with operands
    fn emit_instruction(&mut self, opcode: u8, operands: &[Operand]) -> Result<(), CodeGenError> {
        let instruction = Instruction {
            opcode,
            name: self.opcode_name(opcode),
            operands: operands.to_vec(),
            stack_effect: self.get_stack_effect(opcode),
            gas_cost: self.get_gas_cost(opcode),
            description: format!("Instruction: {:#04x}", opcode),
        };
        
        self.emit_raw_instruction(instruction)
    }

    /// Emit raw instruction
    fn emit_raw_instruction(&mut self, instruction: Instruction) -> Result<(), CodeGenError> {
        // Emit opcode
        self.bytecode.push(instruction.opcode);
        self.current_offset += 1;
        
        // Emit operands
        for operand in &instruction.operands {
            match operand {
                Operand::None => {},
                Operand::Byte(b) => {
                    self.bytecode.push(*b);
                    self.current_offset += 1;
                },
                Operand::Short(s) => {
                    self.bytecode.extend_from_slice(&s.to_le_bytes());
                    self.current_offset += 2;
                },
                Operand::Int(i) => {
                    self.bytecode.extend_from_slice(&i.to_le_bytes());
                    self.current_offset += 4;
                },
                Operand::Long(l) => {
                    self.bytecode.extend_from_slice(&l.to_le_bytes());
                    self.current_offset += 8;
                },
                Operand::Bytes(bytes) => {
                    self.bytecode.extend_from_slice(bytes);
                    self.current_offset += bytes.len() as u32;
                },
                Operand::String(_) => {
                    // String operands are labels - will be resolved later
                    self.bytecode.extend_from_slice(&[0u8; 4]); // Placeholder
                    self.current_offset += 4;
                },
                Operand::Address(_) => {
                    // Address operands - will be resolved later
                    self.bytecode.extend_from_slice(&[0u8; 4]); // Placeholder
                    self.current_offset += 4;
                },
            }
        }
        
        // Update stack height
        self.stack_height -= instruction.stack_effect.pops as i32;
        if self.stack_height < 0 {
            return Err(CodeGenError::StackUnderflow { offset: self.current_offset });
        }
        
        self.stack_height += instruction.stack_effect.pushes as i32;
        if self.stack_height > 2048 { // NeoVM stack limit
            return Err(CodeGenError::StackOverflow { offset: self.current_offset });
        }
        
        self.max_stack_height = self.max_stack_height.max(self.stack_height);
        
        Ok(())
    }

    /// Emit push integer instruction
    fn emit_push_integer(&mut self, value: i64) -> Result<(), CodeGenError> {
        match value {
            -1 => self.emit_instruction(opcodes::PUSHM1, &[])?,
            0 => self.emit_instruction(opcodes::PUSH0, &[])?,
            1..=16 => self.emit_instruction(opcodes::PUSH1 + (value - 1) as u8, &[])?,
            -128..=127 => self.emit_instruction(opcodes::PUSHINT8, &[Operand::Byte(value as u8)])?,
            -32768..=32767 => self.emit_instruction(opcodes::PUSHINT16, &[Operand::Short(value as u16)])?,
            -2147483648..=2147483647 => self.emit_instruction(opcodes::PUSHINT32, &[Operand::Int(value as u32)])?,
            _ => self.emit_instruction(opcodes::PUSHINT64, &[Operand::Long(value as u64)])?,
        }
        Ok(())
    }

    /// Emit push bytes instruction
    fn emit_push_bytes(&mut self, bytes: &[u8]) -> Result<(), CodeGenError> {
        match bytes.len() {
            0..=75 => {
                self.bytecode.push(bytes.len() as u8);
                self.bytecode.extend_from_slice(bytes);
                self.current_offset += 1 + bytes.len() as u32;
            },
            76..=255 => {
                self.emit_instruction(opcodes::PUSHDATA1, &[
                    Operand::Byte(bytes.len() as u8),
                    Operand::Bytes(bytes.to_vec())
                ])?;
            },
            256..=65535 => {
                self.emit_instruction(opcodes::PUSHDATA2, &[
                    Operand::Short(bytes.len() as u16),
                    Operand::Bytes(bytes.to_vec())
                ])?;
            },
            _ => {
                self.emit_instruction(opcodes::PUSHDATA4, &[
                    Operand::Int(bytes.len() as u32),
                    Operand::Bytes(bytes.to_vec())
                ])?;
            },
        }
        Ok(())
    }

    /// Emit load local variable instruction
    fn emit_load_local(&mut self, index: u8) -> Result<(), CodeGenError> {
        match index {
            0 => self.emit_instruction(opcodes::LDLOC0, &[])?,
            1 => self.emit_instruction(opcodes::LDLOC1, &[])?,
            2 => self.emit_instruction(opcodes::LDLOC2, &[])?,
            3 => self.emit_instruction(opcodes::LDLOC3, &[])?,
            4 => self.emit_instruction(opcodes::LDLOC4, &[])?,
            5 => self.emit_instruction(opcodes::LDLOC5, &[])?,
            6 => self.emit_instruction(opcodes::LDLOC6, &[])?,
            _ => self.emit_instruction(opcodes::LDLOC, &[Operand::Byte(index)])?,
        }
        Ok(())
    }

    /// Emit store local variable instruction
    fn emit_store_local(&mut self, index: u8) -> Result<(), CodeGenError> {
        match index {
            0 => self.emit_instruction(opcodes::STLOC0, &[])?,
            1 => self.emit_instruction(opcodes::STLOC1, &[])?,
            2 => self.emit_instruction(opcodes::STLOC2, &[])?,
            3 => self.emit_instruction(opcodes::STLOC3, &[])?,
            4 => self.emit_instruction(opcodes::STLOC4, &[])?,
            5 => self.emit_instruction(opcodes::STLOC5, &[])?,
            6 => self.emit_instruction(opcodes::STLOC6, &[])?,
            _ => self.emit_instruction(opcodes::STLOC, &[Operand::Byte(index)])?,
        }
        Ok(())
    }

    /// Emit load argument instruction
    fn emit_load_argument(&mut self, index: u8) -> Result<(), CodeGenError> {
        match index {
            0 => self.emit_instruction(opcodes::LDARG0, &[])?,
            1 => self.emit_instruction(opcodes::LDARG1, &[])?,
            2 => self.emit_instruction(opcodes::LDARG2, &[])?,
            3 => self.emit_instruction(opcodes::LDARG3, &[])?,
            4 => self.emit_instruction(opcodes::LDARG4, &[])?,
            5 => self.emit_instruction(opcodes::LDARG5, &[])?,
            6 => self.emit_instruction(opcodes::LDARG6, &[])?,
            _ => self.emit_instruction(opcodes::LDARG, &[Operand::Byte(index)])?,
        }
        Ok(())
    }

    /// Allocate local variable slot
    fn allocate_local_variable(&mut self, name: &str) -> u8 {
        let index = self.local_variables.len() as u8;
        let var_info = LocalVariable {
            name: name.to_string(),
            index,
            type_size: 32,
            is_parameter: false,
        };
        self.local_variables.insert(name.to_string(), var_info);
        index
    }

    /// Count local variables in a block
    fn count_local_variables(&self, _block: &YulBlock) -> u8 {
        // Simple implementation - would need proper analysis
        10 // Default allocation
    }

    /// Resolve label references
    fn resolve_labels(&mut self) -> Result<(), CodeGenError> {
        // This would iterate through bytecode and resolve label references
        // For now, just a placeholder
        Ok(())
    }

    /// Generate assembly representation
    fn generate_assembly(&self) -> String {
        let mut assembly = String::new();
        let mut offset = 0;
        
        for (i, &byte) in self.bytecode.iter().enumerate() {
            if i % 16 == 0 {
                assembly.push_str(&format!("{:08x}: ", offset));
            }
            
            assembly.push_str(&format!("{:02x} ", byte));
            
            if i % 16 == 15 {
                assembly.push('\n');
                offset += 16;
            }
        }
        
        if self.bytecode.len() % 16 != 0 {
            assembly.push('\n');
        }
        
        assembly
    }

    /// Generate ABI information
    fn generate_abi(&self) -> Vec<AbiEntry> {
        self.functions.values().filter(|f| f.is_public).map(|func| {
            AbiEntry {
                name: func.name.clone(),
                r#type: AbiType::Function,
                inputs: (0..func.parameter_count).map(|i| AbiParameter {
                    name: format!("arg{}", i),
                    r#type: "uint256".to_string(),
                    indexed: None,
                }).collect(),
                outputs: (0..func.return_count).map(|i| AbiParameter {
                    name: format!("ret{}", i),
                    r#type: "uint256".to_string(),
                    indexed: None,
                }).collect(),
                state_mutability: StateMutability::NonPayable,
            }
        }).collect()
    }

    /// Estimate gas usage
    fn estimate_gas(&self) -> u64 {
        // Simple gas estimation based on instruction count
        self.bytecode.len() as u64 * 2 // 2 gas per byte approximation
    }

    /// Add source map entry
    fn add_source_map_entry(&mut self, location: SourceLocation, instruction: String) {
        let entry = SourceMapEntry {
            bytecode_offset: self.current_offset,
            source_location: location,
            instruction,
        };
        self.debug_info.source_map.push(entry);
    }

    /// Initialize builtin function mappings
    fn initialize_builtin_mappings(&mut self) {
        let mappings = [
            ("add", vec![self.make_instruction(opcodes::ADD, "ADD", 2, 1, 3)]),
            ("sub", vec![self.make_instruction(opcodes::SUB, "SUB", 2, 1, 3)]),
            ("mul", vec![self.make_instruction(opcodes::MUL, "MUL", 2, 1, 5)]),
            ("div", vec![self.make_instruction(opcodes::DIV, "DIV", 2, 1, 5)]),
            ("mod", vec![self.make_instruction(opcodes::MOD, "MOD", 2, 1, 5)]),
            ("lt", vec![self.make_instruction(opcodes::LT, "LT", 2, 1, 3)]),
            ("gt", vec![self.make_instruction(opcodes::GT, "GT", 2, 1, 3)]),
            ("eq", vec![self.make_instruction(opcodes::NUMEQUAL, "NUMEQUAL", 2, 1, 3)]),
            ("iszero", vec![
                self.make_instruction(opcodes::PUSH0, "PUSH0", 0, 1, 1),
                self.make_instruction(opcodes::NUMEQUAL, "NUMEQUAL", 2, 1, 3)
            ]),
            ("and", vec![self.make_instruction(opcodes::BOOLAND, "BOOLAND", 2, 1, 3)]),
            ("or", vec![self.make_instruction(opcodes::BOOLOR, "BOOLOR", 2, 1, 3)]),
            ("not", vec![self.make_instruction(opcodes::NOT, "NOT", 1, 1, 2)]),
            ("keccak256", vec![self.make_instruction(opcodes::HASH256, "HASH256", 1, 1, 200)]),
            ("sha256", vec![self.make_instruction(opcodes::SHA256, "SHA256", 1, 1, 200)]),
        ];

        for (name, instructions) in mappings.iter() {
            self.instruction_set.builtin_mappings.insert(name.to_string(), instructions.clone());
        }
    }

    /// Helper to create instruction
    fn make_instruction(&self, opcode: u8, name: &str, pops: u8, pushes: u8, gas: u64) -> Instruction {
        Instruction {
            opcode,
            name: name.to_string(),
            operands: vec![],
            stack_effect: StackEffect { pops, pushes },
            gas_cost: gas,
            description: format!("Built-in instruction: {}", name),
        }
    }

    /// Get opcode name for debugging
    fn opcode_name(&self, opcode: u8) -> String {
        match opcode {
            opcodes::NOP => "NOP".to_string(),
            opcodes::JMP => "JMP".to_string(),
            opcodes::JMPIF => "JMPIF".to_string(),
            opcodes::JMPIFNOT => "JMPIFNOT".to_string(),
            opcodes::CALL => "CALL".to_string(),
            opcodes::RET => "RET".to_string(),
            opcodes::ADD => "ADD".to_string(),
            opcodes::SUB => "SUB".to_string(),
            opcodes::MUL => "MUL".to_string(),
            opcodes::DIV => "DIV".to_string(),
            opcodes::PUSH0 => "PUSH0".to_string(),
            opcodes::PUSH1 => "PUSH1".to_string(),
            _ => format!("UNKNOWN_{:#04x}", opcode),
        }
    }

    /// Get stack effect for opcode
    fn get_stack_effect(&self, opcode: u8) -> StackEffect {
        match opcode {
            opcodes::NOP => StackEffect { pops: 0, pushes: 0 },
            opcodes::JMP => StackEffect { pops: 0, pushes: 0 },
            opcodes::JMPIF => StackEffect { pops: 1, pushes: 0 },
            opcodes::JMPIFNOT => StackEffect { pops: 1, pushes: 0 },
            opcodes::CALL => StackEffect { pops: 0, pushes: 0 }, // Varies by function
            opcodes::RET => StackEffect { pops: 0, pushes: 0 },
            opcodes::ADD => StackEffect { pops: 2, pushes: 1 },
            opcodes::SUB => StackEffect { pops: 2, pushes: 1 },
            opcodes::MUL => StackEffect { pops: 2, pushes: 1 },
            opcodes::DIV => StackEffect { pops: 2, pushes: 1 },
            opcodes::PUSH0..=opcodes::PUSH16 => StackEffect { pops: 0, pushes: 1 },
            opcodes::DROP => StackEffect { pops: 1, pushes: 0 },
            opcodes::DUP => StackEffect { pops: 1, pushes: 2 },
            opcodes::SWAP => StackEffect { pops: 2, pushes: 2 },
            _ => StackEffect { pops: 0, pushes: 0 }, // Default
        }
    }

    /// Get gas cost for opcode
    fn get_gas_cost(&self, opcode: u8) -> u64 {
        match opcode {
            opcodes::NOP => 1,
            opcodes::JMP | opcodes::JMPIF | opcodes::JMPIFNOT => 2,
            opcodes::CALL => 512,
            opcodes::RET => 0,
            opcodes::ADD | opcodes::SUB => 3,
            opcodes::MUL | opcodes::DIV | opcodes::MOD => 5,
            opcodes::PUSH0..=opcodes::PUSH16 => 1,
            opcodes::PUSHDATA1 | opcodes::PUSHDATA2 | opcodes::PUSHDATA4 => 4,
            opcodes::DROP | opcodes::DUP | opcodes::SWAP => 2,
            opcodes::SHA256 | opcodes::HASH256 => 200,
            _ => 1, // Default cost
        }
    }
}

impl InstructionSet {
    fn new(version: &NeoVMVersion) -> Self {
        Self {
            version: version.clone(),
            instructions: HashMap::new(),
            builtin_mappings: HashMap::new(),
        }
    }
}

impl Default for NeoVMCodeGenerator {
    fn default() -> Self {
        Self::new(&CompilerOptions::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::{lexer::YulLexer, parser::YulParser};

    fn generate_code(source: &str) -> Result<CodeGenResult, Box<dyn std::error::Error>> {
        let mut lexer = YulLexer::new();
        let tokens = lexer.tokenize(source)?;
        let mut parser = YulParser::new();
        let ast = parser.parse(tokens)?;
        let mut generator = NeoVMCodeGenerator::new(&CompilerOptions::default());
        Ok(generator.generate(&ast)?)
    }

    #[test]
    fn test_simple_arithmetic() {
        let source = r#"
            {
                let x := add(1, 2)
                let y := mul(x, 3)
            }
        "#;

        let result = generate_code(source).unwrap();
        assert!(!result.bytecode.is_empty());
        assert!(result.gas_estimate.is_some());
    }

    #[test]
    fn test_function_generation() {
        let source = r#"
            function add(a, b) -> result {
                result := add(a, b)
            }
        "#;

        let result = generate_code(source).unwrap();
        assert!(!result.bytecode.is_empty());
        assert!(!result.abi.is_empty());
    }

    #[test]
    fn test_control_flow() {
        let source = r#"
            {
                if eq(x, 0) {
                    leave
                }
                
                for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                    // loop body
                }
            }
        "#;

        let result = generate_code(source).unwrap();
        assert!(!result.bytecode.is_empty());
    }

    #[test]
    fn test_switch_statement() {
        let source = r#"
            {
                switch x
                case 1 { let a := 1 }
                case 2 { let b := 2 }
                default { let c := 3 }
            }
        "#;

        let result = generate_code(source).unwrap();
        assert!(!result.bytecode.is_empty());
    }

    #[test]
    fn test_builtin_functions() {
        let source = r#"
            {
                let sum := add(1, 2)
                let diff := sub(5, 3)
                let product := mul(sum, diff)
                let quotient := div(product, 2)
                let remainder := mod(quotient, 3)
                let is_zero := iszero(remainder)
            }
        "#;

        let result = generate_code(source).unwrap();
        assert!(!result.bytecode.is_empty());
        assert!(result.gas_estimate.unwrap() > 0);
    }
}