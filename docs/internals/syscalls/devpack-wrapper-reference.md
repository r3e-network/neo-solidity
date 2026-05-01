---
title: "Syscalls: Devpack Wrapper Reference"
description: "Devpack Wrapper Reference from Syscalls."
---

# Devpack Wrapper Reference

[Back to Syscalls](/internals/syscalls)

The devpack provides syscall access at two abstraction levels.

## Low-Level: `Syscalls.sol`

Located at `devpack/contracts/Syscalls.sol`. Provides typed wrappers for the Neo N3 syscall surface, plus convenience functions for native contract calls.

Key features:

- Typed wrappers for the registered syscall surface exposed as `internal` Solidity functions
- Data structures: `Block`, `Transaction`, `Signer`, `StorageContext`, `Iterator`, `Notification`
- Constants: trigger types, witness scopes, witness conditions, named curve hashes
- Native contract script hash constants for the runtime-supported native contracts: NEO, GAS, ContractManagement, Policy, Oracle, RoleManagement, Notary, Treasury, Ledger, CryptoLib, and StdLib

## High-Level: Ergonomic Libraries

| Library   | File                            | Built On                                   |
| --------- | ------------------------------- | ------------------------------------------ |
| `Storage` | `devpack/libraries/Storage.sol` | `System.Storage.*` syscalls                |
| `Runtime` | `devpack/libraries/Runtime.sol` | `System.Runtime.*` syscalls                |
| `Neo`     | `devpack/libraries/Neo.sol`     | Multiple syscall categories + native calls |

The `Storage` library adds batch operations, typed accessors (`putUint256`, `getAddress`, `putBool`), iterator helpers (`findKeys`, `findValues`, `count`), key derivation for mappings and arrays, and storage patterns like expiration and checksummed writes.

```solidity
// Low-level: direct syscall
Syscalls.StorageContext memory ctx = Syscalls.getStorageContext();
Syscalls.storagePut(ctx, "key", abi.encode(42));

// High-level: ergonomic wrapper
Storage.putUint256("key", 42);
uint256 value = Storage.getUint256("key");
```
