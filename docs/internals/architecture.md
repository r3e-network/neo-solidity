# Architecture

Complete reference for the Neo Solidity compiler architecture, covering the 8-stage compilation pipeline, optimizer design, module organization, and extension points.

## Overview

The Neo Solidity compiler (`neo-solc`) is a Rust-based toolchain that translates Solidity 0.8.x smart contracts into NeoVM bytecode. The output is a `.nef` (Neo Executable Format) file and a `.manifest.json` deployment manifest, ready for deployment to the Neo N3 blockchain.

The compiler is built as a single Rust binary with no external runtime dependencies. It uses the `solang-parser` crate for Solidity parsing and `clap` for CLI argument handling.

## 8-Stage Compiler Pipeline

```
Solidity source (*.sol)
    |
    v
1. Frontend (solang-parser)        -- Parse Solidity to AST
    |
    v
2. Metadata Extraction              -- Extract contract/function/event metadata
    |
    v
3. Semantic Analysis                -- Type checking, scope resolution
    |
    v
4. Semantic Model                   -- Build typed intermediate representation
    |
    v
5. IR Generation                    -- Lower to Yul-like IR with Neo extensions
    |
    v
6. Optimizer                        -- Multi-level optimization passes
    |
    v
7. Code Generation                  -- IR to NeoVM bytecode
    |
    v
8. Artifact Builder                 -- Emit NEF + manifest
    |
    v
Output (.nef, .manifest.json)
```

### Stage 1: Frontend (solang-parser)

**Module:** `src/frontend.rs`

The frontend uses the `solang-parser` crate to parse Solidity source code into an AST (Abstract Syntax Tree).

Responsibilities:

- Parse Solidity 0.8.x syntax into AST nodes
- Extract type information from the parse tree
- Map source locations for diagnostic reporting
- Report syntax errors with file/line/column context

Key types from `solang-parser`:

- `ContractDefinition` -- top-level contract structure
- `FunctionDefinition` -- function declarations
- `Statement`, `Expression` -- code body elements

Errors at this stage produce `E1xxx` parse error codes.

### Stage 2: Metadata Extraction

**Module:** `src/solidity.rs`

Extracts structured metadata from the parsed AST for use by later stages.

Extracted metadata includes:

- `ContractMetadata` -- complete contract-level information (name, inheritance, state variables)
- `FunctionMetadata` -- function signatures, visibility, modifiers, return types
- `EventDefinition` -- event names and parameter types
- `NatspecDoc` -- documentation comments (`@notice`, `@param`, `@return`)

This metadata drives manifest generation (ABI, supported standards) and informs the semantic analysis stage.

### Stage 3: Semantic Analysis

**Module:** `src/semantic.rs`

Performs type checking and validation on the parsed AST using the extracted metadata.

Responsibilities:

- Symbol table construction and scope resolution
- Type checking and type inference
- Storage layout validation (Neo-specific constraints)
- Visibility and access control verification
- Detection of unsupported EVM features

Errors at this stage produce `E2xxx` semantic error codes. Security warnings (`E5xxx`) are also emitted here.

### Stage 4: Semantic Model

**Module:** `src/semantic_model.rs`, `src/type_system/`

Constructs a fully typed intermediate representation from the validated AST. This is the bridge between the Solidity-specific frontend and the target-independent IR.

The semantic model resolves:

- All type references to concrete types
- Function overload resolution
- Inheritance linearization
- Storage slot assignments

### Stage 5: IR Generation

**Module:** `src/ir/`

Lowers the semantic model into a custom IR (Intermediate Representation) inspired by Yul with Neo-specific extensions.

Key IR components:

- `ir_types.rs` -- IR type definitions and instruction enum
- `context/` -- lowering context (symbol tables, scope tracking)
- `expressions/` -- expression lowering (arithmetic, calls, storage access)
- `statements/` -- statement lowering (control flow, assignments, returns)

The IR instruction set includes:

```rust
pub enum Instruction {
    PushLiteral(LiteralValue),
    Call(FunctionCall),
    Syscall(SyscallName),
    StorageOp(StorageOperation),
    // ... additional variants
}
```

IR nodes carry stack effect annotations and storage access metadata that the optimizer and code generator use for correctness verification.

Errors at this stage produce `E3xxx` codegen error codes (specifically `E3001 UnsupportedFeature` for EVM-only constructs).

### Stage 6: Optimizer

**Module:** `src/optimizer.rs`, `src/optimizer/`

A multi-level optimization pipeline that transforms the IR to reduce bytecode size and improve execution efficiency.

#### Optimization Levels

| Level | Passes                                                                                       | Description                                                  |
| ----- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| `-O0` | None                                                                                         | No optimization. IR is emitted directly. Best for debugging. |
| `-O1` | Constant folding                                                                             | Evaluate constant expressions at compile time.               |
| `-O2` | Constant folding, dead code elimination                                                      | Also remove unreachable code after returns. Default level.   |
| `-O3` | Constant folding, dead code elimination, function inlining, common subexpression elimination | Maximum optimization. Smallest bytecode.                     |

#### Optimization Passes

**Constant Folding** (`src/optimizer/constant_folding.rs`)

Evaluates expressions with known constant operands at compile time. For example, `2 + 3` becomes `5` in the IR, eliminating runtime computation.

**Dead Code Elimination** (`src/optimizer/dead_code.rs`)

Removes unreachable code paths:

- Code after unconditional `return` statements
- Branches that can never be taken (when the condition is a compile-time constant)
- Unused internal functions (when no call site exists)

**Function Inlining** (`src/optimizer/inlining.rs`)

Replaces function call sites with the function body for small functions. The inline threshold is 50 AST nodes by default. Inlining eliminates `CALL`/`RET` overhead (512 gas per call) at the cost of larger bytecode.

**Common Subexpression Elimination** (`src/optimizer/cse.rs`)

Identifies repeated computations and replaces them with a single computation stored in a local variable.

::: info
Additional passes exist in the optimizer directory (`strength.rs`, `loops.rs`, `gas.rs`) for strength reduction, loop optimizations, and gas-aware transformations. These are wired into the dispatch logic at appropriate optimization levels.
:::

#### Optimization Statistics

The optimizer tracks metrics for each compilation:

- `eliminated_instructions` -- instructions removed by DCE
- `inlined_functions` -- functions inlined
- `folded_constants` -- constant expressions evaluated
- `nodes_before` / `nodes_after` -- AST size reduction
- `reduction_percent` -- overall size reduction percentage

Use `-v` (verbose) to see these statistics in the compiler output.

### Stage 7: Code Generation

**Module:** `src/cli/bytecode/`

Translates the optimized IR into NeoVM bytecode.

Responsibilities:

- Map IR instructions to NeoVM opcodes
- Manage call frames (`INITSLOT` for locals/arguments, `RET` for returns)
- Emit syscall invocations via 4-byte interop IDs
- Resolve jump targets and compute offsets
- Allocate local and argument slots
- Handle control flow (labels, conditional/unconditional jumps)

The code generator also handles the `--callt` flag, emitting `CALLT` instructions with method tokens for native contract calls instead of `SYSCALL`-based dispatch.

### Stage 8: Artifact Builder

**Module:** `src/neo.rs`, `src/cli/cli_parts/cli_manifest/`

Generates the final deployment artifacts from the bytecode and metadata.

**NEF (Neo Executable Format):**

- Magic number and version header
- Compiler identifier string
- Source metadata field (overridable with `--nef-source`)
- Method token table (for `CALLT` instructions)
- Script bytecode
- SHA-256 checksum

**Manifest (JSON):**

- Contract name and ABI (methods, events, parameters)
- Supported standards (NEP-17, NEP-11, etc.)
- Permission declarations (inferred from code analysis)
- Trust settings
- Groups and extra metadata

The manifest builder performs permission inference by analyzing all external contract calls in the bytecode and generating the minimal permission set required. The `--deny-wildcard-*` and `--manifest-permissions` flags control how these permissions are validated and overridden.

::: tip
Contracts with parameterised constructors automatically get a deploy prologue injected that uses `StdLib.jsonDeserialize` and `StdLib.deserialize`. The manifest builder checks that the required permissions are present and warns if they are missing.
:::

## Module Organization

```
src/
├── main.rs                     # CLI entry point
├── lib.rs                      # Library root, public API exports
├── cli/                        # Command-line interface
│   ├── mod.rs                  # CLI orchestration
│   ├── bytecode/               # NeoVM bytecode generation (Stage 7)
│   ├── standard_json/          # Standard JSON interface
│   └── cli_parts/              # CLI components
│       ├── cli_run/            # Compilation orchestration, CLI args
│       ├── cli_compile/        # Compile logic, permissions, error types
│       └── cli_manifest/       # Manifest generation and permission inference
├── frontend.rs                 # Solang parser integration (Stage 1)
├── solidity.rs                 # Solidity metadata extraction (Stage 2)
├── semantic.rs                 # Semantic analysis (Stage 3)
├── semantic_model.rs           # Typed semantic model (Stage 4)
├── type_system/                # Type system definitions
├── ir/                         # Intermediate representation (Stage 5)
│   ├── ir_types.rs             # IR type definitions
│   ├── context/                # IR lowering context
│   ├── expressions/            # Expression lowering
│   └── statements/             # Statement lowering
├── optimizer.rs                # Optimizer entry point (Stage 6)
├── optimizer/                  # Optimization passes
│   ├── types.rs                # Optimizer types and config
│   ├── dispatch.rs             # Pass dispatch logic
│   ├── constant_folding.rs     # Constant folding pass
│   ├── dead_code.rs            # Dead code elimination
│   ├── inlining.rs             # Function inlining
│   ├── cse.rs                  # Common subexpression elimination
│   ├── strength.rs             # Strength reduction
│   ├── loops.rs                # Loop optimizations
│   ├── gas.rs                  # Gas-aware optimizations
│   └── stats.rs                # Optimization statistics
├── codegen.rs                  # Code generation utilities
├── lexer.rs                    # Yul lexer
├── parser.rs                   # Yul parser
├── neo.rs                      # Neo-specific utilities (Stage 8)
├── storage_key.rs              # Storage key handling
├── error.rs                    # Error types and diagnostic builder
├── warning.rs                  # Warning system
└── runtime/                    # Embedded NeoVM runtime
    ├── execution/              # Execution engine
    ├── spec.rs                 # Opcode/syscall/native contract specs
    ├── spec/                   # Spec data files
    │   ├── opcodes.rs          # Full NeoVM opcode table
    │   ├── syscalls.rs         # Syscall registry
    │   ├── native_contracts.rs # Native contract hash table
    │   └── gas.rs              # Syscall gas cost table
    └── helpers/                # Runtime helper functions
```

## Dependencies

### Core

| Crate                      | Purpose                                            |
| -------------------------- | -------------------------------------------------- |
| `solang-parser`            | Solidity parsing (AST generation)                  |
| `clap`                     | CLI argument parsing                               |
| `serde` / `serde_json`     | JSON serialization (manifest, standard-json)       |
| `sha2` / `sha3` / `ripemd` | Cryptographic hash functions                       |
| `thiserror` / `anyhow`     | Error handling                                     |
| `once_cell`                | Lazy static initialization (opcode/syscall tables) |
| `hex`                      | Hex encoding/decoding                              |

### Dev / Test

| Crate       | Purpose                            |
| ----------- | ---------------------------------- |
| `criterion` | Benchmarking                       |
| `proptest`  | Property-based testing             |
| `tempfile`  | Temporary file utilities for tests |

## Extension Points

### Adding a New Opcode

1. Add the opcode entry to `src/runtime/spec/opcodes.rs` with hex code, name, and gas cost
2. Implement the execution logic in `src/runtime/execution/`
3. Add IR support in `src/ir/ir_types.rs` if the opcode needs direct IR representation
4. Update the code generator in `src/cli/bytecode/` to emit the new opcode

### Adding a New Syscall

1. Add the syscall name to the registry in `src/runtime/spec/syscalls.rs`
2. Add the gas cost entry in `src/runtime/spec/gas.rs`
3. Implement the syscall handler in `src/runtime/execution/`
4. Update `src/codegen.rs` if the syscall needs a new interop ID computation

### Adding a New Native Contract

1. Add the contract hash to `src/runtime/spec/native_contracts.rs`
2. Implement method handlers in the runtime execution engine
3. Update the devpack with Solidity interface files in `devpack/contracts/`

### Adding a New Optimization Pass

1. Create a new file in `src/optimizer/` implementing the pass
2. Add the pass to `OptimizationPasses` in `src/optimizer/types.rs`
3. Wire it into the dispatch logic in `src/optimizer/dispatch.rs`
4. Assign it to the appropriate optimization level in `OptimizationPasses::for_level()`

## Build System

```bash
# Debug build
cargo build

# Release build (optimized binary)
cargo build --release

# Run all tests
cargo test --workspace

# Run specific test suite
cargo test runtime_  # Runtime tests
cargo test e2e_      # End-to-end compilation tests

# Format code
cargo fmt

# Lint
cargo clippy

# Benchmark
cargo bench
```

The release binary is at `target/release/neo-solc`.

## Test Organization

```
tests/
├── runtime_*.rs              # Runtime unit tests (opcode/syscall behavior)
├── e2e_compilation_tests.rs  # End-to-end: Solidity source -> NEF output
└── conformance_tests.rs      # Conformance against Neo N3 spec
```

## See Also

- [CLI Reference](/compiler/using-the-compiler) -- all compiler options
- [Runtime Specification](/internals/runtime-specification) -- NeoVM execution model
- [Error Reference](/advisory-content/error-reference) -- diagnostic codes from each pipeline stage
- [Parity and Limitations](/internals/parity-and-limitations) -- known gaps
- [Troubleshooting](/advisory-content/troubleshooting) -- common issues and solutions
