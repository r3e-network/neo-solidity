---
title: ERC / EIP ↔ Neo Standards Mirror
description: Every meaningful Ethereum standard, mirrored to its Neo N3 equivalent — with side-by-side Solidity and Neo C# implementations.
outline: false
---

# ERC / EIP ↔ Neo Standards Mirror

Every Ethereum standard that matters has a Neo N3 counterpart — sometimes a one-to-one
NEP, sometimes a stronger native primitive that makes the EIP unnecessary. This module
catalogs them all: pick a category below, then flip between the **ERC/EIP detail**, the
**Solidity implementation**, and the equivalent **Neo C# implementation**.

The takeaway: anything you can build on Ethereum, you can build on Neo N3 — often with
fewer footguns and lower gas.

## Categories

<div class="mirror-cat-grid">

  <a class="mirror-cat" href="./tokens">
    <div class="cat-eyebrow">Token Standards</div>
    <div class="cat-title">Fungibles, NFTs, Multi-token</div>
    <div class="cat-desc">
      ERC-20, 721, 777, 1155, 2981, 3525, 4906, 4494, 5192, 5484, 6909, 2309
    </div>
    <div class="cat-pill-row">
      <span class="cat-pill cat-pill-direct">12 standards</span>
      <span class="cat-pill cat-pill-neo">NEP-11 / NEP-17 / NEP-24</span>
    </div>
  </a>

  <a class="mirror-cat" href="./account-and-auth">
    <div class="cat-eyebrow">Account & Authentication</div>
    <div class="cat-title">Ownership, signatures, smart accounts</div>
    <div class="cat-desc">
      ERC-173, 1271, 2612, 4337, 5267, 6492, EIP-712, EIP-191, EIP-7702, EIP-3074
    </div>
    <div class="cat-pill-row">
      <span class="cat-pill cat-pill-direct">10 standards</span>
      <span class="cat-pill cat-pill-native">Native witness scopes</span>
    </div>
  </a>

  <a class="mirror-cat" href="./infrastructure">
    <div class="cat-eyebrow">Infrastructure & Patterns</div>
    <div class="cat-title">Proxies, registries, modular contracts</div>
    <div class="cat-desc">
      ERC-165, 1014, 1056, 1820, 1967, 2470, 2535, 2771, 7201, 7579
    </div>
    <div class="cat-pill-row">
      <span class="cat-pill cat-pill-direct">10 standards</span>
      <span class="cat-pill cat-pill-neo">NEP-22 native update</span>
    </div>
  </a>

  <a class="mirror-cat" href="./defi">
    <div class="cat-eyebrow">DeFi Building Blocks</div>
    <div class="cat-title">Vaults, flash loans, governance</div>
    <div class="cat-desc">
      ERC-4626, 3156, 7540, 7575, 5805, 6372, 7818
    </div>
    <div class="cat-pill-row">
      <span class="cat-pill cat-pill-direct">7 standards</span>
      <span class="cat-pill cat-pill-pattern">Composition patterns</span>
    </div>
  </a>

  <a class="mirror-cat" href="./protocol-eips">
    <div class="cat-eyebrow">Protocol-Level EIPs</div>
    <div class="cat-title">Things Ethereum had to fix at the protocol</div>
    <div class="cat-desc">
      EIP-1559, 2718, 2930, 3198, 3855, 3860, 4844, 1153, 6780, 2098
    </div>
    <div class="cat-pill-row">
      <span class="cat-pill cat-pill-direct">10 EIPs</span>
      <span class="cat-pill cat-pill-native">Most are no-ops on Neo</span>
    </div>
  </a>

</div>

## Live on TestNet

**Eighteen standards** in this mirror are deployed and behavior-verified on Neo N3
TestNet (network magic `894710606`) in both implementations side-by-side. The same
invocation matrix runs against the Solidity (compiled with `neo-solc`) and the Neo
C# (compiled with `nccs`) versions, asserting equivalent return values.

| Standard | Solidity | Neo C# |
|---|---|---|
| **ERC-20** ↔ NEP-17 | [d76434af…f96](https://dora.coz.io/contract/neo3/testnet/0xd76434af829dc4c936c12648aa77932fa94c0f96) | [1f3a9b41…b43a](https://dora.coz.io/contract/neo3/testnet/0x1f3a9b414de1c60434543dd8a05ac5e08b75b43a) |
| **ERC-721** ↔ NEP-11 | [48b5f8f5…7aa](https://dora.coz.io/contract/neo3/testnet/0x48b5f8f579810b402fed660844145fed406f77aa) | [15c664d5…baf](https://dora.coz.io/contract/neo3/testnet/0x15c664d51340a102490dbf5dec5647f541775baf) |
| **ERC-1155** Multi-Token | [f1d7867c…317](https://dora.coz.io/contract/neo3/testnet/0xf1d7867c140a016333b69d3e1795b0ee224d0317) | [ef019e6f…6bd](https://dora.coz.io/contract/neo3/testnet/0xef019e6feb75fd331149cb7c9c3ddfcaa8ba86bd) |
| **ERC-2981** ↔ NEP-24 | [ade57dfd…234](https://dora.coz.io/contract/neo3/testnet/0xade57dfd9ad85fff8dca3845cf22206346468234) | [bf3fe7eb…5e1](https://dora.coz.io/contract/neo3/testnet/0xbf3fe7eb875750c81c2915d53123c380685a65e1) |
| **ERC-3525** Bond | [d0fd56da…2c6](https://dora.coz.io/contract/neo3/testnet/0xd0fd56dad510d54ca7877bab2c578d63b82a52c6) | [fcfde62a…6c](https://dora.coz.io/contract/neo3/testnet/0xfcfde62a4764cbcd9b35615084e0075c4bddba6c) |
| **ERC-5192** Soulbound | [1b75ecb9…034](https://dora.coz.io/contract/neo3/testnet/0x1b75ecb9e926203e66283e3f875ba5097f3c3034) | [7081fcf3…2b](https://dora.coz.io/contract/neo3/testnet/0x7081fcf36db56a716b416ef553829ed23c07da2b) |
| **ERC-7818** Expirable | [fcaaf98f…64d](https://dora.coz.io/contract/neo3/testnet/0xfcaaf98f8c4693b326f883d52db9d9e4a8c6564d) | [cb1b0441…6e0](https://dora.coz.io/contract/neo3/testnet/0xcb1b0441c5b02a2f7de348951c6bf8e2a6ae56e0) |
| **ERC-173** Ownable | [19977aea…be4](https://dora.coz.io/contract/neo3/testnet/0x19977aea6f158de3844f3261988b17381156bbe4) | [ce89aec2…459](https://dora.coz.io/contract/neo3/testnet/0xce89aec2e79b121ec264231be49cd96111824459) |
| **ERC-1271** MultiSig | [88eec008…de7](https://dora.coz.io/contract/neo3/testnet/0x88eec008aaeb09d10ce68f93f6d98efbe92b9de7) | [88079ecd…682](https://dora.coz.io/contract/neo3/testnet/0x88079ecdd4af98cf932c25c80c0bb218a8cfb682) |
| **ERC-1820** Registry | [02704624…ee4](https://dora.coz.io/contract/neo3/testnet/0x02704624615747bdcc7994a6be347be42ad52ee4) | [8f36ff27…59d](https://dora.coz.io/contract/neo3/testnet/0x8f36ff27ef6564209956c05a4b886c0c99cec59d) |
| **ERC-1056** DID | [dd6d4a48…f50](https://dora.coz.io/contract/neo3/testnet/0xdd6d4a4806445d04982afc68866c9dc92ef41f50) | [d13806f6…b30](https://dora.coz.io/contract/neo3/testnet/0xd13806f6c06854ad3d8b731aebee40f8b74c1b30) |
| **ERC-1967** Upgradeable | [48f6d58a…245](https://dora.coz.io/contract/neo3/testnet/0x48f6d58aa74ad1d507cb2eb07242e033bfdbd245) | [096f01e4…976](https://dora.coz.io/contract/neo3/testnet/0x096f01e40f7cf9cea4304195cc2ab6bb481be976) |
| **ERC-2535** Diamond | [26b6f333…527](https://dora.coz.io/contract/neo3/testnet/0x26b6f333b18bffd00702348b1cec5b55cf79f527) | [1b3c602c…dcf](https://dora.coz.io/contract/neo3/testnet/0x1b3c602c1a208238f981125e2ad3045734c5bdcf) |
| **ERC-2771** Forwarder | [6653a8da…ed9](https://dora.coz.io/contract/neo3/testnet/0x6653a8da9bac7b622987670d97bf740c2c124ed9) | [1463ad54…0a](https://dora.coz.io/contract/neo3/testnet/0x1463ad54cf6a8fc7c0ffe3740ad1cf04a6280c0a) |
| **ERC-3156** FlashLender | [b7d5cd14…7ec](https://dora.coz.io/contract/neo3/testnet/0xb7d5cd146852006f8bc5d8c1621852c9117d37ec) | [a82c8142…84b](https://dora.coz.io/contract/neo3/testnet/0xa82c8142c02ec0cf748bbaa57819f9c61440984b) |
| **ERC-4626** Vault | [faf678fd…0f](https://dora.coz.io/contract/neo3/testnet/0xfaf678fdb2053a279cf79f14a3623f3f1f9f810f) | [0e515ad2…0c5](https://dora.coz.io/contract/neo3/testnet/0x0e515ad2e892180273ab017a4883084e647740c5) |
| **ERC-5805** Voting | [b87fa58c…110](https://dora.coz.io/contract/neo3/testnet/0xb87fa58c80deef8dc910a0ca3a2cc186035f3110) | [1d33818b…692](https://dora.coz.io/contract/neo3/testnet/0x1d33818b3d053d291424848ed1ac7ebaa3243692) |
| **ERC-6372** Clock | [e3c55758…42b](https://dora.coz.io/contract/neo3/testnet/0xe3c55758861ba8034c9f3d223ed93cf5df77442b) | [eb454a6b…335](https://dora.coz.io/contract/neo3/testnet/0xeb454a6b6e102b2700fc1d3b18d58b861ed6c335) |

**Total: 60 / 69 cross-implementation assertions pass.** The 9 non-blocking
divergences are stale-state mismatches (NFT cumulative mints, ERC-20 faucet
balances accumulating across runs) — every fresh deploy passes 100%.

### What about the other 33 entries?

All 33 are queued for a recurring agent (the `Standards Mirror — Add 2 Pairs Every
Monday` routine) which pops 2 entries each Monday from
[`deployments/DEFERRED.md`](https://github.com/r3e-network/neo-solidity/blob/main/docs/standards-mirror/deployments/DEFERRED.md).
The queue currently holds **23 entries**:

| Category | Count | What the agent will deploy |
|---|---|---|
| Standalone-deployable | 11 | ERC-777 (token w/ hooks), 7540 (async vault), 7575 (multi-asset vault), 7579 (modular account), 4337 (smart account), 6492 (sig pre-deploy), 5267 (EIP-712 domain), 5114 / 5484 / 6147 (SBT variants), 2470 (singleton factory) |
| Extension-as-demo | 5 | Small standalone demos for ERC-2309 (batch mint), 4906 (mutable metadata), 4494 (NFT permit), 2612 (token permit), 1014 (CREATE2-style factory) |
| Pattern-as-demo | 2 | ERC-165 supportsInterface wrapper, ERC-7201 namespaced storage demo |
| Protocol-EIP-as-demo | 5 | EIP-712 verifier, EIP-191 verifier, EIP-3198 fee-introspection, EIP-1153 reentrancy-guard pattern, EIP-2098 compact-sig verifier |

At 2 pairs/week, the queue empties in ~12 weeks (~mid-July 2026). After that, every
catalog entry will have a live testnet contract pair, except for the **8 protocol
EIPs whose only sensible demonstration is the protocol behavior itself** —
EIP-1559 fee market, EIP-2718 typed-tx envelope, EIP-2930 access lists, EIP-3855
PUSH0, EIP-3860 initcode size, EIP-4844 blobs, EIP-6780 selfdestruct nerf, EIP-7702
set-code-for-EOAs. These get prose-only entries because there's nothing to deploy.

Source pairs, deploy script, full results JSON, and instructions to reproduce live
under
[`docs/standards-mirror/deployments/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments).

## How to Read This

Each entry shows three views in a tabbed nav bar:

| Tab | What it shows |
| --- | --- |
| **ERC / EIP Detail** | The standard's purpose, interface, motivation, and the corresponding Neo mechanism. |
| **Solidity Implementation** | A reference Solidity implementation as deployed on Ethereum. |
| **Neo C# Implementation** | The idiomatic Neo equivalent — either a direct NEP, a composition pattern, or an explanation of why the EIP is a no-op on Neo because the protocol already covers it. |

## How To Read The Pills

Each entry carries a parity pill that summarizes the mirror relationship:

| Pill | Meaning |
| --- | --- |
| **NEP-XX** (green) | Direct NEP equivalent. Same interface shape, sometimes simpler. |
| **Native** (blue) | Subsumed by Neo's protocol primitives. No NEP or contract needed. |
| **Pattern** (orange) | Composition pattern in the Neo devpack — no dedicated NEP, but a clear recipe. |

## Module Internals

Each category page registers its entries with a shared `<StandardsMirror>` Vue component
that drives the master/detail UI. All content is plain Markdown — VitePress's bundled
Shiki highlights every Solidity and C# block, and the search index covers everything.

To add another standard, append a `<StandardEntry>` block to the appropriate category
page. The page automatically picks it up.

For deeper reference on the Neo standards behind the mirrors, see
[Standards and Contracts](/additional-material/neo-standards) and
[`devpack/standards/STANDARDS_MAPPING.md`](https://github.com/r3e-network/neo-solidity/blob/main/devpack/standards/STANDARDS_MAPPING.md).

<style scoped>
.mirror-cat-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 1rem;
  margin: 2rem 0 3rem;
}

.mirror-cat {
  display: flex;
  flex-direction: column;
  gap: 0.45rem;
  padding: 1.2rem 1.25rem 1.1rem;
  border: 1px solid color-mix(in srgb, var(--vp-c-brand-1) 18%, transparent);
  border-radius: 12px;
  background: color-mix(in srgb, var(--vp-c-brand-1) 3%, var(--vp-c-bg-soft));
  text-decoration: none !important;
  color: var(--vp-c-text-1);
  transition:
    transform 0.18s ease,
    border-color 0.18s ease,
    box-shadow 0.18s ease,
    background 0.18s ease;
}

.mirror-cat:hover {
  transform: translateY(-2px);
  border-color: var(--vp-c-brand-1);
  background: color-mix(in srgb, var(--vp-c-brand-1) 7%, var(--vp-c-bg-soft));
  box-shadow: 0 4px 14px color-mix(in srgb, var(--vp-c-brand-1) 16%, transparent);
}

.cat-eyebrow {
  font-family: var(--vp-font-family-mono);
  font-size: 0.72rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--vp-c-brand-1);
}

.cat-title {
  font-family: 'Space Grotesk', var(--vp-font-family-base);
  font-size: 1.15rem;
  font-weight: 700;
  letter-spacing: -0.01em;
  line-height: 1.3;
}

.cat-desc {
  font-family: var(--vp-font-family-mono);
  font-size: 0.78rem;
  color: var(--vp-c-text-2);
  line-height: 1.5;
}

.cat-pill-row {
  display: flex;
  flex-wrap: wrap;
  gap: 0.4rem;
  margin-top: 0.4rem;
}

.cat-pill {
  font-family: var(--vp-font-family-mono);
  font-size: 0.66rem;
  padding: 0.15rem 0.5rem;
  border-radius: 999px;
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.cat-pill-direct {
  background: color-mix(in srgb, var(--vp-c-brand-1) 16%, transparent);
  color: var(--vp-c-brand-1);
}

.cat-pill-neo {
  background: color-mix(in srgb, var(--vp-c-brand-1) 10%, transparent);
  color: var(--vp-c-brand-1);
  border: 1px solid color-mix(in srgb, var(--vp-c-brand-1) 30%, transparent);
}

.cat-pill-native {
  background: color-mix(in srgb, #6ea8ff 16%, transparent);
  color: #4d8bff;
}

.dark .cat-pill-native {
  color: #87b3ff;
}

.cat-pill-pattern {
  background: color-mix(in srgb, #ffb14e 16%, transparent);
  color: #c97a14;
}

.dark .cat-pill-pattern {
  color: #ffc874;
}
</style>
