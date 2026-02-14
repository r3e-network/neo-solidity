fn try_lower_variable_call(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if let Expression::Variable(identifier) = func {
        if identifier.name == "require" || identifier.name == "assert" {
            ctx.record_error(format!("{}() cannot be used as an expression", identifier.name));
            return Some(false);
        }

        if identifier.name == "selfdestruct" {
            // Neo N3 auto-compat: selfdestruct(addr) → ContractManagement.destroy()
            // Note: Neo destroy does NOT transfer remaining funds to addr.
            // The addr argument is evaluated (for side effects) then dropped.
            eprintln!(
                "warning: selfdestruct() auto-mapped to ContractManagement.destroy() \
                 on Neo N3. The recipient address argument is ignored — Neo does not \
                 transfer remaining funds on destroy. Use NativeCalls.gasTransfer() \
                 to move funds before destroying."
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
            eprintln!(
                "warning: blockhash() auto-mapped to Ledger.getBlockHash() \
                 on Neo N3. Returns the block hash for the given index."
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
                builtin: BuiltinCall::Syscall(
                    "System.Runtime.GasLeft".to_string(),
                ),
                arg_count: 0,
            });
            return Some(true);
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

        ctx.record_error_with_suggestion(
            format!("unsupported function call '{}'", identifier.name),
            "check spelling or ensure the function is declared in the same contract",
        );
        return Some(false);
    }

    None
}
