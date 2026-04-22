fn try_lower_state_array_helpers(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    let Expression::MemberAccess(_, inner, member) = func else {
        return None;
    };

    if !matches!(member.name.as_str(), "push" | "pop") {
        return None;
    }

    let Expression::Variable(base) = inner.as_ref() else {
        return None;
    };

    let state_index = ctx.state_index_map.get(&base.name).copied()?;

    let Some(ValueType::Array(element_type)) = ctx.state_type(state_index).cloned() else {
        return None;
    };

    match member.name.as_str() {
        "push" => Some(lower_state_array_push(
            state_index,
            element_type.as_ref(),
            args,
            ctx,
            instructions,
        )),
        "pop" => Some(lower_state_array_pop(
            state_index,
            element_type.as_ref(),
            args,
            ctx,
            instructions,
        )),
        _ => None,
    }
}

fn lower_state_array_push(
    state_index: usize,
    element_type: &ValueType,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    // Task #203 — Per Solidity spec, zero-arg `.push()` on an array-of-
    // mappings (e.g. `mapping(uint => uint)[] grids; grids.push();`) is
    // the ONLY valid push shape because mappings cannot be passed by
    // value. Mapping elements are pure storage-slot-derivation — there
    // is no on-chain materialised "empty mapping" value — so the push
    // lowering just needs to increment the length slot. The subsequent
    // `grids[g][k] = v` / `grids[g][k]` double-indexed accesses derive
    // their keccak slots from the (g, k) pair, so no element-write is
    // required.
    if args.is_empty() && matches!(element_type, ValueType::Mapping { .. }) {
        let len_local = ctx.allocate_local("__array_len".to_string(), None);
        instructions.push(Instruction::LoadState(state_index));
        instructions.push(Instruction::StoreLocal(len_local));

        // Increment length.
        instructions.push(Instruction::LoadLocal(len_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
        instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
        instructions.push(Instruction::StoreState(state_index));

        instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
        return true;
    }

    if args.len() != 1 {
        ctx.record_error("array push expects exactly one argument");
        return false;
    }

    if !lower_expression(&args[0], ctx, instructions) {
        return false;
    }
    let value_local = ctx.allocate_local("__array_push_value".to_string(), None);
    instructions.push(Instruction::StoreLocal(value_local));

    let len_local = ctx.allocate_local("__array_len".to_string(), None);
    instructions.push(Instruction::LoadState(state_index));
    instructions.push(Instruction::StoreLocal(len_local));

    // Store element at index `len`.
    instructions.push(Instruction::LoadLocal(value_local));
    instructions.push(Instruction::LoadLocal(len_local));
    // Task #104 — For struct-element arrays (`P[] ps`), `ps.push(P(a,b))`
    // must decompose the struct into per-field storage slots so that
    // subsequent `ps[i].a` / `ps[i].b` reads (which lower to
    // `LoadStructField` with `field_keys = [field_key]` via
    // `resolve_storage_reference` + `.a` walk) see the same slot the
    // push wrote. `StoreMappingElement` writes the whole struct-array
    // blob at `keccak256(serialize(i) || base_slot)`, while the read
    // path derives `keccak256(a_key || keccak256(serialize(i) || base_slot))`.
    // Route struct elements through `StoreStructArrayElement`, which
    // already implements the per-field layout expected by the read
    // side (see src/cli/bytecode/bytecode_helpers/storage/structs/array_elements.rs).
    if matches!(element_type, ValueType::Struct { .. }) {
        instructions.push(Instruction::StoreStructArrayElement {
            state_index,
            key_types: Vec::new(),
            field_keys: Vec::new(),
            element_type: element_type.clone(),
        });
    } else if matches!(element_type, ValueType::Array(_)) {
        instructions.push(Instruction::StoreArrayDeepCopy {
            state_index,
            key_types: vec![ValueType::Integer {
                signed: false,
                bits: 256,
            }],
        });
    } else {
        instructions.push(Instruction::StoreMappingElement {
            state_index,
            key_types: vec![ValueType::Integer {
                signed: false,
                bits: 256,
            }],
        });
    }

    // Increment length.
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreState(state_index));

    instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
    true
}

fn lower_state_array_pop(
    state_index: usize,
    element_type: &ValueType,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    if !args.is_empty() {
        ctx.record_error("array pop expects no arguments");
        return false;
    }

    let len_local = ctx.allocate_local("__array_len".to_string(), None);
    instructions.push(Instruction::LoadState(state_index));
    instructions.push(Instruction::StoreLocal(len_local));

    let empty_label = ctx.next_label();
    let end_label = ctx.next_label();

    // Abort on empty array.
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Ne));
    instructions.push(Instruction::JumpIf { target: empty_label });

    let new_len_local = ctx.allocate_local("__array_new_len".to_string(), None);
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Sub));
    instructions.push(Instruction::StoreLocal(new_len_local));

    // Update length before returning element.
    instructions.push(Instruction::LoadLocal(new_len_local));
    instructions.push(Instruction::StoreState(state_index));

    // Load element at new_len.
    instructions.push(Instruction::LoadLocal(new_len_local));
    // Task #104 — Route struct-element arrays through
    // `LoadStructArrayElement` so the pop read uses the same per-field
    // slot layout the struct-array push wrote (see `lower_state_array_push`).
    if matches!(element_type, ValueType::Struct { .. }) {
        instructions.push(Instruction::LoadStructArrayElement {
            state_index,
            key_types: Vec::new(),
            field_keys: Vec::new(),
            element_type: element_type.clone(),
        });
    } else {
        instructions.push(Instruction::LoadMappingElement {
            state_index,
            key_types: vec![ValueType::Integer {
                signed: false,
                bits: 256,
            }],
        });
    }

    let popped_local = ctx.allocate_local("__array_popped".to_string(), Some(element_type.clone()));
    instructions.push(Instruction::StoreLocal(popped_local));

    // Overwrite removed slot with default value.
    push_default_for_value_type(element_type, ctx, instructions);
    instructions.push(Instruction::LoadLocal(new_len_local));
    if matches!(element_type, ValueType::Struct { .. }) {
        instructions.push(Instruction::StoreStructArrayElement {
            state_index,
            key_types: Vec::new(),
            field_keys: Vec::new(),
            element_type: element_type.clone(),
        });
    } else if matches!(element_type, ValueType::Array(_)) {
        instructions.push(Instruction::StoreArrayDeepCopy {
            state_index,
            key_types: vec![ValueType::Integer {
                signed: false,
                bits: 256,
            }],
        });
    } else {
        instructions.push(Instruction::StoreMappingElement {
            state_index,
            key_types: vec![ValueType::Integer {
                signed: false,
                bits: 256,
            }],
        });
    }

    instructions.push(Instruction::LoadLocal(popped_local));
    instructions.push(Instruction::Jump { target: end_label });

    // Task #98 / Task #107 — Solidity 0.8.x specifies that `.pop()` on an
    // empty array reverts with Panic(0x31) (array-pop-underflow). Route
    // through the shared `emit_panic` helper which emits the canonical
    //   keccak256("Panic(uint256)")[0..4] || abi.encode(0x31)
    // payload so `ExecutionResult.return_data` begins with the EVM-canonical
    // Panic(uint256) envelope — matches the shape already used by
    // `assert(false)` (Panic 0x01) in `src/ir/statements/logical.rs` and
    // enum-cast range guard (Panic 0x21) in
    // `src/ir/expressions/calls/variable_calls.rs`.
    instructions.push(Instruction::Label(empty_label));
    emit_panic(0x31, instructions);

    instructions.push(Instruction::Label(end_label));
    true
}
