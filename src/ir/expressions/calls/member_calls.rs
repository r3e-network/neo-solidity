fn try_lower_member_call(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    fn format_builtin_member_list(members: &[&str]) -> String {
        const MAX_SHOWN: usize = 12;
        if members.len() <= MAX_SHOWN {
            return members.join(", ");
        }

        format!(
            "{} … (+{} more)",
            members[..MAX_SHOWN].join(", "),
            members.len() - MAX_SHOWN
        )
    }

    fn resolve_static_library_base(inner: &Expression, ctx: &LoweringContext) -> Option<String> {
        match inner {
            Expression::Variable(lib_id)
                if !ctx.param_index_map.contains_key(&lib_id.name)
                    && ctx.resolve_local(&lib_id.name).is_none()
                    && !ctx.state_index_map.contains_key(&lib_id.name) =>
            {
                Some(lib_id.name.clone())
            }
            Expression::MemberAccess(_, namespace_expr, imported_symbol)
                if matches!(
                    namespace_expr.as_ref(),
                    Expression::Variable(namespace_id)
                        if !ctx.param_index_map.contains_key(&namespace_id.name)
                            && ctx.resolve_local(&namespace_id.name).is_none()
                            && !ctx.state_index_map.contains_key(&namespace_id.name)
                            && !ctx.is_contract_type_name(&namespace_id.name)
                ) && ctx.is_contract_type_name(&imported_symbol.name) =>
            {
                Some(imported_symbol.name.clone())
            }
            _ => None,
        }
    }

    fn resolve_contract_type_name(inner: &Expression, ctx: &LoweringContext) -> Option<String> {
        match inner {
            Expression::Variable(type_id) if ctx.is_contract_type_name(&type_id.name) => {
                Some(type_id.name.clone())
            }
            Expression::MemberAccess(_, namespace_expr, type_id)
                if matches!(
                    namespace_expr.as_ref(),
                    Expression::Variable(namespace_id)
                        if !ctx.param_index_map.contains_key(&namespace_id.name)
                            && ctx.resolve_local(&namespace_id.name).is_none()
                            && !ctx.state_index_map.contains_key(&namespace_id.name)
                            && !ctx.is_contract_type_name(&namespace_id.name)
                ) && ctx.is_contract_type_name(&type_id.name) =>
            {
                Some(type_id.name.clone())
            }
            _ => None,
        }
    }

    fn native_contract_from_constant(base: &str, constant: &str) -> Option<NativeContract> {
        if !matches!(base, "NativeCalls" | "NativeContracts") {
            return None;
        }

        match constant {
            "NEO_CONTRACT" => Some(NativeContract::Neo),
            "GAS_CONTRACT" => Some(NativeContract::Gas),
            "CONTRACT_MANAGEMENT" => Some(NativeContract::ContractManagement),
            "POLICY_CONTRACT" => Some(NativeContract::Policy),
            "ORACLE_CONTRACT" => Some(NativeContract::Oracle),
            "ROLE_MANAGEMENT" => Some(NativeContract::RoleManagement),
            "NOTARY_CONTRACT" => Some(NativeContract::Notary),
            "TREASURY_CONTRACT" => Some(NativeContract::Treasury),
            "LEDGER_CONTRACT" => Some(NativeContract::Ledger),
            "CRYPTO_LIB" => Some(NativeContract::CryptoLib),
            "STD_LIB" => Some(NativeContract::StdLib),
            _ => None,
        }
    }

    if let Expression::MemberAccess(_, inner, member) = func {
        // `super.method()` — resolve to the renamed base method preserved during
        // inheritance flattening. The flattener stores overridden base methods as
        // `__super_{methodName}` and records the mapping in `super_method_map`.
        if matches!(inner.as_ref(), Expression::Variable(id) if id.name == "super") {
            if let Some(super_name) = ctx.super_method_name(&member.name) {
                let super_name = super_name.to_string();
                let mut success = true;
                for arg in args {
                    if !lower_expression(arg, ctx, instructions) {
                        success = false;
                    }
                }
                if success {
                    let neo_name = ctx
                        .neo_function_name(&super_name, args.len())
                        .unwrap_or_else(|| super_name.clone());
                    instructions.push(Instruction::CallFunction {
                        name: neo_name,
                        arg_count: args.len(),
                    });
                }
                if ctx.is_void_function(&super_name) {
                    return Some(false);
                }
                return Some(success);
            }

            if ctx.function_names.contains(&member.name) {
                let mut success = true;
                for arg in args {
                    if !lower_expression(arg, ctx, instructions) {
                        success = false;
                    }
                }
                if success {
                    if let Some(neo_name) = ctx.neo_function_name(&member.name, args.len()) {
                        instructions.push(Instruction::CallFunction {
                            name: neo_name,
                            arg_count: args.len(),
                        });
                    } else {
                        success = false;
                    }
                }
                if ctx.is_void_function(&member.name) {
                    return Some(false);
                }
                return Some(success);
            }

            // No super method found — the method was not overridden or has no body.
            ctx.record_error_with_suggestion(
                format!(
                    "super.{}() cannot be resolved; no overridden base method with a body was found",
                    member.name
                ),
                "ensure the base contract defines this function with a body and marks it 'virtual'",
            );
            instructions.push(Instruction::PushLiteral(
                LiteralValue::Integer(BigInt::zero()),
            ));
            return Some(false);
        }

        // User-defined value type `wrap`/`unwrap` — compile as no-ops.
        // `TypeName.wrap(value)` and `TypeName.unwrap(value)` are identity operations
        // on NeoVM since user-defined value types are transparent type aliases.
        if (member.name == "wrap" || member.name == "unwrap") && args.len() == 1 {
            if let Expression::Variable(type_id) = inner.as_ref() {
                let is_type_alias = !ctx.param_index_map.contains_key(&type_id.name)
                    && ctx.resolve_local(&type_id.name).is_none()
                    && !ctx.state_index_map.contains_key(&type_id.name);
                if is_type_alias {
                    // No-op: just lower the single argument — the value passes through.
                    let ok = lower_expression(&args[0], ctx, instructions);
                    return Some(ok);
                }
            }
        }

        // Fallback: treat unresolved member calls on address-like values as external
        // contract calls. Lower to System.Contract.Call with default flags.
        let is_external_target = matches!(
            infer_type_from_expression(inner.as_ref(), ctx),
            Some(ValueType::Address)
        ) || matches!(
            inner.as_ref(),
            Expression::FunctionCall(_, cast_func, cast_args)
                if cast_args.len() == 1
                    && resolve_contract_type_name(cast_func.as_ref(), ctx).is_some()
                    && (matches!(
                        infer_type_from_expression(&cast_args[0], ctx),
                        Some(ValueType::Address)
                    ) || address_bytes_le_from_expression(&cast_args[0]).is_some())
        ) || matches!(
            inner.as_ref(),
            // Heuristic: chained member calls on a function-call result often
            // represent interface/contract handles (e.g. `_getRouter().foo()`).
            // If type inference cannot prove an internal target, treat it as
            // an external call target for compatibility.
            Expression::FunctionCall(_, _, _)
        );

        if is_external_target {
            if is_low_level_evm_member(&member.name) {
                let suggestion = match member.name.as_str() {
                    "delegatecall" => "delegatecall is not available on Neo N3; Neo contracts have isolated storage. Use Syscalls.contractCall() for cross-contract calls",
                    "staticcall" => "staticcall is not available on Neo N3; use view/pure functions or Syscalls.contractCallWithFlags() with ReadOnly flags",
                    _ => "Neo N3 does not support low-level EVM calls; use NativeCalls.sol for contract-to-contract interactions",
                };
                ctx.record_error_with_suggestion(
                    format!("unsupported low-level EVM call '{}'", member.name),
                    suggestion,
                );
                return Some(false);
            }

            // NativeCalls/NativeContracts expose native contract hashes as constants. When
            // users write `NativeCalls.GAS_CONTRACT.totalSupply()` or
            // `NativeContracts.GAS_CONTRACT.totalSupply()` we resolve the target at compile
            // time and emit a `NativeCall` builtin. This avoids wildcard manifest permissions
            // and unlocks CALLT/method-token optimizations.
            let native_contract = match inner.as_ref() {
                Expression::MemberAccess(_, base, constant) => match base.as_ref() {
                    Expression::Variable(id) => {
                        native_contract_from_constant(&id.name, &constant.name)
                    }
                    _ => None,
                },
                _ => None,
            };

            if let Some(contract) = native_contract {
                let mut success = true;
                for arg in args {
                    if !lower_expression(arg, ctx, instructions) {
                        success = false;
                    }
                }

                if success {
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::NativeCall {
                            contract,
                            method: member.name.clone(),
                        },
                        arg_count: args.len(),
                    });
                }

                return Some(success);
            }

            // Neo N3 syscall convention: the first argument is at the top of the stack.
            // `System.Contract.Call(hash, method, flags, args)` therefore expects stack order:
            // `[args, flags, method, hash]` (top-of-stack is `hash`).
            //
            // Preserve Solidity evaluation order by evaluating the target expression first,
            // then the arguments, and only then arranging the stack for the syscall.
            let tmp_id = ctx.next_label();
            let target_slot = ctx.allocate_local(format!("__neo_extcall_target_{tmp_id}"), None);

            if !lower_expression(inner.as_ref(), ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(target_slot));

            // Build the argument array directly (more efficient than serialize+deserialize).
            let args_slot = ctx.allocate_local(
                format!("__neo_extcall_args_{tmp_id}"),
                Some(ValueType::Array(Box::new(ValueType::Any))),
            );

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
                args.len(),
            ))));
            instructions.push(Instruction::NewArray {
                element_type: ValueType::Any,
            });
            instructions.push(Instruction::StoreLocal(args_slot));

            for (index, arg) in args.iter().enumerate() {
                instructions.push(Instruction::LoadLocal(args_slot));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(index as u64),
                )));

                if !lower_expression(arg, ctx, instructions) {
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                }

                instructions.push(Instruction::ArraySet);
            }

            instructions.push(Instruction::LoadLocal(args_slot));

            // Use read-only call flags in `view`/`pure` contexts to align with Solidity's
            // static-call behavior and Neo N3 `safe` method expectations.
            let flags = if ctx.is_safe { 0x05u8 } else { 0x0Fu8 };
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
                flags,
            ))));
            instructions.push(Instruction::PushLiteral(LiteralValue::String(
                member.name.as_bytes().to_vec(),
            )));
            instructions.push(Instruction::LoadLocal(target_slot));

            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::Syscall("System.Contract.Call".to_string()),
                arg_count: 4,
            });
            return Some(true);
        }

        // Attempt to lower member calls as internal/library calls.
        if ctx.function_names.contains(&member.name) {
            let static_library_base = resolve_static_library_base(inner.as_ref(), ctx);
            let is_library_static = static_library_base.is_some();

            let mut success = true;
            if is_library_static {
                // Built-in libraries are lowered directly as compiler intrinsics. If builtin
                // resolution didn't match the member, do not fall back to calling an internal
                // function with the same name (which can silently miscompile into recursion).
                if static_library_base.as_deref().is_some_and(|base| {
                    matches!(
                        base,
                        "Runtime" | "abi" | "Storage" | "Syscalls" | "Neo" | "NativeCalls"
                    )
                }) {
                    let base = static_library_base.as_deref().unwrap_or("<library>");
                    let mut message =
                        format!("unsupported builtin library call '{base}.{}'", member.name);
                    if let Some(supported) = builtin_library_supported_members(base) {
                        message.push_str(&format!(
                            "; supported {base} intrinsics: {}",
                            format_builtin_member_list(supported)
                        ));
                    }
                    message.push_str(
                        ". (Builtin devpack libraries are compiler intrinsics; their Solidity bodies are not compiled.)",
                    );
                    ctx.record_error(message);
                    return Some(false);
                }

                for arg in args {
                    if !lower_expression(arg, ctx, instructions) {
                        success = false;
                    }
                }

                if success {
                    if let Some(neo_name) = ctx.neo_function_name(&member.name, args.len()) {
                        instructions.push(Instruction::CallFunction {
                            name: neo_name,
                            arg_count: args.len(),
                        });
                    } else {
                        for _ in args {
                            instructions.push(Instruction::Drop(ValueType::Any));
                        }
                        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                            BigInt::zero(),
                        )));
                    }
                }
                // Void functions don't push a value onto the stack, so return
                // false to prevent the caller from emitting a spurious DROP.
                if ctx.is_void_function(&member.name) {
                    return Some(false);
                }
                return Some(success);
            }

            let with_receiver = ctx
                .neo_function_name(&member.name, args.len() + 1);
            let without_receiver = ctx
                .neo_function_name(&member.name, args.len());

            let (neo_name, arg_count, use_receiver) = if let Some(name) = with_receiver {
                (name, args.len() + 1, true)
            } else if let Some(name) = without_receiver {
                // Compatibility fallback for merged library helpers where the
                // receiver parameter is erased in the lowered signature.
                (name, args.len(), false)
            } else {
                let mut success = true;
                if !lower_expression(inner.as_ref(), ctx, instructions) {
                    success = false;
                } else {
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
                for arg in args {
                    if !lower_expression(arg, ctx, instructions) {
                        success = false;
                    } else {
                        instructions.push(Instruction::Drop(ValueType::Any));
                    }
                }
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
                return Some(success);
            };

            if !lower_expression(inner.as_ref(), ctx, instructions) {
                success = false;
            } else if !use_receiver {
                // Preserve side effects of receiver evaluation.
                instructions.push(Instruction::Drop(ValueType::Any));
            }

            for arg in args {
                if !lower_expression(arg, ctx, instructions) {
                    success = false;
                }
            }

            if success {
                instructions.push(Instruction::CallFunction {
                    name: neo_name,
                    arg_count,
                });
            }

            return Some(success);
        }

        // NeoVM iterator handles are advanced via syscalls. Some devpacks express this as
        // `iterator.next()` / `iterator.value()` on a handle-like type; treat those as
        // `System.Iterator.Next` / `System.Iterator.Value` when no user-defined overload exists.
        if !ctx.function_names.contains(&member.name) && args.is_empty() {
            if member.name == "next" {
                if !lower_expression(inner.as_ref(), ctx, instructions) {
                    return Some(false);
                }
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::Syscall("System.Iterator.Next".to_string()),
                    arg_count: 1,
                });
                return Some(true);
            }

            if member.name == "value" {
                if !lower_expression(inner.as_ref(), ctx, instructions) {
                    return Some(false);
                }
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::Syscall("System.Iterator.Value".to_string()),
                    arg_count: 1,
                });
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::one(),
                )));
                instructions.push(Instruction::ArrayGet);
                return Some(true);
            }
        }

        // Builtin helper libraries (Runtime/Storage/Syscalls/Neo/NativeCalls/abi) are lowered
        // directly by the compiler. When a call targets one of these libraries but does not map
        // to a supported intrinsic, emit a targeted diagnostic rather than a generic
        // "unsupported external/library call" error.
        if let Some(base) = resolve_static_library_base(inner.as_ref(), ctx) {
            if let Some(supported) = builtin_library_supported_members(base.as_str()) {
                ctx.record_error(format!(
                    "unsupported builtin library call '{}.{}'; supported {} intrinsics: {}. (Builtin devpack libraries are compiler intrinsics; their Solidity bodies are not compiled.)",
                    base,
                    member.name,
                    base,
                    format_builtin_member_list(supported)
                ));
                return Some(false);
            }
        }

        if matches!(member.name.as_str(), "push" | "pop") {
            // Compatibility fallback for unresolved array-like helpers in upstream
            // libraries. Preserve side effects and keep control flow.
            let mut success = true;
            if !lower_expression(inner.as_ref(), ctx, instructions) {
                success = false;
            } else {
                instructions.push(Instruction::Drop(ValueType::Any));
            }
            for arg in args {
                if !lower_expression(arg, ctx, instructions) {
                    success = false;
                } else {
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
            }
            if success {
                if member.name == "push" {
                    instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
                } else {
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                }
            }
            return Some(success);
        }

        // Compatibility fallback for unresolved member calls.
        let mut success = true;
        if !lower_expression(inner.as_ref(), ctx, instructions) {
            success = false;
        } else {
            instructions.push(Instruction::Drop(ValueType::Any));
        }
        for arg in args {
            if !lower_expression(arg, ctx, instructions) {
                success = false;
            } else {
                instructions.push(Instruction::Drop(ValueType::Any));
            }
        }
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
        return Some(success);
    }

    None
}
