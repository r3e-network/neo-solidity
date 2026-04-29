---
title: "Syscalls: Syscall Categories"
description: "Syscall Categories from Syscalls."
---

# Syscall Categories

[Back to Syscalls](/internals/syscalls)

Neo N3 defines 38 syscalls across 5 categories. The compiler and devpack cover all of them.

| Category | Count | Prefix              | Purpose                             |
| -------- | ----: | ------------------- | ----------------------------------- |
| Storage  |    11 | `System.Storage.*`  | Persistent key-value state          |
| Runtime  |    19 | `System.Runtime.*`  | Execution context and notifications |
| Contract |     4 | `System.Contract.*` | Cross-contract calls and accounts   |
| Crypto   |     2 | `System.Crypto.*`   | Signature verification              |
| Iterator |     2 | `System.Iterator.*` | Traversal of storage search results |

::: info
Most cryptographic operations (SHA256, RIPEMD160, keccak256, ECDSA verification, BLS12-381) are exposed through the `CryptoLib` native contract, not through syscalls. The two `System.Crypto.*` syscalls handle only signature verification against the current script container.
:::
