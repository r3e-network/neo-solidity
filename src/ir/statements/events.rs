/// Lower a Solidity `emit Event(arg1, arg2, ...)` expression into the
/// EVM-canonical log-emission IR sequence.
///
/// # Generated shape
///
/// Per the EVM ABI spec (Solidity handbook §"Events"), an `emit` must
/// produce a log entry with:
///
///   topic[0]   = `keccak256("Name(type1,type2,...)")` — the canonical
///                signature hash, always 32 bytes, always present.
///   topic[1..] = one 32-byte slot per `indexed` parameter:
///                - static types (uint, int, bool, address, bytesN, enum):
///                  left-padded big-endian value
///                - dynamic types (string, bytes, arrays, structs):
///                  `keccak256(value)`.
///                Capped by Solidity at 3 indexed args for non-anonymous
///                events, but the emitter does not enforce this (the
///                frontend validates earlier).
///   data       = `abi.encode(non_indexed_args...)` — a single ByteArray
///                produced by the StdLib.abiEncode helper (BE-padded,
///                concatenated 32-byte slots per Task #44).
///
/// # Bytecode shape delivered to System.Runtime.Notify
///
/// The runtime expects `Notify(eventName, stateArray)`. We co-opt that
/// signature to carry the EVM shape:
///
///   eventName  = the 32-byte `topic[0]` (keccak signature hash)
///   stateArray = [topic[1], topic[2], ..., data]
///
/// The runtime's `System.Runtime.Notify` handler detects the EVM shape by
/// `event_name.len() == 32 && stateArray.is_array` and rebuilds a proper
/// EVM `LogEntry { topics: vec![topic0, ...indexed], data }`. Legacy
/// `Syscalls.notify(short_name, payload)` paths with non-32-byte names
/// remain on the original Neo-native path.
///
/// # Fallback
///
/// If the event name isn't registered in the module's `event_params_map`
/// (e.g., inherited-event resolution lost the definition), we fall back to
/// the legacy `EmitEventByName` path — the runtime preserves the old
/// single-topic Neo-native shape for those. The compiler also falls back
/// when any `lower_expression` subcall fails, to avoid emitting a
/// half-built log record.
fn lower_emit(expr: &Expression, ctx: &mut LoweringContext, instructions: &mut Vec<Instruction>) {
    let Expression::FunctionCall(_, func, args) = expr else {
        return;
    };
    let Expression::Variable(identifier) = func.as_ref() else {
        return;
    };

    // Fast path: event is known. Produce the EVM-canonical shape.
    if let Some(signature) = ctx.event_evm_signature(&identifier.name).cloned() {
        lower_emit_evm_shape(&identifier.name, &signature, args, ctx, instructions);
        return;
    }

    // Fallback: unknown event name (e.g., inherited event stripped by the
    // frontend). Emit the legacy Neo-native shape so the log still lands.
    lower_emit_legacy(&identifier.name, args, ctx, instructions);
}

fn lower_emit_evm_shape(
    event_name: &str,
    signature: &EventSignature,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    let original_len = instructions.len();

    // Partition args into (indexed, non_indexed) by parameter index.
    let mut indexed: Vec<&Expression> = Vec::new();
    let mut indexed_params: Vec<&EventParamInfo> = Vec::new();
    let mut non_indexed: Vec<&Expression> = Vec::new();
    for (idx, arg) in args.iter().enumerate() {
        let Some(info) = signature.params.get(idx) else {
            // Arg count mismatch vs. declaration — fall back to legacy path
            // rather than emit a malformed log.
            lower_emit_legacy(event_name, args, ctx, instructions);
            return;
        };
        if info.indexed {
            indexed.push(arg);
            indexed_params.push(info);
        } else {
            non_indexed.push(arg);
        }
    }

    // Step 1: push the `eventName` argument for System.Runtime.Notify.
    //
    // Non-anonymous: push topic[0] = keccak256(canonical signature) as a
    // 32-byte ByteArray. The runtime detects `event_name.len() == 32` and
    // routes through the EVM-shape branch, prepending topic[0] to the
    // topics vector.
    //
    // Anonymous: per the EVM ABI (and Solidity handbook §Events), the
    // signature-hash topic0 is suppressed so the event can carry up to 4
    // indexed topics (vs. 3 for non-anonymous). We push an empty ByteArray
    // as the event_name sentinel — the runtime recognises the zero-length
    // form and emits `topics = [indexed_topics...]` with NO topic0 prepend.
    // The zero-length sentinel is unambiguous: the legacy Neo-native path
    // always uses the declared event name (non-empty by construction) and
    // the EVM non-anonymous path uses a 32-byte keccak, so `len == 0`
    // uniquely identifies the anonymous-EVM case.
    if signature.is_anonymous {
        instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(Vec::new())));
    } else {
        instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
            signature.topic0.to_vec(),
        )));
    }

    // Step 2: for each indexed arg, lower the expression and then coerce to
    // a 32-byte topic slot.
    //   - static types → direct 32-byte BE-padded slot (via
    //     `lower_static_abi_slots_for_expr`, which produces real EVM-canonical
    //     bytes instead of the `StdLib.serialize` / JSON-shaped fallback).
    //   - dynamic types → Keccak256(value).
    //
    // Falling back to `CallBuiltin::AbiEncode(1)` is ONLY safe when the
    // expression isn't a static value type — that path goes through
    // `StdLib.serialize([arg])` and produces a Neo-native / JSON-shaped
    // blob rather than a 32-byte EVM topic slot. See the fuzz harness
    // `event_with_indexed_and_dynamic_args_lowers` which caught this when
    // `msg.sender` (an indexed static address) surfaced as ~130 bytes.
    let mut success = true;
    for (arg, info) in indexed.iter().zip(indexed_params.iter()) {
        if info.is_dynamic {
            if !lower_expression(arg, ctx, instructions) {
                success = false;
                break;
            }
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::Keccak256,
                arg_count: 1,
            });
            continue;
        }
        let pre = instructions.len();
        match lower_static_abi_slots_for_expr(arg, ctx, instructions, false) {
            Some(0) => {
                success = false;
                break;
            }
            Some(1) => {
                // 32-byte BE-padded slot already on the stack.
            }
            Some(n) => {
                // Struct-as-indexed: concat the per-field slots and hash,
                // matching Solidity's `keccak256(abi.encode(struct))` rule
                // for indexed reference-type args.
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::BytesConcat,
                    arg_count: n,
                });
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::Keccak256,
                    arg_count: 1,
                });
            }
            None => {
                instructions.truncate(pre);
                if !lower_expression(arg, ctx, instructions) {
                    success = false;
                    break;
                }
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::AbiEncode,
                    arg_count: 1,
                });
            }
        }
    }

    // Step 3: push `data = abi.encode(non_indexed_args...)`. Must come
    // AFTER the indexed topics so PACK captures them in the right order.
    //
    // Task #109 — `bytesN(..)` / `address(..)` cast args route through
    // `coerce_to_fixed_bytes`, which leaves the MEMCPY-returned destination
    // buffer on the stack BENEATH the canonical ByteString result (MEMCPY
    // pushes `dst` back per C-style memcpy semantics — see
    // src/runtime/execution/execution_impl_part3_bytes.rs::memcpy_bytes).
    // When such a cast appears among non-indexed args, the subsequent PACK
    // inside `AbiEncode` picks up the leaked buffer instead of an earlier
    // scalar arg, corrupting `data` (e.g. `emit E(uint256 x, bytes32 y)`
    // yields `bytes32(y) || bytes32(y)` instead of `bytes32(x) || bytes32(y)`,
    // and `x` then leaks into the stateArray as a separate topic).
    // Mirror the Task #66/#89 packed-encoding fix in builtins.rs: follow
    // each leaky arg with `Swap; Drop` to discard the leaked buffer and
    // keep only the canonical result.
    if success {
        if non_indexed.is_empty() {
            instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(Vec::new())));
        } else {
            let pre_data = instructions.len();
            let refs: Vec<&Expression> = non_indexed.clone();
            match lower_abi_encode_args_direct(&refs, ctx, instructions) {
                Some(true) => {
                    // EVM-canonical data bytes already on the stack.
                }
                Some(false) => {
                    success = false;
                }
                None => {
                    instructions.truncate(pre_data);
                    for arg in &non_indexed {
                        if !lower_expression(arg, ctx, instructions) {
                            success = false;
                            break;
                        }
                    }
                    if success {
                        instructions.push(Instruction::CallBuiltin {
                            builtin: BuiltinCall::AbiEncode,
                            arg_count: non_indexed.len(),
                        });
                    }
                }
            }
        }
    }

    if !success {
        instructions.truncate(original_len);
        lower_emit_legacy(event_name, args, ctx, instructions);
        return;
    }

    // Step 4: emit the `EmitEvent` / `EmitEventByName` IR node. The
    // bytecode emitter PACKs the (indexed_topics + data) elements into a
    // stateArray and calls Runtime.Notify; the runtime splits them back
    // out by recognising the 32-byte topic[0] in the event_name slot.
    let state_item_count = indexed.len() + 1; // +1 for the data slot.
    if let Some(index) = ctx.event_index_map.get(event_name) {
        instructions.push(Instruction::EmitEvent {
            event_index: *index,
            arg_count: state_item_count,
        });
    } else {
        instructions.push(Instruction::EmitEventByName {
            name: event_name.to_string(),
            arg_count: state_item_count,
        });
    }
}

fn lower_emit_legacy(
    event_name: &str,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    let original_len = instructions.len();
    instructions.push(Instruction::PushLiteral(LiteralValue::String(
        event_name.as_bytes().to_vec(),
    )));
    let mut success = true;
    for arg in args {
        if !lower_expression(arg, ctx, instructions) {
            success = false;
        }
    }

    if success {
        if let Some(index) = ctx.event_index_map.get(event_name) {
            instructions.push(Instruction::EmitEvent {
                event_index: *index,
                arg_count: args.len(),
            });
        } else {
            instructions.push(Instruction::EmitEventByName {
                name: event_name.to_string(),
                arg_count: args.len(),
            });
        }
        return;
    }

    instructions.truncate(original_len);
}
