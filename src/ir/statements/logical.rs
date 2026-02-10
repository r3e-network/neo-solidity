fn lower_require(
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    if args.is_empty() {
        ctx.record_error_with_suggestion(
            "require() expects at least one argument",
            "usage: require(condition) or require(condition, \"error message\")",
        );
        return;
    }

    let fail_label = ctx.next_label();
    let ok_label = ctx.next_label();

    // IR JumpIf branches when the condition is false.
    if lower_expression(&args[0], ctx, instructions) {
        instructions.push(Instruction::JumpIf { target: fail_label });
        instructions.push(Instruction::Jump { target: ok_label });
    }

    instructions.push(Instruction::Label(fail_label));
    if args.len() > 1 {
        // Solidity 0.8.x supports three forms:
        //   require(cond)              -> THROW null
        //   require(cond, "message")   -> THROW "message"
        //   require(cond, CustomError(args)) -> THROW "CustomError"
        //
        // For the custom error form, the second argument is a FunctionCall whose callee is
        // a Variable naming the error type. We extract the name and serialize the error
        // arguments into the thrown string for better diagnostics. NeoVM has no ABI-encoded
        // error data, so we format as "ErrorName(arg_count)" to preserve maximum context.
        if let Expression::FunctionCall(_, callee, error_args) = &args[1] {
            if let Expression::Variable(error_ident) = callee.as_ref() {
                // Build a diagnostic string: "ErrorName(N args)" where N is the arg count.
                // We still evaluate args for side effects, then drop them.
                for error_arg in error_args {
                    if lower_expression(error_arg, ctx, instructions) {
                        instructions.push(Instruction::Drop(ValueType::Any));
                    }
                }
                let msg = if error_args.is_empty() {
                    error_ident.name.clone()
                } else {
                    format!("{}({} args)", error_ident.name, error_args.len())
                };
                instructions.push(Instruction::PushLiteral(LiteralValue::String(
                    msg.as_bytes().to_vec(),
                )));
                instructions.push(Instruction::Throw);
                instructions.push(Instruction::Label(ok_label));
                return;
            }
        }

        // Preserve diagnostics/type checking for the revert message expression and surface it
        // in the VM fault state when possible (NeoVM THROW).
        if lower_expression(&args[1], ctx, instructions) {
            instructions.push(Instruction::Throw);
            instructions.push(Instruction::Label(ok_label));
            return;
        }
    }

    // NeoVM THROW requires an exception value on the stack. `null` yields an empty message.
    instructions.push(Instruction::PushLiteral(LiteralValue::Null));
    instructions.push(Instruction::Throw);
    instructions.push(Instruction::Label(ok_label));
}

fn lower_assert(args: &[Expression], ctx: &mut LoweringContext, instructions: &mut Vec<Instruction>) {
    if args.len() != 1 {
        ctx.record_error_with_suggestion(
            "assert() expects exactly one argument",
            "usage: assert(condition)",
        );
        return;
    }

    let fail_label = ctx.next_label();
    let ok_label = ctx.next_label();

    // IR JumpIf branches when the condition is false.
    if lower_expression(&args[0], ctx, instructions) {
        instructions.push(Instruction::JumpIf { target: fail_label });
        instructions.push(Instruction::Jump { target: ok_label });
    }

    instructions.push(Instruction::Label(fail_label));
    // Solidity assert() panics. NeoVM THROW is catchable, so we use it with a panic-like marker.
    instructions.push(Instruction::PushLiteral(LiteralValue::String(
        b"Panic: 0x01".to_vec(),
    )));
    instructions.push(Instruction::Throw);

    instructions.push(Instruction::Label(ok_label));
}

fn lower_logical_or(
    left: &Expression,
    right: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let false_label = ctx.next_label();
    let end_label = ctx.next_label();

    if !lower_expression(left, ctx, instructions) {
        return false;
    }

    instructions.push(Instruction::JumpIf {
        target: false_label,
    });
    instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
    instructions.push(Instruction::Jump { target: end_label });
    instructions.push(Instruction::Label(false_label));

    if !lower_expression(right, ctx, instructions) {
        return false;
    }

    instructions.push(Instruction::Label(end_label));
    true
}

fn lower_logical_and(
    left: &Expression,
    right: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let false_label = ctx.next_label();
    let end_label = ctx.next_label();

    if !lower_expression(left, ctx, instructions) {
        return false;
    }

    instructions.push(Instruction::JumpIf {
        target: false_label,
    });

    if !lower_expression(right, ctx, instructions) {
        return false;
    }

    instructions.push(Instruction::Jump { target: end_label });
    instructions.push(Instruction::Label(false_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(false)));
    instructions.push(Instruction::Label(end_label));
    true
}
