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

/// Code generator for Yul AST to NeoVM bytecode
pub struct CodeGenerator {
    _config: CompilerConfig,
}
