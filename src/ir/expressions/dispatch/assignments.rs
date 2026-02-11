fn try_lower_expression_assignments(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    match expr {
        Expression::AssignAdd(_, lhs, rhs) => Some(lower_compound_assignment(
            lhs,
            rhs,
            ctx,
            instructions,
            BinaryOperator::Add,
        )),
        Expression::AssignSubtract(_, lhs, rhs) => Some(lower_compound_assignment(
            lhs,
            rhs,
            ctx,
            instructions,
            BinaryOperator::Sub,
        )),
        Expression::AssignShiftLeft(_, lhs, rhs) => Some(lower_compound_assignment(
            lhs,
            rhs,
            ctx,
            instructions,
            BinaryOperator::Shl,
        )),
        Expression::AssignShiftRight(_, lhs, rhs) => Some(lower_compound_assignment(
            lhs,
            rhs,
            ctx,
            instructions,
            BinaryOperator::Shr,
        )),
        Expression::AssignAnd(_, lhs, rhs) => Some(lower_compound_assignment(
            lhs,
            rhs,
            ctx,
            instructions,
            BinaryOperator::BitAnd,
        )),
        Expression::AssignOr(_, lhs, rhs) => Some(lower_compound_assignment(
            lhs,
            rhs,
            ctx,
            instructions,
            BinaryOperator::BitOr,
        )),
        Expression::AssignXor(_, lhs, rhs) => Some(lower_compound_assignment(
            lhs,
            rhs,
            ctx,
            instructions,
            BinaryOperator::BitXor,
        )),
        Expression::AssignMultiply(_, lhs, rhs) => Some(lower_compound_assignment(
            lhs,
            rhs,
            ctx,
            instructions,
            BinaryOperator::Mul,
        )),
        Expression::AssignDivide(_, lhs, rhs) => Some(lower_compound_assignment(
            lhs,
            rhs,
            ctx,
            instructions,
            BinaryOperator::Div,
        )),
        Expression::AssignModulo(_, lhs, rhs) => Some(lower_compound_assignment(
            lhs,
            rhs,
            ctx,
            instructions,
            BinaryOperator::Mod,
        )),
        Expression::Assign(_, lhs, rhs) => {
            lower_assignment(lhs, rhs, ctx, instructions);
            Some(true)
        }
        Expression::PostIncrement(_, inner) => Some(lower_post_inc_dec(inner, ctx, instructions, true)),
        Expression::PostDecrement(_, inner) => {
            Some(lower_post_inc_dec(inner, ctx, instructions, false))
        }
        Expression::PreIncrement(_, inner) => Some(lower_pre_inc_dec(inner, ctx, instructions, true)),
        Expression::PreDecrement(_, inner) => Some(lower_pre_inc_dec(inner, ctx, instructions, false)),
        Expression::Delete(_, target) => {
            Some(lower_delete(target, ctx, instructions))
        }
        _ => None,
    }
}

fn lower_delete(
    target: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    // Storage delete: reset to the storage-default value.
    if let Some(reference) = resolve_storage_reference(target, ctx) {
        if !ctx.ensure_state_writable(reference.state_index) {
            return false;
        }

        // Solidity allows `delete` on mappings, but it cannot clear all keys; treat as a no-op.
        if matches!(reference.value_type, ValueType::Mapping { .. }) {
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            return true;
        }

        push_default_for_storage_value_type(&reference.value_type, ctx, instructions);
        if !emit_storage_store(&reference, ctx, instructions) {
            instructions.push(Instruction::Drop(ValueType::Any));
        }

        // `delete` is a statement-only expression in Solidity; return a placeholder value so
        // expression statements can safely DROP it without underflowing.
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::zero(),
        )));
        return true;
    }

    // Local/state variable delete.
    if let Expression::Variable(identifier) = target {
        if let Some(local_index) = ctx.resolve_local(&identifier.name) {
            if let Some(value_type) = ctx.local_type(local_index).cloned() {
                push_default_for_value_type(&value_type, ctx, instructions);
            } else {
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::zero(),
                )));
            }
            ctx.clear_call_data_local(local_index);
            instructions.push(Instruction::StoreLocal(local_index));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            return true;
        }

        if let Some(state_index) = ctx.state_index_map.get(&identifier.name).copied() {
            if !ctx.ensure_state_writable(state_index) {
                return false;
            }

            if let Some(state_type) = ctx.state_type(state_index).cloned() {
                if matches!(state_type, ValueType::Mapping { .. }) {
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                    return true;
                }

                if matches!(state_type, ValueType::Struct { .. }) {
                    let reference = StorageReference {
                        state_index,
                        key_expressions: Vec::new(),
                        key_types: Vec::new(),
                        value_type: state_type.clone(),
                        field_path: Vec::new(),
                    };

                    push_default_for_storage_value_type(&reference.value_type, ctx, instructions);
                    if !emit_storage_store(&reference, ctx, instructions) {
                        instructions.push(Instruction::Drop(ValueType::Any));
                    }
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                    return true;
                }

                push_default_for_storage_value_type(&state_type, ctx, instructions);
                instructions.push(Instruction::StoreState(state_index));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::zero(),
                )));
                return true;
            }
        }
    }

    // Memory array element delete: `delete arr[i]`.
    if let Expression::ArraySubscript(_, array, Some(index)) = target {
        if let Some(ValueType::Array(element_type)) = infer_type_from_expression(array, ctx) {
            let tmp_id = ctx.next_label();
            let array_local = ctx.allocate_local(format!("__delete_arr_{tmp_id}"), None);
            let index_local = ctx.allocate_local(format!("__delete_idx_{tmp_id}"), None);

            if !lower_expression(array, ctx, instructions) {
                return false;
            }
            instructions.push(Instruction::StoreLocal(array_local));

            if !lower_expression(index, ctx, instructions) {
                return false;
            }
            instructions.push(Instruction::StoreLocal(index_local));

            instructions.push(Instruction::LoadLocal(array_local));
            instructions.push(Instruction::LoadLocal(index_local));
            push_default_for_value_type(element_type.as_ref(), ctx, instructions);
            instructions.push(Instruction::ArraySet);
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            return true;
        }
    }

    // Memory struct field delete: `delete tmp.field`.
    if let Expression::MemberAccess(_, inner, member) = target {
        if let Expression::Variable(base) = inner.as_ref() {
            if let Some(local_index) = ctx.resolve_local(&base.name) {
                if let Some(ValueType::Struct { fields, .. }) = infer_type_from_expression(inner, ctx)
                {
                    if let Some((field_index, field)) = fields
                        .iter()
                        .enumerate()
                        .find(|(_, field)| field.name == member.name)
                    {
                        instructions.push(Instruction::LoadLocal(local_index));
                        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                            BigInt::from(field_index as u64),
                        )));
                        push_default_for_value_type(&field.ty, ctx, instructions);
                        instructions.push(Instruction::ArraySet);
                        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                            BigInt::zero(),
                        )));
                        return true;
                    }
                }
            }
        }
    }

    ctx.record_error_with_suggestion(
        "unsupported delete target",
        "delete is supported for state variables, mapping entries, and local variables",
    );
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    true
}
