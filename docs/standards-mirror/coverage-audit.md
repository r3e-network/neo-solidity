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

### High priority

| Standard | Title | Status | Why it matters |
| --- | --- | --- | --- |
| **ERC-1167** | Minimal Proxy Contract (EIP-1167 Clones) | Final | Single most-used proxy pattern in Ethereum. Underpins TBA registries, factories, deterministic deploys. Neo equivalent: parameterised manifest deploy via `ContractManagement.Deploy`. |
| **ERC-4361** | Sign-In with Ethereum (SIWE) | Final | The de-facto web3 authentication flow. Maps to Neo signer + tx witness. Worth a dedicated mirror because it's how dApps onboard users. |
| **ERC-3668** | CCIP Read — Off-chain Data Retrieval | Final | The standard pattern for verifiably reading off-chain data inside contracts. Neo's oracle service is a direct replacement; explainer would clarify the model swap. |
| **ERC-7528** | ETH (Native Asset) Address Convention | Final | Standardised representation of the native token (`0xEEEE…EEEE`). Neo equivalent: `NativeContract.NEO.Hash` / `NativeContract.GAS.Hash`. Important for any DeFi protocol indexing native vs ERC-20. |
| **ERC-7656** | Generalized Contract-Linked Services | Final | Generalisation of ERC-6551 — registry for arbitrary services bound to any contract, not just NFTs. Worth mirroring alongside ERC-6551. |

### Medium priority

| Standard | Title | Status | Notes |
| --- | --- | --- | --- |
| ERC-3448 | MetaProxy Standard | Final | Init-supporting variant of ERC-1167. Mirror together with ERC-1167. |
| ERC-3643 | T-REX — Regulated Token | Final | Compliance framework. Niche but important for tokenised securities; growing relevance. |
| ERC-4907 | Rental NFT (User Role Extension) | Final | NFT temporary delegation; popular in gaming. Pairs well with ERC-6551. |
| ERC-5202 | Blueprint Contract Format | Final | Pattern for storing contract code as data. Neo equivalent: NEF blob storage + `ContractManagement.Deploy`. |
| ERC-5313 | Light Contract Ownership | Final | Lighter alternative to ERC-173. |
| ERC-5564 | Stealth Addresses | Final | Privacy-preserving recipient addresses. Neo has primitives that could mirror; useful for analysts. |
| ERC-6066 | NFT Signature Validation | Final | NFT-aware ERC-1271; used by NFT marketplaces. |
| ERC-6093 | Custom errors for tokens | Final | Standardised error definitions. |
| ERC-7535 | Native Asset ERC-4626 Vault | Final | ETH version of ERC-4626. Should be mirrored alongside the existing ERC-4626 page. |

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
