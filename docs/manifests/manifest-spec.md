# Manifest Spec and Policy

Neo N3 contracts deploy with a manifest JSON alongside NEF bytecode.

## Required top-level fields

This compiler emits all required Neo N3 manifest top-level fields:

- `name`
- `groups`
- `features`
- `supportedstandards`
- `abi`
- `permissions`
- `trusts`
- `extra`

The behavior is validated by manifest tests under `src/cli/tests/manifest/`.

## Field behavior

### `features`

Neo N3 requires `features` to remain an empty object. Non-empty custom values are ignored to preserve deploy compatibility.

### `supportedstandards`

Auto-detected from contract methods/events (NEP-17, NEP-11, NEP-24 patterns) and can be overridden via custom tags.

### `permissions`

Compiler infers permissions from cross-contract/native call usage:

- fixed call targets/methods => explicit allowlist entries
- dynamic targets/methods => wildcard entries when unavoidable

### `extra`

Compiler always emits baseline metadata and supports additional custom `extra` keys via NatSpec custom tags.

## NatSpec manifest overrides

Supported tag prefixes:

- `@custom:neo.manifest.*`
- `@custom:manifest.*`

Supported override fields:

- `name`
- `groups`
- `features` (must stay `{}`)
- `supportedstandards`
- `trusts`
- `extra.<Key>` (arbitrary extra metadata fields)

Example:

```solidity
/**
 * @custom:neo.manifest.supportedstandards ["NEP-17"]
 * @custom:neo.manifest.extra.Repository "https://github.com/r3e-network/neo-solidity"
 * @custom:neo.manifest.extra.Build {"commit":"abc123"}
 */
contract MyToken {}
```

## Permission hardening switches

```bash
neo-solc contract.sol --deny-wildcard-permissions
neo-solc contract.sol --deny-wildcard-contracts
neo-solc contract.sol --deny-wildcard-methods
```

## Permission override file

You can merge or replace inferred permissions:

```bash
neo-solc contract.sol \
  --manifest-permissions permissions.json \
  --manifest-permissions-mode replace-wildcards \
  -o build/contract
```

Accepted permission JSON shape:

- array of entries, or
- object with `permissions` array

Each entry must include:

- `contract`: `"*"` or UInt160 hash string
- `methods`: `"*"` or array of method names
