# Getting Started

This section helps you go from source code to Neo N3 contract artifacts quickly and correctly.

## Prerequisites

- Rust toolchain (`stable`)
- Node.js 18+ for tooling/docs tasks
- .NET SDK (optional, for C# runtime and some docs generation)
- Neo-Express (optional for local deploy testing)

## Output Model

`neo-solc` generates:

1. `<name>.nef`: NeoVM executable script.
2. `<name>.manifest.json`: ABI + permissions + standards + metadata.

These artifacts are used directly for deployment.

## Recommended First Path

1. [Installation](/getting-started/installation)
2. [Quick Start](/getting-started/quickstart)
3. [Compile Workflow](/workflows/compile)
4. [Deploy Workflow](/workflows/deploy)
5. [Test Workflow](/workflows/test)

## Safety Defaults to Adopt Early

For production-oriented workflows, use strict manifest controls:

```bash
neo-solc MyContract.sol \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/MyContract
```

Then validate with:

```bash
make production-gate
```

## Support and Feature Boundaries

Solidity support is broad but not identical to EVM behavior for all features.

- For detailed status: [Solidity Feature Support](/solidity/feature-support)
- For precise mapping: [EVM to NeoVM Mapping](/mapping/evm-to-neovm)
- For runtime fidelity gaps: [Parity and Limitations](/reference/parity-limitations)
