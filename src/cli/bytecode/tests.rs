use super::*;
use neo_devpack_solidity::codegen::interop_id_bytes;
use neo_devpack_solidity::solidity::analyse_source;
use neo_devpack_solidity::{
    frontend::VisibilityKind,
    solidity::{FunctionKind, FunctionMetadata, StateMutability},
};

include!("tests/integration.rs");
include!("tests/helpers.rs");
