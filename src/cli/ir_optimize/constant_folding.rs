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

/// Ceiling (in bits) on the magnitude of any literal produced by constant
/// folding. Mirrors `MAX_LITERAL_POW_EXP` in src/ir/expressions/power.rs:
/// without a bound, a one-line source like `return 1 << 18000000000000000000;`
/// makes the `Shl` fold allocate an exabyte-scale BigInt and abort the whole
/// compiler (SIGABRT) at the default `-O2`, and even sub-abort shifts like
/// `1 << 200000000` balloon the emitted .nef to tens of megabytes. Any legal
/// uint256/int256 constant (and 512-bit intermediate products) is far below
/// this ceiling; oversized expressions are simply left to the runtime ops —
/// the exact code `-O0`/`-O1` already emit for the same source.
const MAX_FOLDED_LITERAL_BITS: u64 = 4096;

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
            ir::BinaryOperator::Mul => {
                // The product has at most `a.bits() + b.bits()` bits; decline
                // to fold when that would exceed the literal-size ceiling so
                // chained huge-literal multiplies can't balloon compile-time
                // memory or the emitted bytecode.
                if a.bits().saturating_add(b.bits()) > MAX_FOLDED_LITERAL_BITS {
                    None
                } else {
                    Some(Integer(a * b))
                }
            }
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
                // `a << shift` materialises `shift` extra bits of BigInt with
                // no relation to the source size; leave oversized shifts to
                // the runtime Shl op instead of allocating them here.
                if a.bits().saturating_add(shift) > MAX_FOLDED_LITERAL_BITS {
                    None
                } else {
                    Some(Integer(a << shift))
                }
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

// Regression tests for the MAX_FOLDED_LITERAL_BITS guard (agent key:
// robustness). `1 << 18000000000000000000`-style folds used to abort the
// compiler with an exabyte-scale BigInt allocation at the default -O2.
// `ir_optimize.rs` includes further fragments after this one, so items from
// `labels.rs` would otherwise trip clippy::items_after_test_module.
#[allow(clippy::items_after_test_module)]
#[cfg(test)]
mod fold_literal_bits_guard_tests {
    use super::*;
    use num_bigint::BigInt;

    fn int(value: impl Into<BigInt>) -> ir::LiteralValue {
        ir::LiteralValue::Integer(value.into())
    }

    #[test]
    fn shl_declines_to_fold_oversized_shift() {
        // `1 << 18000000000000000000` — would allocate ~2.25 EB if folded.
        let result = evaluate_binary_literal(
            &int(1),
            &int(18_000_000_000_000_000_000u64),
            ir::BinaryOperator::Shl,
        );
        assert_eq!(result, None, "oversized shift must be left to runtime");

        // Sub-abort DoS shape: `1 << 200000000` used to emit a 25 MB .nef.
        let result =
            evaluate_binary_literal(&int(1), &int(200_000_000), ir::BinaryOperator::Shl);
        assert_eq!(result, None);
    }

    #[test]
    fn shl_still_folds_legal_uint256_constants() {
        let result = evaluate_binary_literal(&int(1), &int(255), ir::BinaryOperator::Shl)
            .expect("legal uint256 shift must keep folding");
        assert_eq!(result, int(BigInt::from(1) << 255u32));
    }

    #[test]
    fn mul_declines_to_fold_when_product_exceeds_ceiling() {
        let big = BigInt::from(1) << 4000u32; // 4001 bits
        let result =
            evaluate_binary_literal(&int(big.clone()), &int(big), ir::BinaryOperator::Mul);
        assert_eq!(result, None, "huge-literal product must be left to runtime");
    }

    #[test]
    fn mul_still_folds_uint256_operands() {
        // Two max-width uint256 operands: product is 512 bits, well under the
        // ceiling — legal constant arithmetic must keep folding.
        let max256: BigInt = (BigInt::from(1) << 256u32) - 1;
        let result = evaluate_binary_literal(
            &int(max256.clone()),
            &int(max256.clone()),
            ir::BinaryOperator::Mul,
        )
        .expect("uint256 product must keep folding");
        assert_eq!(result, int(&max256 * &max256));
    }
}

