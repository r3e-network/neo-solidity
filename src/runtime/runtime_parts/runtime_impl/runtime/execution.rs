impl NeoRuntime {
    /// Override block height for the next execution.
    pub fn override_block_height(&mut self, height: u64) {
        self.execution_context.override_block_height(height);
    }

    /// Override timestamp for the next execution.
    pub fn override_timestamp(&mut self, timestamp: u64) {
        self.execution_context.override_timestamp(timestamp);
    }

    /// Override caller script hash for the next execution.
    pub fn override_caller_account(&mut self, account: &str) -> Result<(), RuntimeError> {
        self.execution_context.override_caller_account(account)
    }

    /// Execute bytecode with optional metadata overrides applied to this invocation.
    pub fn execute_with_overrides(
        &mut self,
        bytecode: &[u8],
        input: &[u8],
        overrides: &ExecutionOverrides,
    ) -> Result<ExecutionResult, RuntimeError> {
        if let Some(height) = overrides.block_height {
            self.override_block_height(height);
        }

        if let Some(timestamp) = overrides.timestamp {
            self.override_timestamp(timestamp);
        }

        if let Some(account) = overrides.caller_account.as_deref() {
            if let Err(err) = self.override_caller_account(account) {
                self.execution_context.clear_pending_overrides();
                return Err(err);
            }
        }

        self.execute(bytecode, input)
    }

    /// Create new runtime instance
    pub fn new(config: RuntimeConfig) -> Result<Self, RuntimeError> {
        Ok(Self {
            execution_context: execution::ExecutionContext::new(&config)?,
            state_manager: state::StateManager::new(&config)?,
            storage_manager: storage::StorageManager::new(&config)?,
            vm_bridge: bridge::VMBridge::new(&config)?,
            gas_tracker: execution::GasTracker::new(config.gas_limit),
        })
    }

    fn capture_metadata(&self) -> ExecutionMetadata {
        let block_height = self.execution_context.block_height();
        let timestamp = self.execution_context.timestamp();
        let caller_account = self.execution_context.caller_account().map(|bytes| {
            if bytes.len() == 20 {
                let mut arr = [0u8; 20];
                arr.copy_from_slice(bytes);
                crate::neo::format_uint160_hex_be(&arr)
            } else {
                format!("0x{}", hex::encode(bytes))
            }
        });

        ExecutionMetadata {
            block_height,
            timestamp,
            caller_account,
        }
    }

    /// Execute bytecode with given input
    pub fn execute(
        &mut self,
        bytecode: &[u8],
        input: &[u8],
    ) -> Result<ExecutionResult, RuntimeError> {
        // Initialize execution context
        self.execution_context.initialize(bytecode, input)?;

        // Reset gas tracker
        self.gas_tracker.reset(self.execution_context.gas_limit());

        // Execute bytecode through VM bridge
        let result = self.vm_bridge.execute(
            &mut self.execution_context,
            &mut self.state_manager,
            &mut self.storage_manager,
            &mut self.gas_tracker,
        )?;

        let mut result = result;
        result.metadata = self.capture_metadata();
        Ok(result)
    }

    /// Execute bytecode with a NEF method token table.
    ///
    /// This is required when the script contains the `CALLT` opcode (0x37).
    pub fn execute_with_tokens(
        &mut self,
        bytecode: &[u8],
        input: &[u8],
        tokens: &[crate::neo::MethodToken],
    ) -> Result<ExecutionResult, RuntimeError> {
        self.execution_context
            .initialize_with_tokens(bytecode, input, tokens)?;

        self.gas_tracker.reset(self.execution_context.gas_limit());

        let result = self.vm_bridge.execute(
            &mut self.execution_context,
            &mut self.state_manager,
            &mut self.storage_manager,
            &mut self.gas_tracker,
        )?;

        let mut result = result;
        result.metadata = self.capture_metadata();
        Ok(result)
    }

    /// Call specific function with arguments
    pub fn call_function(
        &mut self,
        bytecode: &[u8],
        function_name: &str,
        args: &[types::StackItem],
    ) -> Result<ExecutionResult, RuntimeError> {
        // Prepare function call
        let call_data = self.prepare_function_call(function_name, args)?;

        // Execute with call data
        self.execute(bytecode, &call_data)
    }
}
