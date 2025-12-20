//! Neo N3 Utilities Module
//!
//! Provides Neo N3 blockchain-specific utilities for NEF (Neo Executable Format)
//! generation and method tokens.
//!
//! # Key Components
//!
//! - [`MethodToken`] - Cross-contract call optimization tokens
//! - [`build_nef_with_tokens`] - NEF file generation with method tokens
//! - [`clamp_nef_source_with_flag`] - Source code embedding in NEF
//!
//! # NEF Format
//!
//! The Neo Executable Format (NEF) is the standard container for Neo N3 smart
//! contracts, containing bytecode, metadata, and optional method tokens.

use sha2::{Digest, Sha256};
use std::borrow::Cow;

include!("neo/constants.rs");
include!("neo/method_token.rs");
include!("neo/build.rs");
include!("neo/encoding.rs");
include!("neo/source.rs");
include!("neo/contract_hash.rs");

#[cfg(test)]
mod tests;
