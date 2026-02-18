fn flatten_contract_inheritance(
    contract: ContractIR,
    contract_map: &std::collections::HashMap<String, ContractIR>,
) -> Result<(ContractIR, Vec<String>), SolidityError> {
    let order = contract_linearization_base_to_derived(&contract.name, contract_map)?;

    let mut functions: Vec<FunctionIR> = Vec::new();
    let mut function_index: std::collections::HashMap<(u8, String, Vec<String>), (String, usize)> =
        std::collections::HashMap::new();

    let mut events: Vec<EventIR> = Vec::new();
    let mut state_variables: Vec<StateVariableIR> = Vec::new();

    let mut structs: Vec<StructIR> = Vec::new();
    let mut struct_index: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    let mut enums: Vec<EnumIR> = Vec::new();
    let mut enum_index: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    let mut warnings: Vec<String> = Vec::new();

    // Merge user-defined value type aliases from the inheritance chain.
    let mut type_aliases: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    // Maps original method name → renamed super-method name for `super.method()` resolution.
    let mut super_method_map: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    for ancestor_name in &order {
        let Some(ancestor) = contract_map.get(ancestor_name) else {
            continue;
        };

        let ancestor_is_interface = matches!(ancestor.kind, ContractKind::Interface);

        // Preserve Solidity storage layout order: base state variables first, then derived.
        state_variables.extend(ancestor.state_variables.clone());

        // Merge user-defined value type aliases (base-first, derived wins on conflict).
        for (alias_name, underlying) in &ancestor.type_aliases {
            type_aliases
                .entry(alias_name.clone())
                .or_insert_with(|| underlying.clone());
        }

        for s in &ancestor.structs {
            if let Some(idx) = struct_index.get(&s.name).copied() {
                structs[idx] = s.clone();
            } else {
                struct_index.insert(s.name.clone(), structs.len());
                structs.push(s.clone());
            }
        }

        for e in &ancestor.enums {
            if let Some(idx) = enum_index.get(&e.name).copied() {
                enums[idx] = e.clone();
            } else {
                enum_index.insert(e.name.clone(), enums.len());
                enums.push(e.clone());
            }
        }

        // Events are additive in Solidity; duplicates are later de-duplicated by manifest builder.
        events.extend(ancestor.events.clone());

        for func in &ancestor.functions {
            // When flattening, keep only the most-derived constructor to avoid name collisions.
            if matches!(func.ty, FunctionTy::Constructor) && ancestor.name != contract.name {
                continue;
            }

            let key = (
                function_ty_key(func.ty),
                func.name.clone(),
                func.parameters
                    .iter()
                    .map(|param| crate::utils::canonical_param_type(&param.ty))
                    .collect::<Vec<_>>(),
            );
            match function_index.get(&key) {
                Some((origin, _)) if origin == &ancestor.name => {
                    // Duplicate definition within the same contract; preserve it so validation can
                    // emit a proper DUPLICATE_SIGNATURE diagnostic.
                    functions.push(func.clone());
                }
                Some((base_origin, idx)) => {
                    let idx = *idx;
                    let base_func = &functions[idx];
                    let base_origin = base_origin.clone();

                    // Determine if the base contract is an interface.
                    let base_is_interface = contract_map
                        .get(&base_origin)
                        .map(|c| matches!(c.kind, ContractKind::Interface))
                        .unwrap_or(false);

                    // Virtual/override enforcement:
                    // - Base function must be `virtual` (or from an interface, which is implicitly virtual)
                    // - Derived function must be marked `override`
                    if !base_is_interface && !base_func.is_virtual {
                        warnings.push(format!(
                            "function '{}' in '{}' overrides '{}::{}' which is not marked 'virtual'",
                            func.name, ancestor_name, base_origin, func.name
                        ));
                    }

                    if !ancestor_is_interface && !func.is_override {
                        warnings.push(format!(
                            "function '{}' in '{}' overrides a base function but is not marked 'override'",
                            func.name, ancestor_name
                        ));
                    }

                    // Preserve the base method as a renamed internal function so that
                    // `super.method()` can resolve to it during IR lowering.
                    // Only preserve if the base function has a body (skip interface stubs).
                    // In multi-level inheritance (A→B→C), only keep the most recent
                    // base version — replace any existing `__super_*` entry.
                    if base_func.body.is_some() {
                        let super_name = format!("__super_{}", func.name);
                        let mut super_func = base_func.clone();
                        super_func.name = super_name.clone();
                        super_func.visibility = VisibilityKind::Internal;
                        super_func.is_virtual = false;
                        super_func.is_override = false;

                        // Replace existing __super_ entry if present, otherwise append.
                        if let Some(existing_idx) = functions
                            .iter()
                            .position(|f| f.name == super_name)
                        {
                            functions[existing_idx] = super_func;
                        } else {
                            functions.push(super_func);
                        }
                        super_method_map.insert(func.name.clone(), super_name);
                    }

                    functions[idx] = func.clone();
                    function_index.insert(key, (ancestor.name.clone(), idx));
                }
                None => {
                    let idx = functions.len();
                    functions.push(func.clone());
                    function_index.insert(key, (ancestor.name.clone(), idx));
                }
            }
        }
    }

    // Merge interface events from the full inheritance tree.
    events.extend(collect_interface_events(&contract, contract_map));

    // Contracts may reference structs/enums declared on inherited interfaces, but interface types
    // are excluded from the storage linearization order. Merge them explicitly so type parsing
    // (NeoType inference) and IR lowering can recognize `Interface.StructName` and unqualified
    // `StructName` references used in contract bodies and ABI signatures.
    if matches!(
        contract.kind,
        ContractKind::Contract | ContractKind::AbstractContract
    ) {
        let (iface_structs, iface_enums) = collect_interface_types(&contract, contract_map);
        for s in iface_structs {
            if struct_index.contains_key(&s.name) {
                continue;
            }
            struct_index.insert(s.name.clone(), structs.len());
            structs.push(s);
        }
        for e in iface_enums {
            if enum_index.contains_key(&e.name) {
                continue;
            }
            enum_index.insert(e.name.clone(), enums.len());
            enums.push(e);
        }
    }

    Ok((ContractIR {
        name: contract.name,
        kind: contract.kind,
        bases: contract.bases,
        functions,
        events,
        state_variables,
        structs,
        enums,
        doc: contract.doc,
        has_using_for_star: contract.has_using_for_star,
        has_using_function_list: contract.has_using_function_list,
        using_for_libraries: contract.using_for_libraries,
        using_directives: contract.using_directives,
        has_type_definitions: contract.has_type_definitions,
        type_aliases,
        super_method_map,
    }, warnings))
}

fn inheritance_contract_chain(
    contract: &ContractIR,
    contract_map: &std::collections::HashMap<String, ContractIR>,
) -> Result<Vec<String>, SolidityError> {
    contract_linearization_base_to_derived(&contract.name, contract_map)
}
