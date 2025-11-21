use clap::{Arg, ArgAction, Command};
use neo_solidity::ir;
use neo_solidity::neo::build_nef;
use neo_solidity::semantic_model::build_semantic_model;
use neo_solidity::solidity::{
    analyse_all_sources, validate_contract, ContractMetadata, DiagnosticSeverity, FunctionKind,
    FunctionMetadata,
};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

mod bytecode;
pub(crate) use bytecode::generate_contract_bytecode;
#[cfg(test)]
pub(crate) use bytecode::interop_id_bytes;
mod standard_json;
pub(crate) use standard_json::*;

const COMPILER_ID: &str = "neo-solidity-0.9.8";
const COMPILER_EMAIL: &str = "Jimmy <jimmy@r3e.network>";
const VERSION: (u32, u32, u32, u32) = (0, 9, 8, 0);

#[derive(Clone, Debug)]
pub(crate) struct CompilationArtifacts {
    metadata: ContractMetadata,
    bytecode: Vec<u8>,
    manifest: Value,
    warnings: Vec<neo_solidity::solidity::Diagnostic>,
}

#[derive(Debug)]
enum CompileError {
    Diagnostics(Vec<neo_solidity::solidity::Diagnostic>),
    Semantic(Vec<neo_solidity::solidity::Diagnostic>),
    Ir(Vec<String>),
    Message(String),
}

pub fn run() {
    let matches = Command::new("neo-solc")
        .version("0.9.8")
        .author(COMPILER_EMAIL)
        .about("Compiles Solidity to Neo N3 smart contracts (.nef + .manifest.json)")
        .arg(
            Arg::new("source")
                .help("Input Solidity file")
                .required_unless_present("standard-json")
                .index(1),
        )
        .arg(
            Arg::new("standard-json")
                .long("standard-json")
                .help("Use Solidity standard JSON input/output mode")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("standard-json-input")
                .long("input")
                .value_name("FILE")
                .help("Path to standard JSON input file")
                .num_args(1)
                .requires("standard-json"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file prefix (generates .nef and .manifest.json)")
                .num_args(1),
        )
        .arg(
            Arg::new("optimize")
                .short('O')
                .long("optimize")
                .value_name("LEVEL")
                .help("Optimization level (0-3)")
                .num_args(1)
                .default_value("2"),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Output format")
                .num_args(1)
                .value_parser(["nef", "manifest", "complete", "assembly", "json"])
                .default_value("complete"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Verbose output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let use_standard_json = matches.get_flag("standard-json");
    if use_standard_json {
        let input_path = matches
            .get_one::<String>("standard-json-input")
            .expect("Standard JSON input file is required");
        let output_path = matches.get_one::<String>("output").map(|s| s.as_str());
        if let Err(err) = process_standard_json(input_path, output_path) {
            eprintln!("{err}");
            std::process::exit(1);
        }
        return;
    }

    let input_file = matches
        .get_one::<String>("source")
        .expect("Input Solidity file is required");
    let output_prefix = matches
        .get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or_else(|| {
            Path::new(input_file)
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("contract")
        });

    let format = matches.get_one::<String>("format").unwrap();
    let verbose = matches.get_flag("verbose");

    if verbose {
        println!("Neo Solidity Compiler v0.9.8");
        println!("Input: {}", input_file);
        println!("Output prefix: {}", output_prefix);
        println!("Format: {}", format);
    }

    let input_content = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading input file: {err}");
            std::process::exit(1);
        }
    };

    if verbose {
        println!("Read {} bytes from input file", input_content.len());
    }

    let artifacts = match compile_contracts(&input_content, verbose) {
        Ok(list) => list,
        Err(CompileError::Diagnostics(diags)) | Err(CompileError::Semantic(diags)) => {
            for diag in diags {
                match diag.severity {
                    DiagnosticSeverity::Warning => eprintln!("warning: {}", diag.message),
                    DiagnosticSeverity::Error => eprintln!("error: {}", diag.message),
                }
            }
            std::process::exit(1);
        }
        Err(CompileError::Ir(errors)) => {
            for error in errors {
                eprintln!("error: {}", error);
            }
            std::process::exit(1);
        }
        Err(CompileError::Message(message)) => {
            eprintln!("error: {}", message);
            std::process::exit(1);
        }
    };

    if artifacts.is_empty() {
        if verbose {
            eprintln!("warning: No contracts were found in the input file.");
        }
        return;
    }

    if artifacts.len() > 1 {
        println!(
            "(info) detected {} contracts – outputs are suffixed with their contract names",
            artifacts.len()
        );
    }

    for artifact in &artifacts {
        for warning in &artifact.warnings {
            eprintln!("warning ({}): {}", artifact.metadata.name, warning.message);
        }
    }

    for (index, artifact) in artifacts.iter().enumerate() {
        let prefix = contract_output_prefix(
            output_prefix,
            &artifact.metadata.name,
            index,
            artifacts.len(),
        );

        match format.as_str() {
            "nef" => {
                let nef_path = if prefix.ends_with(".nef") {
                    prefix.clone()
                } else {
                    format!("{prefix}.nef")
                };
                write_nef_file(&nef_path, &artifact.bytecode);
                println!(
                    "✅ [{}] NEF file generated: {nef_path}",
                    artifact.metadata.name
                );
            }
            "manifest" => {
                let manifest_path = if prefix.ends_with(".manifest.json") {
                    prefix.clone()
                } else {
                    format!("{prefix}.manifest.json")
                };
                write_manifest_file(&manifest_path, &artifact.manifest);
                println!(
                    "✅ [{}] Manifest file generated: {manifest_path}",
                    artifact.metadata.name
                );
            }
            "complete" => {
                let nef_path = format!("{prefix}.nef");
                let manifest_path = format!("{prefix}.manifest.json");
                write_nef_file(&nef_path, &artifact.bytecode);
                write_manifest_file(&manifest_path, &artifact.manifest);
                println!(
                    "✅ [{}] Contract files generated:\n   📄 {nef_path}\n   📄 {manifest_path}",
                    artifact.metadata.name
                );
            }
            "json" => {
                let json_path = if prefix.ends_with(".json") {
                    prefix.clone()
                } else {
                    format!("{prefix}.json")
                };
                write_json_file(
                    &json_path,
                    &artifact.bytecode,
                    &artifact.manifest,
                    &artifact.metadata,
                );
                println!(
                    "✅ [{}] JSON file generated: {json_path}",
                    artifact.metadata.name
                );
            }
            "assembly" => {
                println!("Assembly output is not yet implemented.");
            }
            other => {
                println!("Unsupported format: {other}");
            }
        }
    }

    println!("🎉 Neo Solidity compilation completed");
}

fn compile_contracts(
    source: &str,
    verbose: bool,
) -> Result<Vec<CompilationArtifacts>, CompileError> {
    let metadatas =
        analyse_all_sources(source).map_err(|err| CompileError::Message(err.to_string()))?;

    let mut outputs = Vec::new();
    for metadata in metadatas {
        outputs.push(compile_metadata(metadata, verbose)?);
    }

    Ok(outputs)
}

fn compile_metadata(
    mut metadata: ContractMetadata,
    verbose: bool,
) -> Result<CompilationArtifacts, CompileError> {
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

    let ir_module = ir::Module::from_contract(&metadata).map_err(CompileError::Ir)?;

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

    let bytecode = generate_contract_bytecode(&mut metadata, &ir_module, verbose);
    let manifest = build_manifest(&metadata);

    Ok(CompilationArtifacts {
        metadata,
        bytecode,
        manifest,
        warnings,
    })
}

impl CompileError {
    fn into_errors(self, file: &str) -> Vec<Value> {
        match self {
            CompileError::Diagnostics(diags) | CompileError::Semantic(diags) => diags
                .into_iter()
                .map(|diag| standard_json::diagnostic_to_standard_error(&diag, file))
                .collect(),
            CompileError::Ir(errors) => errors
                .into_iter()
                .map(|message| {
                    json!({
                        "component": "neo-solidity",
                        "severity": "error",
                        "type": "IrGeneration",
                        "sourceLocation": { "file": file },
                        "formattedMessage": message,
                        "message": message,
                    })
                })
                .collect(),
            CompileError::Message(message) => vec![json!({
                "component": "neo-solidity",
                "severity": "error",
                "type": "Generic",
                "sourceLocation": { "file": file },
                "formattedMessage": message,
                "message": message,
            })],
        }
    }
}

fn contract_output_prefix(base: &str, contract_name: &str, index: usize, total: usize) -> String {
    if total <= 1 {
        return base.to_string();
    }

    let sanitized = standard_json::sanitize_contract_name(contract_name).unwrap_or_else(|| {
        if total <= 1 {
            "contract".to_string()
        } else {
            format!("contract{index}")
        }
    });

    let (stem, ext) = split_extension(base);
    if ext.is_empty() {
        format!("{stem}-{sanitized}")
    } else {
        format!("{stem}-{sanitized}{ext}")
    }
}

fn split_extension(path: &str) -> (String, String) {
    match path.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => (stem.to_string(), format!(".{ext}")),
        _ => (path.to_string(), String::new()),
    }
}

fn write_nef_file(path: &str, script: &[u8]) {
    let nef = build_nef(script, COMPILER_ID, VERSION);
    fs::write(path, nef).expect("Failed to write NEF file");
}

fn write_manifest_file(path: &str, manifest: &serde_json::Value) {
    let manifest_str =
        serde_json::to_string_pretty(manifest).expect("Manifest serialization failed");
    fs::write(path, manifest_str).expect("Failed to write manifest file");
}

fn write_json_file(
    path: &str,
    script: &[u8],
    manifest: &serde_json::Value,
    metadata: &ContractMetadata,
) {
    let json_output = json!({
        "contract": metadata.name,
        "compiler": COMPILER_ID,
        "author": COMPILER_EMAIL,
        "nef": {
            "magic": "NEF3",
            "compiler": COMPILER_ID,
            "version": format!("{}.{}.{}.{}", VERSION.0, VERSION.1, VERSION.2, VERSION.3),
            "script": hex::encode(script),
        },
        "manifest": manifest,
    });

    let json_str =
        serde_json::to_string_pretty(&json_output).expect("Failed to serialise JSON output");
    fs::write(path, json_str).expect("Failed to write JSON file");
}

fn build_manifest(metadata: &ContractMetadata) -> serde_json::Value {
    let methods_json: Vec<_> = metadata
        .methods
        .iter()
        .filter(|method| !matches!(method.kind, FunctionKind::Constructor))
        .map(|method| {
            let params_json: Vec<_> = method
                .parameters
                .iter()
                .enumerate()
                .map(|(param_index, param)| {
                    json!({
                        "name": param
                            .name
                            .clone()
                            .unwrap_or_else(|| format!("arg{}", param_index)),
                        "type": standard_json::solidity_to_manifest_type(&param.ty),
                    })
                })
                .collect();

            json!({
                "name": method.name,
                "offset": method.offset,
                "parameters": params_json,
                "returntype": method
                    .return_parameters
                    .first()
                    .map(|param| standard_json::solidity_to_manifest_type(&param.ty))
                    .unwrap_or("Void"),
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
                            .unwrap_or_else(|| format!("param{}", idx)),
                        "type": standard_json::solidity_to_manifest_type(&param.ty),
                        "indexed": param.indexed,
                    })
                })
                .collect();

            json!({
                "name": event.name,
                "parameters": params,
            })
        })
        .collect();

    let supported_standards = detect_supported_standards(&metadata.methods);

    json!({
        "name": metadata.name,
        "groups": [],
        "features": {
            "storage": metadata.uses_storage,
        },
        "supportedstandards": supported_standards,
        "abi": {
            "methods": methods_json,
            "events": events_json,
        },
        "permissions": [
            {
                "contract": "*",
                "methods": "*"
            }
        ],
        "trusts": [],
        "extra": {
            "Author": COMPILER_EMAIL,
            "Description": format!("Solidity contract '{}' compiled to NeoVM", metadata.name),
            "Version": format!("{}.{}.{}.{}", VERSION.0, VERSION.1, VERSION.2, VERSION.3),
            "Compiler": COMPILER_ID,
        }
    })
}

fn detect_supported_standards(methods: &[FunctionMetadata]) -> Vec<String> {
    let names: HashSet<String> = methods
        .iter()
        .filter(|m| !matches!(m.kind, FunctionKind::Constructor))
        .map(|m| m.name.to_ascii_lowercase())
        .collect();
    let mut standards = Vec::new();

    if names.contains("transfer") && names.contains("balanceof") && names.contains("totalsupply") {
        standards.push("NEP-17".to_string());
    }

    if names.contains("ownerof") && names.contains("transferfrom") {
        standards.push("NEP-11".to_string());
    }

    if names.contains("symbol") && names.contains("decimals") && names.contains("tokensupply") {
        standards.push("NEP-24".to_string());
    }

    standards
}

#[cfg(test)]
mod tests;
