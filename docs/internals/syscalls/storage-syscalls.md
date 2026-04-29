---
title: "Syscalls: Storage Syscalls"
description: "Storage Syscalls from Syscalls."
---

# Storage Syscalls

[Back to Syscalls](/internals/syscalls)

Storage syscalls provide persistent key-value state for contracts. Each contract has its own isolated storage context.

| Syscall Name                        | Gas Cost | Description                       | Devpack Wrapper                        |
| ----------------------------------- | -------: | --------------------------------- | -------------------------------------- |
| `System.Storage.GetContext`         |        1 | Get current storage context       | `Syscalls.getStorageContext()`         |
| `System.Storage.GetReadOnlyContext` |        1 | Get read-only context             | `Syscalls.getReadOnlyStorageContext()` |
| `System.Storage.AsReadOnly`         |        1 | Convert context to read-only      | `Syscalls.storageAsReadOnly(ctx)`      |
| `System.Storage.Get`                |      100 | Read value by key                 | `Syscalls.storageGet(ctx, key)`        |
| `System.Storage.Put`                |    1,000 | Write key-value pair              | `Syscalls.storagePut(ctx, key, val)`   |
| `System.Storage.Delete`             |      100 | Delete key                        | `Syscalls.storageDelete(ctx, key)`     |
| `System.Storage.Find`               |      100 | Find by prefix (returns iterator) | `Syscalls.storageFind(ctx, prefix)`    |
| `System.Storage.Local.Get`          |      100 | Read from local context           | `Syscalls.storageGetLocal(key)`        |
| `System.Storage.Local.Put`          |    1,000 | Write to local context            | `Syscalls.storagePutLocal(key, val)`   |
| `System.Storage.Local.Delete`       |      100 | Delete from local context         | `Syscalls.storageDeleteLocal(key)`     |
| `System.Storage.Local.Find`         |      100 | Find in local context             | `Syscalls.storageFindLocal(prefix)`    |

Local storage (`System.Storage.Local.*`) is contract-private and cannot be read by other contracts even through `System.Storage.GetReadOnlyContext`. Use it for internal bookkeeping that should never be externally visible.

```solidity
import "devpack/contracts/Syscalls.sol";

contract TokenVault {
    // The compiler lowers state variables to Storage.Get/Put automatically.
    // For manual control, use the Syscalls library directly:

    function manualStorageExample(bytes memory key, bytes memory value) internal {
        // Get the current contract's storage context
        Syscalls.StorageContext memory ctx = Syscalls.getStorageContext();

        // Write a key-value pair (costs 1,000 GAS units)
        Syscalls.storagePut(ctx, key, value);

        // Read it back (costs 100 GAS units)
        bytes memory stored = Syscalls.storageGet(ctx, key);

        // Delete when no longer needed (costs 100 GAS units)
        Syscalls.storageDelete(ctx, key);
    }
}
```

::: tip Storage Cost Awareness
`System.Storage.Put` costs 1,000 GAS units — 10x more expensive than `Get` or `Delete` (100 each). Minimize writes by batching updates and avoiding redundant puts. The `Storage.sol` library provides `batchPut()` for multi-key writes, but each individual put still incurs the full syscall cost.
:::
