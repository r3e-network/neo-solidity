impl ExecutionContext {
    fn handle_crypto_syscall(&mut self, name: &str) -> Result<bool, RuntimeError> {
        match name {
            "System.Crypto.CheckSig" => {
                let sig_item = self.pop_stack()?;
                let pub_item = self.pop_stack()?;
                let pubkey = Self::stack_item_to_bytes(pub_item);
                let sig = Self::stack_item_to_bytes(sig_item);
                // Use the current transaction/message hash for verification
                // In a real implementation, this would come from the transaction context
                let msg_hash = self.get_current_message_hash();
                let ok = Self::verify_secp256k1_with_message(&msg_hash, &pubkey, &sig);
                self.push_stack(StackItem::Boolean(ok))?;
                Ok(true)
            }
            "System.Crypto.CheckMultisig" => {
                let sigs_item = self.pop_stack()?;
                let pubs_item = self.pop_stack()?;
                let msg_hash = self.get_current_message_hash();

                // Extract individual items from arrays
                let pub_items = match pubs_item {
                    StackItem::Array(items) => {
                        let items = items.borrow();
                        items.clone()
                    }
                    _ => {
                        // Fallback: treat as single pubkey
                        vec![pubs_item]
                    }
                };
                let sig_items = match sigs_item {
                    StackItem::Array(items) => {
                        let items = items.borrow();
                        items.clone()
                    }
                    _ => {
                        vec![sigs_item]
                    }
                };

                // M-of-N verification: each sig must match a pubkey in order
                let mut pub_idx = 0;
                let mut all_valid = !sig_items.is_empty() && !pub_items.is_empty();

                for sig_item in &sig_items {
                    let sig = Self::stack_item_to_bytes(sig_item.clone());
                    let mut found = false;
                    while pub_idx < pub_items.len() {
                        let pubkey = Self::stack_item_to_bytes(pub_items[pub_idx].clone());
                        pub_idx += 1;
                        if Self::verify_secp256k1_with_message(&msg_hash, &pubkey, &sig) {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        all_valid = false;
                        break;
                    }
                }

                self.push_stack(StackItem::Boolean(all_valid))?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

