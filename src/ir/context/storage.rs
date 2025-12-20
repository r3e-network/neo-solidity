fn resolve_storage_reference(
    expression: &Expression,
    ctx: &LoweringContext,
) -> Option<StorageReference> {
    if let Some(mapping) = resolve_mapping_access(expression, ctx) {
        return Some(mapping.to_storage_reference());
    }

    match expression {
        Expression::Variable(identifier) => {
            if let Some(alias) = ctx.storage_alias(&identifier.name).cloned() {
                return Some(alias);
            }

            // Treat struct-typed state variables as storage references so that:
            // - field accesses (`stateStruct.field`) load from storage slots; and
            // - whole-struct assignments (`stateStruct = StructName({...})`) can be lowered
            //   into per-field stores.
            let state_index = *ctx.state_index_map.get(&identifier.name)?;
            let state_type = ctx.state_type(state_index)?;
            if matches!(state_type, ValueType::Struct { .. }) {
                Some(StorageReference {
                    state_index,
                    key_expressions: Vec::new(),
                    key_types: Vec::new(),
                    value_type: state_type.clone(),
                    field_path: Vec::new(),
                })
            } else {
                None
            }
        }
        Expression::MemberAccess(_, inner, member) => {
            let mut base = resolve_storage_reference(inner, ctx)?;
            let field = find_struct_field(&base.value_type, &member.name)?;
            base.field_path.push(StorageReferenceField {
                key: field.key,
                ty: field.ty.clone(),
            });
            base.value_type = field.ty.clone();
            Some(base)
        }
        _ => None,
    }
}

fn emit_storage_load(
    reference: &StorageReference,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let tmp_id = ctx.next_label();

    // Evaluate mapping/array keys left-to-right (Solidity semantics) and store them so we can
    // push them in the stack order required by the storage slot hashing routine.
    let mut key_locals: Vec<usize> = Vec::new();
    for (index, expr) in reference.key_expressions.iter().enumerate() {
        let local = ctx.allocate_local(
            format!("__storage_key_{tmp_id}_{index}"),
            reference.key_types.get(index).cloned(),
        );
        if !lower_expression(expr, ctx, instructions) {
            return false;
        }
        instructions.push(Instruction::StoreLocal(local));
        key_locals.push(local);
    }

    let push_keys_for_slot = |instructions: &mut Vec<Instruction>| {
        // The bytecode emission expects keys in reverse order (deepest key first), with the
        // outer-most key closest to the top of the stack when the base slot is pushed.
        for local in key_locals.iter().rev() {
            instructions.push(Instruction::LoadLocal(*local));
        }
    };

    if let Some(field) = reference.field_path.last() {
        let field_keys: Vec<[u8; 32]> = reference.field_path.iter().map(|field| field.key).collect();
        push_keys_for_slot(instructions);
        instructions.push(Instruction::LoadStructField {
            state_index: reference.state_index,
            key_types: reference.key_types.clone(),
            field_keys,
            field_type: field.ty.clone(),
        });
        return true;
    }

    // Loading a struct value from storage requires fetching each field individually.
    if let ValueType::Struct { fields, .. } = &reference.value_type {
        let out_local = ctx.allocate_local(
            format!("__storage_struct_{tmp_id}"),
            Some(reference.value_type.clone()),
        );

        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::from(fields.len() as u64),
        )));
        instructions.push(Instruction::NewArray {
            element_type: ValueType::Any,
        });
        instructions.push(Instruction::StoreLocal(out_local));

        for (index, field) in fields.iter().enumerate() {
            instructions.push(Instruction::LoadLocal(out_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
                index as u64,
            ))));
            push_keys_for_slot(instructions);
            instructions.push(Instruction::LoadStructField {
                state_index: reference.state_index,
                key_types: reference.key_types.clone(),
                field_keys: vec![field.key],
                field_type: field.ty.clone(),
            });
            instructions.push(Instruction::ArraySet);
        }

        instructions.push(Instruction::LoadLocal(out_local));
        return true;
    }

    push_keys_for_slot(instructions);
    instructions.push(Instruction::LoadMappingElement {
        state_index: reference.state_index,
        key_types: reference.key_types.clone(),
    });
    true
}

fn emit_storage_store(
    reference: &StorageReference,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let tmp_id = ctx.next_label();

    // Evaluate mapping/array keys left-to-right and store them in locals so we can re-push them
    // in the correct order for slot hashing.
    let mut key_locals: Vec<usize> = Vec::new();
    for (index, expr) in reference.key_expressions.iter().enumerate() {
        let local = ctx.allocate_local(
            format!("__storage_key_{tmp_id}_{index}"),
            reference.key_types.get(index).cloned(),
        );
        if !lower_expression(expr, ctx, instructions) {
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

    if let Some(field) = reference.field_path.last() {
        let field_keys: Vec<[u8; 32]> = reference.field_path.iter().map(|field| field.key).collect();
        push_keys_for_slot(instructions);
        instructions.push(Instruction::StoreStructField {
            state_index: reference.state_index,
            key_types: reference.key_types.clone(),
            field_keys,
            field_type: field.ty.clone(),
        });
        return true;
    }

    // Storing an entire struct value writes each field into its own derived slot.
    if let ValueType::Struct { fields, .. } = &reference.value_type {
        let value_local = ctx.allocate_local(
            format!("__storage_struct_value_{tmp_id}"),
            Some(reference.value_type.clone()),
        );
        instructions.push(Instruction::StoreLocal(value_local));

        for (index, field) in fields.iter().enumerate() {
            instructions.push(Instruction::LoadLocal(value_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
                index as u64,
            ))));
            instructions.push(Instruction::ArrayGet);
            push_keys_for_slot(instructions);
            instructions.push(Instruction::StoreStructField {
                state_index: reference.state_index,
                key_types: reference.key_types.clone(),
                field_keys: vec![field.key],
                field_type: field.ty.clone(),
            });
        }

        return true;
    }

    push_keys_for_slot(instructions);
    instructions.push(Instruction::StoreMappingElement {
        state_index: reference.state_index,
        key_types: reference.key_types.clone(),
    });
    true
}

fn resolve_mapping_access<'a>(
    expression: &'a Expression,
    ctx: &LoweringContext,
) -> Option<MappingAccess<'a>> {
    let mut keys: Vec<&'a Expression> = Vec::new();
    let mut current = expression;

    loop {
        match current {
            Expression::ArraySubscript(_, inner, maybe_index) => {
                let index_expr = maybe_index.as_ref()?.as_ref();
                keys.insert(0, index_expr);
                current = inner;
            }
            Expression::Variable(identifier) => {
                let state_index = *ctx.state_index_map.get(&identifier.name)?;
                let mut current_type = ctx.state_type(state_index)?.clone();
                let mut key_types = Vec::with_capacity(keys.len());

                for _key_expr in &keys {
                    match current_type {
                        ValueType::Mapping { ref key, ref value } => {
                            key_types.push((**key).clone());
                            current_type = (**value).clone();
                        }
                        ValueType::Array(ref element) => {
                            // Storage arrays are lowered like mappings keyed by uint256 index.
                            key_types.push(ValueType::Integer {
                                signed: false,
                                bits: 256,
                            });
                            current_type = (**element).clone();
                        }
                        _ => return None,
                    }
                }

                return Some(MappingAccess {
                    state_index,
                    key_expressions: keys,
                    key_types,
                    value_type: current_type,
                });
            }
            _ => return None,
        }
    }
}
