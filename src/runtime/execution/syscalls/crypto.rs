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
                let sigs = Self::stack_item_to_bytes(self.pop_stack()?);
                let pubs = Self::stack_item_to_bytes(self.pop_stack()?);
                // Use the current transaction/message hash for verification
                let msg_hash = self.get_current_message_hash();
                // Treat as true only if both blobs can be split into at least one valid pair
                let ok = !pubs.is_empty()
                    && !sigs.is_empty()
                    && Self::verify_secp256k1_with_message(&msg_hash, &pubs, &sigs);
                self.push_stack(StackItem::Boolean(ok))?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

