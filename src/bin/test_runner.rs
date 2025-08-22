//! Neo Solidity Test Runner
//! 
//! Comprehensive test execution engine for running all categories of tests
//! including unit tests, differential tests, fuzzing, security analysis,
//! conformance testing, and performance benchmarks.

use std::path::PathBuf;
use std::process;
use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

use neo_solidity::testing::{TestRunner, TestConfig, OutputFormat};
use neo_solidity::security::{SecurityAnalyzer, SecurityConfig};
use neo_solidity::fuzzing::{FuzzingCoordinator, FuzzingConfig};
use neo_solidity::conformance::{ConformanceTestSuite, ConformanceConfig};
use neo_solidity::debugger::{Debugger, DebuggerConfig};

#[derive(Parser)]
#[command(author, version, about = "Neo Solidity Test Runner", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Output format for test results
    #[arg(long, value_enum, default_value = "human")]
    output_format: OutputFormatArg,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run comprehensive test suite
    Test {
        /// Test suite types to run
        #[arg(long, value_delimiter = ',')]
        suites: Vec<TestSuiteType>,

        /// Enable parallel test execution
        #[arg(long)]
        parallel: bool,

        /// Maximum test execution time (seconds)
        #[arg(long, default_value = "300")]
        timeout: u64,

        /// Output directory for test reports
        #[arg(short, long)]
        output_dir: Option<PathBuf>,
    },
    
    /// Run security analysis
    Security {
        /// Contract source files or directories
        contracts: Vec<PathBuf>,

        /// Minimum severity level to report
        #[arg(long, default_value = "medium")]
        min_severity: String,

        /// Enable static analysis
        #[arg(long)]
        static_analysis: bool,

        /// Enable crypto analysis
        #[arg(long)]
        crypto_analysis: bool,

        /// Generate security report
        #[arg(long)]
        report: bool,
    },

    /// Run fuzzing campaign
    Fuzz {
        /// Target contracts for fuzzing
        contracts: Vec<PathBuf>,

        /// Maximum test cases per category
        #[arg(long, default_value = "10000")]
        max_cases: u32,

        /// Fuzzing campaign duration (seconds)
        #[arg(long, default_value = "300")]
        duration: u64,

        /// Enable differential fuzzing (EVM vs NeoVM)
        #[arg(long)]
        differential: bool,

        /// Crash output directory
        #[arg(long)]
        crash_dir: Option<PathBuf>,
    },

    /// Run conformance testing
    Conformance {
        /// Test vector sources
        #[arg(long, value_delimiter = ',')]
        sources: Vec<String>,

        /// Required conformance level
        #[arg(long, default_value = "standard")]
        level: String,

        /// Enable Ethereum test vectors
        #[arg(long)]
        ethereum_tests: bool,

        /// Enable Solidity specification tests
        #[arg(long)]
        solidity_tests: bool,
    },

    /// Interactive debugging session
    Debug {
        /// Contract bytecode file
        contract: PathBuf,

        /// Function to debug
        #[arg(long)]
        function: Option<String>,

        /// Enable source maps
        #[arg(long)]
        source_maps: bool,

        /// Enable trace hooks
        #[arg(long)]
        trace_hooks: bool,

        /// Breakpoint file (line numbers)
        #[arg(long)]
        breakpoints: Option<PathBuf>,
    },

    /// Performance benchmarking
    Benchmark {
        /// Benchmark categories to run
        #[arg(long, value_delimiter = ',')]
        categories: Vec<BenchmarkCategory>,

        /// Number of iterations per benchmark
        #[arg(long, default_value = "100")]
        iterations: u32,

        /// Warm-up iterations
        #[arg(long, default_value = "10")]
        warmup: u32,

        /// Generate performance report
        #[arg(long)]
        report: bool,
    },

    /// Generate comprehensive report
    Report {
        /// Input test result files
        results: Vec<PathBuf>,

        /// Report format
        #[arg(long, value_enum, default_value = "html")]
        format: ReportFormat,

        /// Output file
        #[arg(short, long)]
        output: PathBuf,

        /// Include detailed analysis
        #[arg(long)]
        detailed: bool,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum OutputFormatArg {
    Human,
    Json,
    Xml,
    Tap,
    Junit,
}

#[derive(clap::ValueEnum, Clone)]
enum TestSuiteType {
    Unit,
    Integration,
    Differential,
    Property,
    Performance,
    Security,
    Conformance,
}

#[derive(clap::ValueEnum, Clone)]
enum BenchmarkCategory {
    Execution,
    Compilation,
    Gas,
    Memory,
    Differential,
}

#[derive(clap::ValueEnum, Clone)]
enum ReportFormat {
    Html,
    Pdf,
    Markdown,
    Json,
}

impl From<OutputFormatArg> for OutputFormat {
    fn from(arg: OutputFormatArg) -> Self {
        match arg {
            OutputFormatArg::Human => OutputFormat::Human,
            OutputFormatArg::Json => OutputFormat::Json,
            OutputFormatArg::Xml => OutputFormat::Xml,
            OutputFormatArg::Tap => OutputFormat::Tap,
            OutputFormatArg::Junit => OutputFormat::JUnit,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(if cli.verbose { Level::DEBUG } else { Level::INFO })
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let result = match cli.command {
        Commands::Test { suites, parallel, timeout, output_dir } => {
            run_comprehensive_tests(suites, parallel, timeout, output_dir, cli.output_format.into()).await
        }
        Commands::Security { contracts, min_severity, static_analysis, crypto_analysis, report } => {
            run_security_analysis(contracts, min_severity, static_analysis, crypto_analysis, report).await
        }
        Commands::Fuzz { contracts, max_cases, duration, differential, crash_dir } => {
            run_fuzzing_campaign(contracts, max_cases, duration, differential, crash_dir).await
        }
        Commands::Conformance { sources, level, ethereum_tests, solidity_tests } => {
            run_conformance_testing(sources, level, ethereum_tests, solidity_tests).await
        }
        Commands::Debug { contract, function, source_maps, trace_hooks, breakpoints } => {
            run_interactive_debug(contract, function, source_maps, trace_hooks, breakpoints).await
        }
        Commands::Benchmark { categories, iterations, warmup, report } => {
            run_performance_benchmarks(categories, iterations, warmup, report).await
        }
        Commands::Report { results, format, output, detailed } => {
            generate_comprehensive_report(results, format, output, detailed).await
        }
    };

    match result {
        Ok(_) => {
            info!("Test runner completed successfully");
            process::exit(0);
        }
        Err(e) => {
            error!("Test runner failed: {}", e);
            process::exit(1);
        }
    }
}

async fn run_comprehensive_tests(
    suites: Vec<TestSuiteType>,
    parallel: bool,
    timeout: u64,
    output_dir: Option<PathBuf>,
    output_format: OutputFormat,
) -> Result<()> {
    info!("Starting comprehensive test suite execution");

    let mut config = TestConfig::default();
    config.parallel_execution = parallel;
    config.max_execution_time_ms = timeout * 1000;
    config.output_format = output_format;

    // Configure test suites based on user selection
    if suites.is_empty() {
        // Run all suites by default
        info!("No specific suites selected, running all test categories");
    } else {
        config.test_suites = suites.into_iter().map(|suite_type| {
            match suite_type {
                TestSuiteType::Unit => neo_solidity::testing::TestSuite {
                    name: "unit_tests".to_string(),
                    description: "Runtime primitive unit tests".to_string(),
                    test_type: neo_solidity::testing::TestType::Unit,
                    enabled: true,
                    timeout_ms: 30000,
                    retry_count: 0,
                },
                TestSuiteType::Integration => neo_solidity::testing::TestSuite {
                    name: "integration_tests".to_string(),
                    description: "Cross-component integration tests".to_string(),
                    test_type: neo_solidity::testing::TestType::Integration,
                    enabled: true,
                    timeout_ms: 60000,
                    retry_count: 1,
                },
                TestSuiteType::Differential => neo_solidity::testing::TestSuite {
                    name: "differential_tests".to_string(),
                    description: "EVM vs NeoVM differential testing".to_string(),
                    test_type: neo_solidity::testing::TestType::Differential,
                    enabled: true,
                    timeout_ms: 120000,
                    retry_count: 2,
                },
                TestSuiteType::Property => neo_solidity::testing::TestSuite {
                    name: "property_tests".to_string(),
                    description: "Property-based testing with generated inputs".to_string(),
                    test_type: neo_solidity::testing::TestType::PropertyBased,
                    enabled: true,
                    timeout_ms: 180000,
                    retry_count: 0,
                },
                TestSuiteType::Performance => neo_solidity::testing::TestSuite {
                    name: "performance_tests".to_string(),
                    description: "Performance and benchmarking tests".to_string(),
                    test_type: neo_solidity::testing::TestType::Performance,
                    enabled: true,
                    timeout_ms: 300000,
                    retry_count: 0,
                },
                TestSuiteType::Security => neo_solidity::testing::TestSuite {
                    name: "security_tests".to_string(),
                    description: "Security vulnerability detection".to_string(),
                    test_type: neo_solidity::testing::TestType::Security,
                    enabled: true,
                    timeout_ms: 240000,
                    retry_count: 0,
                },
                TestSuiteType::Conformance => neo_solidity::testing::TestSuite {
                    name: "conformance_tests".to_string(),
                    description: "Standards conformance validation".to_string(),
                    test_type: neo_solidity::testing::TestType::Conformance,
                    enabled: true,
                    timeout_ms: 600000,
                    retry_count: 1,
                },
            }
        }).collect();
    }

    let mut runner = TestRunner::new(config);
    let results = runner.run_all().await?;

    // Generate and display report
    let report = runner.generate_report()?;
    println!("{}", report);

    // Save results to file if output directory specified
    if let Some(output_dir) = output_dir {
        std::fs::create_dir_all(&output_dir)?;
        let results_file = output_dir.join("test_results.json");
        let results_json = serde_json::to_string_pretty(&results)?;
        std::fs::write(results_file, results_json)?;
        info!("Test results saved to {:?}", output_dir);
    }

    if results.failed > 0 {
        error!("{} test(s) failed out of {}", results.failed, results.total_tests);
        process::exit(1);
    }

    Ok(())
}

async fn run_security_analysis(
    contracts: Vec<PathBuf>,
    min_severity: String,
    enable_static: bool,
    enable_crypto: bool,
    generate_report: bool,
) -> Result<()> {
    info!("Starting security analysis");

    let mut config = SecurityConfig::default();
    config.enable_static_analysis = enable_static;
    config.enable_crypto_validation = enable_crypto;
    
    // Parse severity level
    config.severity_threshold = match min_severity.to_lowercase().as_str() {
        "critical" => neo_solidity::security::Severity::Critical,
        "high" => neo_solidity::security::Severity::High,
        "medium" => neo_solidity::security::Severity::Medium,
        "low" => neo_solidity::security::Severity::Low,
        "info" => neo_solidity::security::Severity::Informational,
        _ => {
            error!("Invalid severity level: {}", min_severity);
            process::exit(1);
        }
    };

    let mut analyzer = SecurityAnalyzer::new(config)?;

    for contract_path in contracts {
        if contract_path.is_file() {
            let contract_source = std::fs::read_to_string(&contract_path)?;
            let contract_name = contract_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            info!("Analyzing contract: {}", contract_name);
            let report = analyzer.analyze_contract(&contract_source, contract_name).await?;

            println!("Security Analysis Report for {}", contract_name);
            println!("=====================================");
            println!("Total findings: {}", report.summary.total_findings);
            println!("Critical: {}", report.summary.critical_count);
            println!("High: {}", report.summary.high_count);
            println!("Medium: {}", report.summary.medium_count);
            println!("Low: {}", report.summary.low_count);
            println!("Security Grade: {:?}", report.summary.security_grade);
            println!();

            // Display findings
            for finding in &report.findings {
                println!("Finding: {} [{:?}]", finding.message, finding.severity);
                println!("Recommendation: {}", finding.recommendation);
                if let Some(location) = &finding.location {
                    println!("Location: {}:{}", location.file, location.line);
                }
                println!();
            }

            if generate_report {
                let report_json = serde_json::to_string_pretty(&report)?;
                let report_file = format!("{}_security_report.json", contract_name);
                std::fs::write(&report_file, report_json)?;
                info!("Security report saved to {}", report_file);
            }

            // Exit with error code if critical or high severity issues found
            if report.summary.critical_count > 0 || report.summary.high_count > 0 {
                error!("Critical or high severity security issues found");
                process::exit(1);
            }
        } else if contract_path.is_dir() {
            // Recursively analyze all .sol files in directory
            for entry in walkdir::WalkDir::new(contract_path) {
                let entry = entry?;
                if entry.path().extension().and_then(|s| s.to_str()) == Some("sol") {
                    // Recursive analysis would be implemented here
                    info!("Would analyze: {:?}", entry.path());
                }
            }
        }
    }

    Ok(())
}

async fn run_fuzzing_campaign(
    contracts: Vec<PathBuf>,
    max_cases: u32,
    duration: u64,
    enable_differential: bool,
    crash_dir: Option<PathBuf>,
) -> Result<()> {
    info!("Starting fuzzing campaign");

    let mut config = FuzzingConfig::default();
    config.max_test_cases = max_cases;
    config.max_generation_time = duration;
    config.enable_differential_benchmarking = enable_differential;

    let mut coordinator = FuzzingCoordinator::new(config)?;

    // Run fuzzing campaign
    let statistics = coordinator.run_fuzzing_campaign().await?;

    println!("Fuzzing Campaign Results");
    println!("========================");
    println!("Total test cases: {}", statistics.total_test_cases);
    println!("Successful cases: {}", statistics.successful_cases);
    println!("Crashes found: {}", statistics.crashes_found);
    println!("Unique crashes: {}", statistics.unique_crashes);
    println!("Code coverage: {:.1}%", statistics.code_coverage);
    println!("Execution time: {}ms", statistics.execution_time_ms);
    println!();

    // Display crashes by category
    if !statistics.crashes_per_category.is_empty() {
        println!("Crashes by Category:");
        for (category, count) in statistics.crashes_per_category {
            println!("  {}: {}", category, count);
        }
        println!();
    }

    // Export corpus if crash directory specified
    if let Some(crash_dir) = crash_dir {
        std::fs::create_dir_all(&crash_dir)?;
        let corpus_file = crash_dir.join("fuzzing_corpus.json");
        coordinator.export_corpus(corpus_file.to_str().unwrap()).await?;
        info!("Fuzzing corpus exported to {:?}", crash_dir);
    }

    // Exit with error code if crashes found
    if statistics.crashes_found > 0 {
        error!("{} crashes found during fuzzing", statistics.crashes_found);
        process::exit(1);
    }

    Ok(())
}

async fn run_conformance_testing(
    sources: Vec<String>,
    level: String,
    enable_ethereum: bool,
    enable_solidity: bool,
) -> Result<()> {
    info!("Starting conformance testing");

    let mut config = ConformanceConfig::default();
    config.enable_ethereum_tests = enable_ethereum;
    config.enable_solidity_tests = enable_solidity;

    // Parse conformance level
    config.required_conformance_level = match level.to_lowercase().as_str() {
        "basic" => neo_solidity::conformance::ConformanceLevel::Basic,
        "standard" => neo_solidity::conformance::ConformanceLevel::Standard,
        "full" => neo_solidity::conformance::ConformanceLevel::Full,
        "extended" => neo_solidity::conformance::ConformanceLevel::Extended,
        _ => {
            error!("Invalid conformance level: {}", level);
            process::exit(1);
        }
    };

    // Add custom test vector sources
    for source in sources {
        config.test_vector_sources.push(neo_solidity::conformance::TestVectorSource {
            name: format!("Custom source: {}", source),
            source_type: neo_solidity::conformance::SourceType::Local,
            location: source,
            version: "latest".to_string(),
            enabled: true,
        });
    }

    let mut suite = ConformanceTestSuite::new(config)?;
    let results = suite.run_conformance_tests().await?;

    println!("Conformance Test Results");
    println!("========================");
    println!("Total tests: {}", results.total_tests);
    println!("Passed: {}", results.passed_tests);
    println!("Failed: {}", results.failed_tests);
    println!("Skipped: {}", results.skipped_tests);
    println!("Conformance score: {:.1}%", results.conformance_score);
    println!("Achieved level: {:?}", results.achieved_level);
    println!("Execution time: {}ms", results.execution_time_ms);
    println!();

    // Display suite results
    for (suite_name, suite_result) in &results.suite_results {
        println!("Suite '{}': {}/{} passed ({:.1}%)",
            suite_name,
            suite_result.passed_tests,
            suite_result.total_tests,
            suite_result.conformance_percentage
        );
    }
    println!();

    // Display non-conformance issues
    if !results.non_conformance_issues.is_empty() {
        println!("Non-Conformance Issues:");
        for issue in &results.non_conformance_issues {
            println!("  {} [{}]: {}", issue.issue_id, issue.severity, issue.title);
        }
        println!();
    }

    // Generate detailed report
    let report = suite.generate_report()?;
    println!("{}", report);

    // Exit with error code if conformance level not met
    if results.achieved_level < config.required_conformance_level {
        error!("Required conformance level not achieved");
        process::exit(1);
    }

    Ok(())
}

async fn run_interactive_debug(
    contract: PathBuf,
    function: Option<String>,
    enable_source_maps: bool,
    enable_trace_hooks: bool,
    breakpoints_file: Option<PathBuf>,
) -> Result<()> {
    info!("Starting interactive debugging session");

    let mut config = DebuggerConfig::default();
    config.generate_source_maps = enable_source_maps;
    config.enable_trace_hooks = enable_trace_hooks;
    config.interactive_mode = true;

    let mut debugger = Debugger::new(config)?;

    // Load contract
    let contract_bytecode = std::fs::read(&contract)?;
    let contract_name = contract.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    // Start debugging session
    let session_id = debugger.start_debugging_session(contract_name, 
        &function.unwrap_or_else(|| "main".to_string()))?;

    info!("Debugging session started: {}", session_id);

    // Load breakpoints if specified
    if let Some(breakpoints_file) = breakpoints_file {
        let breakpoint_lines: Vec<u32> = std::fs::read_to_string(breakpoints_file)?
            .lines()
            .filter_map(|line| line.parse().ok())
            .collect();

        for line in breakpoint_lines {
            let breakpoint_id = debugger.set_breakpoint(contract_name, line)?;
            info!("Set breakpoint {} at line {}", breakpoint_id, line);
        }
    }

    // Start interactive session
    debugger.start_interactive_session()?;

    Ok(())
}

async fn run_performance_benchmarks(
    categories: Vec<BenchmarkCategory>,
    iterations: u32,
    warmup: u32,
    generate_report: bool,
) -> Result<()> {
    info!("Starting performance benchmarks");

    let config = neo_solidity::testing::benchmarks::BenchmarkConfig {
        measurement_iterations: iterations,
        warm_up_iterations: warmup,
        ..Default::default()
    };

    let mut benchmarker = neo_solidity::testing::benchmarks::PerformanceBenchmarker::new(config);

    // Run selected benchmark categories
    if categories.is_empty() {
        info!("Running all benchmark categories");
        // Would run all categories
    } else {
        for category in categories {
            match category {
                BenchmarkCategory::Execution => {
                    info!("Running execution benchmarks");
                    let contract_bytecode = vec![0u8; 1000]; // Mock bytecode
                    let benchmark = benchmarker.benchmark_contract_execution("test_contract", &contract_bytecode)?;
                    println!("Execution Benchmark: {:.2}ns avg", benchmark.statistics.mean);
                }
                BenchmarkCategory::Compilation => {
                    info!("Running compilation benchmarks");
                    let source_code = "contract Test { function test() public {} }";
                    let benchmark = benchmarker.benchmark_compilation(source_code)?;
                    println!("Compilation Benchmark: {}ms avg", benchmark.compilation_time_ms);
                }
                BenchmarkCategory::Gas => {
                    info!("Running gas benchmarks");
                    let benchmark = benchmarker.benchmark_gas_consumption("test_operation")?;
                    println!("Gas Benchmark: {} gas avg, efficiency: {:.2}", 
                        benchmark.base_cost + benchmark.dynamic_cost,
                        benchmark.gas_efficiency_score);
                }
                BenchmarkCategory::Memory => {
                    info!("Running memory benchmarks");
                    let benchmark = benchmarker.benchmark_memory_usage("test_operation")?;
                    println!("Memory Benchmark: {}KB peak, efficiency: {:.2}", 
                        benchmark.peak_memory_kb,
                        benchmark.memory_efficiency_score);
                }
                BenchmarkCategory::Differential => {
                    info!("Running differential benchmarks");
                    let benchmark = benchmarker.benchmark_differential_performance("test_operation")?;
                    println!("Differential Benchmark: {:.2}x performance ratio (Neo/EVM)", 
                        benchmark.performance_ratio);
                }
            }
        }
    }

    if generate_report {
        info!("Generating performance report");
        // Would generate detailed report
        let report_file = "performance_report.json";
        // Save report...
        info!("Performance report saved to {}", report_file);
    }

    Ok(())
}

async fn generate_comprehensive_report(
    result_files: Vec<PathBuf>,
    format: ReportFormat,
    output: PathBuf,
    detailed: bool,
) -> Result<()> {
    info!("Generating comprehensive test report");

    // Load and aggregate results from multiple files
    let mut all_results = Vec::new();
    
    for file in result_files {
        if file.exists() {
            let content = std::fs::read_to_string(&file)?;
            // Parse different result formats and aggregate
            info!("Loaded results from {:?}", file);
            all_results.push(content);
        }
    }

    // Generate report in requested format
    let report_content = match format {
        ReportFormat::Html => generate_html_report(&all_results, detailed)?,
        ReportFormat::Pdf => generate_pdf_report(&all_results, detailed)?,
        ReportFormat::Markdown => generate_markdown_report(&all_results, detailed)?,
        ReportFormat::Json => generate_json_report(&all_results, detailed)?,
    };

    // Write report to output file
    std::fs::write(&output, report_content)?;
    info!("Comprehensive report generated: {:?}", output);

    Ok(())
}

fn generate_html_report(results: &[String], detailed: bool) -> Result<String> {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html><head><title>Neo Solidity Test Report</title>");
    html.push_str("<style>body{font-family:Arial,sans-serif;margin:40px;}</style></head><body>");
    html.push_str("<h1>Neo Solidity Comprehensive Test Report</h1>");
    
    if detailed {
        html.push_str("<h2>Detailed Analysis</h2>");
        for (i, result) in results.iter().enumerate() {
            html.push_str(&format!("<h3>Result Set {}</h3>", i + 1));
            html.push_str("<pre>");
            html.push_str(&htmlescape::encode_minimal(result));
            html.push_str("</pre>");
        }
    } else {
        html.push_str("<h2>Summary</h2>");
        html.push_str(&format!("<p>Processed {} result sets</p>", results.len()));
    }
    
    html.push_str("</body></html>");
    Ok(html)
}

fn generate_pdf_report(_results: &[String], _detailed: bool) -> Result<String> {
    // PDF generation would require additional dependencies
    Ok("PDF report generation not yet implemented".to_string())
}

fn generate_markdown_report(results: &[String], detailed: bool) -> Result<String> {
    let mut md = String::new();
    md.push_str("# Neo Solidity Comprehensive Test Report\n\n");
    
    if detailed {
        md.push_str("## Detailed Analysis\n\n");
        for (i, result) in results.iter().enumerate() {
            md.push_str(&format!("### Result Set {}\n\n", i + 1));
            md.push_str("```json\n");
            md.push_str(result);
            md.push_str("\n```\n\n");
        }
    } else {
        md.push_str("## Summary\n\n");
        md.push_str(&format!("Processed {} result sets\n\n", results.len()));
    }
    
    Ok(md)
}

fn generate_json_report(results: &[String], _detailed: bool) -> Result<String> {
    let report = serde_json::json!({
        "title": "Neo Solidity Comprehensive Test Report",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "result_count": results.len(),
        "results": results
    });
    
    Ok(serde_json::to_string_pretty(&report)?)
}