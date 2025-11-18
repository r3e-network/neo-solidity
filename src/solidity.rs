use crate::frontend::{
    parse_source, ContractIR, ContractKind, EventIR, FunctionIR, MutabilityKind, ParameterIR,
    StateVariableIR, StructIR, VisibilityKind,
};
use crate::type_system::{
    NeoType, StructFieldMetadata as NeoStructFieldMetadata, StructTypeMetadata,
};
use sha3::{Digest, Keccak256};
use solang_parser::pt::{CatchClause, Expression, FunctionTy, Statement};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolidityError {
    #[error("{0}")]
    Frontend(#[from] crate::frontend::FrontendError),
    #[error("no contract definitions found in source")]
    NoContracts,
}

#[derive(Debug, Clone)]
pub struct ContractMetadata {
    pub name: String,
    pub methods: Vec<FunctionMetadata>,
    pub events: Vec<EventMetadata>,
    pub uses_storage: bool,
    pub state_variables: Vec<StateVariableMetadata>,
    pub structs: Vec<StructMetadata>,
}

#[derive(Debug, Clone)]
pub struct FunctionMetadata {
    pub name: String,
    pub kind: FunctionKind,
    pub parameters: Vec<ParameterMetadata>,
    pub return_parameters: Vec<ParameterMetadata>,
    pub state_mutability: StateMutability,
    pub visibility: VisibilityKind,
    pub offset: u32,
    pub body: Option<Statement>,
    pub selector: [u8; 4],
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
}

pub fn analyse_source(source: &str) -> Result<ContractMetadata, SolidityError> {
    let mut contracts = analyse_all_sources(source)?;
    Ok(contracts.swap_remove(0))
}

pub fn analyse_all_sources(source: &str) -> Result<Vec<ContractMetadata>, SolidityError> {
    let mut primary = Vec::new();
    let mut fallback = Vec::new();
    let mut interface_events = Vec::new();

    for contract in parse_source(source)? {
        if matches!(
            contract.kind,
            ContractKind::Contract | ContractKind::AbstractContract
        ) {
            primary.push(contract);
        } else {
            if matches!(contract.kind, ContractKind::Interface) {
                interface_events.extend(contract.events.clone());
            }
            fallback.push(contract);
        }
    }

    let mut selected = if !primary.is_empty() {
        primary
    } else {
        fallback
    };

    if selected.is_empty() {
        return Ok(Vec::new());
    }

    let inherited_events: Vec<EventMetadata> =
        interface_events.into_iter().map(convert_event).collect();

    Ok(selected
        .drain(..)
        .map(|contract| convert_contract(contract, &inherited_events))
        .collect())
}

fn convert_contract(contract: ContractIR, inherited_events: &[EventMetadata]) -> ContractMetadata {
    let structs: Vec<StructMetadata> = contract.structs.into_iter().map(convert_struct).collect();

    let struct_type_info = structs_to_type_metadata(&structs);

    let methods = contract
        .functions
        .into_iter()
        .filter(|function| matches!(function.ty, FunctionTy::Function | FunctionTy::Constructor))
        .map(|function| convert_function(function, &struct_type_info))
        .collect();

    use std::collections::BTreeMap;
    let mut event_map: BTreeMap<String, EventMetadata> = BTreeMap::new();
    for event in inherited_events {
        event_map
            .entry(event.normalized_name.clone())
            .or_insert_with(|| event.clone());
    }
    for event in contract.events.into_iter().map(convert_event) {
        event_map.insert(event.normalized_name.clone(), event);
    }
    let events: Vec<EventMetadata> = event_map.into_values().collect();
    let state_variables: Vec<StateVariableMetadata> = contract
        .state_variables
        .into_iter()
        .map(|var| convert_state_variable(var, &struct_type_info))
        .collect();

    ContractMetadata {
        name: contract.name,
        methods,
        events,
        uses_storage: !state_variables.is_empty(),
        state_variables,
        structs,
    }
}

fn convert_function(function: FunctionIR, struct_types: &[StructTypeMetadata]) -> FunctionMetadata {
    let parameters: Vec<ParameterMetadata> = function
        .parameters
        .into_iter()
        .map(|param| convert_parameter(param, struct_types))
        .collect();

    let return_parameters = function
        .returns
        .into_iter()
        .map(|param| convert_parameter(param, struct_types))
        .collect();

    let param_signatures: Vec<String> = parameters
        .iter()
        .map(|param| canonical_param_type(&param.ty))
        .collect();

    let selector = compute_function_selector(&function.name, &param_signatures);

    FunctionMetadata {
        name: function.name,
        kind: match function.ty {
            FunctionTy::Constructor => FunctionKind::Constructor,
            _ => FunctionKind::Regular,
        },
        parameters,
        return_parameters,
        state_mutability: match function.mutability {
            MutabilityKind::Pure => StateMutability::Pure,
            MutabilityKind::View => StateMutability::View,
            MutabilityKind::Payable => StateMutability::Payable,
            MutabilityKind::NonPayable => StateMutability::NonPayable,
        },
        visibility: function.visibility,
        offset: 0,
        body: function.body,
        selector,
    }
}

fn convert_event(event: EventIR) -> EventMetadata {
    let normalized = normalize_event_name(&event.name);
    EventMetadata {
        name: event.name,
        normalized_name: normalized,
        parameters: event
            .parameters
            .into_iter()
            .map(|param| EventParameter {
                name: param.name,
                ty: param.ty,
                indexed: param.indexed,
            })
            .collect(),
    }
}

fn normalize_event_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

fn convert_state_variable(
    var: StateVariableIR,
    struct_types: &[StructTypeMetadata],
) -> StateVariableMetadata {
    let ty = var.ty;
    let neo_type = NeoType::from_solidity(&ty, struct_types).ok();
    StateVariableMetadata {
        name: var.name,
        ty,
        is_constant: var.is_constant,
        is_immutable: var.is_immutable,
        visibility: var.visibility,
        neo_type,
        has_initializer: var.has_initializer,
    }
}

fn convert_parameter(param: ParameterIR, struct_types: &[StructTypeMetadata]) -> ParameterMetadata {
    let ty = param.ty;
    let neo_type = NeoType::from_solidity(&ty, struct_types).ok();
    ParameterMetadata {
        name: param.name,
        ty,
        neo_type,
        storage: param.storage,
    }
}

fn convert_struct(ir: StructIR) -> StructMetadata {
    let fields = ir
        .fields
        .into_iter()
        .map(|field| StructFieldMetadata {
            name: field.name,
            ty: field.ty,
        })
        .collect();

    StructMetadata {
        name: ir.name,
        fields,
    }
}

fn structs_to_type_metadata(structs: &[StructMetadata]) -> Vec<StructTypeMetadata> {
    structs
        .iter()
        .map(|s| StructTypeMetadata {
            name: s.name.clone(),
            fields: s
                .fields
                .iter()
                .map(|f| NeoStructFieldMetadata {
                    name: f.name.clone(),
                    ty: f.ty.clone(),
                })
                .collect(),
        })
        .collect()
}

pub fn extract_return_expression(body: &Option<Statement>) -> Option<Expression> {
    let statement = body.as_ref()?;

    match statement {
        Statement::Block { statements, .. } => {
            if statements.len() == 1 {
                if let Statement::Return(_, expr) = &statements[0] {
                    expr.clone()
                } else {
                    None
                }
            } else {
                None
            }
        }
        Statement::Return(_, expr) => expr.clone(),
        _ => None,
    }
}

pub fn validate_contract(metadata: &ContractMetadata) -> Vec<Diagnostic> {
    use std::collections::HashSet;

    let mut diagnostics = Vec::new();
    let mut signatures = HashSet::new();
    let mut constructor_count = 0usize;

    for function in &metadata.methods {
        match function.kind {
            FunctionKind::Constructor => {
                constructor_count += 1;
                if !function.return_parameters.is_empty() {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        message: "constructor must not specify a return type".to_string(),
                    });
                }
            }
            FunctionKind::Regular => {
                let param_signature: Vec<String> = function
                    .parameters
                    .iter()
                    .map(|param| canonical_param_type(&param.ty))
                    .collect();
                let signature = format!("{}({})", function.name, param_signature.join(","));

                if !signatures.insert(signature.clone()) {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        message: format!("duplicate function signature '{}'", signature),
                    });
                }
            }
        }

        let mut params = HashSet::new();
        for param in &function.parameters {
            if let Some(name) = &param.name {
                if !params.insert(name.clone()) {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        message: format!(
                            "function '{}' has duplicate parameter name '{}'",
                            function.name, name
                        ),
                    });
                }
            }

            if param.neo_type.is_none() {
                diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    message: format!(
                        "function '{}' parameter '{}' uses unsupported type '{}'",
                        function.name,
                        param
                            .name
                            .clone()
                            .unwrap_or_else(|| "<unnamed>".to_string()),
                        param.ty
                    ),
                });
            }

            if let Some(storage) = &param.storage {
                if storage == "storage"
                    && matches!(
                        function.visibility,
                        VisibilityKind::External | VisibilityKind::Public
                    )
                {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        message: format!(
                            "public/external function '{}' parameter '{}' may not use 'storage' data location",
                            function.name,
                            param
                                .name
                                .clone()
                                .unwrap_or_else(|| "<unnamed>".to_string())
                        ),
                    });
                }
            }
        }

        if let Some(body) = &function.body {
            check_return_statements(
                body,
                function.return_parameters.len(),
                &function.name,
                &mut diagnostics,
            );
        } else if !function.return_parameters.is_empty()
            && !matches!(function.kind, FunctionKind::Constructor)
        {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                message: format!(
                    "function '{}' declares a return type but has no implementation",
                    function.name
                ),
            });
        }
    }

    if constructor_count > 1 {
        diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Error,
            message: format!(
                "multiple constructors defined ({} total)",
                constructor_count
            ),
        });
    }

    let mut state_names = HashSet::new();
    for state in &metadata.state_variables {
        match &state.name {
            Some(name) => {
                if !state_names.insert(name.clone()) {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        message: format!("duplicate state variable '{}'", name),
                    });
                }
            }
            None => diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Error,
                message: "state variable declared without a name".to_string(),
            }),
        }

        if state.neo_type.is_none() {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Error,
                message: format!(
                    "state variable '{}' has unsupported type '{}'",
                    state.name.as_deref().unwrap_or("<unnamed>"),
                    state.ty
                ),
            });
        }

        if state.is_constant && !state.has_initializer {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Error,
                message: format!(
                    "constant state variable '{}' must have an initializer",
                    state.name.as_deref().unwrap_or("<unnamed>")
                ),
            });
        }
    }

    for event in &metadata.events {
        if event.parameters.len() > 16 {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                message: format!(
                    "event '{}' has {} parameters which exceeds Neo ABI limits",
                    event.name,
                    event.parameters.len()
                ),
            });
        }
    }

    for function in &metadata.methods {
        if function.return_parameters.len() > 1 {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                message: format!(
                    "function '{}' returns multiple values which Neo ABI does not support",
                    function.name
                ),
            });
        }

        if let Some(ret_param) = function.return_parameters.first() {
            if ret_param.neo_type.is_none() {
                diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    message: format!(
                        "function '{}' return type '{}' is unsupported",
                        function.name, ret_param.ty
                    ),
                });
            }

            let lowered = ret_param.ty.to_ascii_lowercase();
            let supported = lowered.starts_with("uint")
                || lowered.starts_with("int")
                || lowered == "bool"
                || lowered == "string"
                || lowered == "address"
                || lowered == "bytes"
                || lowered == "bytearray"
                || lowered.starts_with("bytes");

            if !supported {
                diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Warning,
                    message: format!(
                        "function '{}' returns '{}', which may not map cleanly to Neo manifest types",
                        function.name, ret_param.ty
                    ),
                });
            }
        }

        for ret_param in &function.return_parameters {
            if let Some(storage) = &ret_param.storage {
                if storage == "storage" {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Warning,
                        message: format!(
                            "function '{}' return value '{}' uses 'storage' data location (treated as Any)",
                            function.name, ret_param.ty
                        ),
                    });
                }
            }
        }
    }

    diagnostics
}

fn check_return_statements(
    statement: &Statement,
    expected_count: usize,
    function_name: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match statement {
        Statement::Block { statements, .. } => {
            for stmt in statements {
                check_return_statements(stmt, expected_count, function_name, diagnostics);
            }
        }
        Statement::If(_, _, then_stmt, else_stmt) => {
            check_return_statements(then_stmt, expected_count, function_name, diagnostics);
            if let Some(else_stmt) = else_stmt {
                check_return_statements(else_stmt, expected_count, function_name, diagnostics);
            }
        }
        Statement::While(_, _, body) => {
            check_return_statements(body, expected_count, function_name, diagnostics);
        }
        Statement::DoWhile(_, body, _) => {
            check_return_statements(body, expected_count, function_name, diagnostics);
        }
        Statement::For(_, init, _, _, body) => {
            if let Some(init_stmt) = init {
                check_return_statements(init_stmt, expected_count, function_name, diagnostics);
            }
            if let Some(body_stmt) = body {
                check_return_statements(body_stmt, expected_count, function_name, diagnostics);
            }
        }
        Statement::Try(_, _, handler, catches) => {
            if let Some((_, handler_stmt)) = handler {
                check_return_statements(handler_stmt, expected_count, function_name, diagnostics);
            }
            for catch in catches {
                match catch {
                    CatchClause::Simple(_, _, stmt) => {
                        check_return_statements(stmt, expected_count, function_name, diagnostics)
                    }
                    CatchClause::Named(_, _, _, stmt) => {
                        check_return_statements(stmt, expected_count, function_name, diagnostics)
                    }
                }
            }
        }
        Statement::Return(_, expr) => match (expected_count, expr) {
            (0, Some(_)) => diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                message: format!(
                    "function '{}' returns a value but is declared without return type",
                    function_name
                ),
            }),
            (0, None) => {}
            (1, None) => diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                message: format!(
                    "function '{}' declares a return value but returns without one",
                    function_name
                ),
            }),
            (1, Some(_)) => {}
            (expected, Some(expr)) => {
                if let Expression::List(_, list) = expr {
                    let actual = list.len();
                    if actual != expected {
                        diagnostics.push(Diagnostic {
                            severity: DiagnosticSeverity::Warning,
                            message: format!(
                                "function '{}' expected {} return values but found {}",
                                function_name, expected, actual
                            ),
                        });
                    }
                } else {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Warning,
                        message: format!(
                            "function '{}' should return {} values but expression does not match tuple",
                            function_name, expected
                        ),
                    });
                }
            }
            (expected, None) => diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                message: format!(
                    "function '{}' declares {} return values but returns without one",
                    function_name, expected
                ),
            }),
        },
        _ => {}
    }
}

fn canonical_param_type(ty: &str) -> String {
    ty.split_whitespace().next().unwrap_or_default().to_string()
}

fn compute_function_selector(name: &str, param_types: &[String]) -> [u8; 4] {
    if name.eq_ignore_ascii_case("constructor") {
        return [0u8; 4];
    }

    let signature = if param_types.is_empty() {
        format!("{name}()")
    } else {
        format!("{name}({})", param_types.join(","))
    };

    let mut hasher = Keccak256::new();
    hasher.update(signature.as_bytes());
    let digest = hasher.finalize();
    let mut selector = [0u8; 4];
    selector.copy_from_slice(&digest[..4]);
    selector
}
