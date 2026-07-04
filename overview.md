# v0.29.0 — Architecture Phase 1: CLI Decomposition

## What was done

Extracted two major subsystems from the CLI god module (151 files) into first-class top-level modules, following the architecture design's Phase 1 plan.

### Extracted modules

| Module | Source | Files | LOC | Dependencies |
|--------|--------|-------|-----|-------------|
| `src/codegen/` | `src/cli/bytecode/` | 29 | ~5,394 | opcode, frontend, ir, solidity |
| `src/optimizer/` | `src/cli/ir_optimize/` | 5 | 692 | ir only |

### CLI module reduction

- **Before**: 151 files (bytecode + optimizer + cli_parts + standard_json + tests)
- **After**: 117 files (cli_parts + standard_json + tests)
- **Extracted**: 34 files (~6,086 LOC)

### Key decisions

1. **Optimizer extracted first** — simplest, depends only on `ir`. Validated the extraction approach.
2. **Codegen extracted second** — self-contained, zero internal CLI coupling. All `bytecode::` references in cli_parts and tests updated to `codegen::`.
3. **Manifest extraction deferred** — `cli_manifest` has two blocking couplings:
   - Bidirectional dependency with `standard_json` (they call each other)
   - `CompileError` ownership (defined in `cli_compile/types.rs`, used by `cli_manifest`)
   - These require breaking before the manifest can be cleanly extracted

### Verification

- `cargo check`: 0 errors, 0 warnings
- `cargo clippy`: 0 warnings
- `cargo test`: **965 tests passed, 0 failed** — zero regressions
- Public API unchanged

## Follow-up items

- Break `standard_json ↔ cli_manifest` bidirectional dependency (sub-phase)
- Break `CompileError` ownership coupling (sub-phase)
- Complete manifest extraction once couplings are resolved
- Phase 2 (Runtime isolation) can begin independently
