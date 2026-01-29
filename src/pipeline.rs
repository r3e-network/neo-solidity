//! Compilation Pipeline
//!
//! Orchestrates the compilation process.

/// Pipeline stage
#[derive(Debug, Clone, Copy)]
pub enum Stage {
    Parse,
    Analyze,
    Optimize,
    Codegen,
    Link,
}

/// Pipeline result
#[derive(Debug)]
pub struct PipelineResult {
    pub stage: Stage,
    pub success: bool,
}
