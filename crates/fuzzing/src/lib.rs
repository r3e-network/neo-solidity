//! Fuzzing Harnesses for Neo Solidity ABI Boundaries
//! 
//! This crate provides comprehensive fuzzing capabilities for:
//! - ABI encoding/decoding boundaries
//! - Contract function call data
//! - Transaction data validation
//! - Type system boundaries
//! - Gas limit edge cases

pub mod abi_fuzzer;
pub mod data_fuzzer;
pub mod transaction_fuzzer;
pub mod contract_fuzzer;
pub mod generators;
pub mod validators;
pub mod corpus;
pub mod coverage;

use arbitrary::Arbitrary;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

/// Main fuzzing coordinator
pub struct FuzzingCoordinator {
    config: FuzzingConfig,
    corpus: corpus::FuzzingCorpus,
    coverage: coverage::CoverageTracker,
    statistics: FuzzingStatistics,
}

/// Fuzzing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzingConfig {
    /// Enable ABI encoding/decoding fuzzing
    pub enable_abi_fuzzing: bool,
    /// Enable transaction data fuzzing
    pub enable_transaction_fuzzing: bool,
    /// Enable contract call fuzzing
    pub enable_contract_fuzzing: bool,
    /// Maximum test case generation time (seconds)
    pub max_generation_time: u64,
    /// Maximum number of test cases per category
    pub max_test_cases: u32,
    /// Seed for reproducible fuzzing
    pub seed: Option<u64>,
    /// Target coverage percentage
    pub target_coverage: f64,
    /// Crash detection settings
    pub crash_detection: CrashDetectionConfig,
}

/// Crash detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashDetectionConfig {
    /// Detect memory corruption
    pub detect_memory_corruption: bool,
    /// Detect buffer overflows
    pub detect_buffer_overflows: bool,
    /// Detect integer overflows
    pub detect_integer_overflows: bool,
    /// Detect infinite loops
    pub detect_infinite_loops: bool,
    /// Maximum execution time before timeout (ms)
    pub execution_timeout_ms: u64,
}

/// Fuzzing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzingStatistics {
    pub total_test_cases: u32,
    pub successful_cases: u32,
    pub crashes_found: u32,
    pub hangs_detected: u32,
    pub unique_crashes: u32,
    pub code_coverage: f64,
    pub execution_time_ms: u64,
    pub crashes_per_category: HashMap<String, u32>,
}

/// Fuzzable data types for ABI boundaries
#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize)]
pub enum FuzzableValue {
    // Basic types
    Bool(bool),
    Uint8(u8),
    Uint16(u16), 
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    Uint256([u8; 32]),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Int256([u8; 32]),
    
    // Dynamic types
    Address([u8; 20]),
    Bytes(Vec<u8>),
    String(String),
    
    // Composite types
    Array(Vec<FuzzableValue>),
    Tuple(Vec<FuzzableValue>),
    
    // Special edge case values
    MaxValue,
    MinValue,
    Zero,
    Null,
    Random(Vec<u8>),
}

/// ABI function signature for fuzzing
#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize)]
pub struct FuzzableFunction {
    pub name: String,
    pub inputs: Vec<FuzzableParameter>,
    pub outputs: Vec<FuzzableParameter>,
    pub stateMutability: StateMutability,
    pub payable: bool,
}

/// Function parameter
#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize)]
pub struct FuzzableParameter {
    pub name: String,
    pub type_info: TypeInfo,
    pub value: FuzzableValue,
}

/// Type information
#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize)]
pub enum TypeInfo {
    Elementary(String),
    Array {
        base_type: Box<TypeInfo>,
        size: Option<u32>,
    },
    Tuple(Vec<TypeInfo>),
    Mapping {
        key_type: Box<TypeInfo>,
        value_type: Box<TypeInfo>,
    },
}

/// State mutability
#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize)]
pub enum StateMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

/// Transaction data for fuzzing
#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize)]
pub struct FuzzableTransaction {
    pub to: Option<[u8; 20]>,
    pub value: u64,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub nonce: u64,
    pub data: Vec<u8>,
    pub chain_id: Option<u64>,
}

/// Contract call data
#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize)]
pub struct FuzzableContractCall {
    pub contract_address: [u8; 20],
    pub function: FuzzableFunction,
    pub call_data: Vec<u8>,
    pub value: u64,
    pub gas_limit: u64,
    pub caller: [u8; 20],
}

/// Fuzzing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FuzzingResult {
    Success {
        test_case: String,
        execution_time_ms: u64,
        gas_used: u64,
        return_data: Vec<u8>,
    },
    Crash {
        test_case: String,
        error_type: CrashType,
        error_message: String,
        stack_trace: Option<String>,
        reproduction_data: Vec<u8>,
    },
    Hang {
        test_case: String,
        timeout_ms: u64,
    },
    InvalidInput {
        test_case: String,
        validation_error: String,
    },
}

/// Types of crashes detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrashType {
    MemoryCorruption,
    BufferOverflow,
    IntegerOverflow,
    IntegerUnderflow,
    DivisionByZero,
    NullPointerDereference,
    StackOverflow,
    HeapCorruption,
    AssertionFailure,
    PanicError,
    ABIDecodingError,
    ABIEncodingError,
    GasExhaustion,
    InfiniteLoop,
    UnhandledException,
}

impl FuzzingCoordinator {
    /// Create new fuzzing coordinator
    pub fn new(config: FuzzingConfig) -> Result<Self> {
        let corpus = corpus::FuzzingCorpus::new()?;
        let coverage = coverage::CoverageTracker::new()?;
        
        Ok(Self {
            config,
            corpus,
            coverage,
            statistics: FuzzingStatistics::default(),
        })
    }

    /// Run comprehensive fuzzing campaign
    pub async fn run_fuzzing_campaign(&mut self) -> Result<FuzzingStatistics> {
        tracing::info!("Starting comprehensive fuzzing campaign");
        
        let start_time = std::time::Instant::now();
        
        // ABI boundary fuzzing
        if self.config.enable_abi_fuzzing {
            self.run_abi_fuzzing().await?;
        }

        // Transaction data fuzzing
        if self.config.enable_transaction_fuzzing {
            self.run_transaction_fuzzing().await?;
        }

        // Contract call fuzzing
        if self.config.enable_contract_fuzzing {
            self.run_contract_fuzzing().await?;
        }

        self.statistics.execution_time_ms = start_time.elapsed().as_millis() as u64;
        self.statistics.code_coverage = self.coverage.get_overall_coverage();

        tracing::info!(
            "Fuzzing campaign completed: {} test cases, {} crashes found in {}ms",
            self.statistics.total_test_cases,
            self.statistics.crashes_found,
            self.statistics.execution_time_ms
        );

        Ok(self.statistics.clone())
    }

    /// Run ABI encoding/decoding fuzzing
    async fn run_abi_fuzzing(&mut self) -> Result<()> {
        tracing::info!("Running ABI fuzzing");
        
        for _ in 0..self.config.max_test_cases {
            // Generate random ABI data
            let function = self.generate_random_function()?;
            
            // Test encoding
            match self.test_abi_encoding(&function).await {
                Ok(result) => self.handle_fuzzing_result(result).await?,
                Err(e) => self.handle_fuzzing_error("abi_encoding", e).await?,
            }
            
            // Test decoding
            match self.test_abi_decoding(&function).await {
                Ok(result) => self.handle_fuzzing_result(result).await?,
                Err(e) => self.handle_fuzzing_error("abi_decoding", e).await?,
            }
            
            self.statistics.total_test_cases += 2;
        }

        Ok(())
    }

    /// Run transaction data fuzzing
    async fn run_transaction_fuzzing(&mut self) -> Result<()> {
        tracing::info!("Running transaction fuzzing");
        
        for _ in 0..self.config.max_test_cases {
            let transaction = self.generate_random_transaction()?;
            
            match self.test_transaction_processing(&transaction).await {
                Ok(result) => self.handle_fuzzing_result(result).await?,
                Err(e) => self.handle_fuzzing_error("transaction_processing", e).await?,
            }
            
            self.statistics.total_test_cases += 1;
        }

        Ok(())
    }

    /// Run contract call fuzzing
    async fn run_contract_fuzzing(&mut self) -> Result<()> {
        tracing::info!("Running contract fuzzing");
        
        for _ in 0..self.config.max_test_cases {
            let contract_call = self.generate_random_contract_call()?;
            
            match self.test_contract_call(&contract_call).await {
                Ok(result) => self.handle_fuzzing_result(result).await?,
                Err(e) => self.handle_fuzzing_error("contract_call", e).await?,
            }
            
            self.statistics.total_test_cases += 1;
        }

        Ok(())
    }

    /// Generate random function for testing
    fn generate_random_function(&self) -> Result<FuzzableFunction> {
        generators::generate_random_function(&self.config)
    }

    /// Generate random transaction
    fn generate_random_transaction(&self) -> Result<FuzzableTransaction> {
        generators::generate_random_transaction(&self.config)
    }

    /// Generate random contract call
    fn generate_random_contract_call(&self) -> Result<FuzzableContractCall> {
        generators::generate_random_contract_call(&self.config)
    }

    /// Test ABI encoding
    async fn test_abi_encoding(&mut self, function: &FuzzableFunction) -> Result<FuzzingResult> {
        abi_fuzzer::test_encoding(function, &self.config).await
    }

    /// Test ABI decoding
    async fn test_abi_decoding(&mut self, function: &FuzzableFunction) -> Result<FuzzingResult> {
        abi_fuzzer::test_decoding(function, &self.config).await
    }

    /// Test transaction processing
    async fn test_transaction_processing(&mut self, transaction: &FuzzableTransaction) -> Result<FuzzingResult> {
        transaction_fuzzer::test_transaction(transaction, &self.config).await
    }

    /// Test contract call
    async fn test_contract_call(&mut self, call: &FuzzableContractCall) -> Result<FuzzingResult> {
        contract_fuzzer::test_contract_call(call, &self.config).await
    }

    /// Handle fuzzing result
    async fn handle_fuzzing_result(&mut self, result: FuzzingResult) -> Result<()> {
        match &result {
            FuzzingResult::Success { .. } => {
                self.statistics.successful_cases += 1;
            }
            FuzzingResult::Crash { error_type, reproduction_data, .. } => {
                self.statistics.crashes_found += 1;
                let category = format!("{:?}", error_type);
                *self.statistics.crashes_per_category.entry(category).or_insert(0) += 1;
                
                // Store crash for reproduction
                self.corpus.store_crash(&result, reproduction_data.clone()).await?;
            }
            FuzzingResult::Hang { .. } => {
                self.statistics.hangs_detected += 1;
            }
            FuzzingResult::InvalidInput { .. } => {
                // Count but don't treat as failure
            }
        }

        // Update coverage
        self.coverage.update_from_result(&result).await?;
        
        Ok(())
    }

    /// Handle fuzzing error
    async fn handle_fuzzing_error(&mut self, category: &str, error: anyhow::Error) -> Result<()> {
        tracing::warn!("Fuzzing error in {}: {}", category, error);
        
        // Convert error to crash result
        let crash_result = FuzzingResult::Crash {
            test_case: category.to_string(),
            error_type: CrashType::UnhandledException,
            error_message: error.to_string(),
            stack_trace: None,
            reproduction_data: vec![],
        };
        
        self.handle_fuzzing_result(crash_result).await
    }

    /// Generate crash reproduction test case
    pub fn generate_reproduction_test(&self, crash_data: &[u8]) -> Result<String> {
        corpus::generate_reproduction_test(crash_data)
    }

    /// Get coverage report
    pub fn get_coverage_report(&self) -> coverage::CoverageReport {
        self.coverage.generate_report()
    }

    /// Export corpus for regression testing
    pub async fn export_corpus(&self, output_path: &str) -> Result<()> {
        self.corpus.export_to_file(output_path).await
    }
}

impl Default for FuzzingConfig {
    fn default() -> Self {
        Self {
            enable_abi_fuzzing: true,
            enable_transaction_fuzzing: true,
            enable_contract_fuzzing: true,
            max_generation_time: 300, // 5 minutes
            max_test_cases: 10000,
            seed: None,
            target_coverage: 80.0,
            crash_detection: CrashDetectionConfig::default(),
        }
    }
}

impl Default for CrashDetectionConfig {
    fn default() -> Self {
        Self {
            detect_memory_corruption: true,
            detect_buffer_overflows: true,
            detect_integer_overflows: true,
            detect_infinite_loops: true,
            execution_timeout_ms: 10000, // 10 seconds
        }
    }
}

impl Default for FuzzingStatistics {
    fn default() -> Self {
        Self {
            total_test_cases: 0,
            successful_cases: 0,
            crashes_found: 0,
            hangs_detected: 0,
            unique_crashes: 0,
            code_coverage: 0.0,
            execution_time_ms: 0,
            crashes_per_category: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fuzzing_coordinator_creation() {
        let config = FuzzingConfig::default();
        let coordinator = FuzzingCoordinator::new(config);
        assert!(coordinator.is_ok());
    }

    #[test]
    fn test_fuzzable_value_generation() {
        use arbitrary::Unstructured;
        
        let data = [0u8; 1024];
        let mut unstructured = Unstructured::new(&data);
        
        let value = FuzzableValue::arbitrary(&mut unstructured);
        assert!(value.is_ok());
    }

    #[test]
    fn test_crash_type_serialization() {
        let crash = CrashType::BufferOverflow;
        let json = serde_json::to_string(&crash).unwrap();
        let deserialized: CrashType = serde_json::from_str(&json).unwrap();
        
        match (crash, deserialized) {
            (CrashType::BufferOverflow, CrashType::BufferOverflow) => (),
            _ => panic!("Serialization failed"),
        }
    }
}