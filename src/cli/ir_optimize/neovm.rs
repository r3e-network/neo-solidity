/// NeoVM-specific peephole optimization: removes redundant stack operations
fn neovm_peephole_optimize(block: &mut ir::BasicBlock) {
    let mut optimized = Vec::with_capacity(block.instructions.len());
    let mut i = 0;

    while i < block.instructions.len() {
        // Pattern: PUSH x followed by DROP → remove both
        if i + 1 < block.instructions.len() {
            if matches!(&block.instructions[i], ir::Instruction::PushLiteral(_))
                && matches!(&block.instructions[i + 1], ir::Instruction::Drop(_))
            {
                i += 2;
                continue;
            }

            // Pattern: LoadLocal x followed by DROP → remove both
            if matches!(&block.instructions[i], ir::Instruction::LoadLocal(_))
                && matches!(&block.instructions[i + 1], ir::Instruction::Drop(_))
            {
                i += 2;
                continue;
            }
        }

        // Note: StoreLocal + LoadLocal(same) pattern is handled at codegen level
        // by using NeoVM's STLOC/LDLOC which can be fused by the NeoVM optimizer

        optimized.push(block.instructions[i].clone());
        i += 1;
    }

    block.instructions = optimized;
}

/// NeoVM-specific: simplify identity operations (x + 0, x * 1, x & MAX, etc.)
fn neovm_simplify_identity_ops(block: &mut ir::BasicBlock) {
    use num_bigint::BigInt;
    use num_traits::Zero;

    let mut optimized = Vec::with_capacity(block.instructions.len());
    let mut i = 0;

    while i < block.instructions.len() {
        // Pattern: PUSH 0, BinaryOp::Add → remove (x + 0 = x)
        if i + 1 < block.instructions.len() {
            if let ir::Instruction::PushLiteral(ir::LiteralValue::Integer(val)) =
                &block.instructions[i]
            {
                if val.is_zero() {
                    if let ir::Instruction::BinaryOp(ir::BinaryOperator::Add) =
                        &block.instructions[i + 1]
                    {
                        // Skip both instructions, identity operation
                        i += 2;
                        continue;
                    }
                    if let ir::Instruction::BinaryOp(ir::BinaryOperator::Sub) =
                        &block.instructions[i + 1]
                    {
                        // x - 0 = x, skip both
                        i += 2;
                        continue;
                    }
                    if let ir::Instruction::BinaryOp(ir::BinaryOperator::BitOr) =
                        &block.instructions[i + 1]
                    {
                        // x | 0 = x, skip both
                        i += 2;
                        continue;
                    }
                    if let ir::Instruction::BinaryOp(ir::BinaryOperator::BitXor) =
                        &block.instructions[i + 1]
                    {
                        // x ^ 0 = x, skip both
                        i += 2;
                        continue;
                    }
                }
                // Pattern: PUSH 1, MUL → identity (x * 1 = x)
                if *val == BigInt::from(1) {
                    if let ir::Instruction::BinaryOp(ir::BinaryOperator::Mul) =
                        &block.instructions[i + 1]
                    {
                        i += 2;
                        continue;
                    }
                    if let ir::Instruction::BinaryOp(ir::BinaryOperator::Div) =
                        &block.instructions[i + 1]
                    {
                        // x / 1 = x
                        i += 2;
                        continue;
                    }
                }
            }
        }

        optimized.push(block.instructions[i].clone());
        i += 1;
    }

    block.instructions = optimized;
}

/// NeoVM-specific: optimize boolean patterns
fn neovm_bool_optimize(block: &mut ir::BasicBlock) {
    let mut optimized = Vec::with_capacity(block.instructions.len());
    let mut i = 0;

    while i < block.instructions.len() {
        // Pattern: PUSH true, EQ → identity for booleans (x == true = x)
        if i + 1 < block.instructions.len() {
            if let ir::Instruction::PushLiteral(ir::LiteralValue::Boolean(true)) =
                &block.instructions[i]
            {
                if let ir::Instruction::BinaryOp(ir::BinaryOperator::Eq) =
                    &block.instructions[i + 1]
                {
                    // x == true → x (skip both)
                    i += 2;
                    continue;
                }
            }

            // Pattern: PUSH false, NE → identity for booleans (x != false = x)
            if let ir::Instruction::PushLiteral(ir::LiteralValue::Boolean(false)) =
                &block.instructions[i]
            {
                if let ir::Instruction::BinaryOp(ir::BinaryOperator::Ne) =
                    &block.instructions[i + 1]
                {
                    // x != false → x (skip both)
                    i += 2;
                    continue;
                }
            }

            // Pattern: PUSH true, NE → converts to negation (x != true = !x)
            // Keep this pattern as-is since we don't have a simple NOT instruction
            // The codegen will handle it appropriately
        }

        // Pattern: BitwiseNot followed by BitwiseNot → identity (removes both)
        if i + 1 < block.instructions.len()
            && matches!(&block.instructions[i], ir::Instruction::BitwiseNot)
            && matches!(&block.instructions[i + 1], ir::Instruction::BitwiseNot)
        {
            i += 2;
            continue;
        }

        optimized.push(block.instructions[i].clone());
        i += 1;
    }

    block.instructions = optimized;
}
