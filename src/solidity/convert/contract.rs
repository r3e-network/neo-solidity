fn convert_contract(
    contract: ContractIR,
    inherited_events: &[EventMetadata],
    contract_types: &[String],
    selector_registry: std::sync::Arc<SelectorRegistry>,
) -> ContractMetadata {
    let structs: Vec<StructMetadata> = contract.structs.into_iter().map(convert_struct).collect();
    let enums: Vec<EnumMetadata> = contract.enums.into_iter().map(convert_enum).collect();

    let struct_type_info = structs_to_type_metadata(&structs);
    let enum_type_info = enums_to_type_metadata(&enums);

    // If the contract already defines an explicit `onNEP17Payment`, do not
    // remap Solidity `receive()` into the Neo NEP-17 callback entrypoint.
    //
    // This keeps the contract compilable when authors include both `receive()`
    // (for Solidity source compatibility) and `onNEP17Payment` (for Neo).
    let has_explicit_on_nep17_payment = contract.functions.iter().any(|function| {
        !matches!(function.ty, FunctionTy::Receive) && function.name == "onNEP17Payment"
    });

    let mut methods: Vec<FunctionMetadata> = contract
        .functions
        .into_iter()
        .filter(|function| {
            matches!(
                function.ty,
                FunctionTy::Function
                    | FunctionTy::Constructor
                    | FunctionTy::Fallback
                    | FunctionTy::Receive
            )
        })
        .map(|function| {
            convert_function(
                function,
                &struct_type_info,
                &enum_type_info,
                contract_types,
                has_explicit_on_nep17_payment,
                &contract.type_aliases,
            )
        })
        .collect();

    // Mangle Neo entrypoint names for overloaded Solidity functions.
    // Neo ABI dispatches by method name and parameter count, so overloaded
    // functions must have unique Neo-visible names to avoid collisions during
    // code generation and manifest export. We preserve the original Solidity
    // name in `FunctionMetadata::name` for selector/ABI purposes.
    use std::collections::HashMap;
    let mut overloads: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, method) in methods.iter().enumerate() {
        if matches!(method.kind, FunctionKind::Regular) {
            overloads.entry(method.name.clone()).or_default().push(idx);
        }
    }
    for (name, indices) in overloads {
        if indices.len() > 1 {
            for index in indices {
                let method = &mut methods[index];
                let param_signatures: Vec<String> = method
                    .parameters
                    .iter()
                    .map(|param| canonical_param_type(&param.ty))
                    .collect();
                method.neo_name = if param_signatures.is_empty() {
                    format!("{name}()")
                } else {
                    format!("{name}({})", param_signatures.join(","))
                };
            }
        }
    }

    use std::collections::BTreeMap;
    let mut event_map: BTreeMap<String, EventMetadata> = BTreeMap::new();
    for event in inherited_events {
        event_map
            .entry(event.normalized_name.clone())
            .or_insert_with(|| event.clone());
    }
    for event in contract.events.into_iter().map(convert_event) {
        event_map.insert(event.normalized_name.clone(), event);
    }
    let events: Vec<EventMetadata> = event_map.into_values().collect();
    let state_variables: Vec<StateVariableMetadata> = contract
        .state_variables
        .into_iter()
        .map(|var| convert_state_variable(var, &struct_type_info, &enum_type_info, contract_types, &contract.type_aliases))
        .collect();

    // Synthesize public state variable getters to match Solidity ABI behavior.
    synthesize_public_getters(&mut methods, &state_variables);

    ContractMetadata {
        name: contract.name,
        is_abstract: matches!(contract.kind, ContractKind::AbstractContract),
        is_interface: matches!(contract.kind, ContractKind::Interface),
        is_library: matches!(contract.kind, ContractKind::Library),
        methods,
        events,
        uses_storage: state_variables.iter().any(|state| !state.is_constant),
        state_variables,
        structs,
        enums,
        contract_types: contract_types.to_vec(),
        selector_registry,
        documentation: contract.doc.into(),
        has_using_for_star: contract.has_using_for_star,
        has_using_function_list: contract.has_using_function_list,
        using_for_libraries: contract.using_for_libraries.clone(),
        using_directives: contract
            .using_directives
            .iter()
            .map(|directive| UsingDirectiveMetadata {
                target_type: directive.target_type.clone(),
                function_names: directive.function_names.clone(),
            })
            .collect(),
        has_type_definitions: contract.has_type_definitions,
        type_aliases: contract.type_aliases,
        flatten_warnings: Vec::new(),
        super_method_map: contract.super_method_map,
    }
}

fn synthesize_public_getters(
    methods: &mut Vec<FunctionMetadata>,
    state_variables: &[StateVariableMetadata],
) {
    for state in state_variables {
        if state
            .visibility
            .as_deref()
            .map(|v| v.eq_ignore_ascii_case("public"))
            != Some(true)
        {
            continue;
        }

        let name = match state.name.as_deref() {
            Some(name) => name.to_string(),
            None => continue,
        };

        let neotype = match state.neo_type.as_ref() {
            Some(neo) => neo.clone(),
            None => continue,
        };

        let (parameters, return_parameters, expr) = getter_signature_from_neotype(&name, &neotype);
        let param_signatures: Vec<String> = parameters
            .iter()
            .map(|param| canonical_param_type(&param.ty))
            .collect();
        let selector = compute_function_selector(&name, &param_signatures);

        methods.push(FunctionMetadata {
            name: name.clone(),
            neo_name: name,
            kind: FunctionKind::Regular,
            parameters,
            return_parameters,
            state_mutability: StateMutability::View,
            visibility: VisibilityKind::Public,
            offset: 0,
            body: Some(Statement::Return(Default::default(), Some(expr))),
            selector,
            is_virtual: false,
            is_override: false,
            documentation: NatspecDoc::default(),
        });
    }
}

fn convert_state_variable(
    var: StateVariableIR,
    struct_types: &[StructTypeMetadata],
    enum_types: &[EnumTypeMetadata],
    contract_types: &[String],
    type_aliases: &std::collections::HashMap<String, String>,
) -> StateVariableMetadata {
    let ty = var.ty;
    let neo_type = NeoType::from_solidity_with_aliases(&ty, struct_types, enum_types, contract_types, type_aliases).ok();
    let initializer = var.initializer;
    StateVariableMetadata {
        name: var.name,
        ty,
        is_constant: var.is_constant,
        is_immutable: var.is_immutable,
        visibility: var.visibility,
        neo_type,
        has_initializer: initializer.is_some(),
        initializer,
    }
}
