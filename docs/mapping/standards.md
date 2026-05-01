# Standards Mapping

Neo DevPack for Solidity supports Neo standards through devpack contracts, manifest
generation, and the deployable ERC/EIP to Neo mirror.

## Standard Equivalents

| Ethereum Standard | Neo Equivalent | Main Difference |
| --- | --- | --- |
| ERC-20 | NEP-17 | NEP-17 uses witness authorization and a 4-parameter transfer. |
| ERC-721 | NEP-11 | NEP-11 requires `tokensOf` and `properties`; token IDs are commonly `bytes32`. |
| ERC-2981 | NEP-24 | Neo royalties can include multiple recipients and a royalty token parameter. |
| EIP-165 | Manifest `supportedstandards` | Interface detection is manifest-based. |
| EIP-2612 permit | `Runtime.checkWitness` | Neo witnesses remove the need for ERC-20 permit approval flow in many cases. |
| EIP-1967 proxy | NEP-22 / NEP-29 / NEP-31 | Neo uses native update, deploy, and destroy lifecycle hooks. |

## Manifest Detection

The compiler inspects public methods and events and populates
`supportedstandards` in the manifest. For example, a valid NEP-17 contract must
expose `symbol`, `decimals`, `totalSupply`, `balanceOf`, and the NEP-17
`transfer` shape.

## Deployed ERC/EIP Mirror

The [ERC / EIP to Neo Mirror](/standards-mirror/) contains deployable Solidity
and Neo C# pairs for representative standards. Use it when you need executable
side-by-side examples instead of only semantic guidance.

## More Detail

- [Standards and Contracts](/additional-material/neo-standards)
- [ERC / EIP to Neo Mirror](/standards-mirror/)
- [Calls and Assets](/mapping/calls-and-assets)
