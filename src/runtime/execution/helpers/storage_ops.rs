use super::*;

/// Neo N3 `FindOptions` flags honoured by `System.Storage.Find` /
/// `System.Storage.Local.Find` (gap `nep11` — previously the bundled runtime
/// ignored the options argument and always yielded `[key, value]` structs,
/// diverging from real Neo N3 for `KeysOnly` / `RemovePrefix` / `ValuesOnly`
/// iterators such as the devpack NEP-11 `tokensOf` / `tokens` index scans).
mod find_options {
    // FindOptions.None is the absence of all flags (0x00).
    pub const KEYS_ONLY: i64 = 0x01;
    pub const REMOVE_PREFIX: i64 = 0x02;
    pub const VALUES_ONLY: i64 = 0x04;
    pub const DESERIALIZE_VALUES: i64 = 0x08;
    pub const PICK_FIELD_0: i64 = 0x10;
    pub const PICK_FIELD_1: i64 = 0x20;
    pub const BACKWARDS: i64 = 0x80;
    pub const ALL: i64 = KEYS_ONLY
        | REMOVE_PREFIX
        | VALUES_ONLY
        | DESERIALIZE_VALUES
        | PICK_FIELD_0
        | PICK_FIELD_1
        | BACKWARDS;
}

impl ExecutionContext {
    /// Validate a `FindOptions` bitmask with the same rules as the C# node
    /// (`Neo.SmartContract.ApplicationEngine.Storage`): unknown bits fault,
    /// `KeysOnly` excludes the value-shaping flags, `PickField*` are mutually
    /// exclusive and require `DeserializeValues`.
    fn validate_find_options(options: i64) -> Result<(), RuntimeError> {
        use find_options::*;
        let fail = |message: String| Err(RuntimeError::ExecutionError { message });
        if options & !ALL != 0 {
            return fail(format!(
                "Storage.Find: invalid FindOptions bits 0x{options:02x}"
            ));
        }
        if options & KEYS_ONLY != 0
            && options & (VALUES_ONLY | DESERIALIZE_VALUES | PICK_FIELD_0 | PICK_FIELD_1) != 0
        {
            return fail("Storage.Find: KeysOnly cannot be combined with value options".into());
        }
        if options & VALUES_ONLY != 0 && options & (KEYS_ONLY | REMOVE_PREFIX) != 0 {
            return fail("Storage.Find: ValuesOnly cannot be combined with key options".into());
        }
        if options & PICK_FIELD_0 != 0 && options & PICK_FIELD_1 != 0 {
            return fail("Storage.Find: PickField0 and PickField1 are mutually exclusive".into());
        }
        if options & (PICK_FIELD_0 | PICK_FIELD_1) != 0 && options & DESERIALIZE_VALUES == 0 {
            return fail("Storage.Find: PickField requires DeserializeValues".into());
        }
        Ok(())
    }

    /// Shape one `(key, value)` storage entry per the validated `options`
    /// bitmask, mirroring real Neo N3 iterator semantics.
    fn shape_find_entry(prefix: &[u8], options: i64, key: Vec<u8>, value: Vec<u8>) -> StackItem {
        use find_options::*;
        let key = if options & REMOVE_PREFIX != 0 {
            key[prefix.len().min(key.len())..].to_vec()
        } else {
            key
        };
        if options & KEYS_ONLY != 0 {
            return StackItem::byte_array(key);
        }
        let mut value_item = StackItem::byte_array(value);
        if options & DESERIALIZE_VALUES != 0 {
            let bytes = Self::stack_item_to_bytes(value_item);
            value_item = serde_json::from_slice::<StackItem>(&bytes).unwrap_or(StackItem::Null);
            let pick = if options & PICK_FIELD_0 != 0 {
                Some(0usize)
            } else if options & PICK_FIELD_1 != 0 {
                Some(1usize)
            } else {
                None
            };
            if let Some(index) = pick {
                value_item = match &value_item {
                    StackItem::Array(items) => items
                        .borrow()
                        .get(index)
                        .cloned()
                        .unwrap_or(StackItem::Null),
                    _ => StackItem::Null,
                };
            }
        }
        if options & VALUES_ONLY != 0 {
            return value_item;
        }
        StackItem::array(vec![StackItem::byte_array(key), value_item])
    }

    pub(crate) fn build_storage_entries(
        &self,
        prefix: Vec<u8>,
        options: i64,
    ) -> Result<Vec<StackItem>, RuntimeError> {
        Self::validate_find_options(options)?;
        let mut entries: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        // Bug #18: bound the host-query result set against `storage_limit` so a
        // host with a giant prefix-matching key set can't be weaponised into a
        // multi-GB Vec allocation here. The host-side `StorageQuery::limit`
        // field accepts a usize cap; using `storage_limit / MIN_ENTRY_BYTES`
        // gives a generous upper bound (a 10 MiB default with 16-byte
        // estimated min entry size = 640K entries) that still rejects truly
        // pathological queries before they materialise.
        const MIN_ENTRY_BYTES: usize = 16;
        let max_entries = self.storage_limit.saturating_div(MIN_ENTRY_BYTES).max(1);
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
                limit: Some(max_entries),
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

        // Final cap: even after merging the overlay, refuse to materialise a
        // result set bigger than `max_entries` (covers the case where the
        // overlay alone has > max_entries matching keys, since the host-side
        // `limit` only constrains the host query).
        if entries.len() > max_entries {
            return Err(RuntimeError::ExecutionError {
                message: format!(
                    "Storage.Find: result set ({} entries) exceeds storage_limit cap ({} entries)",
                    entries.len(),
                    max_entries
                ),
            });
        }

        entries.sort_by(|a, b| a.0.cmp(&b.0));
        if options & find_options::BACKWARDS != 0 {
            entries.reverse();
        }
        let mut items = Vec::with_capacity(entries.len());
        for (k, v) in entries {
            items.push(Self::shape_find_entry(&prefix, options, k, v));
        }
        Ok(items)
    }

    pub(crate) fn allocate_iterator(&mut self, entries: Vec<StackItem>) -> StackItem {
        let id = self.next_iterator_id;
        self.next_iterator_id = self.next_iterator_id.saturating_add(1);
        self.iterators
            .insert(id, IteratorState { entries, index: 0 });
        StackItem::byte_array(id.to_le_bytes().to_vec())
    }

    pub(crate) fn iterator_id_from_item(item: &StackItem) -> Option<u64> {
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

    pub(crate) fn register_contract(&mut self, nef: Vec<u8>, manifest: Vec<u8>) -> ContractState {
        fn nef_checksum(nef: &[u8]) -> u32 {
            if nef.len() >= 4 {
                let mut buf = [0u8; 4];
                let start = nef.len() - 4;
                buf.copy_from_slice(&nef[start..]);
                u32::from_le_bytes(buf)
            } else {
                let first = Sha256::digest(nef);
                let second = Sha256::digest(first);
                let mut checksum = [0u8; 4];
                checksum.copy_from_slice(&second[..4]);
                u32::from_le_bytes(checksum)
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

    pub(crate) fn update_contract(
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

    pub(crate) fn lookup_contract(&self, hash: &[u8]) -> Option<ContractState> {
        self.contract_registry.get(hash).cloned()
    }

    pub(crate) fn contract_to_stackitem(&self, state: &ContractState) -> StackItem {
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

    pub(crate) fn fetch_storage_value(
        &mut self,
        key: &[u8],
    ) -> Result<Option<Vec<u8>>, RuntimeError> {
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
