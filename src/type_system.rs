//! Basic Neo type representations derived from Solidity type strings.

use thiserror::Error;

include!("type_system/types.rs");
include!("type_system/parse.rs");

#[cfg(test)]
mod tests;
