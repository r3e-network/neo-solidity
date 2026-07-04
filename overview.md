# v0.30.1 — Code Quality Fixes

## What was done

Fixed error handling and unsafe code issues identified in the team capability improvement plan's code quality audit.

## Fixes Applied

### 1. Manifest `eprintln!` elimination (7 → 0)

All 7 `eprintln!` calls in `manifest/build.rs` were replaced with a `warnings: &mut Vec<String>` collector parameter. Warnings are now:
- Collected during manifest building
- Returned to the caller (`compile.rs`)
- Converted to `Diagnostic::warning()` entries
- Added to the compilation warnings list

This means manifest warnings now flow through the standard diagnostic pipeline instead of being printed directly to stderr.

### 2. Runtime `unwrap()` hardening (3 risky instances → 0)

| File | Before | After |
|------|--------|-------|
| `execution_impl_part3_conversion.rs` | `bytes.last().unwrap()` (could panic on empty) | Empty-bytes guard + `expect()` |
| `execution_impl_part3_offsets/call_stack.rs` | `.unwrap()` | `.expect("guarded by len > stack_base")` |
| `execution_impl_part2_native/stdlib.rs` | `.unwrap()` | `.expect("guarded by len() == 1")` |

### 3. Unsafe block documentation (1 undocumented → 0)

Added `# Safety` comment to `storage_ops.rs` line 164 explaining the soundness argument.

## Remaining Items

All remaining `unwrap()`/`expect()` (30+ instances) are documented invariants:
- Hardcoded table lookups with matching arms
- `Regex::new()` on compile-time constant patterns (idiomatic Rust)
- `checked_add` with impossible-overflow comments
- Value-range-guarded lookups

CLI `eprintln!` calls (in `single_file.rs`, `standard_json.rs`, `cli_analyze.rs`) are legitimate CLI stderr output — fatal errors followed by `exit(1)`. This is the standard pattern for CLI tools.

## Verification

- `cargo check`: 0 errors, 0 warnings
- `cargo clippy`: 0 warnings
- `cargo test`: **965 tests passed, 0 failed** — zero regressions
