fn lower_try_statement(
    expr: &Expression,
    handler: &Option<(solang_parser::pt::ParameterList, Box<Statement>)>,
    catches: &[solang_parser::pt::CatchClause],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    // Solidity try/catch is only defined for external calls and contract creation.
    // NeoVM supports structured exception handling via TRY/ENDTRY/ENDFINALLY, so we can
    // map it directly. The NeoVM catch handler receives the thrown value on the stack;
    // Solidity catch blocks do not, so we either drop it or bind it to a catch parameter.

    let catch_label = ctx.next_label();
    let success_label = ctx.next_label();
    let end_label = ctx.next_label();

    let (call_expr, inline_success) = match expr {
        Expression::FunctionCallBlock(_, call, block) => (call.as_ref(), Some(block.as_ref())),
        _ => (expr, None),
    };

    let (handler_params, handler_stmt) = handler
        .as_ref()
        .map(|(params, stmt)| (params.as_slice(), Some(stmt.as_ref())))
        .unwrap_or((&[][..], None));

    let success_stmt = handler_stmt.or(inline_success);

    // Choose a catch clause to lower. Prefer a plain `catch {}` when present.
    let mut selected_catch: Option<(&solang_parser::pt::CatchClause, &Statement)> = None;
    for clause in catches {
        match clause {
            solang_parser::pt::CatchClause::Simple(_, param, stmt) => {
                if param.is_none() {
                    selected_catch = Some((clause, stmt));
                    break;
                }
                selected_catch.get_or_insert((clause, stmt));
            }
            solang_parser::pt::CatchClause::Named(_, _, _, stmt) => {
                selected_catch.get_or_insert((clause, stmt));
            }
        }
    }

    if selected_catch.is_none() {
        ctx.record_error("try statement without catch clause is not supported");
        if lower_expression(call_expr, ctx, instructions) {
            instructions.push(Instruction::Drop(ValueType::Any));
        }
        if let Some(success_stmt) = success_stmt {
            let _ = lower_statement(success_stmt, ctx, instructions);
        }
        return false;
    }

    // Lower the call expression inside a TRY, then ENDTRY to the success label.
    instructions.push(Instruction::Try {
        catch_target: catch_label,
    });

    let mut try_return_slot: Option<(usize, ValueType)> = None;
    if lower_expression(call_expr, ctx, instructions) {
        if handler_params.len() == 1 {
            if let Some(param) = handler_params[0].1.as_ref() {
                let inferred_type =
                    infer_type_from_expression(&param.ty, ctx).unwrap_or(ValueType::Any);
                let tmp = ctx.allocate_local("__try_ret".to_string(), Some(inferred_type.clone()));
                instructions.push(Instruction::StoreLocal(tmp));
                try_return_slot = Some((tmp, inferred_type));
            } else {
                ctx.record_error("try returns(...) parameter is missing");
                instructions.push(Instruction::Drop(ValueType::Any));
            }
        } else {
            if !handler_params.is_empty() {
                ctx.record_error("try returns(...) with multiple values is not supported");
            }
            instructions.push(Instruction::Drop(ValueType::Any));
        }
    } else if !handler_params.is_empty() {
        ctx.record_error("try returns(...) expects a return value");
    }

    instructions.push(Instruction::EndTry {
        target: success_label,
    });

    // Catch handler: bind or drop the thrown value, then execute the catch body.
    instructions.push(Instruction::Label(catch_label));
    ctx.enter_scope();

    if let Some((clause, catch_stmt)) = selected_catch {
        let mut bound = false;
        match clause {
            solang_parser::pt::CatchClause::Simple(_, param, _) => {
                if let Some(param) = param {
                    if let Some(name) = param.name.as_ref().map(|id| id.name.clone()) {
                        let inferred =
                            infer_type_from_expression(&param.ty, ctx).unwrap_or(ValueType::Any);
                        let slot = ctx.allocate_local(name, Some(inferred));
                        instructions.push(Instruction::StoreLocal(slot));
                        bound = true;
                    }
                }
            }
            solang_parser::pt::CatchClause::Named(_, _, param, _) => {
                if let Some(name) = param.name.as_ref().map(|id| id.name.clone()) {
                    let inferred =
                        infer_type_from_expression(&param.ty, ctx).unwrap_or(ValueType::Any);
                    let slot = ctx.allocate_local(name, Some(inferred));
                    instructions.push(Instruction::StoreLocal(slot));
                    bound = true;
                }
            }
        }

        if !bound {
            instructions.push(Instruction::Drop(ValueType::Any));
        }

        let _ = lower_statement(catch_stmt, ctx, instructions);
    } else {
        instructions.push(Instruction::Drop(ValueType::Any));
    }

    ctx.exit_scope();
    instructions.push(Instruction::EndTry { target: end_label });

    // Success handler: bind return values (if present) and run handler block.
    instructions.push(Instruction::Label(success_label));
    if let Some(success_stmt) = success_stmt {
        ctx.enter_scope();

        if handler_params.len() == 1 {
            if let Some(param) = handler_params[0].1.as_ref() {
                if let Some(name) = param.name.as_ref().map(|id| id.name.clone()) {
                    let inferred = try_return_slot
                        .as_ref()
                        .map(|(_, ty)| ty.clone())
                        .unwrap_or(ValueType::Any);
                    let slot = ctx.allocate_local(name, Some(inferred.clone()));
                    if let Some((tmp, _)) = try_return_slot {
                        instructions.push(Instruction::LoadLocal(tmp));
                        instructions.push(Instruction::StoreLocal(slot));
                    } else {
                        push_default_for_value_type(&inferred, ctx, instructions);
                        instructions.push(Instruction::StoreLocal(slot));
                    }
                }
            }
        }

        let _ = lower_statement(success_stmt, ctx, instructions);
        ctx.exit_scope();
    }

    instructions.push(Instruction::Label(end_label));
    false
}
