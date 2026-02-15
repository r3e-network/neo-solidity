use solang_parser::pt::{Import, ImportPath, SourceUnitPart};
use std::collections::VecDeque;
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct ResolvedSoliditySources {
    files: Vec<(PathBuf, String)>,
    combined_source: String,
}

fn resolve_solidity_sources_with_imports(
    entry_file: &Path,
    include_paths: &[PathBuf],
) -> Result<ResolvedSoliditySources, String> {
    let mut visited: HashSet<PathBuf> = HashSet::new();
    let mut visiting: HashSet<PathBuf> = HashSet::new();
    let mut ordered: Vec<(PathBuf, String)> = Vec::new();
    let mut stack: VecDeque<PathBuf> = VecDeque::new();

    fn extract_imports(source: &str, file: &Path) -> Result<Vec<String>, String> {
        fn offset_to_line_column(source: &str, offset: usize) -> (usize, usize) {
            let mut line = 1usize;
            let mut column = 1usize;
            let mut current = 0usize;

            for ch in source.chars() {
                if current >= offset {
                    break;
                }

                if ch == '\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }

                current += ch.len_utf8();
            }

            (line, column)
        }

        let (unit, _comments) = solang_parser::parse(source, 0).map_err(|diags| {
            let summary = diags
                .iter()
                .map(|diag| {
                    if let solang_parser::pt::Loc::File(_, start, _) = diag.loc {
                        let (line, column) = offset_to_line_column(source, start);
                        format!("{}:{}: {}", line, column, diag.message)
                    } else {
                        diag.message.clone()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "failed to parse '{}' while resolving imports:\n{}",
                file.display(),
                summary
            )
        })?;

        let mut imports = Vec::new();
        for part in unit.0.iter() {
            let SourceUnitPart::ImportDirective(import) = part else {
                continue;
            };

            match import {
                Import::Plain(path, _) => {
                    imports.push(extract_import_path_string(path, file)?);
                }
                Import::Rename(path, _renames, _) => {
                    imports.push(extract_import_path_string(path, file)?);
                }
                Import::GlobalSymbol(path, _, _) => {
                    imports.push(extract_import_path_string(path, file)?);
                }
            }
        }

        Ok(imports)
    }

    fn extract_import_path_string(path: &ImportPath, file: &Path) -> Result<String, String> {
        match path {
            ImportPath::Filename(lit) => Ok(lit.string.clone()),
            ImportPath::Path(_) => Err(format!(
                "unsupported import path kind in '{}': path imports are not supported",
                file.display()
            )),
        }
    }

    fn resolve_import_file(
        import_path: &str,
        from_file: &Path,
        include_paths: &[PathBuf],
    ) -> Result<PathBuf, String> {
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

        let mut candidates: Vec<PathBuf> = Vec::new();
        for candidate_import in import_aliases(import_path) {
            let import = Path::new(&candidate_import);
            if import.is_absolute() {
                candidates.push(import.to_path_buf());
            } else {
                let from_dir = from_file.parent().unwrap_or_else(|| Path::new("."));
                candidates.push(from_dir.join(import));
                for include_dir in include_paths {
                    candidates.push(include_dir.join(import));
                }
                candidates.push(import.to_path_buf());
            }
        }

        for candidate in candidates {
            if candidate.exists() {
                return Ok(candidate.canonicalize().unwrap_or(candidate));
            }
        }

        Err(format!(
            "failed to resolve import '{import_path}' from '{}'",
            from_file.display()
        ))
    }

    fn visit_file(
        file: &Path,
        include_paths: &[PathBuf],
        visited: &mut HashSet<PathBuf>,
        visiting: &mut HashSet<PathBuf>,
        ordered: &mut Vec<(PathBuf, String)>,
        stack: &mut VecDeque<PathBuf>,
    ) -> Result<(), String> {
        let canonical = file.canonicalize().unwrap_or_else(|_| file.to_path_buf());

        if visited.contains(&canonical) {
            return Ok(());
        }

        if !visiting.insert(canonical.clone()) {
            // Solidity allows cyclic import graphs as long as symbols resolve.
            // Skip this back-edge and let the already-in-progress unit finish.
            return Ok(());
        }

        stack.push_back(canonical.clone());
        let content = fs::read_to_string(&canonical)
            .map_err(|err| format!("failed to read '{}': {err}", canonical.display()))?;

        let imports = extract_imports(&content, &canonical)?;
        for import in imports {
            let resolved = resolve_import_file(&import, &canonical, include_paths)?;
            visit_file(
                &resolved,
                include_paths,
                visited,
                visiting,
                ordered,
                stack,
            )?;
        }

        stack.pop_back();
        visiting.remove(&canonical);
        visited.insert(canonical.clone());
        ordered.push((canonical, content));
        Ok(())
    }

    visit_file(
        entry_file,
        include_paths,
        &mut visited,
        &mut visiting,
        &mut ordered,
        &mut stack,
    )?;

    let mut combined = String::new();
    for (idx, (path, content)) in ordered.iter().enumerate() {
        if idx > 0 {
            combined.push_str("\n\n");
        }
        combined.push_str(&format!("// --- {}\n", path.display()));
        combined.push_str(content);
    }

    Ok(ResolvedSoliditySources {
        files: ordered,
        combined_source: combined,
    })
}
