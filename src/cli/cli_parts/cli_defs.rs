const COMPILER_ID: &str = concat!("neo-solidity-", env!("CARGO_PKG_VERSION"));
const COMPILER_EMAIL: &str = "Jimmy <jimmy@r3e.network>";
const VERSION_STR: &str = env!("CARGO_PKG_VERSION");

fn compiler_version_string_4() -> String {
    let mut parts = VERSION_STR.split('.').map(|p| p.parse::<u32>().unwrap_or(0));
    let major = parts.next().unwrap_or(0);
    let minor = parts.next().unwrap_or(0);
    let patch = parts.next().unwrap_or(0);
    format!("{major}.{minor}.{patch}.0")
}

#[derive(Clone, Debug)]
pub struct CompilationArtifacts {
    pub metadata: ContractMetadata,
    pub bytecode: Vec<u8>,
    pub tokens: Vec<neo_solidity::neo::MethodToken>,
    pub manifest: Value,
    pub warnings: Vec<neo_solidity::solidity::Diagnostic>,
}

#[derive(Debug)]
pub enum CompileError {
    Diagnostics(Vec<neo_solidity::solidity::Diagnostic>),
    Semantic(Vec<neo_solidity::solidity::Diagnostic>),
    Ir(Vec<neo_solidity::ir::IrDiagnostic>),
    Manifest(String),
    Message(String),
}
