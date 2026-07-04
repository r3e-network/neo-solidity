# v0.29.1 — Architecture Phase 1 Complete: Manifest Extraction

## What was done

Completed the manifest extraction, finishing Architecture Phase 1 (CLI decomposition). The CLI god module is now fully decomposed into three first-class top-level modules.

### Extracted module

| Module | Source | Files | LOC | Key change |
|--------|--------|-------|-----|-----------|
| `src/manifest/` | `src/cli/cli_parts/cli_manifest/` | 11 | ~2,332 | Broke 2 blocking couplings |

### Couplings broken

1. **Bidirectional dependency** (`cli_manifest` ↔ `standard_json`):
   - Moved `solidity_to_manifest_type` to `src/manifest/mod.rs`
   - `standard_json` now imports from `crate::manifest` (one-directional)

2. **`CompileError` ownership**:
   - Created `ManifestError` type in manifest module
   - `build_manifest` returns `Result<Value, ManifestError>`
   - Mapped to `CompileError::Manifest` at CLI boundary

### CLI module reduction (full Phase 1)

| Version | CLI files | Extracted |
|---------|-----------|-----------|
| v0.28.1 | 151 | — |
| v0.29.0 | 117 | 34 (codegen + optimizer) |
| v0.29.1 | 106 | 11 (manifest) |
| **Total** | **106** | **45 files extracted** |

### Verification

- `cargo check`: 0 errors, 0 warnings
- `cargo clippy`: 0 warnings
- `cargo test`: **965 tests passed, 0 failed** — zero regressions
- Public API unchanged

## Phase 1 complete

All three extractions from the CLI god module are done:
1. `src/codegen/` (29 files) — v0.29.0
2. `src/optimizer/` (5 files) — v0.29.0
3. `src/manifest/` (11 files) — v0.29.1

CLI reduced from 151 to 106 files (45 files, ~8,418 LOC extracted).

## Next steps

- **Phase 2** (v0.30.x): Runtime isolation — decouple 4 subsystems via port interface
- **Phase 3** (v0.31.x): Extension points — trait plugins (can run parallel with Phase 2)
