fn try_lower_syscalls_member_builtin(
    base: &Identifier,
    member: &Identifier,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if base.name != "Syscalls" {
        return None;
    }

    match member.name.as_str() {
        "getNotifications" => {
            if args.is_empty() {
                // System.Runtime.GetNotifications accepts a nullable hash; omit the argument
                // by pushing Null to request all notifications.
                instructions.push(Instruction::PushLiteral(LiteralValue::Null));
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::Syscall("System.Runtime.GetNotifications".to_string()),
                    arg_count: 1,
                });
                return Some(true);
            }
            None
        }
        "scriptHashToAddress" => {
            if args.len() != 1 {
                ctx.record_error(format!(
                    "Syscalls.scriptHashToAddress requires 1 argument(s), got {}",
                    args.len()
                ));
                return Some(false);
            }

            if let Some(arg_ty) = infer_type_from_expression(&args[0], ctx) {
                if !matches!(
                    arg_ty,
                    ValueType::Address | ValueType::ByteArray { .. } | ValueType::Any
                ) {
                    ctx.record_error(format!(
                        "Syscalls.scriptHashToAddress expects a 20-byte ByteArray/Hash160-like value, got {arg_ty:?}"
                    ));
                    return Some(false);
                }
            }

            Some(lower_expression(&args[0], ctx, instructions))
        }
        "addressToScriptHash" => {
            if args.len() != 1 {
                ctx.record_error(format!(
                    "Syscalls.addressToScriptHash requires 1 argument(s), got {}",
                    args.len()
                ));
                return Some(false);
            }

            if let Some(arg_ty) = infer_type_from_expression(&args[0], ctx) {
                if !matches!(
                    arg_ty,
                    ValueType::Address | ValueType::ByteArray { .. } | ValueType::Any
                ) {
                    ctx.record_error(format!(
                        "Syscalls.addressToScriptHash expects an address/Hash160-like value, got {arg_ty:?}"
                    ));
                    return Some(false);
                }
            }

            Some(lower_expression(&args[0], ctx, instructions))
        }
        "isValidAddress" => {
            if args.len() != 1 {
                ctx.record_error(format!(
                    "Syscalls.isValidAddress requires 1 argument(s), got {}",
                    args.len()
                ));
                return Some(false);
            }

            if let Some(arg_ty) = infer_type_from_expression(&args[0], ctx) {
                if !matches!(
                    arg_ty,
                    ValueType::Address | ValueType::ByteArray { .. } | ValueType::Any
                ) {
                    ctx.record_error(format!(
                        "Syscalls.isValidAddress expects an address/Hash160-like value, got {arg_ty:?}"
                    ));
                    return Some(false);
                }
            }

            if !lower_expression(&args[0], ctx, instructions) {
                return Some(false);
            }

            // Equivalent to Solidity `addr != address(0)`.
            instructions.push(Instruction::PushLiteral(LiteralValue::Address(vec![
                0u8; 20
            ])));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Ne));
            Some(true)
        }
        _ => None,
    }
}
