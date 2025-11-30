//! IR Optimization Module
//!
//! Performs optimization passes on the intermediate representation (IR) before
//! bytecode generation. These optimizations reduce code size and improve runtime
//! performance.
//!
//! # Optimization Levels
//!
//! - **Level 0**: No IR optimization
//! - **Level 1**: Basic dead code elimination
//! - **Level 2**: Constant folding and propagation
//! - **Level 3**: NeoVM-specific optimizations (identity ops, boolean simplification)

use neo_solidity::ir;
use num_traits::{ToPrimitive, Zero};

/// IR optimization hook. Currently performs simple control-flow cleanup to drop
/// instructions that appear after a terminal return in a basic block.
pub(crate) fn optimize_ir(mut module: ir::Module, optimizer_level: u8) -> ir::Module {
    if optimizer_level == 0 {
        return module;
    }

    let enable_neovm_specific = optimizer_level >= 3;
    let enable_constant_folding = optimizer_level >= 2;

    for function in &mut module.functions {
        let mut label_remap = std::collections::HashMap::new();

        for block in &mut function.basic_blocks {
            let mut trimmed = Vec::with_capacity(block.instructions.len());
            let mut terminated = false;

            for instr in block.instructions.drain(..) {
                if terminated {
                    // Preserve labels so jump targets remain addressable, drop other ops.
                    if matches!(instr, ir::Instruction::Label(_)) {
                        trimmed.push(instr);
                        terminated = false; // New label can be targeted, resume collection
                    }
                    continue;
                }

                match instr {
                    ir::Instruction::Return
                    | ir::Instruction::ReturnVoid
                    | ir::Instruction::ReturnDefault(_) => {
                        trimmed.push(instr);
                        terminated = true;
                    }
                    ir::Instruction::Jump { .. } => {
                        trimmed.push(instr);
                        terminated = true;
                    }
                    other => trimmed.push(other),
                }
            }

            block.instructions = trimmed;

            if enable_constant_folding {
                fold_constant_binary_ops(block);
            }

            if enable_neovm_specific {
                dedupe_labels(block, &mut label_remap);
                remove_trivial_jumps(block);

                // NeoVM-specific optimizations at O3
                neovm_peephole_optimize(block);
                neovm_simplify_identity_ops(block);
                neovm_bool_optimize(block);

                // Run peephole again to catch newly exposed patterns
                neovm_peephole_optimize(block);
            }
        }

        if enable_neovm_specific && !label_remap.is_empty() {
            retarget_jumps(&mut function.basic_blocks, &label_remap);
        }
    }

    module
}

fn fold_constant_binary_ops(block: &mut ir::BasicBlock) {
    let mut optimized = Vec::with_capacity(block.instructions.len());
    let mut i = 0;

    while i < block.instructions.len() {
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

        optimized.push(block.instructions[i].clone());
        i += 1;
    }

    block.instructions = optimized;
}

fn remove_trivial_jumps(block: &mut ir::BasicBlock) {
    let mut optimized = Vec::with_capacity(block.instructions.len());
    let mut iter = block.instructions.iter();
    while let Some(instr) = iter.next() {
        if let ir::Instruction::Jump { target } = instr {
            if let Some(ir::Instruction::Label(id)) = iter.clone().next() {
                if id == target {
                    // Skip the jump; fallthrough reaches the label
                    continue;
                }
            }
        }
        optimized.push(instr.clone());
    }
    block.instructions = optimized;
}

fn dedupe_labels(block: &mut ir::BasicBlock, remap: &mut std::collections::HashMap<usize, usize>) {
    let mut optimized = Vec::with_capacity(block.instructions.len());
    let mut last_label: Option<usize> = None;

    for instr in block.instructions.drain(..) {
        match instr {
            ir::Instruction::Label(id) => {
                if let Some(prev) = last_label {
                    remap.insert(id, prev);
                    continue;
                } else {
                    last_label = Some(id);
                    optimized.push(ir::Instruction::Label(id));
                }
            }
            other => {
                last_label = None;
                optimized.push(other);
            }
        }
    }

    block.instructions = optimized;
}

fn retarget_jumps(blocks: &mut [ir::BasicBlock], remap: &std::collections::HashMap<usize, usize>) {
    for block in blocks {
        for instr in &mut block.instructions {
            match instr {
                ir::Instruction::Jump { target } | ir::Instruction::JumpIf { target } => {
                    if let Some(canonical) = remap.get(target) {
                        *target = *canonical;
                    }
                }
                _ => {}
            }
        }
    }
}

fn evaluate_binary_literal(
    lhs: &ir::LiteralValue,
    rhs: &ir::LiteralValue,
    op: ir::BinaryOperator,
) -> Option<ir::LiteralValue> {
    use ir::LiteralValue::*;

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
        if i + 1 < block.instructions.len() {
            if matches!(&block.instructions[i], ir::Instruction::BitwiseNot)
                && matches!(&block.instructions[i + 1], ir::Instruction::BitwiseNot)
            {
                i += 2;
                continue;
            }
        }

        optimized.push(block.instructions[i].clone());
        i += 1;
    }

    block.instructions = optimized;
}
