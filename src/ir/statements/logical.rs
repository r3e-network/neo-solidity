fn lower_require(
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    if args.is_empty() {
        ctx.record_error("require() expects at least one argument");
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
        ctx.record_error("assert() expects exactly one argument");
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
