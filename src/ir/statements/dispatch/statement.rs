fn lower_statement(
    statement: &Statement,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    match statement {
        // NeoVM uses BigInteger arithmetic — no overflow is possible — so
        // `unchecked { }` blocks are semantically identical to normal blocks.
        // The `..` pattern safely ignores the `unchecked: bool` field.
        Statement::Block { statements, .. } => {
            lower_block_statement(statements, ctx, instructions)
        }
        Statement::If(_, condition, then_stmt, else_stmt) => {
            lower_if_statement(condition, then_stmt, else_stmt.as_deref(), ctx, instructions)
        }
        Statement::While(_, condition, body) => {
            lower_while_statement(condition, body, ctx, instructions)
        }
        Statement::DoWhile(_, body, condition) => {
            lower_do_while_statement(body, condition, ctx, instructions)
        }
        Statement::For(_, init, condition, post, body) => lower_for_statement(
            init.as_deref(),
            condition.as_deref(),
            post.as_deref(),
            body.as_deref(),
            ctx,
            instructions,
        ),
        Statement::Expression(_, expr) => lower_expression_statement(expr, ctx, instructions),
        Statement::VariableDefinition(_, decl, init) => {
            lower_variable_definition_statement(decl, init.as_ref(), ctx, instructions)
        }
        Statement::Emit(_, call) => lower_emit_statement(call, ctx, instructions),
        Statement::Assembly { .. } => lower_assembly_statement(ctx, instructions),
        Statement::Try(_, expr, handler, catches) => {
            lower_try_statement(expr, handler, catches, ctx, instructions)
        }
        Statement::Break(_) => lower_break_statement(ctx, instructions),
        Statement::Continue(_) => lower_continue_statement(ctx, instructions),
        Statement::Return(_, expr) => lower_return_statement(expr.as_ref(), ctx, instructions),
        Statement::Revert(_, ident, args) => {
            lower_revert_statement(ident.as_ref(), args, ctx, instructions)
        }
        Statement::RevertNamedArgs(_, ident, args) => {
            lower_revert_named_args(ident.as_ref(), args, ctx, instructions)
        }
        _ => {
            ctx.record_error_with_suggestion(
                format!("unsupported statement '{:?}'", statement),
                "this Solidity statement type is not yet supported by the Neo N3 compiler",
            );
            false
        }
    }
}
