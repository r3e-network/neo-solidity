# Fuzz Testing

Neo Solidity's fuzz system combines **proptest** (property-based testing on
the compiled runtime) with **cargo-fuzz / libFuzzer** (coverage-guided
random input generation). The goals are complementary:

| Layer        | Purpose                                            | Where              |
|--------------|----------------------------------------------------|--------------------|
| proptest     | Correctness — the compiler/runtime behaves per spec | `tests/fuzz_tests/` |
| cargo-fuzz   | Robustness — no panics on pathological input       | `fuzz/fuzz_targets/` |
| differential | Parity — output matches a reference implementation | `tests/fuzz_tests/differential.rs` |

## Quick reference

```bash
# Run the full proptest suite (~2 s, 743 cases by default).
cargo test --test fuzz_tests

# Deep run (100 cases per proptest).
./scripts/run_fuzz_suite.sh deep

# Continuous loop (proptest + cargo-fuzz burst, Ctrl-C to stop).
./scripts/run_continuous_fuzz.sh

# Single cargo-fuzz target (nightly toolchain required).
cargo +nightly fuzz run fuzz_target_1 -- -max_total_time=60
```

## proptest suite (`tests/fuzz_tests/`)

Each module targets a specific compiler / runtime surface. Module names
prefixed `batches_*` are historical — newer tests live in feature-named
files. File count and LoC are tracked in the audit in
`FUZZ_STATUS_REPORT.md`.

### Layout

| File | Purpose |
|------|---------|
| `common.rs` | Shared strategies, `observe()` helper, `decode_uint_le` |
| `compiler_props.rs` | Compiler-level invariants (identifiers, literals, types, manifest) |
| `arithmetic_props.rs` | Integer arithmetic panics, syntax resilience |
| `optimizer_props.rs` | Optimizer must preserve semantics across levels 0-3 |
| `storage_props.rs` | Storage roundtrip, large values, key ordering |
| `differential.rs` | **Differential tests** — compiler output vs `sha2`/`ripemd`/`sha3`/`num-bigint` reference crates |
| `baseline_tests.rs` | Foundational invariants (known good / known gap harnesses) |
| `task107_catch_panic_tests.rs` | Solidity Panic(uint256) canonical envelope |
| `batches_*_*.rs` | Historical numbered batches — see per-file headers for scope |
| `batches_111_115.rs` | Runtime verification: StdLib / CryptoLib / NEO / GAS / Policy / Ledger / EVM auto-maps |
| `batches_116_120.rs` | Solidity features: concat / unchecked / short-circuit / ternary / tuple returns |
| `batches_116_120.rs::batch121_*` | Bare crypto intrinsics (`sha256`, `ripemd160`, `keccak256`) |

### Writing a new proptest

Use `common::compile_and_execute` and `common::observe` when possible so
the fault-shape detection (Panic(0x11) etc.) is consistent across the
suite:

```rust
#[test]
fn my_invariant(data in any::<u8>()) {
    let src = format!(r#"...{data}..."#);
    let arts = compile_contracts(&src, false, 2).unwrap();
    let art = &arts[0];
    let mut rt = NeoRuntime::new(RuntimeConfig::default()).unwrap();
    let r = rt.call_method(&art.bytecode, &art.tokens, &art.manifest, "m", &[]).unwrap();
    prop_assert!(r.success, ...);
    prop_assert_eq!(decode_uint_le(&r.return_data), expected, ...);
}
```

**Assertion strength matters.** Prefer `prop_assert_eq!` against a concrete
expected value over `prop_assert!(r.success)` alone — the latter is
coverage theater that passes when the runtime stubs the underlying
operation. When a stub is the reality, use `#[ignore]` with a TODO
linking to the task, not a weak pass.

### Proptest regressions

Each module has a sibling `.proptest-regressions` file checked into
git. When a proptest fails, proptest minimizes the input and records
the seed + input; subsequent runs replay that input first. **Commit
these files** so regressions are reproducible across CI and dev
machines.

## cargo-fuzz targets (`fuzz/fuzz_targets/`)

libFuzzer-driven coverage-guided fuzzing. Each target tests a different
compiler/runtime surface for robustness (no panics on bad input).

| Target | Surface | Invariant |
|--------|---------|-----------|
| `fuzz_target_1` | Solidity compile pipeline | No panics on random UTF-8 input at opt levels 0-3 |
| `fuzz_target_2` | NEF parser | Rejects malformed NEF without panic |
| `fuzz_target_disasm` | Bytecode disassembler | Total function — any byte sequence → a `String` |
| `fuzz_target_nef_roundtrip` | Parser / serializer asymmetry | `parse(x).ok() ⇒ build(…) must not panic` |
| `fuzz_target_manifest_json` | Manifest JSON parser | Pathological input rejected cleanly, no panic |
| `fuzz_target_standard_json` | solc-compatible Standard JSON input | Malformed JSON / pathological settings rejected without panic |

### Running

```bash
# List all targets.
cargo +nightly fuzz list

# Run one target for 60 seconds.
cargo +nightly fuzz run fuzz_target_1 -- -max_total_time=60

# Run all targets for 30s each.
for t in $(cargo +nightly fuzz list); do
    echo "== $t =="
    cargo +nightly fuzz run "$t" -- -max_total_time=30
done
```

### Corpus

Each target has its corpus under `fuzz/corpus/<target_name>/`. The
corpus grows as libFuzzer discovers new coverage; **do not commit** the
auto-discovered inputs (they're gitignored under `fuzz/corpus/`). Seed
inputs that came from `examples/*.sol` or `*.nef` test artifacts are
prefixed `seed_` and are committed to kickstart coverage on a fresh
checkout.

### Triaging a crash

1. cargo-fuzz writes the crashing input to
   `fuzz/artifacts/<target>/crash-<hash>`.
2. Reproduce:
   ```bash
   cargo +nightly fuzz run <target> fuzz/artifacts/<target>/crash-<hash>
   ```
3. Minimize:
   ```bash
   cargo +nightly fuzz tmin <target> fuzz/artifacts/<target>/crash-<hash>
   ```
4. Add a `#[test]` that wraps the minimized input as a regression guard,
   fix the bug, and delete the crash artifact.

### Recent bug class: DoS-via-unbounded-recursion / expansion

Several recent fuzz-surfaced bugs share the same shape: a parser or
lowering step walks a user-controlled shape (exponent, array dims,
nested struct field types) without a depth / size cap, and a crafted
input pushes it to OOM or stack-overflow the compiler. Fixes add an
explicit bound at the entry point:

- `pow10(exp)` on decimal literals — `MAX_DECIMAL_EXPONENT=1024`
  (`src/ir/build/literals.rs`).
- Constant-folded `base ** exp` — `MAX_LITERAL_POW_EXP=1024`
  (`src/ir/expressions/power.rs`).
- Fixed-array leaf count on returns — `MAX_FIXED_ARRAY_LEAVES=65536`
  (`src/ir/statements/dispatch/return_revert.rs`).
- Recursive struct field resolution — `MAX_STRUCT_RESOLUTION_DEPTH=64`
  (`src/type_system/parse.rs`).

When adding a new lowering pass, ask: *"can user-controlled input make
this loop / recurse unboundedly?"* — if yes, cap it and surface a clean
compiler error.

## Differential tests (`tests/fuzz_tests/differential.rs`)

Compare Solidity output against a reference implementation in the same
process. Covers:

- `sha256 / ripemd160 / keccak256` intrinsics vs the `sha2`, `ripemd`,
  `sha3` crates across 0..=256-byte random inputs.
- `addmod / mulmod` vs native u128 arithmetic across the
  single-slot range.
- Disassembler totality across 0..=1024-byte random inputs (structured
  proptest complement to `fuzz_target_disasm`).

A failing differential test is **always a compiler / runtime bug**, not
a test bug — the reference implementations are correct by assumption.

## Continuous fuzzer

`scripts/run_continuous_fuzz.sh` runs the full proptest suite + each
cargo-fuzz target in a loop. Useful for long overnight runs and for
agents that want to hold the suite green while making changes.

```bash
# Ctrl-C to stop. Logs land in /tmp/fuzz-continuous/.
./scripts/run_continuous_fuzz.sh

# From a Claude Code session, monitor it in the background:
#    Use the Monitor tool with the run_continuous_fuzz.sh command;
#    each round reports pass/fail + coverage to the chat.
```

## CI

`.github/workflows/fuzz.yml` runs nightly at 02:00 UTC:

1. `proptest-deep` — full suite with `PROPTEST_CASES=100`.
2. `cargo-fuzz-smoke` — 60 s per target.

Failing jobs keep the workflow red; a crash artifact is uploaded as a
workflow artifact for later triage.

## Make targets

| Target | What |
|--------|------|
| `make test-fuzz-gate` | Quick gate (default case count, ~2 s) |
| `make test-fuzz-continuous` | Deep run (`PROPTEST_CASES=100`) |
| `make test-fuzz-ci` | Workspace + all features |
| `make test-fuzz-start` | Start background continuous runner |
| `make test-fuzz-status` | Status of background runner |
| `make test-fuzz-stop` | Stop background runner |

## Adding coverage

- New Solidity feature landed? Add a proptest in the matching module —
  `compiler_props.rs` for compile-path invariants, `differential.rs` if
  the feature has a reference implementation, or a new feature-named
  module.
- New runtime behavior exposed? Runtime-verify it with an assertion on
  the concrete expected value (not just `success`).
- New public entry point? Add a cargo-fuzz target covering it.

## Design principles

1. **Tests fail loudly, not silently.** Weak assertions hide bugs.
2. **Strict assertions over coverage theater.** `assert_eq!(x, expected)`
   beats `assert!(r.success)` every time.
3. **Differential > absolute.** When a reference implementation exists,
   use it — production correctness is divergence-free.
4. **Seed real inputs.** Corpus bootstrapped from `examples/` reaches
   coverage faster than pure random.
5. **Commit regressions.** Every proptest failure produces a
   `.proptest-regressions` line — check it in so CI replays it.
6. **Small inputs reveal more bugs than big inputs.** Cap fuzz input
   sizes (<= 256 / <= 1024 bytes) so shrinking is fast.
