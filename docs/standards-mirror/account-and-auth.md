---
title: Account & Authentication — ERC ↔ Neo Mirror
description: Ethereum signature, ownership, and account-abstraction standards mirrored to Neo's native witness model and idiomatic C# patterns.
---

# Account & Authentication

Ten standards covering ownership, off-chain signatures, smart-contract signature
verification, account abstraction, and EOA-as-contract proposals. Most of these are
elaborate workarounds for Ethereum's "EOA can only do ECDSA" limitation; Neo's
witness model handles the same use cases at the protocol level.

## Standards

| Standard | Neo Mapping | Status | Category |
| --- | --- | --- | --- |
| [ERC-173 — Contract Ownership](/standards-mirror/account-and-auth/erc-173) | Owner pattern + NEP-22 | Final | Ownership |
| [ERC-1271 — Smart Contract Signatures](/standards-mirror/account-and-auth/erc-1271) | Native witness model | Final | Signatures |
| [ERC-2612 — Permit (Gasless Approval)](/standards-mirror/account-and-auth/erc-2612) | Native witness scopes | Final | Signatures |
| [ERC-4337 — Account Abstraction](/standards-mirror/account-and-auth/erc-4337) | Native NEP-30 verify | Final | Smart Accounts |
| [EIP-712 — Typed Structured Data Signing](/standards-mirror/account-and-auth/eip-712) | Native witness model | Final | Signatures |
| [EIP-191 — Signed Data Prefix](/standards-mirror/account-and-auth/eip-191) | Native witness model | Final | Signatures |
| [ERC-5267 — EIP-712 Domain Retrieval](/standards-mirror/account-and-auth/erc-5267) | Native | Final | Signatures |
| [ERC-6492 — Signatures for Pre-deployed Contracts](/standards-mirror/account-and-auth/erc-6492) | Native (no counterfactual deploy) | Final | Signatures |
| [EIP-7702 — Set Code for EOAs](/standards-mirror/account-and-auth/eip-7702) | Native (every account is a contract) | Final (Pectra) | Smart Accounts |
| [EIP-3074 — AUTH and AUTHCALL](/standards-mirror/account-and-auth/eip-3074) | Native (witness scopes) | Stagnant (superseded by 7702) | Smart Accounts |

## Related Pages

- [Standards Mirror Overview](/standards-mirror/)
- [Semantic Standards Mapping](/mapping/standards)
- [Standards and Contracts](/additional-material/neo-standards)
