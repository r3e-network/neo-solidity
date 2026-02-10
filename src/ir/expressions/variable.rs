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
    if identifier.name == "super" {
        // `super` is only meaningful as `super.method()`. When used as a bare
        // identifier (e.g., assigned to a variable), emit a targeted diagnostic
        // instead of the generic "unknown identifier" error.
        ctx.record_error_with_suggestion(
            "the 'super' keyword can only be used in member-call expressions (super.method())",
            "use super.methodName() to call a parent contract's function, or inline the parent logic directly",
        );
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
        return false;
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
        let state_index = *index;
        let state_type = ctx.state_type(state_index).cloned();
        let const_initializer = ctx.state_metadata(*index).and_then(|meta| {
            if meta.is_constant {
                meta.initializer.clone()
            } else {
                None
            }
        });
        if let Some(initializer) = const_initializer {
            // Preserve hash160 constants as address literals so downstream manifest
            // permission inference can recover exact contract hashes for
            // `Syscalls.contractCall(CONSTANT_HASH, ...)` patterns.
            if matches!(state_type.as_ref(), Some(ValueType::Address)) {
                if let Some(bytes) = address_bytes_le_from_expression(&initializer) {
                    instructions.push(Instruction::PushLiteral(LiteralValue::Address(bytes)));
                    return true;
                }
            }

            return lower_expression(&initializer, ctx, instructions);
        }

        // Struct-typed state variables are stored field-by-field in derived slots. Loading the
        // base slot does not materialize the struct value, so build it by reading each field.
        if let Some(vt @ ValueType::Struct { .. }) = state_type {
            let reference = StorageReference {
                state_index,
                key_expressions: Vec::new(),
                key_types: Vec::new(),
                value_type: vt,
                field_path: Vec::new(),
            };
            return emit_storage_load(&reference, ctx, instructions);
        }

        instructions.push(Instruction::LoadState(state_index));
        true
    } else {
        ctx.record_error_with_suggestion(
            format!("unknown identifier '{}'", identifier.name),
            "check spelling or ensure the variable is declared in the same contract or an imported library",
        );
        false
    }
}
