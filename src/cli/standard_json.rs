use super::{compile_contracts, CompilationArtifacts, COMPILER_ID, VERSION};
use neo_solidity::frontend::parse_source;
use neo_solidity::neo::{build_nef_with_tokens, clamp_nef_source_with_flag, NEF_SOURCE_MAX_BYTES};
use neo_solidity::solidity::{ContractMetadata, DiagnosticSeverity, FunctionKind, StateMutability};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
pub(crate) struct StandardJsonInput {
    pub(crate) language: String,
    pub(crate) sources: HashMap<String, StandardJsonSource>,
    #[serde(default)]
    pub(crate) settings: Value,
}

#[derive(Deserialize)]
pub(crate) struct StandardJsonSource {
    pub(crate) content: Option<String>,
    pub(crate) urls: Option<Vec<String>>,
}

pub(crate) fn process_standard_json(
    input_path: &str,
    output_path: Option<&str>,
    optimizer_level: u8,
    nef_source: Option<&str>,
) -> Result<(), String> {
    let input_content = fs::read_to_string(input_path)
        .map_err(|err| format!("Failed to read input file: {err}"))?;
    let request: StandardJsonInput = serde_json::from_str(&input_content)
        .map_err(|err| format!("Failed to parse standard JSON input: {err}"))?;

    if request.sources.is_empty() {
        return Err("Standard JSON input must contain at least one source".to_string());
    }

    if !request.language.trim().is_empty() && !request.language.eq_ignore_ascii_case("solidity") {
        return Err(format!(
            "Unsupported language '{}' in standard JSON input",
            request.language
        ));
    }

    let mut optimizer_level = optimizer_level.min(3);
    optimizer_level = read_optimizer_level(&request.settings).unwrap_or(optimizer_level);
    let mut contracts_output = Map::new();
    let mut sources_output = Map::new();
    let mut errors: Vec<Value> = Vec::new();
    if let Some(warning) = unsupported_settings_warning(&request.settings) {
        errors.push(warning);
    }
    let mut ordered_sources: Vec<(String, String, String)> = Vec::new();

    for (index, (file_name, source)) in request.sources.iter().enumerate() {
        sources_output.insert(file_name.clone(), json!({ "id": index as u32 }));
        let Some(content) = source.content.as_ref() else {
            let mut message = format!(
                "Source '{file_name}' is missing inline content; URL imports are not supported."
            );
            if let Some(urls) = &source.urls {
                if let Some(first_url) = urls.first() {
                    message.push_str(&format!(" First URL provided: '{first_url}'."));
                }
            }
            errors.push(json!({
                "component": "neo-solidity",
                "severity": "error",
                "type": "MissingSourceContent",
                "sourceLocation": { "file": file_name },
                "formattedMessage": message,
                "message": message,
            }));
            continue;
        };

        let keccak_hex = keccak256_hex(content);
        let keccak_prefixed = hex_prefixed(&keccak_hex);
        sources_output.insert(
            file_name.clone(),
            json!({
                "id": index as u32,
                "keccak256": keccak_prefixed,
            }),
        );

        ordered_sources.push((file_name.clone(), content.clone(), keccak_prefixed));
    }

    if !ordered_sources.is_empty() {
        let combined_source = ordered_sources
            .iter()
            .map(|(_, content, _)| content.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");

        let contract_file_map = build_contract_file_map(&ordered_sources);

        match compile_contracts(&combined_source, false, optimizer_level) {
            Ok(artifacts) => {
                if artifacts.is_empty() {
                    let file_name = &ordered_sources[0].0;
                    errors.push(json!({
                        "component": "neo-solidity",
                        "severity": "error",
                        "type": "NoContracts",
                        "sourceLocation": { "file": file_name },
                        "formattedMessage": format!("No contracts found in {file_name}"),
                        "message": format!("No contracts found in {file_name}"),
                    }));
                } else {
                    for artifact in artifacts {
                        let target_file = contract_file_map
                            .get(&artifact.metadata.name)
                            .cloned()
                            .unwrap_or_else(|| ordered_sources[0].0.clone());

                        let source_keccak = ordered_sources
                            .iter()
                            .find(|(name, _, _)| *name == target_file)
                            .map(|(_, _, keccak)| keccak.as_str());

                        let abi_entries = build_standard_abi(&artifact.metadata);
                        let raw_source = nef_source.unwrap_or(target_file.as_str());
                        let (source_field, truncated) = clamp_nef_source_with_flag(raw_source);
                        if truncated {
                            errors.push(json!({
                                "component": "neo-solidity",
                                "severity": "warning",
                                "type": "NefSourceTruncated",
                                "code": "NEF_SOURCE_TRUNCATED",
                                "sourceLocation": { "file": target_file },
                                "formattedMessage": format!(
                                    "NEF source exceeds {} bytes and will be truncated",
                                    NEF_SOURCE_MAX_BYTES
                                ),
                                "message": format!(
                                    "NEF source exceeds {} bytes and will be truncated",
                                    NEF_SOURCE_MAX_BYTES
                                ),
                            }));
                        }

                        let compiled_contract = build_compiled_contract_value(
                            &target_file,
                            &artifact,
                            &abi_entries,
                            &request.settings,
                            source_keccak,
                            Some(source_field.as_ref()),
                        );

                        let per_file_value = contracts_output
                            .entry(target_file.clone())
                            .or_insert_with(|| Value::Object(Map::new()));
                        let per_file = per_file_value
                            .as_object_mut()
                            .expect("per-file entry should be an object");

                        per_file.insert(artifact.metadata.name.clone(), compiled_contract);

                        for warning in &artifact.warnings {
                            errors.push(diagnostic_to_standard_error(warning, &target_file));
                        }
                    }
                }
            }
            Err(err) => {
                let file_hint = ordered_sources
                    .first()
                    .map(|(name, _, _)| name.as_str())
                    .unwrap_or("unknown");
                errors.extend(err.into_errors(file_hint));
            }
        }
    }

    let mut output = Map::new();
    output.insert("contracts".into(), Value::Object(contracts_output));
    output.insert("sources".into(), Value::Object(sources_output));
    if !errors.is_empty() {
        output.insert("errors".into(), Value::Array(errors));
    }

    let serialized = serde_json::to_string_pretty(&Value::Object(output))
        .map_err(|err| format!("Failed to serialise standard JSON output: {err}"))?;
    if let Some(path) = output_path {
        fs::write(path, serialized)
            .map_err(|err| format!("Failed to write standard JSON output: {err}"))?;
    } else {
        println!("{serialized}");
    }

    Ok(())
}

pub(crate) fn diagnostic_to_standard_error(
    diagnostic: &neo_solidity::solidity::Diagnostic,
    file: &str,
) -> Value {
    let severity = match diagnostic.severity {
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Error => "error",
    };
    let code = infer_validation_code(&diagnostic.message, diagnostic.severity);

    json!({
        "component": "neo-solidity",
        "severity": severity,
        "type": "Validation",
        "code": code,
        "sourceLocation": { "file": file },
        "formattedMessage": diagnostic.message,
        "message": diagnostic.message,
    })
}

pub(crate) fn infer_validation_code(message: &str, severity: DiagnosticSeverity) -> &'static str {
    let msg = message.to_ascii_lowercase();

    if msg.contains("duplicate function signature") {
        "DUPLICATE_SIGNATURE"
    } else if msg.contains("duplicate parameter name") {
        "DUPLICATE_PARAMETER_NAME"
    } else if msg.contains("unsupported type") && msg.contains("parameter") {
        "UNSUPPORTED_PARAMETER_TYPE"
    } else if msg.contains("unsupported type") && msg.contains("state variable") {
        "UNSUPPORTED_STATE_TYPE"
    } else if msg.contains("unsupported type") && msg.contains("return type") {
        "UNSUPPORTED_RETURN_TYPE"
    } else if msg.contains("unsupported type") {
        "UNSUPPORTED_TYPE"
    } else if msg.contains("constructor must not specify a return type") {
        "INVALID_CONSTRUCTOR_RETURN"
    } else if msg.contains("multiple constructors defined") {
        "MULTIPLE_CONSTRUCTORS"
    } else if msg.contains("state variable declared without a name") {
        "STATE_VARIABLE_NAME_MISSING"
    } else if msg.contains("duplicate state variable") {
        "DUPLICATE_STATE_VARIABLE"
    } else if msg.contains("constant state variable") && msg.contains("must have an initializer") {
        "CONSTANT_MISSING_INITIALIZER"
    } else if msg.contains("may not use 'storage'") {
        "INVALID_STORAGE_PARAM"
    } else if msg.contains("return value") && msg.contains("storage") {
        "INVALID_STORAGE_RETURN"
    } else if msg.contains("returns multiple values") {
        "MULTIPLE_RETURN_VALUES_UNSUPPORTED"
    } else if msg.contains("expected") && msg.contains("return") && msg.contains("values") {
        "RETURN_MISMATCH"
    } else if msg.contains("returns") && msg.contains("may not map cleanly") {
        "RETURN_TYPE_UNMAPPED"
    } else if msg.contains("return type") && msg.contains("unsupported") {
        "UNSUPPORTED_RETURN_TYPE"
    } else if msg.contains("event") && msg.contains("exceeds neo abi limits") {
        "EVENT_PARAM_LIMIT"
    } else if msg.contains("declares a return type but has no implementation") {
        "MISSING_IMPLEMENTATION_RETURN"
    } else {
        match severity {
            DiagnosticSeverity::Warning => "VALIDATION_WARNING",
            DiagnosticSeverity::Error => "VALIDATION_ERROR",
        }
    }
}

pub(crate) fn build_standard_abi(metadata: &ContractMetadata) -> Vec<Value> {
    let mut abi_entries = Vec::new();

    for method in &metadata.methods {
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

pub(crate) fn build_compiled_contract_value(
    file_name: &str,
    artifact: &CompilationArtifacts,
    abi_entries: &[Value],
    settings: &Value,
    source_keccak: Option<&str>,
    nef_source: Option<&str>,
) -> Value {
    let raw_source = nef_source.unwrap_or(file_name);
    let (source_field, _) = clamp_nef_source_with_flag(raw_source);
    let script_hex = hex::encode(&artifact.bytecode);
    let bytecode_object = format!("0x{script_hex}");
    let metadata_blob = build_metadata_blob(
        &artifact.metadata.name,
        abi_entries,
        file_name,
        settings,
        source_keccak,
    );
    let method_identifiers = build_method_identifiers(&artifact.metadata);

    let storage_map = build_storage_map(&artifact.metadata);
    let manifest = artifact.manifest.clone();
    let nef_bytes =
        build_nef_with_tokens(&artifact.bytecode, COMPILER_ID, source_field.as_ref(), &[]);
    let checksum = if nef_bytes.len() >= 4 {
        hex::encode(&nef_bytes[nef_bytes.len() - 4..])
    } else {
        "00000000".to_string()
    };
    let nef_image = hex::encode(nef_bytes);

    json!({
        "abi": abi_entries,
        "metadata": metadata_blob,
        "evm": {
            "bytecode": {
                "object": bytecode_object,
                "opcodes": "",
                "sourceMap": "",
                "linkReferences": {}
            },
            "deployedBytecode": {
                "object": bytecode_object,
                "opcodes": "",
                "sourceMap": "",
                "linkReferences": {}
            },
            "methodIdentifiers": method_identifiers,
        },
        "neo": {
            "nef": {
                "magic": "NEF3",
                "compiler": COMPILER_ID,
                "source": source_field.as_ref(),
                "tokens": [],
                "script": script_hex,
                "image": nef_image,
                "checksum": checksum,
            },
            "manifest": manifest,
            "storageMap": storage_map,
            "gasEstimates": {
                "creation": zero_gas_estimate_value(),
                "functions": Value::Object(Map::new())
            }
        }
    })
}

pub(crate) fn build_metadata_blob(
    contract_name: &str,
    abi_entries: &[Value],
    file_name: &str,
    settings: &Value,
    keccak_hex: Option<&str>,
) -> String {
    let keccak_field = keccak_hex.map(hex_prefixed).unwrap_or_default();
    let metadata = json!({
        "compiler": {
            "name": COMPILER_ID,
            "version": format!("{}.{}.{}.{}", VERSION.0, VERSION.1, VERSION.2, VERSION.3)
        },
        "language": "Solidity",
        "output": {
            "abi": abi_entries,
            "contractName": contract_name,
        },
        "settings": settings,
        "sources": {
            file_name: {
                "keccak256": keccak_field,
                "urls": []
            }
        },
        "version": 1
    });

    serde_json::to_string(&metadata).unwrap_or_else(|_| "{}".to_string())
}

pub(crate) fn build_storage_map(metadata: &ContractMetadata) -> Value {
    let mut entries = Map::new();
    for (slot, variable) in metadata.state_variables.iter().enumerate() {
        let name = variable
            .name
            .clone()
            .unwrap_or_else(|| format!("slot_{slot}"));
        entries.insert(
            name,
            json!({
                "slot": slot,
                "type": variable.ty,
                "description": variable.visibility.clone().unwrap_or_default(),
            }),
        );
    }
    Value::Object(entries)
}

pub(crate) fn zero_gas_estimate_value() -> Value {
    json!({
        "gas": "0",
        "systemFee": "0",
        "networkFee": "0",
    })
}

pub(crate) fn state_mutability_label(state: StateMutability) -> &'static str {
    match state {
        StateMutability::Pure => "pure",
        StateMutability::View => "view",
        StateMutability::Payable => "payable",
        StateMutability::NonPayable => "nonpayable",
    }
}

pub(crate) fn sanitize_contract_name(name: &str) -> Option<String> {
    let filtered: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();

    if filtered.trim_matches('_').is_empty() {
        None
    } else {
        Some(filtered.trim_matches('_').to_string())
    }
}

pub(crate) fn solidity_to_manifest_type(solidity_type: &str) -> &'static str {
    let ty = solidity_type.trim().to_ascii_lowercase();

    // Array types must be checked FIRST (before checking base types)
    // This ensures uint256[] returns "Array" not "Integer"
    if ty.ends_with("[]") {
        return "Array";
    }

    // Mapping types
    if ty.starts_with("mapping") {
        return "Map";
    }

    // Integer types (uint8-256, int8-256)
    if ty.starts_with("uint") || ty.starts_with("int") {
        return "Integer";
    }

    // Boolean
    if ty == "bool" || ty == "boolean" {
        return "Boolean";
    }

    // String
    if ty == "string" {
        return "String";
    }

    // Address types (Neo uses Hash160 for 20-byte addresses)
    if ty == "address" || ty == "bytes20" || ty == "hash160" {
        return "Hash160";
    }

    // Hash types (must check before generic bytes handling)
    if ty == "bytes32" || ty == "hash256" {
        return "Hash256";
    }

    // Fixed-size byte arrays (bytes1-32)
    if ty == "bytes" {
        return "ByteArray";
    }
    if ty.starts_with("bytes") {
        // bytes1, bytes2, ..., bytes32 are fixed-size
        if let Some(size_str) = ty.strip_prefix("bytes") {
            if size_str.parse::<u8>().is_ok() {
                return if size_str == "32" {
                    "Hash256"
                } else {
                    "ByteArray"
                };
            }
        }
        return "ByteArray";
    }

    // Void/empty return type
    if ty == "void" || ty.is_empty() {
        return "Void";
    }

    // Struct and other complex types
    "Any"
}

pub(crate) fn keccak256_hex(input: &str) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(input.as_bytes());
    let digest = hasher.finalize();
    hex::encode(digest)
}

pub(crate) fn hex_prefixed(value: &str) -> String {
    if value.starts_with("0x") || value.starts_with("0X") {
        value.to_string()
    } else {
        format!("0x{value}")
    }
}

fn canonical_param_type(ty: &str) -> String {
    ty.split_whitespace().next().unwrap_or_default().to_string()
}

fn build_contract_file_map(sources: &[(String, String, String)]) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for (file_name, content, _) in sources {
        if let Ok(contracts) = parse_source(content) {
            for contract in contracts {
                map.entry(contract.name.clone())
                    .or_insert_with(|| file_name.clone());
            }
        }
    }

    map
}

fn unsupported_settings_warning(settings: &Value) -> Option<Value> {
    let unsupported_keys: Vec<_> = match settings {
        Value::Null => return None,
        Value::Object(map) if map.is_empty() => return None,
        Value::Object(map) => map
            .keys()
            .filter(|k| k.as_str() != "optimizer")
            .cloned()
            .collect(),
        _ => vec!["<non-object settings>".to_string()],
    };

    if unsupported_keys.is_empty() {
        None
    } else {
        Some(json!({
            "component": "neo-solidity",
            "severity": "warning",
            "type": "UnsupportedSettings",
            "formattedMessage": format!(
                "Standard JSON settings contain unsupported keys: {:?}",
                unsupported_keys
            ),
            "message": "Standard JSON settings are present but not fully supported; unsupported keys were ignored.",
        }))
    }
}

fn read_optimizer_level(settings: &Value) -> Option<u8> {
    let optimizer = match settings {
        Value::Object(map) => map.get("optimizer"),
        _ => None,
    };

    match optimizer {
        Some(Value::Bool(enabled)) => {
            if *enabled {
                Some(3)
            } else {
                Some(0)
            }
        }
        Some(Value::Object(opt_map)) => {
            if let Some(Value::Bool(enabled)) = opt_map.get("enabled") {
                if !enabled {
                    return Some(0);
                }
            }
            if let Some(Value::Number(level_num)) = opt_map.get("level") {
                if let Some(level) = level_num.as_u64() {
                    return Some((level as u8).min(3));
                }
            }
            if let Some(Value::Number(runs)) = opt_map.get("runs") {
                if let Some(runs_val) = runs.as_u64() {
                    return Some(if runs_val > 200 { 3 } else { 2 });
                }
            }
            Some(3)
        }
        _ => None,
    }
}
