# Devpack Standards

The compiler and devpack align Solidity contracts to Neo standards.

## Supported standards

- `NEP-17` (fungible tokens)
- `NEP-11` (non-fungible tokens)
- `NEP-24` (royalty metadata)

## Ethereum to Neo mapping

| Ethereum | Neo | Migration note |
| --- | --- | --- |
| ERC-20 | NEP-17 | 4-parameter transfer + witness auth model |
| ERC-721 | NEP-11 | `bytes32`-style token id patterns, callback-based receives |
| ERC-2981 | NEP-24 | royalty interface differences |
| EIP-165 | Manifest `supportedstandards` | interface detection is manifest-based |

Detailed mapping and examples:

- [`devpack/standards/STANDARDS_MAPPING.md`](https://github.com/r3e-network/neo-solidity/blob/main/devpack/standards/STANDARDS_MAPPING.md)

## Practical migration guidance

1. Replace allowance-centric ERC authorization with `Runtime.checkWitness`-based guards.
2. Replace EVM payment hooks with `onNEP17Payment`/`onNEP11Payment` callback patterns.
3. Verify generated `supportedstandards` and manifest permissions after compile.
