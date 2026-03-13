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
          link: /basics/introduction-to-smart-contracts
        - theme: alt
          text: Repository
          link: https://github.com/r3e-network/neo-solidity
---

<div class="vp-doc home-section">

<div class="solidity-keywords-cloud">
  <code>pragma</code> <code>contract</code> <code>function</code> <code>modifier</code> <code>event</code> <code>struct</code> <code>enum</code> <code>require</code> <code>address</code>
</div>

<br/>

<div class="alert-box">
  <div class="alert-content">
    <h3>Neo Solidity v0.13.1</h3>
    <p>Version 0.13.1 brings seamless EVM-to-NeoVM compatibility, softening strict compilation rejections for EVM-specific syntax (like <code>assembly</code> blocks and extraneous call options) into graceful warnings, enabling frictionless porting of Ethereum contracts. It also fixes critical dataflow paths for infinite loop prevention.</p>
    <a href="https://github.com/r3e-network/neo-solidity/releases" target="_blank">Read the full release notes &rarr;</a>
  </div>
</div>

<br/>

<h2 class="solidity-section-header">NEO SOLIDITY IS EVOLVING RAPIDLY</h2>

Our release cycle prioritizes both stability for production and rapid innovation for developers transitioning from EVM to Neo N3. We regularly ship **non-breaking minor releases** with enhanced mappings, deeper standard support (like NEP-17/NEP-11), and expanded diagnostic intelligence. 

<div style="margin-top: 1.5rem;">
  <a href="/basics/introduction-to-smart-contracts" class="cta-button">Get started</a>
</div>

<hr class="solidity-hr" />

<h2 class="solidity-section-header">CONTRIBUTE TO NEO SOLIDITY</h2>

Neo Solidity is an open-source project. We welcome developers, auditors, and technical writers to shape the future of smart contract development on Neo N3.

<div class="contribute-grid">
  <div class="contribute-card">
    <h4>Reporting issues and vulnerabilities</h4>
    <p>Found a bug or an EVM feature that doesn't map correctly? Let us know on GitHub so we can fix it.</p>
    <a href="https://github.com/r3e-network/neo-solidity/issues">Report an issue &rarr;</a>
  </div>
  <div class="contribute-card">
    <h4>Translating the documentation</h4>
    <p>Help us write tutorials, translate pages, or expand our EVM-to-NeoVM mapping guides.</p>
    <a href="https://github.com/r3e-network/neo-solidity/tree/main/docs">Edit Docs &rarr;</a>
  </div>
  <div class="contribute-card">
    <h4>Fixing and responding to issues</h4>
    <p>Looking for a place to start? Check our repository for issues tagged with <code>good first issue</code>.</p>
    <a href="https://github.com/r3e-network/neo-solidity/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22">Browse issues &rarr;</a>
  </div>
  <div class="contribute-card">
    <h4>Contributing to language design</h4>
    <p>Propose new intrinsics or discuss how EVM paradigms should translate to NeoVM.</p>
    <a href="https://github.com/r3e-network/neo-solidity/discussions">Join Discussions &rarr;</a>
  </div>
</div>

<div style="margin-top: 1.5rem; text-align: center;">
  <a href="/resources/contributing" class="cta-button">Start contributing</a>
</div>

<hr class="solidity-hr" />

<h2 class="solidity-section-header" style="text-align: left;">PLAYGROUND</h2>

<p>Try Neo Solidity for yourself. See how standard Ethereum paradigms translate effortlessly to Neo N3 artifacts.</p>

<div class="playground-mockup">
  <div class="playground-sidebar">
    <div class="tab active">SimpleStorage.sol</div>
    <div class="tab">ERC20Token.sol</div>
    <div class="tab">SimpleAuction.sol</div>
  </div>
  <div class="playground-editor">

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Runtime} from "@neo/Runtime.sol";

contract SimpleStorage {
    uint256 private storedData;
    address public owner;

    constructor() {
        owner = Runtime.getCallingScriptHash(); 
    }

    function set(uint256 x) public {
        require(Runtime.checkWitness(owner), "Unauthorized");
        storedData = x;
    }

    function get() public view returns (uint256) {
        return storedData;
    }
}
```

  </div>
  <div class="playground-output">
    <div class="output-tab active">Compiler result</div>
    <div class="output-tab">Deployment costs</div>
    <div class="output-tab">Bytecode (NEF)</div>
    <div class="output-tab">Manifest</div>
    
    <div class="output-content">
      <strong>Compiler version:</strong> neo-solc 0.13.1<br/><br/>
      ✅ Compilation successful.<br/><br/>
      <strong>Methods:</strong><br/>
      - _deploy(any, bool)<br/>
      - set(uint256)<br/>
      - get() (safe)<br/><br/>
      <strong>Permissions:</strong> Restricted (No wildcards)
    </div>
  </div>
</div>

<hr class="solidity-hr" />

<h2 class="solidity-section-header">NEO SOLIDITY EVENTS</h2>

<h3 style="font-size: 1rem; color: var(--vp-c-text-2); letter-spacing: 0.1em; text-transform: uppercase;">UPCOMING EVENTS</h3>

<p>No upcoming events currently scheduled. Check back later!</p>

</div>