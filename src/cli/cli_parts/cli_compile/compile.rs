pub fn compile_contracts(
    source: &str,
    verbose: bool,
    optimizer_level: u8,
) -> Result<Vec<CompilationArtifacts>, CompileError> {
    compile_contracts_with_options(
        source,
        verbose,
        CompileOptions {
            optimizer_level,
            use_callt: false,
            deny_wildcard_permissions: false,
            deny_wildcard_contracts: false,
            deny_wildcard_methods: false,
            manifest_permissions: None,
        },
    )
}

fn compile_contracts_with_options(
    source: &str,
    verbose: bool,
    options: CompileOptions,
) -> Result<Vec<CompilationArtifacts>, CompileError> {
    let metadatas =
        analyse_all_sources(source).map_err(|err| CompileError::Message(err.to_string()))?;

    metadatas
        .into_iter()
        .map(|metadata| compile_metadata(metadata, verbose, options.clone()))
        .collect()
}

fn compile_metadata(
    mut metadata: ContractMetadata,
    verbose: bool,
    options: CompileOptions,
) -> Result<CompilationArtifacts, CompileError> {
    fn manifest_allows_permission(manifest: &Value, contract: &str, method: &str) -> bool {
        let permissions = manifest
            .get("permissions")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        permissions.iter().any(|entry| {
            let Some(entry_contract) = entry.get("contract").and_then(|v| v.as_str()) else {
                return false;
            };
            let methods_value = entry.get("methods").unwrap_or(&Value::Null);

            let contract_matches =
                entry_contract == "*" || entry_contract.eq_ignore_ascii_case(contract);
            if !contract_matches {
                return false;
            }

            match methods_value {
                Value::String(s) => s == "*",
                Value::Array(arr) => arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .any(|name| name == method),
                _ => false,
            }
        })
    }

    let optimizer_level = options.optimizer_level.min(3);
    let diagnostics = validate_contract(&metadata);
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    for diagnostic in diagnostics {
        match diagnostic.severity {
            DiagnosticSeverity::Warning => warnings.push(diagnostic),
            DiagnosticSeverity::Error => errors.push(diagnostic),
        }
    }

    if !errors.is_empty() {
        return Err(CompileError::Diagnostics(errors));
    }

    if let Err(diags) = build_semantic_model(&metadata) {
        return Err(CompileError::Semantic(diags));
    }

    ensure_deploy_stub(&mut metadata)?;
    let has_parameterised_constructor = metadata.methods.iter().any(|m| {
        matches!(m.kind, FunctionKind::Constructor) && !m.parameters.is_empty()
    });

    let ir_module = ir::Module::from_contract(&metadata).map_err(CompileError::Ir)?;
    let ir_module = optimize_ir(ir_module, optimizer_level);

    if verbose {
        println!(
            "Semantic model built: {} functions, {} state variables",
            ir_module.functions.len(),
            ir_module.state_variables.len()
        );

        for function in &ir_module.functions {
            let instruction_count: usize = function
                .basic_blocks
                .iter()
                .map(|block| block.instructions.len())
                .sum();
            println!(
                "   • IR function '{}' (kind: {:?}) => {} instruction(s)",
                function.name, function.kind, instruction_count
            );
        }
    }

    // Pass optimizer_level to bytecode generation for optimization passes.
    let bytecode_output = generate_contract_bytecode(
        &mut metadata,
        &ir_module,
        verbose,
        optimizer_level,
        options.use_callt,
    )
    .map_err(CompileError::Message)?;
    let mut manifest = build_manifest(&metadata, &ir_module);

    if let Some(override_permissions) = &options.manifest_permissions {
        let mut inferred = parse_manifest_permissions_from_manifest(&manifest).map_err(|err| {
            CompileError::Message(format!("Failed to parse inferred manifest permissions: {err}"))
        })?;

        match override_permissions.mode {
            ManifestPermissionsMode::Merge => {
                merge_manifest_permissions(&mut inferred, &override_permissions.permissions);
            }
            ManifestPermissionsMode::ReplaceWildcards => {
                inferred.retain(|contract, methods| contract != "*" && !methods.is_wildcard());
                merge_manifest_permissions(&mut inferred, &override_permissions.permissions);
            }
        }

        manifest["permissions"] = manifest_permissions_to_json(inferred);
    }

    if has_parameterised_constructor {
        let stdlib_hash_le = bytecode::native_contract_hash(ir::NativeContract::StdLib);
        let stdlib_hash_be = stdlib_hash_le.iter().rev().copied().collect::<Vec<_>>();
        let stdlib_contract = format!("0x{}", hex::encode(stdlib_hash_be));

        let has_json_deserialize =
            manifest_allows_permission(&manifest, &stdlib_contract, "jsonDeserialize");
        let has_deserialize = manifest_allows_permission(&manifest, &stdlib_contract, "deserialize");

        if !(has_json_deserialize && has_deserialize) {
            warnings.push(neo_solidity::solidity::Diagnostic::warning(format!(
                    "contract '{}' has a parameterised constructor; deploy it by passing constructor args through `_deploy(data, update)`. Neo-Express: pass a JSON array string via `-d '[7]'`; SDKs that support StackItems may pass an Array directly. The injected deploy prologue uses `StdLib.jsonDeserialize` with a `StdLib.deserialize` fallback, so the manifest must allow these methods.",
                    metadata.name
                )));
        }
    }

    let (
        has_wildcard_contract,
        has_wildcard_methods,
        has_full_wildcard_permissions,
        wildcard_contract_only_nep_callbacks,
    ) = manifest["permissions"]
        .as_array()
        .map(|permissions| {
            let mut wildcard_contract = false;
            let mut wildcard_methods = false;
            let mut full_wildcard = false;
            let mut wildcard_contract_only_nep_callbacks = true;
            const NEP_CALLBACK_METHODS: [&str; 3] = ["onNEP11Payment", "onNEP17Payment", "onOracleResponse"];
            for entry in permissions {
                let contract_is_wildcard = entry["contract"] == "*";
                let methods_is_wildcard = entry["methods"] == "*";
                wildcard_contract |= contract_is_wildcard;
                wildcard_methods |= methods_is_wildcard;
                full_wildcard |= contract_is_wildcard && methods_is_wildcard;

                if contract_is_wildcard {
                    if methods_is_wildcard {
                        wildcard_contract_only_nep_callbacks = false;
                    } else if let Some(methods) = entry["methods"].as_array() {
                        for method in methods {
                            match method.as_str() {
                                Some(name) if NEP_CALLBACK_METHODS.contains(&name) => {}
                                _ => wildcard_contract_only_nep_callbacks = false,
                            }
                        }
                    } else {
                        wildcard_contract_only_nep_callbacks = false;
                    }
                }
            }
            if !wildcard_contract {
                wildcard_contract_only_nep_callbacks = false;
            }
            (
                wildcard_contract,
                wildcard_methods,
                full_wildcard,
                wildcard_contract_only_nep_callbacks,
            )
        })
        .unwrap_or((false, false, false, false));

    if has_full_wildcard_permissions {
        let message = format!(
            "contract '{}' requires full wildcard manifest permissions (contract='*', methods='*') because at least one contract call is fully dynamic (unknown target + method) or could not be statically analysed. This is usually not acceptable for production deployments; prefer static calls or restrict the call surface. Use --deny-wildcard-permissions to make this a hard error.",
            metadata.name
        );
        if options.deny_wildcard_permissions
            || options.deny_wildcard_contracts
            || options.deny_wildcard_methods
        {
            return Err(CompileError::Manifest(message));
        }
        warnings.push(neo_solidity::solidity::Diagnostic::warning(message));
    } else {
        if has_wildcard_contract {
            let message = format!(
                "contract '{}' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error.",
                metadata.name
            );
            if options.deny_wildcard_contracts && !wildcard_contract_only_nep_callbacks {
                return Err(CompileError::Manifest(message));
            }
            if !wildcard_contract_only_nep_callbacks {
                warnings.push(neo_solidity::solidity::Diagnostic::warning(message));
            }
        }

        if has_wildcard_methods {
            let message = format!(
                "contract '{}' requires wildcard method manifest permissions (methods='*') due to dynamic method names in contract calls. This is riskier than calling fixed methods; use --deny-wildcard-methods to make this a hard error.",
                metadata.name
            );
            if options.deny_wildcard_methods {
                return Err(CompileError::Manifest(message));
            }
            warnings.push(neo_solidity::solidity::Diagnostic::warning(message));
        }
    }

    Ok(CompilationArtifacts {
        metadata,
        bytecode: bytecode_output.script,
        tokens: bytecode_output.tokens,
        manifest,
        warnings,
    })
}
