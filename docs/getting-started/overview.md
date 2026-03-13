# Overview

Neo Solidity is a production-ready compiler that translates Solidity 0.8.x smart contracts into Neo N3 blockchain artifacts. It lets Ethereum developers bring their existing Solidity skills and codebases to the Neo ecosystem without learning a new language.

## Why Neo Solidity?

Neo N3 is a high-performance blockchain with a unique virtual machine (NeoVM), native assets, and a dBFT consensus model. Historically, writing Neo smart contracts required C#, Python, or Go. Neo Solidity bridges the gap by accepting the most widely-used smart contract language and producing deployment-ready Neo N3 output.

Key motivations:

- **Developer reach** -- Solidity has the largest smart contract developer community. Accepting Solidity source removes the biggest adoption barrier for Neo N3.
- **Code reuse** -- Existing Solidity contracts (tokens, DeFi, governance) can be ported to Neo with minimal changes.
- **Tooling compatibility** -- The compiler supports Solidity's standard JSON interface, enabling integration with Hardhat, Foundry, and other established toolchains.

## What It Produces

Every successful compilation emits two files:

| Artifact               | Description                                                                                                                                                                                                                     |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `<name>.nef`           | **Neo Executable Format** -- contains the NeoVM bytecode, method token table, and a SHA-256 checksum. This is the on-chain script that the Neo virtual machine executes.                                                        |
| `<name>.manifest.json` | **Contract manifest** -- declares the ABI (methods, events, parameters), required permissions, supported standards (NEP-17, NEP-11, etc.), trust settings, and extra metadata. Neo nodes validate this manifest at deploy time. |

Together, these two files are everything you need to deploy a contract via `neo-cli`, Neo-Express, or any Neo N3 SDK.

## Architecture

The compiler is implemented in Rust and follows an 8-stage pipeline:

```
Solidity Source (*.sol)
       |
       v
  1. Frontend          -- Parse Solidity via solang-parser crate
       |
       v
  2. Metadata          -- Extract contracts, functions, events, NatSpec
       |
       v
  3. Semantic Model    -- Symbol table, scope resolution, type checking
       |
       v
  4. IR Generation     -- Yul-like IR with Neo-specific extensions
       |
       v
  5. Optimizer         -- Constant folding, DCE, inlining, peephole (O0-O3)
       |
       v
  6. Code Generator    -- Map IR to NeoVM opcodes, manage call frames
       |
       v
  7. Artifact Builder  -- Write NEF binary and manifest JSON
       |
       v
  8. Output            -- .nef + .manifest.json on disk
```

Each stage is a distinct module in the Rust codebase, making the compiler easy to test and extend independently.

### Stage Details

**Frontend** -- Uses the `solang-parser` crate to parse Solidity 0.8.x source into an AST. Produces source-location-aware diagnostics for syntax errors.

**Metadata Extraction** -- Walks the AST to collect `ContractMetadata`, `FunctionMetadata`, `EventMetadata`, `NatspecDoc`, and state variable definitions. This stage also detects NEP standard compliance (NEP-17, NEP-11).

**Semantic Model** -- Builds a symbol table with scope resolution, validates types, checks storage layout, resolves inheritance (C3 linearization), and expands modifiers.

**IR Generation** -- Lowers the semantic model into a custom intermediate representation inspired by Yul. The IR includes Neo-specific nodes for syscalls, storage operations, and native contract calls.

**Optimizer** -- Applies configurable optimization passes at four levels:

| Level | Passes                                                              |
| ----- | ------------------------------------------------------------------- |
| `-O0` | No optimization (debug builds)                                      |
| `-O1` | Dead code elimination, constant folding/propagation                 |
| `-O2` | + function inlining (bounded), peephole optimizations               |
| `-O3` | + stack height reduction, NeoVM-specific cleanups, max optimization |

**Code Generator** -- Translates IR into NeoVM bytecode. Manages `INITSLOT`/`RET` call frames, emits syscalls via interop IDs, resolves jump targets, and allocates locals and arguments.

**Artifact Builder** -- Serializes the bytecode into NEF format (magic bytes, compiler metadata, method token table, SHA-256 checksum) and generates the manifest JSON with ABI, permissions, and standards declarations.

## Solidity Feature Coverage

The compiler supports 114 of 142 audited Solidity features (80%):

| Category              | Status                                                     |
| --------------------- | ---------------------------------------------------------- |
| Fully supported       | 114 features (80%)                                         |
| Partially supported   | 20 features (14%) -- with Neo-specific workarounds         |
| Unsupported           | 3 features (2%) -- also unimplemented in mainline Solidity |
| Intentionally blocked | 5 features (4%) -- EVM-only concepts with Neo alternatives |

Blocked features produce compile-time errors with actionable messages pointing to the Neo equivalent. For example, `address.delegatecall(...)` directs you to use explicit cross-contract call patterns.

For the full matrix, see [Solidity Feature Support](/solidity/feature-support).

## Target Audience

- **Ethereum/Solidity developers** who want to deploy on Neo N3 without learning a new language.
- **Neo developers** who prefer Solidity over C# or Python for contract development.
- **DeFi teams** porting existing protocols (AMMs, lending, vesting, governance) to Neo.
- **Tooling authors** building IDE plugins, CI pipelines, or deployment frameworks for Neo.

## Project Maturity

Neo Solidity is production-ready for most use cases:

- **700+ tests** across unit, integration, E2E compilation, and conformance suites.
- **95% compiler completion** with comprehensive Solidity 0.8.x coverage.
- **32 conformance test vectors** with a 93.8% pass rate against reference Neo implementations.
- **16+ Neo-Express smoke tests** that deploy and invoke real contracts on a local chain.
- **Production gate** (`make production-gate`) that validates formatting, linting, release build, full tests, strict compilation sweeps, and deployment smoke suites in a single command.

::: warning Recommendation
For mainnet deployment, always test your contracts thoroughly on Neo N3 TestNet first. Review the [Parity and Limitations](/reference/parity-limitations) page for known behavioral differences between EVM and NeoVM.
:::

## Key Capabilities

- **Full CLI** with 20+ options for compilation, optimization, manifest hardening, and diagnostics.
- **Standard JSON mode** for integration with Solidity-compatible toolchains.
- **CALLT optimization** for more efficient native contract calls on Neo N3.
- **Manifest permission hardening** with `--deny-wildcard-contracts`, `--deny-wildcard-methods`, and explicit permission allowlists.
- **NatSpec manifest overrides** via `@custom:neo.manifest.*` tags in Solidity source.
- **Structured diagnostics** with `--json-errors` and `--json-warnings` for CI integration.
- **Devpack libraries** providing Solidity interfaces for Neo N3 syscalls, native contracts, and NEP standards.
- **Import resolution** with `-I` include paths for multi-file projects.
- **Batch compilation** for compiling entire directories of contracts.

## Project Structure

```
neo-solidity/
├── src/                    # Rust compiler source
│   ├── main.rs             # CLI entry point (neo-solc binary)
│   ├── frontend.rs         # Solang parser integration
│   ├── solidity.rs         # Metadata extraction
│   ├── semantic_model.rs   # Semantic analysis
│   ├── ir/                 # Intermediate representation
│   ├── optimizer.rs        # Optimization passes
│   ├── cli/bytecode/       # NeoVM code generation
│   ├── neo.rs              # NEF and manifest builders
│   └── runtime/            # Embedded NeoVM runtime for testing
├── devpack/                # Solidity libraries for Neo N3
│   ├── contracts/          # Contract interfaces
│   ├── libraries/          # Utility libraries
│   └── standards/          # NEP-17, NEP-11, etc.
├── examples/               # Example contracts and smoke test scripts
├── tests/                  # Test suites (unit, e2e, conformance)
├── tooling/                # TypeScript packages (Hardhat, Foundry, etc.)
└── docs/                   # This documentation site
```

## Next Steps

1. [Installation](/getting-started/installation) -- Build the compiler from source.
2. [Quick Start](/getting-started/quickstart) -- Compile and inspect your first contract.
3. [Compile Workflow](/workflows/compile) -- Full CLI reference and compilation patterns.
4. [Deploy Workflow](/workflows/deploy) -- Deploy contracts to Neo-Express and TestNet.
5. [Test Workflow](/workflows/test) -- Run the test suites and set up CI.
