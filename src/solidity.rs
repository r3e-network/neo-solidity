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
//! ```ignore
//! use neo_solidity::solidity::{analyse_all_sources, ContractMetadata};
//!
//! let sources = vec![("Contract.sol".to_string(), source_code)];
//! let contracts = analyse_all_sources(&sources)?;
//! ```

use crate::frontend::{
    parse_source, ContractIR, ContractKind, EnumIR, EventIR, FunctionIR, MutabilityKind,
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

include!("solidity/solidity_errors.rs");
include!("solidity/solidity_docs.rs");
include!("solidity/solidity_metadata.rs");
include!("solidity/analyse/inheritance.rs");
include!("solidity/analyse/modifiers.rs");
include!("solidity/solidity_analyse.rs");
include!("solidity/solidity_convert.rs");
include!("solidity/solidity_validate.rs");
