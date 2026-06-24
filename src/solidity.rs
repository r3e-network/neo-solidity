//! Solidity Metadata Extraction Module
//!
//! This module provides functionality for extracting metadata from Solidity source code,
//! including contract definitions, function signatures, state variables, events, and
//! Natspec documentation.
//!
//! # Key Components
//!
//! - [`ContractMetadata`] - Complete metadata for a Solidity contract
//! - [`FunctionMetadata`] - Function signature and body information
//! - [`NatspecDoc`] - Extracted Natspec documentation (@title, @notice, @dev, etc.)
//! - [`analyse_all_sources`] - Main entry point for multi-file analysis
//!
//! # Example
//!
//! ```text
//! use neo_devpack_solidity::solidity::{analyse_all_sources, ContractMetadata};
//!
//! let sources = vec![("Contract.sol".to_string(), source_code)];
//! let contracts = analyse_all_sources(&sources)?;
//! ```

use crate::frontend::{
    parse_source, ContractIR, ContractKind, EnumIR, ErrorIR, EventIR, FunctionIR, MutabilityKind,
    NatspecDocIR, ParameterIR, StateVariableIR, StructIR, VisibilityKind,
};
use crate::type_system::{
    EnumTypeMetadata, NeoType, StructFieldMetadata as NeoStructFieldMetadata, StructTypeMetadata,
};
use sha3::{Digest, Keccak256};
use solang_parser::pt::{
    Base, CatchClause, Expression, FunctionTy, Identifier, Loc, NamedArgument, Parameter, Statement,
};
use thiserror::Error;

mod solidity_analyse;
mod solidity_convert;
mod solidity_docs;
mod solidity_errors;
mod solidity_metadata;
mod solidity_validate;
mod upgrade;

pub(crate) use solidity_analyse::*;
pub(crate) use solidity_convert::*;
pub(crate) use solidity_docs::*;
pub(crate) use solidity_errors::*;
pub(crate) use solidity_metadata::*;
pub(crate) use solidity_validate::*;
pub(crate) use upgrade::*;

#[path = "solidity/analyse/inheritance.rs"]
mod inheritance;
pub(crate) use inheritance::*;
#[path = "solidity/analyse/modifiers.rs"]
mod modifiers;
pub(crate) use modifiers::*;
#[path = "solidity/analyse/sibling_merge_walkers.rs"]
mod sibling_merge_walkers;
pub(crate) use sibling_merge_walkers::*;
#[path = "solidity/analyse/merge_helpers.rs"]
mod merge_helpers;
pub(crate) use merge_helpers::*;
