impl ExecutionContext {
    fn build_storage_entries(&self, prefix: Vec<u8>) -> Result<Vec<StackItem>, RuntimeError> {
        let mut entries: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        if let (Some(mut ptr), Some(account)) = (self.storage_host, self.storage_account.as_ref()) {
            // # Safety
            //
            // The `storage_host` pointer is set via `bind_storage()`, which receives a raw pointer
            // to a `StorageHost` struct. We guarantee:
            //
            // 1. The pointer is valid for the entire execution context lifetime (set at bind time)
            // 2. No other code accesses or frees this storage during execution
            // 3. We only hold mutable references for the duration of this method call
            // 4. The storage host's memory is not modified by any external code
            //
            // This pattern is necessary because we're bridging between the Rust runtime
            // and the storage layer which uses FFI-compatible interfaces.
            let storage = unsafe { ptr.as_mut() };
            let query = storage::StorageQuery {
                account: account.clone(),
                key_prefix: Some(prefix.clone()),
                limit: None,
                include_pending: true,
            };
            entries = storage.query(query)?;
        }

        for (key, entry) in &self.storage_overlay {
            if !key.starts_with(&prefix) {
                continue;
            }
            match &entry.value {
                Some(value) => {
                    entries.retain(|(k, _)| k != key);
                    entries.push((key.clone(), value.clone()));
                }
                None => entries.retain(|(k, _)| k != key),
            }
        }

        entries.sort_by(|a, b| a.0.cmp(&b.0));
        let mut items = Vec::with_capacity(entries.len());
        for (k, v) in entries {
            items.push(StackItem::array(vec![
                StackItem::byte_array(k),
                StackItem::byte_array(v),
            ]));
        }
        Ok(items)
    }

    fn allocate_iterator(&mut self, entries: Vec<StackItem>) -> StackItem {
        let id = self.next_iterator_id;
        self.next_iterator_id = self.next_iterator_id.saturating_add(1);
        self.iterators
            .insert(id, IteratorState { entries, index: 0 });
        StackItem::byte_array(id.to_le_bytes().to_vec())
    }

    fn iterator_id_from_item(item: &StackItem) -> Option<u64> {
        if let StackItem::ByteArray(bytes) = item {
            let bytes = bytes.borrow();
            if bytes.len() >= 8 {
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&bytes[..8]);
                return Some(u64::from_le_bytes(buf));
            }
        }
        None
    }

    fn register_contract(&mut self, nef: Vec<u8>, manifest: Vec<u8>) -> ContractState {
        fn nef_checksum(nef: &[u8]) -> u32 {
            if nef.len() >= 4 {
                let mut buf = [0u8; 4];
                let start = nef.len() - 4;
                buf.copy_from_slice(&nef[start..]);
                u32::from_le_bytes(buf)
            } else {
                let first = Sha256::digest(nef);
                let second = Sha256::digest(first);
                u32::from_le_bytes(second[..4].try_into().expect("checksum bytes"))
            }
        }

        fn manifest_name(manifest: &[u8]) -> Option<String> {
            let text = std::str::from_utf8(manifest).ok()?;
            let json: serde_json::Value = serde_json::from_str(text).ok()?;
            json.get("name")?.as_str().map(|s| s.to_string())
        }

        let checksum = nef_checksum(&nef);
        let name = manifest_name(&manifest).unwrap_or_default();
        let sender_bytes = self
            .caller_account
            .as_deref()
            .unwrap_or(&self.default_account_bytes);
        let mut sender_le = [0u8; 20];
        if sender_bytes.len() >= 20 {
            sender_le.copy_from_slice(&sender_bytes[..20]);
        } else {
            sender_le[..sender_bytes.len()].copy_from_slice(sender_bytes);
        }

        let hash = crate::neo::compute_contract_hash(sender_le, checksum, &name);
        let id = self.next_contract_id;
        self.next_contract_id = self.next_contract_id.saturating_add(1);
        let state = ContractState {
            id,
            hash,
            nef,
            manifest,
            update_counter: 0,
        };
        self.contract_registry.insert(hash.to_vec(), state.clone());
        state
    }

    fn update_contract(
        &mut self,
        hash: &[u8],
        nef: Vec<u8>,
        manifest: Vec<u8>,
    ) -> Option<ContractState> {
        if let Some(existing) = self.contract_registry.get_mut(hash) {
            existing.nef = nef;
            existing.manifest = manifest;
            existing.update_counter = existing.update_counter.saturating_add(1);
            return Some(existing.clone());
        }
        None
    }

    fn lookup_contract(&self, hash: &[u8]) -> Option<ContractState> {
        self.contract_registry.get(hash).cloned()
    }

    fn contract_to_stackitem(&self, state: &ContractState) -> StackItem {
        StackItem::array(vec![
            StackItem::UnsignedInteger(state.id as u64),
            StackItem::UnsignedInteger(state.update_counter as u64),
            StackItem::byte_array(state.hash.to_vec()),
            StackItem::byte_array(state.nef.clone()),
            StackItem::byte_array(state.manifest.clone()),
        ])
    }

    pub fn unbind_storage(&mut self) {
        self.storage_host = None;
        self.storage_account = Some(self.default_account.clone());
        self.storage_overlay.clear();
    }

    pub fn drain_dirty_storage_overlay(&mut self) -> Option<(String, StorageOverlayEntries)> {
        let account = self.storage_account.clone()?;
        let mut entries = Vec::new();
        for (key, entry) in self.storage_overlay.drain() {
            if entry.dirty {
                entries.push((key, entry.value));
            }
        }

        if entries.is_empty() {
            None
        } else {
            Some((account, entries))
        }
    }

    fn fetch_storage_value(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, RuntimeError> {
        match (self.storage_host, self.storage_account.as_ref()) {
            (Some(mut ptr), Some(account)) => {
                // # Safety
                //
                // Same guarantee as `build_storage_entries()`:
                // The storage_host pointer is valid for the execution context lifetime.
                // We only borrow mutably for the duration of this single operation.
                // No other code can access storage during this call since we're in a
                // single-threaded execution model with exclusive access to the context.
                let storage = unsafe { ptr.as_mut() };
                storage.get(account, key)
            }
            _ => Ok(None),
        }
    }
}
