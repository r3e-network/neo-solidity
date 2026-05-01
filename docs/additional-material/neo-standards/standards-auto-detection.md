---
title: "Neo Standards Auto-Detection"
description: "Standards Auto-Detection from Standards and Contracts."
---

# Neo Standards Auto-Detection

[Back to Standards and Contracts](/additional-material/neo-standards)

The `neo-solc` compiler analyzes your contract's public methods and events to automatically detect which NEP standards it implements. Detection results populate the manifest's `supportedstandards` array.

## NEP-17 Detection

All 5 methods must be present as public/external functions:

```
symbol, decimals, totalSupply, balanceOf, transfer
```

Additional rules:

- `ownerOf` must **not** be present (its presence signals NEP-11 instead)
- A `Transfer` event with 3 parameters is expected
- The `transfer` method should have 4 parameters `(from, to, amount, data)`
- **Near-miss warning**: if 3+ of 5 methods are present but not all, the compiler emits a warning listing the missing methods

## NEP-11 Detection

Core requirement:

- `balanceOf` **and** `ownerOf` must both be present

Plus at least one of:

- `transfer`
- `transferFrom`
- `tokensOf`

Additional checks:

- A `Transfer` event with 4 parameters is expected
- The `transfer` method should have 3 parameters `(to, tokenId, data)`
- **Near-miss warnings**: `ownerOf` without a transfer mechanism, or `ownerOf` + transfer without `balanceOf`

## NEP-24 Detection

Either of these methods triggers detection:

- `tokenUri`
- `royaltyInfo`

## Additional NEP Detection

The compiler also auto-detects these contract-lifecycle and callback standards:

- `NEP-22`: `update(nefFile, manifest, data)`
- `NEP-26`: `onNEP11Payment(from, amount, tokenId, data)`
- `NEP-27`: `onNEP17Payment(from, amount, data)`
- `NEP-29`: `_deploy(data, update)`
- `NEP-30`: `verify(...) -> bool`
- `NEP-31`: `destroy()`

::: tip Checking Detection Results
After compilation, inspect the manifest to verify detected standards:

```bash
neo-solc MyToken.sol -I devpack -O2 -o build/MyToken
cat build/MyToken/MyToken.manifest.json | jq '.supportedstandards'
# Expected: ["NEP-17"]
```

:::
