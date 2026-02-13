impl ExecutionContext {
    fn handle_contract_syscall(&mut self, name: &str) -> Result<bool, RuntimeError> {
        match name {
            "System.Contract.Call" => {
                self.handle_contract_call()?;
                Ok(true)
            }
            "System.Contract.GetCallFlags" => {
                // Default to CallFlags.All (ReadStates | WriteStates | AllowCall | AllowNotify).
                self.push_stack(StackItem::UnsignedInteger(0x0F))?;
                Ok(true)
            }
            "System.Contract.CreateStandardAccount" => {
                // Signature: CreateStandardAccount(pubkey: ByteString) -> UInt160
                let pubkey_item = self.pop_stack()?;
                let pubkey = Self::stack_item_to_bytes(pubkey_item);

                // Script = PUSH <pubkey> + SYSCALL System.Crypto.CheckSig
                let mut script = Vec::with_capacity(2 + pubkey.len() + 1 + 4);
                if pubkey.len() <= u8::MAX as usize {
                    script.push(0x0C); // PUSHDATA1
                    script.push(pubkey.len() as u8);
                } else if pubkey.len() <= u16::MAX as usize {
                    script.push(0x0D); // PUSHDATA2
                    script.extend_from_slice(&(pubkey.len() as u16).to_le_bytes());
                } else {
                    script.push(0x0E); // PUSHDATA4
                    script.extend_from_slice(&(pubkey.len() as u32).to_le_bytes());
                }
                script.extend_from_slice(&pubkey);
                script.push(0x41); // SYSCALL
                script.extend_from_slice(&crate::codegen::interop_id_bytes("System.Crypto.CheckSig"));

                // UInt160 = RIPEMD160(SHA256(script))
                let sha = Sha256::digest(&script);
                let hash160 = Ripemd160::digest(sha);
                self.push_stack(StackItem::byte_array(hash160[..].to_vec()))?;
                Ok(true)
            }
            "System.Contract.CreateMultisigAccount" => {
                // Signature: CreateMultisigAccount(m: int, pubkeys: Array) -> UInt160
                // Keep this as a deterministic placeholder for now.
                let pubkeys = self.pop_stack()?;
                let m = self.pop_stack()?;
                let mut input = Vec::new();
                input.extend_from_slice(&Self::stack_item_to_bytes(m));
                input.extend_from_slice(&Self::stack_item_to_bytes(pubkeys));
                let digest = Sha256::digest(&input);
                self.push_stack(StackItem::byte_array(digest[0..20].to_vec()))?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
