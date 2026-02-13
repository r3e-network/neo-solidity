/// Result of compiling Yul source to NeoVM bytecode
#[derive(Debug, Clone)]
pub struct CompilationResult {
    pub bytecode: Vec<u8>,
    pub assembly: String,
    pub abi: serde_json::Value,
    pub manifest: serde_json::Value,
    pub estimated_gas: u64,
    pub source_map: String,
    pub debug_info: serde_json::Value,
}

impl CompilationResult {
    /// Get bytecode as hex string
    pub fn bytecode_hex(&self) -> String {
        hex::encode(&self.bytecode)
    }

    /// Get bytecode size in bytes
    pub fn bytecode_size(&self) -> usize {
        self.bytecode.len()
    }

    /// Check if compilation produced any bytecode
    pub fn is_empty(&self) -> bool {
        self.bytecode.is_empty()
    }
}

/// Metadata for a compiled function, collected during bytecode generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionMeta {
    pub name: String,
    pub params: Vec<String>,
    pub returns: Vec<String>,
    /// Bytecode offset before the init-marker prefix is inserted.
    /// The final offset in the emitted binary is `raw_offset + INIT_MARKER_LEN`.
    pub raw_offset: usize,
}

/// Length of the init marker prepended to bytecode (PUSHDATA1 + len + b"init").
pub const INIT_MARKER_LEN: usize = 6;

/// Loop context for break/continue tracking
#[derive(Debug, Clone)]
pub struct LoopContext {
    /// Bytecode position to jump to for break (loop end)
    pub break_target: usize,
    /// Bytecode position to jump to for continue (loop start)
    pub continue_target: usize,
}

/// Variable information for code generation
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub index: usize,
    pub slot: usize,
}

/// Code generator for Yul AST to NeoVM bytecode
pub struct CodeGenerator {
    _config: CompilerConfig,
    /// Stack of loop contexts for break/continue resolution
    loop_stack: Vec<LoopContext>,
    /// Variable table for tracking local variables
    variable_table: Vec<VariableInfo>,
    /// Current variable index
    next_var_index: usize,
}

impl CodeGenerator {
    /// Create a new code generator with default loop stack
    pub fn new() -> Self {
        Self {
            _config: CompilerConfig::default(),
            loop_stack: Vec::new(),
            variable_table: Vec::new(),
            next_var_index: 0,
        }
    }

    /// Create a new code generator with config
    pub fn with_config(config: &CompilerConfig) -> Self {
        Self {
            _config: config.clone(),
            loop_stack: Vec::new(),
            variable_table: Vec::new(),
            next_var_index: 0,
        }
    }

    /// Push a loop context onto the stack
    pub fn push_loop(&mut self, ctx: LoopContext) {
        self.loop_stack.push(ctx);
    }

    /// Pop a loop context from the stack
    pub fn pop_loop(&mut self) -> Option<LoopContext> {
        self.loop_stack.pop()
    }

    /// Get the current loop context (for break/continue)
    pub fn current_loop(&self) -> Option<&LoopContext> {
        self.loop_stack.last()
    }

    /// Register a new variable and return its index
    pub fn register_variable(&mut self, name: &str) -> usize {
        let index = self.next_var_index;
        self.variable_table.push(VariableInfo {
            name: name.to_string(),
            index,
            slot: index,
        });
        self.next_var_index += 1;
        index
    }

    /// Get variable index by name
    pub fn get_variable_index(&self, name: &str) -> Option<usize> {
        self.variable_table
            .iter()
            .find(|v| v.name == name)
            .map(|v| v.index)
    }

    /// Reset variable table for new function
    pub fn reset_variables(&mut self) {
        self.variable_table.clear();
        self.next_var_index = 0;
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}
