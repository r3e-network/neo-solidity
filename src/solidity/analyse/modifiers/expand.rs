fn build_modifier_definition_map(
    contract: &ContractIR,
) -> std::collections::HashMap<(String, usize), FunctionIR> {
    let mut map: std::collections::HashMap<(String, usize), FunctionIR> =
        std::collections::HashMap::new();
    for func in &contract.functions {
        if matches!(func.ty, FunctionTy::Modifier) {
            let key = (func.name.clone(), func.parameters.len());
            match map.get(&key) {
                Some(existing) => {
                    // Prefer concrete modifier implementations over stubs/bodyless declarations.
                    if existing.body.is_none() && func.body.is_some() {
                        map.insert(key, func.clone());
                    }
                }
                None => {
                    map.insert(key, func.clone());
                }
            }
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
            // Compatibility fallback for abstract/bodyless modifier declarations:
            // treat as a no-op wrapper when no concrete body is available.
            continue;
        };

        let mut normalized_args = arg_list.clone();
        if normalized_args.len() < modifier_def.parameters.len() {
            let missing = modifier_def.parameters.len() - normalized_args.len();
            for _ in 0..missing {
                normalized_args.push(Expression::NumberLiteral(
                    Default::default(),
                    "0".to_string(),
                    "".to_string(),
                    None,
                ));
            }
        } else if normalized_args.len() > modifier_def.parameters.len() {
            normalized_args.truncate(modifier_def.parameters.len());
        }

        let substitutions = build_parameter_substitutions(&modifier_def.parameters, &normalized_args)?;
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
