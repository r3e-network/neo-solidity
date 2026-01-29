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

/// NeoVM opcode constants
pub mod opcodes {
    pub const PUSHDATA1: u8 = 0x0C;
    pub const PUSH0: u8 = 0x10;
    pub const PUSH1: u8 = 0x11;
    pub const RET: u8 = 0x40;
    pub const SYSCALL: u8 = 0x41;
    pub const JMP: u8 = 0x22;
    pub const JMPIFNOT: u8 = 0x23;
    pub const ADD: u8 = 0x9E;
    pub const SUB: u8 = 0x9F;
    pub const MUL: u8 = 0xA0;
    pub const DIV: u8 = 0xA1;
}

/// Gas costs for common operations
pub mod gas {
    pub const PUSH: u64 = 3;
    pub const ARITHMETIC: u64 = 3;
    pub const MUL_DIV: u64 = 5;
    pub const JUMP: u64 = 10;
    pub const FUNCTION_CALL: u64 = 50;
    pub const STORAGE_GET: u64 = 800;
    pub const STORAGE_PUT: u64 = 20000;
    pub const CRYPTO_HASH: u64 = 30;
    pub const CONTRACT_CALL: u64 = 1000;
}

/// Code generator for Yul AST to NeoVM bytecode
pub struct CodeGenerator {
    _config: CompilerConfig,
}
