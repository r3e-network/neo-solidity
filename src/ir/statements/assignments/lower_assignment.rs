fn lower_assignment(
    lhs: &Expression,
    rhs: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    if let Expression::Variable(identifier) = lhs {
        if ctx.storage_alias(&identifier.name).is_some() {
            if let Some(source_reference) = resolve_storage_reference(rhs, ctx) {
                ctx.set_storage_alias(identifier.name.clone(), source_reference);
                return;
            }
        }
    }

    if let Some(reference) = resolve_storage_reference(lhs, ctx) {
        if let ValueType::Struct { name, fields } = &reference.value_type {
            // Support struct construction on the RHS for storage assignments, both
            // named and positional: `S({a:1,b:2})` or `S(1,2)`.
            let mut ctor_args_by_name: Option<&[solang_parser::pt::NamedArgument]> = None;
            let mut ctor_args_positional: Option<&[Expression]> = None;
            let mut ctor_name_matches = false;

            match rhs {
                Expression::NamedFunctionCall(_, func, args) => {
                    if let Expression::Variable(identifier) = func.as_ref() {
                        ctor_name_matches = identifier.name.eq_ignore_ascii_case(name);
                        if ctor_name_matches {
                            ctor_args_by_name = Some(args.as_slice());
                        }
                    }
                }
                Expression::FunctionCall(_, func, args) => {
                    if let Expression::Variable(identifier) = func.as_ref() {
                        ctor_name_matches = identifier.name.eq_ignore_ascii_case(name);
                        if ctor_name_matches {
                            ctor_args_positional = Some(args.as_slice());
                        }
                    }
                }
                _ => {}
            }

            if ctor_name_matches {
                for (index, field) in fields.iter().enumerate() {
                    let mut field_reference = reference.clone();
                    field_reference.field_path.push(StorageReferenceField {
                        key: field.key,
                        ty: field.ty.clone(),
                    });
                    field_reference.value_type = field.ty.clone();

                    let success = if let Some(named_args) = ctor_args_by_name {
                        if let Some(arg) = named_args.iter().find(|arg| arg.name.name == field.name)
                        {
                            lower_expression(&arg.expr, ctx, instructions)
                        } else {
                            push_default_for_storage_value_type(&field.ty, ctx, instructions)
                        }
                    } else if let Some(pos_args) = ctor_args_positional {
                        if let Some(arg) = pos_args.get(index) {
                            lower_expression(arg, ctx, instructions)
                        } else {
                            push_default_for_storage_value_type(&field.ty, ctx, instructions)
                        }
                    } else {
                        push_default_for_storage_value_type(&field.ty, ctx, instructions)
                    };

                    if success && !emit_storage_store(&field_reference, ctx, instructions) {
                        instructions.push(Instruction::Drop(ValueType::Any));
                    }
                }

                return;
            }
        }

        let success = lower_expression(rhs, ctx, instructions);
        if success {
            if !emit_storage_store(&reference, ctx, instructions) {
                instructions.push(Instruction::Drop(ValueType::Any));
            }
        } else {
            instructions.push(Instruction::Drop(ValueType::Any));
        }
        return;
    }

    if let Expression::List(_, params) = lhs {
        // Tuple destructuring assignment: `(a, b) = expr;`
        // We represent tuples as arrays and assign by index.

        #[derive(Clone)]
        enum TupleTarget {
            Ignore,
            DeclaredLocal {
                local_index: usize,
                inferred_type: Option<ValueType>,
            },
            ExistingLocal(usize),
            ExistingState(usize),
            Storage(StorageReference),
            Invalid,
        }

        let mut targets: Vec<TupleTarget> = Vec::with_capacity(params.len());

        // First pass: allocate locals for declarations and resolve assignment targets.
        for (_, param) in params.iter() {
            let Some(parameter) = param else {
                targets.push(TupleTarget::Ignore);
                continue;
            };

            if let Some(name) = parameter.name.as_ref() {
                // Declaration: `(bool ok, uint v) = ...`
                if ctx.is_local_in_current_scope(&name.name) {
                    ctx.record_error(format!("local variable '{}' redeclared", name.name));
                }

                let inferred_type = infer_type_from_expression(&parameter.ty, ctx);
                let local_index = ctx.allocate_local(name.name.clone(), inferred_type.clone());
                targets.push(TupleTarget::DeclaredLocal {
                    local_index,
                    inferred_type,
                });
                continue;
            }

            // Assignment to an existing lvalue: `(ok, v) = ...`
            if let Some(reference) = resolve_storage_reference(&parameter.ty, ctx) {
                targets.push(TupleTarget::Storage(reference));
                continue;
            }

            if let Expression::Variable(identifier) = &parameter.ty {
                if let Some(local_index) = ctx.resolve_local(&identifier.name) {
                    targets.push(TupleTarget::ExistingLocal(local_index));
                } else if let Some(state_index) =
                    ctx.state_index_map.get(&identifier.name).copied()
                {
                    targets.push(TupleTarget::ExistingState(state_index));
                } else {
                    ctx.record_error(format!(
                        "unknown identifier '{}' in tuple assignment",
                        identifier.name
                    ));
                    targets.push(TupleTarget::Invalid);
                }
                continue;
            }

            ctx.record_error("unsupported tuple assignment target");
            targets.push(TupleTarget::Invalid);
        }

        // Lower RHS into a temporary buffer so failures don't leave partial stack state.
        let mut rhs_instrs = Vec::new();
        if !lower_expression(rhs, ctx, &mut rhs_instrs) {
            // Ensure declared locals exist with default values to avoid cascading errors.
            for target in targets {
                if let TupleTarget::DeclaredLocal {
                    local_index,
                    inferred_type,
                } = target
                {
                    if let Some(ty) = inferred_type.as_ref() {
                        push_default_for_value_type(ty, ctx, instructions);
                    } else {
                        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                            BigInt::zero(),
                        )));
                    }
                    instructions.push(Instruction::StoreLocal(local_index));
                }
            }
            return;
        }
        instructions.append(&mut rhs_instrs);

        let tuple_local = ctx.allocate_local("__tuple_assign".to_string(), None);
        instructions.push(Instruction::StoreLocal(tuple_local));

        for (index, target) in targets.iter().enumerate() {
            if matches!(target, TupleTarget::Ignore) {
                continue;
            }

            instructions.push(Instruction::LoadLocal(tuple_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(index as u64),
            )));
            instructions.push(Instruction::ArrayGet);

            match target {
                TupleTarget::Ignore => unreachable!(),
                TupleTarget::DeclaredLocal { local_index, .. }
                | TupleTarget::ExistingLocal(local_index) => {
                    ctx.clear_call_data_local(*local_index);
                    instructions.push(Instruction::StoreLocal(*local_index));
                }
                TupleTarget::ExistingState(state_index) => {
                    instructions.push(Instruction::StoreState(*state_index));
                }
                TupleTarget::Storage(reference) => {
                    if !emit_storage_store(reference, ctx, instructions) {
                        instructions.push(Instruction::Drop(ValueType::Any));
                    }
                }
                TupleTarget::Invalid => {
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
            }
        }

        return;
    }

    if matches!(lhs, Expression::ArraySubscript(_, _, Some(_))) {
        lower_array_store(lhs, rhs, ctx, instructions);
        return;
    }

    if let Expression::Variable(identifier) = lhs {
        if let Some(index) = ctx.resolve_local(&identifier.name) {
            match parse_low_level_call_data(rhs, ctx) {
                Ok(Some((method_name, encode_args))) => {
                    let mut lowered = true;
                    for arg in encode_args {
                        if !lower_expression(arg, ctx, instructions) {
                            lowered = false;
                        }
                    }

                    if lowered {
                        instructions.push(Instruction::CallBuiltin {
                            builtin: BuiltinCall::AbiEncode,
                            arg_count: encode_args.len(),
                        });
                        instructions.push(Instruction::StoreLocal(index));
                        ctx.set_call_data_local(index, method_name);
                    }
                }
                Ok(None) => {
                    if lower_expression(rhs, ctx, instructions) {
                        instructions.push(Instruction::StoreLocal(index));
                        ctx.clear_call_data_local(index);
                    }
                }
                Err(message) => {
                    ctx.record_error(message);
                    ctx.clear_call_data_local(index);
                }
            }
            return;
        }
        if let Some(index) = ctx.state_index_map.get(&identifier.name) {
            if lower_expression(rhs, ctx, instructions) {
                instructions.push(Instruction::StoreState(*index));
            }
            return;
        }

        let index = ctx.ensure_local(&identifier.name);
        match parse_low_level_call_data(rhs, ctx) {
            Ok(Some((method_name, encode_args))) => {
                let mut lowered = true;
                for arg in encode_args {
                    if !lower_expression(arg, ctx, instructions) {
                        lowered = false;
                    }
                }

                if lowered {
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::AbiEncode,
                        arg_count: encode_args.len(),
                    });
                    instructions.push(Instruction::StoreLocal(index));
                    ctx.set_call_data_local(index, method_name);
                }
            }
            Ok(None) => {
                if lower_expression(rhs, ctx, instructions) {
                    instructions.push(Instruction::StoreLocal(index));
                    ctx.clear_call_data_local(index);
                }
            }
            Err(message) => {
                ctx.record_error(message);
                ctx.clear_call_data_local(index);
            }
        }
        return;
    }

    // Fallback: evaluate RHS (if possible) and drop to allow compilation to continue.
    if lower_expression(rhs, ctx, instructions) {
        instructions.push(Instruction::Drop(ValueType::Any));
    }
}
