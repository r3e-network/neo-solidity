---
title: Standards Mirror — Coverage Matrix
description: Every mirrored ERC/EIP standard with category, EIP status, Neo mapping, and live TestNet deployment status.
---

# Standards Mirror — Coverage Matrix

Generated from `docs/standards-mirror/deployments/results.json` snapshot
`2026-04-29T02:47:17.306Z`. **104** mirror pages, **47** with live
Solidity + Neo C# pairs on TestNet (network magic `894710606`).

> **Read the columns:**
> - **EIP Status** — the spec status on Ethereum (Final / Review / Stagnant / Pectra).
> - **Neo Mapping** — what the Solidity-on-Neo or Neo C# implementation reduces to.
> - **Solidity** / **Neo C#** — pass / total assertion count from the snapshot.
> - **—** in the deploy columns means no live pair (catalog-only entry).

## Token Standards

46 mirrored — 13 deployed.

| Standard | EIP Status | Neo Mapping | Solidity | Neo C# |
| --- | --- | --- | --- | --- |
| [ERC-20 — Fungible Token](/standards-mirror/tokens/erc-20) | Final | NEP-17 | 3 / 5 | 4 / 4 |
| [ERC-721 — Non-Fungible Token](/standards-mirror/tokens/erc-721) | Final | NEP-11 | 2 / 3 | 3 / 5 |
| [ERC-777 — Token w/ Hooks](/standards-mirror/tokens/erc-777) | Final | NEP-17 + NEP-27 | 1 / 3 | 3 / 3 |
| [ERC-1155 — Multi-Token](/standards-mirror/tokens/erc-1155) | Final | NEP-11 (divisible) + NEP-17 | 0 / 1 | 1 / 1 |
| [ERC-1363 — Payable Token](/standards-mirror/tokens/erc-1363) | Final | NEP-17 callback | — | — |
| [ERC-2135 — Consumable Interface](/standards-mirror/tokens/erc-2135) | Final | NEP-11 + per-token consumed flag | — | — |
| [ERC-2981 — NFT Royalty Standard](/standards-mirror/tokens/erc-2981) | Final | NEP-24 | 3 / 3 | 2 / 3 |
| [ERC-3525 — Semi-Fungible Token](/standards-mirror/tokens/erc-3525) | Final | Neo C# port | 4 / 4 | 3 / 3 |
| [ERC-3643 — T-REX Regulated Token](/standards-mirror/tokens/erc-3643) | Final | NEP-17 + identity + compliance | — | — |
| [ERC-2309 — Consecutive NFT Mints](/standards-mirror/tokens/erc-2309) | Final | Neo C# port | 1 / 2 | 2 / 2 |
| [ERC-4519 — NFTs Tied to Physical Assets](/standards-mirror/tokens/erc-4519) | Final | NEP-11 + device pubkey + state machine | — | — |
| [ERC-4906 — NFT Metadata Update](/standards-mirror/tokens/erc-4906) | Final | Neo C# port | 1 / 2 | 2 / 2 |
| [ERC-4907 — Rental NFT (User Role)](/standards-mirror/tokens/erc-4907) | Final | NEP-11 + per-token user/expires | — | — |
| [ERC-4494 — Permit for ERC-721](/standards-mirror/tokens/erc-4494) | Final | Native witness scopes | 1 / 2 | 2 / 2 |
| [ERC-5006 — Rental NFT, NFT User Extension](/standards-mirror/tokens/erc-5006) | Final | NEP-11 (divisible) + per-record user/expires | — | — |
| [ERC-5023 — Shareable Non-Fungible Token](/standards-mirror/tokens/erc-5023) | Final | NEP-11 with multi-holder share extension | — | — |
| [ERC-5169 — Client Script URI for Token Contracts](/standards-mirror/tokens/erc-5169) | Final | NEP-11 / NEP-17 + scriptURI() view | — | — |
| [ERC-5192 — Soulbound NFTs](/standards-mirror/tokens/erc-5192) | Final | Neo C# port | 2 / 3 | 1 / 1 |
| [ERC-5375 — NFT Author Information and Consent](/standards-mirror/tokens/erc-5375) | Final | NEP-11 + author/consent metadata fields | — | — |
| [ERC-5380 — ERC-721 Entitlement Extension](/standards-mirror/tokens/erc-5380) | Final | NEP-11 + per-(tokenId, action, delegate) | — | — |
| [ERC-5484 — Consensual Soulbound](/standards-mirror/tokens/erc-5484) | Final | Neo C# port | 1 / 2 | 1 / 1 |
| [ERC-5489 — NFT Hyperlink Extension](/standards-mirror/tokens/erc-5489) | Final | NEP-11 + per-(tokenId, slot) URL storage | — | — |
| [ERC-5570 — Digital Receipt NFT](/standards-mirror/tokens/erc-5570) | Final | NEP-11 + structured receipt metadata schema | — | — |
| [ERC-5615 — ERC-1155 Supply Extension](/standards-mirror/tokens/erc-5615) | Final | NEP-11 (divisible) + per-tokenId supply | — | — |
| [ERC-5679 — Token Minting and Burning](/standards-mirror/tokens/erc-5679) | Final | NEP-17 / NEP-11 mint/burn helper | — | — |
| [ERC-5725 — Transferable Vesting NFT](/standards-mirror/tokens/erc-5725) | Final | NEP-11 + per-token vesting + claim | — | — |
| [ERC-5773 — Context-Dependent Multi-Asset Tokens](/standards-mirror/tokens/erc-5773) | Final | NEP-11 + per-token asset list + priority | — | — |
| [ERC-6059 — Parent-Governed Nestable NFTs](/standards-mirror/tokens/erc-6059) | Final | NEP-11 + parent-child storage + accept flow | — | — |
| [ERC-6093 — Custom Errors for Common Tokens](/standards-mirror/tokens/erc-6093) | Final | Named exception convention | — | — |
| [ERC-6105 — No Intermediary NFT Trading Protocol](/standards-mirror/tokens/erc-6105) | Final | NEP-11 with built-in listing + atomic-swap | — | — |
| [ERC-6150 — Hierarchical NFTs](/standards-mirror/tokens/erc-6150) | Final | NEP-11 + parent/children pointer storage | — | — |
| [ERC-6220 — Composable NFTs Equippable Parts](/standards-mirror/tokens/erc-6220) | Final | NEP-11 + catalog + equip-slot storage | — | — |
| [ERC-6454 — Minimal Transferable NFT detection](/standards-mirror/tokens/erc-6454) | Final | NEP-11 + IsTransferable view | — | — |
| [ERC-6672 — Multi-Redeemable NFTs](/standards-mirror/tokens/erc-6672) | Final | NEP-11 + per-(tokenId, operator, redemptionId) state | — | — |
| [ERC-6909 — Minimal Multi-Token](/standards-mirror/tokens/erc-6909) | Final | Neo C# port | — | — |
| [ERC-6982 — Default Lockable Tokens](/standards-mirror/tokens/erc-6982) | Final | NEP-11 + lock-state storage | — | — |
| [ERC-5114 — Soulbound Badge](/standards-mirror/tokens/erc-5114) | Final | Neo C# port | 1 / 2 | 2 / 2 |
| [ERC-6147 — NFT Guard](/standards-mirror/tokens/erc-6147) | Final | Neo C# port | 2 / 2 | 2 / 2 |
| [ERC-7007 — Verifiable AI-Generated Content Token](/standards-mirror/tokens/erc-7007) | Final | NEP-11 + AIGC attestation | — | — |
| [ERC-7066 — Lockable Extension for ERC-721](/standards-mirror/tokens/erc-7066) | Final | NEP-11 + per-token approval-based lock | — | — |
| [ERC-7144 — ERC-20 with Transaction Validation Step](/standards-mirror/tokens/erc-7144) | Review | NEP-17 transfer override + validator | — | — |
| [ERC-7160 — ERC-721 Multi-Metadata Extension](/standards-mirror/tokens/erc-7160) | Final | NEP-11 + URI list + active-index | — | — |
| [ERC-7432 — Non-Fungible Token Roles](/standards-mirror/tokens/erc-7432) | Final | NEP-11 + (tokenId, role) → grant w/ TTL | — | — |
| [ERC-7528 — Native Asset Address Convention](/standards-mirror/tokens/erc-7528) | Final | Native NEO / GAS contract hashes | — | — |
| [ERC-7634 — Limited Transfer Count NFT](/standards-mirror/tokens/erc-7634) | Final | NEP-11 + per-token transfer counter | — | — |
| [ERC-7943 — Universal Real World Asset Interface](/standards-mirror/tokens/erc-7943) | Last Call | NEP-17/NEP-11 + capability flags + compliance | — | — |

## Account & Authentication

21 mirrored — 9 deployed.

| Standard | EIP Status | Neo Mapping | Solidity | Neo C# |
| --- | --- | --- | --- | --- |
| [ERC-173 — Contract Ownership](/standards-mirror/account-and-auth/erc-173) | Final | Owner pattern + NEP-22 | 1 / 2 | 1 / 1 |
| [ERC-2767 — Contract Ownership Governance](/standards-mirror/account-and-auth/erc-2767) | Final | Governance contract as owner + NEP-22 gate | — | — |
| [ERC-5313 — Light Contract Ownership](/standards-mirror/account-and-auth/erc-5313) | Final | `getOwner()` view convention | — | — |
| [ERC-1271 — Smart Contract Signatures](/standards-mirror/account-and-auth/erc-1271) | Final | Native witness model | 1 / 1 | 1 / 1 |
| [ERC-2612 — Permit (Gasless Approval)](/standards-mirror/account-and-auth/erc-2612) | Final | Native witness scopes | 1 / 2 | 2 / 2 |
| [ERC-3009 — Transfer With Authorization](/standards-mirror/account-and-auth/erc-3009) | Final (USDC) / Stagnant | Native witness scopes | — | — |
| [ERC-7758 — Transfer With Authorization (modern)](/standards-mirror/account-and-auth/erc-7758) | Review | Native witness scopes | — | — |
| [ERC-4337 — Account Abstraction](/standards-mirror/account-and-auth/erc-4337) | Final | Native NEP-30 verify | 1 / 2 | 2 / 2 |
| [ERC-4361 — Sign-In with Ethereum (SIWE)](/standards-mirror/account-and-auth/erc-4361) | Final | Native witness over domain-bound message | — | — |
| [ERC-5564 — Stealth Address Scheme](/standards-mirror/account-and-auth/erc-5564) | Final | secp256r1 ECDH + announcer contract | — | — |
| [ERC-6066 — Signature Validation Method for NFTs](/standards-mirror/account-and-auth/erc-6066) | Final | NEP-11 + NEP-30 verify, per tokenId | — | — |
| [EIP-712 — Typed Structured Data Signing](/standards-mirror/account-and-auth/eip-712) | Final | Native witness model | 0 / 1 | 2 / 2 |
| [EIP-191 — Signed Data Prefix](/standards-mirror/account-and-auth/eip-191) | Final | Native witness model | 0 / 1 | 1 / 1 |
| [ERC-5267 — EIP-712 Domain Retrieval](/standards-mirror/account-and-auth/erc-5267) | Final | Native | 2 / 2 | 2 / 2 |
| [ERC-6492 — Signatures for Pre-deployed Contracts](/standards-mirror/account-and-auth/erc-6492) | Final | Native (no counterfactual) | 1 / 2 | 2 / 2 |
| [ERC-6551 — Token Bound Accounts](/standards-mirror/account-and-auth/erc-6551) | Review | Registry + per-NFT contract | — | — |
| [ERC-7656 — Generalized Contract-Linked Services](/standards-mirror/account-and-auth/erc-7656) | Final | Registry pattern (mode-flagged) | — | — |
| [ERC-7677 — Paymaster Web Service Capability](/standards-mirror/account-and-auth/erc-7677) | Review | Sponsor relayer + signed gas-budget | — | — |
| [ERC-7715 — Permission Grants for Smart Accounts](/standards-mirror/account-and-auth/erc-7715) | Draft | Witness scopes + per-grant allowlist | — | — |
| [EIP-7702 — Set Code for EOAs](/standards-mirror/account-and-auth/eip-7702) | Final (Pectra) | Native (every account is a contract) | 1 / 2 | 2 / 2 |
| [EIP-3074 — AUTH and AUTHCALL](/standards-mirror/account-and-auth/eip-3074) | Stagnant | Native (witness scopes) | — | — |

## Infrastructure & Patterns

16 mirrored — 10 deployed.

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
| [ERC-5202 — Blueprint Contract Format](/standards-mirror/infrastructure/erc-5202) | Final | NEF blob in Storage + ContractManagement.Deploy | — | — |
| [ERC-7201 — Namespaced Storage Layout](/standards-mirror/infrastructure/erc-7201) | Final | Storage prefix convention | 0 / 1 | 2 / 2 |
| [ERC-7579 — Modular Smart Account](/standards-mirror/infrastructure/erc-7579) | Final | NEP-30 verify + module dispatch | 1 / 2 | 2 / 2 |
| [ERC-7786 — Cross-Chain Messaging Gateway](/standards-mirror/infrastructure/erc-7786) | Final | Bridge-adapter pattern | — | — |
| [ERC-8042 — Diamond Storage](/standards-mirror/infrastructure/erc-8042) | Final | Storage prefix per facet | — | — |

## DeFi Building Blocks

11 mirrored — 7 deployed.

| Standard | EIP Status | Neo Mapping | Solidity | Neo C# |
| --- | --- | --- | --- | --- |
| [ERC-3156 — Flash Loans](/standards-mirror/defi/erc-3156) | Final | NEP-17 callback flow | 1 / 1 | 1 / 1 |
| [ERC-3475 — Abstract Storage Bonds](/standards-mirror/defi/erc-3475) | Final | Class/nonce keyed bond storage | — | — |
| [ERC-4626 — Tokenized Vaults](/standards-mirror/defi/erc-4626) | Final | NEP-17 vault pattern | 2 / 2 | 2 / 2 |
| [ERC-5805 — Voting Token w/ Delegation](/standards-mirror/defi/erc-5805) | Final | Neo C# port | 1 / 2 | 2 / 2 |
| [ERC-6372 — Contract Clock](/standards-mirror/defi/erc-6372) | Review | Runtime.Time / block height | 1 / 1 | 1 / 1 |
| [ERC-7092 — Financial Bonds](/standards-mirror/defi/erc-7092) | Final | Issuer-facing bond surface | — | — |
| [ERC-7535 — Native Asset ERC-4626 Vault](/standards-mirror/defi/erc-7535) | Final | NEP-17 vault with NEO/GAS | — | — |
| [ERC-7540 — Asynchronous ERC-4626 Vaults](/standards-mirror/defi/erc-7540) | Final | Request/claim queue pattern | 2 / 3 | 3 / 3 |
| [ERC-7575 — Multi-Asset ERC-4626 Vaults](/standards-mirror/defi/erc-7575) | Final | Multi-NEP-17 vault | 1 / 2 | 2 / 2 |
| [ERC-7818 — Expirable ERC-20](/standards-mirror/defi/erc-7818) | Final | Neo C# port (timestamp expiry) | 1 / 1 | 1 / 1 |
| [ERC-7944 — Async Cancellation for ERC-7540](/standards-mirror/defi/erc-7944) | Final | Cancel-by-id on the request queue | — | — |

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

- **Catalog total:** 104 ERC/EIP pages
- **Deployed pairs:** 47
- **Catalog-only:** 57 (deliberate; see [DEFERRED.md](./deployments/DEFERRED))
- **Solidity assertions:** 62 / 94 pass
- **Neo C# assertions:** 85 / 89 pass
- **Combined assertion pass-rate:** 147 / 183

## Related

- [Standards Mirror Overview](./)
- [Latest TestNet Results](./deployments/RESULTS)
- [Deferred Deployment Queue](./deployments/DEFERRED)
- [Coverage Audit & Gap Report](./coverage-audit)
