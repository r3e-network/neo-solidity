use clap::{Arg, ArgAction, Command};
use neo_solidity::ir;
use neo_solidity::neo::build_nef;
use neo_solidity::semantic_model::build_semantic_model;
use neo_solidity::solidity::{
    analyse_all_sources, validate_contract, ContractMetadata, DiagnosticSeverity, FunctionKind,
    FunctionMetadata,
};
use num_traits::{ToPrimitive, Zero};
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
        let optimizer_level = matches
            .get_one::<String>("optimize")
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(2)
            .min(3);

        if let Err(err) = process_standard_json(input_path, output_path, optimizer_level) {
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
    let optimizer_level = matches
        .get_one::<String>("optimize")
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(2)
        .min(3);
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

    let artifacts = match compile_contracts(&input_content, verbose, optimizer_level) {
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
    optimizer_level: u8,
) -> Result<Vec<CompilationArtifacts>, CompileError> {
    let metadatas =
        analyse_all_sources(source).map_err(|err| CompileError::Message(err.to_string()))?;

    metadatas
        .into_iter()
        .map(|metadata| compile_metadata(metadata, verbose, optimizer_level))
        .collect()
}

fn compile_metadata(
    mut metadata: ContractMetadata,
    verbose: bool,
    optimizer_level: u8,
) -> Result<CompilationArtifacts, CompileError> {
    let optimizer_level = optimizer_level.min(3);
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

    // TODO: thread optimizer_level into IR/codegen when available.
    let _ = optimizer_level;
    let bytecode = generate_contract_bytecode(&mut metadata, &ir_module, verbose, optimizer_level);
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

/// IR optimization hook. Currently performs simple control-flow cleanup to drop
/// instructions that appear after a terminal return in a basic block.
pub(crate) fn optimize_ir(mut module: ir::Module, optimizer_level: u8) -> ir::Module {
    if optimizer_level == 0 {
        return module;
    }

    for function in &mut module.functions {
        let mut label_remap = std::collections::HashMap::new();

        for block in &mut function.basic_blocks {
            let mut trimmed = Vec::with_capacity(block.instructions.len());
            let mut terminated = false;

            for instr in block.instructions.drain(..) {
                if terminated {
                    // Preserve labels so jump targets remain addressable, drop other ops.
                    if matches!(instr, ir::Instruction::Label(_)) {
                        trimmed.push(instr);
                        terminated = false; // New label can be targeted, resume collection
                    }
                    continue;
                }

                match instr {
                    ir::Instruction::Return
                    | ir::Instruction::ReturnVoid
                    | ir::Instruction::ReturnDefault(_) => {
                        trimmed.push(instr);
                        terminated = true;
                    }
                    ir::Instruction::Jump { .. } => {
                        trimmed.push(instr);
                        terminated = true;
                    }
                    other => trimmed.push(other),
                }
            }

            block.instructions = trimmed;

            if optimizer_level >= 2 {
                fold_constant_binary_ops(block);
            }

            if optimizer_level >= 3 {
                dedupe_labels(block, &mut label_remap);
                remove_trivial_jumps(block);

                // NeoVM-specific optimizations at O3
                neovm_peephole_optimize(block);
                neovm_simplify_identity_ops(block);
                neovm_bool_optimize(block);

                // Run peephole again to catch newly exposed patterns
                neovm_peephole_optimize(block);
            }
        }

        if optimizer_level >= 3 && !label_remap.is_empty() {
            retarget_jumps(&mut function.basic_blocks, &label_remap);
        }
    }

    module
}

fn fold_constant_binary_ops(block: &mut ir::BasicBlock) {
    let mut optimized = Vec::with_capacity(block.instructions.len());
    let mut i = 0;

    while i < block.instructions.len() {
        if i + 2 < block.instructions.len() {
            if let (
                ir::Instruction::PushLiteral(lhs),
                ir::Instruction::PushLiteral(rhs),
                ir::Instruction::BinaryOp(op),
            ) = (
                &block.instructions[i],
                &block.instructions[i + 1],
                &block.instructions[i + 2],
            ) {
                if let Some(result) = evaluate_binary_literal(lhs, rhs, *op) {
                    optimized.push(ir::Instruction::PushLiteral(result));
                    i += 3;
                    continue;
                }
            }
        }

        optimized.push(block.instructions[i].clone());
        i += 1;
    }

    block.instructions = optimized;
}

fn remove_trivial_jumps(block: &mut ir::BasicBlock) {
    let mut optimized = Vec::with_capacity(block.instructions.len());
    let mut iter = block.instructions.iter();
    while let Some(instr) = iter.next() {
        if let ir::Instruction::Jump { target } = instr {
            if let Some(ir::Instruction::Label(id)) = iter.clone().next() {
                if id == target {
                    // Skip the jump; fallthrough reaches the label
                    continue;
                }
            }
        }
        optimized.push(instr.clone());
    }
    block.instructions = optimized;
}

fn dedupe_labels(block: &mut ir::BasicBlock, remap: &mut std::collections::HashMap<usize, usize>) {
    let mut optimized = Vec::with_capacity(block.instructions.len());
    let mut last_label: Option<usize> = None;

    for instr in block.instructions.drain(..) {
        match instr {
            ir::Instruction::Label(id) => {
                if let Some(prev) = last_label {
                    remap.insert(id, prev);
                    continue;
                } else {
                    last_label = Some(id);
                    optimized.push(ir::Instruction::Label(id));
                }
            }
            other => {
                last_label = None;
                optimized.push(other);
            }
        }
    }

    block.instructions = optimized;
}

fn retarget_jumps(blocks: &mut [ir::BasicBlock], remap: &std::collections::HashMap<usize, usize>) {
    for block in blocks {
        for instr in &mut block.instructions {
            match instr {
                ir::Instruction::Jump { target } | ir::Instruction::JumpIf { target } => {
                    if let Some(canonical) = remap.get(target) {
                        *target = *canonical;
                    }
                }
                _ => {}
            }
        }
    }
}

fn evaluate_binary_literal(
    lhs: &ir::LiteralValue,
    rhs: &ir::LiteralValue,
    op: ir::BinaryOperator,
) -> Option<ir::LiteralValue> {
    use ir::LiteralValue::*;

    match (lhs, rhs) {
        (Integer(a), Integer(b)) => match op {
            ir::BinaryOperator::Add => Some(Integer(a + b)),
            ir::BinaryOperator::Sub => Some(Integer(a - b)),
            ir::BinaryOperator::Mul => Some(Integer(a * b)),
            ir::BinaryOperator::Div => {
                if b.is_zero() {
                    None
                } else {
                    Some(Integer(a / b))
                }
            }
            ir::BinaryOperator::Mod => {
                if b.is_zero() {
                    None
                } else {
                    Some(Integer(a % b))
                }
            }
            ir::BinaryOperator::BitAnd => Some(Integer(a & b)),
            ir::BinaryOperator::BitOr => Some(Integer(a | b)),
            ir::BinaryOperator::BitXor => Some(Integer(a ^ b)),
            ir::BinaryOperator::Shl => {
                let shift = b.to_u64()?;
                Some(Integer(a << shift))
            }
            ir::BinaryOperator::Shr => {
                let shift = b.to_u64()?;
                Some(Integer(a >> shift))
            }
            ir::BinaryOperator::Lt => Some(Boolean(a < b)),
            ir::BinaryOperator::Le => Some(Boolean(a <= b)),
            ir::BinaryOperator::Gt => Some(Boolean(a > b)),
            ir::BinaryOperator::Ge => Some(Boolean(a >= b)),
            ir::BinaryOperator::Eq => Some(Boolean(a == b)),
            ir::BinaryOperator::Ne => Some(Boolean(a != b)),
        },
        (Boolean(a), Boolean(b)) => match op {
            ir::BinaryOperator::Eq => Some(Boolean(a == b)),
            ir::BinaryOperator::Ne => Some(Boolean(a != b)),
            _ => None,
        },
        _ => None,
    }
}

/// NeoVM-specific peephole optimization: removes redundant stack operations
fn neovm_peephole_optimize(block: &mut ir::BasicBlock) {
    let mut optimized = Vec::with_capacity(block.instructions.len());
    let mut i = 0;

    while i < block.instructions.len() {
        // Pattern: PUSH x followed by DROP → remove both
        if i + 1 < block.instructions.len() {
            if matches!(&block.instructions[i], ir::Instruction::PushLiteral(_))
                && matches!(&block.instructions[i + 1], ir::Instruction::Drop(_))
            {
                i += 2;
                continue;
            }

            // Pattern: LoadLocal x followed by DROP → remove both
            if matches!(&block.instructions[i], ir::Instruction::LoadLocal(_))
                && matches!(&block.instructions[i + 1], ir::Instruction::Drop(_))
            {
                i += 2;
                continue;
            }
        }

        // Note: StoreLocal + LoadLocal(same) pattern is handled at codegen level
        // by using NeoVM's STLOC/LDLOC which can be fused by the NeoVM optimizer

        optimized.push(block.instructions[i].clone());
        i += 1;
    }

    block.instructions = optimized;
}

/// NeoVM-specific: simplify identity operations (x + 0, x * 1, x & MAX, etc.)
fn neovm_simplify_identity_ops(block: &mut ir::BasicBlock) {
    use num_bigint::BigInt;
    use num_traits::Zero;

    let mut optimized = Vec::with_capacity(block.instructions.len());
    let mut i = 0;

    while i < block.instructions.len() {
        // Pattern: PUSH 0, BinaryOp::Add → remove (x + 0 = x)
        if i + 1 < block.instructions.len() {
            if let ir::Instruction::PushLiteral(ir::LiteralValue::Integer(val)) =
                &block.instructions[i]
            {
                if val.is_zero() {
                    if let ir::Instruction::BinaryOp(ir::BinaryOperator::Add) =
                        &block.instructions[i + 1]
                    {
                        // Skip both instructions, identity operation
                        i += 2;
                        continue;
                    }
                    if let ir::Instruction::BinaryOp(ir::BinaryOperator::Sub) =
                        &block.instructions[i + 1]
                    {
                        // x - 0 = x, skip both
                        i += 2;
                        continue;
                    }
                    if let ir::Instruction::BinaryOp(ir::BinaryOperator::BitOr) =
                        &block.instructions[i + 1]
                    {
                        // x | 0 = x, skip both
                        i += 2;
                        continue;
                    }
                    if let ir::Instruction::BinaryOp(ir::BinaryOperator::BitXor) =
                        &block.instructions[i + 1]
                    {
                        // x ^ 0 = x, skip both
                        i += 2;
                        continue;
                    }
                }
                // Pattern: PUSH 1, MUL → identity (x * 1 = x)
                if *val == BigInt::from(1) {
                    if let ir::Instruction::BinaryOp(ir::BinaryOperator::Mul) =
                        &block.instructions[i + 1]
                    {
                        i += 2;
                        continue;
                    }
                    if let ir::Instruction::BinaryOp(ir::BinaryOperator::Div) =
                        &block.instructions[i + 1]
                    {
                        // x / 1 = x
                        i += 2;
                        continue;
                    }
                }
            }
        }

        optimized.push(block.instructions[i].clone());
        i += 1;
    }

    block.instructions = optimized;
}

/// NeoVM-specific: optimize boolean patterns
fn neovm_bool_optimize(block: &mut ir::BasicBlock) {
    let mut optimized = Vec::with_capacity(block.instructions.len());
    let mut i = 0;

    while i < block.instructions.len() {
        // Pattern: PUSH true, EQ → identity for booleans (x == true = x)
        if i + 1 < block.instructions.len() {
            if let ir::Instruction::PushLiteral(ir::LiteralValue::Boolean(true)) =
                &block.instructions[i]
            {
                if let ir::Instruction::BinaryOp(ir::BinaryOperator::Eq) =
                    &block.instructions[i + 1]
                {
                    // x == true → x (skip both)
                    i += 2;
                    continue;
                }
            }

            // Pattern: PUSH false, NE → identity for booleans (x != false = x)
            if let ir::Instruction::PushLiteral(ir::LiteralValue::Boolean(false)) =
                &block.instructions[i]
            {
                if let ir::Instruction::BinaryOp(ir::BinaryOperator::Ne) =
                    &block.instructions[i + 1]
                {
                    // x != false → x (skip both)
                    i += 2;
                    continue;
                }
            }

            // Pattern: PUSH true, NE → converts to negation (x != true = !x)
            // Keep this pattern as-is since we don't have a simple NOT instruction
            // The codegen will handle it appropriately
        }

        // Pattern: BitwiseNot followed by BitwiseNot → identity (removes both)
        if i + 1 < block.instructions.len() {
            if matches!(&block.instructions[i], ir::Instruction::BitwiseNot)
                && matches!(&block.instructions[i + 1], ir::Instruction::BitwiseNot)
            {
                i += 2;
                continue;
            }
        }

        optimized.push(block.instructions[i].clone());
        i += 1;
    }

    block.instructions = optimized;
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
    let permissions = infer_permissions(metadata);

    // Collect safe methods for manifest
    let safe_methods: Vec<String> = metadata
        .methods
        .iter()
        .filter(|m| m.state_mutability.is_safe() && !matches!(m.kind, FunctionKind::Constructor))
        .map(|m| m.name.clone())
        .collect();

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
        "permissions": permissions,
        "trusts": [],
        "safemethods": safe_methods,
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

    // NEP-17: Fungible Token Standard (equivalent to ERC-20)
    // Required: symbol, decimals, totalSupply, balanceOf, transfer
    let nep17_required = ["symbol", "decimals", "totalsupply", "balanceof", "transfer"];
    if nep17_required.iter().all(|m| names.contains(*m)) {
        standards.push("NEP-17".to_string());
    }

    // NEP-11: Non-Fungible Token Standard (equivalent to ERC-721)
    // Core: balanceOf, ownerOf + at least one of: transfer, transferFrom, tokensOf
    let nep11_core = ["balanceof", "ownerof"];
    let nep11_transfer = ["transfer", "transferfrom", "tokensof"];
    if nep11_core.iter().all(|m| names.contains(*m))
        && nep11_transfer.iter().any(|m| names.contains(*m))
    {
        standards.push("NEP-11".to_string());
    }

    // NEP-24: Token Discovery Standard
    // For contracts that implement royalty info, token URIs, etc.
    if names.contains("tokenuri") || names.contains("royaltyinfo") {
        standards.push("NEP-24".to_string());
    }

    // NEP-26: Contract Upgrade Standard
    if names.contains("update") && names.contains("destroy") {
        standards.push("NEP-26".to_string());
    }

    // NEP-27: Invoke File Standard (for dynamic invocation)
    if names.contains("onpayment") || names.contains("onnep17payment") {
        standards.push("NEP-27".to_string());
    }

    standards
}

/// Infer contract permissions based on method signatures and behavior
fn infer_permissions(metadata: &ContractMetadata) -> Vec<serde_json::Value> {
    let mut permissions = Vec::new();
    let mut required_contracts: HashSet<&str> = HashSet::new();
    let mut required_methods: HashSet<&str> = HashSet::new();

    // Analyze method bodies for external calls (simplified heuristic)
    for method in &metadata.methods {
        let method_lower = method.name.to_ascii_lowercase();

        // Token operations typically need to call other contracts
        if method_lower.contains("transfer") || method_lower.contains("approve") {
            required_methods.insert("transfer");
            required_methods.insert("balanceOf");
        }

        // If contract has onPayment handlers, it receives tokens
        if method_lower.contains("onpayment") || method_lower.contains("onnep") {
            required_contracts.insert("*"); // Accept payments from any contract
        }

        // Emergency recovery functions need broad permissions
        if method_lower.contains("emergency") || method_lower.contains("recovery") {
            required_contracts.insert("*");
            required_methods.insert("*");
        }
    }

    // Check if contract uses storage (most contracts do)
    if metadata.uses_storage {
        // Storage contracts don't need special permissions for storage syscalls
    }

    // Build permission entries
    if required_contracts.contains("*") && required_methods.contains("*") {
        // Broad permissions needed
        permissions.push(json!({
            "contract": "*",
            "methods": "*"
        }));
    } else if !required_contracts.is_empty() || !required_methods.is_empty() {
        // Specific permissions
        let methods_value = if required_methods.is_empty() {
            json!([])
        } else if required_methods.contains("*") {
            json!("*")
        } else {
            json!(required_methods.iter().collect::<Vec<_>>())
        };

        permissions.push(json!({
            "contract": if required_contracts.is_empty() { "*" } else { "*" },
            "methods": methods_value
        }));
    } else {
        // Minimal permissions - only allow calls to self
        permissions.push(json!({
            "contract": "*",
            "methods": []
        }));
    }

    // If no permissions were added, use safe default
    if permissions.is_empty() {
        permissions.push(json!({
            "contract": "*",
            "methods": "*"
        }));
    }

    permissions
}

#[cfg(test)]
mod tests;
