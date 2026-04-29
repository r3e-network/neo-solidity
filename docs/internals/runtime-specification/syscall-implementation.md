---
title: "Runtime Specification: Syscall Implementation"
description: "Syscall Implementation from Runtime Specification."
---

# Syscall Implementation

[Back to Runtime Specification](/internals/runtime-specification)

Syscalls are the bridge between NeoVM bytecode and the Neo N3 platform services. They are invoked via the `SYSCALL` opcode followed by a 4-byte interop ID.

### Syscall ID Computation

Syscall IDs are the first 4 bytes of the SHA-256 hash of the syscall name string:

```
ID = SHA256("System.Storage.Get")[0..4]
```

This is computed at compile time and embedded in the bytecode.

### Storage Syscalls

| Syscall                             | Description                                           |
| ----------------------------------- | ----------------------------------------------------- |
| `System.Storage.GetContext`         | Get the current contract's read-write storage context |
| `System.Storage.GetReadOnlyContext` | Get a read-only storage context                       |
| `System.Storage.AsReadOnly`         | Convert a context to read-only                        |
| `System.Storage.Get`                | Read a value by key from storage                      |
| `System.Storage.Put`                | Write a key-value pair to storage                     |
| `System.Storage.Delete`             | Delete a key from storage                             |
| `System.Storage.Find`               | Find entries by key prefix, returns iterator          |
| `System.Storage.Local.Get`          | Read from local storage                               |
| `System.Storage.Local.Put`          | Write to local storage                                |
| `System.Storage.Local.Delete`       | Delete from local storage                             |
| `System.Storage.Local.Find`         | Find in local storage by prefix                       |

### Iterator Syscalls

| Syscall                   | Description                                     |
| ------------------------- | ----------------------------------------------- |
| `System.Iterator.Next`    | Advance iterator to next entry; returns boolean |
| `System.Iterator.Value`   | Get current iterator entry value                |
| `System.Iterator.Dispose` | Free the iterator handle and release resources  |

Iterator handles are created by `Storage.Find` and are validated by `ISTYPE` (type code `0x80`) while alive. Disposed handles fail `ISTYPE` checks.

::: info
`Iterator.Dispose` returns a boolean and invalidates the handle. After disposal, `ISTYPE` checks against type code `0x80` will return false.
:::

### Runtime Syscalls

| Syscall                                 | Description                                                     |
| --------------------------------------- | --------------------------------------------------------------- |
| `System.Runtime.GetTrigger`             | Get execution trigger type                                      |
| `System.Runtime.Platform`               | Get platform identifier string                                  |
| `System.Runtime.GetNetwork`             | Get network magic number                                        |
| `System.Runtime.GetAddressVersion`      | Get address version byte                                        |
| `System.Runtime.GetTime`                | Get current block timestamp                                     |
| `System.Runtime.GetScriptContainer`     | Get the transaction that triggered execution                    |
| `System.Runtime.GetExecutingScriptHash` | Get hash of currently executing contract                        |
| `System.Runtime.GetCallingScriptHash`   | Get hash of the calling contract                                |
| `System.Runtime.GetEntryScriptHash`     | Get hash of the entry-point contract                            |
| `System.Runtime.LoadScript`             | Load and execute a script                                       |
| `System.Runtime.CheckWitness`           | Verify that the specified account has witnessed the transaction |
| `System.Runtime.GetInvocationCounter`   | Get number of times current contract was called                 |
| `System.Runtime.GetRandom`              | Get a pseudo-random number                                      |
| `System.Runtime.Log`                    | Emit a log message                                              |
| `System.Runtime.Notify`                 | Emit a notification event (name + state array)                  |
| `System.Runtime.GetNotifications`       | Get notifications emitted so far                                |
| `System.Runtime.GasLeft`                | Get remaining gas                                               |
| `System.Runtime.BurnGas`                | Burn specified amount of gas                                    |
| `System.Runtime.CurrentSigners`         | Get current transaction signers                                 |
| `System.Runtime.Serialize`              | Serialize a stack item to byte array                            |
| `System.Runtime.Deserialize`            | Deserialize a byte array back to a stack item                   |

### Crypto Syscalls

| Syscall                       | Description                           |
| ----------------------------- | ------------------------------------- |
| `System.Crypto.CheckSig`      | Verify a single signature (secp256k1) |
| `System.Crypto.CheckMultisig` | Verify multiple signatures            |

Additional crypto operations are available through the CryptoLib native contract: SHA256, RIPEMD160, Keccak256, Murmur32, Hash160, Hash256.

### Contract Syscalls

| Syscall                                 | Description                                         |
| --------------------------------------- | --------------------------------------------------- |
| `System.Contract.Call`                  | Call another contract by hash                       |
| `System.Contract.GetCallFlags`          | Get current call flags                              |
| `System.Contract.CreateStandardAccount` | Create standard account script hash from public key |
| `System.Contract.CreateMultisigAccount` | Create multisig account script hash                 |
