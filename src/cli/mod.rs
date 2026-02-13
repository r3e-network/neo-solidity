//! Neo Solidity CLI Module
//!
//! Command-line interface for the Neo Solidity compiler. This module provides
//! the main entry point for compiling Solidity contracts to NeoVM bytecode.
//!
//! # Features
//!
//! - Single file and multi-file compilation
//! - Standard JSON input/output (Solidity compiler compatible)
//! - NEF (Neo Executable Format) generation
//! - Manifest generation for Neo N3 deployment
//! - Multiple optimization levels (0-3)
//! - Verbose output
//!
//! # Submodules
//!
//! - [`bytecode`] - NeoVM bytecode generation from IR
//! - [`standard_json`] - Standard JSON compilation interface
//!
//! # Usage
//!
//! ```bash
//! neo-solc input.sol -o build/
//! neo-solc --standard-json < input.json > output.json
//! neo-solc --standard-json --input input.json --output output.json
//! ```

use clap::{Arg, ArgAction, Command};
use neo_solidity::frontend::VisibilityKind;
use neo_solidity::ir;
use neo_solidity::neo::{build_nef_with_tokens, clamp_nef_source_with_flag, NEF_SOURCE_MAX_BYTES};
use neo_solidity::semantic_model::build_semantic_model;
use neo_solidity::solidity::{
    analyse_all_sources, validate_contract, ContractMetadata, DiagnosticSeverity, EventMetadata,
    FunctionKind, FunctionMetadata, NatspecDoc, ParameterMetadata, StateMutability,
};
use neo_solidity::type_system::NeoType;
use serde_json::{json, Value};
use sha3::{Digest, Keccak256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

mod bytecode;
#[cfg(test)]
pub(crate) use crate::codegen::interop_id_bytes;
pub(crate) use bytecode::generate_contract_bytecode;
mod ir_optimize;
pub(crate) use ir_optimize::optimize_ir;
mod standard_json;
pub(crate) use standard_json::*;

include!("cli_parts/cli_defs.rs");
include!("cli_parts/cli_run.rs");
include!("cli_parts/cli_compile.rs");
include!("cli_parts/cli_output.rs");
include!("cli_parts/cli_manifest.rs");
include!("cli_parts/cli_deploy.rs");
include!("cli_parts/cli_diagnostics.rs");

#[cfg(test)]
mod tests;
