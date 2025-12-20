fn flatten_contract_inheritance(
    contract: ContractIR,
    contract_map: &std::collections::HashMap<String, ContractIR>,
) -> Result<ContractIR, SolidityError> {
    let order = contract_linearization_base_to_derived(&contract.name, contract_map)?;

    let mut functions: Vec<FunctionIR> = Vec::new();
    let mut function_index: std::collections::HashMap<(u8, String, usize), (String, usize)> =
        std::collections::HashMap::new();

    let mut events: Vec<EventIR> = Vec::new();
    let mut state_variables: Vec<StateVariableIR> = Vec::new();

    let mut structs: Vec<StructIR> = Vec::new();
    let mut struct_index: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    let mut enums: Vec<EnumIR> = Vec::new();
    let mut enum_index: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for ancestor_name in &order {
        let Some(ancestor) = contract_map.get(ancestor_name) else {
            continue;
        };

        // Preserve Solidity storage layout order: base state variables first, then derived.
        state_variables.extend(ancestor.state_variables.clone());

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

            let key = (function_ty_key(func.ty), func.name.clone(), func.parameters.len());
            match function_index.get(&key) {
                Some((origin, _)) if origin == &ancestor.name => {
                    // Duplicate definition within the same contract; preserve it so validation can
                    // emit a proper DUPLICATE_SIGNATURE diagnostic.
                    functions.push(func.clone());
                }
                Some((_origin, idx)) => {
                    let idx = *idx;
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

    Ok(ContractIR {
        name: contract.name,
        kind: contract.kind,
        bases: contract.bases,
        functions,
        events,
        state_variables,
        structs,
        enums,
        doc: contract.doc,
    })
}

fn inheritance_contract_chain(
    contract: &ContractIR,
    contract_map: &std::collections::HashMap<String, ContractIR>,
) -> Result<Vec<String>, SolidityError> {
    contract_linearization_base_to_derived(&contract.name, contract_map)
}
