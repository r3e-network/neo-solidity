/// Storage syscall implementations for Neo N3 VM compatibility.
///
/// This module implements the System.Storage.* syscall interface, providing persistent
/// key-value storage operations compatible with Neo N3's storage model.
///
/// # Storage Model
///
/// - **Keys**: Byte arrays (typically SHA-256 hashed for deterministic slot calculation)
/// - **Values**: Byte arrays (empty value represents deleted key)
/// - **Context**: Opaque handle representing storage context (simplified as empty byte array)
/// - **Gas**: Storage operations consume gas based on operation type and data size
///
/// # Syscall Signatures
///
/// ## Context Management
/// - `GetContext()` - Returns a storage context for regular operations
/// - `GetReadOnlyContext()` - Returns a read-only storage context
/// - `AsReadOnly(context)` - Converts a context to read-only view
///
/// ## Data Operations
/// - `Get(context, key)` - Retrieve value for key, returns empty if not found
/// - `Put(context, key, value)` - Store key-value pair, empty value deletes key
/// - `Delete(context, key)` - Remove key from storage
///
/// ## Local Operations (context-free)
/// - `Local.Get(key)` - Get from storage overlay (ephemeral during execution)
/// - `Local.Put(key, value)` - Put to storage overlay
/// - `Local.Delete(key)` - Delete from storage overlay
///
/// ## Query Operations
/// - `Find(context, prefix, options)` - Returns iterator for prefix search
/// - `Local.Find(prefix, options)` - Local version of Find
///
/// # Storage Overlay
///
/// During execution, modifications are tracked in `storage_overlay` as `OverlayEntry`:
/// - `value`: The value (Some) or deletion marker (None)
/// - `dirty`: Whether this entry needs to be persisted
/// - Changes are flushed via `drain_dirty_storage_overlay()` after execution
///
/// # Thread Safety
///
/// Storage operations modify internal state but are safe within single-threaded
/// execution model. Concurrent access must be synchronized externally.
///
/// # Gas Costs
///
/// - GetContext/GetReadOnlyContext/AsReadOnly: 1 gas
/// - Get: 100 gas
/// - Put: 1,000 gas
/// - Delete: 100 gas
/// - Find: 100 gas
/// - Local operations: Same as regular operations
///
/// # Examples
///
/// ## Basic storage operation
/// ```ignore
/// // In NeoVM bytecode (simplified)
/// PUSHDATA1 "my_key"
/// PUSHDATA1 "my_value"
/// SYSCALL System.Storage.GetContext
/// SYSCALL System.Storage.Put
/// ```
///
/// ## Prefix search with Find
/// ```ignore
/// PUSH0                      // options (unused)
/// PUSHDATA1 "prefix"         // search prefix
/// SYSCALL System.Storage.GetContext
/// SYSCALL System.Storage.Find
/// // Returns iterator token
/// SYSCALL System.Iterator.Next
/// SYSCALL System.Iterator.Value
/// ```
impl ExecutionContext {
    fn handle_storage_syscall(&mut self, name: &str) -> Result<bool, RuntimeError> {
        match name {
            "System.Storage.GetContext" | "System.Storage.GetReadOnlyContext" => {
                self.push_stack(StackItem::byte_array(Vec::new()))?;
                Ok(true)
            }
            "System.Storage.AsReadOnly" => {
                // Neo N3 converts a StorageContext into a readonly view. The embedded runtime
                // represents contexts as opaque byte arrays, so we simply pass it through.
                let context = self.pop_stack()?;
                self.push_stack(context)?;
                Ok(true)
            }
            "System.Storage.Get" => {
                let _context = self.pop_stack()?; // ignored
                let slot_item = self.pop_stack()?;
                let key = Self::stack_item_to_bytes(slot_item);

                let value = if let Some(entry) = self.storage_overlay.get(&key) {
                    entry.value.clone().unwrap_or_default()
                } else {
                    let fetched = self.fetch_storage_value(&key)?;
                    let bytes = fetched.clone().unwrap_or_default();
                    self.storage_overlay.insert(
                        key.clone(),
                        OverlayEntry {
                            value: fetched,
                            dirty: false,
                        },
                    );
                    bytes
                };

                self.push_stack(StackItem::byte_array(value))?;
                Ok(true)
            }
            "System.Storage.Put" => {
                // Syscall signature: System.Storage.Put(context, key, value)
                // Stack order: [value, key, context] (top-of-stack is `context`)
                let _context = self.pop_stack()?; // context
                let slot_item = self.pop_stack()?; // key
                let value_item = self.pop_stack()?; // value

                let key = Self::stack_item_to_bytes(slot_item);
                let value = Self::stack_item_to_bytes(value_item);

                let entry = self
                    .storage_overlay
                    .entry(key.clone())
                    .or_insert_with(|| OverlayEntry {
                        value: None,
                        dirty: false,
                    });
                entry.value = if value.is_empty() { None } else { Some(value) };
                entry.dirty = true;
                Ok(true)
            }
            "System.Storage.Delete" => {
                // Syscall signature: System.Storage.Delete(context, key)
                // Stack order: [key, context] (top-of-stack is `context`)
                let _context = self.pop_stack()?; // context
                let slot_item = self.pop_stack()?; // key
                let key = Self::stack_item_to_bytes(slot_item);
                self.storage_overlay.insert(
                    key,
                    OverlayEntry {
                        value: None,
                        dirty: true,
                    },
                );
                Ok(true)
            }
            "System.Storage.Local.Get" => {
                // Call signature: System.Storage.Local.Get(key)
                let slot_item = self.pop_stack()?;
                let key = Self::stack_item_to_bytes(slot_item);

                let value = if let Some(entry) = self.storage_overlay.get(&key) {
                    entry.value.clone().unwrap_or_default()
                } else {
                    let fetched = self.fetch_storage_value(&key)?;
                    let bytes = fetched.clone().unwrap_or_default();
                    self.storage_overlay.insert(
                        key.clone(),
                        OverlayEntry {
                            value: fetched,
                            dirty: false,
                        },
                    );
                    bytes
                };

                self.push_stack(StackItem::byte_array(value))?;
                Ok(true)
            }
            "System.Storage.Local.Put" => {
                // Call signature: System.Storage.Local.Put(key, value)
                // Stack order: [value, key] (top-of-stack is `key`)
                let key_item = self.pop_stack()?;
                let value_item = self.pop_stack()?;

                let key = Self::stack_item_to_bytes(key_item);
                let value = Self::stack_item_to_bytes(value_item);

                let entry = self
                    .storage_overlay
                    .entry(key.clone())
                    .or_insert_with(|| OverlayEntry {
                        value: None,
                        dirty: false,
                    });
                entry.value = if value.is_empty() { None } else { Some(value) };
                entry.dirty = true;
                Ok(true)
            }
            "System.Storage.Local.Delete" => {
                // Call signature: System.Storage.Local.Delete(key)
                let slot_item = self.pop_stack()?;
                let key = Self::stack_item_to_bytes(slot_item);
                self.storage_overlay.insert(
                    key,
                    OverlayEntry {
                        value: None,
                        dirty: true,
                    },
                );
                Ok(true)
            }
            "System.Storage.Find" => {
                // Call signature: Find(context, prefix, options)
                let _context = self.pop_stack()?;
                let prefix = Self::stack_item_to_bytes(self.pop_stack()?);
                let _options = self.pop_stack()?;

                let entries = self.build_storage_entries(prefix.clone())?;
                let token = self.allocate_iterator(entries);
                self.push_stack(token)?;
                Ok(true)
            }
            "System.Storage.Local.Find" => {
                // Call signature: Local.Find(prefix, options)
                let prefix = Self::stack_item_to_bytes(self.pop_stack()?);
                let _options = self.pop_stack()?;

                let entries = self.build_storage_entries(prefix.clone())?;
                let token = self.allocate_iterator(entries);
                self.push_stack(token)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
