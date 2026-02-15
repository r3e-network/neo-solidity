pub fn analyse_source(source: &str) -> Result<ContractMetadata, SolidityError> {
    let mut contracts = analyse_all_sources(source)?;
    Ok(contracts.swap_remove(0))
}

pub fn analyse_all_sources(source: &str) -> Result<Vec<ContractMetadata>, SolidityError> {
    fn is_builtin_library_name(name: &str) -> bool {
        matches!(
            name,
            "Runtime" | "abi" | "Storage" | "Syscalls" | "Neo" | "NativeCalls"
        )
    }

    fn normalize_library_for_neo(mut contract: ContractIR) -> ContractIR {
        if !matches!(contract.kind, ContractKind::Library) {
            return contract;
        }

        // Neo N3 libraries are inlined into contracts; treat externally visible
        // library functions as internal helper functions to avoid exposing them
        // through the contract ABI.
        for function in &mut contract.functions {
            if !matches!(function.ty, FunctionTy::Function) {
                continue;
            }
            if matches!(
                function.visibility,
                VisibilityKind::External | VisibilityKind::Public
            ) {
                function.visibility = VisibilityKind::Internal;
            }
        }

        // Keep merged library state as internal implementation detail.
        // Public library constants would otherwise synthesize contract-level
        // getters and create ABI/name collisions in the consuming contract.
        for state in &mut contract.state_variables {
            state.visibility = Some("internal".to_string());
        }

        contract
    }

    fn collect_contract_types(
        contract_map: &std::collections::HashMap<String, ContractIR>,
    ) -> Vec<String> {
        let mut contract_types: Vec<String> = Vec::new();
        let mut seen_contract_types = std::collections::HashSet::new();

        for contract in contract_map.values() {
            let include_as_contract_type = match contract.kind {
                ContractKind::Contract | ContractKind::AbstractContract | ContractKind::Interface => {
                    true
                }
                ContractKind::Library => !is_builtin_library_name(contract.name.as_str()),
            };

            if include_as_contract_type
                && seen_contract_types.insert(contract.name.to_ascii_lowercase())
            {
                contract_types.push(contract.name.clone());
            }
        }

        contract_types
    }

    let mut primary = Vec::new();
    let mut fallback = Vec::new();

    for contract in parse_source(source)? {
        if matches!(
            contract.kind,
            ContractKind::Contract | ContractKind::AbstractContract
        ) {
            primary.push(contract);
        } else {
            fallback.push(contract);
        }
    }

    let has_primary = !primary.is_empty();
    let pre_merge_contract_map: std::collections::HashMap<String, ContractIR> = primary
        .iter()
        .chain(fallback.iter())
        .map(|contract| (contract.name.clone(), contract.clone()))
        .collect();
    let contract_types = collect_contract_types(&pre_merge_contract_map);

    let libraries: Vec<ContractIR> = if has_primary {
        fallback
            .iter()
            .filter(|contract| matches!(contract.kind, ContractKind::Library))
            // Built-in helper libraries (Runtime/Storage/Syscalls/Neo) are lowered directly during
            // IR generation. Avoid merging their Solidity bodies into user contracts since they
            // may contain EVM-only stubs or unsupported constructs, and they would bloat bytecode.
            .filter(|contract| !is_builtin_library_name(contract.name.as_str()))
            .cloned()
            .map(normalize_library_for_neo)
            .collect()
    } else {
        Vec::new()
    };

    // Validate user libraries before merging. Convert each library to metadata
    // and run the standard validation pipeline to catch library-specific errors
    // (state variables, constructors, external functions) early.
    for lib in &libraries {
        let lib_metadata = convert_contract(
            lib.clone(),
            &[],
            &contract_types,
            std::sync::Arc::new(SelectorRegistry::default()),
        );
        let lib_diagnostics = validate_contract(&lib_metadata);
        let lib_errors: Vec<Diagnostic> = lib_diagnostics
            .into_iter()
            .filter(|d| matches!(d.severity, DiagnosticSeverity::Error))
            .collect();
        if !lib_errors.is_empty() {
            let messages: Vec<String> = lib_errors.iter().map(|d| {
                let mut msg = d.message.clone();
                if let Some(suggestion) = &d.suggestion {
                    msg.push_str(&format!("\n  suggestion: {suggestion}"));
                }
                msg
            }).collect();
            return Err(SolidityError::analysis(messages.join("\n")));
        }
    }

    // Merge library definitions into primary contracts so that library functions
    // (including `using for`-style member calls) can be lowered as internal calls.
    if has_primary && !libraries.is_empty() {
        for contract in primary.iter_mut() {
            for lib in &libraries {
                contract.functions.extend(lib.functions.clone());
                contract.state_variables.extend(lib.state_variables.clone());
                contract.structs.extend(lib.structs.clone());
                contract.enums.extend(lib.enums.clone());
            }
        }
    }

    // Make non-inherited enum/struct namespaces visible across compilation
    // units so expressions like `Enum.Operation.DelegateCall` can resolve even
    // when the defining type lives in another top-level contract/library file.
    if has_primary {
        let shared_type_defs: Vec<(String, Vec<StructIR>, Vec<EnumIR>)> = pre_merge_contract_map
            .values()
            .filter(|contract| {
                !matches!(contract.kind, ContractKind::Library)
                    || !is_builtin_library_name(contract.name.as_str())
            })
            .map(|contract| {
                (
                    contract.name.clone(),
                    contract.structs.clone(),
                    contract.enums.clone(),
                )
            })
            .collect();

        for contract in primary.iter_mut() {
            let mut seen_structs: std::collections::HashSet<String> = contract
                .structs
                .iter()
                .map(|item| item.name.to_ascii_lowercase())
                .collect();
            let mut seen_enums: std::collections::HashSet<String> = contract
                .enums
                .iter()
                .map(|item| item.name.to_ascii_lowercase())
                .collect();

            for (owner_name, structs, enums) in &shared_type_defs {
                if owner_name == &contract.name {
                    continue;
                }
                for item in structs {
                    let key = item.name.to_ascii_lowercase();
                    if seen_structs.insert(key) {
                        contract.structs.push(item.clone());
                    }
                }
                for item in enums {
                    let key = item.name.to_ascii_lowercase();
                    if seen_enums.insert(key) {
                        contract.enums.push(item.clone());
                    }
                }
            }
        }
    }

    // Build a lookup map for inheritance flattening and modifier expansion.
    let contract_map: std::collections::HashMap<String, ContractIR> = primary
        .iter()
        .chain(fallback.iter())
        .map(|contract| (contract.name.clone(), contract.clone()))
        .collect();

    // Build a shared selector registry so `.selector` expressions can resolve against
    // any contract/interface visible to this compilation unit (including those defined
    // after the primary contract in the same file).
    let mut type_method_selectors: std::collections::HashMap<
        String,
        std::collections::HashMap<String, Vec<[u8; 4]>>,
    > = std::collections::HashMap::new();
    let mut interface_types: std::collections::HashSet<String> = std::collections::HashSet::new();
    for contract in contract_map.values() {
        if matches!(contract.kind, ContractKind::Interface) {
            interface_types.insert(contract.name.clone());
        }

        // When building selector lookups for `.selector` / `.interfaceId`, include inherited
        // interface methods as part of the derived interface. This matches Solidity behavior
        // and supports patterns like `type(IChild).interfaceId` where `IChild is IParent`.
        let selector_contract = match contract.kind {
            ContractKind::Contract | ContractKind::AbstractContract | ContractKind::Interface => {
                flatten_contract_inheritance(contract.clone(), &contract_map)
                    .map(|(ir, _warnings)| ir)
                    .unwrap_or_else(|_| contract.clone())
            }
            ContractKind::Library => contract.clone(),
        };

        let mut per_type: std::collections::HashMap<String, Vec<[u8; 4]>> =
            std::collections::HashMap::new();

        for function in &selector_contract.functions {
            if !matches!(function.ty, FunctionTy::Function) {
                continue;
            }

            if !matches!(
                function.visibility,
                VisibilityKind::External | VisibilityKind::Public
            ) {
                continue;
            }

            let param_signatures: Vec<String> = function
                .parameters
                .iter()
                .map(|param| canonical_param_type(&param.ty))
                .collect();
            let selector = compute_function_selector(&function.name, &param_signatures);
            per_type
                .entry(function.name.clone())
                .or_default()
                .push(selector);
        }

        type_method_selectors.insert(contract.name.clone(), per_type);
    }
    let selector_registry = std::sync::Arc::new(SelectorRegistry {
        type_method_selectors,
        interface_types,
    });

    let mut selected = if has_primary { primary } else { fallback };

    if selected.is_empty() {
        return Ok(Vec::new());
    }

    let mut metadatas = Vec::new();
    for contract in selected.drain(..) {
        let (mut flattened, flatten_warnings) =
            flatten_contract_inheritance(contract, &contract_map)?;
        apply_modifiers_and_base_constructors(&mut flattened, &contract_map)?;
        let mut metadata = convert_contract(
            flattened,
            &[],
            &contract_types,
            selector_registry.clone(),
        );
        metadata.flatten_warnings = flatten_warnings;
        metadatas.push(metadata);
    }

    Ok(metadatas)
}
