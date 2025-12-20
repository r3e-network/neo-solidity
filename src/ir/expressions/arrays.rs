fn lower_array_subscript_expression(
    expr: &Expression,
    array: &Expression,
    index: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    if let Some(mapping) = resolve_mapping_access(expr, ctx) {
        let reference = mapping.to_storage_reference();
        emit_storage_load(&reference, ctx, instructions)
    } else if lower_expression(array, ctx, instructions) && lower_expression(index, ctx, instructions) {
        instructions.push(Instruction::ArrayGet);
        true
    } else {
        false
    }
}

fn lower_array_slice_expression(
    array: &Expression,
    start: Option<&Expression>,
    end: Option<&Expression>,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let array_local = ctx.allocate_local("__slice_array".to_string(), None);
    if !lower_expression(array, ctx, instructions) {
        return false;
    }
    instructions.push(Instruction::StoreLocal(array_local));

    let start_local = ctx.allocate_local("__slice_start".to_string(), None);
    if let Some(start_expr) = start {
        if !lower_expression(start_expr, ctx, instructions) {
            return false;
        }
    } else {
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    }
    instructions.push(Instruction::StoreLocal(start_local));

    let end_local = ctx.allocate_local("__slice_end".to_string(), None);
    if let Some(end_expr) = end {
        if !lower_expression(end_expr, ctx, instructions) {
            return false;
        }
    } else {
        instructions.push(Instruction::LoadLocal(array_local));
        instructions.push(Instruction::GetSize);
    }
    instructions.push(Instruction::StoreLocal(end_local));

    // Clamp start to >= 0
    let clamp_start_label = ctx.next_label();
    let clamp_start_done = ctx.next_label();
    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Ge));
    instructions.push(Instruction::JumpIf {
        target: clamp_start_label,
    });
    instructions.push(Instruction::Jump {
        target: clamp_start_done,
    });
    instructions.push(Instruction::Label(clamp_start_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::StoreLocal(start_local));
    instructions.push(Instruction::Label(clamp_start_done));

    // Clamp end to array length
    let size_local = ctx.allocate_local("__slice_size".to_string(), None);
    instructions.push(Instruction::LoadLocal(array_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(size_local));

    let clamp_end_label = ctx.next_label();
    let clamp_end_done = ctx.next_label();
    instructions.push(Instruction::LoadLocal(end_local));
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Le));
    instructions.push(Instruction::JumpIf {
        target: clamp_end_label,
    });
    instructions.push(Instruction::Jump {
        target: clamp_end_done,
    });
    instructions.push(Instruction::Label(clamp_end_label));
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::StoreLocal(end_local));
    instructions.push(Instruction::Label(clamp_end_done));

    let len_local = ctx.allocate_local("__slice_len".to_string(), None);
    instructions.push(Instruction::LoadLocal(end_local));
    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Sub));
    instructions.push(Instruction::StoreLocal(len_local));

    let clamp_label = ctx.next_label();
    let clamp_done = ctx.next_label();
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Ge));
    instructions.push(Instruction::JumpIf { target: clamp_label });
    instructions.push(Instruction::Jump { target: clamp_done });
    instructions.push(Instruction::Label(clamp_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::StoreLocal(len_local));
    instructions.push(Instruction::Label(clamp_done));

    let element_type = infer_array_element_type(array, ctx).unwrap_or(ValueType::Any);
    let slice_array_type = ValueType::Array(Box::new(element_type.clone()));
    let out_local = ctx.allocate_local("__slice_out".to_string(), Some(slice_array_type));
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::NewArray { element_type });
    instructions.push(Instruction::StoreLocal(out_local));

    let idx_local = ctx.allocate_local("__slice_index".to_string(), None);
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::StoreLocal(idx_local));

    let loop_label = ctx.next_label();
    let end_label = ctx.next_label();

    instructions.push(Instruction::Label(loop_label));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: end_label });

    instructions.push(Instruction::LoadLocal(out_local));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::LoadLocal(array_local));
    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::ArrayGet);
    instructions.push(Instruction::ArraySet);

    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(1u8))));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(idx_local));

    instructions.push(Instruction::Jump { target: loop_label });
    instructions.push(Instruction::Label(end_label));
    instructions.push(Instruction::LoadLocal(out_local));
    true
}

fn lower_array_literal_expression(
    elements: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let element_type = infer_literal_array_element_type(elements);
    let array_local = ctx.allocate_local(
        "__array_literal".to_string(),
        Some(ValueType::Array(Box::new(element_type.clone()))),
    );
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        elements.len(),
    ))));
    instructions.push(Instruction::NewArray { element_type });
    instructions.push(Instruction::StoreLocal(array_local));

    for (index, element) in elements.iter().enumerate() {
        instructions.push(Instruction::LoadLocal(array_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
            index as u64,
        ))));
        if !lower_expression(element, ctx, instructions) {
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
        }
        instructions.push(Instruction::ArraySet);
    }

    instructions.push(Instruction::LoadLocal(array_local));
    true
}
