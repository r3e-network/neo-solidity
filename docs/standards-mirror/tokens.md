---
title: "Standards Mirror Token Standards"
description: Every Ethereum token standard mirrored to its Neo N3 implementation — fungibles, NFTs, multi-token, semi-fungible, soulbound, royalty.
---

# Standards Mirror Token Standards

Twenty-four Ethereum token standards, each shown with the Solidity reference and an idiomatic
Neo C# implementation. Where a NEP exists (NEP-17, NEP-11, NEP-24), the C# tab uses
that. Where no NEP exists, the C# tab shows a clean Neo port — proof that the standard
is implementable on Neo today.

## Standards

| Standard | Neo Mapping | Status | Category |
| --- | --- | --- | --- |
| [ERC-20 — Fungible Token](/standards-mirror/tokens/erc-20) | NEP-17 | Final | Fungible |
| [ERC-721 — Non-Fungible Token](/standards-mirror/tokens/erc-721) | NEP-11 | Final | NFT |
| [ERC-777 — Token w/ Hooks](/standards-mirror/tokens/erc-777) | NEP-17 + NEP-27 callback | Final | Fungible |
| [ERC-1155 — Multi-Token](/standards-mirror/tokens/erc-1155) | NEP-11 (divisible) + NEP-17 | Final | Multi-Token |
| [ERC-1363 — Payable Token](/standards-mirror/tokens/erc-1363) | NEP-17 (`onNEP17Payment`) | Final | Fungible Extension |
| [ERC-2981 — NFT Royalty Standard](/standards-mirror/tokens/erc-2981) | NEP-24 | Final | NFT Extension |
| [ERC-6093 — Custom Errors for Common Tokens](/standards-mirror/tokens/erc-6093) | Named exception convention | Final | Convention |
| [ERC-7528 — Native Asset Address](/standards-mirror/tokens/erc-7528) | Native NEO / GAS contract hashes | Final | Convention |
| [ERC-3525 — Semi-Fungible Token](/standards-mirror/tokens/erc-3525) | Neo C# port | Final | Hybrid |
| [ERC-2309 — Consecutive NFT Mints](/standards-mirror/tokens/erc-2309) | Neo C# port | Final | NFT Extension |
| [ERC-4906 — NFT Metadata Update](/standards-mirror/tokens/erc-4906) | Neo C# port | Final | NFT Extension |
| [ERC-4494 — Permit for ERC-721](/standards-mirror/tokens/erc-4494) | Native witness scopes | Final | NFT Extension |
| [ERC-5192 — Soulbound NFTs](/standards-mirror/tokens/erc-5192) | Neo C# port | Final | NFT Extension |
| [ERC-5484 — Consensual Soulbound](/standards-mirror/tokens/erc-5484) | Neo C# port | Final | NFT Extension |
| [ERC-6909 — Minimal Multi-Token](/standards-mirror/tokens/erc-6909) | Neo C# port | Final | Multi-Token |
| [ERC-5114 — Soulbound Badge](/standards-mirror/tokens/erc-5114) | Neo C# port | Final | NFT Extension |
| [ERC-6147 — NFT Guard](/standards-mirror/tokens/erc-6147) | Neo C# port | Final | NFT Extension |
| [ERC-4907 — Rental NFT (User Role)](/standards-mirror/tokens/erc-4907) | NEP-11 + per-token user/expires | Final | NFT Extension |
| [ERC-3643 — T-REX Regulated Token](/standards-mirror/tokens/erc-3643) | NEP-17 + identity registry + compliance modules | Final | Compliance |
| [ERC-5679 — Token Minting and Burning](/standards-mirror/tokens/erc-5679) | NEP-17 / NEP-11 mint/burn helper convention | Final | Convention |
| [ERC-2135 — Consumable Interface](/standards-mirror/tokens/erc-2135) | NEP-11 + per-token consumed flag | Final | NFT Extension |
| [ERC-7160 — ERC-721 Multi-Metadata Extension](/standards-mirror/tokens/erc-7160) | NEP-11 + URI list + active-index storage | Final | NFT Extension |
| [ERC-6982 — Default Lockable Tokens](/standards-mirror/tokens/erc-6982) | NEP-11 + lock-state storage | Final | NFT Extension |
| [ERC-7144 — ERC-20 with Transaction Validation Step](/standards-mirror/tokens/erc-7144) | NEP-17 transfer override + validator contract | Review | Fungible Extension |

## Related Pages

- [Standards Mirror Overview](/standards-mirror/)
- [Semantic Standards Mapping](/mapping/standards)
- [Standards and Contracts](/additional-material/neo-standards)
