impl ExecutionContext {
    fn stack_item_to_bytes(item: StackItem) -> Vec<u8> {
        match item {
            StackItem::ByteArray(bytes) => bytes.borrow().clone(),
            StackItem::Integer(value) => value.to_le_bytes().to_vec(),
            StackItem::UnsignedInteger(value) => value.to_le_bytes().to_vec(),
            StackItem::Boolean(value) => vec![value as u8],
            StackItem::Map(_) | StackItem::Array(_) => serde_json::to_vec(&item).unwrap_or_default(),
            StackItem::Null => Vec::new(),
        }
    }

    fn interop_id_bytes(name: &str) -> [u8; 4] {
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        let digest = hasher.finalize();
        [digest[0], digest[1], digest[2], digest[3]]
    }

    fn normalize_account(account: &str) -> Result<String, RuntimeError> {
        let trimmed = account.trim();
        let without_prefix = trimmed.strip_prefix("0x").unwrap_or(trimmed);
        if !without_prefix.len().is_multiple_of(2) {
            return Err(RuntimeError::ConfigurationError {
                message: "contract account hex string has odd length".to_string(),
            });
        }
        if without_prefix.len() != 40 {
            return Err(RuntimeError::ConfigurationError {
                message: format!(
                    "contract account must be a UInt160 (40 hex chars, got {})",
                    without_prefix.len()
                ),
            });
        }
        let lower = without_prefix.to_ascii_lowercase();
        Ok(format!("0x{}", lower))
    }

    fn account_string_to_bytes(account: &str) -> Result<Vec<u8>, RuntimeError> {
        let normalized = Self::normalize_account(account)?;
        let le = crate::neo::parse_uint160_hex_be(&normalized).map_err(|msg| {
            RuntimeError::ConfigurationError {
                message: format!("invalid contract account: {msg}"),
            }
        })?;
        Ok(le.to_vec())
    }

    pub fn bind_storage(
        &mut self,
        account: &str,
        storage: &mut storage::StorageManager,
    ) -> Result<(), RuntimeError> {
        let normalized = Self::normalize_account(account)?;
        self.storage_account = Some(normalized);
        self.storage_host = Some(NonNull::from(storage));
        self.storage_overlay.clear();
        // Seed native contract balances for default account
        self.neo_balances
            .insert(self.default_account_bytes.clone(), self.neo_total_supply);
        self.gas_balances
            .insert(self.default_account_bytes.clone(), self.gas_total_supply);
        self.try_stack.clear();
        Ok(())
    }
}
