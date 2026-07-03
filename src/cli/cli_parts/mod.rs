//! CLI sub-modules for the Neo DevPack for Solidity compiler.
//!
//! This module groups the CLI implementation files that were previously
//! included via `#[path = "..."]` attributes in `cli/mod.rs`. Converting
//! to a standard `mod` declaration fixes IDE navigation, rust-analyzer
//! support, and refactoring tooling.

use crate::cli::{
    build_manifest, build_upgrade_report, bytecode, compile_contracts_with_options,
    compiler_version_string_4, contract_output_prefix, emit_error, emit_error_with_suggestion,
    emit_warning, emit_warning_with_suggestion, ensure_deploy_stub, ensure_output_dir,
    generate_contract_bytecode, load_manifest_permissions_override, optimize_ir,
    print_upgrade_reports, process_standard_json, process_standard_json_content,
    sanitize_contract_name, standard_json, write_assembly_file, write_json_file,
    write_manifest_file, write_nef_file, write_upgrade_reports, CompilationArtifacts, CompileError,
    CompileOptions, StandardJsonOptions, COMPILER_EMAIL, COMPILER_ID,
};
use crate::frontend::VisibilityKind;
use crate::ir;
use crate::neo::{build_nef_with_tokens, clamp_nef_source_with_flag, NEF_SOURCE_MAX_BYTES};
use crate::semantic_model::build_semantic_model;
use crate::solidity::{
    analyse_all_sources, validate_contract, ContractMetadata, DiagnosticSeverity, EventMetadata,
    FunctionKind, FunctionMetadata, NatspecDoc, ParameterMetadata, StateMutability,
};
use crate::type_system::NeoType;
use serde_json::{json, Value};
use sha3::{Digest, Keccak256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub(crate) mod cli_analyze;
pub(crate) mod cli_compile;
pub(crate) mod cli_defs;
pub(crate) mod cli_deploy;
pub(crate) mod cli_diagnostics;
pub(crate) mod cli_manifest;
pub(crate) mod cli_output;
pub(crate) mod cli_run;
