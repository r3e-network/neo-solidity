//! Solidity frontend integration using `solang-parser`.
//!
//! This module parses Solidity source code into a light-weight intermediate
//! representation that can be consumed by later compiler stages.

use solang_parser::{
    diagnostics::Diagnostic,
    parse,
    pt::{
        Base, Comment, ContractDefinition, ContractPart, ContractTy, EnumDefinition,
        EventDefinition, Expression, FunctionAttribute, FunctionDefinition, FunctionTy, Identifier,
        Loc, Mutability, ParameterList, SourceUnitPart, Statement, StorageLocation,
        StructDefinition, Using, UsingFunction, UsingList, VariableAttribute, VariableDefinition,
        Visibility,
    },
};
use std::collections::HashMap;
use thiserror::Error;

include!("frontend/frontend_errors.rs");
include!("frontend/frontend_ir.rs");
include!("frontend/frontend_diagnostics.rs");
include!("frontend/frontend_parse.rs");
include!("frontend/frontend_convert.rs");

#[cfg(test)]
mod tests;
