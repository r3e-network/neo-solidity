fn convert_contract(
    contract: ContractDefinition,
    comment_map: &HashMap<usize, NatspecDocIR>,
) -> ContractIR {
    let name = contract
        .name
        .as_ref()
        .map(|id| id.name.clone())
        .unwrap_or_else(|| "Contract".to_string());

    let kind = match contract.ty {
        ContractTy::Abstract(_) => ContractKind::AbstractContract,
        ContractTy::Contract(_) => ContractKind::Contract,
        ContractTy::Interface(_) => ContractKind::Interface,
        ContractTy::Library(_) => ContractKind::Library,
    };

    // Extract contract-level documentation
    let doc = find_preceding_doc(&contract.loc, comment_map);

    let bases = contract.base;

    let mut functions = Vec::new();
    let mut events = Vec::new();
    let mut state_variables = Vec::new();
    let mut structs = Vec::new();
    let mut enums = Vec::new();
    let mut has_using_for_star = false;
    let mut has_using_function_list = false;
    let mut using_for_libraries: Vec<String> = Vec::new();
    let mut has_type_definitions = false;
    let mut type_aliases: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    for part in contract.parts.into_iter() {
        match part {
            ContractPart::FunctionDefinition(def) => {
                functions.push(convert_function(*def, comment_map))
            }
            ContractPart::EventDefinition(def) => events.push(convert_event(*def)),
            ContractPart::VariableDefinition(def) => {
                state_variables.push(convert_state_variable(*def))
            }
            ContractPart::StructDefinition(def) => structs.push(convert_struct(*def)),
            ContractPart::EnumDefinition(def) => enums.push(convert_enum(*def)),
            ContractPart::Using(using) => {
                // Collect the library name for diagnostics and dispatch tracking.
                if let UsingList::Library(ref path) = using.list {
                    let lib_name: String = path
                        .identifiers
                        .iter()
                        .map(|id| id.name.as_str())
                        .collect::<Vec<_>>()
                        .join(".");
                    if !using_for_libraries.contains(&lib_name) {
                        using_for_libraries.push(lib_name);
                    }
                }
                // Detect advanced `using` forms so we can emit targeted diagnostics.
                if using.ty.is_none() {
                    // `using X for *` — ty is None when the target is `*`
                    has_using_for_star = true;
                }
                if matches!(using.list, UsingList::Functions(_)) {
                    // `using { f, g } for Y`
                    has_using_function_list = true;
                }
            }
            ContractPart::TypeDefinition(td) => {
                has_type_definitions = true;
                let underlying = format!("{}", td.ty);
                type_aliases.insert(td.name.name.clone(), underlying);
            }
            _ => {}
        }
    }

    ContractIR {
        name,
        kind,
        bases,
        functions,
        events,
        state_variables,
        structs,
        enums,
        doc,
        has_using_for_star,
        has_using_function_list,
        using_for_libraries,
        has_type_definitions,
        type_aliases,
        super_method_map: std::collections::HashMap::new(),
    }
}

fn convert_function(
    function: FunctionDefinition,
    comment_map: &HashMap<usize, NatspecDocIR>,
) -> FunctionIR {
    let name = function_name(&function);
    let mutability = extract_mutability(&function);
    let visibility = extract_visibility(&function);
    let doc = find_preceding_doc(&function.loc, comment_map);

    let mut is_virtual = false;
    let mut is_override = false;
    let mut base_or_modifiers: Vec<Base> = Vec::new();

    for attr in &function.attributes {
        match attr {
            FunctionAttribute::Virtual(_) => is_virtual = true,
            FunctionAttribute::Override(_, _) => is_override = true,
            FunctionAttribute::BaseOrModifier(_, base) => base_or_modifiers.push(base.clone()),
            _ => {}
        }
    }

    let parameters = convert_parameters(&function.params);
    let returns = convert_parameters(&function.returns);
    FunctionIR {
        name,
        ty: function.ty,
        parameters,
        returns,
        mutability,
        visibility,
        is_virtual,
        is_override,
        base_or_modifiers,
        body: function.body,
        doc,
    }
}

fn function_name(function: &FunctionDefinition) -> String {
    match (&function.name, function.ty) {
        (Some(Identifier { name, .. }), _) => name.clone(),
        (None, FunctionTy::Constructor) => "constructor".to_string(),
        (None, FunctionTy::Fallback) => "fallback".to_string(),
        (None, FunctionTy::Receive) => "receive".to_string(),
        (None, FunctionTy::Modifier) => "modifier".to_string(),
        _ => "function".to_string(),
    }
}

fn extract_mutability(function: &FunctionDefinition) -> MutabilityKind {
    for attribute in &function.attributes {
        if let FunctionAttribute::Mutability(m) = attribute {
            return match m {
                Mutability::Pure(_) => MutabilityKind::Pure,
                Mutability::View(_) | Mutability::Constant(_) => MutabilityKind::View,
                Mutability::Payable(_) => MutabilityKind::Payable,
            };
        }
    }

    MutabilityKind::NonPayable
}

fn extract_visibility(function: &FunctionDefinition) -> VisibilityKind {
    for attribute in &function.attributes {
        if let FunctionAttribute::Visibility(visibility) = attribute {
            return match visibility {
                Visibility::External(_) => VisibilityKind::External,
                Visibility::Public(_) => VisibilityKind::Public,
                Visibility::Internal(_) => VisibilityKind::Internal,
                Visibility::Private(_) => VisibilityKind::Private,
            };
        }
    }

    VisibilityKind::Internal
}

fn convert_parameters(params: &ParameterList) -> Vec<ParameterIR> {
    params
        .iter()
        .filter_map(|(_, param)| param.as_ref())
        .map(|param| ParameterIR {
            name: param.name.as_ref().map(|id| id.name.clone()),
            ty: format!("{}", param.ty),
            storage: param.storage.as_ref().map(storage_to_string),
        })
        .collect()
}

fn storage_to_string(storage: &StorageLocation) -> String {
    match storage {
        StorageLocation::Memory(_) => "memory",
        StorageLocation::Storage(_) => "storage",
        StorageLocation::Calldata(_) => "calldata",
    }
    .to_string()
}

fn convert_event(event: EventDefinition) -> EventIR {
    let name = event
        .name
        .as_ref()
        .map(|id| id.name.clone())
        .unwrap_or_else(|| "event".to_string());

    let parameters = event
        .fields
        .into_iter()
        .map(|param| EventParameterIR {
            name: param.name.map(|id| id.name),
            ty: format!("{}", param.ty),
            indexed: param.indexed,
        })
        .collect();

    EventIR { name, parameters }
}

fn convert_state_variable(def: VariableDefinition) -> StateVariableIR {
    let name = def.name.map(|id| id.name);
    let ty = format!("{}", def.ty);
    let initializer = def.initializer.clone();

    let mut visibility = None;
    let mut is_constant = false;
    let mut is_immutable = false;

    for attr in def.attrs {
        match attr {
            VariableAttribute::Visibility(vis) => {
                visibility = Some(
                    match vis {
                        Visibility::External(_) => "external",
                        Visibility::Public(_) => "public",
                        Visibility::Internal(_) => "internal",
                        Visibility::Private(_) => "private",
                    }
                    .to_string(),
                );
            }
            VariableAttribute::Constant(_) => {
                is_constant = true;
            }
            VariableAttribute::Immutable(_) => {
                is_immutable = true;
            }
            _ => {}
        }
    }

    StateVariableIR {
        name,
        ty,
        is_constant,
        is_immutable,
        visibility,
        has_initializer: initializer.is_some(),
        initializer,
    }
}

fn convert_struct(def: StructDefinition) -> StructIR {
    let name = def
        .name
        .as_ref()
        .map(|id| id.name.clone())
        .unwrap_or_else(|| "Struct".to_string());

    let fields = def
        .fields
        .into_iter()
        .filter_map(|field| {
            let field_name = field.name.map(|id| id.name)?;
            Some(StructFieldIR {
                name: field_name,
                ty: format!("{}", field.ty),
            })
        })
        .collect();

    StructIR { name, fields }
}

fn convert_enum(def: EnumDefinition) -> EnumIR {
    let name = def
        .name
        .as_ref()
        .map(|id| id.name.clone())
        .unwrap_or_else(|| "Enum".to_string());

    let values = def
        .values
        .into_iter()
        .filter_map(|value| value.map(|id| id.name))
        .collect();

    EnumIR { name, values }
}
