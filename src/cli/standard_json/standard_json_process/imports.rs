#[derive(Debug, Clone)]
struct StandardJsonContractSource {
    file_name: String,
    contract_name: String,
}

struct CombinedStandardJsonSource {
    source: String,
    contract_sources: Vec<StandardJsonContractSource>,
    source_units: Vec<CombinedStandardJsonSourceUnit>,
}

struct CombinedStandardJsonSourceUnit {
    root_file: String,
    source: String,
    contract_sources: Vec<StandardJsonContractSource>,
}

fn build_combined_source_with_import_validation(
    sources: &[(String, String, String)],
    errors: &mut Vec<Value>,
) -> CombinedStandardJsonSource {
    use solang_parser::pt::{Import, ImportPath, SourceUnitPart};
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::path::{Component, Path};

    fn normalize_virtual_path(path: &Path) -> String {
        let mut components: Vec<String> = Vec::new();
        for comp in path.components() {
            match comp {
                Component::CurDir => {}
                Component::ParentDir => {
                    components.pop();
                }
                Component::Normal(part) => components.push(part.to_string_lossy().to_string()),
                Component::Prefix(_) | Component::RootDir => {}
            }
        }
        components.join("/")
    }

    fn resolve_import_key(
        from_file: &str,
        import_path: &str,
        available: &HashSet<String>,
    ) -> Option<String> {
        fn import_aliases(import_path: &str) -> Vec<String> {
            let mut aliases = vec![import_path.to_string()];

            if let Some(rest) = import_path.strip_prefix("openzeppelin-contracts/contracts/") {
                aliases.push(format!("@openzeppelin/contracts/{rest}"));
            } else if let Some(rest) =
                import_path.strip_prefix("openzeppelin-contracts-upgradeable/contracts/")
            {
                aliases.push(format!("@openzeppelin/contracts-upgradeable/{rest}"));
            } else if let Some(rest) = import_path.strip_prefix("openzeppelin-contracts/") {
                aliases.push(format!("@openzeppelin/contracts/{rest}"));
            } else if let Some(rest) =
                import_path.strip_prefix("openzeppelin-contracts-upgradeable/")
            {
                aliases.push(format!("@openzeppelin/contracts-upgradeable/{rest}"));
            }

            aliases
        }

        let from_dir = Path::new(from_file)
            .parent()
            .unwrap_or_else(|| Path::new(""));

        for candidate_import in import_aliases(import_path) {
            let import = Path::new(&candidate_import);
            if import.is_absolute() && available.contains(candidate_import.as_str()) {
                return Some(candidate_import);
            }

            let candidate = normalize_virtual_path(&from_dir.join(import));
            if available.contains(&candidate) {
                return Some(candidate);
            }

            // Fallbacks when sources use canonicalized/trimmed paths.
            if available.contains(candidate_import.as_str()) {
                return Some(candidate_import);
            }
            if let Some(stripped) = candidate_import.strip_prefix("./") {
                let candidate = normalize_virtual_path(&from_dir.join(stripped));
                if available.contains(&candidate) {
                    return Some(candidate);
                }
                if available.contains(stripped) {
                    return Some(stripped.to_string());
                }
            }
        }

        None
    }

    fn push_error(errors: &mut Vec<Value>, file: &str, typ: &str, message: String) {
        errors.push(json!({
            "component": "neo-solidity",
            "severity": "error",
            "type": typ,
            "code": standard_json_manual_code(typ),
            "sourceLocation": { "file": file },
            "formattedMessage": message,
            "message": message,
        }));
    }

    let available: HashSet<String> = sources.iter().map(|(name, _, _)| name.clone()).collect();
    let mut content_map: HashMap<String, String> = HashMap::new();
    for (name, content, _) in sources {
        content_map.insert(name.clone(), content.clone());
    }

    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    for (file_name, content, _) in sources {
        let (unit, _comments) = match solang_parser::parse(content, 0) {
            Ok(parsed) => parsed,
            Err(diags) => {
                let summary = diags
                    .iter()
                    .map(|d| d.message.as_str())
                    .collect::<Vec<_>>()
                    .join("; ");
                push_error(
                    errors,
                    file_name,
                    "ImportParseError",
                    format!("failed to parse source to resolve imports: {summary}"),
                );
                continue;
            }
        };

        let mut imports: Vec<String> = Vec::new();
        for part in unit.0.iter() {
            let SourceUnitPart::ImportDirective(import) = part else {
                continue;
            };

            match import {
                Import::Plain(path, _) => {
                    if let ImportPath::Filename(lit) = path {
                        imports.push(lit.string.clone());
                    } else {
                        push_error(
                            errors,
                            file_name,
                            "UnsupportedImportPath",
                            "unsupported import path kind (path imports are not supported)"
                                .to_string(),
                        );
                    }
                }
                Import::Rename(path, _renames, _) => {
                    if let ImportPath::Filename(lit) = path {
                        imports.push(lit.string.clone());
                    } else {
                        push_error(
                            errors,
                            file_name,
                            "UnsupportedImportPath",
                            "unsupported import path kind (path imports are not supported)"
                                .to_string(),
                        );
                    }
                }
                Import::GlobalSymbol(path, _, _) => {
                    if let ImportPath::Filename(lit) = path {
                        imports.push(lit.string.clone());
                    } else {
                        push_error(
                            errors,
                            file_name,
                            "UnsupportedImportPath",
                            "unsupported import path kind (path imports are not supported)"
                                .to_string(),
                        );
                    }
                }
            }
        }

        let mut resolved: Vec<String> = Vec::new();
        for import in imports {
            if let Some(key) = resolve_import_key(file_name, &import, &available) {
                resolved.push(key);
            } else {
                push_error(
                    errors,
                    file_name,
                    "MissingImport",
                    format!("import '{import}' not found in standard JSON sources"),
                );
            }
        }
        deps.insert(file_name.clone(), resolved);
    }

    let mut visited: HashSet<String> = HashSet::new();
    let mut visiting: HashSet<String> = HashSet::new();
    let mut stack: VecDeque<String> = VecDeque::new();
    let mut order: Vec<String> = Vec::new();

    fn dfs(
        node: &str,
        deps: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        stack: &mut VecDeque<String>,
        order: &mut Vec<String>,
        _errors: &mut Vec<Value>,
    ) {
        if visited.contains(node) {
            return;
        }
        if !visiting.insert(node.to_string()) {
            // Solidity source graphs can be cyclic; this is a back-edge.
            // Do not fail import resolution on cycles.
            return;
        }

        stack.push_back(node.to_string());
        if let Some(children) = deps.get(node) {
            for dep in children {
                dfs(dep, deps, visited, visiting, stack, order, _errors);
            }
        }
        stack.pop_back();

        visiting.remove(node);
        visited.insert(node.to_string());
        order.push(node.to_string());
    }

    for (file, _, _) in sources {
        dfs(
            file,
            &deps,
            &mut visited,
            &mut visiting,
            &mut stack,
            &mut order,
            errors,
        );
    }

    fn append_sources_for_files(files: &[String], content_map: &HashMap<String, String>) -> String {
        let mut combined = String::new();
        for (idx, file) in files.iter().enumerate() {
            let Some(content) = content_map.get(file) else {
                continue;
            };
            if idx > 0 {
                combined.push_str("\n\n");
            }
            combined.push_str(&format!("// --- {file}\n"));
            combined.push_str(content);
        }
        combined
    }

    let mut parsed_contracts: HashMap<String, Vec<(String, ContractKind)>> = HashMap::new();
    for file in &order {
        let Some(content) = content_map.get(file) else {
            continue;
        };
        if let Ok(contracts) = parse_source(content) {
            let mut file_contracts = Vec::new();
            for contract in contracts {
                file_contracts.push((contract.name, contract.kind));
            }
            parsed_contracts.insert(file.clone(), file_contracts);
        }
    }

    let has_primary_contract = parsed_contracts.values().flatten().any(|(_, kind)| {
        matches!(
            kind,
            ContractKind::Contract | ContractKind::AbstractContract
        )
    });

    fn selected_contract_sources(
        files: &[String],
        parsed_contracts: &HashMap<String, Vec<(String, ContractKind)>>,
        has_primary_contract: bool,
    ) -> Vec<StandardJsonContractSource> {
        let mut contract_sources = Vec::new();
        for file in files {
            let Some(contracts) = parsed_contracts.get(file) else {
                continue;
            };
            for (contract_name, kind) in contracts {
                if has_primary_contract
                    && !matches!(
                        kind,
                        ContractKind::Contract | ContractKind::AbstractContract
                    )
                {
                    continue;
                }
                contract_sources.push(StandardJsonContractSource {
                    file_name: file.clone(),
                    contract_name: contract_name.clone(),
                });
            }
        }
        contract_sources
    }

    fn collect_closure_order(
        node: &str,
        deps: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        order: &mut Vec<String>,
    ) {
        if visited.contains(node) {
            return;
        }
        if !visiting.insert(node.to_string()) {
            return;
        }
        if let Some(children) = deps.get(node) {
            for dep in children {
                collect_closure_order(dep, deps, visited, visiting, order);
            }
        }
        visiting.remove(node);
        visited.insert(node.to_string());
        order.push(node.to_string());
    }

    let combined = append_sources_for_files(&order, &content_map);
    let contract_sources =
        selected_contract_sources(&order, &parsed_contracts, has_primary_contract);
    let source_units = sources
        .iter()
        .map(|(file, _, _)| {
            let mut visited = HashSet::new();
            let mut visiting = HashSet::new();
            let mut unit_order = Vec::new();
            collect_closure_order(file, &deps, &mut visited, &mut visiting, &mut unit_order);

            CombinedStandardJsonSourceUnit {
                root_file: file.clone(),
                source: append_sources_for_files(&unit_order, &content_map),
                contract_sources: selected_contract_sources(
                    &unit_order,
                    &parsed_contracts,
                    has_primary_contract,
                ),
            }
        })
        .collect();

    CombinedStandardJsonSource {
        source: combined,
        contract_sources,
        source_units,
    }
}
