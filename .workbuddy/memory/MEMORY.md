# Neo DevPack Solidity — Project Conventions

## Architecture
- Solidity-to-NeoVM compiler: Parsing → IR → Optimization → Bytecode → NEF + Manifest
- 503 Rust source files + 115 test files (~90.5K LOC)
- 965 tests across 54 targets, ~87% famous-contract compilation coverage
- 11 Neo N3 native contracts supported, all syscall surfaces verified
- 196/196 opcodes implemented, 35/35 syscalls handled, 101/107 native methods (6 intentional stubs)
- `foundry-solang-parser` v0.3.9 for Solidity parsing (latest available, published May 2025)

## Module Layout (post-refactoring)
- `src/frontend/` — Solidity parsing, pragma validation, contract IR extraction
- `src/solidity/` — Solidity analysis (inheritance, modifiers, sibling merge)
- `src/ir/` — Intermediate representation construction, lowering, expressions, statements
- `src/runtime/` — Embedded NeoVM runtime simulator (167 files, deep nesting, high coupling)
- `src/cli/` — Command-line interface, bytecode assembly, manifest generation, tests
- `devpack/contracts/` — Solidity library contracts: Framework, Syscalls (split into domain libs), NativeCalls (split into domain libs)
- `devpack/libraries/` — Neo.sol, Runtime.sol, Storage.sol

## Key Conventions
- No `#[path = "..."]` anti-pattern — all modules use standard Rust layout (0 remaining)
- Maximum directory depth target: ≤7 levels (was 10)
- Files >800 lines: 13 remaining. Monolithic pipelines (assembly, solidity_analyse) resist mechanical splitting
- Test files follow Rust conventions: `tests/mod.rs` for integration tests, `#[cfg(test)] mod tests;` for unit tests
- Solidity contracts use `^0.8.19` pragma (minimum) or `>=0.8.19 <0.8.28` for compiler-supported range
- Native contract hashes defined in both Rust (`NATIVE_CONTRACTS` array) and Solidity (`NativeContracts.sol`)

## File Splitting Best Practices
- ✅ Files with clear domain boundaries + independent pub functions: safe to mechanically split
- ❌ Single monolithic function (match/if-let chain): do NOT mechanically split — use surgical refactoring
- ❌ Deeply nested modules (>3 levels): import complexity can cause cascading visibility issues
- `opcode.rs` and `member_nativecalls.rs` are good examples of successful splits

## Testing
- `cargo check` must pass clean (0 warnings, 0 errors) before any commit
- `cargo test --no-run` to verify all test binaries compile
- Famous contract compilation test: `cargo test -p neo-devpack-solidity` (55 targets)
- Fuzz tests in `tests/fuzz_tests/` — now using standard `mod.rs` layout (no #[path])

## v1.0 Remaining Roadmap
- Runtime streamlining (167→120 files) — HIGH RISK, VM bridging coupling
- Solidity 0.8.29+ feature additions (function types, fixed/ufixed) — feature work
- 13 remaining 800+ line files — require surgical refactoring per file

## Version History
- **v0.28.1** (2026-07-03): Audit-driven patch — fixed all P2/P3 issues from v0.28.0 audit
  - P2-1: Nested EQUAL type-strictness (recursive Array/Map comparison)
  - P2-2: NativeTypes.ContractState field types corrected
  - P2-3: 20 missing runtime handlers implemented (NEO 5, ContractMgmt 4, StdLib 10, CryptoLib 1)
  - P2-4/P2-5: Dead code removed
  - P3-1 through P3-5: Clippy, imports, using directives, pragma, 14 missing Solidity wrappers
  - New deps: bs58 (base58), ed25519-dalek (Ed25519 verification)
- **v0.28.0** (2026-07-03): Precision & correctness release (ByteString/Buffer distinction, gas scaling, streaming iterator, Oracle enrichment)
