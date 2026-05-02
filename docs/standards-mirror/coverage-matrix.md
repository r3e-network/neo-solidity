---
title: Standards Mirror — Coverage Matrix
description: Every mirrored ERC/EIP standard with category, EIP status, Neo mapping, and live TestNet deployment status.
---

# Standards Mirror — Coverage Matrix

Generated from `docs/standards-mirror/deployments/results.json` snapshot
`2026-04-29T02:47:17.306Z`. **64** mirror pages, **47** with live
Solidity + Neo C# pairs on TestNet (network magic `894710606`).

Use this page to scan every mirrored standard side-by-side: catalog status,
Neo equivalent, and the latest assertion pass-rate from the most recent deploy
snapshot. Per-pair contract addresses and per-test detail live in
[`RESULTS.md`](./deployments/RESULTS).

> **Read the columns:**
> - **EIP Status** — the spec status on Ethereum (Final / Review / Stagnant / Pectra).
> - **Neo Mapping** — what the Solidity-on-Neo or Neo C# implementation reduces to.
> - **Solidity** / **Neo C#** — pass / total assertion count from the snapshot.
> - **—** in the deploy columns means no live pair (catalog-only entry).

## Token Standards

18 mirrored — 13 deployed.

| Standard | EIP Status | Neo Mapping | Solidity | Neo C# |
| --- | --- | --- | --- | --- |
| [ERC-20 — Fungible Token](/standards-mirror/tokens/erc-20) | Final | NEP-17 | 3 / 5 | 4 / 4 |
| [ERC-721 — Non-Fungible Token](/standards-mirror/tokens/erc-721) | Final | NEP-11 | 2 / 3 | 3 / 5 |
| [ERC-777 — Token w/ Hooks](/standards-mirror/tokens/erc-777) | Final | NEP-17 + NEP-27 | 1 / 3 | 3 / 3 |
| [ERC-1155 — Multi-Token](/standards-mirror/tokens/erc-1155) | Final | NEP-11 (divisible) + NEP-17 | 0 / 1 | 1 / 1 |
| [ERC-1363 — Payable Token](/standards-mirror/tokens/erc-1363) | Final | NEP-17 callback | — | — |
| [ERC-2981 — NFT Royalty Standard](/standards-mirror/tokens/erc-2981) | Final | NEP-24 | 3 / 3 | 2 / 3 |
| [ERC-3525 — Semi-Fungible Token](/standards-mirror/tokens/erc-3525) | Final | Neo C# port | 4 / 4 | 3 / 3 |
| [ERC-2309 — Consecutive NFT Mints](/standards-mirror/tokens/erc-2309) | Final | Neo C# port | 1 / 2 | 2 / 2 |
| [ERC-4906 — NFT Metadata Update](/standards-mirror/tokens/erc-4906) | Final | Neo C# port | 1 / 2 | 2 / 2 |
| [ERC-4907 — Rental NFT (User Role)](/standards-mirror/tokens/erc-4907) | Final | NEP-11 + per-token user/expires | — | — |
| [ERC-4494 — Permit for ERC-721](/standards-mirror/tokens/erc-4494) | Final | Native witness scopes | 1 / 2 | 2 / 2 |
| [ERC-5192 — Soulbound NFTs](/standards-mirror/tokens/erc-5192) | Final | Neo C# port | 2 / 3 | 1 / 1 |
| [ERC-5484 — Consensual Soulbound](/standards-mirror/tokens/erc-5484) | Final | Neo C# port | 1 / 2 | 1 / 1 |
| [ERC-6093 — Custom Errors for Common Tokens](/standards-mirror/tokens/erc-6093) | Final | Named exception convention | — | — |
| [ERC-6909 — Minimal Multi-Token](/standards-mirror/tokens/erc-6909) | Final | Neo C# port | — | — |
| [ERC-5114 — Soulbound Badge](/standards-mirror/tokens/erc-5114) | Final | Neo C# port | 1 / 2 | 2 / 2 |
| [ERC-6147 — NFT Guard](/standards-mirror/tokens/erc-6147) | Final | Neo C# port | 2 / 2 | 2 / 2 |
| [ERC-7528 — Native Asset Address Convention](/standards-mirror/tokens/erc-7528) | Final | Native NEO / GAS contract hashes | — | — |

## Account & Authentication

14 mirrored — 9 deployed.

| Standard | EIP Status | Neo Mapping | Solidity | Neo C# |
| --- | --- | --- | --- | --- |
| [ERC-173 — Contract Ownership](/standards-mirror/account-and-auth/erc-173) | Final | Owner pattern + NEP-22 | 1 / 2 | 1 / 1 |
| [ERC-1271 — Smart Contract Signatures](/standards-mirror/account-and-auth/erc-1271) | Final | Native witness model | 1 / 1 | 1 / 1 |
| [ERC-2612 — Permit (Gasless Approval)](/standards-mirror/account-and-auth/erc-2612) | Final | Native witness scopes | 1 / 2 | 2 / 2 |
| [ERC-3009 — Transfer With Authorization](/standards-mirror/account-and-auth/erc-3009) | Final (USDC) / Stagnant | Native witness scopes | — | — |
| [ERC-4337 — Account Abstraction](/standards-mirror/account-and-auth/erc-4337) | Final | Native NEP-30 verify | 1 / 2 | 2 / 2 |
| [ERC-4361 — Sign-In with Ethereum (SIWE)](/standards-mirror/account-and-auth/erc-4361) | Final | Native witness over domain-bound message | — | — |
| [EIP-712 — Typed Structured Data Signing](/standards-mirror/account-and-auth/eip-712) | Final | Native witness model | 0 / 1 | 2 / 2 |
| [EIP-191 — Signed Data Prefix](/standards-mirror/account-and-auth/eip-191) | Final | Native witness model | 0 / 1 | 1 / 1 |
| [ERC-5267 — EIP-712 Domain Retrieval](/standards-mirror/account-and-auth/erc-5267) | Final | Native | 2 / 2 | 2 / 2 |
| [ERC-6492 — Signatures for Pre-deployed Contracts](/standards-mirror/account-and-auth/erc-6492) | Final | Native (no counterfactual) | 1 / 2 | 2 / 2 |
| [ERC-6551 — Token Bound Accounts](/standards-mirror/account-and-auth/erc-6551) | Review | Registry + per-NFT contract | — | — |
| [ERC-7656 — Generalized Contract-Linked Services](/standards-mirror/account-and-auth/erc-7656) | Final | Registry pattern (mode-flagged) | — | — |
| [EIP-7702 — Set Code for EOAs](/standards-mirror/account-and-auth/eip-7702) | Final (Pectra) | Native (every account is a contract) | 1 / 2 | 2 / 2 |
| [EIP-3074 — AUTH and AUTHCALL](/standards-mirror/account-and-auth/eip-3074) | Stagnant | Native (witness scopes) | — | — |

## Infrastructure & Patterns

14 mirrored — 10 deployed.

| Standard | EIP Status | Neo Mapping | Solidity | Neo C# |
| --- | --- | --- | --- | --- |
| [ERC-165 — Standard Interface Detection](/standards-mirror/infrastructure/erc-165) | Final | Manifest `supportedstandards` | 1 / 1 | 2 / 2 |
| [ERC-1014 — Deterministic Deploy (CREATE2)](/standards-mirror/infrastructure/erc-1014) | Final | ContractManagement + nonce | 1 / 2 | 2 / 2 |
| [ERC-1056 — Lightweight DID Registry](/standards-mirror/infrastructure/erc-1056) | Final | Neo C# port | 1 / 1 | 1 / 1 |
| [ERC-1167 — Minimal Proxy (Clones)](/standards-mirror/infrastructure/erc-1167) | Final | ContractManagement.Deploy (parameterised) | — | — |
| [ERC-1820 — Pseudo-introspection Registry](/standards-mirror/infrastructure/erc-1820) | Final | Neo C# port | 1 / 1 | 1 / 1 |
| [ERC-1967 — Proxy Storage Slots](/standards-mirror/infrastructure/erc-1967) | Final | NEP-22 in-place update | 1 / 2 | 2 / 2 |
| [ERC-2470 — Singleton Factory](/standards-mirror/infrastructure/erc-2470) | Final | ContractManagement deterministic | 1 / 2 | 2 / 2 |
| [ERC-2535 — Diamond Multi-Facet Proxy](/standards-mirror/infrastructure/erc-2535) | Final | Method-name router pattern | 1 / 2 | 1 / 1 |
| [ERC-2771 — Trusted Forwarder (Meta-Tx)](/standards-mirror/infrastructure/erc-2771) | Final | Native witness scopes | 1 / 2 | 1 / 1 |
| [ERC-3448 — MetaProxy Standard](/standards-mirror/infrastructure/erc-3448) | Final | ContractManagement.Deploy with constant slots | — | — |
| [ERC-3668 — CCIP Read (Off-chain Data)](/standards-mirror/infrastructure/erc-3668) | Final | Native Oracle service | — | — |
| [ERC-7201 — Namespaced Storage Layout](/standards-mirror/infrastructure/erc-7201) | Final | Storage prefix convention | 0 / 1 | 2 / 2 |
| [ERC-7579 — Modular Smart Account](/standards-mirror/infrastructure/erc-7579) | Final | NEP-30 verify + module dispatch | 1 / 2 | 2 / 2 |
| [ERC-7786 — Cross-Chain Messaging Gateway](/standards-mirror/infrastructure/erc-7786) | Final | Bridge-adapter pattern | — | — |

## DeFi Building Blocks

8 mirrored — 7 deployed.

| Standard | EIP Status | Neo Mapping | Solidity | Neo C# |
| --- | --- | --- | --- | --- |
| [ERC-3156 — Flash Loans](/standards-mirror/defi/erc-3156) | Final | NEP-17 callback flow | 1 / 1 | 1 / 1 |
| [ERC-4626 — Tokenized Vaults](/standards-mirror/defi/erc-4626) | Final | NEP-17 vault pattern | 2 / 2 | 2 / 2 |
| [ERC-5805 — Voting Token w/ Delegation](/standards-mirror/defi/erc-5805) | Final | Neo C# port | 1 / 2 | 2 / 2 |
| [ERC-6372 — Contract Clock](/standards-mirror/defi/erc-6372) | Review | Runtime.Time / block height | 1 / 1 | 1 / 1 |
| [ERC-7535 — Native Asset ERC-4626 Vault](/standards-mirror/defi/erc-7535) | Final | NEP-17 vault with NEO/GAS | — | — |
| [ERC-7540 — Asynchronous ERC-4626 Vaults](/standards-mirror/defi/erc-7540) | Final | Request/claim queue pattern | 2 / 3 | 3 / 3 |
| [ERC-7575 — Multi-Asset ERC-4626 Vaults](/standards-mirror/defi/erc-7575) | Final | Multi-NEP-17 vault | 1 / 2 | 2 / 2 |
| [ERC-7818 — Expirable ERC-20](/standards-mirror/defi/erc-7818) | Final | Neo C# port (timestamp expiry) | 1 / 1 | 1 / 1 |

## Protocol-Level EIPs

10 mirrored — 8 deployed.

| Standard | EIP Status | Neo Mapping | Solidity | Neo C# |
| --- | --- | --- | --- | --- |
| [EIP-1153 — Transient Storage](/standards-mirror/protocol-eips/eip-1153) | Final | NeoVM stack-only opcodes | 2 / 3 | 2 / 3 |
| [EIP-1559 — Fee Market Auction](/standards-mirror/protocol-eips/eip-1559) | Final | N/A — Neo uses flat sysfee | — | — |
| [EIP-2098 — Compact Signature Representation](/standards-mirror/protocol-eips/eip-2098) | Final | Native ECDSA | 2 / 2 | 2 / 2 |
| [EIP-2718 — Typed Transaction Envelope](/standards-mirror/protocol-eips/eip-2718) | Final | Neo Transaction.Version | 3 / 3 | 2 / 2 |
| [EIP-2930 — Access List](/standards-mirror/protocol-eips/eip-2930) | Final | Native witness scopes | 2 / 2 | 2 / 2 |
| [EIP-3198 — BASEFEE Opcode](/standards-mirror/protocol-eips/eip-3198) | Final | Runtime.GasLeft / no base fee | 1 / 2 | 1 / 1 |
| [EIP-3855 — PUSH0 Opcode](/standards-mirror/protocol-eips/eip-3855) | Final | NeoVM PUSH0 | 2 / 2 | 2 / 2 |
| [EIP-3860 — Initcode Size Limit](/standards-mirror/protocol-eips/eip-3860) | Final | NEF size limits | 1 / 1 | 1 / 1 |
| [EIP-4844 — Blob Transactions](/standards-mirror/protocol-eips/eip-4844) | Final | N/A — Neo has no blobs | — | — |
| [EIP-6780 — SELFDESTRUCT Nerf](/standards-mirror/protocol-eips/eip-6780) | Final | ContractManagement.Destroy (NEP-31) | 1 / 2 | 2 / 2 |

## Aggregate

- **Catalog total:** 64 ERC/EIP pages
- **Deployed pairs:** 47
- **Catalog-only:** 17 (deliberate; see [DEFERRED.md](./deployments/DEFERRED))
- **Solidity assertions:** 62 / 94 pass
- **Neo C# assertions:** 85 / 89 pass
- **Combined assertion pass-rate:** 147 / 183

## Related

- [Standards Mirror Overview](./)
- [Latest TestNet Results](./deployments/RESULTS)
- [Deferred Deployment Queue](./deployments/DEFERRED)
- [Coverage Audit & Gap Report](./coverage-audit)
