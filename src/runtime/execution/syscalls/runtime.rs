impl ExecutionContext {
    fn handle_runtime_syscall(&mut self, name: &str) -> Result<bool, RuntimeError> {
        match name {
            "System.Runtime.GetNetwork" => {
                self.push_stack(StackItem::UnsignedInteger(self.network_magic as u64))?;
                Ok(true)
            }
            "System.Runtime.GetTrigger" => {
                // Default to Application trigger (0x10) for the embedded runtime.
                // This matches the most common execution context for deployed contracts.
                self.push_stack(StackItem::Integer(0x10))?;
                Ok(true)
            }
            "System.Runtime.Platform" => {
                self.push_stack(StackItem::byte_array(b"NEO".to_vec()))?;
                Ok(true)
            }
            "System.Runtime.GetAddressVersion" => {
                // Neo N3 default address version (ProtocolSettings.AddressVersion).
                // MainNet/TestNet use 53 (0x35).
                self.push_stack(StackItem::UnsignedInteger(53))?;
                Ok(true)
            }
            "System.Runtime.GasLeft" => {
                let remaining = self.gas_limit.saturating_sub(self.gas_used);
                self.push_stack(StackItem::UnsignedInteger(remaining))?;
                Ok(true)
            }
            "System.Runtime.GetInvocationCounter" => {
                self.push_stack(StackItem::UnsignedInteger(self.invocation_counter))?;
                Ok(true)
            }
            "System.Runtime.GetCallingScriptHash" => {
                // Task #123 — honour per-frame `msg.sender` overrides pushed
                // by `handle_contract_call` when entering a self-offsets
                // "virtual contract boundary". Walking the call stack here
                // (rather than mutating `caller_account`) keeps `tx.origin`
                // pinned to `Transaction.Sender` across nested frames while
                // still giving `msg.sender` the direct caller's identity.
                if let Some(override_bytes) = self.active_msg_sender_override() {
                    self.push_stack(StackItem::byte_array(override_bytes))?;
                    return Ok(true);
                }
                if self.caller_account.is_none() {
                    self.caller_account = Some(self.default_account_bytes.clone());
                    self.storage_account = Some(self.default_account.clone());
                }
                let bytes = self
                    .caller_account
                    .clone()
                    .unwrap_or_else(|| self.default_account_bytes.clone());
                self.push_stack(StackItem::byte_array(bytes))?;
                Ok(true)
            }
            "System.Runtime.GetEntryScriptHash" | "System.Runtime.GetExecutingScriptHash" => {
                self.push_stack(StackItem::byte_array(self.default_account_bytes.clone()))?;
                Ok(true)
            }
            "System.Runtime.LoadScript" => {
                // The embedded runtime does not support dynamic script loading.
                // Consume arguments to maintain stack discipline, then return error.
                let _args = self.pop_stack()?;
                let _call_flags = self.pop_stack()?;
                let _script = self.pop_stack()?;
                Err(RuntimeError::ExecutionError {
                    message: "System.Runtime.LoadScript is not supported in the embedded \
                             runtime. Use System.Contract.Call for inter-contract calls."
                        .to_string(),
                })
            }
            "System.Runtime.GetScriptContainer" => {
                // Return a Transaction-like array matching the Neo devpack field order:
                // [Hash, Version, Nonce, Sender, SystemFee, NetworkFee, ValidUntilBlock, Script]
                // Only `Sender` is currently used by the Solidity compiler (for msg.sender).
                let sender = self
                    .caller_account
                    .clone()
                    .unwrap_or_else(|| self.default_account_bytes.clone());
                let tx = StackItem::array(vec![
                    StackItem::byte_array(vec![0u8; 32]),
                    StackItem::UnsignedInteger(0),
                    StackItem::UnsignedInteger(0),
                    StackItem::byte_array(sender),
                    StackItem::Integer(0),
                    StackItem::Integer(0),
                    StackItem::UnsignedInteger(0),
                    StackItem::byte_array(self.input_data.clone()),
                ]);
                self.push_stack(tx)?;
                Ok(true)
            }
            "System.Runtime.GetTime" => {
                let timestamp = *self.timestamp.get_or_insert(self.default_timestamp);
                self.push_stack(StackItem::UnsignedInteger(timestamp))?;
                Ok(true)
            }
            // Task #113 — Solidity `msg.value` host-injection slot. Returns
            // the value the host injected via
            // `NeoRuntime::override_value` / `ExecutionOverrides::value`
            // for the current invocation. Coalesces `None` → 0 so source
            // that reads `msg.value` without a host override observes the
            // Neo-native "no attached value" default (matching the former
            // literal `push 0` lowering).
            "System.Runtime.GetMsgValue" => {
                let value = self.msg_value.unwrap_or(0);
                self.push_stack(StackItem::UnsignedInteger(value))?;
                Ok(true)
            }
            "System.Runtime.GetRandom" => {
                // Initialize seed on first call from block hash + tx context
                if self.random_seed.is_none() {
                    let height = self.block_height.unwrap_or(self.default_block_height);
                    let mut seed_input = height.to_le_bytes().to_vec();
                    seed_input.extend_from_slice(&self.default_account_bytes);
                    let hash = Sha256::digest(&seed_input);
                    let mut seed = [0u8; 32];
                    seed.copy_from_slice(&hash);
                    self.random_seed = Some(seed);
                }
                // Hash seed || counter for each call
                let seed = self.random_seed.ok_or_else(|| RuntimeError::ExecutionError {
                    message: "Random seed not initialized".to_string(),
                })?;
                let mut input = seed.to_vec();
                input.extend_from_slice(&self.random_counter.to_le_bytes());
                self.random_counter += 1;
                let hash = Sha256::digest(&input);
                self.push_stack(StackItem::byte_array(hash[..].to_vec()))?;
                Ok(true)
            }
            "System.Runtime.Notify" => {
                // Neo N3 signature: `Notify(eventName, stateArray)`.
                //
                // The Solidity frontend uses this same syscall to deliver an
                // EVM-canonical `LogEntry { topics, data }`. The three shapes
                // are distinguished by the `eventName` length:
                //
                //   * **EVM non-anonymous shape** — `eventName.len() == 32`
                //     AND `stateArray` is an `Array`. The 32-byte event_name
                //     is `keccak256("Name(type1,type2,...)")` i.e. `topic[0]`.
                //     The stateArray is `[topic1, topic2, ..., data]` —
                //     indexed-arg topics first (already 32 bytes each), then
                //     the abi-encoded non-indexed `data` payload as the final
                //     element. This matches Ethereum's log model: Etherscan,
                //     TheGraph, and Ethers consumers subscribe by the
                //     signature keccak and filter by indexed-topic values.
                //
                //   * **EVM anonymous shape** — `eventName.is_empty()` AND
                //     `stateArray` is an `Array`. Per the EVM ABI (and the
                //     Solidity handbook §Events), anonymous events suppress
                //     the signature-hash topic0 so they can carry up to 4
                //     indexed topics. Layout is the same as non-anonymous
                //     except NO topic0 is prepended — `topics` is composed
                //     solely of the indexed-arg slots from the stateArray.
                //     The empty event_name is the lowering sentinel (the
                //     non-anonymous and legacy paths both use non-empty
                //     names, so it uniquely identifies the anonymous case).
                //
                //   * **Legacy Neo shape** — any other combination. The
                //     event_name is a short ByteArray (the event's
                //     declaration-time name, e.g. `"Custom"`), and the
                //     stateArray is the full Neo-native payload. We preserve
                //     this path for `Syscalls.notify(name, data)` calls in
                //     devpack code and for any frontend that hasn't adopted
                //     the EVM shape.
                let event_name = self.pop_stack()?;
                let state = self.pop_stack()?;

                let event_name_bytes = Self::stack_item_to_bytes(event_name);
                let state_is_array = matches!(state, StackItem::Array(_));
                let is_evm_non_anonymous = event_name_bytes.len() == 32 && state_is_array;
                let is_evm_anonymous = event_name_bytes.is_empty() && state_is_array;

                if is_evm_non_anonymous || is_evm_anonymous {
                    // Split the stateArray into [topic1, topic2, ..., data].
                    // The data element is always the LAST element — indexed
                    // args precede it in declaration order.
                    let StackItem::Array(items_rc) = state else {
                        // Defensive — we just matched Array above.
                        return Ok(true);
                    };
                    let items = items_rc.borrow().clone();
                    drop(items_rc);

                    let (indexed_topics, data_bytes) = if items.is_empty() {
                        (Vec::new(), Vec::new())
                    } else {
                        let (indexed_slice, tail_slice) = items.split_at(items.len() - 1);
                        let topics: Vec<Vec<u8>> = indexed_slice
                            .iter()
                            .map(|item| Self::stack_item_to_bytes(item.clone()))
                            .collect();
                        let data =
                            Self::stack_item_to_bytes(tail_slice[0].clone());
                        (topics, data)
                    };

                    let mut topics = Vec::with_capacity(1 + indexed_topics.len());
                    if is_evm_non_anonymous {
                        // Non-anonymous: topic0 = keccak(signature) prepended.
                        topics.push(event_name_bytes);
                    }
                    // Anonymous: topics = indexed topics only; no topic0.
                    topics.extend(indexed_topics);
                    self.logs.push(LogEntry {
                        address: self.default_account.clone(),
                        topics,
                        data: data_bytes.clone(),
                    });
                    self.return_data = data_bytes;
                } else {
                    // Legacy Neo-native shape.
                    let bytes = Self::stack_item_to_bytes(state);
                    self.logs.push(LogEntry {
                        address: self.default_account.clone(),
                        topics: vec![event_name_bytes],
                        data: bytes.clone(),
                    });
                    self.return_data = bytes;
                }
                Ok(true)
            }
            "System.Runtime.Log" => {
                let msg = Self::stack_item_to_bytes(self.pop_stack()?);
                self.logs.push(LogEntry {
                    address: self.default_account.clone(),
                    topics: Vec::new(),
                    data: msg,
                });
                Ok(true)
            }
            "System.Runtime.CheckWitness" => {
                let witness_item = self.pop_stack()?;
                let caller_bytes = self
                    .caller_account
                    .clone()
                    .unwrap_or_else(|| self.default_account_bytes.clone());

                let check_bytes = |bytes: &[u8]| -> bool {
                    // If witness_signers is populated, check against it
                    if !self.witness_signers.is_empty() {
                        return self.witness_signers.iter().any(|s| s == bytes);
                    }
                    // Fall back to default account / caller check
                    bytes == caller_bytes || bytes == self.default_account_bytes
                };

                let is_match = match witness_item {
                    StackItem::Array(items) => items.borrow().iter().any(|w| {
                        let bytes = Self::stack_item_to_bytes(w.clone());
                        check_bytes(&bytes)
                    }),
                    other => {
                        let bytes = Self::stack_item_to_bytes(other);
                        check_bytes(&bytes)
                    }
                };
                self.push_stack(StackItem::Boolean(is_match))?;
                Ok(true)
            }
            "System.Runtime.GetNotifications" => {
                // Call signature: GetNotifications([hash160])
                let _hash = self.pop_stack()?;
                self.push_stack(StackItem::array(Vec::new()))?;
                Ok(true)
            }
            "System.Runtime.BurnGas" => {
                // Call signature: BurnGas(datoshi)
                let amount_item = self.pop_stack()?;
                let amount = match amount_item {
                    StackItem::UnsignedInteger(u) => u,
                    StackItem::Integer(i) if i > 0 => i as u64,
                    StackItem::ByteArray(bytes) => {
                        let mut buf = [0u8; 8];
                        for (i, b) in bytes.borrow().iter().take(8).enumerate() {
                            buf[i] = *b;
                        }
                        u64::from_le_bytes(buf)
                    }
                    _ => 0,
                };

                if amount == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "BurnGas: GAS must be positive".to_string(),
                    });
                }
                self.gas_used = self.gas_used.saturating_add(amount);
                Ok(true)
            }
            "System.Runtime.CurrentSigners" => {
                // The embedded runtime does not model transaction signers. Return an empty array.
                self.push_stack(StackItem::array(Vec::new()))?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
