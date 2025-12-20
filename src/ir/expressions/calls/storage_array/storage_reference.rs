fn try_lower_storage_reference_array_helpers(
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

    let reference = resolve_storage_reference(inner, ctx)?;

    // We support pushing/popping on:
    // - state arrays (`arr.push(x)`)
    // - storage references to arrays in mappings (`m[key].arr.push(x)`)
    // - storage references to array-typed struct fields (`m[key].struct.arr.push(x)`)
    // - nested struct paths (`m[key].outer.inner.arr.push(x)`)

    let ValueType::Array(element_type) = reference.value_type.clone() else {
        return None;
    };

    match member.name.as_str() {
        "push" => Some(lower_storage_reference_push(
            &reference,
            args,
            ctx,
            instructions,
        )),
        "pop" => Some(lower_storage_reference_pop(
            &reference,
            element_type.as_ref(),
            args,
            ctx,
            instructions,
        )),
        _ => None,
    }
}

fn lower_storage_reference_push(
    reference: &StorageReference,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
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
    if !emit_storage_load(reference, ctx, instructions) {
        return false;
    }
    instructions.push(Instruction::StoreLocal(len_local));

    // Store element at index `len`.
    instructions.push(Instruction::LoadLocal(value_local));
    instructions.push(Instruction::LoadLocal(len_local));
    for expr in reference.key_expressions.iter().rev() {
        load_expression(expr, ctx, instructions);
    }

    if !reference.field_path.is_empty() {
        let field_keys: Vec<[u8; 32]> = reference.field_path.iter().map(|field| field.key).collect();
        let ValueType::Array(element_type) = &reference.value_type else {
            ctx.record_error("array push target is not an array");
            return false;
        };
        instructions.push(Instruction::StoreStructArrayElement {
            state_index: reference.state_index,
            key_types: reference.key_types.clone(),
            field_keys,
            element_type: (**element_type).clone(),
        });
    } else {
        let mut key_types = reference.key_types.clone();
        key_types.push(ValueType::Integer {
            signed: false,
            bits: 256,
        });

        instructions.push(Instruction::StoreMappingElement {
            state_index: reference.state_index,
            key_types,
        });
    }

    // Increment length.
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));

    if !emit_storage_store(reference, ctx, instructions) {
        return false;
    }

    instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
    true
}

fn lower_storage_reference_pop(
    reference: &StorageReference,
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
    if !emit_storage_load(reference, ctx, instructions) {
        return false;
    }
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
    if !emit_storage_store(reference, ctx, instructions) {
        return false;
    }

    // Load element at new_len.
    instructions.push(Instruction::LoadLocal(new_len_local));
    for expr in reference.key_expressions.iter().rev() {
        load_expression(expr, ctx, instructions);
    }

    let mut key_types = reference.key_types.clone();
    key_types.push(ValueType::Integer {
        signed: false,
        bits: 256,
    });

    if !reference.field_path.is_empty() {
        let field_keys: Vec<[u8; 32]> = reference.field_path.iter().map(|field| field.key).collect();
        instructions.push(Instruction::LoadStructArrayElement {
            state_index: reference.state_index,
            key_types: reference.key_types.clone(),
            field_keys,
            element_type: element_type.clone(),
        });
    } else {
        instructions.push(Instruction::LoadMappingElement {
            state_index: reference.state_index,
            key_types: key_types.clone(),
        });
    }

    let popped_local = ctx.allocate_local("__array_popped".to_string(), Some(element_type.clone()));
    instructions.push(Instruction::StoreLocal(popped_local));

    // Overwrite removed slot with default value.
    push_default_for_value_type(element_type, ctx, instructions);
    instructions.push(Instruction::LoadLocal(new_len_local));
    for expr in reference.key_expressions.iter().rev() {
        load_expression(expr, ctx, instructions);
    }
    if !reference.field_path.is_empty() {
        let field_keys: Vec<[u8; 32]> = reference.field_path.iter().map(|field| field.key).collect();
        instructions.push(Instruction::StoreStructArrayElement {
            state_index: reference.state_index,
            key_types: reference.key_types.clone(),
            field_keys,
            element_type: element_type.clone(),
        });
    } else {
        instructions.push(Instruction::StoreMappingElement {
            state_index: reference.state_index,
            key_types,
        });
    }

    instructions.push(Instruction::LoadLocal(popped_local));
    instructions.push(Instruction::Jump { target: end_label });

    instructions.push(Instruction::Label(empty_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Null));
    instructions.push(Instruction::Throw);

    instructions.push(Instruction::Label(end_label));
    true
}
