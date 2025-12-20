fn try_lower_expression_calls(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    match expr {
        Expression::FunctionCallBlock(_, call, block) => {
            let _ = (call, block);
            ctx.record_error(
                "function call options (`{...}`) are not supported; Neo N3 requires explicit NEP-17 transfers (`NativeCalls.gasTransfer` / `NativeCalls.neoTransfer`) + `onNEP17Payment`",
            );
            Some(false)
        }
        Expression::NamedFunctionCall(_, func, args) => {
            if let Some(result) =
                try_lower_struct_constructor_named_call(func.as_ref(), args, ctx, instructions)
            {
                return Some(result);
            }

            ctx.record_error("named argument calls are not supported");
            Some(false)
        }
        Expression::FunctionCall(_, func, args) => {
            Some(lower_function_call_expression(func.as_ref(), args, ctx, instructions))
        }
        Expression::New(_, expr) => {
            Some(lower_new_expression(expr.as_ref(), ctx, instructions))
        }
        Expression::Type(_, ty) => {
            push_default_for_type(ty, instructions);
            Some(true)
        }
        Expression::Parenthesis(_, inner) => Some(lower_expression(inner, ctx, instructions)),
        Expression::MemberAccess(_, inner, member) => Some(lower_member_access_expression(
            expr,
            inner.as_ref(),
            member,
            ctx,
            instructions,
        )),
        _ => None,
    }
}

fn lower_new_expression(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    match expr {
        Expression::FunctionCall(_, func, args) => {
            // `new bytes(n)` / `new string(n)`
            if matches!(
                func.as_ref(),
                Expression::Type(_, PtType::DynamicBytes | PtType::String)
            ) {
                if args.len() != 1 {
                    ctx.record_error("new bytes/string expects exactly one length argument");
                    for arg in args {
                        if lower_expression(arg, ctx, instructions) {
                            instructions.push(Instruction::Drop(ValueType::Any));
                        }
                    }
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                } else if !lower_expression(&args[0], ctx, instructions) {
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                }

                instructions.push(Instruction::NewBuffer);
                return true;
            }

            // `new T[](n)` dynamic arrays.
            if let Expression::ArraySubscript(_, _, index) = func.as_ref() {
                if index.is_some() {
                    ctx.record_error("new fixed-size arrays are not supported");
                    for arg in args {
                        if lower_expression(arg, ctx, instructions) {
                            instructions.push(Instruction::Drop(ValueType::Any));
                        }
                    }
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                    instructions.push(Instruction::NewArray {
                        element_type: ValueType::Any,
                    });
                    return true;
                }

                let Some(ValueType::Array(element_type)) = infer_type_from_expression(func, ctx)
                else {
                    ctx.record_error(format!(
                        "unable to infer element type for new array allocation (`new {}`)",
                        func.as_ref()
                    ));
                    for arg in args {
                        if lower_expression(arg, ctx, instructions) {
                            instructions.push(Instruction::Drop(ValueType::Any));
                        }
                    }
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                    instructions.push(Instruction::NewArray {
                        element_type: ValueType::Any,
                    });
                    return true;
                };

                let tmp_id = ctx.next_label();
                let len_local = ctx.allocate_local(format!("__new_array_len_{tmp_id}"), None);
                if args.len() != 1 {
                    ctx.record_error("new array expects exactly one length argument");
                    for arg in args {
                        if lower_expression(arg, ctx, instructions) {
                            instructions.push(Instruction::Drop(ValueType::Any));
                        }
                    }
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                } else if !lower_expression(&args[0], ctx, instructions) {
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                }
                instructions.push(Instruction::StoreLocal(len_local));

                let array_type = ValueType::Array(element_type.clone());
                let array_local =
                    ctx.allocate_local(format!("__new_array_{tmp_id}"), Some(array_type));

                instructions.push(Instruction::LoadLocal(len_local));
                instructions.push(Instruction::NewArray {
                    element_type: (*element_type).clone(),
                });
                instructions.push(Instruction::StoreLocal(array_local));

                // Solidity initializes new memory arrays with element defaults. NeoVM NEWARRAY
                // fills with nulls, so explicitly write default values for value types.
                let idx_local = ctx.allocate_local(format!("__new_array_idx_{tmp_id}"), None);
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::zero(),
                )));
                instructions.push(Instruction::StoreLocal(idx_local));

                let loop_label = ctx.next_label();
                let end_label = ctx.next_label();

                instructions.push(Instruction::Label(loop_label));
                instructions.push(Instruction::LoadLocal(idx_local));
                instructions.push(Instruction::LoadLocal(len_local));
                instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
                instructions.push(Instruction::JumpIf { target: end_label });

                instructions.push(Instruction::LoadLocal(array_local));
                instructions.push(Instruction::LoadLocal(idx_local));
                push_default_for_value_type(element_type.as_ref(), ctx, instructions);
                instructions.push(Instruction::ArraySet);

                instructions.push(Instruction::LoadLocal(idx_local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
                instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
                instructions.push(Instruction::StoreLocal(idx_local));

                instructions.push(Instruction::Jump { target: loop_label });
                instructions.push(Instruction::Label(end_label));

                instructions.push(Instruction::LoadLocal(array_local));
                return true;
            }

            // `new Contract(...)` isn't supported on Neo N3 (no contract creation from within contracts).
            if let Expression::Variable(identifier) = func.as_ref() {
                if ctx.is_contract_type_name(&identifier.name) {
                    ctx.record_error(format!(
                        "contract creation via `new {}` is not supported on Neo N3; use ContractManagement.deploy from an admin/entry contract instead",
                        identifier.name
                    ));
                    for arg in args {
                        if lower_expression(arg, ctx, instructions) {
                            instructions.push(Instruction::Drop(ValueType::Any));
                        }
                    }
                    instructions.push(Instruction::PushLiteral(LiteralValue::Address(vec![
                        0u8;
                        20
                    ])));
                    return true;
                }
            }

            ctx.record_error("unsupported `new` expression");
            for arg in args {
                if lower_expression(arg, ctx, instructions) {
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
            }
            instructions.push(Instruction::PushLiteral(LiteralValue::Null));
            true
        }
        Expression::FunctionCallBlock(_, _, _) => {
            ctx.record_error("function call blocks on `new` are not supported");
            instructions.push(Instruction::PushLiteral(LiteralValue::Null));
            true
        }
        _ => {
            ctx.record_error("unsupported `new` expression");
            instructions.push(Instruction::PushLiteral(LiteralValue::Null));
            true
        }
    }
}
