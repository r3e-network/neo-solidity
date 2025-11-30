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

macro_rules! op {
    ($code:expr, $name:expr, $gas:expr) => {
        OpcodeSpec {
            code: $code,
            name: $name,
            gas: $gas,
        }
    };
}

// Canonical Neo N3 opcode table (from provided OpCode enum).
pub static OPCODES: Lazy<HashMap<u8, OpcodeSpec>> = Lazy::new(|| {
    let mut m = HashMap::new();

    let entries = [
        // Constants
        op!(0x00, "PUSHINT8", 1),
        op!(0x01, "PUSHINT16", 1),
        op!(0x02, "PUSHINT32", 1),
        op!(0x03, "PUSHINT64", 1),
        op!(0x04, "PUSHINT128", 1),
        op!(0x05, "PUSHINT256", 1),
        op!(0x08, "PUSHT", 1),
        op!(0x09, "PUSHF", 1),
        op!(0x0A, "PUSHA", 1),
        op!(0x0B, "PUSHNULL", 1),
        op!(0x0C, "PUSHDATA1", 2),
        op!(0x0D, "PUSHDATA2", 2),
        op!(0x0E, "PUSHDATA4", 2),
        op!(0x0F, "PUSHM1", 1),
        op!(0x10, "PUSH0", 1),
        op!(0x11, "PUSH1", 1),
        op!(0x12, "PUSH2", 1),
        op!(0x13, "PUSH3", 1),
        op!(0x14, "PUSH4", 1),
        op!(0x15, "PUSH5", 1),
        op!(0x16, "PUSH6", 1),
        op!(0x17, "PUSH7", 1),
        op!(0x18, "PUSH8", 1),
        op!(0x19, "PUSH9", 1),
        op!(0x1A, "PUSH10", 1),
        op!(0x1B, "PUSH11", 1),
        op!(0x1C, "PUSH12", 1),
        op!(0x1D, "PUSH13", 1),
        op!(0x1E, "PUSH14", 1),
        op!(0x1F, "PUSH15", 1),
        op!(0x20, "PUSH16", 1),
        // Flow control
        op!(0x21, "NOP", 1),
        op!(0x22, "JMP", 2),
        op!(0x23, "JMP_L", 2),
        op!(0x24, "JMPIF", 2),
        op!(0x25, "JMPIF_L", 2),
        op!(0x26, "JMPIFNOT", 2),
        op!(0x27, "JMPIFNOT_L", 2),
        op!(0x28, "JMPEQ", 2),
        op!(0x29, "JMPEQ_L", 2),
        op!(0x2A, "JMPNE", 2),
        op!(0x2B, "JMPNE_L", 2),
        op!(0x2C, "JMPGT", 2),
        op!(0x2D, "JMPGT_L", 2),
        op!(0x2E, "JMPGE", 2),
        op!(0x2F, "JMPGE_L", 2),
        op!(0x30, "JMPLT", 2),
        op!(0x31, "JMPLT_L", 2),
        op!(0x32, "JMPLE", 2),
        op!(0x33, "JMPLE_L", 2),
        op!(0x34, "CALL", 512),
        op!(0x35, "CALL_L", 512),
        op!(0x36, "CALLA", 512),
        op!(0x37, "CALLT", 512),
        op!(0x38, "ABORT", 1),
        op!(0x39, "ASSERT", 1),
        op!(0x3A, "THROW", 1),
        op!(0x3B, "TRY", 1),
        op!(0x3C, "TRY_L", 1),
        op!(0x3D, "ENDTRY", 1),
        op!(0x3E, "ENDTRY_L", 1),
        op!(0x3F, "ENDFINALLY", 1),
        op!(0x40, "RET", 0),
        op!(0x41, "SYSCALL", 10),
        // Stack ops
        op!(0x43, "DEPTH", 2),
        op!(0x45, "DROP", 2),
        op!(0x46, "NIP", 2),
        op!(0x48, "XDROP", 2),
        op!(0x49, "CLEAR", 2),
        op!(0x4A, "DUP", 2),
        op!(0x4B, "OVER", 2),
        op!(0x4D, "PICK", 2),
        op!(0x4E, "TUCK", 2),
        op!(0x50, "SWAP", 2),
        op!(0x51, "ROT", 2),
        op!(0x52, "ROLL", 2),
        op!(0x53, "REVERSE3", 2),
        op!(0x54, "REVERSE4", 2),
        op!(0x55, "REVERSEN", 2),
        // Slot ops
        op!(0x56, "INITSSLOT", 2),
        op!(0x57, "INITSLOT", 3),
        op!(0x58, "LDSFLD0", 2),
        op!(0x59, "LDSFLD1", 2),
        op!(0x5A, "LDSFLD2", 2),
        op!(0x5B, "LDSFLD3", 2),
        op!(0x5C, "LDSFLD4", 2),
        op!(0x5D, "LDSFLD5", 2),
        op!(0x5E, "LDSFLD6", 2),
        op!(0x5F, "LDSFLD", 2),
        op!(0x60, "STSFLD0", 2),
        op!(0x61, "STSFLD1", 2),
        op!(0x62, "STSFLD2", 2),
        op!(0x63, "STSFLD3", 2),
        op!(0x64, "STSFLD4", 2),
        op!(0x65, "STSFLD5", 2),
        op!(0x66, "STSFLD6", 2),
        op!(0x67, "STSFLD", 2),
        op!(0x68, "LDLOC0", 2),
        op!(0x69, "LDLOC1", 2),
        op!(0x6A, "LDLOC2", 2),
        op!(0x6B, "LDLOC3", 2),
        op!(0x6C, "LDLOC4", 2),
        op!(0x6D, "LDLOC5", 2),
        op!(0x6E, "LDLOC6", 2),
        op!(0x6F, "LDLOC", 2),
        op!(0x70, "STLOC0", 2),
        op!(0x71, "STLOC1", 2),
        op!(0x72, "STLOC2", 2),
        op!(0x73, "STLOC3", 2),
        op!(0x74, "STLOC4", 2),
        op!(0x75, "STLOC5", 2),
        op!(0x76, "STLOC6", 2),
        op!(0x77, "STLOC", 2),
        op!(0x78, "LDARG0", 2),
        op!(0x79, "LDARG1", 2),
        op!(0x7A, "LDARG2", 2),
        op!(0x7B, "LDARG3", 2),
        op!(0x7C, "LDARG4", 2),
        op!(0x7D, "LDARG5", 2),
        op!(0x7E, "LDARG6", 2),
        op!(0x7F, "LDARG", 2),
        op!(0x80, "STARG0", 2),
        op!(0x81, "STARG1", 2),
        op!(0x82, "STARG2", 2),
        op!(0x83, "STARG3", 2),
        op!(0x84, "STARG4", 2),
        op!(0x85, "STARG5", 2),
        op!(0x86, "STARG6", 2),
        op!(0x87, "STARG", 2),
        // Splice / buffer
        op!(0x88, "NEWBUFFER", 4),
        op!(0x89, "MEMCPY", 4),
        op!(0x8B, "CAT", 4),
        op!(0x8C, "SUBSTR", 4),
        op!(0x8D, "LEFT", 4),
        op!(0x8E, "RIGHT", 4),
        // Bitwise / logic
        op!(0x90, "INVERT", 3),
        op!(0x91, "AND", 3),
        op!(0x92, "OR", 3),
        op!(0x93, "XOR", 3),
        op!(0x97, "EQUAL", 3),
        op!(0x98, "NOTEQUAL", 3),
        // Numeric
        op!(0x99, "SIGN", 3),
        op!(0x9A, "ABS", 3),
        op!(0x9B, "NEGATE", 3),
        op!(0x9C, "INC", 3),
        op!(0x9D, "DEC", 3),
        op!(0x9E, "ADD", 3),
        op!(0x9F, "SUB", 3),
        op!(0xA0, "MUL", 5),
        op!(0xA1, "DIV", 5),
        op!(0xA2, "MOD", 5),
        op!(0xA3, "POW", 8),
        op!(0xA4, "SQRT", 6),
        op!(0xA5, "MODMUL", 8),
        op!(0xA6, "MODPOW", 8),
        op!(0xA8, "SHL", 3),
        op!(0xA9, "SHR", 3),
        op!(0xAA, "NOT", 2),
        op!(0xAB, "BOOLAND", 2),
        op!(0xAC, "BOOLOR", 2),
        op!(0xB1, "NZ", 2),
        op!(0xB3, "NUMEQUAL", 3),
        op!(0xB4, "NUMNOTEQUAL", 3),
        op!(0xB5, "LT", 3),
        op!(0xB6, "LE", 3),
        op!(0xB7, "GT", 3),
        op!(0xB8, "GE", 3),
        op!(0xB9, "MIN", 3),
        op!(0xBA, "MAX", 3),
        op!(0xBB, "WITHIN", 3),
        // Compound / collections
        op!(0xBE, "PACKMAP", 4),
        op!(0xBF, "PACKSTRUCT", 4),
        op!(0xC0, "PACK", 4),
        op!(0xC1, "UNPACK", 4),
        op!(0xC2, "NEWARRAY0", 4),
        op!(0xC3, "NEWARRAY", 4),
        op!(0xC4, "NEWARRAY_T", 4),
        op!(0xC5, "NEWSTRUCT0", 4),
        op!(0xC6, "NEWSTRUCT", 4),
        op!(0xC8, "NEWMAP", 4),
        op!(0xCA, "SIZE", 4),
        op!(0xCB, "HASKEY", 4),
        op!(0xCC, "KEYS", 4),
        op!(0xCD, "VALUES", 4),
        op!(0xCE, "PICKITEM", 4),
        op!(0xCF, "APPEND", 4),
        op!(0xD0, "SETITEM", 4),
        op!(0xD1, "REVERSEITEMS", 4),
        op!(0xD2, "REMOVE", 4),
        op!(0xD3, "CLEARITEMS", 4),
        op!(0xD4, "POPITEM", 4),
        // Types
        op!(0xD8, "ISNULL", 2),
        op!(0xD9, "ISTYPE", 2),
        op!(0xDB, "CONVERT", 2),
        // Extensions
        op!(0xE0, "ABORTMSG", 1),
        op!(0xE1, "ASSERTMSG", 1),
    ];

    for entry in entries {
        m.insert(entry.code, entry);
    }
    m
});

pub fn opcode_name(code: u8) -> Option<&'static str> {
    OPCODES.get(&code).map(|op| op.name)
}

pub fn opcode_gas(code: u8) -> Option<u64> {
    OPCODES.get(&code).map(|op| op.gas)
}

// Syscall registry – the names are authoritative; IDs are derived via SHA-256.
pub static SYSCALLS: Lazy<HashMap<[u8; 4], SyscallSpec>> = Lazy::new(|| {
    let names = [
        // Storage
        "System.Storage.GetContext",
        "System.Storage.GetReadOnlyContext",
        "System.Storage.Get",
        "System.Storage.Put",
        "System.Storage.Delete",
        "System.Storage.Find",
        "System.Iterator.Next",
        "System.Iterator.Value",
        "System.Iterator.Dispose",
        // Runtime
        "System.Runtime.Platform",
        "System.Runtime.GetNetwork",
        "System.Runtime.GetTime",
        "System.Runtime.GetBlockTime",
        "System.Runtime.GetGasLeft",
        "System.Runtime.GetInvocationCounter",
        "System.Runtime.CallingScriptHash",
        "System.Runtime.EntryScriptHash",
        "System.Runtime.ExecutingScriptHash",
        "System.Runtime.CheckWitness",
        "System.Runtime.Log",
        "System.Runtime.Notify",
        "System.Runtime.Serialize",
        "System.Runtime.Deserialize",
        "System.Runtime.GetRandom",
        // Crypto
        "System.Crypto.SHA256",
        "System.Crypto.RIPEMD160",
        "System.Crypto.Keccak256",
        "System.Crypto.Murmur32",
        "System.Crypto.Hash160",
        "System.Crypto.Hash256",
        "System.Crypto.CheckSig",
        "System.Crypto.CheckMultisig",
        // Blockchain
        "System.Blockchain.GetHeight",
        "System.Blockchain.GetBlock",
        "System.Blockchain.GetTransaction",
        "System.Blockchain.GetTransactionHeight",
        "System.Blockchain.GetTransactionFromBlock",
        "System.Blockchain.GetContract",
        "System.Blockchain.GetCommittee",
        "System.Blockchain.GetValidators",
        "System.Blockchain.GetBlockHash",
        "System.Blockchain.GetRandom",
        // Contract / native calls
        "System.Contract.Call",
        "System.Contract.CallEx",
        "System.Contract.GetCallFlags",
        "System.Contract.CreateStandardAccount",
        "System.Contract.CreateMultisigAccount",
        "System.ContractManagement.Deploy",
        "System.ContractManagement.Update",
        // Oracle & Policy
        "System.Oracle.Request",
        "System.Policy.GetFeePerByte",
        "System.Policy.GetExecFeeFactor",
        "System.Policy.GetStoragePrice",
    ];

    let mut m = HashMap::new();
    for name in names {
        m.insert(
            interop_id_bytes(name),
            SyscallSpec {
                id: interop_id_bytes(name),
                name,
            },
        );
    }
    m
});

/// Known native contract hashes (big-endian as emitted in devpack contracts).
pub static NATIVE_CONTRACTS: Lazy<HashMap<[u8; 20], NativeContractSpec>> = Lazy::new(|| {
    let entries = [
        (
            *b"\xef@\x73\xa0\xf2\xb3\x05\xa3\x8e\xc4\x05\x0e\x4d\x3d\x28\xbc\x40\xea\x63\xf5",
            "NEO",
        ),
        (
            *b"\xd2\xa4\xcf\xf3\x19\x13\x01\x61\x55\xe3\x8e\x47\x4a\x2c\x06\xd0\x8b\xe2\x76\xcf",
            "GAS",
        ),
        (
            *b"\xff\xfd\xc9\x37\x64\xdb\xad\xdd\x97\xc4\x8f\x25\x2a\x53\xea\x46\x43\xfa\xa3\xfd",
            "ContractManagement",
        ),
        (
            *b"\xcc\x5e\x4e\xdd\x9f\x5f\x8d\xba\x8b\xb6\x57\x34\x54\x1d\xf7\xa1\xc0\x81\xc6\x7b",
            "Policy",
        ),
        (
            *b"\xfe\x92\x4b\x7c\xfe\x89\xdd\xd2\x71\xab\xaf\x72\x10\xa8\x0a\x7e\x11\x17\x87\x58",
            "Oracle",
        ),
        (
            *b"\x49\xcf\x4e\x53\x78\xff\xcd\x4d\xec\x03\x4f\xd9\x8a\x17\x4c\x54\x91\xe3\x95\xe2",
            "RoleManagement",
        ),
        (
            *b"\x74\xfd\x00\x9f\x26\x05\xdd\xfc\xe1\x20\xf8\xfd\x76\x22\x45\x08\xa8\x16\xd8\x52",
            "Ledger",
        ),
        (
            *b"\x45\xd4\xe3\x07\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
            "ContractManagementHashStub",
        ),
    ];

    let mut m = HashMap::new();
    for (hash, name) in entries {
        m.insert(hash, NativeContractSpec { hash, name });
    }
    m
});

pub fn syscall_name(id: &[u8; 4]) -> Option<&'static str> {
    SYSCALLS.get(id).map(|s| s.name)
}

pub fn native_contract_name(hash: &[u8; 20]) -> Option<&'static str> {
    NATIVE_CONTRACTS.get(hash).map(|s| s.name)
}

/// Minimal syscall gas hints (best-effort; many syscalls are cheap stubs here).
pub fn syscall_gas_table() -> std::collections::HashMap<[u8; 4], u64> {
    use std::collections::HashMap;
    let mut m = HashMap::new();
    for (_, spec) in SYSCALLS.iter() {
        let gas = match spec.name {
            // Storage
            "System.Storage.GetContext" | "System.Storage.GetReadOnlyContext" => 1,
            "System.Storage.Get" => 100,
            "System.Storage.Put" => 1_000,
            "System.Storage.Delete" => 100,
            "System.Storage.Find" => 100,
            // Iterators
            "System.Iterator.Next" | "System.Iterator.Value" | "System.Iterator.Dispose" => 1,
            // Runtime
            "System.Runtime.Platform"
            | "System.Runtime.GetNetwork"
            | "System.Runtime.GetTime"
            | "System.Runtime.GetBlockTime"
            | "System.Runtime.GetGasLeft"
            | "System.Runtime.GetInvocationCounter"
            | "System.Runtime.CallingScriptHash"
            | "System.Runtime.EntryScriptHash"
            | "System.Runtime.ExecutingScriptHash" => 1,
            "System.Runtime.CheckWitness" => 200,
            "System.Runtime.Log" | "System.Runtime.Notify" => 1,
            "System.Runtime.Serialize" | "System.Runtime.Deserialize" => 100,
            "System.Runtime.GetRandom" | "System.Blockchain.GetRandom" => 50,
            // Crypto
            "System.Crypto.SHA256"
            | "System.Crypto.RIPEMD160"
            | "System.Crypto.Keccak256"
            | "System.Crypto.Murmur32" => 10,
            "System.Crypto.Hash160" | "System.Crypto.Hash256" => 20,
            "System.Crypto.CheckSig" | "System.Crypto.CheckMultisig" => 1_000,
            // Blockchain
            "System.Blockchain.GetHeight"
            | "System.Blockchain.GetBlock"
            | "System.Blockchain.GetTransaction"
            | "System.Blockchain.GetTransactionHeight"
            | "System.Blockchain.GetTransactionFromBlock"
            | "System.Blockchain.GetContract"
            | "System.Blockchain.GetCommittee"
            | "System.Blockchain.GetValidators"
            | "System.Blockchain.GetBlockHash" => 1,
            // Contract / native calls
            "System.Contract.Call" | "System.Contract.CallEx" | "System.Contract.GetCallFlags" => {
                10
            }
            "System.Contract.CreateStandardAccount" | "System.Contract.CreateMultisigAccount" => 10,
            "System.ContractManagement.Deploy"
            | "System.ContractManagement.Update"
            | "System.ContractManagement.GetContract" => 10,
            // Oracle & Policy
            "System.Oracle.Request" => 100_000,
            "System.Policy.GetFeePerByte"
            | "System.Policy.GetExecFeeFactor"
            | "System.Policy.GetStoragePrice" => 1,
            _ => 1,
        };
        m.insert(spec.id, gas);
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_table_includes_core_surface() {
        assert!(OPCODES.contains_key(&0x34), "CALL should be registered");
        assert!(OPCODES.contains_key(&0xC3), "NEWARRAY should be registered");
        assert!(
            OPCODES.contains_key(&0x0C),
            "PUSHDATA1 should be registered"
        );
        assert!(OPCODES.contains_key(&0xE0), "ABORTMSG should be registered");
        assert!(OPCODES.len() >= 120, "expected a wide opcode surface");
    }

    #[test]
    fn syscall_table_hashes_are_stable() {
        let id = interop_id_bytes("System.Runtime.Notify");
        assert!(SYSCALLS.contains_key(&id));
        assert_eq!(SYSCALLS.get(&id).unwrap().name, "System.Runtime.Notify");
    }

    #[test]
    fn native_contracts_present() {
        assert!(native_contract_name(
            b"\xd2\xa4\xcf\xf3\x19\x13\x01\x61\x55\xe3\x8e\x47\x4a\x2c\x06\xd0\x8b\xe2\x76\xcf"
        )
        .is_some());
    }
}
