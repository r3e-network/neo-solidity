/// IR module representing a compiled contract
#[derive(Debug)]
pub struct Module {
    pub functions: Vec<Function>,
    pub state_variables: Vec<StateVariable>,
    pub events: Vec<Event>,
}

impl Module {
    /// Get the constructor function if present
    pub fn constructor(&self) -> Option<&Function> {
        self.functions.iter().find(|f| f.kind == FunctionKind::Constructor)
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name == name)
    }

    /// Count total instructions across all functions
    pub fn instruction_count(&self) -> usize {
        self.functions.iter()
            .flat_map(|f| &f.basic_blocks)
            .map(|bb| bb.instructions.len())
            .sum()
    }
}

/// IR function representation
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub kind: FunctionKind,
    pub parameters: Vec<ValueType>,
    pub returns: Vec<ValueType>,
    pub basic_blocks: Vec<BasicBlock>,
    pub local_count: u16,
}

impl Function {
    /// Check if this is a constructor
    pub fn is_constructor(&self) -> bool {
        self.kind == FunctionKind::Constructor
    }

    /// Get total instruction count
    pub fn instruction_count(&self) -> usize {
        self.basic_blocks.iter().map(|bb| bb.instructions.len()).sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionKind {
    Constructor,
    Regular,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateVariable {
    pub name: Option<String>,
    pub ty: ValueType,
    pub is_constant: bool,
    pub is_immutable: bool,
    pub storage_key: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicBlock {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeValue {
    MsgSender,
    MsgValue,
    MsgData,
    TxOrigin,
    BlockTimestamp,
    BlockNumber,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Drop(ValueType),
    /// NeoVM DUP - duplicate the top stack item
    Dup,
    /// NeoVM SWAP - swap the top two stack items
    Swap,
    LoadParameter(usize),
    PushLiteral(LiteralValue),
    Return,
    ReturnVoid,
    ReturnDefault(ValueType),
    BinaryOp(BinaryOperator),
    LoadState(usize),
    StoreState(usize),
    LoadStorageDynamic,
    LoadLocal(usize),
    StoreLocal(usize),
    LoadMappingElement {
        state_index: usize,
        key_types: Vec<ValueType>,
    },
    StoreMappingElement {
        state_index: usize,
        key_types: Vec<ValueType>,
    },
    LoadStructField {
        state_index: usize,
        key_types: Vec<ValueType>,
        field_keys: Vec<[u8; 32]>,
        field_type: ValueType,
    },
    StoreStructField {
        state_index: usize,
        key_types: Vec<ValueType>,
        field_keys: Vec<[u8; 32]>,
        field_type: ValueType,
    },
    LoadStructArrayElement {
        state_index: usize,
        key_types: Vec<ValueType>,
        field_keys: Vec<[u8; 32]>,
        element_type: ValueType,
    },
    StoreStructArrayElement {
        state_index: usize,
        key_types: Vec<ValueType>,
        field_keys: Vec<[u8; 32]>,
        element_type: ValueType,
    },
    LoadRuntimeValue(RuntimeValue),
    GetSize,
    CallFunction {
        name: String,
        arg_count: usize,
    },
    CallBuiltin {
        builtin: BuiltinCall,
        arg_count: usize,
    },
    EmitEvent {
        event_index: usize,
        arg_count: usize,
    },
    EmitEventByName {
        name: String,
        arg_count: usize,
    },
    /// NeoVM `CONVERT` opcode. Converts the top stack item to a different StackItemType.
    Convert {
        target: ConvertTarget,
    },
    /// Allocate a new mutable buffer (NeoVM `NEWBUFFER`) whose length is taken from the stack.
    /// Used to implement Solidity `new bytes(n)` / `new string(n)` allocations.
    NewBuffer,
    NewArray {
        element_type: ValueType,
    },
    ArrayGet,
    ArraySet,
    /// NeoVM MEMCPY opcode: copy a slice of bytes into a buffer.
    /// Stack order (bottom -> top): [dst, dst_offset, src, src_offset, count]
    MemCpy,
    /// NeoVM REVERSEITEMS opcode: reverse an Array/Buffer in place.
    /// Note: consumes one reference and does not push it back.
    ReverseItems,
    BitwiseNot,
    /// NeoVM NOT (0xAA) — logical boolean negation.
    /// Pops one value, pushes `!value.is_truthy()`.
    /// Correctly handles null (returns true), unlike `PUSHF + EQUAL`.
    LogicalNot,
    Try {
        catch_target: usize,
    },
    EndTry {
        target: usize,
    },
    Jump {
        target: usize,
    },
    JumpIf {
        target: usize,
    },
    Label(usize),
    /// NeoVM THROW - raises a catchable exception using the value on the stack.
    Throw,
    AbortMsg,
    Abort,
}

/// Literal values in IR
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralValue {
    Integer(BigInt),
    Boolean(bool),
    String(Vec<u8>),
    ByteArray(Vec<u8>),
    Address(Vec<u8>),
    Null,
}

impl LiteralValue {
    /// Create an integer literal
    pub fn int(value: impl Into<BigInt>) -> Self {
        Self::Integer(value.into())
    }

    /// Create a boolean literal
    pub fn bool(value: bool) -> Self {
        Self::Boolean(value)
    }

    /// Check if this is a null value
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Get the type name
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Integer(_) => "integer",
            Self::Boolean(_) => "boolean",
            Self::String(_) => "string",
            Self::ByteArray(_) => "bytes",
            Self::Address(_) => "address",
            Self::Null => "null",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvertTarget {
    Any,
    Boolean,
    Integer,
    ByteArray,
    Array,
    Map,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueType {
    Integer {
        signed: bool,
        bits: u16,
    },
    Boolean,
    String,
    Address,
    ByteArray {
        fixed_len: Option<u16>,
    },
    Array(Box<ValueType>),
    Mapping {
        key: Box<ValueType>,
        value: Box<ValueType>,
    },
    Struct {
        name: String,
        fields: Vec<StructField>,
    },
    Any,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManifestType {
    Integer,
    Boolean,
    String,
    Hash160,
    Hash256,
    ByteArray,
    Array,
    Map,
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    pub name: String,
    pub ty: ValueType,
    pub key: [u8; 32],
}

/// Binary operators for IR instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

impl BinaryOperator {
    /// Check if this is an arithmetic operator
    pub fn is_arithmetic(&self) -> bool {
        matches!(self, Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Mod)
    }

    /// Check if this is a comparison operator
    pub fn is_comparison(&self) -> bool {
        matches!(self, Self::Lt | Self::Le | Self::Gt | Self::Ge | Self::Eq | Self::Ne)
    }

    /// Check if this is a bitwise operator
    pub fn is_bitwise(&self) -> bool {
        matches!(self, Self::BitAnd | Self::BitOr | Self::BitXor | Self::Shl | Self::Shr)
    }

    /// Get the operator symbol
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Mod => "%",
            Self::BitAnd => "&",
            Self::BitOr => "|",
            Self::BitXor => "^",
            Self::Shl => "<<",
            Self::Shr => ">>",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::Eq => "==",
            Self::Ne => "!=",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeContract {
    Neo,
    Gas,
    ContractManagement,
    Policy,
    Oracle,
    RoleManagement,
    Notary,
    Treasury,
    Ledger,
    CryptoLib,
    StdLib,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinCall {
    RuntimeNotify,
    RuntimeCheckWitness,
    AbiEncode,
    AbiEncodePacked,
    AbiEncodeWithSignature,
    AbiDecode,
    Keccak256,
    Ecrecover,
    StorageFind,
    StoragePut,
    StorageGet,
    StorageDelete,
    ContractCall,
    ContractCallWithFlags,
    NotifySerialized,
    VerifySignature,
    /// Convenience wrapper for `ContractManagement.deploy` that returns only the deployed
    /// contract hash (UInt160) rather than the full `ContractState`.
    DeployContract,
    /// Convenience wrapper for `ContractManagement.getContract` that reshapes the returned
    /// `ContractState` into a devpack-friendly struct:
    /// `[hash, nef, serialize(manifest), updateCounter]`.
    GetContract,
    /// Convenience wrapper for fetching the NEF file from a contract state.
    /// This is exposed as `Syscalls.getContractScript` in the devpack.
    GetContractScript,
    /// Convenience wrapper for `NeoToken.getAccountState` that:
    /// - Returns a default struct when the native contract returns `null`.
    /// - Normalizes the `voteTo` field to an empty byte string when it is `null`.
    GetNeoAccountState,
    NativeCall {
        contract: NativeContract,
        method: String,
    },
    Syscall(String),
    TypeOf,
}
