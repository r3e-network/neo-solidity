fn lower_emit(expr: &Expression, ctx: &mut LoweringContext, instructions: &mut Vec<Instruction>) {
    if let Expression::FunctionCall(_, func, args) = expr {
        if let Expression::Variable(identifier) = func.as_ref() {
            let original_len = instructions.len();
            instructions.push(Instruction::PushLiteral(LiteralValue::String(
                identifier.name.as_bytes().to_vec(),
            )));
            let mut success = true;
            for arg in args {
                if !lower_expression(arg, ctx, instructions) {
                    success = false;
                }
            }

            if success {
                if let Some(index) = ctx.event_index_map.get(&identifier.name) {
                    instructions.push(Instruction::EmitEvent {
                        event_index: *index,
                        arg_count: args.len(),
                    });
                } else {
                    instructions.push(Instruction::EmitEventByName {
                        name: identifier.name.clone(),
                        arg_count: args.len(),
                    });
                }
                return;
            }

            instructions.truncate(original_len);
        }
    }
}
