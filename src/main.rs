use clap::{Arg, ArgAction, Command};
use neo_solidity::ir::{self, LiteralValue, ValueType};
use neo_solidity::neo::build_nef;
use neo_solidity::semantic_model::build_semantic_model;
use neo_solidity::solidity::{
    analyse_all_sources, validate_contract, ContractMetadata, DiagnosticSeverity, FunctionKind,
    FunctionMetadata, StateMutability,
};
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

const COMPILER_ID: &str = "neo-solidity-1.0.0";
const COMPILER_EMAIL: &str = "Jimmy <jimmy@r3e.network>";
const VERSION: (u32, u32, u32, u32) = (1, 0, 0, 0);

#[derive(Clone)]
struct CompilationArtifacts {
    metadata: ContractMetadata,
    bytecode: Vec<u8>,
    manifest: Value,
    warnings: Vec<neo_solidity::solidity::Diagnostic>,
}

#[derive(Deserialize)]
struct StandardJsonInput {
    language: String,
    sources: HashMap<String, StandardJsonSource>,
    #[serde(default)]
    settings: Value,
}

#[derive(Deserialize)]
struct StandardJsonSource {
    content: Option<String>,
    urls: Option<Vec<String>>,
}

enum CompileError {
    Diagnostics(Vec<neo_solidity::solidity::Diagnostic>),
    Semantic(Vec<neo_solidity::solidity::Diagnostic>),
    Ir(Vec<String>),
    Message(String),
}

fn main() {
    let matches = Command::new("neo-solc")
        .version("1.0.0")
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
        println!("Neo Solidity Compiler v1.0.0");
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
        eprintln!("error: No contracts were found in the input file.");
        std::process::exit(1);
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

fn process_standard_json(input_path: &str, output_path: Option<&str>) -> Result<(), String> {
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

    let mut contracts_output = serde_json::Map::new();
    let mut sources_output = serde_json::Map::new();
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
                    .or_insert_with(|| Value::Object(serde_json::Map::new()));
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

    let mut output = serde_json::Map::new();
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
                .map(|diag| diagnostic_to_standard_error(&diag, file))
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

fn diagnostic_to_standard_error(
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

fn build_standard_abi(metadata: &ContractMetadata) -> Vec<Value> {
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

fn build_compiled_contract_value(
    file_name: &str,
    artifact: &CompilationArtifacts,
    abi_entries: &[Value],
    settings: &Value,
) -> Value {
    let script_hex = hex::encode(&artifact.bytecode);
    let bytecode_object = format!("0x{script_hex}");
    let metadata_blob =
        build_metadata_blob(&artifact.metadata.name, abi_entries, file_name, settings);

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
            "methodIdentifiers": {}
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
                "functions": Value::Object(serde_json::Map::new())
            }
        }
    })
}

fn build_metadata_blob(
    contract_name: &str,
    abi_entries: &[Value],
    file_name: &str,
    settings: &Value,
) -> String {
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
                "keccak256": "",
                "urls": []
            }
        },
        "version": 1
    });

    serde_json::to_string(&metadata).unwrap_or_else(|_| "{}".to_string())
}

fn build_storage_map(metadata: &ContractMetadata) -> Value {
    let mut entries = serde_json::Map::new();
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

fn zero_gas_estimate_value() -> Value {
    json!({
        "gas": "0",
        "systemFee": "0",
        "networkFee": "0",
    })
}

fn state_mutability_label(state: StateMutability) -> &'static str {
    match state {
        StateMutability::Pure => "pure",
        StateMutability::View => "view",
        StateMutability::Payable => "payable",
        StateMutability::NonPayable => "nonpayable",
    }
}

fn contract_output_prefix(base: &str, contract_name: &str, index: usize, total: usize) -> String {
    if total <= 1 {
        return base.to_string();
    }

    let sanitized = sanitize_contract_name(contract_name).unwrap_or_else(|| {
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

fn sanitize_contract_name(name: &str) -> Option<String> {
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

struct CallPatch {
    position: usize,
    target: String,
}

fn generate_contract_bytecode(
    metadata: &mut ContractMetadata,
    ir_module: &ir::Module,
    verbose: bool,
) -> Vec<u8> {
    let function_map: HashMap<_, _> = ir_module
        .functions
        .iter()
        .map(|function| (function.name.as_str(), function))
        .collect();

    let mut bytecode = Vec::new();
    let mut call_fixups: Vec<CallPatch> = Vec::new();

    for method in metadata.methods.iter_mut() {
        if matches!(method.kind, FunctionKind::Constructor) {
            continue;
        }

        let method_name = method.name.clone();
        let ir_function = match function_map.get(method_name.as_str()) {
            Some(func) => *func,
            None => {
                eprintln!(
                    "error: internal compiler error: missing IR for method '{}'",
                    method_name
                );
                std::process::exit(1);
            }
        };

        let instruction_count: usize = ir_function
            .basic_blocks
            .iter()
            .map(|block| block.instructions.len())
            .sum();

        let offset = bytecode.len() as u32;
        method.offset = offset;

        if verbose {
            println!(
                "  • Emitting method '{}' at offset {} ({} IR instruction(s))",
                method_name, offset, instruction_count
            );
        }

        let (function_bytes, patches) = emit_ir_function(ir_function, ir_module, method);
        let base_position = bytecode.len();
        bytecode.extend_from_slice(&function_bytes);

        for patch in patches {
            call_fixups.push(CallPatch {
                position: base_position + patch.position,
                target: patch.target,
            });
        }
    }

    let offset_map: HashMap<String, u32> = metadata
        .methods
        .iter()
        .filter(|method| !matches!(method.kind, FunctionKind::Constructor))
        .map(|method| (method.name.clone(), method.offset))
        .collect();

    for fixup in call_fixups {
        if let Some(target_offset) = offset_map.get(&fixup.target) {
            let bytes = target_offset.to_le_bytes();
            if fixup.position + 4 <= bytecode.len() {
                bytecode[fixup.position..fixup.position + 4].copy_from_slice(&bytes);
            }
        } else {
            eprintln!(
                "warning: unresolved call target '{}' (offset unavailable)",
                fixup.target
            );
        }
    }

    if bytecode.is_empty() {
        bytecode.push(0x40); // RET
    }

    bytecode
}

fn emit_ir_function(
    function: &ir::Function,
    module: &ir::Module,
    method: &FunctionMetadata,
) -> (Vec<u8>, Vec<CallPatch>) {
    use std::{collections::HashMap, convert::TryFrom};

    let mut local = Vec::new();
    let arg_count = method.parameters.len() as u8;
    let local_count = u8::try_from(function.local_count).unwrap_or(u8::MAX);
    if local_count > 0 || arg_count > 0 {
        local.push(0x57); // INITSLOT
        local.push(local_count);
        local.push(arg_count);
    }
    let mut label_offsets: HashMap<usize, u32> = HashMap::new();
    let mut jump_patches: Vec<(usize, usize)> = Vec::new();
    let mut call_patches: Vec<CallPatch> = Vec::new();

    for block in &function.basic_blocks {
        for instruction in &block.instructions {
            match instruction {
                ir::Instruction::Drop(_) => local.push(0x45),
                ir::Instruction::LoadParameter(index) => {
                    emit_load_parameter(&mut local, method, *index)
                }
                ir::Instruction::PushLiteral(literal) => {
                    push_literal_value(&mut local, literal);
                }
                ir::Instruction::BinaryOp(operator) => emit_binary_op(&mut local, *operator),
                ir::Instruction::Return | ir::Instruction::ReturnVoid => local.push(0x40),
                ir::Instruction::ReturnDefault(value_type) => {
                    append_default_value(&mut local, value_type);
                    local.push(0x40);
                }
                ir::Instruction::LoadState(index) => emit_load_state(&mut local, module, *index),
                ir::Instruction::StoreState(index) => emit_store_state(&mut local, module, *index),
                ir::Instruction::LoadLocal(index) => emit_load_local(&mut local, *index),
                ir::Instruction::StoreLocal(index) => emit_store_local(&mut local, *index),
                ir::Instruction::LoadMappingElement {
                    state_index,
                    key_types,
                } => emit_load_mapping(&mut local, module, *state_index, key_types),
                ir::Instruction::StoreMappingElement {
                    state_index,
                    key_types,
                } => emit_store_mapping(&mut local, module, *state_index, key_types),
                ir::Instruction::LoadStructField {
                    state_index,
                    key_types,
                    field_key,
                    ..
                } => emit_load_struct_field(&mut local, module, *state_index, key_types, field_key),
                ir::Instruction::StoreStructField {
                    state_index,
                    key_types,
                    field_key,
                    ..
                } => {
                    emit_store_struct_field(&mut local, module, *state_index, key_types, field_key)
                }
                ir::Instruction::LoadRuntimeValue(value) => {
                    emit_load_runtime_value(&mut local, value)
                }
                ir::Instruction::GetSize => local.push(0x5A),
                ir::Instruction::CallBuiltin { builtin, .. } => {
                    emit_builtin_call(&mut local, builtin);
                }
                ir::Instruction::CallFunction { name, .. } => {
                    local.push(0x2B); // CALL
                    let patch_pos = local.len();
                    local.extend_from_slice(&[0, 0, 0, 0]);
                    call_patches.push(CallPatch {
                        position: patch_pos,
                        target: name.clone(),
                    });
                }
                ir::Instruction::EmitEvent { event_index } => {
                    emit_event(&mut local, module, *event_index)
                }
                ir::Instruction::Jump { target } => {
                    local.push(0x22); // JMP
                    let position = local.len();
                    local.extend_from_slice(&[0, 0, 0, 0]);
                    jump_patches.push((position, *target));
                }
                ir::Instruction::JumpIf { target } => {
                    local.push(0x24); // JMPIFNOT
                    let position = local.len();
                    local.extend_from_slice(&[0, 0, 0, 0]);
                    jump_patches.push((position, *target));
                }
                ir::Instruction::Label(label) => {
                    label_offsets.insert(*label, local.len() as u32);
                }
                ir::Instruction::Abort => {
                    local.push(0x2E); // ABORT
                }
            }
        }
    }

    for (position, label) in jump_patches {
        let target_offset = label_offsets
            .get(&label)
            .copied()
            .unwrap_or(local.len() as u32);
        local[position..position + 4].copy_from_slice(&target_offset.to_le_bytes());
    }

    (local, call_patches)
}

fn append_default_value(bytecode: &mut Vec<u8>, value_type: &ValueType) {
    match value_type {
        ValueType::Integer { .. } => bytecode.push(0x10),
        ValueType::Boolean => bytecode.push(0x10),
        ValueType::String => push_data(bytecode, &[]),
        ValueType::Address => push_data(bytecode, &[0u8; 20]),
        ValueType::ByteArray { fixed_len } => {
            if let Some(len) = fixed_len {
                let zeros = vec![0u8; *len as usize];
                push_data(bytecode, &zeros);
            } else {
                push_data(bytecode, &[]);
            }
        }
        ValueType::Array(_) => bytecode.push(0xC4),
        ValueType::Mapping { .. } => bytecode.push(0x0B),
        ValueType::Struct { .. } => bytecode.push(0x0B),
        ValueType::Any => bytecode.push(0x0B),
    }
}

fn emit_load_parameter(bytecode: &mut Vec<u8>, _method: &FunctionMetadata, index: usize) {
    if index <= 6 {
        bytecode.push(0x78 + index as u8);
    } else {
        bytecode.push(0x7F); // LDARG
        bytecode.push(index as u8);
    }
}

fn emit_load_local(bytecode: &mut Vec<u8>, index: usize) {
    match index {
        0..=6 => bytecode.push(0x68 + index as u8),
        _ => {
            bytecode.push(0x6F); // LDLOC
            bytecode.push(index as u8);
        }
    }
}

fn emit_store_local(bytecode: &mut Vec<u8>, index: usize) {
    match index {
        0..=6 => bytecode.push(0x70 + index as u8),
        _ => {
            bytecode.push(0x77); // STLOC
            bytecode.push(index as u8);
        }
    }
}

fn emit_binary_op(bytecode: &mut Vec<u8>, operator: ir::BinaryOperator) {
    let opcode = match operator {
        ir::BinaryOperator::Add => 0x95,
        ir::BinaryOperator::Sub => 0x96,
        ir::BinaryOperator::Mul => 0x97,
        ir::BinaryOperator::Div => 0x98,
        ir::BinaryOperator::Mod => 0x99,
        ir::BinaryOperator::Lt => 0xA5,
        ir::BinaryOperator::Le => 0xA6,
        ir::BinaryOperator::Gt => 0xA7,
        ir::BinaryOperator::Ge => 0xA8,
        ir::BinaryOperator::Eq => 0xA3,
        ir::BinaryOperator::Ne => 0xA4,
    };
    bytecode.push(opcode);
}

fn push_literal_value(bytecode: &mut Vec<u8>, literal: &LiteralValue) {
    match literal {
        LiteralValue::Integer(value) => push_integer_bigint(bytecode, value),
        LiteralValue::Boolean(true) => bytecode.push(0x11),
        LiteralValue::Boolean(false) => bytecode.push(0x10),
        LiteralValue::String(bytes) => push_data(bytecode, bytes),
        LiteralValue::ByteArray(bytes) => push_data(bytecode, bytes),
        LiteralValue::Address(bytes) => push_data(bytecode, bytes),
    }
}

fn push_integer_bigint(bytecode: &mut Vec<u8>, value: &BigInt) {
    if value.is_zero() {
        bytecode.push(0x10);
        return;
    }

    if *value == BigInt::from(-1) {
        bytecode.push(0x0F);
        return;
    }

    if value.is_positive() {
        if let Some(n) = value.to_u8() {
            if n <= 16 {
                bytecode.push(0x10 + n);
                return;
            }
        }
    }

    let mut bytes = value.to_signed_bytes_le();
    if bytes.is_empty() {
        bytes.push(0);
    }
    push_data(bytecode, &bytes);
}

fn emit_load_state(bytecode: &mut Vec<u8>, module: &ir::Module, index: usize) {
    let key = module
        .state_variables
        .get(index)
        .map(|state| state.storage_key.as_slice())
        .unwrap_or(&[]);

    emit_syscall(bytecode, "System.Storage.GetContext");
    push_data(bytecode, key);
    emit_syscall(bytecode, "System.Storage.Get");
}

fn emit_store_state(bytecode: &mut Vec<u8>, module: &ir::Module, index: usize) {
    let key = module
        .state_variables
        .get(index)
        .map(|state| state.storage_key.as_slice())
        .unwrap_or(&[]);

    push_data(bytecode, key);
    emit_syscall(bytecode, "System.Storage.GetContext");
    bytecode.push(0x51); // ROT
    bytecode.push(0x50); // SWAP
    emit_syscall(bytecode, "System.Storage.Put");
}

fn emit_serialize_key(bytecode: &mut Vec<u8>, _key_type: &ValueType) {
    emit_syscall(bytecode, "System.Runtime.Serialize");
}

fn emit_mapping_slot(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
) {
    let base_slot = module
        .state_variables
        .get(state_index)
        .map(|state| state.storage_key.clone())
        .unwrap_or_else(|| vec![0u8; 32]);

    push_data(bytecode, &base_slot);

    for key_type in key_types {
        bytecode.push(0x50); // swap slot <-> key
        emit_serialize_key(bytecode, key_type);
        bytecode.push(0x50); // swap key_bytes <-> slot
        bytecode.push(0x8B); // concatenate key and slot
        emit_syscall(bytecode, "System.Crypto.SHA256");
    }
}

fn emit_struct_field_slot(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    field_key: &[u8; 32],
) {
    emit_mapping_slot(bytecode, module, state_index, key_types);
    push_data(bytecode, field_key);
    bytecode.push(0x50); // swap slot and field key bytes
    bytecode.push(0x8B); // concatenate
    emit_syscall(bytecode, "System.Crypto.SHA256");
}

fn emit_load_mapping(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
) {
    emit_mapping_slot(bytecode, module, state_index, key_types);
    emit_syscall(bytecode, "System.Storage.GetContext");
    bytecode.push(0x50); // SWAP
    emit_syscall(bytecode, "System.Storage.Get");
}

fn emit_store_mapping(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
) {
    emit_mapping_slot(bytecode, module, state_index, key_types);
    emit_syscall(bytecode, "System.Storage.GetContext");
    bytecode.push(0x50); // swap slot and context -> [value, context, slot]
    bytecode.push(0x51); // ROT -> [context, slot, value]
    emit_syscall(bytecode, "System.Storage.Put");
}

fn emit_load_struct_field(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    field_key: &[u8; 32],
) {
    emit_struct_field_slot(bytecode, module, state_index, key_types, field_key);
    emit_syscall(bytecode, "System.Storage.GetContext");
    bytecode.push(0x50); // swap context and slot
    emit_syscall(bytecode, "System.Storage.Get");
}

fn emit_store_struct_field(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    field_key: &[u8; 32],
) {
    emit_struct_field_slot(bytecode, module, state_index, key_types, field_key);
    emit_syscall(bytecode, "System.Storage.GetContext");
    bytecode.push(0x50); // swap slot and context -> [value, context, slot]
    bytecode.push(0x51); // ROT -> [context, slot, value]
    emit_syscall(bytecode, "System.Storage.Put");
}

fn emit_load_runtime_value(bytecode: &mut Vec<u8>, value: &ir::RuntimeValue) {
    match value {
        ir::RuntimeValue::MsgSender => emit_syscall(bytecode, "System.Runtime.CallingScriptHash"),
        ir::RuntimeValue::BlockTimestamp => emit_syscall(bytecode, "System.Runtime.GetTime"),
    }
}

fn emit_builtin_call(bytecode: &mut Vec<u8>, builtin: &ir::BuiltinCall) {
    match builtin {
        ir::BuiltinCall::RuntimeNotify => emit_syscall(bytecode, "System.Runtime.Notify"),
        ir::BuiltinCall::RuntimeCheckWitness => {
            emit_syscall(bytecode, "System.Runtime.CheckWitness")
        }
        ir::BuiltinCall::Keccak256
        | ir::BuiltinCall::StorageFind
        | ir::BuiltinCall::TypeOf
        | ir::BuiltinCall::AbiEncode
        | ir::BuiltinCall::AbiEncodePacked
        | ir::BuiltinCall::AbiEncodeWithSignature
        | ir::BuiltinCall::AbiDecode => {}
    }
}

fn emit_event(bytecode: &mut Vec<u8>, module: &ir::Module, index: usize) {
    if let Some(event) = module.events.get(index) {
        push_data(bytecode, event.name.as_bytes());
        emit_syscall(bytecode, "System.Runtime.Log");
    }
}

fn emit_syscall(bytecode: &mut Vec<u8>, name: &str) {
    bytecode.push(0x41);
    bytecode.extend_from_slice(&interop_id_bytes(name));
}

fn interop_id_bytes(name: &str) -> [u8; 4] {
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    let digest = hasher.finalize();
    [digest[0], digest[1], digest[2], digest[3]]
}

fn push_data(bytecode: &mut Vec<u8>, data: &[u8]) {
    if data.len() <= u8::MAX as usize {
        bytecode.push(0x0C);
        bytecode.push(data.len() as u8);
    } else if data.len() <= u16::MAX as usize {
        bytecode.push(0x0D);
        bytecode.extend_from_slice(&(data.len() as u16).to_le_bytes());
    } else {
        bytecode.push(0x0E);
        bytecode.extend_from_slice(&(data.len() as u32).to_le_bytes());
    }
    bytecode.extend_from_slice(data);
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
                        "type": solidity_to_manifest_type(&param.ty),
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
                    .map(|param| solidity_to_manifest_type(&param.ty))
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
                        "type": solidity_to_manifest_type(&param.ty),
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

fn solidity_to_manifest_type(solidity_type: &str) -> &'static str {
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
mod tests {
    use super::*;
    use neo_solidity::solidity::analyse_source;

    #[test]
    fn mapping_code_generation_emits_storage_ops() {
        let source = r#"
        pragma solidity ^0.8.19;

        contract MappingExample {
            mapping(address => uint256) public balances;

            function setBalance(address owner, uint256 amount) public {
                balances[owner] = amount;
            }

            function getBalance(address owner) public view returns (uint256) {
                return balances[owner];
            }
        }
        "#;

        let mut metadata = analyse_source(source).expect("analysis failed");
        let ir_module = ir::Module::from_contract(&metadata).expect("IR lowering failed");
        let bytecode = generate_contract_bytecode(&mut metadata, &ir_module, false);

        assert!(!bytecode.is_empty());

        let sha_id = interop_id_bytes("System.Crypto.SHA256");
        assert!(bytecode.windows(4).any(|window| window == sha_id));

        let put_id = interop_id_bytes("System.Storage.Put");
        assert!(bytecode.windows(4).any(|window| window == put_id));
        let get_id = interop_id_bytes("System.Storage.Get");
        assert!(bytecode.windows(4).any(|window| window == get_id));
    }
}
