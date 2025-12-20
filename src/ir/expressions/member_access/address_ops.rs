fn try_lower_address_balance(
    inner: &Expression,
    member: &Identifier,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if member.name != "balance" {
        return None;
    }

    if matches!(
        infer_type_from_expression(inner, ctx),
        Some(ValueType::Address)
    ) {
        if !lower_expression(inner, ctx, instructions) {
            return Some(false);
        }
        instructions.push(Instruction::CallBuiltin {
            builtin: BuiltinCall::NativeCall {
                contract: NativeContract::Gas,
                method: "balanceOf".to_string(),
            },
            arg_count: 1,
        });
        return Some(true);
    }

    None
}

fn try_lower_length_property(
    inner: &Expression,
    member: &Identifier,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if member.name != "length" {
        return None;
    }

    // Neo doesn't expose EVM bytecode, but many Solidity contracts use
    // `address.code.length > 0` as a contract-existence check. On Neo N3 we can
    // approximate this by calling `System.Contract.GetContract(address)`:
    // - null => non-contract => length 0
    // - non-null => contract exists => length 1
    if let Expression::MemberAccess(_, code_inner, code_member) = inner {
        if code_member.name == "code"
            && matches!(
                infer_type_from_expression(code_inner.as_ref(), ctx),
                Some(ValueType::Address)
            )
        {
            if !lower_expression(code_inner.as_ref(), ctx, instructions) {
                return Some(false);
            }

            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::Syscall("System.Contract.GetContract".to_string()),
                arg_count: 1,
            });
            instructions.push(Instruction::PushLiteral(LiteralValue::Null));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq)); // isNull

            let not_null_label = ctx.next_label();
            let end_label = ctx.next_label();

            // JumpIf branches on false, so this jumps when isNull == false.
            instructions.push(Instruction::JumpIf {
                target: not_null_label,
            });
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::Jump { target: end_label });
            instructions.push(Instruction::Label(not_null_label));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::Label(end_label));
            return Some(true);
        }
    }

    if let Expression::Variable(base) = inner {
        if let Some(state_index) = ctx.state_index_map.get(&base.name) {
            if matches!(ctx.state_type(*state_index), Some(ValueType::Array(_))) {
                // Storage array length is stored at the base slot.
                instructions.push(Instruction::LoadState(*state_index));
                return Some(true);
            }
        }
    }

    if lower_expression(inner, ctx, instructions) {
        instructions.push(Instruction::GetSize);
        return Some(true);
    }

    Some(false)
}

fn try_lower_current_key(
    inner: &Expression,
    member: &Identifier,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if member.name != "currentKey" {
        return None;
    }

    // Neo storage iterators yield [key, value] pairs. Devpacks sometimes model this as
    // a struct field; extract key via `System.Iterator.Value` + PICKITEM(0).
    if !lower_expression(inner, ctx, instructions) {
        return Some(false);
    }
    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::Syscall("System.Iterator.Value".to_string()),
        arg_count: 1,
    });
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::ArrayGet);
    Some(true)
}

fn try_lower_code_property(
    inner: &Expression,
    member: &Identifier,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if member.name != "code" {
        return None;
    }

    if lower_expression(inner, ctx, instructions) {
        instructions.push(Instruction::Drop(ValueType::Any));
        instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(vec![])));
        return Some(true);
    }

    Some(false)
}
