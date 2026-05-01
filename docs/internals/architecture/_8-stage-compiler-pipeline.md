---
title: "Architecture: 8-Stage Compiler Pipeline"
description: "8-Stage Compiler Pipeline from Architecture."
---

# 8-Stage Compiler Pipeline

[Back to Architecture](/internals/architecture)

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

## Stage 1: Frontend (solang-parser)

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

## Stage 2: Metadata Extraction

**Module:** `src/solidity.rs`

Extracts structured metadata from the parsed AST for use by later stages.

Extracted metadata includes:

- `ContractMetadata` -- complete contract-level information (name, inheritance, state variables)
- `FunctionMetadata` -- function signatures, visibility, modifiers, return types
- `EventDefinition` -- event names and parameter types
- `NatspecDoc` -- documentation comments (`@notice`, `@param`, `@return`)

This metadata drives manifest generation (ABI, supported standards) and informs the semantic analysis stage.

## Stage 3: Semantic Analysis

**Module:** `src/semantic.rs`

Performs type checking and validation on the parsed AST using the extracted metadata.

Responsibilities:

- Symbol table construction and scope resolution
- Type checking and type inference
- Storage layout validation (Neo-specific constraints)
- Visibility and access control verification
- Detection of unsupported EVM features

Errors at this stage produce `E2xxx` semantic error codes. Security warnings (`E5xxx`) are also emitted here.

## Stage 4: Semantic Model

**Module:** `src/semantic_model.rs`, `src/type_system/`

Constructs a fully typed intermediate representation from the validated AST. This is the bridge between the Solidity-specific frontend and the target-independent IR.

The semantic model resolves:

- All type references to concrete types
- Function overload resolution
- Inheritance linearization
- Storage slot assignments

## Stage 5: IR Generation

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

## Stage 6: Optimizer

**Module:** `src/optimizer.rs`, `src/optimizer/`

A multi-level optimization pipeline that transforms the IR to reduce bytecode size and improve execution efficiency.

### Optimization Levels

| Level | Passes                                                                                       | Description                                                  |
| ----- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| `-O0` | None                                                                                         | No optimization. IR is emitted directly. Best for debugging. |
| `-O1` | Constant folding                                                                             | Evaluate constant expressions at compile time.               |
| `-O2` | Constant folding, dead code elimination                                                      | Also remove unreachable code after returns. Default level.   |
| `-O3` | Constant folding, dead code elimination, function inlining, common subexpression elimination | Maximum optimization. Smallest bytecode.                     |

### Optimization Passes

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

### Optimization Statistics

The optimizer tracks metrics for each compilation:

- `eliminated_instructions` -- instructions removed by DCE
- `inlined_functions` -- functions inlined
- `folded_constants` -- constant expressions evaluated
- `nodes_before` / `nodes_after` -- AST size reduction
- `reduction_percent` -- overall size reduction percentage

Use `-v` (verbose) to see these statistics in the compiler output.

## Stage 7: Code Generation

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

## Stage 8: Artifact Builder

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
