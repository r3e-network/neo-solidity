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

impl SemanticModel {
    /// Get all public functions
    pub fn public_functions(&self) -> Vec<&FunctionSymbol> {
        self.functions
            .iter()
            .filter(|f| {
                matches!(
                    f.visibility,
                    VisibilityKind::Public | VisibilityKind::External
                )
            })
            .collect()
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&FunctionSymbol> {
        self.functions.iter().find(|f| f.name == name)
    }

    /// Get all state variables
    pub fn get_state_variables(&self) -> &[StateVariableSymbol] {
        &self.state_variables
    }

    /// Check if this is a payable contract
    pub fn is_payable(&self) -> bool {
        self.functions
            .iter()
            .any(|f| f.state_mutability == StateMutability::Payable)
    }
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
        // Task #94 — tolerate parenthesised tuple return/parameter types whose
        // components are all individually supported. The frontend loses the
        // structured tuple shape (solang flattens the top-level tuple into
        // sibling ParameterIR entries), but nested tuples still arrive as a
        // raw `(T1, T2, ...)` string here. Downstream IR lowering treats the
        // slot as `NeoType::Any` and routes it through `abiEncode`, which
        // produces the EVM-canonical flat head layout the spec calls for.
        None if is_supported_tuple_type(&param.ty) => Ok(ParameterSymbol {
            name: param.name.clone(),
            ty: NeoType::Any,
            storage: param.storage.clone(),
        }),
        // Gap `nep11` — the devpack NeoVM iterator handle (`Syscalls.Iterator`).
        // The builtin helper libraries are never struct-merged into user
        // contracts, so the type never resolves to a `NeoType::Struct`; treat
        // the opaque handle as `Any` so NEP-11 `tokensOf`/`tokens` can declare
        // it (manifest returntype `InteropInterface`).
        None if is_devpack_iterator_type(&param.ty) => Ok(ParameterSymbol {
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

/// Gap `nep11` — true iff `ty` names the devpack NeoVM iterator handle type
/// (`Syscalls.Iterator` / `Storage.Iterator` / bare `Iterator`). Mirrors
/// `is_devpack_iterator_type` in `validate/contract/return_types.rs`.
fn is_devpack_iterator_type(ty: &str) -> bool {
    let lowered = ty.trim().to_ascii_lowercase();
    matches!(
        lowered.as_str(),
        "iterator" | "syscalls.iterator" | "storage.iterator"
    )
}

/// Task #94 — recognise `(T1, T2, ...)` tuple-shaped type strings with all
/// inner components individually supported by the scalar/array/bytes/mapping
/// rules. Recursive on nested parens.
fn is_supported_tuple_type(ty: &str) -> bool {
    let trimmed = ty.trim();
    let bytes = trimmed.as_bytes();
    if bytes.len() < 2 || bytes[0] != b'(' || bytes[bytes.len() - 1] != b')' {
        return false;
    }
    let mut depth: i32 = 0;
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 && i != bytes.len() - 1 {
                    return false;
                }
            }
            _ => {}
        }
    }
    if depth != 0 {
        return false;
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    let parts = split_tuple_components_for_params(inner);
    !parts.is_empty() && parts.iter().all(|p| is_supported_leaf_or_tuple(p))
}

fn split_tuple_components_for_params(body: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth: i32 = 0;
    let mut start = 0usize;
    let bytes = body.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            b',' if depth == 0 => {
                parts.push(body[start..i].trim().to_string());
                start = i + 1;
            }
            _ => {}
        }
    }
    if start < body.len() {
        let last = body[start..].trim();
        if !last.is_empty() {
            parts.push(last.to_string());
        }
    }
    parts
}

fn is_supported_leaf_or_tuple(ty: &str) -> bool {
    let trimmed = ty.trim();
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        return is_supported_tuple_type(trimmed);
    }
    let lowered = trimmed.to_ascii_lowercase();
    lowered.starts_with("uint")
        || lowered.starts_with("int")
        || lowered == "bool"
        || lowered == "string"
        || lowered == "address"
        || lowered == "bytes"
        || lowered == "bytearray"
        || lowered.starts_with("bytes")
        || lowered.ends_with("[]")
        || lowered.starts_with("mapping")
}
