fn build_modifier_definition_map(
    contract: &ContractIR,
) -> std::collections::HashMap<(String, usize), FunctionIR> {
    let mut map = std::collections::HashMap::new();
    for func in &contract.functions {
        if matches!(func.ty, FunctionTy::Modifier) {
            map.insert((func.name.clone(), func.parameters.len()), func.clone());
        }
    }
    map
}

fn apply_modifier_calls_to_body(
    original_body: &Statement,
    modifier_calls: &[Base],
    modifier_defs: &std::collections::HashMap<(String, usize), FunctionIR>,
) -> Result<Statement, SolidityError> {
    let mut current = original_body.clone();

    for modifier_call in modifier_calls.iter().rev() {
        let Some(name) = base_last_name(modifier_call) else {
            continue;
        };
        let arg_list: Vec<Expression> = modifier_call.args.clone().unwrap_or_default();
        let key = (name.clone(), arg_list.len());
        let Some(modifier_def) = modifier_defs.get(&key) else {
            return Err(SolidityError::Analysis(format!(
                "unresolved modifier '{name}' with {} argument(s)",
                arg_list.len()
            )));
        };

        let Some(modifier_body) = modifier_def.body.as_ref() else {
            return Err(SolidityError::Analysis(format!(
                "modifier '{name}' has no body"
            )));
        };

        let substitutions = build_parameter_substitutions(&modifier_def.parameters, &arg_list)?;
        current = rewrite_statement(modifier_body, &substitutions, Some(&current));
    }

    Ok(current)
}

fn statement_list_from_body(body: &Statement) -> Vec<Statement> {
    match body {
        Statement::Block { statements, .. } => statements.clone(),
        stmt => vec![stmt.clone()],
    }
}
