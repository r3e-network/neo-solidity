//! Canonical intermediate representation for Neo DevPack for Solidity.

use crate::solidity::{
    ContractMetadata, EnumMetadata, EventMetadata, FunctionKind as MetadataFunctionKind,
    FunctionMetadata, ParameterMetadata, SelectorRegistry, StateVariableMetadata,
};
use crate::storage_key::compute_state_slot;
use hex::decode as hex_decode;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive, Zero};
use sha3::{Digest, Keccak256};
use solang_parser::pt::{
    Expression, HexLiteral as PtHexLiteral, Identifier, Statement,
    StorageLocation as PtStorageLocation, StringLiteral as PtStringLiteral, Type as PtType,
};
use std::collections::{HashMap, HashSet};

include!("ir/ir_types.rs");
include!("ir/ir_build.rs");
include!("ir/ir_context.rs");
include!("ir/ir_statements.rs");
include!("ir/ir_expressions.rs");
include!("ir/ir_deploy.rs");

#[cfg(test)]
mod tests;
