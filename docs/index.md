---
layout: home

title: Neo Solidity Documentation
titleTemplate: Solidity to Neo N3

hero:
  name: "Neo Solidity"
  text: "Solidity to Neo N3 Compiler + Devpack"
  tagline: "Compile Solidity contracts into production-ready NeoVM artifacts (.nef + .manifest.json), with explicit EVM→Neo mapping and deploy/test workflows."
  actions:
    - theme: brand
      text: Start Here
      link: /getting-started/overview
    - theme: alt
      text: Compile and Deploy Guide
      link: /workflows/compile

features:
  - title: Compiler + Runtime Semantics
    details: The documentation tracks actual compiler behavior, diagnostics, manifest generation, and NeoVM execution semantics.

  - title: End-to-End Operational Guides
    details: Complete workflows for compiling, deploying, and testing contracts on Neo N3 and Neo-Express with production readiness checks.

  - title: Solidity to NeoVM Mapping
    details: Clear, explicit mapping of Solidity syntax/features to NeoVM functionality, including warnings where EVM semantics differ.

  - title: Devpack and Standards
    details: NEP-17/NEP-11 and companion libraries are documented with strict-safe examples and manifest permission expectations.
---

## What This Project Is

Neo Solidity is a production-grade compiler and toolchain that translates Solidity contracts to Neo N3 artifacts:

- `.nef` executable bytecode for NeoVM
- `.manifest.json` contract manifest (ABI, permissions, standards, metadata)

The compiler is designed for practical deployability and auditability:

- strict manifest permission controls
- explicit compatibility diagnostics for EVM-only behavior
- Neo-Express smoke suites and production-gate verification

## Documentation Scope

This site is organized to support both contract developers and compiler integrators:

1. **Getting Started**: installation and first compilation.
2. **Workflows**: compile, deploy, test, and production hardening.
3. **Solidity on Neo**: feature support and syntax behavior.
4. **EVM→NeoVM Mapping**: semantic mapping and limitations.
5. **NeoVM Integration**: native contracts, syscalls, and runtime behavior.
6. **Manifest + Devpack**: standards, permissions, and metadata.
7. **Reference**: CLI, errors, architecture, runtime spec, parity notes.

## Quick Links

- [Getting Started](/getting-started/overview)
- [Compile Contracts](/workflows/compile)
- [Deploy Contracts](/workflows/deploy)
- [Test Contracts](/workflows/test)
- [Solidity Feature Support](/solidity/feature-support)
- [EVM to NeoVM Mapping](/mapping/evm-to-neovm)
- [Manifest System](/manifests/manifest-spec)
- [Devpack Overview](/devpack/overview)
