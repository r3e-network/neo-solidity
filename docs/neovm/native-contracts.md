# NeoVM Native Contracts

Neo N3 exposes major platform functionality through native contracts invoked via `System.Contract.Call`.

The compiler and devpack lower supported calls into manifest-aware native invocations.

## Core native contracts

| Native contract | Typical usage |
| --- | --- |
| `NEO` | balances, voting, candidate operations |
| `GAS` | balances and token transfers |
| `ContractManagement` | deploy/update/destroy/getContract |
| `Policy` | network fee/storage policy values |
| `Oracle` | oracle request workflows |
| `RoleManagement` | designated role queries |
| `Ledger` | block/transaction queries |

## Deterministic script hashes

The devpack exposes canonical hashes in `devpack/contracts/NativeCalls.sol` as constants.

Common examples:

- `NEO_CONTRACT = 0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5`
- `GAS_CONTRACT = 0xd2a4cff31913016155e38e474a2c06d08be276cf`
- `CONTRACT_MANAGEMENT = 0xfffdc93764dbaddd97c48f252a53ea4643faa3fd`

## Permission model

Calling native contracts requires matching manifest permissions.

The compiler infers permissions from IR and emits explicit `contract + methods` entries where possible. Fully dynamic calls may require wildcards.

Harden with:

```bash
neo-solc contract.sol --deny-wildcard-contracts --deny-wildcard-methods -o build/contract
```

## Recommended usage

1. Prefer fixed native calls (`NativeCalls.*`) over dynamic generic call wrappers.
2. Keep method names static where possible.
3. Audit generated manifest before deployment.
