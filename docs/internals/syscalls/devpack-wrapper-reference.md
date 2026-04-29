---
title: "Syscalls: Devpack Wrapper Reference"
description: "Devpack Wrapper Reference from Syscalls."
---

# Devpack Wrapper Reference

[Back to Syscalls](/internals/syscalls)

The devpack provides syscall access at two abstraction levels.

### Low-Level: `Syscalls.sol`

Located at `devpack/contracts/Syscalls.sol`. Provides 1:1 typed wrappers for every Neo N3 syscall, plus convenience functions for native contract calls (CryptoLib, StdLib, Ledger, Policy, Oracle, RoleManagement).

Key features:

- All 38 syscalls exposed as `internal` Solidity functions
- Data structures: `Block`, `Transaction`, `Signer`, `StorageContext`, `Iterator`, `Notification`
- Constants: trigger types, witness scopes, witness conditions, named curve hashes
- Native contract script hash constants for all 7 core native contracts

### High-Level: Ergonomic Libraries

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
