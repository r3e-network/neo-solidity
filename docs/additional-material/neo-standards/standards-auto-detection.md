---
title: "Neo Standards Auto-Detection"
description: "Standards Auto-Detection from Standards and Contracts."
---

# Neo Standards Auto-Detection

[Back to Standards and Contracts](/additional-material/neo-standards)

The `neo-solc` compiler analyzes your contract's public methods and events to automatically detect which NEP standards it implements. Detection results populate the manifest's `supportedstandards` array.

::: warning Detection is not full compliance
Auto-detection is intentionally permissive. It identifies likely standard
surfaces and emits diagnostics for near misses, wrong transfer arity, or missing
events. A production contract should still be checked against the canonical NEP
ABI, event shape, authorization rules, and receiver-callback behavior.
:::

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

For strict NEP-17 compliance, the `transfer` method returns `bool`, the `data`
parameter maps to Neo ABI `Any`, and token contracts call
`onNEP17Payment(from, amount, data)` after a successful transfer to a deployed
contract.

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

For strict NEP-11 compliance, also check the common methods (`symbol`,
`decimals`, `totalSupply`, `balanceOf`, `tokensOf`, `transfer`), the
appropriate indivisible or divisible ownership methods, and receiver callback
behavior. `properties(tokenId)` is optional in the NEP-11 specification but is
included in the devpack's full NFT interface for metadata-rich tokens.

## NEP-24 Detection

Either of these methods triggers detection:

- `tokenURI` / `tokenUri` (method matching is case-insensitive)
- `royaltyInfo`

For strict NEP-24 compliance, `royaltyInfo(tokenId, royaltyToken, salePrice)`
returns an array of `[royaltyRecipient, royaltyAmount]` pairs. The
`royaltyAmount` values are final amounts for the provided `salePrice`; basis
points are only an internal configuration pattern used by many implementations.

## Additional NEP Detection

The compiler also auto-detects these contract-lifecycle and callback standards:

- `NEP-22`: `update(nefFile, manifest, data)`
- `NEP-26`: `onNEP11Payment(from, amount, tokenId, data)`
- `NEP-27`: `onNEP17Payment(from, amount, data)`
- `NEP-29`: `_deploy(data, update)`
- `NEP-30`: `verify(...) -> bool`
- `NEP-31`: `destroy()`

## Explicit Manifest Declarations

Contracts can also declare standards via
`@custom:neo.manifest.supportedstandards`. Explicit declarations are validated
more strictly for NEP-17 and NEP-11: missing required methods or the wrong
`Transfer` event arity are treated as manifest errors instead of advisory
diagnostics.

::: tip Checking Detection Results
After compilation, inspect the manifest to verify detected standards:

```bash
neo-solc MyToken.sol -I devpack -O2 -o build/MyToken
cat build/MyToken/MyToken.manifest.json | jq '.supportedstandards'
# Expected: ["NEP-17"]
```

:::
