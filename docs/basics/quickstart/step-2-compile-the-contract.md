---
title: "Quick Start: Step 2: Compile the Contract"
description: "Step 2: Compile the Contract from Quick Start."
---

# Step 2: Compile the Contract

[Back to Quick Start](/basics/quickstart)

```bash
./target/release/neo-solc MyStorage.sol -I devpack -O2 -o build/MyStorage
```

Flags explained:

| Flag                 | Purpose                                                                                    |
| -------------------- | ------------------------------------------------------------------------------------------ |
| `MyStorage.sol`      | Input Solidity source file                                                                 |
| `-I devpack`         | Add the `devpack/` directory as an import search path (provides Neo N3 library interfaces) |
| `-O2`                | Optimization level 2 (standard optimization: inlining, peephole, DCE, constant folding)    |
| `-o build/MyStorage` | Output prefix -- generates `build/MyStorage.nef` and `build/MyStorage.manifest.json`       |

Expected output (with `-v` for verbose):

```bash
./target/release/neo-solc MyStorage.sol -I devpack -O2 -v -o build/MyStorage
```

```
[info] Parsing MyStorage.sol
[info] Extracting metadata for contract MyStorage
[info] Building semantic model
[info] Generating IR (optimization level 2)
[info] Emitting NeoVM bytecode
[info] Writing build/MyStorage.nef (XXX bytes)
[info] Writing build/MyStorage.manifest.json
```
