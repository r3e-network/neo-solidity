//! Neo N3 opcode/syscall/native-contract registry

use once_cell::sync::Lazy;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Opcode metadata (lightweight – only what the runtime needs today).
#[derive(Debug, Clone, Copy)]
pub struct OpcodeSpec {
    pub code: u8,
    pub name: &'static str,
    pub gas: u64,
}

/// Syscall metadata.
#[derive(Debug, Clone, Copy)]
pub struct SyscallSpec {
    pub id: [u8; 4],
    pub name: &'static str,
}

/// Native contract metadata.
#[derive(Debug, Clone, Copy)]
pub struct NativeContractSpec {
    pub hash: [u8; 20],
    pub name: &'static str,
}

fn interop_id_bytes(name: &str) -> [u8; 4] {
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    let digest = hasher.finalize();
    [digest[0], digest[1], digest[2], digest[3]]
}

include!("spec/opcodes.rs");
include!("spec/syscalls.rs");
include!("spec/native_contracts.rs");
include!("spec/gas.rs");

#[cfg(test)]
mod tests;
