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
                self.invocation_counter += 1;
                self.push_stack(StackItem::UnsignedInteger(self.invocation_counter))?;
                Ok(true)
            }
            "System.Runtime.GetCallingScriptHash" => {
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
                // Call signature: LoadScript(script, callFlags, args)
                // The embedded runtime doesn't support dynamic script loading. We still
                // consume the arguments to keep stack discipline.
                let _args = self.pop_stack()?;
                let _call_flags = self.pop_stack()?;
                let _script = self.pop_stack()?;
                Ok(true)
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
            "System.Runtime.GetRandom" => {
                let seed = self.invocation_counter.to_le_bytes().to_vec();
                let hash = Sha256::digest(&seed);
                self.push_stack(StackItem::byte_array(hash[..].to_vec()))?;
                Ok(true)
            }
            "System.Runtime.Notify" => {
                // Neo N3 signature: Notify(eventName, stateArray)
                let event_name = self.pop_stack()?;
                let state = self.pop_stack()?;
                let bytes = Self::stack_item_to_bytes(state);
                // Preserve event name as the first topic for inspection in tests/debugging.
                let topic = Self::stack_item_to_bytes(event_name);
                self.logs.push(LogEntry {
                    address: self.default_account.clone(),
                    topics: vec![topic],
                    data: bytes.clone(),
                });
                self.return_data = bytes;
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
                let is_match = match witness_item {
                    StackItem::Array(items) => items.borrow().iter().any(|w| {
                        let bytes = Self::stack_item_to_bytes(w.clone());
                        bytes == caller_bytes || bytes == self.default_account_bytes
                    }),
                    other => {
                        let bytes = Self::stack_item_to_bytes(other);
                        bytes == caller_bytes || bytes == self.default_account_bytes
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
