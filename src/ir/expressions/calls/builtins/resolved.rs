fn try_lower_resolved_builtin_call(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    let builtin = resolve_builtin_call(func)?;

    if matches!(&builtin, BuiltinCall::RuntimeNotify) {
        validate_runtime_notify_call(args, ctx);
    }

    if matches!(&builtin, BuiltinCall::TypeOf) {
        ctx.record_error(
            "`type(...)` is only supported via members like `type(uint256).max`, `type(int256).min`, or `type(IInterface).interfaceId`",
        );
        return Some(false);
    }

    let (min_args, max_args) = match &builtin {
        BuiltinCall::RuntimeNotify => (2, Some(2)),
        BuiltinCall::RuntimeCheckWitness => (1, Some(1)),
        // Solidity allows abi.encode()/encodePacked() with zero arguments.
        BuiltinCall::AbiEncode | BuiltinCall::AbiEncodePacked | BuiltinCall::AbiEncodeCall => (0, None),
        BuiltinCall::AbiEncodeWithSignature => (1, None),
        BuiltinCall::AbiDecode => (2, None),
        BuiltinCall::Keccak256 => (1, Some(1)),
        BuiltinCall::Ecrecover => (4, Some(4)),
        BuiltinCall::StorageFind => (1, None),
        BuiltinCall::StoragePut => (2, Some(2)),
        BuiltinCall::StorageGet | BuiltinCall::StorageDelete => (1, Some(1)),
        BuiltinCall::ContractCall => (3, Some(3)),
        BuiltinCall::ContractCallWithFlags => (4, Some(4)),
        BuiltinCall::NotifySerialized => (1, Some(1)),
        BuiltinCall::VerifySignature => (3, Some(3)),
        BuiltinCall::DeployContract => (2, Some(3)),
        BuiltinCall::GetContract | BuiltinCall::GetContractScript => (1, Some(1)),
        BuiltinCall::GetNeoAccountState => (1, Some(1)),
        BuiltinCall::NativeCall { .. } => (0, None),
        BuiltinCall::Syscall(_) => (0, None),
        BuiltinCall::TypeOf => (1, Some(1)),
        BuiltinCall::BytesConcat => (0, None),
    };

    if args.len() < min_args || max_args.is_some_and(|max| args.len() > max) {
        ctx.record_error(format!(
            "builtin call requires between {} and {} argument(s), got {}",
            min_args,
            max_args
                .map(|v| v.to_string())
                .unwrap_or_else(|| "∞".to_string()),
            args.len()
        ));
        return Some(false);
    }

    let mut success = true;
    match &builtin {
        BuiltinCall::AbiDecode => {
            if let Some(first) = args.first() {
                if !lower_expression(first, ctx, instructions) {
                    success = false;
                }
            }
        }
        _ => {
            for arg in args {
                if !lower_expression(arg, ctx, instructions) {
                    success = false;
                }
            }
        }
    }

    if success {
        match builtin {
            BuiltinCall::RuntimeNotify
            | BuiltinCall::RuntimeCheckWitness
            | BuiltinCall::AbiEncode
            | BuiltinCall::AbiEncodePacked
            | BuiltinCall::AbiEncodeCall
            | BuiltinCall::AbiDecode
            | BuiltinCall::Keccak256
            | BuiltinCall::Ecrecover
            | BuiltinCall::StorageFind
            | BuiltinCall::StoragePut
            | BuiltinCall::StorageGet
            | BuiltinCall::StorageDelete
            | BuiltinCall::ContractCallWithFlags
            | BuiltinCall::NotifySerialized
            | BuiltinCall::VerifySignature
            | BuiltinCall::DeployContract
            | BuiltinCall::GetContract
            | BuiltinCall::GetContractScript
            | BuiltinCall::GetNeoAccountState
            | BuiltinCall::NativeCall { .. }
            | BuiltinCall::Syscall(_)
            | BuiltinCall::BytesConcat => {
                instructions.push(Instruction::CallBuiltin {
                    builtin,
                    arg_count: args.len(),
                });
            }
            BuiltinCall::ContractCall => {
                if ctx.is_safe {
                    // In safe (view/pure) contexts, default contract calls to read-only.
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::from(0x05u8),
                    )));
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::ContractCallWithFlags,
                        arg_count: 4,
                    });
                } else {
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::ContractCall,
                        arg_count: args.len(),
                    });
                }
            }
            BuiltinCall::AbiEncodeWithSignature => {
                let mut selector = Vec::new();
                if let Some(Expression::StringLiteral(parts)) = args.first() {
                    let bytes = string_literal_bytes(parts);
                    let mut hasher = Keccak256::new();
                    hasher.update(&bytes);
                    let digest = hasher.finalize();
                    selector.extend_from_slice(&digest[..4]);
                }
                for _ in args {
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
                instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(selector)));
            }
            BuiltinCall::TypeOf => {}
        }
    }

    Some(success)
}
