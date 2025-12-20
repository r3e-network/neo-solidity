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
        "push" => Some(lower_state_array_push(state_index, args, ctx, instructions)),
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
    instructions.push(Instruction::LoadState(state_index));
    instructions.push(Instruction::StoreLocal(len_local));

    // Store element at index `len`.
    instructions.push(Instruction::LoadLocal(value_local));
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::StoreMappingElement {
        state_index,
        key_types: vec![ValueType::Integer {
            signed: false,
            bits: 256,
        }],
    });

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
    instructions.push(Instruction::LoadMappingElement {
        state_index,
        key_types: vec![ValueType::Integer {
            signed: false,
            bits: 256,
        }],
    });

    let popped_local = ctx.allocate_local("__array_popped".to_string(), Some(element_type.clone()));
    instructions.push(Instruction::StoreLocal(popped_local));

    // Overwrite removed slot with default value.
    push_default_for_value_type(element_type, ctx, instructions);
    instructions.push(Instruction::LoadLocal(new_len_local));
    instructions.push(Instruction::StoreMappingElement {
        state_index,
        key_types: vec![ValueType::Integer {
            signed: false,
            bits: 256,
        }],
    });

    instructions.push(Instruction::LoadLocal(popped_local));
    instructions.push(Instruction::Jump { target: end_label });

    instructions.push(Instruction::Label(empty_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Null));
    instructions.push(Instruction::Throw);

    instructions.push(Instruction::Label(end_label));
    true
}
