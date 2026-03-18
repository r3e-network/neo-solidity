fn try_lower_member_builtin(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    let Expression::MemberAccess(_, inner, member) = func else {
        return None;
    };
    let Expression::Variable(base) = inner.as_ref() else {
        return None;
    };

    if base.name == "abi"
        && matches!(
            member.name.as_str(),
            "encodeWithSignature" | "encodeWithSelector"
        )
    {
        ctx.record_warning_with_suggestion(
            format!(
                "abi.{}(...) is approximated on Neo N3 as selector bytes concatenated with abi.encode(args). This differs from raw EVM calldata semantics.",
                member.name
            ),
            "Prefer typed contract calls when possible, or use the returned bytes as a Neo-side calldata approximation only.",
        );

        if member.name == "encodeWithSignature" {
            let Some((signature_expr, payload_args)) = args.split_first() else {
                ctx.record_error("abi.encodeWithSignature requires a signature argument");
                return Some(false);
            };

            let Some(signature) = resolve_signature_string(signature_expr, ctx) else {
                ctx.record_error(
                    "abi.encodeWithSignature signature must be a string literal or a constant string",
                );
                return Some(false);
            };

            let mut hasher = Keccak256::new();
            hasher.update(signature.as_bytes());
            let digest = hasher.finalize();
            let selector = digest[..4].to_vec();

            instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(selector)));

            let mut success = true;
            for arg in payload_args {
                if !lower_expression(arg, ctx, instructions) {
                    success = false;
                }
            }
            if !success {
                return Some(false);
            }

            if !payload_args.is_empty() {
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::AbiEncode,
                    arg_count: payload_args.len(),
                });
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::BytesConcat,
                    arg_count: 2,
                });
            }

            return Some(true);
        }

        let Some((selector_expr, payload_args)) = args.split_first() else {
            ctx.record_error("abi.encodeWithSelector requires a selector argument");
            return Some(false);
        };

        if !lower_expression(selector_expr, ctx, instructions) {
            return Some(false);
        }

        let mut success = true;
        for arg in payload_args {
            if !lower_expression(arg, ctx, instructions) {
                success = false;
            }
        }
        if !success {
            return Some(false);
        }

        if !payload_args.is_empty() {
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::AbiEncode,
                arg_count: payload_args.len(),
            });
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::BytesConcat,
                arg_count: 2,
            });
        }

        return Some(true);
    }

    if let Some(result) = try_lower_runtime_member_builtin(base, member, args, ctx, instructions) {
        return Some(result);
    }

    if let Some(result) = try_lower_syscalls_member_builtin(base, member, args, ctx, instructions)
    {
        return Some(result);
    }

    if let Some(result) = try_lower_storage_member_builtin(base, member, args, ctx, instructions) {
        return Some(result);
    }

    if let Some(result) = try_lower_neo_member_builtin(base, member, args, ctx, instructions) {
        return Some(result);
    }

    if let Some(result) =
        try_lower_nativecalls_member_builtin(base, member, args, ctx, instructions)
    {
        return Some(result);
    }

    None
}
