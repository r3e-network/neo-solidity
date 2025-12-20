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
                    ir::Instruction::Abort
                    | ir::Instruction::AbortMsg
                    | ir::Instruction::Throw => {
                        trimmed.push(instr);
                        terminated = true;
                    }
                    ir::Instruction::EndTry { .. } => {
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
