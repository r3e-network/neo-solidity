use super::*;

impl ExecutionContext {
    pub(crate) fn handle_contract_syscall(&mut self, name: &str) -> Result<bool, RuntimeError> {
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
                Self::append_pushdata(&mut script, &pubkey);
                script.push(0x41); // SYSCALL
                script
                    .extend_from_slice(&crate::interop::interop_id_bytes("System.Crypto.CheckSig"));

                // UInt160 = RIPEMD160(SHA256(script))
                let sha = Sha256::digest(&script);
                let hash160 = Ripemd160::digest(sha);
                self.push_stack(StackItem::byte_array(hash160[..].to_vec()))?;
                Ok(true)
            }
            "System.Contract.CreateMultisigAccount" => {
                // Signature: CreateMultisigAccount(m: Integer, pubkeys: Array) -> UInt160
                //
                // S4 fix: build the real Neo N3 multisig verification script
                //   PUSHINT <m> ; PUSHDATA <pk_1> ... PUSHDATA <pk_n> ;
                //   PUSHINT <n> ; SYSCALL System.Crypto.CheckMultisig
                // and return RIPEMD160(SHA256(script)) — matching the on-chain
                // account-derivation rule. The previous implementation returned
                // SHA256(m || pubkeys)[..20], which produced a UInt160 that
                // matched nothing on-chain.
                //
                // m and n are integers, so they use the PUSHINT8/16/32/64
                // encoding (mirroring how a real verification script would be
                // assembled); public keys are byte strings and use PUSHDATA1/2/4.
                let pubkeys_item = self.pop_stack()?;
                let m_item = self.pop_stack()?;
                let m: i64 = match m_item {
                    StackItem::Integer(i) => i,
                    StackItem::UnsignedInteger(u) => u as i64,
                    _ => {
                        return Err(RuntimeError::ExecutionError {
                            message: "CreateMultisigAccount: m must be an Integer".to_string(),
                        })
                    }
                };
                if m < 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "CreateMultisigAccount: m must be non-negative".to_string(),
                    });
                }

                let pubkeys: Vec<Vec<u8>> = match &pubkeys_item {
                    StackItem::Array(items) => items
                        .borrow()
                        .iter()
                        .map(|it| Self::stack_item_to_bytes(it.clone()))
                        .collect(),
                    _ => vec![Self::stack_item_to_bytes(pubkeys_item)],
                };

                let mut script: Vec<u8> = Vec::new();
                Self::append_push_int(&mut script, m);
                for pk in &pubkeys {
                    Self::append_pushdata(&mut script, pk);
                }
                Self::append_push_int(&mut script, pubkeys.len() as i64);
                script.push(0x41); // SYSCALL
                script
                    .extend_from_slice(&crate::interop::interop_id_bytes("System.Crypto.CheckMultisig"));

                // UInt160 = RIPEMD160(SHA256(script))
                let sha = Sha256::digest(&script);
                let hash160 = Ripemd160::digest(sha);
                self.push_stack(StackItem::byte_array(hash160[..].to_vec()))?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// Append a `PUSHDATA1/2/4`-encoded operand to `script`.
    ///
    /// Shared by [`System.Contract.CreateStandardAccount`] and
    /// [`System.Contract.CreateMultisigAccount`] so the single-sig and
    /// multi-sig account-derivation paths emit byte-identical verification
    /// scripts for the same operand. This matters because the resulting
    /// `UInt160 = RIPEMD160(SHA256(script))` is the on-chain account hash and
    /// any encoding divergence would silently produce a different address.
    pub(crate) fn append_pushdata(script: &mut Vec<u8>, data: &[u8]) {
        if data.len() <= u8::MAX as usize {
            script.push(0x0C); // PUSHDATA1
            script.push(data.len() as u8);
        } else if data.len() <= u16::MAX as usize {
            script.push(0x0D); // PUSHDATA2
            script.extend_from_slice(&(data.len() as u16).to_le_bytes());
        } else {
            script.push(0x0E); // PUSHDATA4
            script.extend_from_slice(&(data.len() as u32).to_le_bytes());
        }
        script.extend_from_slice(data);
    }

    /// Append a small integer to `script` using the NeoVM `PUSHINT8/16/32/64`
    /// opcodes (0x00–0x03), little-endian. Used by
    /// [`System.Contract.CreateMultisigAccount`] to emit the `m` and `n`
    /// operands of a multisig verification script the way a real Neo
    /// assembler would, so the derived account hash matches the on-chain
    /// derivation. Values must be non-negative.
    pub(crate) fn append_push_int(script: &mut Vec<u8>, value: i64) {
        if value >= 0 && value <= 0xFF {
            script.push(0x00); // PUSHINT8
            script.push(value as u8);
        } else if value >= 0 && value <= 0xFFFF {
            script.push(0x01); // PUSHINT16
            script.extend_from_slice(&(value as u16).to_le_bytes());
        } else if value >= 0 && value <= 0xFFFF_FFFF {
            script.push(0x02); // PUSHINT32
            script.extend_from_slice(&(value as u32).to_le_bytes());
        } else {
            script.push(0x03); // PUSHINT64
            script.extend_from_slice(&value.to_le_bytes());
        }
    }
}
