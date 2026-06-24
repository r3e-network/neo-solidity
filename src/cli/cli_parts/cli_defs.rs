use super::*;

pub(crate) const COMPILER_ID: &str = concat!("neo-devpack-solidity-", env!("CARGO_PKG_VERSION"));
pub(crate) const COMPILER_EMAIL: &str = "Jimmy <jimmy@r3e.network>";
pub(crate) const VERSION_STR: &str = env!("CARGO_PKG_VERSION");

pub(crate) fn compiler_version_string_4() -> String {
    let mut parts = VERSION_STR
        .split('.')
        .map(|p| p.parse::<u32>().unwrap_or(0));
    let major = parts.next().unwrap_or(0);
    let minor = parts.next().unwrap_or(0);
    let patch = parts.next().unwrap_or(0);
    format!("{major}.{minor}.{patch}.0")
}

#[derive(Clone, Debug)]
pub struct CompilationArtifacts {
    pub metadata: ContractMetadata,
    pub bytecode: Vec<u8>,
    pub tokens: Vec<neo_devpack_solidity::neo::MethodToken>,
    pub manifest: Value,
    pub warnings: Vec<neo_devpack_solidity::solidity::Diagnostic>,
}

#[derive(Debug)]
pub enum CompileError {
    Diagnostics(Vec<neo_devpack_solidity::solidity::Diagnostic>),
    Semantic(Vec<neo_devpack_solidity::solidity::Diagnostic>),
    Ir(Vec<neo_devpack_solidity::ir::IrDiagnostic>),
    Manifest(String),
    /// Structured parser diagnostics — emitted as one standard-JSON error per
    /// diagnostic with a precise `sourceLocation`, instead of one Generic blob.
    ParseErrors(Vec<neo_devpack_solidity::frontend::ParseDiagnostic>),
    Message(String),
}
