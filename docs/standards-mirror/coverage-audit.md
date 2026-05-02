---
title: Standards Mirror — Coverage Audit & Gap Report
description: Cross-reference of STANDARDS_MAPPING.md against the mirror catalog, plus a prioritized list of meaningful Ethereum standards not yet mirrored.
---

# Standards Mirror — Coverage Audit & Gap Report

This page answers the two recurring questions about the mirror:

1. **Audit** — does every standard mentioned in
   [`devpack/standards/STANDARDS_MAPPING.md`](https://github.com/r3e-network/neo-devpack-solidity/blob/main/devpack/standards/STANDARDS_MAPPING.md)
   have a corresponding mirror page?
2. **Gap report** — which Ethereum-side standards are notable enough to be
   worth mirroring but have not been added yet?

For the live deploy state of every mirrored standard, see the
[Coverage Matrix](./coverage-matrix).

## 1. STANDARDS_MAPPING.md Audit

The devpack mapping doc references the following ERCs / EIPs in its quick
reference table. Each row below shows whether a dedicated mirror page exists.

| Mapping Doc Row | Mirror Page |
| --- | --- |
| ERC-20 ↔ NEP-17 | ✅ [tokens/erc-20](./tokens/erc-20) |
| ERC-721 ↔ NEP-11 | ✅ [tokens/erc-721](./tokens/erc-721) |
| ERC-2981 ↔ NEP-24 | ✅ [tokens/erc-2981](./tokens/erc-2981) |
| ERC-1155 — Multi-Token | ✅ [tokens/erc-1155](./tokens/erc-1155) |
| EIP-165 ↔ Manifest | ✅ [infrastructure/erc-165](./infrastructure/erc-165) |
| EIP-712 ↔ Witness model | ✅ [account-and-auth/eip-712](./account-and-auth/eip-712) |
| EIP-2612 (Permit) | ✅ [account-and-auth/erc-2612](./account-and-auth/erc-2612) |
| EIP-1967 (Proxy) | ✅ [infrastructure/erc-1967](./infrastructure/erc-1967) |
| ERC-721 Receiver ↔ NEP-26 | ✅ Documented inline on [tokens/erc-721](./tokens/erc-721); no dedicated page (it's a callback contract, not an ERC) |
| ERC-677 / ERC-1363 hooks ↔ NEP-27 | ✅ [tokens/erc-1363](./tokens/erc-1363) |

**Audit result:** every ERC/EIP row in the quick-reference table now has a
mirror page. The Neo-side callback NEPs (NEP-22, NEP-26, NEP-27, NEP-29,
NEP-30, NEP-31) are described inline in the relevant ERC pages and in
[`additional-material/neo-standards/`](/additional-material/neo-standards),
which is the right place for the Neo-native side of the mirror.

### Suggested follow-ups

- **Standalone NEP pages.** `additional-material/neo-standards/` ships
  [NEP-11](/additional-material/neo-standards/nep-11-non-fungible-tokens),
  [NEP-17](/additional-material/neo-standards/nep-17-fungible-tokens), and
  [NEP-24](/additional-material/neo-standards/nep-24-royalty-standard) but no
  dedicated docs for NEP-22 (update), NEP-26 / NEP-27 (payment callbacks),
  NEP-29 (deploy lifecycle), NEP-30 (verify), or NEP-31 (destroy). These are
  referenced by many ERC mirror pages and warrant first-class explainers.
- **Mapping doc consolidation.** `STANDARDS_MAPPING.md` lives under
  `devpack/standards/` and predates the mirror's category split. Most of its
  prose is now duplicated in the per-ERC mirror pages. Either (a) convert it
  to a one-page index that links into the mirror, or (b) keep it as the
  canonical machine-readable mapping table and remove the long-form prose.

## 2. Gap Report — Notable Final ERCs Not Mirrored

The Ethereum EIPs index lists ~140 ERCs at status **Final**. The mirror
covers the standards with the highest application-level relevance for Neo
migration — token primitives, accounts, and infrastructure. The list below
is the curated set of meaningful gaps, grouped by priority.

### Recently closed (now mirrored)

Ten consecutive priority passes have closed fifty meaningful
Final / Review / Last-Call / Draft-status ERCs. Included for the audit trail:

| Standard | Mirror Page | Why it mattered |
| --- | --- | --- |
| ERC-1167 | [infrastructure/erc-1167](./infrastructure/erc-1167) | Single most-used proxy pattern; underpins every Ethereum factory |
| ERC-3448 | [infrastructure/erc-3448](./infrastructure/erc-3448) | Init-supporting MetaProxy variant of ERC-1167 |
| ERC-3643 | [tokens/erc-3643](./tokens/erc-3643) | Compliance framework for tokenised securities (T-REX) |
| ERC-3668 | [infrastructure/erc-3668](./infrastructure/erc-3668) | Off-chain data via CCIP Read; mirrors to native Oracle |
| ERC-4361 | [account-and-auth/erc-4361](./account-and-auth/erc-4361) | De-facto web3 sign-in (SIWE) |
| ERC-4907 | [tokens/erc-4907](./tokens/erc-4907) | Rental NFT user-role extension; gaming + scholarship use cases |
| ERC-5202 | [infrastructure/erc-5202](./infrastructure/erc-5202) | Blueprint contract format; completes the "deploy from data" family |
| ERC-5313 | [account-and-auth/erc-5313](./account-and-auth/erc-5313) | Light Contract Ownership; view-only convention |
| ERC-5564 | [account-and-auth/erc-5564](./account-and-auth/erc-5564) | Stealth Addresses; privacy primitive on secp256r1 |
| ERC-6066 | [account-and-auth/erc-6066](./account-and-auth/erc-6066) | NFT-aware signature validation (ERC-1271 per tokenId) |
| ERC-6093 | [tokens/erc-6093](./tokens/erc-6093) | Standardised error vocabulary for NEP-17 / NEP-11 ports |
| ERC-7528 | [tokens/erc-7528](./tokens/erc-7528) | Native asset address convention; relevant for any DEX or vault |
| ERC-7535 | [defi/erc-7535](./defi/erc-7535) | Native asset version of ERC-4626 vaults |
| ERC-7656 | [account-and-auth/erc-7656](./account-and-auth/erc-7656) | Generalisation of ERC-6551 to any contract |
| ERC-7786 | [infrastructure/erc-7786](./infrastructure/erc-7786) | Cross-chain messaging gateway interface |
| ERC-7944 | [defi/erc-7944](./defi/erc-7944) | Async cancellation for ERC-7540 vaults |
| ERC-8042 | [infrastructure/erc-8042](./infrastructure/erc-8042) | Diamond storage convention for ERC-2535 |
| ERC-5679 | [tokens/erc-5679](./tokens/erc-5679) | Standardised mint/burn surface across token types |
| ERC-2135 | [tokens/erc-2135](./tokens/erc-2135) | Consumable interface (tickets, permits, vouchers) |
| ERC-7160 | [tokens/erc-7160](./tokens/erc-7160) | ERC-721 multi-metadata extension |
| ERC-3475 | [defi/erc-3475](./defi/erc-3475) | Abstract storage bonds (multi-class, multi-nonce) |
| ERC-7092 | [defi/erc-7092](./defi/erc-7092) | Financial bonds (issuer-facing surface) |
| ERC-6982 | [tokens/erc-6982](./tokens/erc-6982) | Default lockable tokens (lighter than 5192/6147) |
| ERC-7677 | [account-and-auth/erc-7677](./account-and-auth/erc-7677) | Paymaster web service capability (AA handshake) |
| ERC-7144 | [tokens/erc-7144](./tokens/erc-7144) | ERC-20 with transaction validation step (lightweight T-REX) |
| ERC-7943 | [tokens/erc-7943](./tokens/erc-7943) | Universal Real World Asset Interface (uRWA) |
| ERC-5006 | [tokens/erc-5006](./tokens/erc-5006) | Rental NFT user extension for ERC-1155 multi-tokens |
| ERC-2767 | [account-and-auth/erc-2767](./account-and-auth/erc-2767) | Contract Ownership Governance |
| ERC-7758 | [account-and-auth/erc-7758](./account-and-auth/erc-7758) | Modern Transfer-With-Authorization |
| ERC-5169 | [tokens/erc-5169](./tokens/erc-5169) | Client Script URI for token contracts |
| ERC-5375 | [tokens/erc-5375](./tokens/erc-5375) | NFT Author Information and Consent |
| ERC-5023 | [tokens/erc-5023](./tokens/erc-5023) | Shareable Non-Fungible Token (multi-holder) |
| ERC-7066 | [tokens/erc-7066](./tokens/erc-7066) | Approval-based NFT Lock Extension |
| ERC-7432 | [tokens/erc-7432](./tokens/erc-7432) | NFT Roles (time-bounded grants) |
| ERC-6105 | [tokens/erc-6105](./tokens/erc-6105) | No-Intermediary NFT Trading Protocol |
| ERC-5615 | [tokens/erc-5615](./tokens/erc-5615) | ERC-1155 Supply Extension |
| ERC-5773 | [tokens/erc-5773](./tokens/erc-5773) | Context-Dependent Multi-Asset Tokens |
| ERC-6059 | [tokens/erc-6059](./tokens/erc-6059) | Parent-Governed Nestable NFTs |
| ERC-4519 | [tokens/erc-4519](./tokens/erc-4519) | NFTs Tied to Physical Assets |
| ERC-5570 | [tokens/erc-5570](./tokens/erc-5570) | Digital Receipt NFT |
| ERC-6150 | [tokens/erc-6150](./tokens/erc-6150) | Hierarchical NFTs |
| ERC-6220 | [tokens/erc-6220](./tokens/erc-6220) | Composable NFTs Equippable Parts |
| ERC-5380 | [tokens/erc-5380](./tokens/erc-5380) | ERC-721 Entitlement Extension |
| ERC-5489 | [tokens/erc-5489](./tokens/erc-5489) | NFT Hyperlink Extension |
| ERC-6672 | [tokens/erc-6672](./tokens/erc-6672) | Multi-Redeemable NFTs |
| ERC-7634 | [tokens/erc-7634](./tokens/erc-7634) | Limited Transfer Count NFT |
| ERC-7007 | [tokens/erc-7007](./tokens/erc-7007) | Verifiable AI-Generated Content Token |
| ERC-5725 | [tokens/erc-5725](./tokens/erc-5725) | Transferable Vesting NFT |
| ERC-6454 | [tokens/erc-6454](./tokens/erc-6454) | Minimal Transferable NFT detection |
| ERC-7715 | [account-and-auth/erc-7715](./account-and-auth/erc-7715) | Permission Grants for Smart Accounts (AA) |

### High priority (next-up)

Ten batches closed; the remaining "high priority" set covers refunds,
state-fingerprinting, signed-claim attestations, and a few NFT
extensions that round out the catalog:

| Standard | Title | Status | Why it matters |
| --- | --- | --- | --- |
| **ERC-5507** | Refundable Tokens | Final | Refund mechanism for both fungible and NFT tokens; consumer-protection primitive. |
| **ERC-5646** | Token State Fingerprint | Final | Verify complete token state at specific block heights; useful for indexers / auditors. |
| **ERC-5521** | Referable NFT | Final | Referral relationships between NFTs; growing relevance for influencer / referral economies. |
| **ERC-5528** | Refundable Fungible Token | Final | ERC-20 supporting asset recovery and refunds; complements ERC-5507. |
| **ERC-7437** | Multi-redeemable NFT discounts | Final | Per-merchant discount stacking, complements ERC-6672. |

### Medium priority

| Standard | Title | Status | Notes |
| --- | --- | --- | --- |
| ERC-7053 | Interoperable Digital Media Indexing | Final | Cross-platform media discovery. |
| ERC-7405 | Modular Wallet Interface | Draft | Pairs with ERC-7579 mirror. |
| ERC-7585 | Permitted Authentication Scheme | Draft | Generalised SIWE for non-Ethereum chains. |
| ERC-6093 | Already mirrored ✓ | — | — |
| ERC-7144 | Already mirrored ✓ | — | — |
| ERC-7758 | Already mirrored ✓ | — | — |

### Watch list (Review / Last Call)

These are not yet Final but are widely deployed or actively shipped:

| Standard | Title | Status | Notes |
| --- | --- | --- | --- |
| **ERC-7758** | Transfer With Authorization (modern) | Review | Successor to ERC-3009 (which we mirror). Add when promoted to Final or when adoption shifts. |
| ERC-7715 | Permission grants for accounts | Draft | Account-abstraction permission model. Track for ERC-4337 / ERC-7579 integration. |
| ERC-7943 | Universal Real World Asset Interface | Last Call | RWA standardisation. Track and add when Final. |
| ERC-7786 | Cross-Chain Messaging Gateway | Final | Cross-chain interop. Worth mirroring once the bridge story for Neo is more concrete. |

### Intentionally out of scope

Some ERCs are not worth mirroring because they describe Ethereum-specific
infrastructure with no meaningful Neo equivalent:

- **EIP-2333 / 2334 / 2335** (BLS12-381 key derivation) — Beacon-chain specific.
- **EIP-4844** (Blob transactions) — Already explained in `protocol-eips`.
- **ERC-820 / ERC-1820** — We mirror 1820 (the live one); 820 is the
  superseded predecessor.
- Most NFT-extension drafts under `4400-7900` that are single-use community
  proposals without traction — track in the watch list, add only on demand.

## How to Add a New Mirror Page

1. Pick the right category directory under
   `docs/standards-mirror/{tokens,defi,infrastructure,protocol-eips,account-and-auth}/`.
2. Copy an existing mirror page as a template (recommended: `tokens/erc-20.md`
   for a deployed example, `tokens/erc-6909.md` for a catalog-only example).
3. Fill in `<StandardEntry>` props (`id`, `title`, `eip`, `status`,
   `neoMapping`, `category`, `parityLabel`, `parityClass`).
4. Add the row to the category index page (e.g. `tokens.md`) and bump the
   "X standards" count in the prose.
5. Update the main [Standards Mirror Overview](./) — bump category counts
   and the live-on-TestNet headline if you also deploy a pair.
6. Add to the [Coverage Matrix](./coverage-matrix) and (if catalog-only)
   to the explanatory list in `deployments/DEFERRED.md`.
7. Run `npm run docs:check` to validate links.

## Related

- [Standards Mirror Overview](./)
- [Coverage Matrix](./coverage-matrix)
- [Latest TestNet Results](./deployments/RESULTS)
- [Deferred Deployment Queue](./deployments/DEFERRED)
- [`devpack/standards/STANDARDS_MAPPING.md`](https://github.com/r3e-network/neo-devpack-solidity/blob/main/devpack/standards/STANDARDS_MAPPING.md)
