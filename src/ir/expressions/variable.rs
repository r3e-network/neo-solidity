fn lower_variable_expression(
    identifier: &Identifier,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    if identifier.name == "this" {
        instructions.push(Instruction::CallBuiltin {
            builtin: BuiltinCall::Syscall("System.Runtime.GetExecutingScriptHash".to_string()),
            arg_count: 0,
        });
        return true;
    }
    if identifier.name == "block" || identifier.name == "msg" {
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
        return true;
    }
    if let Some(alias) = ctx.storage_alias(&identifier.name).cloned() {
        return emit_storage_load(&alias, ctx, instructions);
    }
    if let Some(index) = ctx.param_index_map.get(&identifier.name) {
        instructions.push(Instruction::LoadParameter(*index));
        true
    } else if let Some(index) = ctx.resolve_local(&identifier.name) {
        instructions.push(Instruction::LoadLocal(index));
        true
    } else if let Some(index) = ctx.state_index_map.get(&identifier.name) {
        let const_initializer = ctx.state_metadata(*index).and_then(|meta| {
            if meta.is_constant {
                meta.initializer.clone()
            } else {
                None
            }
        });
        if let Some(initializer) = const_initializer {
            return lower_expression(&initializer, ctx, instructions);
        }

        // Struct-typed state variables are stored field-by-field in derived slots. Loading the
        // base slot does not materialize the struct value, so build it by reading each field.
        if matches!(ctx.state_type(*index), Some(ValueType::Struct { .. })) {
            let reference = StorageReference {
                state_index: *index,
                key_expressions: Vec::new(),
                key_types: Vec::new(),
                value_type: ctx.state_type(*index).unwrap().clone(),
                field_path: Vec::new(),
            };
            return emit_storage_load(&reference, ctx, instructions);
        }

        instructions.push(Instruction::LoadState(*index));
        true
    } else {
        ctx.record_error(format!("unknown identifier '{}'", identifier.name));
        false
    }
}
