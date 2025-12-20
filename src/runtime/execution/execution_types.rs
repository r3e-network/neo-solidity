/// Execution context for runtime operations
#[derive(Debug)]
pub struct ExecutionContext {
    bytecode: Vec<u8>,
    /// NEF method tokens used by `CALLT` (0x37). When executing raw scripts
    /// without a NEF header, this table is empty and `CALLT` will error.
    method_tokens: Vec<crate::neo::MethodToken>,
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
    uncaught_exception: Option<String>,
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
    next_contract_id: u32,
    /// When true, arithmetic overflow/underflow will return an error instead of wrapping
    strict_arithmetic: bool,
}

/// Stack item in execution context
#[derive(Debug, Clone)]
pub enum StackItem {
    Integer(i64),
    UnsignedInteger(u64),
    ByteArray(std::rc::Rc<std::cell::RefCell<Vec<u8>>>),
    Array(std::rc::Rc<std::cell::RefCell<Vec<StackItem>>>),
    Map(std::rc::Rc<std::cell::RefCell<std::collections::HashMap<Vec<u8>, StackItem>>>),
    Boolean(bool),
    Null,
}

impl StackItem {
    pub fn byte_array(bytes: Vec<u8>) -> Self {
        Self::ByteArray(std::rc::Rc::new(std::cell::RefCell::new(bytes)))
    }

    pub fn array(items: Vec<StackItem>) -> Self {
        Self::Array(std::rc::Rc::new(std::cell::RefCell::new(items)))
    }

    pub fn map(map: std::collections::HashMap<Vec<u8>, StackItem>) -> Self {
        Self::Map(std::rc::Rc::new(std::cell::RefCell::new(map)))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
enum StackItemSerde {
    Integer(i64),
    UnsignedInteger(u64),
    ByteArray(Vec<u8>),
    Array(Vec<StackItemSerde>),
    Map(Vec<(Vec<u8>, StackItemSerde)>),
    Boolean(bool),
    Null,
}

impl StackItemSerde {
    fn from_item(item: &StackItem) -> Self {
        match item {
            StackItem::Integer(value) => StackItemSerde::Integer(*value),
            StackItem::UnsignedInteger(value) => StackItemSerde::UnsignedInteger(*value),
            StackItem::ByteArray(bytes) => StackItemSerde::ByteArray(bytes.borrow().clone()),
            StackItem::Array(items) => StackItemSerde::Array(
                items
                    .borrow()
                    .iter()
                    .map(StackItemSerde::from_item)
                    .collect(),
            ),
            StackItem::Map(map) => StackItemSerde::Map(
                map.borrow()
                    .iter()
                    .map(|(k, v)| (k.clone(), StackItemSerde::from_item(v)))
                    .collect(),
            ),
            StackItem::Boolean(value) => StackItemSerde::Boolean(*value),
            StackItem::Null => StackItemSerde::Null,
        }
    }

    fn into_item(self) -> StackItem {
        match self {
            StackItemSerde::Integer(value) => StackItem::Integer(value),
            StackItemSerde::UnsignedInteger(value) => StackItem::UnsignedInteger(value),
            StackItemSerde::ByteArray(bytes) => StackItem::byte_array(bytes),
            StackItemSerde::Array(items) => {
                let converted = items.into_iter().map(StackItemSerde::into_item).collect();
                StackItem::array(converted)
            }
            StackItemSerde::Map(entries) => {
                let mut map = std::collections::HashMap::new();
                for (key, value) in entries {
                    map.insert(key, value.into_item());
                }
                StackItem::map(map)
            }
            StackItemSerde::Boolean(value) => StackItem::Boolean(value),
            StackItemSerde::Null => StackItem::Null,
        }
    }
}

impl serde::Serialize for StackItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        StackItemSerde::from_item(self).serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for StackItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        StackItemSerde::deserialize(deserializer).map(StackItemSerde::into_item)
    }
}

impl PartialEq for StackItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StackItem::Integer(a), StackItem::Integer(b)) => a == b,
            (StackItem::UnsignedInteger(a), StackItem::UnsignedInteger(b)) => a == b,
            (StackItem::Boolean(a), StackItem::Boolean(b)) => a == b,
            (StackItem::Null, StackItem::Null) => true,
            (StackItem::ByteArray(a), StackItem::ByteArray(b)) => a.borrow().eq(&*b.borrow()),
            (StackItem::Array(a), StackItem::Array(b)) => a.borrow().eq(&*b.borrow()),
            (StackItem::Map(a), StackItem::Map(b)) => a.borrow().eq(&*b.borrow()),
            _ => false,
        }
    }
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
    end_target: Option<u32>,
    state: TryFrameState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TryFrameState {
    Try,
    Catch,
    Finally,
}

#[derive(Debug, Clone)]
struct IteratorState {
    entries: Vec<StackItem>,
    index: usize,
}

#[derive(Debug, Clone)]
struct ContractState {
    id: u32,
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

/// Storage key that includes account for isolation
/// Used for cross-contract storage isolation to prevent storage collisions
#[allow(dead_code)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct IsolatedStorageKey {
    /// Account/contract that owns this storage
    account: String,
    /// The actual storage key
    key: Vec<u8>,
}

#[allow(dead_code)]
impl IsolatedStorageKey {
    fn new(account: &str, key: Vec<u8>) -> Self {
        Self {
            account: account.to_string(),
            key,
        }
    }
}

type StorageOverlayEntries = Vec<(Vec<u8>, Option<Vec<u8>>)>;
