impl ExecutionContext {
    /// Create new execution context
    pub fn new(config: &RuntimeConfig) -> Result<Self, RuntimeError> {
        let default_account = Self::normalize_account(&config.contract_account)?;
        let default_account_bytes = Self::account_string_to_bytes(&default_account)?;
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
            return_data: Vec::new(),
            logs: Vec::new(),
            call_stack: Vec::new(),
            try_stack: Vec::new(),
            uncaught_exception: None,
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
            neo_balances: HashMap::new(),
            gas_balances: HashMap::new(),
            neo_total_supply: 100_000_000,
            gas_total_supply: 30_000_000_000,
            contract_registry: HashMap::new(),
            next_contract_id: 1,
            strict_arithmetic: config.strict_mode,
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
        self.invocation_counter = 0;
        self.try_stack.clear();
        self.uncaught_exception = None;
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
    pub fn override_caller_account(&mut self, account: &str) -> Result<(), RuntimeError> {
        let normalized = Self::normalize_account(account)?;
        let bytes = Self::account_string_to_bytes(&normalized)?;
        self.pending_caller_account = Some(bytes);
        Ok(())
    }

    /// Clear any pending metadata overrides before the next execution.
    pub fn clear_pending_overrides(&mut self) {
        self.pending_block_height = None;
        self.pending_timestamp = None;
        self.pending_caller_account = None;
    }
}
