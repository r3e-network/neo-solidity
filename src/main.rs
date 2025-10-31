use clap::{Arg, Command};
use neo_solidity::ir::{self, LiteralValue, ValueType};
use neo_solidity::neo::build_nef;
use neo_solidity::semantic_model::build_semantic_model;
use neo_solidity::solidity::{
    analyse_source, validate_contract, ContractMetadata, DiagnosticSeverity, FunctionKind,
    FunctionMetadata,
};
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

const COMPILER_ID: &str = "neo-solidity-1.0.0";
const COMPILER_EMAIL: &str = "Jimmy <jimmy@r3e.network>";
const VERSION: (u32, u32, u32, u32) = (1, 0, 0, 0);

fn main() {
    let matches = Command::new("neo-solc")
        .version("1.0.0")
        .author(COMPILER_EMAIL)
        .about("Compiles Solidity to Neo N3 smart contracts (.nef + .manifest.json)")
        .arg(
            Arg::new("input")
                .help("Input Solidity file")
                .required(true)
                .index(1),
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

    let input_file = matches.get_one::<String>("input").unwrap();
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

    let mut metadata = match analyse_source(&input_content) {
        Ok(metadata) => metadata,
        Err(err) => {
            eprintln!("Solidity analysis error: {err}");
            std::process::exit(1);
        }
    };

    let diagnostics = validate_contract(&metadata);
    let mut has_error = false;
    for diagnostic in &diagnostics {
        match diagnostic.severity {
            DiagnosticSeverity::Warning => {
                eprintln!("warning: {}", diagnostic.message);
            }
            DiagnosticSeverity::Error => {
                eprintln!("error: {}", diagnostic.message);
                has_error = true;
            }
        }
    }

    if has_error {
        std::process::exit(1);
    }

    let _semantic_model = match build_semantic_model(&metadata) {
        Ok(model) => model,
        Err(diags) => {
            for diag in diags {
                match diag.severity {
                    DiagnosticSeverity::Warning => eprintln!("warning: {}", diag.message),
                    DiagnosticSeverity::Error => {
                        eprintln!("error: {}", diag.message);
                    }
                }
            }
            std::process::exit(1);
        }
    };

    let ir_module = match ir::Module::from_contract(&metadata) {
        Ok(module) => module,
        Err(errors) => {
            for error in errors {
                eprintln!("error: {}", error);
            }
            std::process::exit(1);
        }
    };

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

    let contract_bytecode = generate_contract_bytecode(&mut metadata, &ir_module, verbose);

    if verbose {
        println!(
            "Generated {} bytes of NeoVM bytecode",
            contract_bytecode.len()
        );
    }

    let manifest = build_manifest(&metadata);

    match format.as_str() {
        "nef" => {
            let nef_path = if output_prefix.ends_with(".nef") {
                output_prefix.to_string()
            } else {
                format!("{}.nef", output_prefix)
            };
            write_nef_file(&nef_path, &contract_bytecode);
            println!("✅ NEF file generated: {nef_path}");
        }
        "manifest" => {
            let manifest_path = if output_prefix.ends_with(".manifest.json") {
                output_prefix.to_string()
            } else {
                format!("{}.manifest.json", output_prefix)
            };
            write_manifest_file(&manifest_path, &manifest);
            println!("✅ Manifest file generated: {manifest_path}");
        }
        "complete" => {
            let nef_path = format!("{}.nef", output_prefix);
            let manifest_path = format!("{}.manifest.json", output_prefix);
            write_nef_file(&nef_path, &contract_bytecode);
            write_manifest_file(&manifest_path, &manifest);
            println!("✅ Contract files generated:\n   📄 {nef_path}\n   📄 {manifest_path}");
        }
        "json" => {
            let json_path = if output_prefix.ends_with(".json") {
                output_prefix.to_string()
            } else {
                format!("{}.json", output_prefix)
            };
            write_json_file(&json_path, &contract_bytecode, &manifest, &metadata);
            println!("✅ JSON file generated: {json_path}");
        }
        "assembly" => {
            println!("Assembly output is not yet implemented.");
        }
        other => {
            println!("Unsupported format: {other}");
        }
    }

    println!("🎉 Neo Solidity compilation completed");
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
        ir::BuiltinCall::AbiEncode
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
