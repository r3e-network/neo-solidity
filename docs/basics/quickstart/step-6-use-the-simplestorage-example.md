---
title: "Quick Start: Step 6: Use the SimpleStorage Example"
description: "Step 6: Use the SimpleStorage Example from Quick Start."
---

# Step 6: Use the SimpleStorage Example

[Back to Quick Start](/basics/quickstart)

The repository ships a more complete example at `examples/SimpleStorage.sol` with key-value storage, ownership, reentrancy guards, and balance tracking.

Compile it:

```bash
./target/release/neo-solc examples/SimpleStorage.sol -I devpack -O2 -o build/SimpleStorage
```

Inspect the manifest:

```bash
# List all methods
jq '[.abi.methods[].name]' build/SimpleStorage.manifest.json
```

Expected methods:

```json
[
    "setNumber",
    "getNumber",
    "setString",
    "getString",
    "setKeyValue",
    "getKeyValue",
    "setKeyString",
    "getKeyString",
    "increment",
    "decrement",
    "add",
    "getOwner",
    "transferOwnership",
    "reset",
    "hasKey",
    "balanceOf",
    "setBalance"
]
```
