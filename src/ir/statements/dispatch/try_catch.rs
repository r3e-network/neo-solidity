fn value_type_to_catch_guard(value_type: &ValueType) -> Option<ConvertTarget> {
    match value_type {
        ValueType::Any => None,
        ValueType::Boolean => Some(ConvertTarget::Boolean),
        ValueType::Integer { .. } => Some(ConvertTarget::Integer),
        ValueType::String | ValueType::Address | ValueType::ByteArray { .. } => {
            Some(ConvertTarget::ByteArray)
        }
        ValueType::Array(_) | ValueType::Struct { .. } => Some(ConvertTarget::Array),
        ValueType::Mapping { .. } => Some(ConvertTarget::Map),
    }
}

fn catch_clause_param(
    clause: &solang_parser::pt::CatchClause,
) -> Option<&solang_parser::pt::Parameter> {
    match clause {
        solang_parser::pt::CatchClause::Simple(_, param, _) => param.as_ref(),
        solang_parser::pt::CatchClause::Named(_, _, param, _) => Some(param),
    }
}

fn catch_clause_statement(clause: &solang_parser::pt::CatchClause) -> &Statement {
    match clause {
        solang_parser::pt::CatchClause::Simple(_, _, stmt) => stmt,
        solang_parser::pt::CatchClause::Named(_, _, _, stmt) => stmt,
    }
}

fn is_bare_catch_clause(clause: &solang_parser::pt::CatchClause) -> bool {
    matches!(clause, solang_parser::pt::CatchClause::Simple(_, None, _))
}

fn catch_clause_guard_target(
    clause: &solang_parser::pt::CatchClause,
    ctx: &mut LoweringContext,
) -> Option<ConvertTarget> {
    if let Some(parameter) = catch_clause_param(clause) {
        return infer_type_from_expression(&parameter.ty, ctx)
            .as_ref()
            .and_then(value_type_to_catch_guard);
    }

    if let solang_parser::pt::CatchClause::Named(_, ident, _, _) = clause {
        if ident.name == "Panic" {
            return Some(ConvertTarget::Integer);
        }
    }

    None
}

fn bind_catch_clause_parameter(
    clause: &solang_parser::pt::CatchClause,
    catch_local: usize,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    let Some(parameter) = catch_clause_param(clause) else {
        return;
    };

    let Some(name) = parameter.name.as_ref().map(|id| id.name.clone()) else {
        return;
    };

    let inferred = infer_type_from_expression(&parameter.ty, ctx).unwrap_or(ValueType::Any);
    let slot = ctx.allocate_local(name, Some(inferred));
    instructions.push(Instruction::LoadLocal(catch_local));
    instructions.push(Instruction::StoreLocal(slot));
}

fn lower_try_statement(
    expr: &Expression,
    handler: &Option<(solang_parser::pt::ParameterList, Box<Statement>)>,
    catches: &[solang_parser::pt::CatchClause],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    // Solidity try/catch is only defined for external calls and contract creation.
    // NeoVM supports structured exception handling via TRY/ENDTRY/ENDFINALLY, so we can
    // map it directly.

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

    if catches.is_empty() {
        ctx.record_error_with_suggestion(
            "try statement without catch clause is not supported",
            "add a catch clause: try expr { ... } catch { ... }",
        );
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

    let mut try_return_slots: Vec<(usize, ValueType)> = Vec::new();
    if lower_expression(call_expr, ctx, instructions) {
        if handler_params.len() == 1 {
            if let Some(param) = handler_params[0].1.as_ref() {
                let inferred_type =
                    infer_type_from_expression(&param.ty, ctx).unwrap_or(ValueType::Any);
                let tmp = ctx.allocate_local("__try_ret".to_string(), Some(inferred_type.clone()));
                instructions.push(Instruction::StoreLocal(tmp));
                try_return_slots.push((tmp, inferred_type));
            } else {
                ctx.record_error_with_suggestion(
                    "try returns(...) parameter is missing",
                    "specify a return parameter: try func() returns (uint256 result) { ... }",
                );
                instructions.push(Instruction::Drop(ValueType::Any));
            }
        } else if handler_params.len() > 1 {
            let array_tmp = ctx.allocate_local(
                "__try_ret_array".to_string(),
                Some(ValueType::Array(Box::new(ValueType::Any))),
            );
            instructions.push(Instruction::StoreLocal(array_tmp));

            for (i, param_tuple) in handler_params.iter().enumerate() {
                if let Some(param) = param_tuple.1.as_ref() {
                    let inferred_type =
                        infer_type_from_expression(&param.ty, ctx).unwrap_or(ValueType::Any);
                    let tmp = ctx.allocate_local(
                        format!("__try_ret_{i}"),
                        Some(inferred_type.clone()),
                    );

                    instructions.push(Instruction::LoadLocal(array_tmp));
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::from(i),
                    )));
                    instructions.push(Instruction::ArrayGet);
                    instructions.push(Instruction::StoreLocal(tmp));

                    try_return_slots.push((tmp, inferred_type));
                }
            }
        } else {
            instructions.push(Instruction::Drop(ValueType::Any));
        }
    } else if !handler_params.is_empty() {
        ctx.record_error("try returns(...) expects a return value");
    }

    instructions.push(Instruction::EndTry {
        target: success_label,
    });

    // Catch handler: bind or drop the thrown value, then execute the matching catch body.
    instructions.push(Instruction::Label(catch_label));
    ctx.enter_scope();

    if catches.len() == 1 && is_bare_catch_clause(&catches[0]) {
        // Preserve existing compact lowering for a lone `catch { ... }`.
        instructions.push(Instruction::Drop(ValueType::Any));
        let _ = lower_statement(catch_clause_statement(&catches[0]), ctx, instructions);
    } else {
        let catch_local = ctx.allocate_local("__catch_exception".to_string(), None);
        instructions.push(Instruction::StoreLocal(catch_local));

        let mut fallback_clause: Option<&solang_parser::pt::CatchClause> = None;

        for clause in catches {
            if is_bare_catch_clause(clause) {
                if fallback_clause.is_none() {
                    fallback_clause = Some(clause);
                }
                continue;
            }

            let next_clause_label = ctx.next_label();
            if let Some(guard_target) = catch_clause_guard_target(clause, ctx) {
                instructions.push(Instruction::LoadLocal(catch_local));
                instructions.push(Instruction::IsType {
                    target: guard_target,
                });
                instructions.push(Instruction::JumpIf {
                    target: next_clause_label,
                });
            }

            if let solang_parser::pt::CatchClause::Named(_, ident, _, _) = clause {
                if ident.name == "Panic" {
                    ctx.record_warning_with_suggestion(
                        "catch Panic(uint256) uses NeoVM stack-item integer checks; EVM panic code semantics do not fully apply on Neo N3.",
                        "Prefer catch (bytes memory reason) when you need portable exception handling across EVM and Neo.",
                    );
                }
            }

            ctx.enter_scope();
            bind_catch_clause_parameter(clause, catch_local, ctx, instructions);
            let _ = lower_statement(catch_clause_statement(clause), ctx, instructions);
            ctx.exit_scope();
            instructions.push(Instruction::Jump { target: end_label });
            instructions.push(Instruction::Label(next_clause_label));
        }

        if let Some(clause) = fallback_clause {
            ctx.enter_scope();
            let _ = lower_statement(catch_clause_statement(clause), ctx, instructions);
            ctx.exit_scope();
        } else {
            // No fallback clause and no type guard matched: rethrow the original exception.
            instructions.push(Instruction::LoadLocal(catch_local));
            instructions.push(Instruction::Throw);
        }
    }

    ctx.exit_scope();
    instructions.push(Instruction::EndTry { target: end_label });

    // Success handler: bind return values (if present) and run handler block.
    instructions.push(Instruction::Label(success_label));
    if let Some(success_stmt) = success_stmt {
        ctx.enter_scope();

        for (i, param_tuple) in handler_params.iter().enumerate() {
            if let Some(param) = param_tuple.1.as_ref() {
                if let Some(name) = param.name.as_ref().map(|id| id.name.clone()) {
                    let (tmp, inferred) = try_return_slots
                        .get(i)
                        .cloned()
                        .unwrap_or_else(|| (0, ValueType::Any));
                    let slot = ctx.allocate_local(name, Some(inferred.clone()));
                    if try_return_slots.get(i).is_some() {
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
