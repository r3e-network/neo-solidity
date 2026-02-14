# Syntax and Behavior

This page summarizes behavior differences that matter when moving Solidity contracts to Neo N3.

## Authorization model

- Ethereum idiom: `msg.sender` + allowance/proxy patterns.
- Neo idiom: witness verification (`Runtime.checkWitness(address)`).

Implication: designs based on `approve/allowance/transferFrom` should be reconsidered for Neo-first flows.

## Value transfer model

Neo contracts do not receive native value in the EVM sense.

- `receive()` and fallback payment patterns do not map directly.
- Token payments are handled through NEP callbacks (`onNEP17Payment`, `onNEP11Payment`).

## ABI and encoding behavior

`abi.encode`/`abi.decode` are supported for Neo interop workflows, but byte-level equivalence with Ethereum ABI is not the design goal for all paths.

Use compiler/devpack patterns for cross-contract calls on Neo instead of assuming Ethereum ABI wire compatibility.

## Storage model

State variables and mappings are lowered to Neo storage access with deterministic key derivation.

- Nested mappings and struct storage are supported.
- Array and struct lowering follows NeoVM/Storage constraints rather than EVM slot rules.

## Exceptions and assertions

- `require`, `assert`, `revert`, custom errors are supported.
- `try/catch` lowering works on NeoVM exception semantics, not exact EVM panic-channel semantics.

## Function dispatch and overloading

Neo dispatches by method names declared in manifest ABI.

- Overloads with conflicting Neo-dispatch shape can fail or require mangled names.
- Keep public/external ABI names unique and explicit.

## Recommended migration pattern

1. Start from strict compile flags.
2. Resolve warnings from unsupported/partial features.
3. Validate runtime and deployment with Neo-Express smoke tests.
4. Confirm manifest permissions and standards before production.
