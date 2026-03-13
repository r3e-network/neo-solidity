---
layout: home

title: Neo Solidity
titleTemplate: Compile Solidity to Neo N3

hero:
    name: "{NEO_SOLIDITY}"
    text: "Compile Solidity to Neo N3"
    tagline: "A statically-typed curly-braces programming language compiler designed for developing smart contracts that run on the Neo N3 blockchain."
    image:
        src: /assets/neo-solidity-logo.png
        alt: Neo Solidity Compiler
    actions:
        - theme: brand
          text: Read the docs
          link: /getting-started/overview
        - theme: alt
          text: Repository
          link: https://github.com/r3e-network/neo-solidity

features:
    - icon: "\U0001F527"
      title: Seamless EVM-to-Neo Mapping
      details: "Transparent semantic mapping of EVM globals (`msg.sender`, `tx.origin`), opcodes, and patterns to Neo N3 equivalents. Most EVM contracts compile with zero code changes."
      link: /language-description/units-and-global-variables
      linkText: Mapping reference

    - icon: "\U0001F4CB"
      title: "Solidity 0.8.x Support"
      details: "142 audited features: 114 fully supported, 23 partial with Neo solutions, 1 unsupported, 4 intentionally blocked with actionable diagnostics."
      link: /solidity/feature-support
      linkText: Feature matrix

    - icon: "\U0001F4E6"
      title: Neo Native Standards
      details: "First-class support for NEP-17, NEP-11, and NEP-24 tokens. Built-in Runtime, Storage, NativeCalls, and Syscalls intrinsic libraries."
      link: /devpack/overview
      linkText: Devpack docs
---

<div class="vp-doc home-section">

<div class="solidity-keywords-cloud">
  <code>pragma</code> <code>contract</code> <code>modifier</code> <code>event</code> <code>address</code> <code>NativeCalls.neoTransfer</code> <code>Storage.put</code>
</div>

<br/>

<div class="alert-box">
  <div class="alert-content">
    <h3>Latest Release: Neo Solidity v0.13.1</h3>
    <p>Version 0.13.1 brings seamless EVM-to-NeoVM compatibility, softening strict compilation rejections for EVM-specific syntax (like <code>assembly</code> blocks and extraneous call options) into graceful warnings, enabling frictionless porting of Ethereum contracts. It also fixes critical dataflow paths for infinite loop prevention.</p>
    <a href="https://github.com/r3e-network/neo-solidity/releases" target="_blank">Read the full release notes &rarr;</a>
  </div>
</div>

<br/>

## Neo Solidity is Evolving Rapidly

Our release cycle prioritizes both stability for production and rapid innovation for developers transitioning from EVM to Neo N3. We regularly ship **non-breaking minor releases** with enhanced mappings, deeper standard support (like NEP-17/NEP-11), and expanded diagnostic intelligence. 

[**Get started with Neo Solidity**](/basics/introduction-to-smart-contracts) or review our [Architecture](/internals/architecture) to see how the 8-stage compiler works.

<hr class="solidity-hr" />

## Contribute to Neo Solidity

Neo Solidity is an open-source project. We welcome developers, auditors, and technical writers to shape the future of smart contract development on Neo N3.

<div class="contribute-grid">
  <div class="contribute-card">
    <h4>Report Issues</h4>
    <p>Found a bug or an EVM feature that doesn't map correctly? Let us know on GitHub so we can fix it.</p>
    <a href="https://github.com/r3e-network/neo-solidity/issues">Report a bug &rarr;</a>
  </div>
  <div class="contribute-card">
    <h4>Improve Documentation</h4>
    <p>Help us write tutorials, translate pages, or expand our EVM-to-NeoVM mapping guides.</p>
    <a href="https://github.com/r3e-network/neo-solidity/tree/main/docs">Edit Docs &rarr;</a>
  </div>
  <div class="contribute-card">
    <h4>Fix Issues</h4>
    <p>Looking for a place to start? Check our repository for issues tagged with <code>good first issue</code>.</p>
    <a href="https://github.com/r3e-network/neo-solidity/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22">Browse issues &rarr;</a>
  </div>
  <div class="contribute-card">
    <h4>Language Design</h4>
    <p>Propose new intrinsics or discuss how EVM paradigms should translate to NeoVM.</p>
    <a href="https://github.com/r3e-network/neo-solidity/discussions">Join Discussions &rarr;</a>
  </div>
</div>

<br/>

<div class="action-cta-box">
  <h3>Start building for Neo N3 today</h3>
  <a href="/compiler/analysing-the-compiler-output" class="cta-button">Compile & Deploy Contracts</a>
</div>

<hr class="solidity-hr" />

## Quick Code: Simple NEP-17 Storage

Neo Solidity lets you use standard Solidity syntax, automatically translating types, operations, and globals to their Neo N3 runtime equivalents:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import {NativeCalls} from "@neo/NativeCalls.sol";
import {Runtime} from "@neo/Runtime.sol";

contract SimpleStorage {
    uint256 private storedData;
    address public owner;

    event DataChanged(address indexed setter, uint256 newValue);

    constructor() {
        owner = Runtime.getCallingScriptHash(); // Native NeoVM intrinsic
    }

    function set(uint256 x) public {
        require(Runtime.checkWitness(owner), "Unauthorized");
        storedData = x;
        emit DataChanged(owner, x);
    }

    function get() public view returns (uint256) {
        return storedData;
    }
}
```

</div>