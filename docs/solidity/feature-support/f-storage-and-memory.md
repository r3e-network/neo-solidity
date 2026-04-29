---
title: "Solidity Feature Support: F. Storage and Memory"
description: "F. Storage and Memory from Solidity Feature Support."
---

# F. Storage and Memory

[Back to Solidity Feature Support](/solidity/feature-support)

| Feature                          | Status | Notes                                                                  |
| -------------------------------- | :----: | ---------------------------------------------------------------------- |
| State variables                  |   ✅   | Mapped to Neo Storage with prefix-based keys.                          |
| `constant`                       |   ✅   | Compile-time constants inlined.                                        |
| `immutable`                      |   ✅   | Tracked via `is_immutable` flag. Modification blocked at compile time. |
| `memory` keyword                 |   ✅   | Parsed. NeoVM is stack-based so memory is implicit.                    |
| `storage` keyword                |   ✅   | Storage references for mappings and state variables.                   |
| `calldata` keyword               |   ✅   | Parsed. Treated as `memory` — NeoVM has no calldata region.            |
| Nested mappings                  |   ✅   | `mapping(K1 => mapping(K2 => V))` with composite storage keys.         |
| Struct in storage                |   ✅   | Serialized/deserialized via `StdLib.serialize`/`StdLib.deserialize`.   |
| Array `.push()` / `.pop()`       |   ✅   | Storage array operations supported.                                    |
| Array `.length`                  |   ✅   | Both memory and storage arrays.                                        |
| `new bytes(n)` / `new string(n)` |   ✅   | Buffer allocation via `NEWBUFFER`.                                     |
| `new T[](n)`                     |   ✅   | Dynamic array allocation via `NEWARRAY`.                               |
| `new Contract(...)`              |   🚫   | Blocked: "use ContractManagement for contract deployment".             |

### Storage key derivation

State variables are stored in Neo Storage using deterministic key derivation. For simple state variables, the key is derived from the variable name. For mappings, the key is computed as:

```
SHA256(key_bytes || slot_hash)
```

Where `slot_hash` is `SHA256(variable_name)`. Nested mappings iterate this process for each key level. See [Types](/language-description/types) for the full storage lowering specification.

---
