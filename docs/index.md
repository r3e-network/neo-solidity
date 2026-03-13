---
layout: home

title: Neo Solidity
titleTemplate: Compile Solidity to Neo N3

hero:
    name: "Neo Solidity"
    text: "Compile Solidity to Neo N3"
    tagline: "Production-grade compiler that transforms Solidity 0.8.x contracts into deployable NeoVM artifacts (.nef + .manifest.json) with full EVM-to-Neo semantic mapping, permission hardening, and deploy/test workflows."
    image:
        src: /assets/neo-solidity-logo.png
        alt: Neo Solidity Compiler
    actions:
        - theme: brand
          text: Get Started
          link: /getting-started/overview
        - theme: alt
          text: Compile & Deploy
          link: /workflows/compile
        - theme: alt
          text: GitHub
          link: https://github.com/r3e-network/neo-solidity

features:
    - icon: "\U0001F527"
      title: Compiler Pipeline
      details: "8-stage Solidity-to-NeoVM compilation: lexing, parsing, semantic analysis, IR lowering, optimization (O0\u2013O3), code generation, manifest emission, and NEF packaging."
      link: /reference/architecture
      linkText: Architecture docs

    - icon: "\U0001F4CB"
      title: "Solidity 0.8.x"
      details: "142 audited features -- 114 fully supported, 20 partial with Neo solutions, 3 unsupported, 5 intentionally blocked with actionable diagnostics."
      link: /solidity/feature-support
      linkText: Feature matrix

    - icon: "\U0001F504"
      title: "EVM \u2192 NeoVM Mapping"
      details: "Transparent semantic mapping of EVM globals, opcodes, and patterns to Neo N3 equivalents. 9 auto-mapped features compile with warnings, zero code changes."
      link: /mapping/evm-to-neovm
      linkText: Mapping reference

    - icon: "\U0001F4E6"
      title: Devpack & Standards
      details: "NEP-17, NEP-11, and NEP-24 token standards with strict-safe examples. Runtime, Storage, NativeCalls, and Syscalls intrinsic libraries included."
      link: /devpack/overview
      linkText: Devpack docs

    - icon: "\U0001F512"
      title: Manifest Security
      details: "Permission hardening with --deny-wildcard-permissions, --deny-wildcard-contracts, and --deny-wildcard-methods. NatSpec-driven manifest overrides for groups, trusts, and extras."
      link: /manifests/manifest-spec
      linkText: Manifest spec

    - icon: "\U0001F9EA"
      title: Testing & Deployment
      details: "700+ Rust tests plus maintained tooling and optional runtime test slices. Neo-Express smoke tests, constructor arg validation, and a one-command production-gate for CI."
      link: /workflows/test
      linkText: Testing guide
---

<div class="vp-doc home-section">

## What is Neo Solidity?

Neo Solidity is a **Rust-based compiler and toolchain** that translates Solidity smart contracts into Neo N3 artifacts. It produces two files per contract:

- **`.nef`** -- NeoVM executable bytecode, ready for on-chain deployment
- **`.manifest.json`** -- contract manifest with ABI, permissions, supported standards, and metadata

The compiler is designed for **practical deployability and auditability**. It enforces strict manifest permission controls, emits explicit compatibility diagnostics where EVM semantics diverge from Neo, and integrates with Neo-Express for local smoke testing before mainnet deployment.

Unlike transpilation approaches that attempt to emulate EVM behavior on NeoVM, Neo Solidity performs **native semantic mapping** -- translating Solidity constructs directly to their Neo N3 equivalents while preserving developer intent.

<p align="center">
  <img src="/assets/evm-neovm-mapping.png" alt="EVM to NeoVM Mapping" width="80%">
</p>

## Quick Stats

<div class="stats-grid">
  <div class="stat-card">
    <span class="stat-value">114 / 142</span>
    <span class="stat-label">Supported Features (80%)</span>
  </div>
  <div class="stat-card">
    <span class="stat-value">700+</span>
    <span class="stat-label">Test Cases</span>
  </div>
  <div class="stat-card">
    <span class="stat-value">v0.13.1</span>
    <span class="stat-label">Latest Release</span>
  </div>
  <div class="stat-card">
    <span class="stat-value">8</span>
    <span class="stat-label">Pipeline Stages</span>
  </div>
</div>

## Compiler Architecture

The compilation pipeline processes Solidity source through eight stages to produce deployment-ready Neo N3 artifacts:

<div class="pipeline">
  <span class="stage">.sol Source</span>
  <span class="arrow">&rarr;</span>
  <span class="stage">Lexer</span>
  <span class="arrow">&rarr;</span>
  <span class="stage">Parser</span>
  <span class="arrow">&rarr;</span>
  <span class="stage">Semantic Analysis</span>
  <span class="arrow">&rarr;</span>
  <span class="stage">IR Lowering</span>
  <span class="arrow">&rarr;</span>
  <span class="stage">Optimizer</span>
  <span class="arrow">&rarr;</span>
  <span class="stage">CodeGen</span>
  <span class="arrow">&rarr;</span>
  <span class="stage">.nef + .manifest.json</span>
</div>

Each stage produces diagnostics. The optimizer supports four levels (O0--O3), and the code generator emits both NeoVM bytecode and the contract manifest in a single pass.

## Quick Start

```bash
# Install from source
git clone https://github.com/r3e-network/neo-solidity.git
cd neo-solidity && cargo install --path .

# Compile a contract
neo-solc contract.sol -O2 -o contract

# Deploy to Neo-Express (local)
neoxp contract deploy contract.nef contract.manifest.json

# Run production gate (format + lint + build + tests + deploy smokes)
make production-gate
```

## Quick Links

<div class="links-grid">
  <a href="/getting-started/overview">Getting Started</a>
  <a href="/workflows/compile">Compile Contracts</a>
  <a href="/workflows/deploy">Deploy Contracts</a>
  <a href="/workflows/test">Test Contracts</a>
  <a href="/solidity/original-contracts/">Original Famous Contracts</a>
  <a href="/solidity/feature-support">Solidity Feature Support</a>
  <a href="/mapping/evm-to-neovm">EVM to NeoVM Mapping</a>
  <a href="/manifests/manifest-spec">Manifest System</a>
  <a href="/devpack/overview">Devpack Overview</a>
  <a href="/reference/cli">CLI Reference</a>
  <a href="/reference/architecture">Architecture</a>
</div>

</div>
