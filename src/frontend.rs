//! Solidity frontend integration using `solang-parser`.
//!
//! This module parses Solidity source code into a light-weight intermediate
//! representation that can be consumed by later compiler stages.

use solang_parser::{
    diagnostics::Diagnostic,
    parse,
    pt::{
        ContractDefinition, ContractPart, ContractTy, EventDefinition, FunctionAttribute,
        FunctionDefinition, FunctionTy, Identifier, Mutability, ParameterList, SourceUnitPart,
        Statement, StorageLocation, StructDefinition, VariableAttribute, VariableDefinition,
        Visibility,
    },
};
use thiserror::Error;

/// Errors emitted by the frontend while parsing Solidity code.
#[derive(Debug, Error)]
pub enum FrontendError {
    /// Parsing failed; the contained message aggregates all diagnostics.
    #[error("Solidity parsing failed:\n{0}")]
    Parse(String),
}

/// Representation of a Solidity contract.
#[derive(Debug, Clone)]
pub struct ContractIR {
    pub name: String,
    pub kind: ContractKind,
    pub functions: Vec<FunctionIR>,
    pub events: Vec<EventIR>,
    pub state_variables: Vec<StateVariableIR>,
    pub structs: Vec<StructIR>,
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
    pub body: Option<Statement>,
}

/// Simplified function mutability classification.
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

/// Parse Solidity source into [`ContractIR`] values.
pub fn parse_source(source: &str) -> Result<Vec<ContractIR>, FrontendError> {
    let (source_unit, _) = parse(source, 0)
        .map_err(|diags| FrontendError::Parse(format_diagnostics(source, &diags)))?;

    let mut contracts = Vec::new();

    for part in source_unit.0.into_iter() {
        if let SourceUnitPart::ContractDefinition(contract) = part {
            contracts.push(convert_contract(*contract));
        }
    }

    Ok(contracts)
}

fn convert_contract(contract: ContractDefinition) -> ContractIR {
    let name = contract
        .name
        .as_ref()
        .map(|id| id.name.clone())
        .unwrap_or_else(|| "Contract".to_string());

    let kind = match contract.ty {
        ContractTy::Abstract(_) => ContractKind::AbstractContract,
        ContractTy::Contract(_) => ContractKind::Contract,
        ContractTy::Interface(_) => ContractKind::Interface,
        ContractTy::Library(_) => ContractKind::Library,
    };

    let mut functions = Vec::new();
    let mut events = Vec::new();
    let mut state_variables = Vec::new();
    let mut structs = Vec::new();

    for part in contract.parts.into_iter() {
        match part {
            ContractPart::FunctionDefinition(def) => functions.push(convert_function(*def)),
            ContractPart::EventDefinition(def) => events.push(convert_event(*def)),
            ContractPart::VariableDefinition(def) => {
                state_variables.push(convert_state_variable(*def))
            }
            ContractPart::StructDefinition(def) => structs.push(convert_struct(*def)),
            _ => {}
        }
    }

    ContractIR {
        name,
        kind,
        functions,
        events,
        state_variables,
        structs,
    }
}

fn convert_function(function: FunctionDefinition) -> FunctionIR {
    let name = function_name(&function);
    let mutability = extract_mutability(&function);
    let visibility = extract_visibility(&function);

    let parameters = convert_parameters(&function.params);
    let returns = convert_parameters(&function.returns);
    FunctionIR {
        name,
        ty: function.ty,
        parameters,
        returns,
        mutability,
        visibility,
        body: function.body,
    }
}

fn function_name(function: &FunctionDefinition) -> String {
    match (&function.name, function.ty) {
        (Some(Identifier { name, .. }), _) => name.clone(),
        (None, FunctionTy::Constructor) => "constructor".to_string(),
        (None, FunctionTy::Fallback) => "fallback".to_string(),
        (None, FunctionTy::Receive) => "receive".to_string(),
        (None, FunctionTy::Modifier) => "modifier".to_string(),
        _ => "function".to_string(),
    }
}

fn extract_mutability(function: &FunctionDefinition) -> MutabilityKind {
    for attribute in &function.attributes {
        if let FunctionAttribute::Mutability(m) = attribute {
            return match m {
                Mutability::Pure(_) => MutabilityKind::Pure,
                Mutability::View(_) | Mutability::Constant(_) => MutabilityKind::View,
                Mutability::Payable(_) => MutabilityKind::Payable,
            };
        }
    }

    MutabilityKind::NonPayable
}

fn extract_visibility(function: &FunctionDefinition) -> VisibilityKind {
    for attribute in &function.attributes {
        if let FunctionAttribute::Visibility(visibility) = attribute {
            return match visibility {
                Visibility::External(_) => VisibilityKind::External,
                Visibility::Public(_) => VisibilityKind::Public,
                Visibility::Internal(_) => VisibilityKind::Internal,
                Visibility::Private(_) => VisibilityKind::Private,
            };
        }
    }

    VisibilityKind::Internal
}

fn convert_parameters(params: &ParameterList) -> Vec<ParameterIR> {
    params
        .iter()
        .filter_map(|(_, param)| param.as_ref())
        .map(|param| ParameterIR {
            name: param.name.as_ref().map(|id| id.name.clone()),
            ty: format!("{}", param.ty),
            storage: param.storage.as_ref().map(storage_to_string),
        })
        .collect()
}

fn storage_to_string(storage: &StorageLocation) -> String {
    match storage {
        StorageLocation::Memory(_) => "memory",
        StorageLocation::Storage(_) => "storage",
        StorageLocation::Calldata(_) => "calldata",
    }
    .to_string()
}

fn convert_event(event: EventDefinition) -> EventIR {
    let name = event
        .name
        .as_ref()
        .map(|id| id.name.clone())
        .unwrap_or_else(|| "event".to_string());

    let parameters = event
        .fields
        .into_iter()
        .map(|param| EventParameterIR {
            name: param.name.map(|id| id.name),
            ty: format!("{}", param.ty),
            indexed: param.indexed,
        })
        .collect();

    EventIR { name, parameters }
}

fn convert_state_variable(def: VariableDefinition) -> StateVariableIR {
    let name = def.name.map(|id| id.name);
    let ty = format!("{}", def.ty);

    let mut visibility = None;
    let mut is_constant = false;
    let mut is_immutable = false;

    for attr in def.attrs {
        match attr {
            VariableAttribute::Visibility(vis) => {
                visibility = Some(
                    match vis {
                        Visibility::External(_) => "external",
                        Visibility::Public(_) => "public",
                        Visibility::Internal(_) => "internal",
                        Visibility::Private(_) => "private",
                    }
                    .to_string(),
                );
            }
            VariableAttribute::Constant(_) => {
                is_constant = true;
            }
            VariableAttribute::Immutable(_) => {
                is_immutable = true;
            }
            _ => {}
        }
    }

    StateVariableIR {
        name,
        ty,
        is_constant,
        is_immutable,
        visibility,
        has_initializer: def.initializer.is_some(),
    }
}

fn convert_struct(def: StructDefinition) -> StructIR {
    let name = def
        .name
        .as_ref()
        .map(|id| id.name.clone())
        .unwrap_or_else(|| "Struct".to_string());

    let fields = def
        .fields
        .into_iter()
        .filter_map(|field| {
            let field_name = field.name.map(|id| id.name)?;
            Some(StructFieldIR {
                name: field_name,
                ty: format!("{}", field.ty),
            })
        })
        .collect();

    StructIR { name, fields }
}

fn format_diagnostics(source: &str, diagnostics: &[Diagnostic]) -> String {
    diagnostics
        .iter()
        .map(|diag| {
            if let solang_parser::pt::Loc::File(_, start, _) = diag.loc {
                let (line, column) = offset_to_line_column(source, start);
                format!("{}:{}: {}", line, column, diag.message)
            } else {
                diag.message.clone()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn offset_to_line_column(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut column = 1usize;
    let mut current = 0usize;

    for ch in source.chars() {
        if current >= offset {
            break;
        }

        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }

        current += ch.len_utf8();
    }

    (line, column)
}
