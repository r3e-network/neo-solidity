impl Module {
    pub fn from_contract_with_warnings(
        metadata: &ContractMetadata,
    ) -> Result<(Self, Vec<crate::solidity::Diagnostic>), Vec<IrDiagnostic>> {
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

        // Build the EVM-canonical event-signature map — one `EventSignature`
        // per declared event, with the keccak-256 of the canonical signature
        // string precomputed as `topic[0]`. The `lower_emit` statement reads
        // this map to emit the EVM-spec log shape:
        //   topic[0] = keccak256("Name(type1,type2,...)")
        //   topic[1..N] = each indexed arg (32-byte BE or keccak256(value))
        //   data = abi.encode(non_indexed_args)
        let event_params_map: HashMap<String, EventSignature> = metadata
            .events
            .iter()
            .map(|event| {
                let canonical = event_canonical_signature(event);
                let mut hasher = sha3::Keccak256::new();
                hasher.update(canonical.as_bytes());
                let digest = hasher.finalize();
                let mut topic0 = [0u8; 32];
                topic0.copy_from_slice(&digest);
                let params = event
                    .parameters
                    .iter()
                    .map(|p| {
                        let canonical_type = event_canonical_param_type(&p.ty);
                        let is_dynamic = event_canonical_type_is_dynamic(&canonical_type);
                        EventParamInfo {
                            canonical_type,
                            indexed: p.indexed,
                            is_dynamic,
                        }
                    })
                    .collect();
                (
                    event.name.clone(),
                    EventSignature {
                        canonical,
                        topic0,
                        params,
                    },
                )
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
        let mut function_first_param_types: HashMap<(String, usize), ValueType> = HashMap::new();
        let mut function_param_names: HashMap<(String, usize), Vec<String>> = HashMap::new();
        for method in &metadata.methods {
            let key = (method.name.clone(), method.parameters.len());
            function_overloads.insert(key.clone(), method.neo_name.clone());
            if let Some(first_param) = method.parameters.first() {
                function_first_param_types
                    .insert(key.clone(), ValueType::from_parameter(first_param));
            }
            let param_names: Vec<String> = method
                .parameters
                .iter()
                .map(|p| p.name.clone().unwrap_or_default())
                .collect();
            function_param_names.insert(key, param_names);
        }

        let using_target_types: Vec<Option<String>> = metadata
            .using_directives
            .iter()
            .map(|directive| directive.target_type.clone())
            .collect();

        let mut using_function_list_targets: HashMap<String, Vec<Option<String>>> = HashMap::new();
        let mut using_function_list_scope_targets: Vec<Option<String>> = Vec::new();
        for directive in &metadata.using_directives {
            let Some(function_names) = &directive.function_names else {
                continue;
            };
            if !using_function_list_scope_targets.contains(&directive.target_type) {
                using_function_list_scope_targets.push(directive.target_type.clone());
            }
            for function_name in function_names {
                let targets = using_function_list_targets
                    .entry(function_name.to_ascii_lowercase())
                    .or_default();
                if !targets.contains(&directive.target_type) {
                    targets.push(directive.target_type.clone());
                }
            }
        }

        let defined_struct_types = build_defined_struct_types(
            &metadata.structs,
            &metadata.enums,
            &metadata.contract_types,
        );

        // Task #91 — collect inlinable bodies for methods whose first
        // parameter is `T storage` (a Solidity storage pointer). These are
        // typically library functions merged from `using L for L.Data`;
        // member-style calls against a struct state variable cannot pass
        // the receiver by value, so `member_calls.rs` inlines the body.
        let library_storage_bodies: HashMap<(String, usize), LibraryStorageBody> = metadata
            .methods
            .iter()
            .filter_map(|method| {
                let first_param = method.parameters.first()?;
                let is_storage_pointer =
                    first_param.storage.as_deref() == Some("storage");
                let first_ty =
                    first_param.neo_type.as_ref().map(ValueType::from_neotype)?;
                if !is_storage_pointer || !matches!(first_ty, ValueType::Struct { .. }) {
                    return None;
                }
                let body = method.body.clone()?;
                let param_names: Vec<String> = method
                    .parameters
                    .iter()
                    .map(|p| p.name.clone().unwrap_or_default())
                    .collect();
                if param_names.iter().any(|n| n.is_empty()) {
                    return None;
                }
                let value_param_types: Vec<Option<ValueType>> = method
                    .parameters
                    .iter()
                    .skip(1)
                    .map(|p| p.neo_type.as_ref().map(ValueType::from_neotype))
                    .collect();
                let return_type = method
                    .return_parameters
                    .first()
                    .and_then(|p| p.neo_type.as_ref().map(ValueType::from_neotype));
                Some((
                    (method.name.clone(), method.parameters.len()),
                    LibraryStorageBody { param_names, value_param_types, body, return_type },
                ))
            })
            .collect();

        let mut functions = Vec::new();
        let mut constructor_indices = Vec::new();
        let mut deploy_metadata: Option<&FunctionMetadata> = None;
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        for method in metadata.methods.iter() {
            if method.name == "_deploy" {
                deploy_metadata = Some(method);
                continue;
            }

            match Function::from_metadata_with_warnings(
                method,
                &metadata.name,
                &metadata.state_variables,
                &state_index_map,
                &state_types,
                &defined_struct_types,
                &event_index_map,
                &event_signature_map,
                &event_params_map,
                &enum_variant_map,
                &contract_types,
                selector_registry,
                &function_names,
                &function_overloads,
                &function_first_param_types,
                &using_target_types,
                &using_function_list_targets,
                &using_function_list_scope_targets,
                &function_param_names,
                &void_functions,
                &metadata.super_method_map,
                &library_storage_bodies,
            ) {
                Ok((function, mut function_warnings)) => {
                    if matches!(function.kind, FunctionKind::Constructor) {
                        constructor_indices.push(functions.len());
                    }
                    functions.push(function);
                    warnings.append(&mut function_warnings);
                }
                Err(mut errs) => errors.append(&mut errs),
            }
        }

        if let Some(deploy_metadata) = deploy_metadata {
            let constructors: Vec<&Function> = constructor_indices
                .iter()
                .map(|index| &functions[*index])
                .collect();

            match build_deploy_function_with_warnings(
                deploy_metadata,
                &metadata.name,
                &constructors,
                &metadata.state_variables,
                &state_index_map,
                &state_types,
                &defined_struct_types,
                &event_index_map,
                &event_signature_map,
                &event_params_map,
                &enum_variant_map,
                &contract_types,
                selector_registry,
                &function_names,
                &function_overloads,
                &function_first_param_types,
                &using_target_types,
                &using_function_list_targets,
                &using_function_list_scope_targets,
                &function_param_names,
                &void_functions,
                &metadata.super_method_map,
                &library_storage_bodies,
            ) {
                Ok((function, mut deploy_warnings)) => {
                    functions.push(function);
                    warnings.append(&mut deploy_warnings);
                }
                Err(mut errs) => errors.append(&mut errs),
            }
        }

        let hazards = compute_transitive_hazard_map(&functions);
        errors.extend(validate_safe_methods(metadata, &hazards));
        errors.extend(validate_pure_methods(metadata, &hazards));

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok((
            Module {
                functions,
                state_variables,
                events,
            },
            warnings,
        ))
    }

    pub fn from_contract(metadata: &ContractMetadata) -> Result<Self, Vec<IrDiagnostic>> {
        Self::from_contract_with_warnings(metadata).map(|(module, _warnings)| module)
    }
}
