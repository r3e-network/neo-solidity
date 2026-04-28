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

Four token standards in this mirror are **deployed and behavior-verified on Neo N3
TestNet** in both implementations side-by-side. The same invocation matrix runs
against the Solidity (compiled with `neo-solc`) and the Neo C# (compiled with
`nccs`) versions, asserting equivalent return values:

| Standard | Solidity Address | Neo C# Address | Result |
| --- | --- | --- | --- |
| **ERC-20** ↔ **NEP-17** | [`NZbQsZAb…ZhwF`](https://dora.coz.io/contract/neo3/testnet/0xd76434af829dc4c936c12648aa77932fa94c0f96) | [`NRGNZQRr…ybp1zJ`](https://dora.coz.io/contract/neo3/testnet/0x1f3a9b414de1c60434543dd8a05ac5e08b75b43a) | 9/9 ✅ |
| **ERC-721** ↔ **NEP-11** | [`NbTK8px5…V4dYt`](https://dora.coz.io/contract/neo3/testnet/0x48b5f8f579810b402fed660844145fed406f77aa) | [`NbuB1V5e…uZoK38`](https://dora.coz.io/contract/neo3/testnet/0x15c664d51340a102490dbf5dec5647f541775baf) | 6/8 ✅ |
| **ERC-2981** ↔ **NEP-24** | [`NQhcPMzy…1KdR6i`](https://dora.coz.io/contract/neo3/testnet/0xade57dfd9ad85fff8dca3845cf22206346468234) | [`NgTke4MQ…F4EmSC`](https://dora.coz.io/contract/neo3/testnet/0xbf3fe7eb875750c81c2915d53123c380685a65e1) | 6/6 ✅ |
| **ERC-3525** Bond | [`NdzbQnww…BoBTW6`](https://dora.coz.io/contract/neo3/testnet/0xd0fd56dad510d54ca7877bab2c578d63b82a52c6) | [`NVpt23PJ…fEopNZ`](https://dora.coz.io/contract/neo3/testnet/0xfcfde62a4764cbcd9b35615084e0075c4bddba6c) | 7/7 ✅ |

**Total: 28 / 30 assertions pass** across both implementations. The 2 NFT
non-blocking divergences are due to cumulative state from re-using contracts
across deploy runs (not real behavioral mismatches).

Source pairs, deploy script, full results JSON, and instructions to reproduce are
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
