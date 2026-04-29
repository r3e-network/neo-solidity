---
title: "Devpack Overview: Directory Layout"
description: "Directory Layout from Devpack Overview."
---

# Directory Layout

[Back to Devpack Overview](/additional-material/neo-devpack)

```
devpack/
├── contracts/               # Syscall/native-call wrappers and framework contracts
│   ├── FrameworkBase.sol    # Base framework with ownership, storage, gas management
│   ├── Framework.sol        # Extended framework (strict-manifest mode)
│   ├── Syscalls.sol         # Complete Neo N3 syscall surface (1,323 lines)
│   ├── NativeCalls.sol      # Native contract wrappers (1,068 lines)
│   ├── OracleService.sol    # Oracle convenience wrapper
│   └── NEP17Rescue.sol      # Emergency native-token recovery for NEP-17 contracts
├── libraries/               # High-level helper libraries
│   ├── Neo.sol              # Blockchain integration (542 lines)
│   ├── Storage.sol          # Advanced storage operations (827 lines)
│   └── Runtime.sol          # Runtime services and utilities (688 lines)
├── standards/               # NEP token standard implementations
│   ├── NEP17.sol            # Fungible token (NEP-17)
│   ├── NEP11.sol            # Non-fungible token (NEP-11, divisible + indivisible)
│   └── NEP24.sol            # Royalty standard (NEP-24)
├── examples/                # Production-oriented usage examples
│   ├── CompleteNEP17Token.sol
│   ├── CompleteNEP11NFT.sol
│   └── VaultPattern.sol
├── DEVPACK_GUIDE.md
├── README.md
└── standards/STANDARDS_MAPPING.md
```
