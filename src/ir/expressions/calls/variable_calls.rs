fn try_lower_variable_call(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if let Expression::Variable(identifier) = func {
        if identifier.name == "require" || identifier.name == "assert" {
            ctx.record_error(format!("{}() cannot be used as an expression", identifier.name));
            return Some(false);
        }

        // Treat `ContractType(addressExpr)` as a no-op cast for known contract/interface
        // types when the argument is already address-like (including 20-byte hex literals).
        if args.len() == 1 && ctx.is_contract_type_name(&identifier.name) {
            if matches!(
                infer_type_from_expression(&args[0], ctx),
                Some(ValueType::Address)
            ) {
                return Some(lower_expression(&args[0], ctx, instructions));
            }

            if let Some(bytes) = address_bytes_le_from_expression(&args[0]) {
                instructions.push(Instruction::PushLiteral(LiteralValue::Address(bytes)));
                return Some(true);
            }
        }

        if ctx.function_names.contains(&identifier.name) {
            let mut success = true;
            for arg in args {
                if !lower_expression(arg, ctx, instructions) {
                    success = false;
                }
            }

            if success {
                if let Some(neo_name) = ctx.neo_function_name(&identifier.name, args.len()) {
                    instructions.push(Instruction::CallFunction {
                        name: neo_name,
                        arg_count: args.len(),
                    });
                } else {
                    ctx.record_error(format!(
                        "no overload of '{}' with {} argument(s)",
                        identifier.name,
                        args.len()
                    ));
                    success = false;
                }
            }

            return Some(success);
        }

        ctx.record_error(format!("unsupported function call '{}'", identifier.name));
        return Some(false);
    }

    None
}
