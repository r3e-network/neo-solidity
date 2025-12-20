fn lower_post_inc_dec(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    increment: bool,
) -> bool {
    // Post-increment/decrement semantics:
    // 1. Load the original value (this will be the result)
    // 2. Perform the increment/decrement and store back
    // 3. The original value remains on the stack as the expression result

    // Step 1: Load original value onto stack (this is the return value)
    if !lower_expression(expr, ctx, instructions) {
        return false;
    }

    // Step 2: Perform compound assignment (x = x + 1 or x = x - 1)
    // This modifies the variable but we don't need its result on stack
    let one = Expression::NumberLiteral(Default::default(), "1".to_string(), "".to_string(), None);
    let op = if increment {
        BinaryOperator::Add
    } else {
        BinaryOperator::Sub
    };

    if !lower_compound_assignment(expr, &one, ctx, instructions, op) {
        return false;
    }

    // lower_compound_assignment leaves the updated value on the stack as the expression result.
    // For post-inc/dec we must discard it and keep the original value from Step 1.
    instructions.push(Instruction::Drop(ValueType::Any));

    // The original value loaded in Step 1 is already on the stack as the result
    // Do NOT load the value again - that was the bug!
    true
}

fn lower_pre_inc_dec(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    increment: bool,
) -> bool {
    let one = Expression::NumberLiteral(Default::default(), "1".to_string(), "".to_string(), None);
    let op = if increment {
        BinaryOperator::Add
    } else {
        BinaryOperator::Sub
    };

    if !lower_compound_assignment(expr, &one, ctx, instructions, op) {
        return false;
    }

    // lower_compound_assignment returns the updated value as the expression result.
    true
}
