use super::{compile_contracts, CompilationArtifacts, COMPILER_ID, VERSION};
use neo_solidity::neo::build_nef;
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
) -> Result<(), String> {
    let input_content = fs::read_to_string(input_path)
        .map_err(|err| format!("Failed to read input file: {err}"))?;
    let request: StandardJsonInput = serde_json::from_str(&input_content)
        .map_err(|err| format!("Failed to parse standard JSON input: {err}"))?;

    if request.sources.is_empty() {
        return Err("Standard JSON input must contain at least one source".to_string());
    }

    if !request.language.trim().is_empty() && request.language.to_ascii_lowercase() != "solidity" {
        return Err(format!(
            "Unsupported language '{}' in standard JSON input",
            request.language
        ));
    }

    let mut contracts_output = Map::new();
    let mut sources_output = Map::new();
    let mut errors: Vec<Value> = Vec::new();

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
        sources_output.insert(
            file_name.clone(),
            json!({
                "id": index as u32,
                "keccak256": hex_prefixed(&keccak_hex),
            }),
        );

        match compile_contracts(content, false) {
            Ok(artifacts) => {
                if artifacts.is_empty() {
                    errors.push(json!({
                        "component": "neo-solidity",
                        "severity": "error",
                        "type": "NoContracts",
                        "sourceLocation": { "file": file_name },
                        "formattedMessage": format!("No contracts found in {file_name}"),
                        "message": format!("No contracts found in {file_name}"),
                    }));
                    continue;
                }

                let per_file_value = contracts_output
                    .entry(file_name.clone())
                    .or_insert_with(|| Value::Object(Map::new()));
                let per_file = per_file_value
                    .as_object_mut()
                    .expect("per-file entry should be an object");

                for artifact in artifacts {
                    let abi_entries = build_standard_abi(&artifact.metadata);
                    let compiled_contract = build_compiled_contract_value(
                        file_name,
                        &artifact,
                        &abi_entries,
                        &request.settings,
                        Some(&hex_prefixed(&keccak_hex)),
                    );
                    per_file.insert(artifact.metadata.name.clone(), compiled_contract);

                    for warning in &artifact.warnings {
                        errors.push(diagnostic_to_standard_error(warning, file_name));
                    }
                }
            }
            Err(err) => {
                errors.extend(err.into_errors(file_name));
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

    json!({
        "component": "neo-solidity",
        "severity": severity,
        "type": "Validation",
        "sourceLocation": { "file": file },
        "formattedMessage": diagnostic.message,
        "message": diagnostic.message,
    })
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
) -> Value {
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
    let nef_bytes = build_nef(&artifact.bytecode, COMPILER_ID, VERSION);
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
                "source": file_name,
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
    if ty.starts_with("uint") || ty.starts_with("int") {
        "Integer"
    } else if ty == "bool" {
        "Boolean"
    } else if ty == "string" {
        "String"
    } else if ty == "address" || ty == "bytes20" || ty == "hash160" {
        "Hash160"
    } else if ty == "bytes" || ty.starts_with("bytes") {
        "ByteArray"
    } else if ty.ends_with("[]") {
        "Array"
    } else if ty == "void" || ty.is_empty() {
        "Void"
    } else {
        "Any"
    }
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
