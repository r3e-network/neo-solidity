impl Module {
    pub fn from_contract(metadata: &ContractMetadata) -> Result<Self, Vec<IrDiagnostic>> {
        let state_variables: Vec<StateVariable> = metadata
            .state_variables
            .iter()
            .map(StateVariable::from_metadata)
            .collect();

        let state_index_map: HashMap<String, usize> = metadata
            .state_variables
            .iter()
            .enumerate()
            .filter_map(|(index, state)| state.name.as_ref().map(|name| (name.clone(), index)))
            .collect();

        let state_types: Vec<ValueType> = state_variables
            .iter()
            .map(|state| state.ty.clone())
            .collect();

        let events: Vec<Event> = metadata.events.iter().map(Event::from_metadata).collect();
        let event_index_map: HashMap<String, usize> = metadata
            .events
            .iter()
            .enumerate()
            .map(|(index, event)| (event.name.clone(), index))
            .collect();

        let event_signature_map: HashMap<String, Vec<ManifestType>> = metadata
            .events
            .iter()
            .map(|event| {
                let signature = event
                    .parameters
                    .iter()
                    .map(|param| manifest_type_from_solidity_type(&param.ty))
                    .collect::<Vec<_>>();
                (event.name.clone(), signature)
            })
            .collect();

        let enum_variant_map = build_enum_variant_map(&metadata.enums);
        let contract_types: HashSet<String> = metadata.contract_types.iter().cloned().collect();
        let selector_registry = metadata.selector_registry.as_ref();

        let function_names: HashSet<String> = metadata
            .methods
            .iter()
            .map(|method| method.name.clone())
            .collect();

        let void_functions: HashSet<String> = metadata
            .methods
            .iter()
            .filter(|method| method.return_parameters.is_empty())
            .map(|method| method.name.clone())
            .collect();

        let mut function_overloads = HashMap::new();
        let mut function_param_names: HashMap<(String, usize), Vec<String>> = HashMap::new();
        for method in &metadata.methods {
            function_overloads.insert(
                (method.name.clone(), method.parameters.len()),
                method.neo_name.clone(),
            );
            let param_names: Vec<String> = method
                .parameters
                .iter()
                .map(|p| p.name.clone().unwrap_or_default())
                .collect();
            function_param_names.insert(
                (method.name.clone(), method.parameters.len()),
                param_names,
            );
        }

        let defined_struct_types =
            build_defined_struct_types(&metadata.structs, &metadata.enums, &metadata.contract_types);

        let mut functions = Vec::new();
        let mut constructor_indices = Vec::new();
        let mut deploy_metadata: Option<&FunctionMetadata> = None;
        let mut errors = Vec::new();

        for method in &metadata.methods {
            if method.name == "_deploy" {
                deploy_metadata = Some(method);
                continue;
            }

            match Function::from_metadata(
                method,
                &metadata.state_variables,
                &state_index_map,
                &state_types,
                &defined_struct_types,
                &event_index_map,
                &event_signature_map,
                &enum_variant_map,
                &contract_types,
                selector_registry,
                &function_names,
                &function_overloads,
                &function_param_names,
                &void_functions,
                &metadata.super_method_map,
            ) {
                Ok(function) => {
                    if matches!(function.kind, FunctionKind::Constructor) {
                        constructor_indices.push(functions.len());
                    }
                    functions.push(function);
                }
                Err(mut errs) => errors.append(&mut errs),
            }
        }

        if let Some(deploy_metadata) = deploy_metadata {
            let constructors: Vec<&Function> = constructor_indices
                .iter()
                .map(|index| &functions[*index])
                .collect();

            match build_deploy_function(
                deploy_metadata,
                &constructors,
                &metadata.state_variables,
                &state_index_map,
                &state_types,
                &defined_struct_types,
                &event_index_map,
                &event_signature_map,
                &enum_variant_map,
                &contract_types,
                selector_registry,
                &function_names,
                &function_overloads,
                &function_param_names,
                &void_functions,
                &metadata.super_method_map,
            ) {
                Ok(function) => functions.push(function),
                Err(mut errs) => errors.append(&mut errs),
            }
        }

        let hazards = compute_transitive_hazard_map(&functions);
        errors.extend(validate_safe_methods(metadata, &hazards));
        errors.extend(validate_pure_methods(metadata, &hazards));

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Module {
            functions,
            state_variables,
            events,
        })
    }
}
