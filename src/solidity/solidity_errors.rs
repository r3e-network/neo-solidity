#[derive(Debug, Error)]
pub enum SolidityError {
    #[error("{0}")]
    Frontend(#[from] crate::frontend::FrontendError),
    #[error("{0}")]
    Analysis(String),
    #[error("no contract definitions found in source")]
    NoContracts,
}
