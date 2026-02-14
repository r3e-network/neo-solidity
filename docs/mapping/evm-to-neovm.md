# EVM to NeoVM Mapping

This compiler maps Solidity syntax to NeoVM instructions, native contract calls, and Neo manifest ABI.

## Core runtime values

| Solidity / EVM Concept | NeoVM / Neo N3 Mapping |
| --- | --- |
| `msg.sender` | `System.Runtime.GetCallingScriptHash` |
| `this` | `System.Runtime.GetExecutingScriptHash` |
| `block.timestamp` | `System.Runtime.GetTime` |
| `block.number` | `Ledger.currentIndex` |
| `block.chainid` | Neo network magic |
| `gasleft()` | `System.Runtime.GasLeft` |

## Cryptography and hashing

| Solidity builtin | Neo mapping |
| --- | --- |
| `keccak256` | `CryptoLib.keccak256` |
| `sha256` | `CryptoLib.sha256` |
| `ecrecover` | `CryptoLib.verifyWithECDsa` / recovery path |

## Contract calls

| Solidity pattern | Neo mapping |
| --- | --- |
| `address.call(...)` | `System.Contract.Call` |
| `address.staticcall(...)` | `System.Contract.Call` with safe/read-only intent |
| `delegatecall` | Unsupported by design |

## Storage lowering

Mappings, struct fields, and nested state paths are lowered to Neo storage operations with deterministic key derivation and hashing.

Design notes: [`docs/mapping_lowering_design.md`](../mapping_lowering_design.md).

## Standards mapping

| Ethereum | Neo |
| --- | --- |
| ERC-20 | NEP-17 |
| ERC-721 | NEP-11 |
| ERC-2981 | NEP-24 |
| EIP-165 interface checks | Manifest `supportedstandards` |

Detailed standard mapping: [`devpack/standards/STANDARDS_MAPPING.md`](https://github.com/r3e-network/neo-solidity/blob/main/devpack/standards/STANDARDS_MAPPING.md).

## Manifest impact

Cross-contract and native calls are reflected into manifest permissions. Dynamic call sites can force wildcard permissions unless you constrain call targets and methods.

Use strict flags in production:

```bash
neo-solc contract.sol --deny-wildcard-contracts --deny-wildcard-methods -o build/contract
```
