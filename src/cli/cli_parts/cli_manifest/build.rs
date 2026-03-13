fn parse_json_value(raw: &str, tag: &str) -> Option<Value> {
    match serde_json::from_str::<Value>(raw) {
        Ok(value) => Some(value),
        Err(err) => {
            eprintln!("warning: ignoring @custom:{tag} because its value is not valid JSON: {err}");
            None
        }
    }
}

fn parse_json_or_string(raw: &str) -> Value {
    serde_json::from_str::<Value>(raw).unwrap_or_else(|_| Value::String(raw.to_string()))
}

fn upsert_manifest_extra(manifest: &mut Value) -> Option<&mut serde_json::Map<String, Value>> {
    let object = manifest.as_object_mut()?;
    let entry = object
        .entry("extra".to_string())
        .or_insert_with(|| Value::Object(serde_json::Map::new()));

    if !entry.is_object() {
        *entry = Value::Object(serde_json::Map::new());
    }

    entry.as_object_mut()
}

fn apply_manifest_custom_overrides(manifest: &mut Value, metadata: &ContractMetadata) {
    for (tag, raw_value) in &metadata.documentation.custom {
        let raw = raw_value.trim();
        if raw.is_empty() {
            continue;
        }

        let Some(field) = tag
            .strip_prefix("neo.manifest.")
            .or_else(|| tag.strip_prefix("manifest."))
        else {
            continue;
        };

        match field {
            "name" => {
                let parsed = parse_json_or_string(raw);
                let name = parsed
                    .as_str()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| raw.to_string());
                manifest["name"] = Value::String(name);
            }
            "groups" => {
                let Some(value) = parse_json_value(raw, tag) else {
                    continue;
                };
                if value.is_array() {
                    manifest["groups"] = value;
                } else {
                    eprintln!(
                        "warning: ignoring @custom:{tag}; expected a JSON array for manifest.groups"
                    );
                }
            }
            "features" => {
                let Some(value) = parse_json_value(raw, tag) else {
                    continue;
                };
                if !value.is_object() {
                    eprintln!(
                        "warning: ignoring @custom:{tag}; expected a JSON object for manifest.features"
                    );
                } else if value
                    .as_object()
                    .is_some_and(|features| features.is_empty())
                {
                    manifest["features"] = value;
                } else {
                    eprintln!(
                        "warning: ignoring @custom:{tag}; Neo N3 requires manifest.features to be an empty object"
                    );
                }
            }
            "supportedstandards" => {
                let Some(value) = parse_json_value(raw, tag) else {
                    continue;
                };
                if value.is_array() {
                    manifest["supportedstandards"] = value;
                } else {
                    eprintln!(
                        "warning: ignoring @custom:{tag}; expected a JSON array for manifest.supportedstandards"
                    );
                }
            }
            "trusts" => {
                let Some(value) = parse_json_value(raw, tag) else {
                    continue;
                };
                if value.is_array() || value.is_string() {
                    manifest["trusts"] = value;
                } else {
                    eprintln!(
                        "warning: ignoring @custom:{tag}; expected a JSON array or string for manifest.trusts"
                    );
                }
            }
            _ => {
                if let Some(extra_key) = field.strip_prefix("extra.") {
                    if extra_key.is_empty() {
                        continue;
                    }
                    if let Some(extra) = upsert_manifest_extra(manifest) {
                        extra.insert(extra_key.to_string(), parse_json_or_string(raw));
                    }
                }
            }
        }
    }
}

fn build_manifest(metadata: &ContractMetadata, ir_module: &ir::Module) -> serde_json::Value {
    fn neotype_to_manifest_type(neotype: Option<&NeoType>, solidity_type: &str) -> &'static str {
        match neotype {
            Some(NeoType::Integer { .. }) => "Integer",
            Some(NeoType::Boolean) => "Boolean",
            Some(NeoType::String) => "String",
            Some(NeoType::Address) => "Hash160",
            Some(NeoType::ByteArray {
                fixed_len: Some(32),
            }) => "Hash256",
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
                            .unwrap_or_else(|| format!("arg{param_index}")),
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
                            .unwrap_or_else(|| format!("param{idx}")),
                        "type": neotype_to_manifest_type(param.neo_type.as_ref(), &param.ty),
                    })
                })
                .collect();

            json!({
                "name": event.name,
                "parameters": params,
            })
        })
        .collect();

    let detection = detect_supported_standards(&metadata.methods, &metadata.events);
    let supported_standards = detection.standards;
    for diag in &detection.diagnostics {
        let prefix = match diag.level {
            StandardsDiagnosticLevel::Warning => "warning",
            StandardsDiagnosticLevel::Info => "info",
        };
        eprintln!(
            "[{prefix}][{standard}] {msg}",
            standard = diag.standard,
            msg = diag.message
        );
    }
    let permissions = infer_permissions(metadata, ir_module);
    // Neo N3 keeps `features` reserved for future use; Neo's manifest parser will
    // reject any populated keys. Keep the object empty for chain compatibility.
    let features = serde_json::Map::new();

    let mut manifest = json!({
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
    });

    apply_manifest_custom_overrides(&mut manifest, metadata);
    manifest
}
