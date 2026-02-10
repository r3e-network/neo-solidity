//! Execution context for runtime operations.
//!
//! ExecutionContext is the main structure that holds all state during
//! NeoVM bytecode execution, including the evaluation stack, local variables,
//! memory, call stack, and storage.

use crate::neo::MethodToken;
use crate::runtime::{storage, LogEntry};
use super::stack::StackItem;
use super::frame::{CallFrame, TryFrame};
use super::state::{
    IteratorState, ContractState, OverlayEntry,
    WhitelistedFeeContract, OracleRequest, LedgerBlock, LedgerTransaction, NotaryDeposit,
};
use std::collections::{HashMap, HashSet};
use std::ptr::NonNull;

/// Execution context for runtime operations
#[derive(Debug)]
pub struct ExecutionContext {
    pub(crate) bytecode: Vec<u8>,
    /// NEF method tokens used by `CALLT` (0x37). When executing raw scripts
    /// without a NEF header, this table is empty and `CALLT` will error.
    pub(crate) method_tokens: Vec<MethodToken>,
    pub(crate) input_data: Vec<u8>,
    pub(crate) gas_limit: u64,
    pub(crate) gas_used: u64,
    pub(crate) instruction_pointer: u32,
    pub(crate) call_stack_limit: u32,
    pub(crate) stack: Vec<StackItem>,
    pub(crate) locals: Vec<StackItem>,
    pub(crate) args: Vec<StackItem>,
    pub(crate) static_fields: Vec<StackItem>,
    pub(crate) memory: Vec<u8>,
    pub(crate) memory_limit: usize,
    pub(crate) return_data: Vec<u8>,
    pub(crate) logs: Vec<LogEntry>,
    pub(crate) call_stack: Vec<CallFrame>,
    pub(crate) try_stack: Vec<TryFrame>,
    pub(crate) uncaught_exception: Option<String>,
    pub(crate) iterators: HashMap<u64, IteratorState>,
    pub(crate) next_iterator_id: u64,
    pub(crate) syscall_gas: HashMap<[u8; 4], u64>,
    pub(crate) debugging_enabled: bool,
    pub(crate) breakpoints: HashSet<u32>,
    pub(crate) instruction_count: u64,
    pub(crate) max_stack_depth: u32,
    pub(crate) storage_overlay: HashMap<Vec<u8>, OverlayEntry>,
    pub(crate) storage_account: Option<String>,
    pub(crate) storage_host: Option<NonNull<storage::StorageManager>>,
    pub(crate) default_account: String,
    pub(crate) default_account_bytes: Vec<u8>,
    pub(crate) caller_account: Option<Vec<u8>>,
    pub(crate) block_height: Option<u64>,
    pub(crate) default_block_height: u64,
    pub(crate) timestamp: Option<u64>,
    pub(crate) default_timestamp: u64,
    pub(crate) invocation_counter: u64,
    pub(crate) network_magic: u32,
    pub(crate) pending_caller_account: Option<Vec<u8>>,
    pub(crate) pending_block_height: Option<u64>,
    pub(crate) pending_timestamp: Option<u64>,
    pub(crate) neo_balances: HashMap<Vec<u8>, u64>,
    pub(crate) gas_balances: HashMap<Vec<u8>, u64>,
    pub(crate) neo_total_supply: u64,
    pub(crate) gas_total_supply: u64,
    pub(crate) contract_registry: HashMap<Vec<u8>, ContractState>,
    pub(crate) next_contract_id: u32,
    /// When true, arithmetic overflow/underflow will return an error instead of wrapping
    pub(crate) strict_arithmetic: bool,

    // ── Policy native contract state ──
    pub(crate) policy_fee_per_byte: u64,
    pub(crate) policy_exec_fee_factor: u32,
    pub(crate) policy_storage_price: u64,
    pub(crate) policy_milliseconds_per_block: u32,
    pub(crate) policy_max_valid_until_block_increment: u32,
    pub(crate) policy_max_traceable_blocks: u32,
    pub(crate) policy_attribute_fees: HashMap<u8, u64>,
    pub(crate) policy_blocked_accounts: HashSet<Vec<u8>>,
    pub(crate) policy_whitelisted_fee_contracts: Vec<WhitelistedFeeContract>,

    // ── Oracle native contract state ──
    pub(crate) oracle_price: u64,
    pub(crate) oracle_requests: HashMap<u64, OracleRequest>,
    pub(crate) oracle_next_request_id: u64,

    // ── RoleManagement native contract state ──
    /// Maps role id → list of designated public keys
    pub(crate) role_designations: HashMap<u8, Vec<Vec<u8>>>,

    // ── Ledger native contract state ──
    pub(crate) ledger_blocks: HashMap<u64, LedgerBlock>,
    pub(crate) ledger_transactions: HashMap<[u8; 32], LedgerTransaction>,
    pub(crate) ledger_current_hash: [u8; 32],

    // ── Notary native contract state ──
    pub(crate) notary_deposits: HashMap<Vec<u8>, NotaryDeposit>,
    pub(crate) notary_max_not_valid_before_delta: u32,

    // ── Treasury native contract state ──
    /// NEP-17 balances tracked by treasury: contract_hash → amount
    pub(crate) treasury_nep17_balances: HashMap<Vec<u8>, u64>,
    /// NEP-11 tokens tracked by treasury: contract_hash → set of token_ids
    pub(crate) treasury_nep11_tokens: HashMap<Vec<u8>, Vec<Vec<u8>>>,

    // ── Syscall hardening state ──
    /// Witness signers for CheckWitness verification
    pub(crate) witness_signers: Vec<Vec<u8>>,
    /// Seed for deterministic random number generation
    pub(crate) random_seed: Option<[u8; 32]>,
    /// Counter for sequential random number generation
    pub(crate) random_counter: u64,
}
