//! Neo Solidity Compiler CLI
//! 
//! Command-line interface for the Neo Solidity compiler with comprehensive
//! options, error reporting, and output formats.

use clap::{Arg, Command, ArgMatches, ValueEnum};
use neo_solidity::compiler::{SolidityCompiler, CompilerOptions, NeoVMVersion, OutputFormat};
use neo_solidity::compiler::error::{ErrorReporter, ErrorContext, CompilationPhase};
use neo_solidity::{Result, VERSION};
use serde_json;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

/// CLI application for Neo Solidity compiler
#[derive(Debug)]
struct CliApp {
    compiler: SolidityCompiler,
    error_reporter: ErrorReporter,
    verbose: bool,
    quiet: bool,
}

/// Output format options for CLI
#[derive(Debug, Clone, ValueEnum)]
enum CliOutputFormat {
    Binary,
    Hex,
    Assembly,
    Json,
    DebugInfo,
}

/// NeoVM version options for CLI
#[derive(Debug, Clone, ValueEnum)]
enum CliNeoVMVersion {
    #[value(name = "3.0")]
    V30,
    #[value(name = "3.1")]
    V31,
    #[value(name = "3.2")]
    V32,
    #[value(name = "3.3")]
    V33,
    #[value(name = "3.4")]
    V34,
    #[value(name = "3.5")]
    V35,
    Latest,
}

fn main() {
    let matches = build_cli().get_matches();
    
    let mut app = CliApp::new(&matches);
    
    if let Err(e) = app.run(&matches) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn build_cli() -> Command {
    Command::new("neo-solc")
        .version(VERSION)
        .author("Neo Solidity Team")
        .about("Solidity to Neo blockchain compiler")
        .long_about(
            "A complete compiler for transforming Solidity smart contracts into Neo blockchain bytecode. \
             Supports full Yul intermediate representation with advanced optimizations and comprehensive debugging."
        )
        .arg(
            Arg::new("input")
                .help("Input Yul source file")
                .required(true)
                .value_name("FILE")
                .value_parser(clap::value_parser!(PathBuf))
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file path")
                .value_name("FILE")
                .value_parser(clap::value_parser!(PathBuf))
        )
        .arg(
            Arg::new("output-dir")
                .long("output-dir")
                .help("Output directory")
                .value_name("DIR")
                .value_parser(clap::value_parser!(PathBuf))
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("Output format")
                .value_enum::<CliOutputFormat>()
                .default_value("hex")
        )
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .help("Target NeoVM version")
                .value_enum::<CliNeoVMVersion>()
                .default_value("latest")
        )
        .arg(
            Arg::new("optimization")
                .short('O')
                .long("optimization")
                .help("Optimization level (0-3)")
                .value_name("LEVEL")
                .value_parser(clap::value_parser!(u8).range(0..=3))
                .default_value("2")
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Include debug information")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("source-maps")
                .long("source-maps")
                .help("Generate source maps")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("gas-limit")
                .long("gas-limit")
                .help("Maximum gas limit")
                .value_name("LIMIT")
                .value_parser(clap::value_parser!(u64))
        )
        .arg(
            Arg::new("no-security-checks")
                .long("no-security-checks")
                .help("Disable security checks")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("no-runtime-validation")
                .long("no-runtime-validation")
                .help("Disable runtime validation")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("custom-passes")
                .long("custom-passes")
                .help("Custom optimization passes")
                .value_name("PASSES")
                .action(clap::ArgAction::Append)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Verbose output")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Quiet mode (errors only)")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("color")
                .long("color")
                .help("When to use colored output")
                .value_parser(["auto", "always", "never"])
                .default_value("auto")
        )
        .arg(
            Arg::new("emit")
                .long("emit")
                .help("Emit additional outputs")
                .value_parser(["ast", "tokens", "assembly", "metadata"])
                .action(clap::ArgAction::Append)
        )
        .arg(
            Arg::new("validate-only")
                .long("validate-only")
                .help("Only validate input without compilation")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("stats")
                .long("stats")
                .help("Show compilation statistics")
                .action(clap::ArgAction::SetTrue)
        )
        .subcommand(
            Command::new("version")
                .about("Show version information")
                .arg(
                    Arg::new("detailed")
                        .long("detailed")
                        .help("Show detailed version information")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("check")
                .about("Check syntax without compilation")
                .arg(
                    Arg::new("input")
                        .help("Input Yul source file")
                        .required(true)
                        .value_name("FILE")
                        .value_parser(clap::value_parser!(PathBuf))
                )
        )
        .subcommand(
            Command::new("optimize")
                .about("Run optimization passes only")
                .arg(
                    Arg::new("input")
                        .help("Input Yul source file")
                        .required(true)
                        .value_name("FILE")
                        .value_parser(clap::value_parser!(PathBuf))
                )
                .arg(
                    Arg::new("level")
                        .short('l')
                        .long("level")
                        .help("Optimization level")
                        .value_parser(clap::value_parser!(u8).range(0..=3))
                        .default_value("3")
                )
        )
        .subcommand(
            Command::new("analyze")
                .about("Analyze code without compilation")
                .arg(
                    Arg::new("input")
                        .help("Input Yul source file")
                        .required(true)
                        .value_name("FILE")
                        .value_parser(clap::value_parser!(PathBuf))
                )
                .arg(
                    Arg::new("report")
                        .long("report")
                        .help("Generate analysis report")
                        .action(clap::ArgAction::SetTrue)
                )
        )
}

impl CliApp {
    fn new(matches: &ArgMatches) -> Self {
        let options = Self::build_compiler_options(matches);
        let compiler = SolidityCompiler::with_options(options);
        
        let error_reporter = ErrorReporter {
            colored_output: Self::should_use_color(matches),
            show_suggestions: !matches.get_flag("quiet"),
            show_source_context: matches.get_flag("verbose"),
            max_context_lines: if matches.get_flag("verbose") { 5 } else { 3 },
        };

        Self {
            compiler,
            error_reporter,
            verbose: matches.get_flag("verbose"),
            quiet: matches.get_flag("quiet"),
        }
    }

    fn run(&mut self, matches: &ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("version", sub_matches)) => self.handle_version(sub_matches),
            Some(("check", sub_matches)) => self.handle_check(sub_matches),
            Some(("optimize", sub_matches)) => self.handle_optimize(sub_matches),
            Some(("analyze", sub_matches)) => self.handle_analyze(sub_matches),
            _ => self.handle_compile(matches),
        }
    }

    fn handle_compile(&mut self, matches: &ArgMatches) -> Result<()> {
        let input_path = matches.get_one::<PathBuf>("input").unwrap();
        
        if !input_path.exists() {
            return Err(format!("Input file not found: {}", input_path.display()).into());
        }

        if self.verbose {
            println!("Compiling: {}", input_path.display());
        }

        // Validate-only mode
        if matches.get_flag("validate-only") {
            return self.validate_file(input_path);
        }

        // Read source file
        let source = fs::read_to_string(input_path)?;
        
        // Emit tokens if requested
        if let Some(emit_options) = matches.get_many::<String>("emit") {
            for option in emit_options {
                match option.as_str() {
                    "tokens" => self.emit_tokens(&source)?,
                    "ast" => self.emit_ast(&source)?,
                    "assembly" => {}, // Will be handled after compilation
                    "metadata" => {}, // Will be handled after compilation
                    _ => {
                        eprintln!("Warning: Unknown emit option: {}", option);
                    }
                }
            }
        }

        // Compile
        let start_time = std::time::Instant::now();
        let result = self.compiler.compile(&source);
        let compile_time = start_time.elapsed();

        match result {
            Ok(result) => {
                if self.verbose {
                    println!("Compilation successful in {:?}", compile_time);
                    println!("Generated {} bytes of bytecode", result.bytecode.len());
                    println!("Gas estimate: {:?}", result.gas_estimate);
                }

                // Handle diagnostics
                if !result.diagnostics.is_empty() {
                    self.print_diagnostics(&result.diagnostics);
                }

                // Write output
                self.write_output(matches, &result)?;

                // Show statistics if requested
                if matches.get_flag("stats") {
                    self.show_statistics(&result, compile_time);
                }

                Ok(())
            }
            Err(e) => {
                let mut context = ErrorContext::new(CompilationPhase::CodeGeneration);
                context.add_source_file(input_path.to_string_lossy().to_string(), source);
                
                eprintln!("{}", self.error_reporter.format_error(&e, &context));
                Err(e)
            }
        }
    }

    fn handle_version(&self, matches: &ArgMatches) -> Result<()> {
        if matches.get_flag("detailed") {
            println!("neo-solc {}", VERSION);
            println!("Git hash: {}", env!("GIT_HASH"));
            println!("Build date: {}", env!("BUILD_DATE"));
            println!("Rust version: {}", env!("RUSTC_VERSION"));
            println!("Target triple: {}", env!("TARGET"));
        } else {
            println!("{}", VERSION);
        }
        Ok(())
    }

    fn handle_check(&mut self, matches: &ArgMatches) -> Result<()> {
        let input_path = matches.get_one::<PathBuf>("input").unwrap();
        self.validate_file(input_path)
    }

    fn handle_optimize(&mut self, matches: &ArgMatches) -> Result<()> {
        let input_path = matches.get_one::<PathBuf>("input").unwrap();
        let level = *matches.get_one::<u8>("level").unwrap();
        
        if self.verbose {
            println!("Optimizing: {} (level {})", input_path.display(), level);
        }

        let source = fs::read_to_string(input_path)?;
        
        // Update optimization level
        let mut options = self.compiler.version_info();
        let mut compiler_options = CompilerOptions::default();
        compiler_options.optimization_level = level;
        self.compiler.set_options(compiler_options);

        let result = self.compiler.compile(&source)?;
        
        println!("Optimization complete");
        println!("Original size: {} bytes", source.len());
        println!("Optimized bytecode: {} bytes", result.bytecode.len());
        
        if let Some(gas_estimate) = result.gas_estimate {
            println!("Gas estimate: {}", gas_estimate);
        }

        Ok(())
    }

    fn handle_analyze(&mut self, matches: &ArgMatches) -> Result<()> {
        let input_path = matches.get_one::<PathBuf>("input").unwrap();
        let source = fs::read_to_string(input_path)?;
        
        if self.verbose {
            println!("Analyzing: {}", input_path.display());
        }

        let diagnostics = self.compiler.validate(&source)?;
        
        if diagnostics.is_empty() {
            println!("No issues found");
        } else {
            self.print_diagnostics(&diagnostics);
        }

        if matches.get_flag("report") {
            self.generate_analysis_report(&diagnostics, input_path)?;
        }

        Ok(())
    }

    fn validate_file(&mut self, input_path: &PathBuf) -> Result<()> {
        let source = fs::read_to_string(input_path)?;
        let diagnostics = self.compiler.validate(&source)?;
        
        if diagnostics.is_empty() {
            if !self.quiet {
                println!("✓ Validation passed");
            }
        } else {
            self.print_diagnostics(&diagnostics);
            return Err("Validation failed".into());
        }

        Ok(())
    }

    fn emit_tokens(&self, source: &str) -> Result<()> {
        use neo_solidity::compiler::lexer::YulLexer;
        
        let mut lexer = YulLexer::new();
        let tokens = lexer.tokenize(source)?;
        
        println!("=== TOKENS ===");
        for (i, token) in tokens.iter().enumerate() {
            println!("{:3}: {:?}", i, token);
        }
        
        Ok(())
    }

    fn emit_ast(&self, source: &str) -> Result<()> {
        use neo_solidity::compiler::{lexer::YulLexer, parser::YulParser};
        
        let mut lexer = YulLexer::new();
        let tokens = lexer.tokenize(source)?;
        
        let mut parser = YulParser::new();
        let ast = parser.parse(tokens)?;
        
        println!("=== AST ===");
        let json = serde_json::to_string_pretty(&ast)?;
        println!("{}", json);
        
        Ok(())
    }

    fn write_output(&self, matches: &ArgMatches, result: &neo_solidity::compiler::CompilerResult) -> Result<()> {
        let format = matches.get_one::<CliOutputFormat>("format").unwrap();
        let output_path = self.determine_output_path(matches)?;

        let content = match format {
            CliOutputFormat::Binary => result.bytecode.clone(),
            CliOutputFormat::Hex => hex::encode(&result.bytecode).into_bytes(),
            CliOutputFormat::Assembly => result.assembly.as_bytes().to_vec(),
            CliOutputFormat::Json => {
                let json_output = serde_json::json!({
                    "bytecode": hex::encode(&result.bytecode),
                    "assembly": result.assembly,
                    "abi": result.abi,
                    "metadata": result.metadata,
                    "diagnostics": result.diagnostics
                });
                serde_json::to_string_pretty(&json_output)?.into_bytes()
            },
            CliOutputFormat::DebugInfo => {
                if let Some(source_maps) = &result.source_maps {
                    serde_json::to_string_pretty(source_maps)?.into_bytes()
                } else {
                    b"No debug information available".to_vec()
                }
            }
        };

        if let Some(path) = output_path {
            fs::write(&path, content)?;
            if self.verbose {
                println!("Output written to: {}", path.display());
            }
        } else {
            io::stdout().write_all(&content)?;
        }

        Ok(())
    }

    fn determine_output_path(&self, matches: &ArgMatches) -> Result<Option<PathBuf>> {
        if let Some(output) = matches.get_one::<PathBuf>("output") {
            return Ok(Some(output.clone()));
        }

        if let Some(output_dir) = matches.get_one::<PathBuf>("output-dir") {
            let input_path = matches.get_one::<PathBuf>("input").unwrap();
            let format = matches.get_one::<CliOutputFormat>("format").unwrap();
            
            let extension = match format {
                CliOutputFormat::Binary => "bin",
                CliOutputFormat::Hex => "hex",
                CliOutputFormat::Assembly => "asm",
                CliOutputFormat::Json => "json",
                CliOutputFormat::DebugInfo => "debug",
            };

            let filename = input_path.file_stem()
                .ok_or("Invalid input filename")?
                .to_string_lossy();
            
            return Ok(Some(output_dir.join(format!("{}.{}", filename, extension))));
        }

        // Output to stdout
        Ok(None)
    }

    fn print_diagnostics(&self, diagnostics: &[neo_solidity::compiler::Diagnostic]) {
        for diagnostic in diagnostics {
            let level = match diagnostic.level {
                neo_solidity::compiler::DiagnosticLevel::Error => "Error",
                neo_solidity::compiler::DiagnosticLevel::Warning => "Warning",
                neo_solidity::compiler::DiagnosticLevel::Info => "Info",
                neo_solidity::compiler::DiagnosticLevel::Hint => "Hint",
            };

            if self.error_reporter.colored_output {
                let color = match diagnostic.level {
                    neo_solidity::compiler::DiagnosticLevel::Error => "\x1b[1;31m",
                    neo_solidity::compiler::DiagnosticLevel::Warning => "\x1b[1;33m",
                    neo_solidity::compiler::DiagnosticLevel::Info => "\x1b[1;34m",
                    neo_solidity::compiler::DiagnosticLevel::Hint => "\x1b[1;36m",
                };
                println!("{}{}:\x1b[0m {}", color, level, diagnostic.message);
            } else {
                println!("{}: {}", level, diagnostic.message);
            }

            if let Some(location) = &diagnostic.location {
                println!("  --> {}:{}:{}", location.file, location.line, location.column);
            }

            if let Some(suggestion) = &diagnostic.suggestion {
                println!("  Suggestion: {}", suggestion);
            }
        }
    }

    fn show_statistics(&self, result: &neo_solidity::compiler::CompilerResult, compile_time: std::time::Duration) {
        println!("\n=== COMPILATION STATISTICS ===");
        println!("Compilation time: {:?}", compile_time);
        println!("Bytecode size: {} bytes", result.bytecode.len());
        println!("Assembly lines: {}", result.assembly.lines().count());
        
        if let Some(gas_estimate) = result.gas_estimate {
            println!("Gas estimate: {}", gas_estimate);
        }

        println!("Compiler version: {}", result.metadata.compiler_version);
        println!("Optimization level: {}", result.metadata.optimization_level);
        println!("Security warnings: {}", result.metadata.security_warnings);
        
        let error_count = result.diagnostics.iter()
            .filter(|d| matches!(d.level, neo_solidity::compiler::DiagnosticLevel::Error))
            .count();
        let warning_count = result.diagnostics.iter()
            .filter(|d| matches!(d.level, neo_solidity::compiler::DiagnosticLevel::Warning))
            .count();
            
        println!("Diagnostics: {} errors, {} warnings", error_count, warning_count);
    }

    fn generate_analysis_report(&self, diagnostics: &[neo_solidity::compiler::Diagnostic], input_path: &PathBuf) -> Result<()> {
        let report_path = input_path.with_extension("analysis.json");
        
        let report = serde_json::json!({
            "file": input_path.to_string_lossy(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "diagnostics": diagnostics,
            "summary": {
                "total_issues": diagnostics.len(),
                "errors": diagnostics.iter().filter(|d| matches!(d.level, neo_solidity::compiler::DiagnosticLevel::Error)).count(),
                "warnings": diagnostics.iter().filter(|d| matches!(d.level, neo_solidity::compiler::DiagnosticLevel::Warning)).count(),
                "info": diagnostics.iter().filter(|d| matches!(d.level, neo_solidity::compiler::DiagnosticLevel::Info)).count(),
                "hints": diagnostics.iter().filter(|d| matches!(d.level, neo_solidity::compiler::DiagnosticLevel::Hint)).count(),
            }
        });

        fs::write(&report_path, serde_json::to_string_pretty(&report)?)?;
        
        if self.verbose {
            println!("Analysis report written to: {}", report_path.display());
        }

        Ok(())
    }

    fn build_compiler_options(matches: &ArgMatches) -> CompilerOptions {
        let target_version = match matches.get_one::<CliNeoVMVersion>("target").unwrap() {
            CliNeoVMVersion::V30 => NeoVMVersion::V3_0,
            CliNeoVMVersion::V31 => NeoVMVersion::V3_1,
            CliNeoVMVersion::V32 => NeoVMVersion::V3_2,
            CliNeoVMVersion::V33 => NeoVMVersion::V3_3,
            CliNeoVMVersion::V34 => NeoVMVersion::V3_4,
            CliNeoVMVersion::V35 => NeoVMVersion::V3_5,
            CliNeoVMVersion::Latest => NeoVMVersion::Latest,
        };

        let output_format = match matches.get_one::<CliOutputFormat>("format").unwrap() {
            CliOutputFormat::Binary => OutputFormat::Binary,
            CliOutputFormat::Hex => OutputFormat::Hex,
            CliOutputFormat::Assembly => OutputFormat::Assembly,
            CliOutputFormat::Json => OutputFormat::Binary, // JSON wraps binary
            CliOutputFormat::DebugInfo => OutputFormat::DebugInfo,
        };

        let custom_passes = matches.get_many::<String>("custom-passes")
            .map(|values| values.map(|s| s.clone()).collect())
            .unwrap_or_default();

        CompilerOptions {
            target_version,
            optimization_level: *matches.get_one::<u8>("optimization").unwrap(),
            debug: matches.get_flag("debug"),
            output_format,
            source_maps: matches.get_flag("source-maps"),
            gas_limit: matches.get_one::<u64>("gas-limit").cloned(),
            security_checks: !matches.get_flag("no-security-checks"),
            runtime_validation: !matches.get_flag("no-runtime-validation"),
            custom_passes,
        }
    }

    fn should_use_color(matches: &ArgMatches) -> bool {
        match matches.get_one::<String>("color").unwrap().as_str() {
            "always" => true,
            "never" => false,
            "auto" => {
                // Check if output is a terminal
                atty::is(atty::Stream::Stdout) && atty::is(atty::Stream::Stderr)
            }
            _ => false,
        }
    }
}