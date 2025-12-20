use super::*;
use neo_solidity::frontend::VisibilityKind;
use neo_solidity::solidity::{
    analyse_source, EventMetadata, EventParameter, NatspecDoc, ParameterMetadata, StateMutability,
    StateVariableMetadata,
};
use neo_solidity::type_system::NeoType;
use sha3::{Digest, Keccak256};

include!("metadata/identifiers.rs");
include!("metadata/standards.rs");
include!("metadata/deploy.rs");
include!("metadata/output_and_storage.rs");
