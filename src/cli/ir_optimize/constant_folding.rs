fn fold_constant_binary_ops(block: &mut ir::BasicBlock) {
    let mut optimized = Vec::with_capacity(block.instructions.len());
    let mut i = 0;

    while i < block.instructions.len() {
        // Task #88 — fold `PushLiteral(const); JumpIf(target)` into an
        // unconditional Jump or a no-op. IR `JumpIf` branches when the
        // stack-top is FALSE (emitted as NeoVM JMPIFNOT_L); collapsing
        // the constant-condition guard lets the downstream terminator
        // pruner drop the dead arm's instructions, including any
        // `LoadState`/storage reads the compiler emitted for a
        // statically-unreachable `return s;` tail (see fuzz harness
        // batch35_k3_view_dead_branch_storage_read_not_eliminated).
        if i + 1 < block.instructions.len() {
            if let (
                ir::Instruction::PushLiteral(lit),
                ir::Instruction::JumpIf { target },
            ) = (&block.instructions[i], &block.instructions[i + 1]) {
                let branch_taken = match lit {
                    ir::LiteralValue::Boolean(b) => Some(!*b),
                    ir::LiteralValue::Integer(n) => Some(n.is_zero()),
                    _ => None,
                };
                match branch_taken {
                    Some(true) => {
                        // Constant condition is false → JumpIf always taken.
                        optimized.push(ir::Instruction::Jump { target: *target });
                        i += 2;
                        continue;
                    }
                    Some(false) => {
                        // Constant condition is true → JumpIf never taken.
                        i += 2;
                        continue;
                    }
                    None => {}
                }
            }
        }

        // Try constant folding for binary ops
        if i + 2 < block.instructions.len() {
            if let (
                ir::Instruction::PushLiteral(lhs),
                ir::Instruction::PushLiteral(rhs),
                ir::Instruction::BinaryOp(op),
            ) = (
                &block.instructions[i],
                &block.instructions[i + 1],
                &block.instructions[i + 2],
            ) {
                if let Some(result) = evaluate_binary_literal(lhs, rhs, *op) {
                    optimized.push(ir::Instruction::PushLiteral(result));
                    i += 3;
                    continue;
                }
            }
        }

        // Try identity elimination: x + 0, x * 1, etc.
        if i + 1 < block.instructions.len() {
            if let Some(simplified) = try_identity_elimination(
                &block.instructions[i],
                &block.instructions[i + 1],
            ) {
                if let Some(instr) = simplified {
                    optimized.push(instr);
                }
                // Skip both instructions (or just drop if None)
                i += 2;
                continue;
            }
        }

        optimized.push(block.instructions[i].clone());
        i += 1;
    }

    block.instructions = optimized;
}

/// Try to eliminate identity operations
fn try_identity_elimination(
    first: &ir::Instruction,
    second: &ir::Instruction,
) -> Option<Option<ir::Instruction>> {
    use ir::{Instruction::{PushLiteral, BinaryOp}, LiteralValue::Integer};

    match (first, second) {
        // PUSH 0, ADD -> no-op (keep original value)
        (PushLiteral(Integer(n)), BinaryOp(ir::BinaryOperator::Add)) if n.is_zero() => {
            Some(None)
        }
        // PUSH 1, MUL -> no-op
        (PushLiteral(Integer(n)), BinaryOp(ir::BinaryOperator::Mul)) if *n == 1.into() => {
            Some(None)
        }
        // PUSH 0, MUL -> replace with PUSH 0
        (PushLiteral(Integer(n)), BinaryOp(ir::BinaryOperator::Mul)) if n.is_zero() => {
            Some(Some(PushLiteral(Integer(0.into()))))
        }
        // PUSH 1, DIV -> no-op
        (PushLiteral(Integer(n)), BinaryOp(ir::BinaryOperator::Div)) if *n == 1.into() => {
            Some(None)
        }
        _ => None,
    }
}

fn evaluate_binary_literal(
    lhs: &ir::LiteralValue,
    rhs: &ir::LiteralValue,
    op: ir::BinaryOperator,
) -> Option<ir::LiteralValue> {
    use ir::LiteralValue::{Integer, Boolean};

    match (lhs, rhs) {
        (Integer(a), Integer(b)) => match op {
            ir::BinaryOperator::Add => Some(Integer(a + b)),
            ir::BinaryOperator::Sub => Some(Integer(a - b)),
            ir::BinaryOperator::Mul => Some(Integer(a * b)),
            ir::BinaryOperator::Div => {
                if b.is_zero() {
                    None
                } else {
                    Some(Integer(a / b))
                }
            }
            ir::BinaryOperator::Mod => {
                if b.is_zero() {
                    None
                } else {
                    Some(Integer(a % b))
                }
            }
            ir::BinaryOperator::BitAnd => Some(Integer(a & b)),
            ir::BinaryOperator::BitOr => Some(Integer(a | b)),
            ir::BinaryOperator::BitXor => Some(Integer(a ^ b)),
            ir::BinaryOperator::Shl => {
                let shift = b.to_u64()?;
                Some(Integer(a << shift))
            }
            ir::BinaryOperator::Shr => {
                let shift = b.to_u64()?;
                Some(Integer(a >> shift))
            }
            ir::BinaryOperator::Lt => Some(Boolean(a < b)),
            ir::BinaryOperator::Le => Some(Boolean(a <= b)),
            ir::BinaryOperator::Gt => Some(Boolean(a > b)),
            ir::BinaryOperator::Ge => Some(Boolean(a >= b)),
            ir::BinaryOperator::Eq => Some(Boolean(a == b)),
            ir::BinaryOperator::Ne => Some(Boolean(a != b)),
        },
        (Boolean(a), Boolean(b)) => match op {
            ir::BinaryOperator::Eq => Some(Boolean(a == b)),
            ir::BinaryOperator::Ne => Some(Boolean(a != b)),
            _ => None,
        },
        _ => None,
    }
}

