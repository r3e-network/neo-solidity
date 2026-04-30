fn lower_return_statement(
    expr: Option<&Expression>,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    // Task #91 — inside an inlined library-storage body, redirect
    // `return expr;` into the synthesised slot + jump-to-end label instead of
    // emitting a raw `Return` that would exit the caller function.
    if let Some((slot, end_label)) = ctx.inline_return_target() {
        if let Some(expression) = expr {
            if lower_expression(expression, ctx, instructions) {
                if let Some(index) = slot {
                    instructions.push(Instruction::StoreLocal(index));
                } else {
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
            }
        }
        instructions.push(Instruction::Jump { target: end_label });
        return true;
    }

    // Task #114 — inside a function whose body was expanded with modifier
    // epilogues (`had_modifier_epilogue == true`), redirect `return expr;`
    // into the synthesised return slots + jump to the innermost
    // modifier-wrap break label. That label sits between the inlined body
    // and the modifier epilogue statements, so the epilogue (`locked = 0;`
    // etc.) still runs before the function performs its actual RET. If we
    // have not yet entered the modifier-wrap `do { } while(false)` scope
    // (e.g. `return` before the wrap for some pathological rewrite), fall
    // back to the redirect's `end_label` which sits after the whole
    // expanded body.
    if let Some((slots, fallback_label)) = ctx.modifier_return_target() {
        let jump_label = ctx
            .innermost_modifier_break_label()
            .unwrap_or(fallback_label);

        if let Some(expression) = expr {
            // Multi-return: `return (a, b);` where expr is Expression::List.
            // Pair each tuple element with the matching synthetic slot.
            if let Expression::List(_, params) = expression {
                if params.len() == slots.len() && slots.iter().all(|s| s.is_some()) {
                    // Iterate in reverse-store order: lower each arg, store
                    // into slot. We lower left-to-right and store each as we
                    // go (popping the stack in the correct order).
                    //
                    // Safer approach: lower all args (stack: [v_0, v_1, ...]),
                    // then store in reverse (v_N-1 first).
                    let pre_len = instructions.len();
                    let mut all_ok = true;
                    for (_, param) in params.iter() {
                        let Some(parameter) = param else {
                            all_ok = false;
                            break;
                        };
                        if !lower_expression(&parameter.ty, ctx, instructions) {
                            all_ok = false;
                            break;
                        }
                    }
                    if all_ok {
                        // Pop in reverse: last value lowered is at TOS.
                        for slot in slots.iter().rev() {
                            if let Some(idx) = slot {
                                instructions.push(Instruction::StoreLocal(*idx));
                            } else {
                                instructions.push(Instruction::Drop(ValueType::Any));
                            }
                        }
                        instructions.push(Instruction::Jump { target: jump_label });
                        return true;
                    }
                    instructions.truncate(pre_len);
                }
            }

            // Single-value return (or fall-through from multi-return
            // lowering failure): lower the expression, store into the first
            // slot.
            if lower_expression(expression, ctx, instructions) {
                if let Some(first_slot) = slots.first().and_then(|s| *s) {
                    instructions.push(Instruction::StoreLocal(first_slot));
                } else {
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
            }
        }
        instructions.push(Instruction::Jump { target: jump_label });
        return true;
    }
    if let Some(expression) = expr {
        // Task #64 — multi-value return lowering for Externally-callable
        // functions (Public/External).
        //
        // For `return (a, b);` on a function declared `returns (T, U)` we must
        // emit EVM-canonical ABI-encoded bytes (BE-padded 32-byte slots),
        // NOT a `StackItem::Array` that would serialize via serde_json at the
        // main-frame RET. We reuse the `abiEncode` StdLib handler wired up in
        // Task #44: lower each tuple element as a separate expression and
        // wrap the sequence in `CallBuiltin { AbiEncode, arg_count: N }`,
        // which the bytecode emitter lowers to PACK + StdLib.abiEncode.
        //
        // Guards:
        //   (1) only rewrite when the tuple arity matches the function's
        //       declared return arity (keeps `return (single_tuple);` or
        //       parenthesised single expressions on the Array path),
        //   (2) only rewrite for externally-callable functions — internal
        //       callers expect a StackItem::Array they can destructure via
        //       `ArrayGet` (see tuple_assignment_picks_items_from_returned_array).
        if let Expression::List(_, params) = expression {
            let return_types = ctx.return_types().to_vec();
            if ctx.is_externally_callable()
                && return_types.len() >= 2
                && params.len() == return_types.len()
            {
                let tuple_exprs: Option<Vec<Expression>> = params
                    .iter()
                    .map(|(_, param)| param.as_ref().map(|p| p.ty.clone()))
                    .collect();
                if let Some(tuple_exprs) = tuple_exprs.as_ref() {
                    if !tuple_exprs
                        .iter()
                        .any(|expr| matches!(expr, Expression::List(_, _)))
                    {
                        let original_len = instructions.len();
                        if let Some(ok) =
                            lower_abi_encode_args_direct_from_slice(tuple_exprs, ctx, instructions)
                        {
                            if ok {
                                instructions.push(Instruction::Return);
                                return true;
                            }
                            instructions.truncate(original_len);
                        }
                    }
                }

                let original_len = instructions.len();
                if return_types.iter().all(is_static_abi_slot_value_type) {
                    let mut success = true;
                    for ((_, param), value_type) in params.iter().zip(return_types.iter()) {
                        let Some(parameter) = param else {
                            success = false;
                            break;
                        };
                        if !lower_static_abi_return_expr_slot(
                            &parameter.ty,
                            value_type,
                            ctx,
                            instructions,
                        ) {
                            success = false;
                            break;
                        }
                    }
                    if success {
                        instructions.push(Instruction::CallBuiltin {
                            builtin: BuiltinCall::BytesConcat,
                            arg_count: return_types.len(),
                        });
                        instructions.push(Instruction::Return);
                        return true;
                    }
                    instructions.truncate(original_len);
                }

                let original_len = instructions.len();
                // Task #94 — flatten nested tuple return expressions so
                // `return ((1, 2), 3)` for `returns ((uint,uint), uint)`
                // emits 3 head slots (`01 || 02 || 03`) rather than wrapping
                // the inner tuple in a StackItem::Array (which `abiEncode`
                // would classify as dynamic and encode with a junk tail).
                // Spec: Solidity inlines a static inner tuple into the
                // parent's head section.
                let mut flat_count = 0usize;
                let mut success =
                    flatten_tuple_return_params(params, ctx, instructions, &mut flat_count);
                if success && flat_count < params.len() {
                    // Defensive: the flatten walk must produce at least one
                    // leaf per declared param. An empty inner tuple would
                    // silently drop slots — fall back to the legacy path.
                    success = false;
                }
                if success {
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::BytesConcat,
                        arg_count: flat_count,
                    });
                    instructions.push(Instruction::Return);
                    return true;
                }
                // Fall through to legacy path on failure.
                instructions.truncate(original_len);
            }
        }

        // Task #116 — `return abi.decode(buf, (T1, ..., Tn));` where every
        // `Ti` is a static 32-byte ABI type (uintN/intN/address/bool/bytesN)
        // and the function is externally-callable with matching return arity
        // collapses to a pure pass-through: the ABI canonical layout of the
        // return tuple IS the input buffer, so round-tripping through
        // `abiDecode → Array → abiEncode` would be wasted work. More
        // importantly, the intermediate `StackItem::Array` would fall
        // through to `stack_item_to_bytes` at the main-frame RET and
        // serialize as a serde_json blob (see batch50 Z3 witness: 236 bytes
        // of JSON instead of the expected 96-byte BE-packed tuple).
        //
        // Short-circuit the round trip: lower only the buffer, reuse the
        // same panic(0x41) short-buffer guard that Task #84 installed on
        // the direct `abi.decode` path, and hand the buffer to RET. The
        // outer main-frame RET then treats the buffer as-is since the
        // ByteArray arm of `stack_item_to_bytes` is already a verbatim
        // copy.
        if let Some(abi_decode_args) = extract_abi_decode_call(expression) {
            let return_types = ctx.return_types().to_vec();
            if ctx.is_externally_callable()
                && return_types.len() >= 2
                && abi_decode_types_match_return_arity(&abi_decode_args, return_types.len())
            {
                if let Some(expected_bytes) = abi_decode_expected_static_bytes(&abi_decode_args) {
                    if let Some(buffer_expr) = abi_decode_args.first() {
                        let pre_len = instructions.len();
                        if lower_expression(buffer_expr, ctx, instructions) {
                            let decode_ok_label = ctx.next_label();
                            instructions.push(Instruction::Dup);
                            instructions.push(Instruction::GetSize);
                            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                                BigInt::from(expected_bytes),
                            )));
                            instructions.push(Instruction::BinaryOp(BinaryOperator::Ne));
                            instructions.push(Instruction::JumpIf {
                                target: decode_ok_label,
                            });
                            emit_panic(0x41, instructions);
                            instructions.push(Instruction::Label(decode_ok_label));
                            instructions.push(Instruction::Return);
                            return true;
                        }
                        instructions.truncate(pre_len);
                    }
                }
            }

            // Task #127 — mixed-head/tail variant of the Task #116 short-
            // circuit. For `return abi.decode(buf, (uint, string, address))`
            // (or any tuple mixing static 32-byte slots with dynamic-tail
            // types like `string`/`bytes`/`T[]`) the EVM canonical encoding
            // of the return tuple is ALSO the input buffer verbatim: every
            // type (static or dynamic) occupies exactly one head slot, with
            // dynamic types' head slot holding an offset that points to
            // their length+payload tail section. Re-encoding would yield
            // the same byte sequence.
            //
            // The previous lowering path ran the buffer through the runtime
            // `abiDecode` handler, which for a mixed tuple returned a
            // `StackItem::Array` of per-slot items. That Array then fell
            // through to `stack_item_to_bytes` at the main-frame RET and
            // serialised as a serde_json blob (`{"type":"Array","value":...}`),
            // leaking 380 bytes of UTF-8 JSON where the caller expected the
            // 160-byte EVM-canonical re-encoding (batch58 HH4 witness).
            //
            // The short-circuit here is identical to the static-tuple case
            // (lower the buffer, guard its size, hand it to RET) with a
            // looser size guard: dynamic payloads vary, so we can only
            // assert `size >= head_slot_count * 32` — every declared type
            // contributes one head slot, and the tail must exist below.
            // When the guard fails we still panic(0x41) for parity with
            // the static case and Task #84's direct-decode guard.
            if ctx.is_externally_callable()
                && return_types.len() >= 2
                && abi_decode_types_match_return_arity_mixed(&abi_decode_args, return_types.len())
            {
                if let Some(buffer_expr) = abi_decode_args.first() {
                    let pre_len = instructions.len();
                    if lower_expression(buffer_expr, ctx, instructions) {
                        // Minimum buffer size = one 32-byte head slot per
                        // declared ABI type. Every type (static or dynamic)
                        // contributes exactly one head slot; the dynamic
                        // tails live below the head block at the offsets
                        // declared within the head slots themselves.
                        let min_bytes: u32 = (return_types.len() as u32).saturating_mul(32);
                        let decode_ok_label = ctx.next_label();
                        instructions.push(Instruction::Dup);
                        instructions.push(Instruction::GetSize);
                        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                            BigInt::from(min_bytes),
                        )));
                        // IR `JumpIf` is JMPIFNOT semantics (branches when the
                        // condition is FALSE), matching the Task #116 static
                        // case: we emit `size < min_bytes` (the BAD condition),
                        // then `JumpIf decode_ok` jumps when that condition is
                        // false — i.e. when `size >= min_bytes` (the GOOD case).
                        // When the guard fails (size < min_bytes) we fall
                        // through to panic(0x41), matching Task #84's short-
                        // buffer Panic(uint256) envelope.
                        instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
                        instructions.push(Instruction::JumpIf {
                            target: decode_ok_label,
                        });
                        emit_panic(0x41, instructions);
                        instructions.push(Instruction::Label(decode_ok_label));
                        instructions.push(Instruction::Return);
                        return true;
                    }
                    instructions.truncate(pre_len);
                }
            }

            if ctx.is_externally_callable() && return_types.len() == 1 {
                if let Some(decoded_types) = abi_decode_value_types(&abi_decode_args[1], ctx) {
                    let decoded_type = decoded_types.first();
                    let return_type = return_types.first();
                    let matching_single_type = decoded_types.len() == 1
                        && decoded_type.zip(return_type).is_some_and(|(decoded, ret)| {
                            decoded == ret
                                || (abi_static_slot_count(decoded) == Some(1)
                                    && abi_static_slot_count(ret) == Some(1))
                                || (abi_value_type_is_dynamic(decoded)
                                    && abi_value_type_is_dynamic(ret))
                        });

                    if matching_single_type
                        && decoded_type.is_some_and(abi_value_type_is_dynamic)
                    {
                        if let Some(buffer_expr) = abi_decode_args.first() {
                            let pre_len = instructions.len();
                            if lower_expression(buffer_expr, ctx, instructions) {
                                let decode_ok_label = ctx.next_label();
                                instructions.push(Instruction::Dup);
                                instructions.push(Instruction::GetSize);
                                instructions.push(Instruction::PushLiteral(
                                    LiteralValue::Integer(BigInt::from(64u32)),
                                ));
                                instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
                                instructions.push(Instruction::JumpIf {
                                    target: decode_ok_label,
                                });
                                emit_panic(0x41, instructions);
                                instructions.push(Instruction::Label(decode_ok_label));
                                instructions.push(Instruction::Return);
                                return true;
                            }
                            instructions.truncate(pre_len);
                        }
                    }
                }
            }
        }

        if lower_expression(expression, ctx, instructions) {
            // Task #137 — single-value dynamic-array return. For
            // `function f(uint[] memory a) external pure returns (uint[] memory)
            // { ... return a; }` the `return a;` lowers to a raw
            // `StackItem::Array` push that, at the main-frame RET, falls through
            // to `stack_item_to_bytes` and gets serde_json-wrapped
            // (`{"type":"Array","value":[...]}`). Task #121 fixed the internally
            // constructed `return abi.decode(abi.encode(a), (uint[]))` round
            // trip by routing its payload through the runtime `abiencode`
            // helper; the PARAMETER-passthrough shape (`return a;`) never
            // entered that helper because no explicit `abi.encode` call exists
            // in the source. Wrap the single-value externally-callable return
            // with `AbiEncode(1 arg)` so the runtime produces the EVM-canonical
            // `offset=32 || length=N || N × 32-byte BE-padded elements` layout
            // instead of the JSON wrapper.
            //
            // Scope: only `ValueType::Array(_)` fires today — `bytes memory`
            // and `string memory` returns already land as `StackItem::ByteArray`
            // (verbatim in `stack_item_to_bytes`) and existing tests (e.g.
            // `abi_encodePacked_small_width_matches_spec`) pin the raw-bytes
            // shape, so re-wrapping them would regress. Struct / mapping
            // single-value returns are out of scope until a harness exists.
            if ctx.is_externally_callable() {
                let return_types = ctx.return_types().to_vec();
                if return_types.len() == 1
                    && matches!(return_types.first(), Some(ValueType::Array(_)))
                {
                    // Task #185 — nested fixed-size array return (e.g.
                    // `uint[3][2] memory`). The EVM canonical encoding is a
                    // flat concat of leaf values: `pad32_be(a[0][0]) ||
                    // pad32_be(a[0][1]) || ... || pad32_be(a[N-1][M-1])`,
                    // NOT the dynamic-array `offset || length || elements`
                    // wrapper that generic `AbiEncode(Array)` emits. We
                    // detect this from the raw Solidity type string (e.g.
                    // "uint[3][2]") since `ValueType::Array` alone does not
                    // preserve fixed-size dimensions. When it matches, stash
                    // the array in a local, unroll the nested index
                    // traversal into per-leaf `ArrayGet` sequences, then emit
                    // each leaf as a direct 32-byte ABI slot.
                    let ret_ty_strings = ctx.return_type_strings();
                    if let Some(first_ty) = ret_ty_strings.first() {
                        if let Some(dims) = parse_nested_fixed_array_shape(first_ty) {
                            if let Some(leaf_type) =
                                array_leaf_static_value_type(return_types.first().unwrap())
                            {
                                // Cap the unrolled instruction count so a
                                // pathological return type like
                                // `uint[1000000000][1000000000] memory`
                                // (from a malicious source) doesn't OOM /
                                // time-out the compiler emitting billions
                                // of `LoadLocal`/`ArrayGet` pairs.
                                // Real contract returns top out at a few
                                // hundred leaves; 65_536 is a generous
                                // cap well above any legitimate use.
                                const MAX_FIXED_ARRAY_LEAVES: usize = 65_536;
                                let total_leaves: usize = dims
                                    .iter()
                                    .try_fold(1usize, |acc, d| acc.checked_mul(*d))
                                    .filter(|n| *n <= MAX_FIXED_ARRAY_LEAVES)
                                    .unwrap_or(0);
                                if total_leaves > 0 {
                                    // Stash the outer array in a temp local
                                    // so each per-leaf traversal can start
                                    // from the same base.
                                    let tmp_id = ctx.next_label();
                                    let array_local = ctx
                                        .allocate_local(format!("__flat_ret_arr_{tmp_id}"), None);
                                    instructions.push(Instruction::StoreLocal(array_local));

                                    // Enumerate every leaf coordinate in
                                    // row-major (outer-most first) order so
                                    // the flat output matches the Solidity
                                    // static encoding layout.
                                    let mut coord = vec![0usize; dims.len()];
                                    loop {
                                        // Walk from the stashed array down
                                        // through each dimension: load
                                        // array, PushLiteral(i_k), ArrayGet
                                        // for k in 0..dims.len().
                                        instructions.push(Instruction::LoadLocal(array_local));
                                        for &idx in &coord {
                                            instructions.push(Instruction::PushLiteral(
                                                LiteralValue::Integer(BigInt::from(idx as u64)),
                                            ));
                                            instructions.push(Instruction::ArrayGet);
                                        }
                                        if !emit_static_abi_slot_for_value_type(
                                            &leaf_type,
                                            ctx,
                                            instructions,
                                        ) {
                                            ctx.record_error("failed to encode fixed-array return leaf");
                                            return false;
                                        }

                                        // Advance to the next coordinate
                                        // (least-significant dim increments
                                        // first, with carry into outer
                                        // dims) until all coords exhausted.
                                        let mut i = dims.len();
                                        let mut done = true;
                                        while i > 0 {
                                            i -= 1;
                                            coord[i] += 1;
                                            if coord[i] < dims[i] {
                                                done = false;
                                                break;
                                            }
                                            coord[i] = 0;
                                        }
                                        if done {
                                            break;
                                        }
                                    }

                                    instructions.push(Instruction::CallBuiltin {
                                        builtin: BuiltinCall::BytesConcat,
                                        arg_count: total_leaves,
                                    });
                                    instructions.push(Instruction::Return);
                                    return true;
                                }
                            }
                        }
                    }
                    if emit_abi_encode_single_stack_value_for_type(
                        return_types.first().unwrap(),
                        ctx,
                        instructions,
                    )
                    .is_none()
                    {
                        instructions.push(Instruction::CallBuiltin {
                            builtin: BuiltinCall::AbiEncode,
                            arg_count: 1,
                        });
                    }
                }
            }
            instructions.push(Instruction::Return);
            return true;
        }
    } else {
        let return_types = ctx.return_types().to_vec();
        let return_slots = ctx.return_slots().to_vec();

        if return_types.is_empty() {
            instructions.push(Instruction::ReturnVoid);
            return true;
        }

        if return_slots.iter().any(|slot| slot.is_none()) {
            ctx.record_error_with_suggestion(
                "return without value requires named return variables for this function",
                "either use named return variables (e.g. function f() returns (uint256 result)) or provide an explicit return value",
            );
        }

        if return_types.len() == 1 {
            match return_slots.first().and_then(|slot| *slot) {
                Some(local_index) => instructions.push(Instruction::LoadLocal(local_index)),
                None => {
                    if let Some(value_type) = return_types.first() {
                        push_default_for_value_type(value_type, ctx, instructions);
                    } else {
                        instructions.push(Instruction::PushLiteral(LiteralValue::Null));
                    }
                }
            }
            instructions.push(Instruction::Return);
            return true;
        }

        // Task #64 — implicit multi-value return (named returns, no expr).
        // For externally-callable functions, push each declared return local
        // in order and hand the sequence to `abiEncode` so the main-frame RET
        // emits BE-packed bytes instead of a JSON-serialised Array. For
        // internal/private functions, preserve the legacy Array shape so
        // intra-contract callers can destructure via `ArrayGet`.
        if ctx.is_externally_callable() {
            let static_slot_return = return_types.iter().all(is_static_abi_slot_value_type);
            for (slot, value_type) in return_slots.iter().zip(return_types.iter()) {
                if let Some(local_index) = slot {
                    instructions.push(Instruction::LoadLocal(*local_index));
                } else {
                    push_default_for_value_type(value_type, ctx, instructions);
                }
                if static_slot_return
                    && !emit_static_abi_slot_for_value_type(value_type, ctx, instructions)
                {
                    ctx.record_error("failed to encode static ABI return slot");
                    return false;
                }
            }
            if static_slot_return {
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::BytesConcat,
                    arg_count: return_types.len(),
                });
            } else {
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::AbiEncode,
                    arg_count: return_types.len(),
                });
            }
            instructions.push(Instruction::Return);
            return true;
        }

        // Legacy Array-packed shape for internal/private multi-return.
        let tmp_id = ctx.next_label();
        let tuple_local = ctx.allocate_local(
            format!("__return_tuple_{tmp_id}"),
            Some(ValueType::Array(Box::new(ValueType::Any))),
        );

        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::from(return_types.len() as u64),
        )));
        instructions.push(Instruction::NewArray {
            element_type: ValueType::Any,
        });
        instructions.push(Instruction::StoreLocal(tuple_local));

        for (index, (slot, value_type)) in return_slots.iter().zip(return_types.iter()).enumerate()
        {
            instructions.push(Instruction::LoadLocal(tuple_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(index as u64),
            )));

            if let Some(local_index) = slot {
                instructions.push(Instruction::LoadLocal(*local_index));
            } else {
                push_default_for_value_type(value_type, ctx, instructions);
            }

            instructions.push(Instruction::ArraySet);
        }

        instructions.push(Instruction::LoadLocal(tuple_local));
        instructions.push(Instruction::Return);
        return true;
    }
    false
}

/// Task #116 — true iff `expr` is `abi.decode(buf, types)` (with 2 args)
/// where `buf` is any expression and `types` is the tuple type spec.
/// Returns a Vec<Expression> clone of the arg slice for downstream
/// helpers that only take `&[Expression]` (matches the shape those
/// helpers already accept, e.g. `abi_decode_expected_static_bytes`).
fn extract_abi_decode_call(expr: &Expression) -> Option<Vec<Expression>> {
    let Expression::FunctionCall(_, func, args) = expr else {
        return None;
    };
    if args.len() != 2 {
        return None;
    }
    let Expression::MemberAccess(_, receiver, member) = func.as_ref() else {
        return None;
    };
    if member.name != "decode" {
        return None;
    }
    match receiver.as_ref() {
        Expression::Variable(id) if id.name == "abi" => Some(args.clone()),
        _ => None,
    }
}

/// Task #116 — true iff the second arg to `abi.decode` is a tuple of
/// exactly `expected_arity` static 32-byte types (uintN / intN / address /
/// bool / bytesN). Callers use this to decide whether
/// `return abi.decode(buf, tuple)` can short-circuit via a verbatim
/// buffer return: the ABI canonical layout of a static tuple IS the
/// input buffer, so no decode/re-encode round trip is needed.
fn abi_decode_types_match_return_arity(args: &[Expression], expected_arity: usize) -> bool {
    if expected_arity < 2 {
        return false;
    }
    let Some(types_arg) = args.get(1) else {
        return false;
    };
    match types_arg {
        Expression::List(_, params) => {
            if params.len() != expected_arity {
                return false;
            }
            params
                .iter()
                .all(|(_, param)| param.as_ref().is_some_and(|p| is_static_abi_type(&p.ty)))
        }
        // Parenthesis / single-type forms can never match arity >= 2.
        _ => false,
    }
}

/// Task #127 — true iff the second arg to `abi.decode` is a tuple of
/// exactly `expected_arity` ABI-decodable types (either static 32-byte
/// types OR dynamic-tail types like `string`/`bytes`/`T[]`), AND at
/// least one element is dynamic. This is the "mixed head/tail" shape
/// that the all-static matcher (`abi_decode_types_match_return_arity`)
/// deliberately rejects.
///
/// The short-circuit for this shape is still "pass the input buffer
/// through verbatim": Solidity's EVM canonical encoding of a tuple
/// with dynamic tails uses offset indirection in the head slots to
/// point at the length+payload tail — which is exactly the input
/// layout the caller handed to `abi.decode`. Re-encoding would yield
/// an identical byte sequence (modulo trivial offset arithmetic that's
/// already baked into the buffer).
///
/// This matcher exists separately from `abi_decode_types_match_return_arity`
/// because (a) the static case can emit a tighter size guard
/// (`size == N*32`) whereas the mixed case must use a loose lower
/// bound (`size >= head_slots*32`) since dynamic payloads vary, and
/// (b) future refinements (e.g. enforcing that offsets are in-range)
/// can slot in here without perturbing the static-tuple path.
fn abi_decode_types_match_return_arity_mixed(args: &[Expression], expected_arity: usize) -> bool {
    if expected_arity < 2 {
        return false;
    }
    let Some(types_arg) = args.get(1) else {
        return false;
    };
    let Expression::List(_, params) = types_arg else {
        return false;
    };
    if params.len() != expected_arity {
        return false;
    }
    let mut all_ok = true;
    let mut has_dynamic = false;
    for (_, param) in params.iter() {
        let Some(p) = param.as_ref() else {
            return false;
        };
        if is_static_abi_type(&p.ty) {
            continue;
        }
        if is_dynamic_abi_type(&p.ty) {
            has_dynamic = true;
            continue;
        }
        all_ok = false;
    }
    all_ok && has_dynamic
}

/// Task #94 — lower the children of a tuple-return expression, recursively
/// flattening nested `Expression::List` into leaf values. Each leaf is a
/// single expression lowered via `lower_expression`; `flat_count` is
/// incremented once per leaf so the caller can pass the right `arg_count`
/// to `AbiEncode`.
///
/// For `return ((1, 2), 3)` with `returns ((uint,uint), uint)`, this emits
/// pushes for `1`, `2`, `3` — three static head slots, which `abiEncode`
/// packs into `32z|01 || 32z|02 || 32z|03`, matching Solidity's inlining
/// of a static inner tuple into its parent's head section.
fn flatten_tuple_return_params(
    params: &[(solang_parser::pt::Loc, Option<solang_parser::pt::Parameter>)],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    flat_count: &mut usize,
) -> bool {
    for (_, param) in params.iter() {
        let Some(parameter) = param else {
            return false;
        };
        match &parameter.ty {
            Expression::List(_, inner_params) => {
                if !flatten_tuple_return_params(inner_params, ctx, instructions, flat_count) {
                    return false;
                }
            }
            expr => {
                let Some(value_type) = infer_type_from_expression(expr, ctx) else {
                    return false;
                };
                if !lower_static_abi_return_expr_slot(expr, &value_type, ctx, instructions) {
                    return false;
                }
                *flat_count += 1;
            }
        }
    }
    true
}

/// Task #112 — left-align a bytesN value (ByteArray of length N where
/// 1 ≤ N < 32) into a 32-byte buffer. The expected stack shape on entry is
/// `[.., src_bytes]`; on exit it is `[.., dst_bytes32]`.
///
/// Emits: allocate a fresh 32-byte NewBuffer, MEMCPY `N` bytes from `src[0..]`
/// into `dst[0..N]`, then re-push the destination and canonicalise as
/// ByteArray. The tail 32-N bytes remain zero-padded — matching
/// `abi.encode(bytesN)`'s spec (left-aligned content, zero-padded on the right).
fn emit_pad_bytesn_to_32(ctx: &mut LoweringContext, instructions: &mut Vec<Instruction>, n: usize) {
    let tmp_id = ctx.next_label();
    let src_local = ctx.allocate_local(format!("__bytesn_pad_src_{tmp_id}"), None);
    let dst_local = ctx.allocate_local(format!("__bytesn_pad_dst_{tmp_id}"), None);

    // Store the source bytesN buffer (top of stack) into a local so we can
    // reference it twice (for MEMCPY src + the size-min computation below).
    instructions.push(Instruction::StoreLocal(src_local));

    // Allocate a zero-initialised 32-byte destination buffer.
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u64),
    )));
    instructions.push(Instruction::NewBuffer);
    instructions.push(Instruction::StoreLocal(dst_local));

    // Compute count = min(src.len(), n). Solidity `bytesN` guarantees the
    // source buffer has EXACTLY N bytes, but static bytesN produced by
    // `bytesN(<shorter_slice>)` can be shorter (the compiler's
    // `coerce_to_fixed_bytes` caps this at N) — defensively clamp.
    let size_local = ctx.allocate_local(format!("__bytesn_pad_size_{tmp_id}"), None);
    let count_local = ctx.allocate_local(format!("__bytesn_pad_count_{tmp_id}"), None);
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(size_local));

    let ge_label = ctx.next_label();
    let end_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(n as u64),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: ge_label });
    // size >= n → count = n
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::StoreLocal(count_local));
    instructions.push(Instruction::Jump { target: end_label });
    instructions.push(Instruction::Label(ge_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(n as u64),
    )));
    instructions.push(Instruction::StoreLocal(count_local));
    instructions.push(Instruction::Label(end_label));

    // NeoVM MEMCPY stack order: [dst, dst_offset, src, src_offset, count].
    // dst_offset = 0 (left-aligned per EVM `abi.encode(bytesN)` layout).
    instructions.push(Instruction::LoadLocal(dst_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::MemCpy);
    // Real NeoVM MEMCPY: Pop 5, Push 0. Load dst explicitly.
    instructions.push(Instruction::LoadLocal(dst_local));
    // Canonicalise via Convert so the downstream `abiEncode`
    // sees a stable ByteString (value equality with storage-loaded slots).
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
}

fn is_static_abi_slot_value_type(value_type: &ValueType) -> bool {
    matches!(
        value_type,
        ValueType::Integer { .. }
            | ValueType::Boolean
            | ValueType::Address
            | ValueType::ByteArray {
                fixed_len: Some(1..=32)
            }
    )
}

fn emit_static_abi_slot_for_value_type(
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    match value_type {
        ValueType::Integer { .. } | ValueType::Boolean | ValueType::Address => {
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            emit_revert_static_slot_32(ctx, instructions, true);
            true
        }
        ValueType::ByteArray {
            fixed_len: Some(len),
        } if *len == 32 => true,
        ValueType::ByteArray {
            fixed_len: Some(len),
        } if *len < 32 => {
            emit_pad_bytesn_to_32(ctx, instructions, *len as usize);
            true
        }
        _ => false,
    }
}

fn lower_static_abi_return_expr_slot(
    expr: &Expression,
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    if !lower_expression(expr, ctx, instructions) {
        return false;
    }
    emit_static_abi_slot_for_value_type(value_type, ctx, instructions)
}

fn is_direct_static_revert_arg(expr: &Expression, ctx: &LoweringContext) -> bool {
    if resolve_struct_type_for_revert_arg(expr, ctx).is_some() {
        return false;
    }
    matches!(
        infer_type_from_expression(expr, ctx),
        Some(ValueType::Integer { signed: false, .. })
            | Some(ValueType::Boolean)
            | Some(ValueType::Address)
            | Some(ValueType::ByteArray { fixed_len: Some(_) })
    )
}

fn emit_revert_static_slot_32(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    reverse: bool,
) {
    let tmp_id = ctx.next_label();
    let src_local = ctx.allocate_local(format!("__revert_slot_src_{tmp_id}"), None);
    let dst_local = ctx.allocate_local(format!("__revert_slot_dst_{tmp_id}"), None);
    let size_local = ctx.allocate_local(format!("__revert_slot_size_{tmp_id}"), None);
    let count_local = ctx.allocate_local(format!("__revert_slot_count_{tmp_id}"), None);

    instructions.push(Instruction::StoreLocal(src_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u64),
    )));
    instructions.push(Instruction::NewBuffer);
    instructions.push(Instruction::StoreLocal(dst_local));

    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(size_local));

    let ge_label = ctx.next_label();
    let end_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u64),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: ge_label });
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::StoreLocal(count_local));
    instructions.push(Instruction::Jump { target: end_label });
    instructions.push(Instruction::Label(ge_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u64),
    )));
    instructions.push(Instruction::StoreLocal(count_local));
    instructions.push(Instruction::Label(end_label));

    instructions.push(Instruction::LoadLocal(dst_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::MemCpy);
    // Real NeoVM MEMCPY: Pop 5, Push 0. Nothing to discard.

    if reverse {
        instructions.push(Instruction::LoadLocal(dst_local));
        instructions.push(Instruction::LoadLocal(dst_local));
        instructions.push(Instruction::ReverseItems);
        instructions.push(Instruction::Convert {
            target: ConvertTarget::ByteArray,
        });
    } else {
        instructions.push(Instruction::LoadLocal(dst_local));
        instructions.push(Instruction::Convert {
            target: ConvertTarget::ByteArray,
        });
    }
}

fn lower_direct_static_revert_arg_slot(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let Some(value_type) = infer_type_from_expression(expr, ctx) else {
        return false;
    };
    if !lower_expression(expr, ctx, instructions) {
        return false;
    }
    // No Swap; Drop needed: real NeoVM MEMCPY pushes nothing.

    match value_type {
        ValueType::Integer { signed: false, .. } | ValueType::Boolean | ValueType::Address => {
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            emit_revert_static_slot_32(ctx, instructions, true);
            true
        }
        ValueType::ByteArray {
            fixed_len: Some(len),
        } => {
            if len == 32 {
                true
            } else if len < 32 {
                emit_pad_bytesn_to_32(ctx, instructions, len as usize);
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Task #181 — recursively render a `ValueType` as its EVM-canonical ABI
/// signature fragment. Used by `revert_arg_canonical_type` to expand struct
/// args into their parenthesised tuple form (`(address,uint256)`) instead of
/// the bare struct-name or fallback `bytes` token.
///
/// Mirrors `utils::canonical_param_type_with_structs` but operates on the
/// IR's `ValueType` tree (which already carries resolved field types) so
/// the revert-selector path doesn't need the solang-level string→fields map
/// that `solidity_analyse.rs` built for Task #106.
fn value_type_canonical_abi(ty: &ValueType) -> String {
    match ty {
        ValueType::Integer { signed: true, bits } => format!("int{bits}"),
        ValueType::Integer {
            signed: false,
            bits,
        } => format!("uint{bits}"),
        ValueType::Boolean => "bool".to_string(),
        ValueType::String => "string".to_string(),
        ValueType::Address => "address".to_string(),
        ValueType::ByteArray {
            fixed_len: Some(len),
        } => format!("bytes{len}"),
        ValueType::ByteArray { fixed_len: None } => "bytes".to_string(),
        ValueType::Array(inner) => format!("{}[]", value_type_canonical_abi(inner)),
        ValueType::Struct { fields, .. } => {
            let parts: Vec<String> = fields
                .iter()
                .map(|f| value_type_canonical_abi(&f.ty))
                .collect();
            format!("({})", parts.join(","))
        }
        // Mapping args cannot actually appear in an ABI signature; fall back
        // to `bytes` for robustness (same policy as the legacy surface).
        ValueType::Mapping { .. } | ValueType::Any => "bytes".to_string(),
    }
}

/// Task #181 — resolve the struct type of a revert-arg expression.
///
/// `infer_type_from_expression` handles the easy cases (Variable bound to a
/// struct param/local/state, MemberAccess into a struct). The revert-arg
/// harness also hits STRUCT-CONSTRUCTOR CALLS — `revert NotAuthorized(Actor(addr, 5))` —
/// where the callee is a bare `Variable("Actor")` naming a struct and the
/// 2-arg function-call shape has no inference rule. Look that up via
/// `resolve_struct_type_by_name` (shared with the struct-constructor
/// lowering) so the selector and flatten paths see the same fields.
fn resolve_struct_type_for_revert_arg(
    expr: &Expression,
    ctx: &LoweringContext,
) -> Option<ValueType> {
    let mut current = expr;
    while let Expression::Parenthesis(_, inner) = current {
        current = inner.as_ref();
    }
    if let Some(ty) = infer_type_from_expression(current, ctx) {
        if matches!(ty, ValueType::Struct { .. }) {
            return Some(ty);
        }
    }
    if let Expression::FunctionCall(_, func, _) = current {
        let name = match func.as_ref() {
            Expression::Variable(id) => Some(id.name.clone()),
            Expression::MemberAccess(_, _, id) => Some(id.name.clone()),
            _ => None,
        }?;
        return resolve_struct_type_by_name(ctx, &name);
    }
    None
}

/// Canonical-type signature for an argument passed to a custom-error revert.
///
/// Task #27 (compiler slice) — the EVM-style payload is
/// `keccak256("ErrorName(t1,t2,...)")[0..4] || abi.encode(args)`. For the
/// selector to match what Ethereum tooling expects, each `ti` must use
/// Solidity's canonical ABI form (`uint256`, not `uint`; `bytes32`, not a
/// struct display string).
///
/// We infer the type from the argument expression; for the common
/// integer-literal case this defaults to `uint256`, which matches the
/// Solidity language rule that number literals implicitly convert to the
/// error's declared parameter type. Unknown types degrade to `bytes` — a
/// conservative default that yields a stable (though non-matching)
/// selector so the payload still flows through the runtime.
///
/// Task #181 — struct args (both Variable references and constructor calls
/// like `Actor(addr, role)`) now expand to their canonical tuple form
/// `(t1,t2,...)` rather than falling through to `bytes`. Extends the Task
/// #106 struct-expansion pattern from `abi.encodeCall` to custom-error
/// reverts so `revert NotAuthorized(Actor(addr, 5))` emits the spec
/// selector `keccak256("NotAuthorized((address,uint256))")[..4]`.
fn revert_arg_canonical_type(expr: &Expression, ctx: &LoweringContext) -> String {
    // Struct-typed args first: cover both the Variable path (handled by
    // infer) AND the constructor-call path (`Actor(...)`) which has no
    // inference rule but resolves via `resolve_struct_type_by_name`.
    if let Some(struct_ty) = resolve_struct_type_for_revert_arg(expr, ctx) {
        return value_type_canonical_abi(&struct_ty);
    }
    match infer_type_from_expression(expr, ctx) {
        Some(ValueType::Integer { signed: true, bits }) => format!("int{bits}"),
        Some(ValueType::Integer {
            signed: false,
            bits,
        }) => format!("uint{bits}"),
        Some(ValueType::Boolean) => "bool".to_string(),
        Some(ValueType::String) => "string".to_string(),
        Some(ValueType::Address) => "address".to_string(),
        Some(ValueType::ByteArray {
            fixed_len: Some(len),
        }) => format!("bytes{len}"),
        Some(ValueType::ByteArray { fixed_len: None }) => "bytes".to_string(),
        Some(ValueType::Array(inner)) => format!("{}[]", value_type_canonical_abi(&inner)),
        _ => "bytes".to_string(),
    }
}

/// Task #181 — lower a single revert arg and, when the arg is a struct with
/// all-static fields, flatten it into per-field stack items so the downstream
/// `AbiEncode` builtin sees N scalar args (one per field) instead of a single
/// `StackItem::Array` that the runtime classifier would treat as dynamic and
/// wrap in Task #121's `offset || length || elements` layout.
///
/// Returns the number of stack items pushed (1 for non-struct args, N for
/// flattened struct args) so the caller can thread the correct `arg_count`
/// into `CallBuiltin { AbiEncode, arg_count }`. Returns 0 on lowering
/// failure; callers treat that as a fatal condition and fall back to the
/// bare error-name string path.
///
/// Mirrors the Task #124 `try_flatten_struct_arg_for_abi_encode` pattern but
/// (a) uses `resolve_struct_type_for_revert_arg` so struct-constructor calls
/// are covered, and (b) materialises the struct into a single temp local
/// before indexing so the constructor runs ONCE per arg (the Task #124
/// version re-lowers the expression per field — fine for bare Variable
/// references, wasteful for constructor calls that build the struct array
/// on every index).
fn lower_and_flatten_revert_arg(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> usize {
    if let Some(struct_ty) = resolve_struct_type_for_revert_arg(expr, ctx) {
        if let ValueType::Struct { fields, .. } = &struct_ty {
            if !fields.is_empty() && fields.iter().all(|f| is_static_abi_type_value(&f.ty)) {
                if !lower_expression(expr, ctx, instructions) {
                    return 0;
                }
                let tmp_id = ctx.next_label();
                let tmp_local = ctx
                    .allocate_local(format!("__revert_struct_{tmp_id}"), Some(struct_ty.clone()));
                instructions.push(Instruction::StoreLocal(tmp_local));
                for i in 0..fields.len() {
                    instructions.push(Instruction::LoadLocal(tmp_local));
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::from(i as u64),
                    )));
                    instructions.push(Instruction::ArrayGet);
                }
                return fields.len();
            }
        }
    }
    if !lower_expression(expr, ctx, instructions) {
        return 0;
    }
    1
}

/// Compute the 4-byte keccak selector for a custom error signature.
///
/// Signature form: `Name(t1,t2,...)` per Solidity's ABI spec.
fn revert_error_selector(name: &str, arg_types: &[String]) -> [u8; 4] {
    let signature = format!("{}({})", name, arg_types.join(","));
    let mut hasher = Keccak256::new();
    hasher.update(signature.as_bytes());
    let digest = hasher.finalize();
    [digest[0], digest[1], digest[2], digest[3]]
}

/// Build the canonical `Error(string)` revert envelope for a compile-time
/// string literal without routing through the on-chain `StdLib.abiEncode`
/// helper.
///
/// Layout:
/// - 4 bytes: selector = keccak256("Error(string)")[..4]
/// - 32 bytes: offset = 0x20
/// - 32 bytes: string length
/// - N bytes: UTF-8 string data
/// - padding: zero bytes to the next 32-byte boundary
fn error_string_literal_envelope(literal_bytes: &[u8]) -> Vec<u8> {
    let selector = revert_error_selector("Error", &["string".to_string()]);
    let padded_len = literal_bytes.len().div_ceil(32) * 32;
    let mut out = Vec::with_capacity(4 + 32 + 32 + padded_len);
    out.extend_from_slice(&selector);

    let mut offset_slot = [0u8; 32];
    offset_slot[31] = 0x20;
    out.extend_from_slice(&offset_slot);

    let mut len_slot = [0u8; 32];
    len_slot[24..].copy_from_slice(&(literal_bytes.len() as u64).to_be_bytes());
    out.extend_from_slice(&len_slot);

    out.extend_from_slice(literal_bytes);
    out.resize(4 + 32 + 32 + padded_len, 0);
    out
}

fn lower_revert_statement(
    ident: Option<&solang_parser::pt::IdentifierPath>,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    if let Some(path) = ident {
        let name = path
            .identifiers
            .last()
            .map(|id| id.name.clone())
            .unwrap_or_default();

        // Task #27 (compiler slice) — emit the EVM-canonical revert payload
        // `keccak256("Name(t1,...)")[0..4] || abi.encode(args)` instead of
        // dropping the args and pushing the bare error name. The runtime
        // `THROW` captures this byte string verbatim into
        // `ExecutionResult.return_data`, giving callers the same surface
        // Ethereum tooling expects for `revert Name(args...)`.
        let arg_types: Vec<String> = args
            .iter()
            .map(|arg| revert_arg_canonical_type(arg, ctx))
            .collect();
        let selector = revert_error_selector(&name, &arg_types);

        let direct_static_path = args.iter().all(|arg| is_direct_static_revert_arg(arg, ctx));
        if direct_static_path {
            let pre_len = instructions.len();
            instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                selector.to_vec(),
            )));
            let mut success = true;
            for arg in args {
                if !lower_direct_static_revert_arg_slot(arg, ctx, instructions) {
                    success = false;
                    break;
                }
            }
            if success {
                if !args.is_empty() {
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::BytesConcat,
                        arg_count: args.len() + 1,
                    });
                }
                instructions.push(Instruction::Throw);
                return true;
            }
            instructions.truncate(pre_len);
        }

        let pre_len = instructions.len();
        instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
            selector.to_vec(),
        )));
        if args.is_empty() {
            instructions.push(Instruction::Throw);
            return true;
        }
        if let Some(ok) = lower_abi_encode_args_direct_from_slice(args, ctx, instructions) {
            if ok {
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::BytesConcat,
                    arg_count: 2,
                });
                instructions.push(Instruction::Throw);
                return true;
            }
            instructions.truncate(pre_len);
            instructions.push(Instruction::PushLiteral(LiteralValue::String(
                name.into_bytes(),
            )));
            instructions.push(Instruction::Throw);
            return true;
        }
        instructions.truncate(pre_len);

        // Task #181 — struct args must be flattened into per-field stack
        // items (Task #124 pattern) so the AbiEncode builtin emits the
        // canonical 2-slot packed struct head (`addr || role`) instead of
        // the dynamic-array `offset || count || elements` shape the runtime
        // classifier produces for a `StackItem::Array`. Track `flat_count`
        // separately from `args.len()` so the `AbiEncode` call sees the
        // flattened slot count.
        let pre_len = instructions.len();
        let mut flat_count = 0usize;
        let mut success = true;
        for arg in args {
            let pushed = lower_and_flatten_revert_arg(arg, ctx, instructions);
            if pushed == 0 {
                success = false;
                break;
            }
            flat_count += pushed;
        }

        if !success {
            // Fall back to the legacy error-name string on lowering failure
            // so we still surface *something* on the error path.
            instructions.truncate(pre_len);
            instructions.push(Instruction::PushLiteral(LiteralValue::String(
                name.into_bytes(),
            )));
            instructions.push(Instruction::Throw);
            return true;
        }

        // Rewind the argument pushes so we can emit selector, then args, in
        // the correct order for AbiEncode + BytesConcat.
        let mut arg_instrs = instructions.split_off(pre_len);
        instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
            selector.to_vec(),
        )));
        instructions.append(&mut arg_instrs);
        if flat_count == 0 {
            // `revert ErrorName()` — payload is just the 4-byte selector.
        } else {
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::AbiEncode,
                arg_count: flat_count,
            });
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::BytesConcat,
                arg_count: 2,
            });
        }
        instructions.push(Instruction::Throw);
        return true;
    }

    // `revert("msg")` — EVM convention is to push
    // `Error(string)` selector (0x08c379a0) || abi.encode(msg). We follow
    // the same shape so tooling can decode the revert reason uniformly.
    if args.len() == 1 {
        if let Expression::StringLiteral(parts) = &args[0] {
            instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                error_string_literal_envelope(&string_literal_bytes(parts)),
            )));
            instructions.push(Instruction::Throw);
            return true;
        }
        let arg_is_string = matches!(
            infer_type_from_expression(&args[0], ctx),
            Some(ValueType::String)
        ) || matches!(&args[0], Expression::StringLiteral(_))
            || matches!(
                &args[0],
                Expression::FunctionCall(_, func, _)
                    if matches!(
                        func.as_ref(),
                        Expression::MemberAccess(_, inner, member)
                            if member.name == "concat"
                                && matches!(
                                    inner.as_ref(),
                                    Expression::Type(_, solang_parser::pt::Type::String)
                                )
                    )
            );
        if arg_is_string {
            let selector = revert_error_selector("Error", &["string".to_string()]);
            let pre_len = instructions.len();
            instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                selector.to_vec(),
            )));
            if let Some(ok) =
                lower_abi_encode_args_direct_from_slice(&[args[0].clone()], ctx, instructions)
            {
                if ok {
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::BytesConcat,
                        arg_count: 2,
                    });
                    instructions.push(Instruction::Throw);
                    return true;
                }
            }
            instructions.truncate(pre_len);

            if lower_expression(&args[0], ctx, instructions) {
                let mut arg_instrs = instructions.split_off(pre_len);
                instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                    selector.to_vec(),
                )));
                instructions.append(&mut arg_instrs);
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::AbiEncode,
                    arg_count: 1,
                });
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::BytesConcat,
                    arg_count: 2,
                });
                instructions.push(Instruction::Throw);
                return true;
            }
            instructions.truncate(pre_len);
        }
        if lower_expression(&args[0], ctx, instructions) {
            instructions.push(Instruction::Throw);
            return true;
        }
    } else {
        for arg in args {
            if lower_expression(arg, ctx, instructions) {
                instructions.push(Instruction::Drop(ValueType::Any));
            }
        }
    }

    instructions.push(Instruction::PushLiteral(LiteralValue::Null));
    instructions.push(Instruction::Throw);
    true
}

fn lower_revert_named_args(
    ident: Option<&solang_parser::pt::IdentifierPath>,
    args: &[solang_parser::pt::NamedArgument],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    if let Some(path) = ident {
        let name = path
            .identifiers
            .last()
            .map(|id| id.name.clone())
            .unwrap_or_default();

        // Task #27 (compiler slice) — named-args form mirrors the positional
        // custom-error path: `selector(4) || abi.encode(args)`. Named args
        // come in declaration order in the AST already, so we lower them
        // left-to-right and let AbiEncode pack them identically to the
        // positional form.
        let arg_types: Vec<String> = args
            .iter()
            .map(|arg| revert_arg_canonical_type(&arg.expr, ctx))
            .collect();
        let selector = revert_error_selector(&name, &arg_types);

        // Task #181 — named-args mirror of the positional custom-error
        // flatten. See `lower_revert_statement` for rationale.
        let direct_args: Vec<Expression> = args.iter().map(|arg| arg.expr.clone()).collect();
        let direct_pre_len = instructions.len();
        instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
            selector.to_vec(),
        )));
        if direct_args.is_empty() {
            instructions.push(Instruction::Throw);
            return true;
        }
        if let Some(ok) =
            lower_abi_encode_args_direct_from_slice(&direct_args, ctx, instructions)
        {
            if ok {
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::BytesConcat,
                    arg_count: 2,
                });
                instructions.push(Instruction::Throw);
                return true;
            }
            instructions.truncate(direct_pre_len);
            instructions.push(Instruction::PushLiteral(LiteralValue::String(
                name.into_bytes(),
            )));
            instructions.push(Instruction::Throw);
            return true;
        }
        instructions.truncate(direct_pre_len);

        let pre_len = instructions.len();
        let mut flat_count = 0usize;
        let mut success = true;
        for arg in args {
            let pushed = lower_and_flatten_revert_arg(&arg.expr, ctx, instructions);
            if pushed == 0 {
                success = false;
                break;
            }
            flat_count += pushed;
        }

        if !success {
            instructions.truncate(pre_len);
            instructions.push(Instruction::PushLiteral(LiteralValue::String(
                name.into_bytes(),
            )));
            instructions.push(Instruction::Throw);
            return true;
        }

        let mut arg_instrs = instructions.split_off(pre_len);
        instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
            selector.to_vec(),
        )));
        instructions.append(&mut arg_instrs);
        if flat_count > 0 {
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::AbiEncode,
                arg_count: flat_count,
            });
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::BytesConcat,
                arg_count: 2,
            });
        }
        instructions.push(Instruction::Throw);
        return true;
    }

    for arg in args {
        if lower_expression(&arg.expr, ctx, instructions) {
            instructions.push(Instruction::Drop(ValueType::Any));
        }
    }

    instructions.push(Instruction::PushLiteral(LiteralValue::Null));
    instructions.push(Instruction::Throw);
    true
}

/// Task #185 — parse a Solidity return type string like "uint[3][2]" into
/// its nested fixed-size dimensions in OUTER-FIRST order (matches Solidity
/// source convention: `uint[3][2]` is an array of 2 arrays of 3 uints, so
/// this returns `vec![2, 3]`).
///
/// Returns `None` when:
/// - the type isn't a nested fixed-size array (contains `[]` dynamic, or is
///   a scalar/struct),
/// - any dimension is non-numeric or zero,
/// - the leaf element type is not a static 32-byte ABI primitive (uintN/
///   intN/address/bool/bytesN); nested dynamics (string/bytes/T[]) or
///   structs would require recursive tail encoding and fall back to the
///   generic `AbiEncode(1)` path.
///
/// Mutates nothing. Whitespace-tolerant. Ignores trailing " memory" /
/// " calldata" / " storage" markers that `format!("{}", pt::Type)` may emit
/// on parameters.
fn parse_nested_fixed_array_shape(ty_str: &str) -> Option<Vec<usize>> {
    // Strip any storage-location suffix and surrounding whitespace.
    let cleaned: String = ty_str
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();
    let base = cleaned
        .strip_suffix("memory")
        .or_else(|| cleaned.strip_suffix("calldata"))
        .or_else(|| cleaned.strip_suffix("storage"))
        .unwrap_or(&cleaned);

    // Walk from the rightmost `[N]` inward, collecting dimensions. Solidity
    // source order `uint[3][2]` parses as `(uint[3])[2]`: the RIGHTMOST
    // bracket pair is the OUTERMOST dimension (here `[2]` → outer dim 2),
    // with each inner `[N]` one step closer to the leaf scalar type. Because
    // we walk right-to-left (outer first) and push in that order, the final
    // vector is ALREADY in outer-first order — no reversal needed.
    let mut remaining = base;
    let mut dims_outer_first: Vec<usize> = Vec::new();
    while let Some(close_idx) = remaining.rfind(']') {
        if close_idx + 1 != remaining.len() {
            // Trailing characters after the last `]` would mean something
            // like `uint[3][2]foo` — not a parseable array type.
            return None;
        }
        let open_idx = remaining.rfind('[')?;
        if open_idx >= close_idx {
            return None;
        }
        let dim_text = &remaining[open_idx + 1..close_idx];
        if dim_text.is_empty() {
            // Dynamic array `T[]` — cannot flat-encode at compile time.
            return None;
        }
        let dim: usize = dim_text.parse().ok()?;
        if dim == 0 {
            return None;
        }
        dims_outer_first.push(dim);
        remaining = &remaining[..open_idx];
    }

    if dims_outer_first.is_empty() {
        return None;
    }

    // Leaf type must be a static 32-byte ABI primitive. Anything else
    // (string / bytes / T[] / structs / mappings) needs dynamic-tail
    // encoding and falls outside this flat path.
    if !is_static_32_byte_leaf_type(remaining) {
        return None;
    }

    Some(dims_outer_first)
}

fn array_leaf_static_value_type(value_type: &ValueType) -> Option<ValueType> {
    let mut current = value_type;
    while let ValueType::Array(inner) = current {
        current = inner.as_ref();
    }
    if is_static_abi_type_value(current) {
        Some(current.clone())
    } else {
        None
    }
}

/// Task #185 — predicate for scalar Solidity types whose EVM canonical
/// encoding is a single 32-byte BE-padded slot. Matches the signature used
/// by `abi_is_dynamic` / `abi_pad32_be` in the runtime's StdLib handler so
/// the flat-array return path only fires for cases where the runtime's
/// static fast-path produces correct output.
fn is_static_32_byte_leaf_type(ty: &str) -> bool {
    let compact = ty.trim();
    match compact {
        "uint" | "int" | "address" | "bool" => true,
        _ => {
            if let Some(width) = compact
                .strip_prefix("uint")
                .or_else(|| compact.strip_prefix("int"))
            {
                if let Ok(bits) = width.parse::<u32>() {
                    return bits > 0 && bits <= 256 && bits.is_multiple_of(8);
                }
            }
            if let Some(width) = compact.strip_prefix("bytes") {
                if let Ok(len) = width.parse::<u32>() {
                    return (1..=32).contains(&len);
                }
            }
            false
        }
    }
}
