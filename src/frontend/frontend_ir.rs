/// Natspec documentation extracted from source comments.
#[derive(Debug, Clone, Default)]
pub struct NatspecDocIR {
    /// @title - Contract title
    pub title: Option<String>,
    /// @author - Author information
    pub author: Option<String>,
    /// @notice - User-facing description
    pub notice: Option<String>,
    /// @dev - Developer-facing notes
    pub dev: Option<String>,
    /// @param name description
    pub params: Vec<(String, String)>,
    /// @return descriptions
    pub returns: Vec<String>,
    /// @custom:tag value pairs
    pub custom: Vec<(String, String)>,
}

/// Representation of a Solidity contract.
#[derive(Debug, Clone)]
pub struct ContractIR {
    pub name: String,
    pub kind: ContractKind,
    /// Inheritance specifiers (`contract X is A, B(...) { ... }`).
    pub bases: Vec<Base>,
    pub functions: Vec<FunctionIR>,
    pub events: Vec<EventIR>,
    pub state_variables: Vec<StateVariableIR>,
    pub structs: Vec<StructIR>,
    pub enums: Vec<EnumIR>,
    /// Natspec documentation for this contract
    pub doc: NatspecDocIR,
}

/// Classification of contract kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractKind {
    Contract,
    AbstractContract,
    Interface,
    Library,
}

/// Representation of a Solidity function or constructor.
#[derive(Debug, Clone)]
pub struct FunctionIR {
    pub name: String,
    pub ty: FunctionTy,
    pub parameters: Vec<ParameterIR>,
    pub returns: Vec<ParameterIR>,
    pub mutability: MutabilityKind,
    pub visibility: VisibilityKind,
    /// Modifier applications and constructor base invocations.
    pub base_or_modifiers: Vec<Base>,
    pub body: Option<Statement>,
    /// Natspec documentation for this function
    pub doc: NatspecDocIR,
}

/// Function mutability classification based on Solidity state mutability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutabilityKind {
    Pure,
    View,
    Payable,
    NonPayable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityKind {
    External,
    Public,
    Internal,
    Private,
}

/// Representation of a Solidity parameter.
#[derive(Debug, Clone)]
pub struct ParameterIR {
    pub name: Option<String>,
    pub ty: String,
    pub storage: Option<String>,
}

/// Representation of a Solidity event.
#[derive(Debug, Clone)]
pub struct EventIR {
    pub name: String,
    pub parameters: Vec<EventParameterIR>,
}

/// Representation of a Solidity event parameter.
#[derive(Debug, Clone)]
pub struct EventParameterIR {
    pub name: Option<String>,
    pub ty: String,
    pub indexed: bool,
}

/// Representation of a state variable.
#[derive(Debug, Clone)]
pub struct StateVariableIR {
    pub name: Option<String>,
    pub ty: String,
    pub is_constant: bool,
    pub is_immutable: bool,
    pub visibility: Option<String>,
    pub has_initializer: bool,
    pub initializer: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct StructIR {
    pub name: String,
    pub fields: Vec<StructFieldIR>,
}

#[derive(Debug, Clone)]
pub struct StructFieldIR {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
pub struct EnumIR {
    pub name: String,
    pub values: Vec<String>,
}
