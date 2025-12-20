fn lower_compound_assignment(
    lhs: &Expression,
    rhs: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    op: BinaryOperator,
) -> bool {
    // Solidity compound assignment semantics:
    // - Evaluate the LHS location once (including any array/mapping index expressions).
    // - Compute `lhs_value op rhs_value`.
    // - Store the result back to the same LHS.
    // - The expression result is the stored value (so statement lowering can safely DROP it).

    if let Some(mapping) = resolve_mapping_access(lhs, ctx) {
        // Evaluate keys left-to-right (Solidity semantics) and store them so we can reuse them
        // for both the load and the store without re-evaluating side effects.
        let tmp_id = ctx.next_label();
        let mut key_locals: Vec<usize> = Vec::new();
        for (index, key_expr) in mapping.key_expressions.iter().enumerate() {
            let local = ctx.allocate_local(
                format!("__compound_key_{tmp_id}_{index}"),
                mapping.key_types.get(index).cloned(),
            );
            if !lower_expression(key_expr, ctx, instructions) {
                ctx.record_error("failed to lower mapping key in compound assignment");
                return false;
            }
            instructions.push(Instruction::StoreLocal(local));
            key_locals.push(local);
        }

        // Load current value (keys must be pushed in reverse order for slot hashing).
        for local in key_locals.iter().rev() {
            instructions.push(Instruction::LoadLocal(*local));
        }
        instructions.push(Instruction::LoadMappingElement {
            state_index: mapping.state_index,
            key_types: mapping.key_types.clone(),
        });

        // Evaluate RHS after LHS value (left-to-right semantics for `lhs op rhs`).
        if !lower_expression(rhs, ctx, instructions) {
            return false;
        }

        instructions.push(Instruction::BinaryOp(op));

        // Store result while preserving it as the expression value.
        let result_local = ctx.allocate_local(format!("__compound_value_{tmp_id}"), None);
        instructions.push(Instruction::StoreLocal(result_local));

        instructions.push(Instruction::LoadLocal(result_local));
        for local in key_locals.iter().rev() {
            instructions.push(Instruction::LoadLocal(*local));
        }
        instructions.push(Instruction::StoreMappingElement {
            state_index: mapping.state_index,
            key_types: mapping.key_types.clone(),
        });

        instructions.push(Instruction::LoadLocal(result_local));
        return true;
    }

    if let Some(reference) = resolve_storage_reference(lhs, ctx) {
        if let Some(field) = reference.field_path.last() {
            let field_keys: Vec<[u8; 32]> = reference.field_path.iter().map(|field| field.key).collect();
            let tmp_id = ctx.next_label();
            let mut key_locals: Vec<usize> = Vec::new();

            for (index, key_expr) in reference.key_expressions.iter().enumerate() {
                let local = ctx.allocate_local(
                    format!("__compound_key_{tmp_id}_{index}"),
                    reference.key_types.get(index).cloned(),
                );
                if !lower_expression(key_expr, ctx, instructions) {
                    ctx.record_error("failed to lower storage key in compound assignment");
                    return false;
                }
                instructions.push(Instruction::StoreLocal(local));
                key_locals.push(local);
            }

            let push_keys_for_slot = |instructions: &mut Vec<Instruction>| {
                for local in key_locals.iter().rev() {
                    instructions.push(Instruction::LoadLocal(*local));
                }
            };

            // Load current field value.
            push_keys_for_slot(instructions);
            instructions.push(Instruction::LoadStructField {
                state_index: reference.state_index,
                key_types: reference.key_types.clone(),
                field_keys: field_keys.clone(),
                field_type: field.ty.clone(),
            });

            // Evaluate RHS after LHS value (left-to-right semantics for `lhs op rhs`).
            if !lower_expression(rhs, ctx, instructions) {
                return false;
            }

            instructions.push(Instruction::BinaryOp(op));

            // Store result while preserving it as the expression value.
            let result_local = ctx.allocate_local(format!("__compound_value_{tmp_id}"), None);
            instructions.push(Instruction::StoreLocal(result_local));

            instructions.push(Instruction::LoadLocal(result_local));
            push_keys_for_slot(instructions);
            instructions.push(Instruction::StoreStructField {
                state_index: reference.state_index,
                key_types: reference.key_types.clone(),
                field_keys,
                field_type: field.ty.clone(),
            });

            instructions.push(Instruction::LoadLocal(result_local));
            return true;
        }
    }

    if let Expression::ArraySubscript(_, array, Some(index)) = lhs {
        // Memory array element compound assignment: `arr[i] op= rhs`.
        // Preserve evaluation order and avoid evaluating `array` / `index` twice.
        let tmp_id = ctx.next_label();
        let array_local = ctx.allocate_local(format!("__compound_arr_{tmp_id}"), None);
        let index_local = ctx.allocate_local(format!("__compound_idx_{tmp_id}"), None);

        if !lower_expression(array, ctx, instructions) {
            return false;
        }
        instructions.push(Instruction::StoreLocal(array_local));

        if !lower_expression(index, ctx, instructions) {
            return false;
        }
        instructions.push(Instruction::StoreLocal(index_local));

        // Load current element.
        instructions.push(Instruction::LoadLocal(array_local));
        instructions.push(Instruction::LoadLocal(index_local));
        instructions.push(Instruction::ArrayGet);

        if !lower_expression(rhs, ctx, instructions) {
            return false;
        }

        instructions.push(Instruction::BinaryOp(op));

        // Store and return the updated value.
        let result_local = ctx.allocate_local(format!("__compound_value_{tmp_id}"), None);
        instructions.push(Instruction::StoreLocal(result_local));

        instructions.push(Instruction::LoadLocal(array_local));
        instructions.push(Instruction::LoadLocal(index_local));
        instructions.push(Instruction::LoadLocal(result_local));
        instructions.push(Instruction::ArraySet);

        instructions.push(Instruction::LoadLocal(result_local));
        return true;
    }

    if let Expression::Variable(identifier) = lhs {
        let store_instr = if let Some(local) = ctx.resolve_local(&identifier.name) {
            Instruction::StoreLocal(local)
        } else if let Some(state) = ctx.state_index_map.get(&identifier.name) {
            Instruction::StoreState(*state)
        } else {
            let local = ctx.ensure_local(&identifier.name);
            Instruction::StoreLocal(local)
        };

        let tmp_id = ctx.next_label();
        let result_local = ctx.allocate_local(format!("__compound_value_{tmp_id}"), None);

        // Evaluate lhs then rhs (left-to-right) so BinaryOp sees [lhs, rhs].
        if !lower_expression(lhs, ctx, instructions) {
            return false;
        }
        if !lower_expression(rhs, ctx, instructions) {
            return false;
        }

        instructions.push(Instruction::BinaryOp(op));
        instructions.push(Instruction::StoreLocal(result_local));

        instructions.push(Instruction::LoadLocal(result_local));
        instructions.push(store_instr);

        instructions.push(Instruction::LoadLocal(result_local));
        return true;
    }

    ctx.record_error("unsupported compound assignment target");
    false
}
