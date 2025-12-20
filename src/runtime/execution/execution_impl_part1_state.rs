impl ExecutionContext {
    /// Get gas limit
    pub fn gas_limit(&self) -> u64 {
        self.gas_limit
    }

    /// Get gas used so far
    pub fn gas_used(&self) -> u64 {
        self.gas_used
    }

    /// Get instruction count
    pub fn instruction_count(&self) -> u64 {
        self.instruction_count
    }

    /// Get pending block height override, if any.
    pub fn pending_block_height(&self) -> Option<u64> {
        self.pending_block_height
    }

    /// Get pending timestamp override, if any.
    pub fn pending_timestamp(&self) -> Option<u64> {
        self.pending_timestamp
    }

    /// Get pending caller account override bytes, if any.
    pub fn pending_caller_account(&self) -> Option<&[u8]> {
        self.pending_caller_account.as_deref()
    }

    /// Get the active block height for the current execution.
    pub fn block_height(&self) -> Option<u64> {
        self.block_height
    }

    /// Get the active timestamp for the current execution.
    pub fn timestamp(&self) -> Option<u64> {
        self.timestamp
    }

    /// Get the active caller account for the current execution.
    pub fn caller_account(&self) -> Option<&[u8]> {
        self.caller_account.as_deref()
    }

    /// Get the default account bytes configured for this context.
    pub fn default_account_bytes(&self) -> &[u8] {
        &self.default_account_bytes
    }

    /// Number of contracts currently tracked by the in-memory registry.
    pub fn contract_registry_len(&self) -> usize {
        self.contract_registry.len()
    }

    /// Update counter for a registered contract (if present).
    pub fn contract_update_counter(&self, hash: &[u8]) -> Option<u32> {
        self.contract_registry.get(hash).map(|c| c.update_counter)
    }

    /// Known contract hashes (20-byte each).
    pub fn contract_hashes(&self) -> Vec<[u8; 20]> {
        self.contract_registry
            .keys()
            .filter_map(|k| {
                if k.len() == 20 {
                    let mut arr = [0u8; 20];
                    arr.copy_from_slice(k);
                    Some(arr)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get maximum stack depth
    pub fn max_stack_depth(&self) -> u32 {
        self.max_stack_depth
    }
}

