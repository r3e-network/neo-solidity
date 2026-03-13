pub(crate) fn build_standard_abi(metadata: &ContractMetadata) -> Vec<Value> {
    let mut abi_entries = Vec::new();

    for method in &metadata.methods {
        if matches!(method.kind, FunctionKind::Regular)
            && !matches!(
                method.visibility,
                VisibilityKind::Public | VisibilityKind::External
            )
        {
            continue;
        }
        let inputs: Vec<Value> = method
            .parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                json!({
                    "name": parameter.name.clone().unwrap_or_else(|| format!("arg{index}")),
                    "type": parameter.ty,
                    "internalType": parameter.ty,
                })
            })
            .collect();

        let outputs: Vec<Value> = method
            .return_parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                json!({
                    "name": parameter.name.clone().unwrap_or_else(|| format!("ret{index}")),
                    "type": parameter.ty,
                    "internalType": parameter.ty,
                })
            })
            .collect();

        match method.kind {
            FunctionKind::Constructor => {
                abi_entries.push(json!({
                    "type": "constructor",
                    "inputs": inputs,
                    "stateMutability": state_mutability_label(method.state_mutability),
                }));
            }
            FunctionKind::Regular => {
                abi_entries.push(json!({
                    "type": "function",
                    "name": method.name,
                    "inputs": inputs,
                    "outputs": outputs,
                    "stateMutability": state_mutability_label(method.state_mutability),
                }));
            }
        }
    }

    for event in &metadata.events {
        let inputs: Vec<Value> = event
            .parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                json!({
                    "name": parameter.name.clone().unwrap_or_else(|| format!("param{index}")),
                    "type": parameter.ty,
                    "indexed": parameter.indexed,
                })
            })
            .collect();

        abi_entries.push(json!({
            "type": "event",
            "name": event.name,
            "inputs": inputs,
            "anonymous": false,
        }));
    }

    abi_entries
}

pub(crate) fn build_method_identifiers(metadata: &ContractMetadata) -> Map<String, Value> {
    let mut identifiers = Map::new();

    for method in &metadata.methods {
        if matches!(method.kind, FunctionKind::Constructor) {
            continue;
        }

        if !matches!(
            method.visibility,
            VisibilityKind::Public | VisibilityKind::External
        ) {
            continue;
        }

        let param_signatures: Vec<String> = method
            .parameters
            .iter()
            .map(|param| canonical_param_type(&param.ty))
            .collect();
        let signature = if param_signatures.is_empty() {
            format!("{}()", method.name)
        } else {
            format!("{}({})", method.name, param_signatures.join(","))
        };

        identifiers.insert(
            signature,
            Value::String(hex_prefixed(&hex::encode(method.selector))),
        );
    }

    identifiers
}

pub(crate) fn build_neo_method_map(metadata: &ContractMetadata) -> Map<String, Value> {
    let mut map = Map::new();

    for method in &metadata.methods {
        if matches!(method.kind, FunctionKind::Constructor) {
            continue;
        }
        if !matches!(
            method.visibility,
            VisibilityKind::Public | VisibilityKind::External
        ) {
            continue;
        }

        let param_signature: Vec<String> = method
            .parameters
            .iter()
            .map(|param| canonical_param_type(&param.ty))
            .collect();
        let signature = format!("{}({})", method.name, param_signature.join(","));
        map.insert(signature, Value::String(method.neo_name.clone()));
    }

    map
}
