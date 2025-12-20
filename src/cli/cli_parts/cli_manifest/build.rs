fn build_manifest(metadata: &ContractMetadata, ir_module: &ir::Module) -> serde_json::Value {
    fn neotype_to_manifest_type(neotype: Option<&NeoType>, solidity_type: &str) -> &'static str {
        match neotype {
            Some(NeoType::Integer { .. }) => "Integer",
            Some(NeoType::Boolean) => "Boolean",
            Some(NeoType::String) => "String",
            Some(NeoType::Address) => "Hash160",
            Some(NeoType::ByteArray { fixed_len: Some(32) }) => "Hash256",
            Some(NeoType::ByteArray { .. }) => "ByteArray",
            Some(NeoType::Array(_)) => "Array",
            Some(NeoType::Mapping { .. }) => "Map",
            Some(NeoType::Struct { .. }) => "Array",
            Some(NeoType::Any) | None => standard_json::solidity_to_manifest_type(solidity_type),
        }
    }

    let abi_methods: Vec<&FunctionMetadata> = metadata
        .methods
        .iter()
        .filter(|method| {
            !matches!(method.kind, FunctionKind::Constructor)
                && matches!(
                    method.visibility,
                    VisibilityKind::Public | VisibilityKind::External
                )
        })
        .collect();

    // Neo N3 dispatches methods by `name` and does not provide Solidity-style overload
    // resolution. To keep manifests compatible with NEP tooling while still supporting
    // overloaded functions, we emit:
    // - the Solidity-visible `name` for a single "primary" overload (picked by max arity)
    // - the mangled `neo_name` for all other overloads
    //
    // This guarantees unique manifest ABI names and preserves canonical names for common
    // standards (e.g., NEP-17 `transfer(from,to,amount,data)` vs a convenience overload).
    let mut overload_groups: HashMap<&str, Vec<&FunctionMetadata>> = HashMap::new();
    for method in &abi_methods {
        overload_groups
            .entry(method.name.as_str())
            .or_default()
            .push(*method);
    }

    let mut primary_overload: HashMap<&str, &FunctionMetadata> = HashMap::new();
    for (name, group) in &overload_groups {
        if group.len() <= 1 {
            continue;
        }

        let mut best = group[0];
        for candidate in group.iter().skip(1) {
            let best_params = best.parameters.len();
            let cand_params = candidate.parameters.len();
            if cand_params > best_params {
                best = candidate;
                continue;
            }
            if cand_params == best_params && candidate.neo_name < best.neo_name {
                best = candidate;
            }
        }
        primary_overload.insert(*name, best);
    }

    let methods_json: Vec<_> = abi_methods
        .iter()
        .map(|method| {
            let is_overloaded = overload_groups
                .get(method.name.as_str())
                .is_some_and(|group| group.len() > 1);

            let is_primary_overload = primary_overload
                .get(method.name.as_str())
                .is_some_and(|primary| primary.neo_name == method.neo_name)
                && is_overloaded;

            let abi_name = if !is_overloaded || is_primary_overload {
                method.name.clone()
            } else {
                method.neo_name.clone()
            };

            let params_json: Vec<_> = method
                .parameters
                .iter()
                .enumerate()
                .map(|(param_index, param)| {
                    json!({
                        "name": param
                            .name
                            .clone()
                            .unwrap_or_else(|| format!("arg{}", param_index)),
                        "type": neotype_to_manifest_type(param.neo_type.as_ref(), &param.ty),
                    })
                })
                .collect();

            json!({
                "name": abi_name,
                "offset": method.offset,
                "parameters": params_json,
                "returntype": if method.return_parameters.len() > 1 {
                    "Array"
                } else {
                    method
                        .return_parameters
                        .first()
                        .map(|param| neotype_to_manifest_type(param.neo_type.as_ref(), &param.ty))
                        .unwrap_or("Void")
                },
                "safe": method.state_mutability.is_safe(),
            })
        })
        .collect();

    let events_json: Vec<_> = metadata
        .events
        .iter()
        .map(|event| {
            let params: Vec<_> = event
                .parameters
                .iter()
                .enumerate()
                .map(|(idx, param)| {
                    json!({
                        "name": param
                            .name
                            .clone()
                            .unwrap_or_else(|| format!("param{}", idx)),
                        "type": standard_json::solidity_to_manifest_type(&param.ty),
                    })
                })
                .collect();

            json!({
                "name": event.name,
                "parameters": params,
            })
        })
        .collect();

    let supported_standards = detect_supported_standards(&metadata.methods);
    let permissions = infer_permissions(metadata, ir_module);
    // Neo N3 keeps `features` reserved for future use; Neo's manifest parser will
    // reject any populated keys. Keep the object empty for chain compatibility.
    let features = serde_json::Map::new();

    json!({
        "name": metadata.name,
        "groups": [],
        "features": features,
        "supportedstandards": supported_standards,
        "abi": {
            "methods": methods_json,
            "events": events_json,
        },
        "permissions": permissions,
        "trusts": [],
        "extra": {
            "Author": COMPILER_EMAIL,
            "Description": format!("Solidity contract '{}' compiled to NeoVM", metadata.name),
            "Version": compiler_version_string_4(),
            "Compiler": COMPILER_ID,
        }
    })
}
