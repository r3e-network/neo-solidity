//! Solidity frontend integration using `solang-parser`.
//!
//! This module parses Solidity source code into a light-weight intermediate
//! representation that can be consumed by later compiler stages.

use solang_parser::{
    diagnostics::Diagnostic,
    parse,
    pt::{
        Comment, ContractDefinition, ContractPart, ContractTy, EventDefinition, FunctionAttribute,
        FunctionDefinition, FunctionTy, Identifier, Loc, Mutability, ParameterList, SourceUnitPart,
        Statement, StorageLocation, StructDefinition, VariableAttribute, VariableDefinition,
        Visibility,
    },
};
use std::collections::HashMap;
use thiserror::Error;

/// Errors emitted by the frontend while parsing Solidity code.
#[derive(Debug, Error)]
pub enum FrontendError {
    /// Parsing failed; the contained message aggregates all diagnostics.
    #[error("Solidity parsing failed:\n{0}")]
    Parse(String),
}

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
    pub functions: Vec<FunctionIR>,
    pub events: Vec<EventIR>,
    pub state_variables: Vec<StateVariableIR>,
    pub structs: Vec<StructIR>,
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
    pub body: Option<Statement>,
    /// Natspec documentation for this function
    pub doc: NatspecDocIR,
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
    let (source_unit, comments) = parse(source, 0)
        .map_err(|diags| FrontendError::Parse(format_diagnostics(source, &diags)))?;

    // Build a map of end positions to preceding doc comments
    let comment_map = build_comment_map(&comments, source);

    let mut contracts = Vec::new();

    for part in source_unit.0.into_iter() {
        if let SourceUnitPart::ContractDefinition(contract) = part {
            contracts.push(convert_contract(*contract, &comment_map));
        }
    }

    Ok(contracts)
}

/// Build a map from source positions to their preceding Natspec comments.
fn build_comment_map(comments: &[Comment], _source: &str) -> HashMap<usize, NatspecDocIR> {
    let mut map = HashMap::new();
    let mut last_doc_comment: Option<(usize, String)> = None;

    for comment in comments {
        match comment {
            Comment::DocLine(loc, text) | Comment::DocBlock(loc, text) => {
                if let Loc::File(_, _, end) = loc {
                    // Accumulate doc comments - update end position to latest
                    let clean_text = clean_doc_comment(text);
                    if let Some((ref mut end_pos, ref mut existing)) = last_doc_comment {
                        *end_pos = *end; // Update to latest end position
                        existing.push('\n');
                        existing.push_str(&clean_text);
                    } else {
                        last_doc_comment = Some((*end, clean_text));
                    }
                }
            }
            Comment::Line(_loc, _) | Comment::Block(_loc, _) => {
                // Regular comments break doc comment sequences
                if let Some((end_pos, doc_text)) = last_doc_comment.take() {
                    map.insert(end_pos, parse_natspec(&doc_text));
                }
            }
        }
    }

    // Handle trailing doc comment
    if let Some((end_pos, doc_text)) = last_doc_comment {
        map.insert(end_pos, parse_natspec(&doc_text));
    }

    map
}

/// Remove comment delimiters and leading asterisks/slashes
fn clean_doc_comment(text: &str) -> String {
    text.lines()
        .map(|line| {
            let trimmed = line.trim();
            // Remove /// prefix from line doc comments
            if let Some(rest) = trimmed.strip_prefix("///") {
                rest.trim().to_string()
            // Remove /** and */ from block doc comments
            } else if let Some(rest) = trimmed.strip_prefix("/**") {
                rest.trim_end_matches("*/").trim().to_string()
            } else if let Some(rest) = trimmed.strip_suffix("*/") {
                rest.trim().to_string()
            // Remove leading * from block comment lines
            } else if let Some(rest) = trimmed.strip_prefix('*') {
                rest.trim().to_string()
            } else {
                trimmed.to_string()
            }
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Parse Natspec tags from a documentation comment
fn parse_natspec(text: &str) -> NatspecDocIR {
    let mut doc = NatspecDocIR::default();
    let mut current_tag: Option<&str> = None;
    let mut current_content = String::new();

    for line in text.lines() {
        let trimmed = line.trim();

        // Check for tag at start of line
        if trimmed.starts_with('@') {
            // Save previous tag content
            if let Some(tag) = current_tag {
                save_tag_content(&mut doc, tag, &current_content);
            }

            // Parse new tag
            let parts: Vec<&str> = trimmed.splitn(2, char::is_whitespace).collect();
            current_tag = Some(parts[0]);
            current_content = parts.get(1).map(|s| s.trim().to_string()).unwrap_or_default();
        } else if current_tag.is_some() {
            // Continue previous tag content
            if !current_content.is_empty() {
                current_content.push(' ');
            }
            current_content.push_str(trimmed);
        } else {
            // No tag yet - treat as @notice
            if doc.notice.is_none() && !trimmed.is_empty() {
                doc.notice = Some(trimmed.to_string());
            } else if let Some(ref mut notice) = doc.notice {
                notice.push(' ');
                notice.push_str(trimmed);
            }
        }
    }

    // Save final tag
    if let Some(tag) = current_tag {
        save_tag_content(&mut doc, tag, &current_content);
    }

    doc
}

fn save_tag_content(doc: &mut NatspecDocIR, tag: &str, content: &str) {
    let content = content.trim().to_string();
    if content.is_empty() {
        return;
    }

    match tag {
        "@title" => doc.title = Some(content),
        "@author" => doc.author = Some(content),
        "@notice" => doc.notice = Some(content),
        "@dev" => doc.dev = Some(content),
        "@param" => {
            // Format: @param name description
            let parts: Vec<&str> = content.splitn(2, char::is_whitespace).collect();
            if parts.len() >= 2 {
                doc.params.push((parts[0].to_string(), parts[1].trim().to_string()));
            } else if !parts.is_empty() {
                doc.params.push((parts[0].to_string(), String::new()));
            }
        }
        "@return" => doc.returns.push(content),
        tag if tag.starts_with("@custom:") => {
            let custom_tag = tag.strip_prefix("@custom:").unwrap_or("");
            doc.custom.push((custom_tag.to_string(), content));
        }
        _ => {} // Ignore unknown tags
    }
}

/// Find the doc comment that precedes a given source location
fn find_preceding_doc(loc: &Loc, comment_map: &HashMap<usize, NatspecDocIR>) -> NatspecDocIR {
    if let Loc::File(_, start, _) = loc {
        // Look for a doc comment that ends near this start position
        // Allow some whitespace between comment end and definition start
        for offset in 0..100 {
            if let Some(pos) = start.checked_sub(offset) {
                if let Some(doc) = comment_map.get(&pos) {
                    return doc.clone();
                }
            } else {
                break;
            }
        }
    }
    NatspecDocIR::default()
}

fn convert_contract(
    contract: ContractDefinition,
    comment_map: &HashMap<usize, NatspecDocIR>,
) -> ContractIR {
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

    // Extract contract-level documentation
    let doc = find_preceding_doc(&contract.loc, comment_map);

    let mut functions = Vec::new();
    let mut events = Vec::new();
    let mut state_variables = Vec::new();
    let mut structs = Vec::new();

    for part in contract.parts.into_iter() {
        match part {
            ContractPart::FunctionDefinition(def) => {
                functions.push(convert_function(*def, comment_map))
            }
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
        doc,
    }
}

fn convert_function(
    function: FunctionDefinition,
    comment_map: &HashMap<usize, NatspecDocIR>,
) -> FunctionIR {
    let name = function_name(&function);
    let mutability = extract_mutability(&function);
    let visibility = extract_visibility(&function);
    let doc = find_preceding_doc(&function.loc, comment_map);

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
        doc,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_natspec_extracts_title() {
        let text = "@title My Token\n@author Alice";
        let doc = parse_natspec(text);
        assert_eq!(doc.title, Some("My Token".to_string()));
        assert_eq!(doc.author, Some("Alice".to_string()));
    }

    #[test]
    fn parse_natspec_extracts_notice_and_dev() {
        let text = "@notice This is a token\n@dev Internal implementation";
        let doc = parse_natspec(text);
        assert_eq!(doc.notice, Some("This is a token".to_string()));
        assert_eq!(doc.dev, Some("Internal implementation".to_string()));
    }

    #[test]
    fn parse_natspec_extracts_params() {
        let text = "@param to The recipient address\n@param amount The amount to send";
        let doc = parse_natspec(text);
        assert_eq!(doc.params.len(), 2);
        assert_eq!(doc.params[0], ("to".to_string(), "The recipient address".to_string()));
        assert_eq!(doc.params[1], ("amount".to_string(), "The amount to send".to_string()));
    }

    #[test]
    fn parse_natspec_extracts_return() {
        let text = "@return The balance amount\n@return success Whether it succeeded";
        let doc = parse_natspec(text);
        assert_eq!(doc.returns.len(), 2);
        assert_eq!(doc.returns[0], "The balance amount");
        assert_eq!(doc.returns[1], "success Whether it succeeded");
    }

    #[test]
    fn parse_natspec_extracts_custom_tags() {
        let text = "@custom:security-contact security@example.com\n@custom:version 1.0.0";
        let doc = parse_natspec(text);
        assert_eq!(doc.custom.len(), 2);
        assert_eq!(doc.custom[0], ("security-contact".to_string(), "security@example.com".to_string()));
        assert_eq!(doc.custom[1], ("version".to_string(), "1.0.0".to_string()));
    }

    #[test]
    fn clean_doc_comment_removes_prefixes() {
        assert_eq!(clean_doc_comment("/// Hello"), "Hello");
        assert_eq!(clean_doc_comment("/** Hello */"), "Hello");
        assert_eq!(clean_doc_comment("   * Hello"), "Hello");
        assert_eq!(clean_doc_comment("Hello"), "Hello");
    }

    #[test]
    fn parse_source_captures_contract_doc() {
        let source = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @title Test Contract
/// @author Test Author
contract Test {
    function foo() public pure returns (uint256) {
        return 42;
    }
}
"#;
        let contracts = parse_source(source).expect("should parse");
        assert_eq!(contracts.len(), 1);
        assert_eq!(contracts[0].doc.title, Some("Test Contract".to_string()));
        assert_eq!(contracts[0].doc.author, Some("Test Author".to_string()));
    }

    #[test]
    fn parse_source_captures_function_doc() {
        let source = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Test {
    /// @notice Get the value
    /// @param x The input value
    /// @return The output value
    function getValue(uint256 x) public pure returns (uint256) {
        return x * 2;
    }
}
"#;
        let contracts = parse_source(source).expect("should parse");
        assert_eq!(contracts.len(), 1);
        let func = &contracts[0].functions[0];
        assert_eq!(func.doc.notice, Some("Get the value".to_string()));
        assert_eq!(func.doc.params.len(), 1);
        assert_eq!(func.doc.params[0].0, "x");
        assert_eq!(func.doc.returns.len(), 1);
    }
}
