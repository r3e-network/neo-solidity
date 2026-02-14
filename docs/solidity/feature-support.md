# Solidity Feature Support

Support is tracked by audited matrix, tests, and compiler diagnostics.

## Current audited coverage

- Total audited features: `141`
- Fully supported: `110` (78%)
- Partial support: `19` (13%)
- Unsupported: `3` (2%)
- Intentionally blocked: `9` (6%)

Source of truth: [`docs/SOLIDITY_SUPPORT_MATRIX.md`](../SOLIDITY_SUPPORT_MATRIX.md).

## Fully supported categories (high level)

- Core types (`bool`, `int/uint`, `address`, `bytes`, `string`, `struct`, `mapping`, arrays)
- Control flow (`if`, loops, `break`, `continue`, `return`)
- Error flow (`require`, `assert`, `revert`, custom errors)
- Inheritance and overrides
- Events to Neo notifications
- NEP-17/NEP-11/NEP-24 standards detection

## Partial features you should evaluate carefully

- `address payable` semantics
- `abi.encode` / `abi.encodePacked` behavior on Neo
- Function overloading collisions in Neo ABI name dispatch
- `receive()` / `fallback()` remapping expectations
- `msg.value` context (payment callbacks)
- `tx.origin` and witness-based authorization differences

## Intentionally blocked EVM patterns

The compiler deliberately rejects EVM-only behavior that has no safe Neo equivalent, including:

- `delegatecall`
- `address.transfer` / `address.send`
- inline `assembly {}`
- `new Contract(...)`
- `type(X).creationCode` / `runtimeCode`

## How to build with safe defaults

```bash
neo-solc MyContract.sol \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/MyContract
```

## Where to check exact behavior

1. [Syntax and Behavior](/solidity/syntax-and-behavior)
2. [EVM to NeoVM Mapping](/mapping/evm-to-neovm)
3. [Runtime Spec](/reference/runtime)
4. [Parity and Limitations](/reference/parity-limitations)
