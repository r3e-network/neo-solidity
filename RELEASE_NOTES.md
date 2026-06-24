# neo-devpack-solidity v0.23.0 — Deep-Refactor & Correctness Release

**Release date:** 2026-06-24
**Compiler / CLI / workspace:** **v0.23.0**
**devpack (`@neo-devpack-solidity/contracts`):** **v0.23.0**
**Target Neo N3 node:** **v3.10.0** (Gorgon-prep; no hardfork activation)

> Canonical change list: [`CHANGELOG.md`](./CHANGELOG.md) `[v0.23.0]`.
> Previous releases: see the
> [GitHub releases page](https://github.com/r3e-network/neo-devpack-solidity/releases)
> and `CHANGELOG.md` history (previous: v0.22.0 / devpack v2.2.0).

---

## TL;DR

- **2 correctness fixes:** `mulmod` 512-bit intermediate precision + `CheckSig`
  synthetic-hash default removed.
- **2 audit fixes:** L-FE1 (silent SourceUnitPart drop) + L-DEV (governance
  enumeration).
- **−2391 LOC** of dead code removed. 5 god-objects split into 19 modules.
- **All 186 production `include!` → `mod`** — proper Rust module tree.
- **Runtime feature-gated** — lean binary builds skip 17K-LOC simulator.
- **49 test suites green** throughout (zero regressions).
- **v0.22 audit validated:** 26 findings, 19 YES / 7 PARTIAL / 0 NO.

---

## 🔴 Breaking correctness changes

### mulmod full 512-bit precision

`mulmod(a, b, m)` previously truncated `a*b` to 256 bits, producing silently
wrong results when `a*b ≥ 2^256`. Now computes the full 512-bit product via
8-column schoolbook multiplication, then reduces via bit-serial shift-subtract.

**Before:** `mulmod(type(uint128).max, type(uint128).max, type(uint256).max)`
returned a truncated wrong value.

**After:** returns the correct `1` (since `2^256 mod (2^256-1) == 1`).

### CheckSig default behavior

`System.Crypto.CheckSig` without `override_signing_hash()` now returns
`false` (reject) instead of verifying against a synthetic hash. The synthetic
hash (`SHA256(bytecode‖account‖counter)`) matched no real signature, so
uninjected results were meaningless. Tests requiring meaningful CheckSig must
call `override_signing_hash()`.

---

## 🟡 Improvements

- **Optimizer differential** (M-TEST3) now compares storage state + notification
  logs across O0/O3, not just `return_data`.
- **BLS12-381 Gt encoding** (S5) pinned with a stability test guard.
- **Criterion benchmarks** added — first-ever perf baseline.
- **Neo-Express CI** expanded: 16 scripts in 4-way parallel matrix (15-min).
- **Rayon parallel compilation** for multi-contract inputs.
- **Public API** reduced to 2 documented modules (`cli`, `neo`); 9 internal
  modules are `#[doc(hidden)]`.

---

## 🔵 Internal architecture (zero behavior change)

| Area | Before | After |
|---|---|---|
| Dead modules | 8 unused (security, docs, testing, codegen_helpers, validation, warning, types, error) | Deleted (−2400 LOC) |
| God-objects | 5 files >1100 LOC (largest 2426) | Split into 19 modules (largest ~1024) |
| Error pipeline | Catch-all `to_string()` → `GENERIC_ERROR` | 11 explicit variant arms |
| Code inference | 25-branch substring matching | Codes set at construction |
| Module system | 186 production `include!` textual includes | All converted to `mod` declarations |
| Runtime | Always compiled (17K LOC) | `#[cfg(feature = "runtime")]` (default-on) |

---

## Upgrade notes

- If your tests depend on `CheckSig` returning `true` without
  `override_signing_hash()`, add the injection call.
- `cargo build --no-default-features --bin neo-solc` produces a leaner binary
  (skips the runtime simulator).
- `cargo bench` now works (criterion benchmarks in `benches/compile.rs`).
