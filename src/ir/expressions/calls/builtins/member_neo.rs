fn try_lower_neo_member_builtin(
    base: &Identifier,
    member: &Identifier,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if base.name != "Neo" {
        return None;
    }

    match member.name.as_str() {
        "isCommittee" => {
            if args.len() != 1 {
                ctx.record_error(format!(
                    "Neo.isCommittee requires 1 argument(s), got {}",
                    args.len()
                ));
                return Some(false);
            }

            // Neo.getCommittee() returns committee public keys (ECPoint[]). On Neo N3, a
            // committee member account is the standard signature contract derived from one
            // of these keys. Use System.Contract.CreateStandardAccount to derive the UInt160
            // and compare against the provided address.
            let tmp_id = ctx.next_label();
            let account_slot = ctx.allocate_local(
                format!("__neo_is_committee_account_{tmp_id}"),
                Some(ValueType::Address),
            );
            let committee_slot = ctx.allocate_local(
                format!("__neo_is_committee_committee_{tmp_id}"),
                Some(ValueType::Any),
            );
            let index_slot = ctx.allocate_local(
                format!("__neo_is_committee_index_{tmp_id}"),
                Some(ValueType::Integer {
                    signed: false,
                    bits: 256,
                }),
            );

            if !lower_expression(&args[0], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(account_slot));

            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Neo,
                    method: "getCommittee".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::StoreLocal(committee_slot));

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::StoreLocal(index_slot));

            let loop_label = ctx.next_label();
            let advance_label = ctx.next_label();
            let done_label = ctx.next_label();
            let end_label = ctx.next_label();

            instructions.push(Instruction::Label(loop_label));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(committee_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
            instructions.push(Instruction::JumpIf { target: done_label });

            // committee[index]
            instructions.push(Instruction::LoadLocal(committee_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::Syscall("System.Contract.CreateStandardAccount".to_string()),
                arg_count: 1,
            });
            instructions.push(Instruction::LoadLocal(account_slot));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: advance_label,
            });

            instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
            instructions.push(Instruction::Jump { target: end_label });

            instructions.push(Instruction::Label(advance_label));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(index_slot));
            instructions.push(Instruction::Jump { target: loop_label });

            instructions.push(Instruction::Label(done_label));
            instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(false)));
            instructions.push(Instruction::Label(end_label));
            Some(true)
        }
        "getCommittee" => {
            if !args.is_empty() {
                ctx.record_error(format!(
                    "Neo.getCommittee requires 0 argument(s), got {}",
                    args.len()
                ));
                return Some(false);
            }

            // Neo native contract exposes committee members as ECPoint public keys.
            // Convert to UInt160 standard accounts via System.Contract.CreateStandardAccount.
            let tmp_id = ctx.next_label();
            let committee_keys_slot = ctx.allocate_local(
                format!("__neo_committee_keys_{tmp_id}"),
                Some(ValueType::Any),
            );
            let committee_addrs_slot = ctx.allocate_local(
                format!("__neo_committee_addrs_{tmp_id}"),
                Some(ValueType::Array(Box::new(ValueType::Address))),
            );
            let index_slot = ctx.allocate_local(
                format!("__neo_committee_index_{tmp_id}"),
                Some(ValueType::Integer {
                    signed: false,
                    bits: 256,
                }),
            );

            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Neo,
                    method: "getCommittee".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::StoreLocal(committee_keys_slot));

            instructions.push(Instruction::LoadLocal(committee_keys_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::NewArray {
                element_type: ValueType::Address,
            });
            instructions.push(Instruction::StoreLocal(committee_addrs_slot));

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::StoreLocal(index_slot));

            let loop_label = ctx.next_label();
            let done_label = ctx.next_label();
            instructions.push(Instruction::Label(loop_label));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(committee_keys_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
            instructions.push(Instruction::JumpIf { target: done_label });

            // committee_addrs[index] = CreateStandardAccount(committee_keys[index])
            instructions.push(Instruction::LoadLocal(committee_addrs_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(committee_keys_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::Syscall("System.Contract.CreateStandardAccount".to_string()),
                arg_count: 1,
            });
            instructions.push(Instruction::ArraySet);

            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(index_slot));
            instructions.push(Instruction::Jump { target: loop_label });

            instructions.push(Instruction::Label(done_label));
            instructions.push(Instruction::LoadLocal(committee_addrs_slot));
            Some(true)
        }
        "getValidators" => {
            if !args.is_empty() {
                ctx.record_error(format!(
                    "Neo.getValidators requires 0 argument(s), got {}",
                    args.len()
                ));
                return Some(false);
            }

            // Neo native contract exposes validators as ECPoint public keys.
            // Convert to UInt160 standard accounts via System.Contract.CreateStandardAccount.
            let tmp_id = ctx.next_label();
            let validator_keys_slot = ctx.allocate_local(
                format!("__neo_validator_keys_{tmp_id}"),
                Some(ValueType::Any),
            );
            let validator_addrs_slot = ctx.allocate_local(
                format!("__neo_validator_addrs_{tmp_id}"),
                Some(ValueType::Array(Box::new(ValueType::Address))),
            );
            let index_slot = ctx.allocate_local(
                format!("__neo_validator_index_{tmp_id}"),
                Some(ValueType::Integer {
                    signed: false,
                    bits: 256,
                }),
            );

            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Neo,
                    method: "getNextBlockValidators".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::StoreLocal(validator_keys_slot));

            instructions.push(Instruction::LoadLocal(validator_keys_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::NewArray {
                element_type: ValueType::Address,
            });
            instructions.push(Instruction::StoreLocal(validator_addrs_slot));

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::StoreLocal(index_slot));

            let loop_label = ctx.next_label();
            let done_label = ctx.next_label();
            instructions.push(Instruction::Label(loop_label));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(validator_keys_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
            instructions.push(Instruction::JumpIf { target: done_label });

            instructions.push(Instruction::LoadLocal(validator_addrs_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(validator_keys_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::Syscall("System.Contract.CreateStandardAccount".to_string()),
                arg_count: 1,
            });
            instructions.push(Instruction::ArraySet);

            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(index_slot));
            instructions.push(Instruction::Jump { target: loop_label });

            instructions.push(Instruction::Label(done_label));
            instructions.push(Instruction::LoadLocal(validator_addrs_slot));
            Some(true)
        }
        "isValidator" => {
            if args.len() != 1 {
                ctx.record_error(format!(
                    "Neo.isValidator requires 1 argument(s), got {}",
                    args.len()
                ));
                return Some(false);
            }

            let tmp_id = ctx.next_label();
            let account_slot = ctx.allocate_local(
                format!("__neo_is_validator_account_{tmp_id}"),
                Some(ValueType::Address),
            );
            let validators_slot = ctx.allocate_local(
                format!("__neo_is_validator_keys_{tmp_id}"),
                Some(ValueType::Any),
            );
            let index_slot = ctx.allocate_local(
                format!("__neo_is_validator_index_{tmp_id}"),
                Some(ValueType::Integer {
                    signed: false,
                    bits: 256,
                }),
            );

            if !lower_expression(&args[0], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(account_slot));

            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Neo,
                    method: "getNextBlockValidators".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::StoreLocal(validators_slot));

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::StoreLocal(index_slot));

            let loop_label = ctx.next_label();
            let advance_label = ctx.next_label();
            let done_label = ctx.next_label();
            let end_label = ctx.next_label();

            instructions.push(Instruction::Label(loop_label));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(validators_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
            instructions.push(Instruction::JumpIf { target: done_label });

            instructions.push(Instruction::LoadLocal(validators_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::Syscall("System.Contract.CreateStandardAccount".to_string()),
                arg_count: 1,
            });
            instructions.push(Instruction::LoadLocal(account_slot));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: advance_label,
            });

            instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
            instructions.push(Instruction::Jump { target: end_label });

            instructions.push(Instruction::Label(advance_label));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(index_slot));
            instructions.push(Instruction::Jump { target: loop_label });

            instructions.push(Instruction::Label(done_label));
            instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(false)));
            instructions.push(Instruction::Label(end_label));
            Some(true)
        }
        _ => None,
    }
}
