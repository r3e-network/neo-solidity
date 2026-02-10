#[derive(Debug, Clone, Default)]
pub struct SelectorRegistry {
    /// Mapping of Solidity type name -> method name -> list of selectors (one per overload).
    ///
    /// This supports lowering expressions like `IERC20.transfer.selector` into a constant
    /// `bytes4` value and allows recovering Neo method names from `.selector` expressions in
    /// low-level EVM call shims (`abi.encodeWithSelector(...)`).
    pub type_method_selectors:
        std::collections::HashMap<String, std::collections::HashMap<String, Vec<[u8; 4]>>>,
    /// Set of known Solidity interface type names visible to the compilation unit.
    pub interface_types: std::collections::HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct ContractMetadata {
    pub name: String,
    /// Whether this contract was declared `abstract`.
    pub is_abstract: bool,
    /// Whether this contract was declared as a `library`.
    pub is_library: bool,
    pub methods: Vec<FunctionMetadata>,
    pub events: Vec<EventMetadata>,
    pub uses_storage: bool,
    pub state_variables: Vec<StateVariableMetadata>,
    pub structs: Vec<StructMetadata>,
    pub enums: Vec<EnumMetadata>,
    /// All contract/interface type names visible to this compilation unit.
    ///
    /// This is used during IR lowering to recognize Solidity-style contract casts
    /// like `IERC20(token).transfer(...)` without accidentally treating unknown
    /// function calls as casts.
    pub contract_types: Vec<String>,
    /// Registry of known function selectors for contract/interface types visible to this
    /// compilation unit (shared across contracts via `Arc`).
    pub selector_registry: std::sync::Arc<SelectorRegistry>,
    /// Natspec documentation for the contract
    pub documentation: NatspecDoc,
    /// Whether this contract contains `using X for *` directives.
    pub has_using_for_star: bool,
    /// Whether this contract contains `using { f, g } for Y` directives.
    pub has_using_function_list: bool,
    /// Library names referenced by `using X for Y` directives.
    pub using_for_libraries: Vec<String>,
    /// Whether this contract contains `type X is Y` definitions.
    pub has_type_definitions: bool,
    /// User-defined value type aliases (`type X is Y`).
    /// Maps type name to underlying Solidity type string.
    pub type_aliases: std::collections::HashMap<String, String>,
    /// Warnings collected during inheritance flattening (e.g. virtual/override checks).
    pub flatten_warnings: Vec<String>,
    /// Mapping from original method name to the renamed super-method name.
    /// Populated during inheritance flattening so `super.method()` can resolve.
    pub super_method_map: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct FunctionMetadata {
    pub name: String,
    /// Neo entrypoint name. This may be mangled to disambiguate overloaded
    /// Solidity functions because Neo ABI dispatches by name+arg count.
    pub neo_name: String,
    pub kind: FunctionKind,
    pub parameters: Vec<ParameterMetadata>,
    pub return_parameters: Vec<ParameterMetadata>,
    pub state_mutability: StateMutability,
    pub visibility: VisibilityKind,
    pub offset: u32,
    pub body: Option<Statement>,
    pub selector: [u8; 4],
    /// Whether this function is marked `virtual`.
    pub is_virtual: bool,
    /// Whether this function is marked `override`.
    pub is_override: bool,
    /// Natspec documentation for the function
    pub documentation: NatspecDoc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionKind {
    Constructor,
    Regular,
}

#[derive(Debug, Clone)]
pub struct ParameterMetadata {
    pub name: Option<String>,
    pub ty: String,
    pub neo_type: Option<NeoType>,
    pub storage: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EventMetadata {
    pub name: String,
    pub normalized_name: String,
    pub parameters: Vec<EventParameter>,
}

#[derive(Debug, Clone)]
pub struct EventParameter {
    pub name: Option<String>,
    pub ty: String,
    pub indexed: bool,
}

#[derive(Debug, Clone)]
pub struct StateVariableMetadata {
    pub name: Option<String>,
    pub ty: String,
    pub is_constant: bool,
    pub is_immutable: bool,
    pub visibility: Option<String>,
    pub neo_type: Option<NeoType>,
    pub has_initializer: bool,
    pub initializer: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct StructMetadata {
    pub name: String,
    pub fields: Vec<StructFieldMetadata>,
}

#[derive(Debug, Clone)]
pub struct StructFieldMetadata {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
pub struct EnumMetadata {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

impl StateMutability {
    pub fn is_safe(self) -> bool {
        matches!(self, StateMutability::Pure | StateMutability::View)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub code: Option<String>,
    pub suggestion: Option<String>,
}

impl Diagnostic {
    /// Create a warning diagnostic.
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Warning,
            message: message.into(),
            code: None,
            suggestion: None,
        }
    }

    /// Create an error diagnostic.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            message: message.into(),
            code: None,
            suggestion: None,
        }
    }

    /// Attach a diagnostic code (e.g. "W101", "E042").
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Attach an actionable fix suggestion.
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}
