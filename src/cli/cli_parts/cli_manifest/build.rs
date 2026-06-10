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

/// Task #28: collect standards the contract EXPLICITLY advertises via
/// `@custom:neo.manifest.supportedstandards`. Only well-formed string entries
/// in a JSON array count — malformed overrides are already warned about and
/// dropped by `apply_manifest_custom_overrides`.
fn extract_declared_supportedstandards(metadata: &ContractMetadata) -> Vec<String> {
    for (tag, raw_value) in &metadata.documentation.custom {
        let Some(field) = tag
            .strip_prefix("neo.manifest.")
            .or_else(|| tag.strip_prefix("manifest."))
        else {
            continue;
        };
        if field != "supportedstandards" {
            continue;
        }
        let raw = raw_value.trim();
        if raw.is_empty() {
            continue;
        }
        if let Ok(Value::Array(items)) = serde_json::from_str::<Value>(raw) {
            return items
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
        }
    }
    Vec::new()
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

/// Compute the manifest-visible ABI name for each exposed (public/external,
/// non-constructor) method.
///
/// Neo N3 resolves calls by `(name, parameter count)`, so overloads that
/// differ in arity may legally share their Solidity name in the manifest.
/// Methods whose `(name, arity)` pair is unique among `exposed` therefore
/// keep `method.name`; only true same-arity collisions fall back to the
/// mangled `neo_name` (one deterministic "primary" per collision group —
/// smallest `neo_name` — keeps the original name).
///
/// Shared by manifest emission (`build_manifest`) and the standard-json
/// `methodMap` (`build_neo_method_map`) so both report the same callable
/// names.
fn manifest_abi_method_name_fn<'a>(
    exposed: impl Iterator<Item = &'a FunctionMetadata>,
) -> impl Fn(&FunctionMetadata) -> String {
    let mut groups: HashMap<(String, usize), Vec<String>> = HashMap::new();
    for method in exposed {
        groups
            .entry((method.name.clone(), method.parameters.len()))
            .or_default()
            .push(method.neo_name.clone());
    }

    let mut same_arity_primary: HashMap<(String, usize), String> = HashMap::new();
    for (key, neo_names) in groups {
        if neo_names.len() > 1 {
            let primary = neo_names
                .into_iter()
                .min()
                .expect("collision group is non-empty");
            same_arity_primary.insert(key, primary);
        }
    }

    move |method: &FunctionMetadata| {
        let key = (method.name.clone(), method.parameters.len());
        match same_arity_primary.get(&key) {
            Some(primary) if *primary != method.neo_name => method.neo_name.clone(),
            _ => method.name.clone(),
        }
    }
}

fn build_manifest(
    metadata: &ContractMetadata,
    ir_module: &ir::Module,
) -> Result<serde_json::Value, CompileError> {
    /// True for NeoTypes whose EVM ABI encoding is a single static 32-byte
    /// slot — the member shapes `lower_static_abi_slots_for_expr` supports.
    fn neotype_is_static_abi_scalar(neotype: &NeoType) -> bool {
        matches!(
            neotype,
            NeoType::Integer { .. }
                | NeoType::Boolean
                | NeoType::Address
                | NeoType::ByteArray { fixed_len: Some(_) }
        )
    }

    /// Mirrors the IR return-lowering's encode support
    /// (`lower_abi_encode_args_direct` and friends): a multi-value return is
    /// ABI-encoded into a single ByteString iff every member is a static
    /// slot type, a supported dynamic (string / bytes / static-element
    /// array), or an all-static struct. Members that defeat the encoder
    /// (mappings, structs with non-scalar fields) keep the legacy
    /// `StackItem::Array` return shape.
    fn neotype_member_is_abi_encodable(neotype: &NeoType) -> bool {
        match neotype {
            NeoType::Integer { .. }
            | NeoType::Boolean
            | NeoType::Address
            | NeoType::String
            | NeoType::ByteArray { .. } => true,
            NeoType::Array(inner) => {
                neotype_is_static_abi_scalar(inner)
                    || matches!(
                        inner.as_ref(),
                        NeoType::Struct { fields, .. }
                            if !fields.is_empty()
                                && fields.iter().all(|f| neotype_is_static_abi_scalar(&f.ty))
                    )
            }
            NeoType::Struct { fields, .. } => {
                !fields.is_empty() && fields.iter().all(|f| neotype_is_static_abi_scalar(&f.ty))
            }
            NeoType::Mapping { .. } | NeoType::Any => false,
        }
    }

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

    // Neo N3 dispatches methods by `(name, parameter count)` — distinct-arity
    // overloads sharing one name are legal (native ContractManagement ships
    // `deploy` with 2 and 3 parameters). So every method whose `(name, arity)`
    // pair is unique among the exposed methods keeps its original Solidity
    // name in the manifest; the mangled `neo_name` (e.g. `foo(uint256)`) is
    // used only for true same-arity collisions, where Neo cannot otherwise
    // disambiguate. Within a same-arity collision group one "primary" keeps
    // the original name (picked by lexicographically-smallest `neo_name` for
    // determinism) and the rest fall back to `neo_name`.
    let abi_method_name = manifest_abi_method_name_fn(abi_methods.iter().copied());

    let methods_json: Vec<_> = abi_methods
        .iter()
        .map(|method| {
            let abi_name = abi_method_name(method);

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
                // Task #64 lowering note: externally-callable multi-value
                // returns are ABI-encoded into a single ByteString
                // (BytesConcat / AbiEncode in `lower_return_statement`) when
                // every member is encodable, and single Solidity ARRAY
                // returns (`T[]` / `T[N]`) are wrapped the same way (Tasks
                // #137/#185). The on-chain stack item is therefore bytes,
                // not a NeoVM Array — report "ByteArray" so typed Neo SDK
                // clients parse the result instead of failing. Multi-returns
                // with non-encodable members (e.g. struct-with-mapping
                // auto-getters) fall back to the legacy StackItem::Array
                // shape and keep "Array"; single struct returns stay "Array"
                // too (see `return_revert.rs` Task #64 comment).
                "returntype": if method.return_parameters.len() > 1 {
                    let all_encodable = method.return_parameters.iter().all(|param| {
                        param
                            .neo_type
                            .as_ref()
                            .is_some_and(neotype_member_is_abi_encodable)
                    });
                    if all_encodable { "ByteArray" } else { "Array" }
                } else {
                    method
                        .return_parameters
                        .first()
                        .map(|param| {
                            if matches!(param.neo_type.as_ref(), Some(NeoType::Array(_))) {
                                "ByteArray"
                            } else {
                                neotype_to_manifest_type(param.neo_type.as_ref(), &param.ty)
                            }
                        })
                        .unwrap_or("Void")
                },
                "safe": method.state_mutability.is_safe(),
            })
        })
        .collect();

    // The emit lowering (src/ir/statements/events.rs) sends
    // `System.Runtime.Notify` the EVM-canonical state array
    //   [topic0, indexed..., data]   (non-anonymous events)
    //   [indexed..., data]           (anonymous events)
    // — NOT the Solidity-declared parameter list. Neo N3 nodes since 3.6
    // (HF_Basilisk) validate every notification against the manifest ABI
    // (state item count + per-item type), so the manifest must declare the
    // payload that is actually notified or every `emit` FAULTs on-chain.
    // Every state item is a ByteString (the 32-byte topic slots / keccak
    // hashes and the `abi.encode` data blob), hence `ByteArray` throughout.
    // The declared parameter names are preserved for the indexed slots so
    // indexers can still associate topics with the Solidity declaration.
    let events_json: Vec<_> = metadata
        .events
        .iter()
        .map(|event| {
            let mut params: Vec<Value> = Vec::new();
            if !event.anonymous {
                params.push(json!({
                    "name": "topic0",
                    "type": "ByteArray",
                }));
            }
            for (idx, param) in event.parameters.iter().enumerate() {
                if !param.indexed {
                    continue;
                }
                params.push(json!({
                    "name": param
                        .name
                        .clone()
                        .unwrap_or_else(|| format!("param{idx}")),
                    "type": "ByteArray",
                }));
            }
            params.push(json!({
                "name": "data",
                "type": "ByteArray",
            }));

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

    // Task #28: when the user EXPLICITLY advertises a NEP standard via
    // `@custom:neo.manifest.supportedstandards`, required methods and the
    // `Transfer` event become hard requirements — never ship a manifest that
    // falsely claims a standard the contract cannot fulfill.
    let declared = extract_declared_supportedstandards(metadata);
    if !declared.is_empty() {
        validate_declared_standards(&declared, &metadata.methods, &metadata.events)
            .map_err(CompileError::Manifest)?;
    }
    Ok(manifest)
}
