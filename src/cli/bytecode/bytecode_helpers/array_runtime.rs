fn emit_new_array(bytecode: &mut Vec<u8>) {
    bytecode.push(0xC3); // NEWARRAY
}

fn emit_array_get(bytecode: &mut Vec<u8>) {
    bytecode.push(0xCE); // PICKITEM
}

fn emit_array_set(bytecode: &mut Vec<u8>) {
    bytecode.push(0xD0); // SETITEM
}

fn emit_load_runtime_value(
    bytecode: &mut Vec<u8>,
    value: &ir::RuntimeValue,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    match value {
        ir::RuntimeValue::MsgSender => {
            // Solidity `msg.sender` is the immediate caller:
            // - entry contract: Transaction.Sender (first signer)
            // - internal contract call: CallingScriptHash (caller contract)
            //
            // In Neo N3, contracts are typically invoked from an entry script (transaction
            // script) that then calls into the contract. Detect entry calls by checking
            // whether CallingScriptHash == EntryScriptHash.
            emit_syscall(bytecode, "System.Runtime.GetEntryScriptHash");
            emit_syscall(bytecode, "System.Runtime.GetCallingScriptHash");
            bytecode.push(0x97); // EQUAL

            // if !(calling == entry) jump to else (calling script hash)
            let jmp_if_not_pos = bytecode.len();
            bytecode.push(0x27); // JMPIFNOT_L
            let jmp_if_not_operand = bytecode.len();
            bytecode.extend_from_slice(&[0, 0, 0, 0]);

            // then: Transaction.Sender
            emit_syscall(bytecode, "System.Runtime.GetScriptContainer");
            // Neo N3 Transaction stack item layout (devpack order):
            // [Hash, Version, Nonce, Sender, ...]
            push_integer_bigint(bytecode, &BigInt::from(3u8));
            bytecode.push(0xCE); // PICKITEM (Transaction.Sender field)

            // jump to end
            let jmp_end_pos = bytecode.len();
            bytecode.push(0x23); // JMP_L
            let jmp_end_operand = bytecode.len();
            bytecode.extend_from_slice(&[0, 0, 0, 0]);

            // else:
            let else_pos = bytecode.len();
            emit_syscall(bytecode, "System.Runtime.GetCallingScriptHash");

            // end:
            let end_pos = bytecode.len();

            let rel_else = (else_pos as i32)
                .checked_sub(jmp_if_not_pos as i32)
                .unwrap_or(0);
            bytecode[jmp_if_not_operand..jmp_if_not_operand + 4]
                .copy_from_slice(&rel_else.to_le_bytes());

            let rel_end = (end_pos as i32).checked_sub(jmp_end_pos as i32).unwrap_or(0);
            bytecode[jmp_end_operand..jmp_end_operand + 4].copy_from_slice(&rel_end.to_le_bytes());
        }
        ir::RuntimeValue::MsgValue => {
            // Task #113 — lower Solidity `msg.value` to a runtime-side
            // syscall so the host can inject a value via
            // `NeoRuntime::override_value` /
            // `ExecutionOverrides::value`. The handler
            // (`src/runtime/execution/syscalls/runtime.rs`) pushes the
            // active override (or 0 when none is set), preserving the
            // previous "no attached value → 0" behaviour for callers
            // that never set an override while unblocking test harnesses
            // that need to drive `msg.value` to a specific u64 (e.g.
            // payable-fallback tests, NEP-17 `onPayment` checks).
            emit_syscall(bytecode, "System.Runtime.GetMsgValue");
        }
        ir::RuntimeValue::MsgData => {
            // Solidity `msg.data` == the raw calldata bytes the runtime received at
            // `execute(bytecode, input)`. Neo N3 exposes those bytes through the
            // `Script` slot (index 7) of the Transaction-shaped array returned by
            // `System.Runtime.GetScriptContainer`:
            //   [Hash, Version, Nonce, Sender, SystemFee, NetworkFee, ValidUntilBlock, Script]
            // (see src/runtime/execution/syscalls/runtime.rs "GetScriptContainer").
            emit_syscall(bytecode, "System.Runtime.GetScriptContainer");
            push_integer_bigint(bytecode, &BigInt::from(7u8));
            bytecode.push(0xCE); // PICKITEM — Transaction.Script (input_data)
        }
        ir::RuntimeValue::TxOrigin => {
            emit_syscall(bytecode, "System.Runtime.GetScriptContainer");
            // Neo N3 Transaction stack item layout (devpack order):
            // [Hash, Version, Nonce, Sender, ...]
            push_integer_bigint(bytecode, &BigInt::from(3u8));
            bytecode.push(0xCE); // PICKITEM (Transaction.Sender field)
        }
        // Neo's System.Runtime.GetTime returns milliseconds since epoch, while Solidity's
        // block.timestamp is seconds. Normalize here to preserve Solidity semantics.
        ir::RuntimeValue::BlockTimestamp => {
            emit_syscall(bytecode, "System.Runtime.GetTime");
            push_integer_bigint(bytecode, &BigInt::from(1000u64));
            bytecode.push(0xA1); // DIV
        }
        ir::RuntimeValue::BlockNumber => {
            emit_native_contract_call(
                bytecode,
                ir::NativeContract::Ledger,
                "currentIndex",
                0,
                use_callt,
                token_patches,
            );
        }
    }
}
