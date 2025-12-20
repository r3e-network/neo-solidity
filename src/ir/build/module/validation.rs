fn compute_transitive_hazard_map(functions: &[Function]) -> HashMap<String, Hazards> {
    let mut direct = HashMap::new();
    for function in functions {
        direct.insert(function.name.clone(), direct_hazards(function));
    }

    let graph = build_call_graph(functions);
    let mut memo = HashMap::new();
    let mut visiting = HashSet::new();

    for name in direct.keys() {
        compute_transitive_hazards(name, &direct, &graph, &mut memo, &mut visiting);
    }

    memo
}

fn validate_safe_methods(
    metadata: &ContractMetadata,
    hazards: &HashMap<String, Hazards>,
) -> Vec<String> {
    let mut errors = Vec::new();

    for method in &metadata.methods {
        if method.state_mutability != crate::solidity::StateMutability::View {
            continue;
        }

        let hazards = hazards
            .get(&method.neo_name)
            .copied()
            .unwrap_or_default();

        if !hazards.safe_violation() {
            continue;
        }

        if hazards.writes_state {
            errors.push(format!(
                "function '{}' declared view/pure but writes contract storage",
                method.neo_name
            ));
        }

        if hazards.notifies {
            errors.push(format!(
                "function '{}' declared view/pure but emits events/notifications",
                method.neo_name
            ));
        }

        if hazards.unsafe_contract_call {
            errors.push(format!(
                "function '{}' declared view/pure but performs non-readonly contract calls",
                method.neo_name
            ));
        }
    }

    errors
}

fn validate_pure_methods(
    metadata: &ContractMetadata,
    hazards: &HashMap<String, Hazards>,
) -> Vec<String> {
    let mut errors = Vec::new();

    for method in &metadata.methods {
        if method.state_mutability != crate::solidity::StateMutability::Pure {
            continue;
        }

        let hazards = hazards
            .get(&method.neo_name)
            .copied()
            .unwrap_or_default();

        if !hazards.pure_violation() {
            continue;
        }

        if hazards.writes_state {
            errors.push(format!(
                "function '{}' declared pure but writes contract storage",
                method.neo_name
            ));
        }

        if hazards.notifies {
            errors.push(format!(
                "function '{}' declared pure but emits events/notifications",
                method.neo_name
            ));
        }

        if hazards.reads_state {
            errors.push(format!(
                "function '{}' declared pure but reads contract storage",
                method.neo_name
            ));
        }

        if hazards.reads_environment {
            errors.push(format!(
                "function '{}' declared pure but reads the execution environment",
                method.neo_name
            ));
        }

        if hazards.contract_calls {
            let label = if hazards.unsafe_contract_call {
                "performs non-readonly contract calls"
            } else {
                "performs contract calls"
            };
            errors.push(format!(
                "function '{}' declared pure but {}",
                method.neo_name, label
            ));
        }
    }

    errors
}
