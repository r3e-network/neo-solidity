# v0.30.0 — Architecture Phase 4: File Refactoring

## What was done

Split 3 of the largest monolithic files (>800 lines) into smaller, domain-focused submodules. Reduces the count of non-test files exceeding 800 lines from 13 to 10.

### Files split

| File | Before | After | Strategy |
|------|--------|-------|----------|
| `assembly.rs` | 1440 lines | 5 files (10 + 190 + 289 + 562 + 407) | Domain separation: extsload vs Yul, then Yul dispatch vs opcodes |
| `frontend_parse.rs` | 1037 lines | 5 files (13 + 246 + 312 + 304 + 184) | Domain separation: parse vs pragma vs semver vs natspec |
| `lower_assignment.rs` | 1140 lines | 2 files (728 + 413) | Section extraction: storage array ops separated from main lowering |

### Remaining files >800 lines (10)

These are monolithic match chains or dispatchers that resist mechanical splitting:

| File | Lines | Type |
|------|-------|------|
| `stdlib.rs` | 1372 | Monolithic match dispatch |
| `solidity_analyse.rs` | 1207 | Monolithic pipeline (1 fn, 5 stages) |
| `low_level.rs` | 1103 | Monolithic dispatch |
| `abi_encode.rs` | 1017 | Monomorphic encoder |
| `abi_decode.rs` | 979 | Monomorphic decoder |
| `resolve.rs` | 944 | Monolithic resolver |
| `arrays.rs` | 918 | Monolithic handler |
| `member_calls.rs` | 912 | Monolithic dispatch |
| `binary_u256_softarith.rs` | 897 | Monolithic handler |
| `return_lower.rs` | 866 | Monolithic handler |

These require surgical refactoring — each match arm may have implicit dependencies on shared locals or fallthrough behavior.

### Verification

- `cargo check`: 0 errors, 0 warnings
- `cargo clippy`: 0 warnings
- `cargo test`: **965 tests passed, 0 failed** — zero regressions
- Public API unchanged
