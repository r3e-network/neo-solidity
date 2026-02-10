//! Semantic model extracted from Solidity metadata.

use crate::frontend::VisibilityKind;
use crate::solidity::{
    ContractMetadata, Diagnostic, DiagnosticSeverity, FunctionKind, FunctionMetadata,
    ParameterMetadata, StateMutability, StateVariableMetadata,
};
use crate::type_system::NeoType;

#[derive(Debug, Clone)]
pub struct SemanticModel {
    pub functions: Vec<FunctionSymbol>,
    pub state_variables: Vec<StateVariableSymbol>,
}

#[derive(Debug, Clone)]
pub struct FunctionSymbol {
    pub name: String,
    pub kind: FunctionKind,
    pub parameters: Vec<ParameterSymbol>,
    pub returns: Vec<ParameterSymbol>,
    pub state_mutability: StateMutability,
    pub visibility: VisibilityKind,
}

#[derive(Debug, Clone)]
pub struct ParameterSymbol {
    pub name: Option<String>,
    pub ty: NeoType,
    pub storage: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StateVariableSymbol {
    pub name: Option<String>,
    pub ty: NeoType,
    pub is_constant: bool,
    pub is_immutable: bool,
    pub visibility: Option<String>,
}

pub fn build_semantic_model(metadata: &ContractMetadata) -> Result<SemanticModel, Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();

    let mut functions = Vec::new();
    for function in &metadata.methods {
        match convert_function(function) {
            Ok(symbol) => functions.push(symbol),
            Err(mut diags) => diagnostics.append(&mut diags),
        }
    }

    let mut state_variables = Vec::new();
    for state in &metadata.state_variables {
        match convert_state_variable(state) {
            Ok(symbol) => state_variables.push(symbol),
            Err(mut diags) => diagnostics.append(&mut diags),
        }
    }

    let has_error = diagnostics
        .iter()
        .any(|diag| matches!(diag.severity, DiagnosticSeverity::Error));

    if has_error {
        Err(diagnostics)
    } else {
        Ok(SemanticModel {
            functions,
            state_variables,
        })
    }
}

fn convert_function(function: &FunctionMetadata) -> Result<FunctionSymbol, Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let allow_unsupported_internal_types = !matches!(
        function.visibility,
        VisibilityKind::Public | VisibilityKind::External
    );

    let mut parameters = Vec::new();
    for param in &function.parameters {
        match convert_parameter(
            param,
            FunctionSide::Parameter,
            &function.name,
            allow_unsupported_internal_types,
        ) {
            Ok(symbol) => parameters.push(symbol),
            Err(diag) => diagnostics.push(diag),
        }
    }

    let mut returns = Vec::new();
    for param in &function.return_parameters {
        match convert_parameter(
            param,
            FunctionSide::Return,
            &function.name,
            allow_unsupported_internal_types,
        ) {
            Ok(symbol) => returns.push(symbol),
            Err(diag) => diagnostics.push(diag),
        }
    }

    if diagnostics
        .iter()
        .any(|diag| matches!(diag.severity, DiagnosticSeverity::Error))
    {
        Err(diagnostics)
    } else {
        Ok(FunctionSymbol {
            name: function.name.clone(),
            kind: function.kind,
            parameters,
            returns,
            state_mutability: function.state_mutability,
            visibility: function.visibility,
        })
    }
}

fn convert_state_variable(
    state: &StateVariableMetadata,
) -> Result<StateVariableSymbol, Vec<Diagnostic>> {
    match &state.neo_type {
        Some(neo_type) => Ok(StateVariableSymbol {
            name: state.name.clone(),
            ty: neo_type.clone(),
            is_constant: state.is_constant,
            is_immutable: state.is_immutable,
            visibility: state.visibility.clone(),
        }),
        None => Err(vec![Diagnostic::error(format!(
            "state variable '{}' has unsupported type '{}'",
            state.name.as_deref().unwrap_or("<unnamed>"),
            state.ty
        ))]),
    }
}

enum FunctionSide {
    Parameter,
    Return,
}

fn convert_parameter(
    param: &ParameterMetadata,
    side: FunctionSide,
    function_name: &str,
    allow_unsupported_internal_types: bool,
) -> Result<ParameterSymbol, Diagnostic> {
    match &param.neo_type {
        Some(neo_type) => Ok(ParameterSymbol {
            name: param.name.clone(),
            ty: neo_type.clone(),
            storage: param.storage.clone(),
        }),
        None if allow_unsupported_internal_types => Ok(ParameterSymbol {
            name: param.name.clone(),
            ty: NeoType::Any,
            storage: param.storage.clone(),
        }),
        None => Err(Diagnostic::error(match side {
            FunctionSide::Parameter => format!(
                "function '{}' parameter '{}' uses unsupported type '{}'",
                function_name,
                param
                    .name
                    .clone()
                    .unwrap_or_else(|| "<unnamed>".to_string()),
                param.ty
            ),
            FunctionSide::Return => format!(
                "function '{}' return type '{}' is unsupported",
                function_name, param.ty
            ),
        })),
    }
}
