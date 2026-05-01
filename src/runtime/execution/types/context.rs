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
    /// Bug #17: cap on cumulative storage_overlay byte footprint. Wired through
    /// `enforce_storage_limit` on every `Storage.Put` / `Local.Put`. Before this
    /// was added, the `RuntimeConfig::storage_limit` field had no consumer and
    /// any contract could `Storage.put` arbitrary-length values until the host
    /// process OOM-aborted.
    pub(crate) storage_limit: usize,
    pub(crate) return_data: Vec<u8>,
    pub(crate) logs: Vec<LogEntry>,
    pub(crate) call_stack: Vec<CallFrame>,
    pub(crate) try_stack: Vec<TryFrame>,
    pub(crate) uncaught_exception: Option<String>,
    /// Task #27 (runtime slice) — raw bytes popped from the evaluation stack
    /// by the most recent `THROW` opcode, preserved verbatim (not UTF-8
    /// lossy'd into a `String` the way `uncaught_exception` is).
    ///
    /// The IR lowerer currently `Drop`s revert args at
    /// `src/ir/statements/dispatch/return_revert.rs`, so for source like
    /// `revert TooSmall(7)` this captures only the error-name bytes. When the
    /// compiler slice of Task #27 lands and `revert` lowers to
    /// `PUSH (selector || abi.encode(args)); THROW`, the full ABI-encoded
    /// payload flows through this field unchanged, and the bridge routes it
    /// to `ExecutionResult.return_data`.
    ///
    /// In the meantime this gives consumers that hand-roll their revert
    /// payload (push bytes, THROW) a working EVM-convention return_data
    /// surface.
    pub(crate) revert_payload: Vec<u8>,
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
    /// When true, `default_account_bytes` is derived from the loaded
    /// bytecode via `Hash160(bytecode)` during `initialize`, matching the
    /// Neo VM semantics for the executing script hash.
    ///
    /// This flag is set to `true` in `new()` when the configured
    /// `contract_account` is the zero UInt160 (the default), so that
    /// `address(this)` (which reads `default_account_bytes` via the
    /// `System.Runtime.GetExecutingScriptHash` handler) returns the
    /// deterministic contract hash instead of 20 zero bytes.
    ///
    /// Tests that need an explicit pinned account can use
    /// [`Self::force_default_account_explicit_for_tests`] to clear the flag
    /// and preserve the configured bytes.
    pub(crate) default_account_derived: bool,
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
    /// Task #176: sticky caller-account override that survives across
    /// consecutive `call_method` / `call_method_with_deploy_args` invocations
    /// on the same `NeoRuntime`. Unlike `pending_caller_account` (which the
    /// `initialize` drain clears after one execution — the correct semantic
    /// for `execute_with_overrides` tests), this slot holds the last override
    /// the host set via `NeoRuntime::override_caller_account` until either
    /// (a) the host sets a new one, or (b) the host explicitly clears it via
    /// `clear_pending_overrides`. `call_method_with_deploy_args` re-arms
    /// `pending_caller_account` from this slot before dispatching the user
    /// method so that `msg.sender` stays pinned across a multi-call session
    /// (the vault deposit→withdraw→balanceOf pattern). Without this slot
    /// the second `call_method` saw `caller_account = None` after
    /// `initialize`'s `take()` drain, which fell back to
    /// `default_account_bytes` in the `GetCallingScriptHash` syscall handler
    /// — breaking mapping-by-sender invariants (`balances[alice]` keyed under
    /// `alice` on deposit, re-keyed under `default` on withdraw).
    pub(crate) sticky_caller_account: Option<Vec<u8>>,
    /// Task #113: value set by `NeoRuntime::override_value` /
    /// `ExecutionOverrides::value`. Drained into `msg_value` on each
    /// `initialize` call so the override applies to exactly one execution,
    /// mirroring the pending_timestamp / pending_block_height pattern.
    pub(crate) pending_msg_value: Option<u64>,
    /// Task #113: active Solidity `msg.value` for the current execution.
    /// `None` means no override was set; the compiled `GetMsgValue` syscall
    /// handler defaults to 0 in that case (Neo N3 has no native attached
    /// value). Populated from `pending_msg_value` in `initialize`.
    pub(crate) msg_value: Option<u64>,
    pub(crate) neo_balances: HashMap<Vec<u8>, u64>,
    pub(crate) gas_balances: HashMap<Vec<u8>, u64>,
    pub(crate) neo_total_supply: u64,
    pub(crate) gas_total_supply: u64,
    pub(crate) contract_registry: HashMap<Vec<u8>, ContractState>,
    pub(crate) next_contract_id: u32,
    /// Task #70: method-name → bytecode-offset table for the currently executing
    /// contract, used by `System.Contract.Call` to route `this.someFn()` self
    /// external calls. Populated by `NeoRuntime::call_method` from the manifest
    /// before each invocation. Empty for raw `execute()` paths.
    pub(crate) self_method_offsets: HashMap<String, u32>,
    /// Task #70: args-count for each self method, used to pop the right number of args
    /// from the `StackItem::Array` `params` passed by `handle_contract_call`.
    pub(crate) self_method_arg_counts: HashMap<String, u16>,
    /// Task #70: set by `handle_contract_call` when a self external call
    /// rewires `instruction_pointer` directly to a compiled method offset.
    /// The SYSCALL dispatcher consults this flag to skip its unconditional
    /// `instruction_pointer += 5` post-increment (which would otherwise land
    /// the VM past the target method's INITSLOT prologue).
    pub(crate) syscall_suppress_ip_advance: bool,
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
