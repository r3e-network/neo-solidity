use super::*;
use neo_solidity::ir::{
    BasicBlock, Function, FunctionKind as IrFunctionKind, Instruction, Module, ValueType,
};

#[test]
fn optimize_ir_trims_unreachable_instructions() {
    let module = Module {
        functions: vec![Function {
            name: "foo".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![BasicBlock {
                instructions: vec![
                    Instruction::Return,
                    Instruction::Drop(ValueType::Boolean),
                    Instruction::ReturnVoid,
                ],
            }],
            local_count: 0,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 2);
    let instrs = &optimized.functions[0].basic_blocks[0].instructions;
    assert_eq!(instrs.len(), 1);
    assert!(matches!(instrs[0], Instruction::Return));
}

#[test]
fn optimize_ir_trims_after_unconditional_jump() {
    let module = Module {
        functions: vec![Function {
            name: "bar".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![BasicBlock {
                instructions: vec![
                    Instruction::Jump { target: 1 },
                    Instruction::Drop(ValueType::Boolean),
                    Instruction::Label(1),
                    Instruction::ReturnVoid,
                ],
            }],
            local_count: 0,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 1);
    let instrs = &optimized.functions[0].basic_blocks[0].instructions;
    assert_eq!(instrs.len(), 3);
    assert!(matches!(instrs[0], Instruction::Jump { .. }));
    assert!(matches!(instrs[1], Instruction::Label(_)));
    assert!(matches!(instrs[2], Instruction::ReturnVoid));
}

#[test]
fn optimize_ir_noop_when_disabled() {
    let module = Module {
        functions: vec![Function {
            name: "noop".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![BasicBlock {
                instructions: vec![Instruction::Return, Instruction::Drop(ValueType::Boolean)],
            }],
            local_count: 0,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 0);
    let instrs = &optimized.functions[0].basic_blocks[0].instructions;
    assert_eq!(instrs.len(), 2);
    assert!(matches!(instrs[0], Instruction::Return));
    assert!(matches!(instrs[1], Instruction::Drop(_)));
}

#[test]
fn dedupe_labels_when_optimizer_high() {
    let module = Module {
        functions: vec![Function {
            name: "labels".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![BasicBlock {
                instructions: vec![
                    Instruction::Label(1),
                    Instruction::Label(2),
                    Instruction::ReturnVoid,
                ],
            }],
            local_count: 0,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 3);
    let instrs = &optimized.functions[0].basic_blocks[0].instructions;
    let label_count = instrs
        .iter()
        .filter(|i| matches!(i, Instruction::Label(_)))
        .count();
    assert_eq!(label_count, 1);
}

#[test]
fn deduped_labels_retarget_jumps() {
    let module = Module {
        functions: vec![Function {
            name: "labels".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![BasicBlock {
                instructions: vec![
                    Instruction::Label(1),
                    Instruction::Label(2),
                    Instruction::Jump { target: 2 },
                    Instruction::ReturnVoid,
                ],
            }],
            local_count: 0,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 3);
    let instrs = &optimized.functions[0].basic_blocks[0].instructions;
    assert!(
        matches!(instrs[0], Instruction::Label(1)),
        "first label should be preserved"
    );
    match &instrs[1] {
        Instruction::Jump { target } => {
            assert_eq!(*target, 1, "jump should retarget to canonical label")
        }
        other => panic!("expected jump after label, got {:?}", other),
    }
    let label_count = instrs
        .iter()
        .filter(|i| matches!(i, Instruction::Label(_)))
        .count();
    assert_eq!(label_count, 1);
}

#[test]
fn deduped_labels_retarget_jumps_across_blocks() {
    let module = Module {
        functions: vec![Function {
            name: "labels2".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![
                BasicBlock {
                    instructions: vec![Instruction::Jump { target: 2 }, Instruction::ReturnVoid],
                },
                BasicBlock {
                    instructions: vec![
                        Instruction::Label(1),
                        Instruction::Label(2),
                        Instruction::ReturnVoid,
                    ],
                },
            ],
            local_count: 0,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 3);
    let jump_instrs = &optimized.functions[0].basic_blocks[0].instructions;
    match &jump_instrs[0] {
        Instruction::Jump { target } => assert_eq!(
            *target, 1,
            "jump should retarget to canonical label across blocks"
        ),
        other => panic!("expected jump instruction, got {:?}", other),
    }
    let label_instrs = &optimized.functions[0].basic_blocks[1].instructions;
    assert!(
        matches!(label_instrs[0], Instruction::Label(1)),
        "first label should be retained"
    );
    let label_count = label_instrs
        .iter()
        .filter(|i| matches!(i, Instruction::Label(_)))
        .count();
    assert_eq!(label_count, 1);
}

#[test]
fn removes_trivial_jump_to_next_label() {
    let module = Module {
        functions: vec![Function {
            name: "trivialjump".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![BasicBlock {
                instructions: vec![
                    Instruction::Jump { target: 1 },
                    Instruction::Label(1),
                    Instruction::ReturnVoid,
                ],
            }],
            local_count: 0,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 3);
    let instrs = &optimized.functions[0].basic_blocks[0].instructions;
    assert!(
        !matches!(instrs[0], Instruction::Jump { .. }),
        "trivial jump should be removed"
    );
    assert!(matches!(instrs[0], Instruction::Label(1)));
    assert!(matches!(instrs[1], Instruction::ReturnVoid));
}

#[test]
fn neovm_peephole_removes_push_drop() {
    use neo_solidity::ir::LiteralValue;
    use num_bigint::BigInt;

    let module = Module {
        functions: vec![Function {
            name: "peephole".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![BasicBlock {
                instructions: vec![
                    Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(42))),
                    Instruction::Drop(ValueType::Integer { signed: false, bits: 256 }),
                    Instruction::ReturnVoid,
                ],
            }],
            local_count: 0,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 3);
    let instrs = &optimized.functions[0].basic_blocks[0].instructions;
    // PUSH followed by DROP should be removed
    assert_eq!(instrs.len(), 1);
    assert!(matches!(instrs[0], Instruction::ReturnVoid));
}

#[test]
fn neovm_peephole_removes_load_local_drop() {
    let module = Module {
        functions: vec![Function {
            name: "peephole_local".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![BasicBlock {
                instructions: vec![
                    Instruction::LoadLocal(0),
                    Instruction::Drop(ValueType::Integer { signed: false, bits: 256 }),
                    Instruction::ReturnVoid,
                ],
            }],
            local_count: 1,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 3);
    let instrs = &optimized.functions[0].basic_blocks[0].instructions;
    // LoadLocal followed by DROP should be removed
    assert_eq!(instrs.len(), 1);
    assert!(matches!(instrs[0], Instruction::ReturnVoid));
}

#[test]
fn neovm_simplify_removes_add_zero() {
    use neo_solidity::ir::{BinaryOperator, LiteralValue};
    use num_bigint::BigInt;

    let module = Module {
        functions: vec![Function {
            name: "identity_add".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![BasicBlock {
                instructions: vec![
                    Instruction::LoadLocal(0),
                    Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(0))),
                    Instruction::BinaryOp(BinaryOperator::Add),
                    Instruction::Return,
                ],
            }],
            local_count: 1,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 3);
    let instrs = &optimized.functions[0].basic_blocks[0].instructions;
    // x + 0 should be simplified to just x
    assert_eq!(instrs.len(), 2);
    assert!(matches!(instrs[0], Instruction::LoadLocal(0)));
    assert!(matches!(instrs[1], Instruction::Return));
}

#[test]
fn neovm_simplify_removes_mul_one() {
    use neo_solidity::ir::{BinaryOperator, LiteralValue};
    use num_bigint::BigInt;

    let module = Module {
        functions: vec![Function {
            name: "identity_mul".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![BasicBlock {
                instructions: vec![
                    Instruction::LoadLocal(0),
                    Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(1))),
                    Instruction::BinaryOp(BinaryOperator::Mul),
                    Instruction::Return,
                ],
            }],
            local_count: 1,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 3);
    let instrs = &optimized.functions[0].basic_blocks[0].instructions;
    // x * 1 should be simplified to just x
    assert_eq!(instrs.len(), 2);
    assert!(matches!(instrs[0], Instruction::LoadLocal(0)));
    assert!(matches!(instrs[1], Instruction::Return));
}

#[test]
fn neovm_bool_removes_eq_true() {
    use neo_solidity::ir::{BinaryOperator, LiteralValue};

    let module = Module {
        functions: vec![Function {
            name: "bool_eq_true".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![BasicBlock {
                instructions: vec![
                    Instruction::LoadLocal(0),
                    Instruction::PushLiteral(LiteralValue::Boolean(true)),
                    Instruction::BinaryOp(BinaryOperator::Eq),
                    Instruction::Return,
                ],
            }],
            local_count: 1,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 3);
    let instrs = &optimized.functions[0].basic_blocks[0].instructions;
    // x == true should be simplified to just x
    assert_eq!(instrs.len(), 2);
    assert!(matches!(instrs[0], Instruction::LoadLocal(0)));
    assert!(matches!(instrs[1], Instruction::Return));
}

#[test]
fn neovm_bool_removes_ne_false() {
    use neo_solidity::ir::{BinaryOperator, LiteralValue};

    let module = Module {
        functions: vec![Function {
            name: "bool_ne_false".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![BasicBlock {
                instructions: vec![
                    Instruction::LoadLocal(0),
                    Instruction::PushLiteral(LiteralValue::Boolean(false)),
                    Instruction::BinaryOp(BinaryOperator::Ne),
                    Instruction::Return,
                ],
            }],
            local_count: 1,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 3);
    let instrs = &optimized.functions[0].basic_blocks[0].instructions;
    // x != false should be simplified to just x
    assert_eq!(instrs.len(), 2);
    assert!(matches!(instrs[0], Instruction::LoadLocal(0)));
    assert!(matches!(instrs[1], Instruction::Return));
}

#[test]
fn neovm_double_bitwise_not_removed() {
    let module = Module {
        functions: vec![Function {
            name: "double_not".to_string(),
            kind: IrFunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![BasicBlock {
                instructions: vec![
                    Instruction::LoadLocal(0),
                    Instruction::BitwiseNot,
                    Instruction::BitwiseNot,
                    Instruction::Return,
                ],
            }],
            local_count: 1,
        }],
        state_variables: vec![],
        events: vec![],
    };

    let optimized = optimize_ir(module, 3);
    let instrs = &optimized.functions[0].basic_blocks[0].instructions;
    // ~~x should be simplified to just x
    assert_eq!(instrs.len(), 2);
    assert!(matches!(instrs[0], Instruction::LoadLocal(0)));
    assert!(matches!(instrs[1], Instruction::Return));
}
