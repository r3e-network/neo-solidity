fn build_parameter_substitutions(
    params: &[ParameterIR],
    args: &[Expression],
) -> Result<std::collections::HashMap<String, Expression>, SolidityError> {
    if params.len() != args.len() {
        return Err(SolidityError::Analysis(format!(
            "modifier/constructor argument mismatch: expected {}, got {}",
            params.len(),
            args.len()
        )));
    }

    let mut map = std::collections::HashMap::new();
    for (param, arg) in params.iter().zip(args.iter()) {
        let Some(name) = param.name.as_ref() else {
            return Err(SolidityError::Analysis(
                "modifier/constructor parameter is missing an identifier".to_string(),
            ));
        };
        map.insert(name.clone(), arg.clone());
    }
    Ok(map)
}
