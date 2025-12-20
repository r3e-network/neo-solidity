fn try_lower_runtime_member_access(
    inner: &Expression,
    member: &Identifier,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    match member.name.as_str() {
        "sender" => {
            if let Expression::Variable(base) = inner {
                if base.name == "msg" {
                    if ctx.function_name == "onNEP17Payment" {
                        instructions.push(Instruction::LoadParameter(0));
                    } else {
                        instructions
                            .push(Instruction::LoadRuntimeValue(RuntimeValue::MsgSender));
                    }
                    return Some(true);
                }
            }
            None
        }
        "value" => {
            if let Expression::Variable(base) = inner {
                if base.name == "msg" {
                    if ctx.function_name == "onNEP17Payment" {
                        instructions.push(Instruction::LoadParameter(1));
                    } else {
                        instructions.push(Instruction::LoadRuntimeValue(RuntimeValue::MsgValue));
                    }
                    return Some(true);
                }
            }
            None
        }
        "data" => {
            if let Expression::Variable(base) = inner {
                if base.name == "msg" {
                    if ctx.function_name == "onNEP17Payment" {
                        instructions.push(Instruction::LoadParameter(2));
                    } else {
                        instructions.push(Instruction::LoadRuntimeValue(RuntimeValue::MsgData));
                    }
                    return Some(true);
                }
            }
            None
        }
        "origin" => {
            if let Expression::Variable(base) = inner {
                if base.name == "tx" {
                    instructions.push(Instruction::LoadRuntimeValue(RuntimeValue::TxOrigin));
                    return Some(true);
                }
            }
            None
        }
        "timestamp" => {
            if let Expression::Variable(base) = inner {
                if base.name == "block" {
                    instructions.push(Instruction::LoadRuntimeValue(RuntimeValue::BlockTimestamp));
                    return Some(true);
                }
            }
            None
        }
        "number" => {
            if let Expression::Variable(base) = inner {
                if base.name == "block" {
                    instructions.push(Instruction::LoadRuntimeValue(RuntimeValue::BlockNumber));
                    return Some(true);
                }
            }
            None
        }
        "chainid" => {
            if let Expression::Variable(base) = inner {
                if base.name == "block" {
                    // Solidity `block.chainid` is a uint256 chain identifier. Neo N3 exposes a
                    // network "magic" number via `System.Runtime.GetNetwork`; use that as the
                    // closest equivalent.
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::Syscall("System.Runtime.GetNetwork".to_string()),
                        arg_count: 0,
                    });
                    return Some(true);
                }
            }
            None
        }
        _ => None,
    }
}
