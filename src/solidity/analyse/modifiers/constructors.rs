fn apply_base_constructors_and_modifiers(
    contract: &ContractIR,
    constructor: &FunctionIR,
    modifier_defs: &std::collections::HashMap<(String, usize), FunctionIR>,
    contract_map: &std::collections::HashMap<String, ContractIR>,
) -> Result<Statement, SolidityError> {
    let chain = inheritance_contract_chain(contract, contract_map)?;
    let base_contracts: std::collections::HashSet<String> = chain
        .iter()
        .filter(|name| *name != &contract.name)
        .cloned()
        .collect();

    fn find_contract_constructor(contract: &ContractIR) -> Option<&FunctionIR> {
        contract
            .functions
            .iter()
            .find(|func| matches!(func.ty, FunctionTy::Constructor))
    }

    fn args_from_base_spec(base: &Base) -> Vec<Expression> {
        base.args.clone().unwrap_or_default()
    }

    fn find_base_args_in_invocations(invocations: &[Base], base_name: &str) -> Option<Vec<Expression>> {
        invocations
            .iter()
            .find(|b| base_last_name(b).as_deref() == Some(base_name))
            .map(args_from_base_spec)
    }

    fn find_base_args_in_bases(bases: &[Base], base_name: &str) -> Option<Vec<Expression>> {
        bases
            .iter()
            .find(|b| base_last_name(b).as_deref() == Some(base_name))
            .map(args_from_base_spec)
    }

    fn contract_directly_inherits(contract: &ContractIR, base_name: &str) -> bool {
        contract
            .bases
            .iter()
            .any(|b| base_last_name(b).as_deref() == Some(base_name))
    }

    fn resolve_base_constructor_args(
        base_name: &str,
        contract: &ContractIR,
        constructor: &FunctionIR,
        chain: &[String],
        contract_map: &std::collections::HashMap<String, ContractIR>,
    ) -> Result<Vec<Expression>, SolidityError> {
        // 1) Explicit base invocation in the most-derived constructor.
        if let Some(args) = find_base_args_in_invocations(&constructor.base_or_modifiers, base_name) {
            return Ok(args);
        }
        // 2) Base arguments on the most-derived contract inheritance specifier.
        if let Some(args) = find_base_args_in_bases(&contract.bases, base_name) {
            return Ok(args);
        }

        // 3) Walk down the linearized chain to find the contract that directly inherits this base,
        // then use its constructor invocation / inheritance specifier.
        let base_pos = chain
            .iter()
            .position(|name| name == base_name)
            .ok_or_else(|| {
                SolidityError::Analysis(format!(
                    "internal error: base '{base_name}' missing from linearization"
                ))
            })?;

        let mut candidates: Vec<Vec<Expression>> = Vec::new();
        for name in chain.iter().skip(base_pos + 1) {
            let Some(child) = contract_map.get(name) else {
                continue;
            };
            if !contract_directly_inherits(child, base_name) {
                continue;
            }

            if let Some(child_ctor) = find_contract_constructor(child) {
                if let Some(args) =
                    find_base_args_in_invocations(&child_ctor.base_or_modifiers, base_name)
                {
                    candidates.push(args);
                    continue;
                }
            }
            if let Some(args) = find_base_args_in_bases(&child.bases, base_name) {
                candidates.push(args);
            }
        }

        if candidates.is_empty() {
            return Ok(Vec::new());
        }

        // Detect conflicts (e.g., diamond inheritance specifying different constructor args).
        let first = candidates[0].clone();
        for other in candidates.iter().skip(1) {
            if *other != first {
                return Err(SolidityError::Analysis(format!(
                    "conflicting constructor arguments specified for base contract '{base_name}'"
                )));
            }
        }

        Ok(first)
    }

    // Base constructor prologue (runs before constructor modifiers).
    let mut prologue: Vec<Statement> = Vec::new();
    for base_name in chain
        .iter()
        .filter(|name| *name != &contract.name)
        .cloned()
        .collect::<Vec<_>>()
    {
        let Some(base_contract) = contract_map.get(&base_name) else {
            continue;
        };
        let Some(base_ctor) = find_contract_constructor(base_contract) else {
            continue;
        };
        let Some(base_body) = base_ctor.body.as_ref() else {
            continue;
        };

        let args = resolve_base_constructor_args(
            base_name.as_str(),
            contract,
            constructor,
            &chain,
            contract_map,
        )?;

        // Apply any modifiers used on the base constructor itself before substituting parameters.
        let base_ctor_modifiers: Vec<Base> = base_ctor
            .base_or_modifiers
            .iter()
            .filter(|b| {
                let Some(name) = base_last_name(b) else {
                    return false;
                };
                let arg_count = b.args.as_ref().map(|args| args.len()).unwrap_or(0);
                modifier_defs.contains_key(&(name, arg_count))
            })
            .cloned()
            .collect();

        let base_wrapped = if base_ctor_modifiers.is_empty() {
            base_body.clone()
        } else {
            apply_modifier_calls_to_body(base_body, &base_ctor_modifiers, modifier_defs)?
        };

        let substitutions = build_parameter_substitutions(&base_ctor.parameters, &args)?;
        let rewritten = rewrite_statement(&base_wrapped, &substitutions, None);
        prologue.extend(statement_list_from_body(&rewritten));
    }

    // Constructor modifiers exclude base constructor invocations.
    let constructor_modifiers: Vec<Base> = constructor
        .base_or_modifiers
        .iter()
        .filter(|b| {
            let name = base_last_name(b);
            match name {
                Some(n) => !base_contracts.contains(&n),
                None => true,
            }
        })
        .cloned()
        .collect();

    let Some(body) = constructor.body.as_ref() else {
        return Ok(Statement::Block {
            loc: Loc::Implicit,
            unchecked: false,
            statements: prologue,
        });
    };

    let wrapped = apply_modifier_calls_to_body(body, &constructor_modifiers, modifier_defs)?;
    let mut statements = prologue;
    match wrapped {
        Statement::Block { statements: inner, .. } => statements.extend(inner),
        other => statements.push(other),
    }

    Ok(Statement::Block {
        loc: Loc::Implicit,
        unchecked: false,
        statements,
    })
}
