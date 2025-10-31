# Neo Solidity Compiler Architecture

This document captures the technical plan for delivering a production-grade
Solidity → NeoVM toolchain. It pinpoints the core subsystems, the libraries we
intend to reuse, and the order in which they will be implemented.

## High-Level Pipeline

```
Solidity source
    │
    ▼
Frontend (Solang)
    │  - Parse Solidity → AST/IR
    │  - Run builtin Solidity diagnostics
    ▼
Canonical IR Layer (Yul-like)
    │  - Normalise control flow & data types
    │  - Provide stable input for optimisations
    ▼
Semantic Analysis
    │  - Symbol table & scope resolution
    │  - Type checking, storage layout validation
    │  - Runtime safety diagnostics (re-entrancy, permissions) [stretch]
    ▼
Optimiser Pipeline
    │  - Constant folding, DCE, inlining
    │  - Neo-specific passes (stack depth, syscall hoisting)
    ▼
Code Generator
    │  - Lower IR blocks to NeoVM bytecode
    │  - Allocate locals/arguments, manage evaluation stack
    │  - Encode syscalls, storage ops, inter-contract calls
    ▼
Artifact Builder
    │  - NEF writer (tokens, method table, checksum)
    │  - Manifest writer (ABI, permissions, standards)
    │  - Debug info (source map, sequence points)
    ▼
Deployment Outputs (.nef, .manifest.json, optional debug bundle)
```

## Key Components and Responsibilities

### 1. Frontend Integration
- **Dependency:** [`solang`](https://github.com/hyperledger-labs/solang) Rust
  crate for Solidity parsing and semantic diagnostics.
- **Responsibility:** translate Solidity into Solang's intermediate
  representation (IR) or Yul AST while preserving metadata (source map,
  types, modifiers).
- **Deliverables:** `frontend` crate/module exposing a structured IR plus
  diagnostic reporting that plugs into our compiler driver.

### 2. Canonical IR Layer
- **Format:** custom, inspired by Yul with Neo-specific extensions (explicit
  stack effects, storage annotations, syscall nodes).
- **Goals:**
  - Normalise Solang output (control flow graphs, explicit temporaries).
  - Provide deterministic serialisation for testing.
  - Serve as the input to optimisation and codegen stages.

### 3. Semantic Analysis
- **Scopes & Symbols:** build symbol tables for contracts, functions, events,
  state variables.
- **Type System:** enforce Solidity type rules, Neo limitations (e.g., struct
  size, supported numeric widths), storage location correctness.
- **Diagnostics:** surface errors/warnings with source locations; produce
  machine-readable diagnostics for tooling.

### 4. Optimiser Pipeline
- **Pass Ordering:**
  1. Constant Folding / Propagation
  2. Dead Code Elimination
  3. Function Inlining (bounded by heuristics)
  4. Stack Height Reduction (peephole rewriting)
  5. NeoVM-Specific cleanups (e.g., merging adjacent PUSH operations)
- **Infrastructure:** generic pass manager with change tracking, statistics,
  and optional debug dumps.

### 5. NeoVM Code Generator
- **Responsibilities:**
  - Map IR instructions to opcodes while maintaining stack discipline.
  - Manage call frames (arguments, locals, returns) using `INITSLOT` /
    `RET` semantics.
  - Emit syscalls via interop IDs; support native contract calls and triggers.
  - Handle control flow (labels, forward jumps) with fixups.
- **Outputs:** script byte vector, per-method offsets, gas estimates, debug
  symbols (source map, variable info).

### 6. Artifact Builder
- **NEF:** build method token table, include manifest hash, compute CRC32,
  support optional `nef.debug.json` emission.
- **Manifest:** derive ABI from analysed symbols, populate `supportedstandards`,
  `permissions`, `features`, and `extra` metadata.
- **Debug:** optional PDB-like JSON for the debugger (breakpoints, locals).

### 7. DevPack & Runtime Bindings
- Replace placeholder Solidity libraries (`Syscalls.sol`, `NativeCalls.sol`)
  with code that matches Neo's ABI:
  - Use proper interop IDs and parameter marshalling.
  - Support `CallFlags`, iterator handling, oracle callbacks, etc.
- Provide Rust-side helpers for inserting runtime support stubs (e.g.,
  storage contexts, iterator wrappers).

### 8. Testing & Validation
- **Unit Tests:** per module—frontend parsing, semantic rules, optimiser passes.
- **Integration Tests:** compile canonical contracts (NEP-17, NEP-11, oracle
  samples) and check bytecode/manifest snapshots.
- **VM Regression:** execute generated scripts inside Neo VM runner (via
  `neo-vm-rs` or `NeoExpress`) to verify behaviour.
- **CI:** GitHub Actions matrix covering `cargo fmt`, `clippy`, unit, integration,
  and VM tests.

## Implementation Order
1. Finalise architecture (this document) ✔️
2. Integrate Solang frontend (parsing + basic diagnostic bridging).
3. Define canonical IR structs and converters from Solang output.
4. Implement semantic analyser atop the IR.
5. Rework optimiser to operate on the new IR (reusing current scaffolding
   where possible).
6. Build the NeoVM code generator and replace stub emitter in `src/main.rs`.
7. Expand artifact builder to emit full NEF/Manifest tied to generated code.
8. Replace devpack libraries with spec-accurate bindings.
9. Stand up comprehensive tests/CI, including VM execution harness.

## Runtime Metadata Overrides
The runtime exposes an `ExecutionOverrides` helper alongside
`NeoRuntime::execute_with_overrides`. Tests or embedding applications can supply
per-execution block height, timestamp, and calling-script hash without mutating
global state. After each run the overrides are cleared and the context falls
back to the deterministic defaults from `RuntimeConfig`, which keeps the CLI
and other integrations predictable while still allowing VM-level simulations of
chain metadata. `ExecutionResult` now carries an `ExecutionMetadata` payload so
callers can inspect the effective values that were in play for a given
invocation.

## Tooling & Dependencies
- `solang` crate (GPLv3) — confirm license compatibility or consider invoking
  external solc with JSON outputs if required.
- `neo-vm` crate for executing bytecode in tests.
- `serde`/`serde_json` for manifest & debug info serialisation.
- `crc32fast` (already integrated) for NEF checksum.
- `anyhow`/`thiserror` for diagnostics.

## Open Questions
- **Licensing:** decide whether embedding Solang (GPL) is acceptable for the
  project; if not, plan to invoke `solc` externally and parse its JSON IR.
- **Storage Layout:** align Solidity storage semantics with Neo's key-value
  storage model—may require additional lowering steps.
- **Gas Model:** determine whether to estimate NeoVM gas during codegen or rely
  on external tooling.
- **Debug Info:** confirm format expected by Neo debugger for source maps and
  breakpoints.

## Next Steps
Following this architecture baseline, the next milestone is to integrate the
Solidity frontend so that real contracts can be parsed into a structured IR:
- Add a `frontend` module/crate wrapping Solang.
- Translate Solang AST/IR into the canonical IR defined above.
- Surface compiler diagnostics with line/column info through the CLI.

Subsequent milestones will build upon that foundation as outlined in the plan.
