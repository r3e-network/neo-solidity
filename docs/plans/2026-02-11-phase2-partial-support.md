# Phase 2 Partial-Feature Support Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Extend feasible partial features by adding import alias/wildcard support, improving multi-catch lowering semantics, and broadening low-level call-data recognition.

**Architecture:** Keep parser/front-end behavior unchanged and focus on IR lowering and source-resolution layers. Add focused regression tests first, then implement minimal compiler changes needed to pass. Preserve NeoVM constraints where semantic parity with EVM is impossible.

**Tech Stack:** Rust, `solang_parser`, existing IR (`Instruction`, `LoweringContext`), Cargo test suite.

### Task 1: Import syntax support parity

**Files:**
- Modify: `src/cli/tests/imports.rs`
- Modify: `src/cli/tests/standard_json/input_errors.rs`
- Modify: `src/cli/cli_parts/cli_run/imports.rs`
- Modify: `src/cli/standard_json/standard_json_process/imports.rs`

**Step 1: Write failing tests**
- Convert single-file alias/wildcard import tests from rejection to successful resolution+compile.
- Convert standard-json alias/wildcard tests from error expectation to success/no UnsupportedImportSyntax.

**Step 2: Run tests to verify failure**
- Run targeted imports and standard-json tests, confirm failures come from unsupported syntax checks.

**Step 3: Write minimal implementation**
- Accept `Import::Rename` and `Import::GlobalSymbol` as dependency edges by extracting filename path and removing hard rejection diagnostics.

**Step 4: Re-run tests**
- Run targeted import tests and standard-json tests, ensure green.

### Task 2: Multi-catch lowering improvements

**Files:**
- Modify: `src/cli/tests/ir_codegen/control_flow/try_catch.rs`
- Modify: `src/ir/statements/dispatch/try_catch.rs`

**Step 1: Write failing tests**
- Add IR test asserting the lowering does not silently discard earlier catch clauses and prefers explicit fallback strategy.

**Step 2: Run tests to verify failure**
- Run targeted try/catch IR tests.

**Step 3: Write minimal implementation**
- Improve catch-clause selection logic deterministically with best fallback and remove misleading behavior/warnings.

**Step 4: Re-run tests**
- Run targeted try/catch tests and verify pass.

### Task 3: Broader low-level call-data pattern support

**Files:**
- Modify: `src/cli/tests/selectors/low_level_calls.rs`
- Modify: `src/ir/expressions/calls/low_level.rs`

**Step 1: Write failing tests**
- Add tests for wrapped call-data expressions (e.g., `bytes(...)` / `string(...)` wrappers) and selector/signature extraction cases currently unsupported.

**Step 2: Run tests to verify failure**
- Run targeted low-level selector tests.

**Step 3: Write minimal implementation**
- Extend call-data parser helpers to unwrap additional expression wrappers and reuse existing selector/signature resolution.

**Step 4: Re-run tests**
- Run low-level selector tests to green.

### Task 4: Documentation and verification

**Files:**
- Modify: `FEATURE_MATRIX.md`
- Modify: `README.md`
- Modify: `docs/SOLIDITY_SUPPORT_MATRIX.md`

**Step 1: Align docs with implemented behavior**
- Update imports and catch clauses notes to match actual support.

**Step 2: Run verification commands**
- `cargo test cli::tests::imports:: -- --nocapture`
- `cargo test cli::tests::standard_json:: -- --nocapture`
- `cargo test cli::tests::ir_codegen::control_flow:: -- --nocapture`
- `cargo test cli::tests::selectors:: -- --nocapture`
- `cargo test --lib`
- `cargo fmt --all -- --check`
