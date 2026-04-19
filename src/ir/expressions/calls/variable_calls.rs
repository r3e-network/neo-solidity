fn try_lower_variable_call(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if let Expression::Variable(identifier) = func {
        if identifier.name == "require" || identifier.name == "assert" {
            ctx.record_error(format!(
                "{}() cannot be used as an expression",
                identifier.name
            ));
            return Some(false);
        }

        if identifier.name == "selfdestruct" {
            // Neo N3 auto-compat: selfdestruct(addr) → ContractManagement.destroy()
            // Note: Neo destroy does NOT transfer remaining funds to addr.
            // The addr argument is evaluated (for side effects) then dropped.
            ctx.record_warning_with_suggestion(
                "selfdestruct() auto-mapped to ContractManagement.destroy() on Neo N3. The recipient address argument is ignored because Neo does not transfer remaining funds on destroy.",
                "Use NativeCalls.gasTransfer() to move funds before destroying the contract.",
            );
            if args.len() == 1 {
                // Evaluate the address argument for side effects, then drop it.
                if !lower_expression(&args[0], ctx, instructions) {
                    return Some(false);
                }
                instructions.push(Instruction::Drop(ValueType::Any));
            }
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::ContractManagement,
                    method: "destroy".to_string(),
                },
                arg_count: 0,
            });
            return Some(false); // void — no return value
        }

        if identifier.name == "blockhash" {
            // Neo N3 auto-compat: blockhash(n) → Ledger.getBlockHash(n)
            ctx.record_warning_with_suggestion(
                "blockhash() auto-mapped to Ledger.getBlockHash() on Neo N3.",
                "Use Ledger.getBlockHash(index) explicitly in Neo-native Solidity.",
            );
            if args.len() == 1 {
                if !lower_expression(&args[0], ctx, instructions) {
                    return Some(false);
                }
            } else {
                ctx.record_error("blockhash() requires exactly 1 argument");
                return Some(false);
            }
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Ledger,
                    method: "getBlockHash".to_string(),
                },
                arg_count: 1,
            });
            return Some(true);
        }

        if identifier.name == "gasleft" {
            // Neo N3 auto-compat: gasleft() → System.Runtime.GasLeft
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::Syscall("System.Runtime.GasLeft".to_string()),
                arg_count: 0,
            });
            return Some(true);
        }

        if identifier.name == "mulmod" || identifier.name == "addmod" {
            // EVM builtin compatibility:
            // - mulmod(a, b, m): m == 0 ? 0 : (a * b) % m
            // - addmod(a, b, m): m == 0 ? 0 : (a + b) % m
            if args.len() != 3 {
                ctx.record_error(format!(
                    "{}() requires exactly 3 arguments",
                    identifier.name
                ));
                return Some(false);
            }

            let tmp_id = ctx.next_label();
            let lhs_slot = ctx.allocate_local(format!("__{}_lhs_{tmp_id}", identifier.name), None);
            let rhs_slot = ctx.allocate_local(format!("__{}_rhs_{tmp_id}", identifier.name), None);
            let modulus_slot =
                ctx.allocate_local(format!("__{}_modulus_{tmp_id}", identifier.name), None);

            if !lower_expression(&args[0], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(lhs_slot));

            if !lower_expression(&args[1], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(rhs_slot));

            if !lower_expression(&args[2], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(modulus_slot));

            let compute_label = ctx.next_label();
            let end_label = ctx.next_label();

            instructions.push(Instruction::LoadLocal(modulus_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            // In this IR, JumpIf branches when the condition is false.
            // Jump to compute branch when modulus != 0.
            instructions.push(Instruction::JumpIf {
                target: compute_label,
            });

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::Jump { target: end_label });

            instructions.push(Instruction::Label(compute_label));
            instructions.push(Instruction::LoadLocal(lhs_slot));
            instructions.push(Instruction::LoadLocal(rhs_slot));
            instructions.push(Instruction::BinaryOp(if identifier.name == "mulmod" {
                BinaryOperator::Mul
            } else {
                BinaryOperator::Add
            }));
            instructions.push(Instruction::LoadLocal(modulus_slot));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Mod));
            instructions.push(Instruction::Label(end_label));
            return Some(true);
        }

        // Task #92 — `E(v)` enum value cast. Parser shape is `Variable(E)`
        // + one arg; previously this fell through to the unresolved-call
        // compatibility path that dropped `v` and pushed 0. Preserve the
        // discriminant and emit the Solidity-spec range guard: Panic(0x21)
        // when `v >= variant_count`.
        if args.len() == 1 {
            if let Some(variants) = ctx.enum_variant_map.get(&identifier.name) {
                let variant_count = variants.len() as u64;
                let fail_label = ctx.next_label();
                let ok_label = ctx.next_label();
                if !lower_expression(&args[0], ctx, instructions) {
                    instructions
                        .push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
                    return Some(false);
                }
                let tmp_id = ctx.next_label();
                let value_slot = ctx.allocate_local(
                    format!("__enum_cast_{tmp_id}"),
                    Some(ValueType::Integer { signed: false, bits: 8 }),
                );
                instructions.push(Instruction::StoreLocal(value_slot));
                // IR `JumpIf` branches when the operand is false. Compute
                // `v < variant_count`: when true, fall through and Jump to
                // `ok_label`; when false (v out-of-range), JumpIf branches
                // to `fail_label` and emits the Panic(0x21) payload.
                instructions.push(Instruction::LoadLocal(value_slot));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(variant_count),
                )));
                instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
                instructions.push(Instruction::JumpIf { target: fail_label });
                instructions.push(Instruction::Jump { target: ok_label });
                instructions.push(Instruction::Label(fail_label));
                // Task #107 — route through the shared `emit_panic` helper
                // which emits the canonical EVM Panic(uint256) envelope so
                // `catch Panic(uint code)` can bind code = 0x21.
                emit_panic(0x21, instructions);
                instructions.push(Instruction::Label(ok_label));
                instructions.push(Instruction::LoadLocal(value_slot));
                return Some(true);
            }
        }

        // Treat `ContractType(addressExpr)` as a no-op cast for known contract/interface
        // types when the argument is already address-like (including 20-byte hex literals).
        if args.len() == 1 && ctx.is_contract_type_name(&identifier.name) {
            if matches!(
                infer_type_from_expression(&args[0], ctx),
                Some(ValueType::Address)
            ) {
                return Some(lower_expression(&args[0], ctx, instructions));
            }

            if let Some(bytes) = address_bytes_le_from_expression(&args[0]) {
                instructions.push(Instruction::PushLiteral(LiteralValue::Address(bytes)));
                return Some(true);
            }
        }

        if ctx.function_names.contains(&identifier.name) {
            let mut success = true;
            for arg in args {
                if !lower_expression(arg, ctx, instructions) {
                    success = false;
                }
            }

            if success {
                if let Some(neo_name) = ctx.neo_function_name(&identifier.name, args.len()) {
                    instructions.push(Instruction::CallFunction {
                        name: neo_name,
                        arg_count: args.len(),
                    });
                } else {
                    ctx.record_error(format!(
                        "no overload of '{}' with {} argument(s)",
                        identifier.name,
                        args.len()
                    ));
                    success = false;
                }
            }

            // Void functions don't push a value onto the stack, so return
            // false to prevent the caller from emitting a spurious DROP.
            if ctx.is_void_function(&identifier.name) {
                return Some(false);
            }
            return Some(success);
        }

        if ctx.resolve_local(&identifier.name).is_some() {
            // Compatibility fallback for function-typed locals and unresolved callables.
            // Preserve argument side effects and materialize a default return value.
            let mut success = true;
            for arg in args {
                if !lower_expression(arg, ctx, instructions) {
                    success = false;
                } else {
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
            }
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            return Some(success);
        }

        // Compatibility fallback for unresolved free-function calls.
        let mut success = true;
        for arg in args {
            if !lower_expression(arg, ctx, instructions) {
                success = false;
            } else {
                instructions.push(Instruction::Drop(ValueType::Any));
            }
        }
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::zero(),
        )));
        return Some(success);
    }

    None
}
