use clap::{App, Arg, SubCommand};
use std::fs;
use std::path::PathBuf;
use std::process;

mod lexer;
mod parser;
mod semantic;
mod optimizer;
mod codegen;
mod error;
mod types;

use crate::error::CompilerError;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use crate::optimizer::Optimizer;
use crate::codegen::CodeGenerator;

#[derive(Debug, Clone)]
pub struct CompilerConfig {
    pub input_file: PathBuf,
    pub output_file: Option<PathBuf>,
    pub optimization_level: u8,
    pub output_format: OutputFormat,
    pub target_version: String,
    pub include_debug_info: bool,
    pub include_abi: bool,
    pub include_source_map: bool,
    pub gas_model: GasModel,
    pub validate_only: bool,
    pub analyze_only: bool,
    pub verbose: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Binary,
    Hex,
    Assembly,
    Json,
    DebugInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GasModel {
    Ethereum,
    Neo,
    Hybrid,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            input_file: PathBuf::new(),
            output_file: None,
            optimization_level: 2,
            output_format: OutputFormat::Hex,
            target_version: "3.0".to_string(),
            include_debug_info: false,
            include_abi: true,
            include_source_map: false,
            gas_model: GasModel::Neo,
            validate_only: false,
            analyze_only: false,
            verbose: false,
        }
    }
}

fn main() {
    let matches = App::new("Neo Solidity Compiler")
        .version("1.0.0")
        .author("Jimmy <jimmy@r3e.network>")
        .about("Compiles Solidity to NeoVM bytecode")
        .arg(
            Arg::with_name("INPUT")
                .help("Input Yul file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Output file path")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("optimize")
                .short("O")
                .long("optimize")
                .value_name("LEVEL")
                .help("Optimization level (0-3)")
                .takes_value(true)
                .default_value("2"),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Output format")
                .takes_value(true)
                .possible_values(&["binary", "hex", "assembly", "json", "debug"])
                .default_value("hex"),
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("VERSION")
                .help("Target NeoVM version")
                .takes_value(true)
                .possible_values(&["3.0", "3.1", "3.2", "3.3", "3.4", "3.5"])
                .default_value("3.0"),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Include debug information"),
        )
        .arg(
            Arg::with_name("no-abi")
                .long("no-abi")
                .help("Don't generate ABI information"),
        )
        .arg(
            Arg::with_name("source-map")
                .short("s")
                .long("source-map")
                .help("Generate source map"),
        )
        .arg(
            Arg::with_name("gas-model")
                .short("g")
                .long("gas-model")
                .value_name("MODEL")
                .help("Gas cost model")
                .takes_value(true)
                .possible_values(&["ethereum", "neo", "hybrid"])
                .default_value("neo"),
        )
        .arg(
            Arg::with_name("validate")
                .long("validate")
                .help("Validate input only, don't compile"),
        )
        .arg(
            Arg::with_name("analyze")
                .long("analyze")
                .help("Analyze code and report issues"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Verbose output"),
        )
        .subcommand(
            SubCommand::with_name("info")
                .about("Show compiler information")
                .arg(
                    Arg::with_name("capabilities")
                        .long("capabilities")
                        .help("Show compiler capabilities"),
                ),
        )
        .subcommand(
            SubCommand::with_name("version")
                .about("Show version information"),
        )
        .get_matches();

    // Handle subcommands
    if let Some(_) = matches.subcommand_matches("version") {
        println!("Neo Solidity Compiler v1.0.0");
        println!("Built with Rust {}", env!("RUSTC_VERSION", default = "unknown"));
        println!("Target: NeoVM 3.0+");
        return;
    }

    if let Some(info_matches) = matches.subcommand_matches("info") {
        if info_matches.is_present("capabilities") {
            show_capabilities();
        } else {
            show_info();
        }
        return;
    }

    // Parse configuration
    let mut config = CompilerConfig::default();
    
    config.input_file = PathBuf::from(matches.value_of("INPUT").unwrap());
    
    if let Some(output) = matches.value_of("output") {
        config.output_file = Some(PathBuf::from(output));
    }
    
    if let Some(opt_level) = matches.value_of("optimize") {
        config.optimization_level = opt_level.parse().unwrap_or_else(|_| {
            eprintln!("Invalid optimization level: {}", opt_level);
            process::exit(1);
        });
        if config.optimization_level > 3 {
            eprintln!("Optimization level must be 0-3");
            process::exit(1);
        }
    }
    
    config.output_format = match matches.value_of("format") {
        Some("binary") => OutputFormat::Binary,
        Some("hex") => OutputFormat::Hex,
        Some("assembly") => OutputFormat::Assembly,
        Some("json") => OutputFormat::Json,
        Some("debug") => OutputFormat::DebugInfo,
        _ => OutputFormat::Hex,
    };
    
    if let Some(target) = matches.value_of("target") {
        config.target_version = target.to_string();
    }
    
    config.include_debug_info = matches.is_present("debug");
    config.include_abi = !matches.is_present("no-abi");
    config.include_source_map = matches.is_present("source-map");
    
    config.gas_model = match matches.value_of("gas-model") {
        Some("ethereum") => GasModel::Ethereum,
        Some("neo") => GasModel::Neo,
        Some("hybrid") => GasModel::Hybrid,
        _ => GasModel::Neo,
    };
    
    config.validate_only = matches.is_present("validate");
    config.analyze_only = matches.is_present("analyze");
    config.verbose = matches.is_present("verbose");

    // Run compiler
    if let Err(e) = compile(config) {
        eprintln!("Compilation failed: {}", e);
        process::exit(1);
    }
}

fn compile(config: CompilerConfig) -> Result<(), CompilerError> {
    if config.verbose {
        println!("Neo Solidity Compiler starting...");
        println!("Input: {:?}", config.input_file);
        println!("Optimization level: {}", config.optimization_level);
        println!("Target: NeoVM {}", config.target_version);
    }

    // Read input file
    let input_content = fs::read_to_string(&config.input_file)
        .map_err(|e| CompilerError::IoError(e.to_string()))?;

    if config.verbose {
        println!("Read {} bytes from input file", input_content.len());
    }

    // Lexical analysis
    if config.verbose {
        println!("Starting lexical analysis...");
    }
    
    let mut lexer = Lexer::new(&input_content);
    let tokens = lexer.tokenize()?;
    
    if config.verbose {
        println!("Generated {} tokens", tokens.len());
    }

    if config.validate_only {
        println!("Validation complete - no syntax errors found");
        return Ok(());
    }

    // Parsing
    if config.verbose {
        println!("Starting parsing...");
    }
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    if config.verbose {
        println!("Generated AST with {} nodes", count_ast_nodes(&ast));
    }

    // Semantic analysis
    if config.verbose {
        println!("Starting semantic analysis...");
    }
    
    let mut semantic_analyzer = SemanticAnalyzer::new();
    let semantic_result = semantic_analyzer.analyze(&ast)?;
    
    if config.verbose {
        println!("Semantic analysis complete");
        if !semantic_result.warnings.is_empty() {
            println!("Warnings: {}", semantic_result.warnings.len());
            for warning in &semantic_result.warnings {
                println!("  Warning: {}", warning);
            }
        }
    }

    if config.analyze_only {
        println!("Analysis complete");
        print_analysis_results(&semantic_result);
        return Ok(());
    }

    // Optimization
    if config.verbose {
        println!("Starting optimization (level {})...", config.optimization_level);
    }
    
    let mut optimizer = Optimizer::new(config.optimization_level);
    let optimized_ast = optimizer.optimize(ast)?;
    
    if config.verbose {
        println!("Optimization complete");
        let stats = optimizer.get_stats();
        println!("  Instructions eliminated: {}", stats.eliminated_instructions);
        println!("  Functions inlined: {}", stats.inlined_functions);
        println!("  Constants folded: {}", stats.folded_constants);
    }

    // Code generation
    if config.verbose {
        println!("Starting code generation...");
    }
    
    let mut code_generator = CodeGenerator::new(&config);
    let compilation_result = code_generator.generate(&optimized_ast)?;
    
    if config.verbose {
        println!("Generated {} bytes of bytecode", compilation_result.bytecode.len());
        println!("Estimated gas cost: {}", compilation_result.estimated_gas);
    }

    // Output generation
    let output_path = config.output_file.unwrap_or_else(|| {
        let mut path = config.input_file.clone();
        path.set_extension(get_output_extension(&config.output_format));
        path
    });

    write_output(&output_path, &compilation_result, &config)?;
    
    if config.verbose {
        println!("Output written to: {:?}", output_path);
    }

    // Generate additional files if requested
    if config.include_abi {
        let mut abi_path = output_path.clone();
        abi_path.set_extension("abi.json");
        write_abi(&abi_path, &compilation_result.abi)?;
        if config.verbose {
            println!("ABI written to: {:?}", abi_path);
        }
    }

    if config.include_source_map {
        let mut source_map_path = output_path.clone();
        source_map_path.set_extension("map");
        write_source_map(&source_map_path, &compilation_result.source_map)?;
        if config.verbose {
            println!("Source map written to: {:?}", source_map_path);
        }
    }

    if config.include_debug_info {
        let mut debug_path = output_path.clone();
        debug_path.set_extension("debug.json");
        write_debug_info(&debug_path, &compilation_result.debug_info)?;
        if config.verbose {
            println!("Debug info written to: {:?}", debug_path);
        }
    }

    println!("Compilation successful!");
    
    Ok(())
}

fn get_output_extension(format: &OutputFormat) -> &'static str {
    match format {
        OutputFormat::Binary => "nef",
        OutputFormat::Hex => "hex",
        OutputFormat::Assembly => "asm",
        OutputFormat::Json => "json",
        OutputFormat::DebugInfo => "debug.json",
    }
}

fn write_output(
    path: &PathBuf,
    result: &codegen::CompilationResult,
    config: &CompilerConfig,
) -> Result<(), CompilerError> {
    match config.output_format {
        OutputFormat::Binary => {
            fs::write(path, &result.bytecode)
                .map_err(|e| CompilerError::IoError(e.to_string()))?;
        }
        OutputFormat::Hex => {
            let hex_string = hex::encode(&result.bytecode);
            fs::write(path, hex_string)
                .map_err(|e| CompilerError::IoError(e.to_string()))?;
        }
        OutputFormat::Assembly => {
            fs::write(path, &result.assembly)
                .map_err(|e| CompilerError::IoError(e.to_string()))?;
        }
        OutputFormat::Json => {
            let json_output = serde_json::json!({
                "bytecode": hex::encode(&result.bytecode),
                "assembly": result.assembly,
                "abi": result.abi,
                "estimated_gas": result.estimated_gas,
                "source_map": result.source_map,
                "debug_info": result.debug_info
            });
            fs::write(path, serde_json::to_string_pretty(&json_output).unwrap())
                .map_err(|e| CompilerError::IoError(e.to_string()))?;
        }
        OutputFormat::DebugInfo => {
            fs::write(path, serde_json::to_string_pretty(&result.debug_info).unwrap())
                .map_err(|e| CompilerError::IoError(e.to_string()))?;
        }
    }
    Ok(())
}

fn write_abi(path: &PathBuf, abi: &serde_json::Value) -> Result<(), CompilerError> {
    fs::write(path, serde_json::to_string_pretty(abi).unwrap())
        .map_err(|e| CompilerError::IoError(e.to_string()))?;
    Ok(())
}

fn write_source_map(path: &PathBuf, source_map: &str) -> Result<(), CompilerError> {
    fs::write(path, source_map)
        .map_err(|e| CompilerError::IoError(e.to_string()))?;
    Ok(())
}

fn write_debug_info(path: &PathBuf, debug_info: &serde_json::Value) -> Result<(), CompilerError> {
    fs::write(path, serde_json::to_string_pretty(debug_info).unwrap())
        .map_err(|e| CompilerError::IoError(e.to_string()))?;
    Ok(())
}

fn count_ast_nodes(ast: &parser::AstNode) -> usize {
    // Simple recursive count of AST nodes
    let mut count = 1;
    match &ast.node_type {
        parser::AstNodeType::Object { statements, .. } => {
            for stmt in statements {
                count += count_ast_nodes(stmt);
            }
        }
        parser::AstNodeType::Function { body, .. } => {
            count += count_ast_nodes(body);
        }
        parser::AstNodeType::Block { statements } => {
            for stmt in statements {
                count += count_ast_nodes(stmt);
            }
        }
        parser::AstNodeType::If { condition, then_branch, else_branch } => {
            count += count_ast_nodes(condition);
            count += count_ast_nodes(then_branch);
            if let Some(else_stmt) = else_branch {
                count += count_ast_nodes(else_stmt);
            }
        }
        parser::AstNodeType::For { init, condition, update, body } => {
            if let Some(init_stmt) = init {
                count += count_ast_nodes(init_stmt);
            }
            count += count_ast_nodes(condition);
            if let Some(update_stmt) = update {
                count += count_ast_nodes(update_stmt);
            }
            count += count_ast_nodes(body);
        }
        parser::AstNodeType::Switch { expression, cases, default } => {
            count += count_ast_nodes(expression);
            for case in cases {
                count += count_ast_nodes(&case.value);
                count += count_ast_nodes(&case.body);
            }
            if let Some(default_case) = default {
                count += count_ast_nodes(default_case);
            }
        }
        parser::AstNodeType::FunctionCall { arguments, .. } => {
            for arg in arguments {
                count += count_ast_nodes(arg);
            }
        }
        parser::AstNodeType::Assignment { value, .. } => {
            count += count_ast_nodes(value);
        }
        _ => {}
    }
    count
}

fn print_analysis_results(result: &semantic::SemanticResult) {
    println!("=== Analysis Results ===");
    
    if !result.warnings.is_empty() {
        println!("\nWarnings:");
        for warning in &result.warnings {
            println!("  - {}", warning);
        }
    }
    
    if !result.suggestions.is_empty() {
        println!("\nSuggestions:");
        for suggestion in &result.suggestions {
            println!("  - {}", suggestion);
        }
    }
    
    println!("\nComplexity Analysis:");
    println!("  Cyclomatic complexity: {}", result.complexity_metrics.cyclomatic);
    println!("  Function count: {}", result.complexity_metrics.function_count);
    println!("  Maximum nesting depth: {}", result.complexity_metrics.max_nesting_depth);
    
    if !result.security_issues.is_empty() {
        println!("\nSecurity Issues:");
        for issue in &result.security_issues {
            println!("  - {} (severity: {:?})", issue.message, issue.severity);
        }
    }
    
    println!("\nPerformance Analysis:");
    println!("  Estimated gas cost: {}", result.performance_metrics.estimated_gas);
    println!("  Hot paths identified: {}", result.performance_metrics.hot_paths.len());
    
    if !result.performance_metrics.optimization_opportunities.is_empty() {
        println!("  Optimization opportunities:");
        for opportunity in &result.performance_metrics.optimization_opportunities {
            println!("    - {}", opportunity);
        }
    }
}

fn show_info() {
    println!("Neo Solidity Compiler v1.0.0");
    println!("A production-ready compiler for Solidity to NeoVM bytecode");
    println!();
    println!("Features:");
    println!("  - Complete Yul IR support");
    println!("  - 4-level optimization (0-3)");
    println!("  - Multiple output formats");
    println!("  - Comprehensive error reporting");
    println!("  - Security analysis");
    println!("  - Performance optimization");
    println!("  - Source map generation");
    println!("  - ABI generation");
    println!("  - Debug information");
    println!();
    println!("Supported NeoVM versions: 3.0, 3.1, 3.2, 3.3, 3.4, 3.5");
    println!("Gas models: Ethereum, Neo, Hybrid");
}

fn show_capabilities() {
    println!("Compiler Capabilities:");
    println!();
    println!("Yul Language Support:");
    println!("  ✓ Objects and code blocks");
    println!("  ✓ Functions with parameters and return values");
    println!("  ✓ Variables and assignments");
    println!("  ✓ Control flow (if, for, switch)");
    println!("  ✓ All built-in functions");
    println!("  ✓ Memory and storage operations");
    println!("  ✓ Arithmetic and logical operations");
    println!("  ✓ Environment operations");
    println!();
    println!("NeoVM Instruction Set:");
    println!("  ✓ Stack operations (PUSH, POP, DUP, SWAP, ROT)");
    println!("  ✓ Arithmetic (ADD, SUB, MUL, DIV, MOD, NEG)");
    println!("  ✓ Logical (AND, OR, XOR, NOT)");
    println!("  ✓ Comparison (EQ, NE, LT, LE, GT, GE)");
    println!("  ✓ Control flow (JMP, JMPIF, JMPIFNOT, CALL, RET)");
    println!("  ✓ Memory (LDLOC, STLOC, LDARG, STARG)");
    println!("  ✓ Array operations (PACK, UNPACK, PICKITEM, SETITEM)");
    println!("  ✓ String operations (CAT, SUBSTR, SIZE)");
    println!("  ✓ Cryptographic (SHA256, HASH160, HASH256, CHECKSIG)");
    println!("  ✓ System calls (SYSCALL)");
    println!();
    println!("Optimization Passes:");
    println!("  ✓ Dead code elimination");
    println!("  ✓ Constant folding");
    println!("  ✓ Function inlining");
    println!("  ✓ Common subexpression elimination");
    println!("  ✓ Peephole optimization");
    println!("  ✓ Stack optimization");
    println!();
    println!("Analysis Features:");
    println!("  ✓ Semantic validation");
    println!("  ✓ Type checking");
    println!("  ✓ Security vulnerability detection");
    println!("  ✓ Performance analysis");
    println!("  ✓ Complexity metrics");
    println!("  ✓ Gas estimation");
}