fn parse_manifest_permissions_mode(value: &str) -> Result<ManifestPermissionsMode, String> {
    match value.trim() {
        "merge" => Ok(ManifestPermissionsMode::Merge),
        "replace-wildcards" => Ok(ManifestPermissionsMode::ReplaceWildcards),
        other => Err(format!(
            "invalid manifest permissions mode '{other}' (expected 'merge' or 'replace-wildcards')"
        )),
    }
}

fn load_manifest_permissions_override(
    path: &str,
    mode: &str,
) -> Result<ManifestPermissionsOverride, String> {
    let mode = parse_manifest_permissions_mode(mode)?;
    let content =
        fs::read_to_string(path).map_err(|err| format!("Failed to read manifest permissions file '{path}': {err}"))?;
    let root: Value =
        serde_json::from_str(&content).map_err(|err| format!("Failed to parse manifest permissions JSON: {err}"))?;
    let permissions_value = match root {
        Value::Array(_) => root,
        Value::Object(mut map) => map
            .remove("permissions")
            .ok_or_else(|| "manifest permissions JSON object must contain a 'permissions' array".to_string())?,
        _ => {
            return Err(
                "manifest permissions JSON must be either an array or an object containing a 'permissions' array"
                    .to_string(),
            )
        }
    };

    let permissions = parse_manifest_permissions_array(&permissions_value)?;

    Ok(ManifestPermissionsOverride { mode, permissions })
}

fn parse_manifest_permissions_array(value: &Value) -> Result<ManifestPermissionMap, String> {
    let arr = value.as_array().ok_or_else(|| {
        "manifest permissions must be an array of objects like {\"contract\":\"0x...\",\"methods\":[...]}"
            .to_string()
    })?;

    let mut out: ManifestPermissionMap = ManifestPermissionMap::new();
    for (index, entry) in arr.iter().enumerate() {
        let obj = entry.as_object().ok_or_else(|| {
            format!("manifest permission entry #{index} must be an object")
        })?;

        let contract_value = obj.get("contract").ok_or_else(|| {
            format!("manifest permission entry #{index} is missing 'contract'")
        })?;
        let contract_raw = contract_value.as_str().ok_or_else(|| {
            format!("manifest permission entry #{index} field 'contract' must be a string")
        })?;
        let contract = if contract_raw.trim() == "*" {
            "*".to_string()
        } else {
            let parsed = neo_solidity::neo::parse_uint160_hex_be(contract_raw).map_err(|err| {
                format!("manifest permission entry #{index} has invalid 'contract': {err}")
            })?;
            neo_solidity::neo::format_uint160_hex_be(&parsed)
        };

        let methods_value = obj.get("methods").ok_or_else(|| {
            format!("manifest permission entry #{index} is missing 'methods'")
        })?;
        let methods = match methods_value {
            Value::String(s) if s.trim() == "*" => ManifestPermissionMethods::All,
            Value::Array(list) => {
                let mut set = BTreeSet::new();
                for (method_index, method) in list.iter().enumerate() {
                    let method_str = method.as_str().ok_or_else(|| {
                        format!(
                            "manifest permission entry #{index} methods[{method_index}] must be a string"
                        )
                    })?;
                    let trimmed = method_str.trim();
                    if trimmed.is_empty() {
                        return Err(format!(
                            "manifest permission entry #{index} methods[{method_index}] must not be empty"
                        ));
                    }
                    set.insert(trimmed.to_string());
                }
                ManifestPermissionMethods::Some(set)
            }
            _ => {
                return Err(format!(
                    "manifest permission entry #{index} field 'methods' must be \"*\" or an array of strings"
                ))
            }
        };

        out.entry(contract)
            .and_modify(|existing| existing.merge_in(methods.clone()))
            .or_insert(methods);
    }

    Ok(out)
}

fn manifest_permissions_to_json(permissions: ManifestPermissionMap) -> Value {
    Value::Array(
        permissions
            .into_iter()
            .map(|(contract, methods)| {
                let methods_json = match methods {
                    ManifestPermissionMethods::All => json!("*"),
                    ManifestPermissionMethods::Some(set) => json!(set.into_iter().collect::<Vec<_>>()),
                };
                json!({
                    "contract": contract,
                    "methods": methods_json,
                })
            })
            .collect(),
    )
}

fn parse_manifest_permissions_from_manifest(manifest: &Value) -> Result<ManifestPermissionMap, String> {
    match manifest.get("permissions") {
        Some(Value::Array(_)) => parse_manifest_permissions_array(&manifest["permissions"]),
        Some(other) => Err(format!(
            "manifest 'permissions' must be an array (got {})",
            other
        )),
        None => Ok(ManifestPermissionMap::new()),
    }
}

fn merge_manifest_permissions(into: &mut ManifestPermissionMap, other: &ManifestPermissionMap) {
    for (contract, methods) in other {
        into.entry(contract.clone())
            .and_modify(|existing| existing.merge_in(methods.clone()))
            .or_insert_with(|| methods.clone());
    }
}
