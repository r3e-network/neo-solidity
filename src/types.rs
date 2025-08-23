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
        }
    }
}
