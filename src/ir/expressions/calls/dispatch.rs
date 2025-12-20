fn lower_function_call_expression(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let mut func = func;

    fn is_compile_time_zero(expr: &Expression) -> bool {
        match expr {
            Expression::Parenthesis(_, inner) => is_compile_time_zero(inner),
            Expression::FunctionCall(_, func, args) if args.len() == 1 => {
                if matches!(
                    func.as_ref(),
                    Expression::Type(_, PtType::Uint(_) | PtType::Int(_))
                ) {
                    return is_compile_time_zero(&args[0]);
                }
                false
            }
            _ => matches!(
                literal_from_expression(expr),
                Some(LiteralValue::Integer(value)) if value.is_zero()
            ),
        }
    }

    // Solidity call options `foo{gas: ..., value: ...}()` are represented by solang-parser as a
    // `FunctionCall` where the callee expression is a `FunctionCallBlock`. Neo N3 does not
    // support EVM-style attached value or gas limits, but we can safely ignore `{gas: ...}` to
    // improve compatibility with Solidity sources.
    if let Expression::FunctionCallBlock(_, inner_call, block) = func {
        if let Statement::Args(_, named_args) = block.as_ref() {
            let mut unsupported = Vec::new();
            for arg in named_args {
                match arg.name.name.as_str() {
                    // Neo has no per-invocation gas limit; evaluate for side effects then ignore.
                    "gas" => {
                        if lower_expression(&arg.expr, ctx, instructions) {
                            instructions.push(Instruction::Drop(ValueType::Any));
                        }
                    }
                    // Neo has no attached value; require explicit NEP-17 transfers instead.
                    "value" => {
                        if is_compile_time_zero(&arg.expr) {
                            if lower_expression(&arg.expr, ctx, instructions) {
                                instructions.push(Instruction::Drop(ValueType::Any));
                            }
                        } else {
                            unsupported.push("value");
                        }
                    }
                    "salt" => unsupported.push("salt"),
                    other => unsupported.push(other),
                }
            }

            if !unsupported.is_empty() {
                ctx.record_error(format!(
                    "function call options (`{{...}}`) are not supported ({}); Neo N3 requires explicit NEP-17 transfers (`NativeCalls.gasTransfer` / `NativeCalls.neoTransfer`) + `onNEP17Payment`",
                    unsupported.join(", ")
                ));
                return false;
            }

            func = inner_call.as_ref();
        } else {
            ctx.record_error(
                "function call blocks are not supported; only call options (`{...}`) are recognized",
            );
            return false;
        }
    }

    if let Some(result) = try_lower_storage_array_helpers(func, args, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_value_transfer_helpers(func, args, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_low_level_address_call(func, args, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_type_constructor_call(func, args, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_struct_constructor_call(func, args, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_builtin_call(func, args, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_member_call(func, args, ctx, instructions) {
        return result;
    }

    if let Some(result) = try_lower_variable_call(func, args, ctx, instructions) {
        return result;
    }

    for arg in args {
        load_expression(arg, ctx, instructions);
        instructions.push(Instruction::Drop(ValueType::Any));
    }
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    true
}
