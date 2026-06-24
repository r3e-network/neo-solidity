use super::*;

pub(crate) fn try_lower_value_transfer_helpers(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    // Convenience wrappers commonly used in Neo-focused devpacks:
    //
    // - `Neo.transferGas(from, to, amount)` -> GAS.transfer(from, to, amount, data="")
    // - `Neo.transferNeo(from, to, amount)` -> NEO.transfer(from, to, amount, data="")
    if let Expression::MemberAccess(_, inner, member) = func {
        if let Expression::Variable(base) = inner.as_ref() {
            if base.name == "Neo" && matches!(args.len(), 3 | 4) {
                let contract = match member.name.as_str() {
                    "transferGas" => NativeContract::Gas,
                    "transferNeo" => NativeContract::Neo,
                    _ => return None,
                };
                if !lower_expression(&args[0], ctx, instructions) {
                    return Some(false);
                }
                if !lower_expression(&args[1], ctx, instructions) {
                    return Some(false);
                }
                if !lower_expression(&args[2], ctx, instructions) {
                    return Some(false);
                }

                if let Some(data) = args.get(3) {
                    if !lower_expression(data, ctx, instructions) {
                        return Some(false);
                    }
                } else {
                    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                        Vec::new(),
                    )));
                }

                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::NativeCall {
                        contract,
                        method: "transfer".to_string(),
                    },
                    arg_count: 4,
                });

                return Some(true);
            }
        }
    }

    // Solidity value-transfer helpers mapped to GAS NEP-17 transfers:
    //
    // - `address.transfer(amount)` aborts on failure (no return value in Solidity)
    // - `address.send(amount)` returns bool and does not abort
    //
    // Neo does not have an "attached value" for contract invocations; the closest
    // equivalent is calling the GAS native contract's `transfer` method.
    if let Expression::MemberAccess(_, inner, member) = func {
        if matches!(member.name.as_str(), "transfer" | "send") && args.len() == 1 {
            let is_address_target = matches!(
                infer_type_from_expression(inner.as_ref(), ctx),
                Some(ValueType::Address)
            );

            if is_address_target {
                // GAS.transfer(from, to, amount, data)
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::Syscall(
                        "System.Runtime.GetExecutingScriptHash".to_string(),
                    ),
                    arg_count: 0,
                });

                if !lower_expression(inner.as_ref(), ctx, instructions) {
                    return Some(false);
                }

                if !lower_expression(&args[0], ctx, instructions) {
                    return Some(false);
                }

                // `data` argument - empty by default.
                instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                    Vec::new(),
                )));

                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::NativeCall {
                        contract: NativeContract::Gas,
                        method: "transfer".to_string(),
                    },
                    arg_count: 4,
                });

                if member.name == "transfer" {
                    // Abort on failure, push a truthy sentinel on success.
                    let abort_label = ctx.next_label();
                    let end_label = ctx.next_label();
                    instructions.push(Instruction::JumpIf {
                        target: abort_label,
                    });
                    instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
                    instructions.push(Instruction::Jump { target: end_label });
                    instructions.push(Instruction::Label(abort_label));
                    instructions.push(Instruction::PushLiteral(LiteralValue::Null));
                    instructions.push(Instruction::Throw);
                    instructions.push(Instruction::Label(end_label));
                }

                return Some(true);
            }
        }
    }

    None
}
