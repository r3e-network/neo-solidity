fn lower_return_statement(
    expr: Option<&Expression>,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    if let Some(expression) = expr {
        if lower_expression(expression, ctx, instructions) {
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
            ctx.record_error("return without value requires named return variables for this function");
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

        let tmp_id = ctx.next_label();
        let tuple_local = ctx.allocate_local(
            format!("__return_tuple_{tmp_id}"),
            Some(ValueType::Array(Box::new(ValueType::Any))),
        );

        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
            return_types.len() as u64,
        ))));
        instructions.push(Instruction::NewArray {
            element_type: ValueType::Any,
        });
        instructions.push(Instruction::StoreLocal(tuple_local));

        for (index, (slot, value_type)) in return_slots
            .iter()
            .zip(return_types.iter())
            .enumerate()
        {
            instructions.push(Instruction::LoadLocal(tuple_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
                index as u64,
            ))));

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

fn lower_revert_statement(
    ident: Option<&solang_parser::pt::IdentifierPath>,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    if let Some(path) = ident {
        for arg in args {
            if lower_expression(arg, ctx, instructions) {
                instructions.push(Instruction::Drop(ValueType::Any));
            }
        }

        let name = path
            .identifiers
            .last()
            .map(|id| id.name.clone())
            .unwrap_or_default();
        instructions.push(Instruction::PushLiteral(LiteralValue::String(
            name.into_bytes(),
        )));
        instructions.push(Instruction::Throw);
        return true;
    }

    if args.len() == 1 {
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
    for arg in args {
        if lower_expression(&arg.expr, ctx, instructions) {
            instructions.push(Instruction::Drop(ValueType::Any));
        }
    }

    if let Some(path) = ident {
        let name = path
            .identifiers
            .last()
            .map(|id| id.name.clone())
            .unwrap_or_default();
        instructions.push(Instruction::PushLiteral(LiteralValue::String(
            name.into_bytes(),
        )));
        instructions.push(Instruction::Throw);
        return true;
    }

    instructions.push(Instruction::PushLiteral(LiteralValue::Null));
    instructions.push(Instruction::Throw);
    true
}
