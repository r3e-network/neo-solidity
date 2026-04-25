fn lower_array_store(
    target: &Expression,
    rhs: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    if let Expression::ArraySubscript(_, array, Some(index)) = target {
        let checkpoint = instructions.len();
        if lower_expression(array, ctx, instructions)
            && lower_expression(index, ctx, instructions)
            && lower_expression(rhs, ctx, instructions)
        {
            // No Swap; Drop needed: real NeoVM MEMCPY pushes nothing.
            instructions.push(Instruction::ArraySet);
            return;
        }
        instructions.truncate(checkpoint);
    }

    load_expression(rhs, ctx, instructions);
    instructions.push(Instruction::Drop(ValueType::Any));
}

