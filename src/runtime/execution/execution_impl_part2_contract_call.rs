impl ExecutionContext {
    fn handle_contract_call(&mut self) -> Result<(), RuntimeError> {
        // Neo N3 syscall convention: the first argument is at the top of the stack.
        // For System.Contract.Call(hash, method, flags, args), the evaluation stack
        // order is: [args, flags, method, hash].
        let contract_item = self.pop_stack()?;
        let method_item = self.pop_stack()?;
        let _flags = self.pop_stack()?; // call flags ignored in emulator
        let params = self.pop_stack()?;

        let method = String::from_utf8(Self::stack_item_to_bytes(method_item)).unwrap_or_default();
        let contract_bytes = Self::stack_item_to_bytes(contract_item);
        let mut hash = [0u8; 20];
        for (i, b) in contract_bytes.iter().take(20).enumerate() {
            hash[i] = *b;
        }

        // Task #70: route `this.someFn()` self external calls to the
        // compiled method offset from the manifest, so `invoke_native_contract`
        // doesn't return `Null` (silently 0/false for typed returns).
        //
        // Task #83 (batch #33 K1): also route the 20-byte zero placeholder
        // that `new Contract()` pushes for factory-deploy receivers — the
        // analyse phase merges the sibling's public methods into the host
        // (see `analyse_all_sources`), so `self_method_offsets` already
        // carries an entry for the target method name.
        let is_zero_placeholder = hash.iter().all(|b| *b == 0);
        if hash[..] == self.default_account_bytes[..] || is_zero_placeholder {
            // Task #126 — if the explicitly-named method isn't in the merged
            // self-offsets table (e.g. `Target(t).nonExistentMethod()` where
            // `TargetImpl` declares only `fallback()`), route to the
            // `fallback` entry instead. This mirrors Solidity's ABI
            // dispatcher: an unknown selector on a contract with a
            // `fallback()` falls through to that fallback. Without this
            // redirect, the unknown-method path would drop into
            // `invoke_native_contract`, which returns `Null` for the zero-
            // placeholder hash — silently absorbing the call and letting
            // the caller's try-arm fire with a bogus zero/empty result
            // instead of propagating the fallback's revert envelope.
            let resolved_offset = self
                .self_method_offsets
                .get(&method)
                .copied()
                .or_else(|| self.self_method_offsets.get("fallback").copied());
            if let Some(target_offset) = resolved_offset {
                // Unpack the params array and push args in reverse so that the
                // target method's INITSLOT pops them into arg slots in order.
                let args_vec = match &params {
                    StackItem::Array(items) => items.borrow().clone(),
                    _ => Vec::new(),
                };
                for arg in args_vec.iter().rev() {
                    self.push_stack(arg.clone())?;
                }
                // The SYSCALL opcode handler normally post-increments the
                // instruction pointer by 5 (see instruction/syscall.rs). The
                // return address must therefore be `instruction_pointer + 5`
                // (i.e. past the SYSCALL). We set IP directly to the target
                // method's compiled offset and raise the `suppress_ip_advance`
                // flag so the SYSCALL dispatcher skips its post-increment.
                let return_address = self.instruction_pointer.wrapping_add(5);
                // Task #123 — compute the synthetic "virtual caller" script
                // hash BEFORE pushing the frame (needs `&self`). The bundle's
                // `default_account_bytes` already represents the currently
                // executing contract (Middleware, in the canonical User →
                // Middleware → Caller scenario). We derive a distinct
                // caller-side identity from it so the nested callee's
                // `CallingScriptHash` reads back a value that differs from
                // both `EntryScriptHash` (which stays pinned to the bundle
                // hash) and `Transaction.Sender` (which backs `tx.origin`).
                //
                // Without this override, every self-offsets dispatch collapsed
                // `CallingScriptHash == EntryScriptHash`, short-circuiting the
                // `msg.sender` lowering in
                // `src/cli/bytecode/bytecode_helpers/array_runtime.rs`
                // (`RuntimeValue::MsgSender`) to `Transaction.Sender` — i.e.
                // `tx.origin` leaked into `msg.sender` on every nested call,
                // defeating the whole "only the direct caller can authorise
                // X" guard that the EVM convention relies on.
                let synthetic_caller =
                    Self::derive_self_offsets_caller_hash(&self.default_account_bytes);
                self.push_call_frame(return_address)?;
                if let Some(frame) = self.call_stack.last_mut() {
                    frame.msg_sender_override = Some(synthetic_caller);
                    // Task #160 — mark the frame so `return_from_function`
                    // synthesises a `StackItem::Null` result for void callees.
                    // The caller site emitted by `try_catch.rs` (and the generic
                    // `lower_expression_statement` path) assumes every external
                    // call leaves exactly one syscall result on the evaluation
                    // stack — matching what `invoke_native_contract` does on
                    // the non-zero-placeholder branch below. Without this flag,
                    // a void target's `ReturnVoid` (`0x40`) pushes nothing, and
                    // the caller's implicit `DROP` faults into the catch arm
                    // — turning a happy-path `try Target(t).voidFn() {}` into
                    // an unintended catch firing.
                    frame.syscall_result_expected = true;
                }
                self.instruction_pointer = target_offset;
                self.syscall_suppress_ip_advance = true;
                // Do NOT push a result here: the target method's RET will
                // leave its return value on the evaluation stack, which is
                // exactly what the caller (the SYSCALL site) expects.
                return Ok(());
            }
        }

        let result = self.invoke_native_contract(&hash, &method, params);
        self.push_stack(result)?;
        Ok(())
    }

    /// Task #123 — deterministic derivation of the "virtual caller" script
    /// hash for a self-offsets dispatch frame.
    ///
    /// Neo's native `CallingScriptHash` rolls over whenever the VM crosses a
    /// real contract boundary. The embedded runtime's self-offsets routing
    /// (see `handle_contract_call` above and the Task #83 sibling-merge pass
    /// in `src/solidity/solidity_analyse.rs::analyse_all_sources`) executes a
    /// simulated boundary *inside* the same compiled bundle, so there is no
    /// real script-hash rotation to observe. This helper synthesises a
    /// deterministic callee-facing identity by hashing the bundle's executing
    /// script hash with a fixed domain tag, guaranteeing it differs from both
    /// `default_account_bytes` (the entry/executing hash) and any plausible
    /// `Transaction.Sender` bytes that the host might inject.
    ///
    /// Determinism matters: the same bundle invoked with the same dispatch
    /// path always produces the same 20-byte value, so tests that assert on
    /// `msg.sender` byte content stay stable across runs.
    fn derive_self_offsets_caller_hash(executing: &[u8]) -> Vec<u8> {
        use ripemd::Ripemd160;
        use sha2::{Digest, Sha256};
        // Domain tag keeps the derivation distinct from any other `Hash160`
        // preimage the compiler emits (address derivation, bytecode hashing,
        // etc.) so accidental collisions are not possible.
        const DOMAIN: &[u8] = b"neo-solidity/self-offsets/msg-sender/v1";
        let mut preimage = Vec::with_capacity(DOMAIN.len() + executing.len());
        preimage.extend_from_slice(DOMAIN);
        preimage.extend_from_slice(executing);
        let sha = Sha256::digest(&preimage);
        let hash: [u8; 20] = Ripemd160::digest(sha).into();
        hash.to_vec()
    }

    /// Task #123 — resolve the active `msg.sender` (`CallingScriptHash`) for
    /// the current frame.
    ///
    /// Walks the call stack from the most recent frame downwards, returning
    /// the first `msg_sender_override` that is populated. When no frame
    /// carries an override (entry-depth execution, or a plain `CALL` frame
    /// from the Solidity internal-call lowering), this returns `None` and the
    /// `System.Runtime.GetCallingScriptHash` handler falls through to its
    /// pre-Task-123 `caller_account`-first behaviour.
    pub(crate) fn active_msg_sender_override(&self) -> Option<Vec<u8>> {
        for frame in self.call_stack.iter().rev() {
            if let Some(bytes) = frame.msg_sender_override.as_ref() {
                return Some(bytes.clone());
            }
        }
        None
    }
}
