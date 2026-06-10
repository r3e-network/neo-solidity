use super::*;

impl ExecutionContext {
    /// Create new execution context
    pub fn new(config: &RuntimeConfig) -> Result<Self, RuntimeError> {
        let default_account = Self::normalize_account(&config.contract_account)?;
        let default_account_bytes = Self::account_string_to_bytes(&default_account)?;
        // If the configured `contract_account` is the default zero UInt160,
        // treat the executing script hash as derivable from the loaded
        // bytecode — `initialize` will populate `default_account_bytes`
        // with `Hash160(bytecode)` on each load.
        //
        // This preserves pinned-account behavior when the caller explicitly
        // configures a non-zero `contract_account`, while making
        // `address(this)` return a deterministic hash for every other
        // caller (notably the test harness, which uses `RuntimeConfig::default`).
        let default_account_derived = default_account_bytes.iter().all(|&b| b == 0);
        let default_block_height = config.default_block_height;
        let default_timestamp = config.default_timestamp;

        Ok(Self {
            bytecode: Vec::new(),
            method_tokens: Vec::new(),
            input_data: Vec::new(),
            gas_limit: config.gas_limit,
            gas_used: 0,
            instruction_pointer: 0,
            call_stack_limit: config.call_stack_limit,
            stack: Vec::new(),
            locals: Vec::new(),
            args: Vec::new(),
            static_fields: Vec::new(),
            memory: Vec::new(),
            memory_limit: config.memory_limit,
            storage_limit: config.storage_limit,
            return_data: Vec::new(),
            logs: Vec::new(),
            call_stack: Vec::new(),
            try_stack: Vec::new(),
            uncaught_exception: None,
            revert_payload: Vec::new(),
            iterators: HashMap::new(),
            next_iterator_id: 1,
            syscall_gas: spec::syscall_gas_table(),
            debugging_enabled: config.enable_debugging,
            breakpoints: HashSet::new(),
            instruction_count: 0,
            max_stack_depth: 0,
            storage_overlay: HashMap::new(),
            storage_account: Some(default_account.clone()),
            storage_host: None,
            default_account,
            default_account_bytes,
            default_account_derived,
            caller_account: None,
            block_height: Some(default_block_height),
            default_block_height,
            timestamp: Some(default_timestamp),
            default_timestamp,
            invocation_counter: 0,
            network_magic: config.network_magic,
            pending_caller_account: None,
            pending_block_height: None,
            pending_timestamp: None,
            pending_msg_value: None,
            sticky_caller_account: None,
            msg_value: None,
            neo_balances: HashMap::new(),
            gas_balances: HashMap::new(),
            neo_total_supply: 100_000_000,
            gas_total_supply: 30_000_000_000,
            contract_registry: HashMap::new(),
            next_contract_id: 1,
            self_method_offsets: HashMap::new(),
            self_method_arg_counts: HashMap::new(),
            syscall_suppress_ip_advance: false,
            strict_arithmetic: config.strict_mode,

            // ── Policy native contract defaults (Neo N3 MainNet) ──
            policy_fee_per_byte: 1000,
            policy_exec_fee_factor: 30,
            policy_storage_price: 100_000,
            policy_milliseconds_per_block: 15_000,
            policy_max_valid_until_block_increment: 5760,
            policy_max_traceable_blocks: 2_102_400,
            policy_attribute_fees: HashMap::new(),
            policy_blocked_accounts: HashSet::new(),
            policy_whitelisted_fee_contracts: Vec::new(),

            // ── Oracle native contract defaults ──
            oracle_price: 50_000_000,
            oracle_requests: HashMap::new(),
            oracle_next_request_id: 0,

            // ── RoleManagement ──
            role_designations: HashMap::new(),

            // ── Ledger ──
            ledger_blocks: HashMap::new(),
            ledger_transactions: HashMap::new(),
            ledger_current_hash: [0u8; 32],

            // ── Notary ──
            notary_deposits: HashMap::new(),
            notary_max_not_valid_before_delta: 140,

            // ── Treasury ──
            treasury_nep17_balances: HashMap::new(),
            treasury_nep11_tokens: HashMap::new(),

            // ── Syscall hardening ──
            witness_signers: Vec::new(),
            random_seed: None,
            random_counter: 0,
        })
    }

    /// Initialize context for execution
    pub fn initialize(&mut self, bytecode: &[u8], input: &[u8]) -> Result<(), RuntimeError> {
        self.bytecode = bytecode.to_vec();
        self.method_tokens.clear();
        self.input_data = input.to_vec();
        self.instruction_pointer = 0;
        self.stack.clear();
        self.locals.clear();
        self.args.clear();
        self.static_fields.clear();
        self.memory.clear();
        self.return_data.clear();
        self.logs.clear();
        self.call_stack.clear();
        self.gas_used = 0;
        self.instruction_count = 0;
        self.storage_overlay.clear();
        self.iterators.clear();
        self.next_iterator_id = 1;
        // Derive the executing script hash from the loaded bytecode when the
        // context was configured with the default zero `contract_account`.
        //
        // Neo VM semantics: the `System.Runtime.GetExecutingScriptHash`
        // syscall returns `Hash160(script) = RIPEMD160(SHA256(script))` of
        // the currently executing script. Without this derivation,
        // `address(this)` would read back 20 zero bytes and silently
        // validate self-signed EIP-2612 messages / self-allowance checks.
        if self.default_account_derived && !bytecode.is_empty() {
            use ripemd::Ripemd160;
            use sha2::{Digest, Sha256};
            let sha = Sha256::digest(bytecode);
            let hash: [u8; 20] = Ripemd160::digest(sha).into();
            self.default_account_bytes = hash.to_vec();
            let mut le = [0u8; 20];
            le.copy_from_slice(&hash);
            self.default_account = crate::neo::format_uint160_hex_be(&le);
        }
        self.storage_account = Some(self.default_account.clone());
        self.storage_host = None;
        self.caller_account = self.pending_caller_account.take();
        self.block_height = self
            .pending_block_height
            .take()
            .or(Some(self.default_block_height));
        self.timestamp = self
            .pending_timestamp
            .take()
            .or(Some(self.default_timestamp));
        // Task #113 — drain the pending msg.value override into the active
        // slot. Unlike block_height/timestamp, msg.value has no default
        // fallback (None is semantically distinct from Some(0): the former
        // indicates "host never set a value", the latter indicates "host
        // explicitly injected 0"). The `GetMsgValue` syscall handler coalesces
        // None → 0 when pushing to the stack.
        self.msg_value = self.pending_msg_value.take();
        // Neo N3 invocation counter is 1 for the first invocation of a contract
        // within a script execution and increments on re-entry.
        self.invocation_counter = 1;
        self.try_stack.clear();
        self.uncaught_exception = None;
        self.revert_payload.clear();
        self.iterators.clear();
        self.next_iterator_id = 1;
        Ok(())
    }

    /// Initialize execution context with a NEF method token table.
    ///
    /// This is required to execute scripts that contain the `CALLT` opcode (0x37).
    pub fn initialize_with_tokens(
        &mut self,
        bytecode: &[u8],
        input: &[u8],
        tokens: &[crate::neo::MethodToken],
    ) -> Result<(), RuntimeError> {
        self.initialize(bytecode, input)?;
        self.method_tokens = tokens.to_vec();
        Ok(())
    }

    /// Override the block height for the next execution.
    pub fn override_block_height(&mut self, height: u64) {
        self.pending_block_height = Some(height);
    }

    /// Override the timestamp for the next execution.
    pub fn override_timestamp(&mut self, timestamp: u64) {
        self.pending_timestamp = Some(timestamp);
    }

    /// Override the calling script hash for the next execution.
    ///
    /// Task #176 — in addition to arming `pending_caller_account` (drained
    /// by the next `initialize` into the active `caller_account`), this
    /// mirrors the bytes into `sticky_caller_account` so that successive
    /// `call_method` invocations on the same `NeoRuntime` keep observing
    /// the host-specified caller. The `call_method_with_deploy_args` entry
    /// re-arms `pending_caller_account` from the sticky slot before
    /// dispatching the user method. `execute_with_overrides` does not
    /// consult the sticky slot (its per-invocation override semantics are
    /// preserved). `clear_pending_overrides` and explicit replacement via
    /// a subsequent `override_caller_account` call both reset the sticky
    /// slot.
    pub fn override_caller_account(&mut self, account: &str) -> Result<(), RuntimeError> {
        let normalized = Self::normalize_account(account)?;
        let bytes = Self::account_string_to_bytes(&normalized)?;
        self.pending_caller_account = Some(bytes.clone());
        self.sticky_caller_account = Some(bytes);
        Ok(())
    }

    /// Task #113 — override Solidity `msg.value` for the next execution.
    ///
    /// Neo N3 has no native attached-value concept; this slot is purely a
    /// host-side injection that the compiled `MsgValue` lowering reads via
    /// `System.Runtime.GetMsgValue`. Drained by `initialize` so the override
    /// applies to exactly one invocation (matching the
    /// `override_timestamp` / `override_block_height` contract).
    pub fn override_value(&mut self, value: u64) {
        self.pending_msg_value = Some(value);
    }

    /// Clear any pending metadata overrides before the next execution.
    ///
    /// Task #176 — also clears the sticky caller-account slot so that
    /// subsequent `call_method` invocations no longer re-arm
    /// `pending_caller_account` from a prior `override_caller_account` call.
    pub fn clear_pending_overrides(&mut self) {
        self.pending_block_height = None;
        self.pending_timestamp = None;
        self.pending_caller_account = None;
        self.pending_msg_value = None;
        self.sticky_caller_account = None;
    }
}
