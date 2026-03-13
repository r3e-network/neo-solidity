pub(crate) fn process_standard_json(
    input_path: &str,
    output_path: Option<&str>,
    options: StandardJsonOptions<'_>,
) -> Result<(), String> {
    let input_content = fs::read_to_string(input_path)
        .map_err(|err| format!("Failed to read input file: {err}"))?;
    process_standard_json_content(&input_content, output_path, options)
}

pub(crate) fn process_standard_json_content(
    input_content: &str,
    output_path: Option<&str>,
    options: StandardJsonOptions<'_>,
) -> Result<(), String> {
    let request: StandardJsonInput = serde_json::from_str(input_content)
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

    let mut optimizer_level = options.optimizer_level.min(3);
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
                "code": standard_json_manual_code("MissingSourceContent"),
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
        let combined_source =
            build_combined_source_with_import_validation(&ordered_sources, &mut errors);

        let contract_file_map = build_contract_file_map(&ordered_sources);

        match compile_contracts_with_options(
            &combined_source,
            false,
            CompileOptions {
                optimizer_level,
                use_callt: options.use_callt,
                deny_wildcard_permissions: options.deny_wildcard_permissions,
                deny_wildcard_contracts: options.deny_wildcard_contracts,
                deny_wildcard_methods: options.deny_wildcard_methods,
                manifest_permissions: options.manifest_permissions.clone(),
            },
        ) {
            Ok(artifacts) => {
                let artifacts: Vec<CompilationArtifacts> = if options.contract_names.is_empty() {
                    artifacts
                } else {
                    artifacts
                        .into_iter()
                        .filter(|artifact| {
                            options
                                .contract_names
                                .iter()
                                .any(|name| name == &artifact.metadata.name)
                        })
                        .collect()
                };

                if artifacts.is_empty() {
                    let file_name = &ordered_sources[0].0;
                    if options.contract_names.is_empty() {
                        errors.push(json!({
                            "component": "neo-solidity",
                            "severity": "error",
                            "type": "NoContracts",
                            "code": standard_json_manual_code("NoContracts"),
                            "sourceLocation": { "file": file_name },
                            "formattedMessage": format!("No contracts found in {file_name}"),
                            "message": format!("No contracts found in {file_name}"),
                        }));
                    } else {
                        let names = options.contract_names.join(", ");
                        errors.push(json!({
                            "component": "neo-solidity",
                            "severity": "error",
                            "type": "ContractNotFound",
                            "code": standard_json_manual_code("ContractNotFound"),
                            "sourceLocation": { "file": file_name },
                            "formattedMessage": format!("No matching contract(s) found for --contract {names}"),
                            "message": format!("No matching contract(s) found for --contract {names}"),
                        }));
                    }
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
                        let raw_source = options.nef_source.unwrap_or(target_file.as_str());
                        let (source_field, truncated) = clamp_nef_source_with_flag(raw_source);
                        if truncated {
                            errors.push(json!({
                                "component": "neo-solidity",
                                "severity": "warning",
                                "type": "NefSourceTruncated",
                                "code": standard_json_manual_code("NefSourceTruncated"),
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

                        let compiled_contract = match build_compiled_contract_value(
                            &target_file,
                            &artifact,
                            &abi_entries,
                            &request.settings,
                            source_keccak,
                            Some(source_field.as_ref()),
                        ) {
                            Ok(value) => value,
                            Err(message) => {
                                errors.push(json!({
                                    "component": "neo-solidity",
                                    "severity": "error",
                                    "type": "StandardJsonOutput",
                                    "code": standard_json_manual_code("StandardJsonOutput"),
                                    "sourceLocation": { "file": target_file },
                                    "formattedMessage": message,
                                    "message": message,
                                }));
                                continue;
                            }
                        };

                        let per_file_value = contracts_output
                            .entry(target_file.clone())
                            .or_insert_with(|| Value::Object(Map::new()));
                        let per_file = match per_file_value {
                            Value::Object(map) => map,
                            other => {
                                *other = Value::Object(Map::new());
                                match other {
                                    Value::Object(map) => map,
                                    _ => {
                                        errors.push(json!({
                                            "component": "neo-solidity",
                                            "severity": "error",
                                            "type": "StandardJsonOutput",
                                            "code": standard_json_manual_code("StandardJsonOutput"),
                                            "sourceLocation": { "file": target_file },
                                            "formattedMessage": "Failed to build standard JSON output: contracts entry is not an object",
                                            "message": "Failed to build standard JSON output: contracts entry is not an object",
                                        }));
                                        continue;
                                    }
                                }
                            }
                        };

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
