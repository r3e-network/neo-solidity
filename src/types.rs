//! Compiler Configuration Types
//!
//! This module defines configuration structures for the Neo Solidity compiler,
//! including input/output settings, optimization levels, and output formats.
//!
//! # Key Types
//!
//! - [`CompilerConfig`] - Main configuration for compilation
//! - [`OutputFormat`] - Supported output formats (NEF, JSON, etc.)
//! - [`GasModel`] - Gas calculation model selection

use std::path::PathBuf;

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
    pub warnings_as_errors: bool,
    pub max_contract_size: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Nef,       // Neo Executable Format (.nef)
    Manifest,  // Neo Manifest (.manifest.json)
    Complete,  // Both .nef and .manifest.json
    Assembly,  // Human-readable assembly
    Json,      // Complete JSON with all outputs
    DebugInfo, // Debug information
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
            output_format: OutputFormat::Complete,
            target_version: "3.0".to_string(),
            include_debug_info: false,
            include_abi: true,
            include_source_map: false,
            gas_model: GasModel::Neo,
            validate_only: false,
            analyze_only: false,
            verbose: false,
            warnings_as_errors: false,
            max_contract_size: 512 * 1024,
        }
    }
}

impl CompilerConfig {
    /// Create a new builder for CompilerConfig
    pub fn builder() -> CompilerConfigBuilder {
        CompilerConfigBuilder::default()
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.optimization_level > 3 {
            return Err(ConfigError::InvalidOptimizationLevel(
                self.optimization_level,
            ));
        }
        if self.input_file.as_os_str().is_empty() {
            return Err(ConfigError::MissingInputFile);
        }
        Ok(())
    }

    /// Check if debug output is enabled
    pub fn is_debug_enabled(&self) -> bool {
        self.include_debug_info || self.verbose
    }

    /// Check if optimization is enabled
    pub fn is_optimized(&self) -> bool {
        self.optimization_level > 0
    }

    /// Get optimization passes based on level
    pub fn optimization_passes(&self) -> u32 {
        match self.optimization_level {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            _ => 2,
        }
    }
}

/// Builder for CompilerConfig with fluent API
#[derive(Debug, Clone, Default)]
pub struct CompilerConfigBuilder {
    config: CompilerConfig,
}

impl CompilerConfigBuilder {
    pub fn input_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.config.input_file = path.into();
        self
    }

    pub fn output_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.config.output_file = Some(path.into());
        self
    }

    pub fn optimization_level(mut self, level: u8) -> Self {
        self.config.optimization_level = level;
        self
    }

    pub fn output_format(mut self, format: OutputFormat) -> Self {
        self.config.output_format = format;
        self
    }

    pub fn target_version(mut self, version: impl Into<String>) -> Self {
        self.config.target_version = version.into();
        self
    }

    pub fn debug_info(mut self, enabled: bool) -> Self {
        self.config.include_debug_info = enabled;
        self
    }

    pub fn verbose(mut self, enabled: bool) -> Self {
        self.config.verbose = enabled;
        self
    }

    pub fn gas_model(mut self, model: GasModel) -> Self {
        self.config.gas_model = model;
        self
    }

    pub fn warnings_as_errors(mut self, enabled: bool) -> Self {
        self.config.warnings_as_errors = enabled;
        self
    }

    pub fn max_contract_size(mut self, size: usize) -> Self {
        self.config.max_contract_size = size;
        self
    }

    pub fn include_abi(mut self, enabled: bool) -> Self {
        self.config.include_abi = enabled;
        self
    }

    pub fn include_source_map(mut self, enabled: bool) -> Self {
        self.config.include_source_map = enabled;
        self
    }

    pub fn validate_only(mut self, enabled: bool) -> Self {
        self.config.validate_only = enabled;
        self
    }

    pub fn analyze_only(mut self, enabled: bool) -> Self {
        self.config.analyze_only = enabled;
        self
    }

    /// Build and validate the configuration
    pub fn build(self) -> Result<CompilerConfig, ConfigError> {
        self.config.validate()?;
        Ok(self.config)
    }

    /// Build without validation (for testing)
    pub fn build_unchecked(self) -> CompilerConfig {
        self.config
    }
}

/// Configuration validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    MissingInputFile,
    InvalidOptimizationLevel(u8),
    InvalidTargetVersion(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingInputFile => write!(f, "input file is required"),
            Self::InvalidOptimizationLevel(l) => {
                write!(f, "optimization level {l} is invalid (must be 0-3)")
            }
            Self::InvalidTargetVersion(v) => write!(f, "invalid target version: {v}"),
        }
    }
}

impl std::error::Error for ConfigError {}
