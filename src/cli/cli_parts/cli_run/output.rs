fn emit_contract_warnings(artifacts: &[CompilationArtifacts], json_warnings: bool) {
    for artifact in artifacts {
        for warning in &artifact.warnings {
            emit_warning(
                &warning.message,
                Some(&artifact.metadata.name),
                json_warnings,
                Some(standard_json::infer_validation_code(
                    &warning.message,
                    warning.severity,
                )),
            );
        }
    }
}

struct OutputConfig<'a> {
    format: &'a str,
    output_prefix: &'a str,
    input_file: &'a str,
    nef_source_override: Option<&'a str>,
    deployer: Option<[u8; 20]>,
    json_errors: bool,
    json_warnings: bool,
}

fn write_contract_outputs(
    artifacts: &[CompilationArtifacts],
    config: &OutputConfig<'_>,
) {
    for (index, artifact) in artifacts.iter().enumerate() {
        let prefix = contract_output_prefix(
            config.output_prefix,
            &artifact.metadata.name,
            index,
            artifacts.len(),
        );

        match config.format {
            "nef" => {
                let nef_path = if prefix.ends_with(".nef") {
                    prefix.clone()
                } else {
                    format!("{prefix}.nef")
                };
                let checksum = match write_nef_file(
                    &nef_path,
                    &artifact.bytecode,
                    &artifact.tokens,
                    config.nef_source_override.unwrap_or(config.input_file),
                    config.json_warnings,
                ) {
                    Ok(checksum) => checksum,
                    Err(err) => {
                        emit_error(&err, "OUTPUT_WRITE_ERROR", config.json_errors);
                        std::process::exit(1);
                    }
                };
                println!(
                    "✅ [{}] NEF file generated: {nef_path}",
                    artifact.metadata.name
                );
                if let Some(sender) = config.deployer {
                    let predicted = neo_solidity::neo::compute_contract_hash(
                        sender,
                        checksum,
                        artifact.metadata.name.as_str(),
                    );
                    println!(
                        "   • Predicted contract hash (sender {}, checksum 0x{checksum:08x}): {}",
                        neo_solidity::neo::format_uint160_hex_be(&sender),
                        neo_solidity::neo::format_uint160_hex_be(&predicted)
                    );
                }
            }
            "manifest" => {
                let manifest_path = if prefix.ends_with(".manifest.json") {
                    prefix.clone()
                } else {
                    format!("{prefix}.manifest.json")
                };
                if let Err(err) = write_manifest_file(&manifest_path, &artifact.manifest) {
                    emit_error(&err, "OUTPUT_WRITE_ERROR", config.json_errors);
                    std::process::exit(1);
                }
                println!(
                    "✅ [{}] Manifest file generated: {manifest_path}",
                    artifact.metadata.name
                );
            }
            "complete" => {
                let nef_path = format!("{prefix}.nef");
                let manifest_path = format!("{prefix}.manifest.json");
                let checksum = match write_nef_file(
                    &nef_path,
                    &artifact.bytecode,
                    &artifact.tokens,
                    config.nef_source_override.unwrap_or(config.input_file),
                    config.json_warnings,
                ) {
                    Ok(checksum) => checksum,
                    Err(err) => {
                        emit_error(&err, "OUTPUT_WRITE_ERROR", config.json_errors);
                        std::process::exit(1);
                    }
                };
                if let Err(err) = write_manifest_file(&manifest_path, &artifact.manifest) {
                    emit_error(&err, "OUTPUT_WRITE_ERROR", config.json_errors);
                    std::process::exit(1);
                }
                println!(
                    "✅ [{}] Contract files generated:\n   📄 {nef_path}\n   📄 {manifest_path}",
                    artifact.metadata.name
                );
                if let Some(sender) = config.deployer {
                    let predicted = neo_solidity::neo::compute_contract_hash(
                        sender,
                        checksum,
                        artifact.metadata.name.as_str(),
                    );
                    println!(
                        "   • Predicted contract hash (sender {}, checksum 0x{checksum:08x}): {}",
                        neo_solidity::neo::format_uint160_hex_be(&sender),
                        neo_solidity::neo::format_uint160_hex_be(&predicted)
                    );
                }
            }
            "json" => {
                let json_path = if prefix.ends_with(".json") {
                    prefix.clone()
                } else {
                    format!("{prefix}.json")
                };
                if let Err(err) = write_json_file(
                    &json_path,
                    &artifact.bytecode,
                    &artifact.tokens,
                    &artifact.manifest,
                    &artifact.metadata,
                    config.nef_source_override.unwrap_or(config.input_file),
                    config.json_warnings,
                ) {
                    emit_error(&err, "OUTPUT_WRITE_ERROR", config.json_errors);
                    std::process::exit(1);
                }
                println!(
                    "✅ [{}] JSON file generated: {json_path}",
                    artifact.metadata.name
                );
            }
            "assembly" => {
                let asm_path = if prefix.ends_with(".asm") {
                    prefix.clone()
                } else {
                    format!("{prefix}.asm")
                };
                if let Err(err) = write_assembly_file(&asm_path, &artifact.bytecode) {
                    emit_error(&err, "OUTPUT_WRITE_ERROR", config.json_errors);
                    std::process::exit(1);
                }
                println!(
                    "✅ [{}] Assembly file generated: {asm_path}",
                    artifact.metadata.name
                );
            }
            other => {
                println!("Unsupported format: {other}");
            }
        }
    }
}
