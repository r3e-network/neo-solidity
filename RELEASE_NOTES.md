# neo-devpack-solidity v0.24.0 — Hardening & Real-Node Verification

**Release date:** 2026-06-27
**Compiler / CLI / workspace:** **v0.24.0**
**devpack (`@neo-devpack-solidity/contracts`):** **v0.24.0**
**Target Neo N3 node:** **v3.10.0** (Gorgon-prep; no hardfork activation)

> Canonical change list: [`CHANGELOG.md`](./CHANGELOG.md) `[v0.24.0]`.
> Previous releases: see the
> [GitHub releases page](https://github.com/r3e-network/neo-devpack-solidity/releases)
> and `CHANGELOG.md` history (previous: v0.23.0 / devpack v2.3.0).

---

## TL;DR

- **5 correctness fixes** surfaced by the new real-Node differential
  harness and the famous-contracts runtime smoke:
  `a ** b` on-chain fault, S6 manifest permission gap (compiler +
  runtime), S6 CallFlags propagation, M-IR2 logical-operator bool
  normalize, NEP-11 `mintToSelf` reverts on bad receiver.
- **2 safety hardening passes**: silent u16-index collision on
  local-slot overflow → loud panic; `CONVERT`-to-Integer 33-byte
  leniency → real-node-faithful error.
- **2 new real-Node test harnesses**: the differential harness
  (14/14 PASS on Neo-Express 3.9.1) and the famous-contracts
  runtime smoke (92 contracts scanned, WETH9 fully passing).
- **Round-4 dedup / refactor**: −25 LOC net across 19 files, 5
  duplicated definitions collapsed into single sources of truth,
  dead code removed.
- **1885 tests green** (up from 1844 in v0.23.0); clippy + fmt clean.

---

## 🔴 Breaking correctness changes (real-node oracle-discovered)

### `a ** b` no longer faults on real NeoVM

The previous overflow check compared the loop's accumulated product
against a 33-byte `2^256` literal — a `PUSHDATA1 <33 bytes>` followed
by `ADD 0` — which NeoVM accepts only as a `ByteString` and REJECTS
the moment the value is coerced to an `Integer` (fault message:
`MaxSize of Integer is exceeded: 33/32`). Real Neo-Express 3.9.1
rejected `2 ** 10` on a fresh deployment.

**After:** the check is `(result >> 255) >= 2`. The 255-bit shift fits
in 32 signed bytes, and `2^255 >> 255 = 1` (no overflow) vs
`2^256 >> 255 = 2` (overflow detected) is a clean discriminator.
Post-truncate result also `Instruction::Convert { Integer }`-ed so
the new CONVERT fidelity gate (see below) catches it locally next time.

Diff harness: `pow_test([2, 10])` was FAULT → HALT(1024).

### S6 manifest permission gate fully wired (compiler + runtime)

Before, the compiler's manifest permission derivation only saw IR-level
`BuiltinCall::NativeCall` markers. It silently missed the codegen paths
that emit native calls directly to bytecode — most notably the
`keccak256` / `serialize` storage-key-derivation helpers reached via
`StoreState(computed_slot)` for fixed-size array elements inside structs.
Those contracts emitted valid-looking bytecode that faulted on real
nodes with `no permission to call System.Runtime.Serialize`.

**After:**
- `src/cli/cli_parts/cli_manifest/permissions/native.rs::collect_bytecode_native_permissions`
  scans both `CALLT` (0x37) method tokens and `System.Contract.Call`
  (0x41) syscall sites, extracting `(hash_le, method)` by walking the
  operand stack backwards for the 20-byte hash PUSHDATA and the
  preceding method-name string PUSHDATA.
- Wired into `infer_permissions(metadata, ir_module, bytecode, tokens)
  → build_manifest(metadata, ir_module, bytecode, tokens) → compile.rs`.
- Runtime: `ExecutionContext::manifest_permissions: Option<Vec<ManifestPermission>>`,
  parsed from the manifest JSON; `manifest_permits(target_hash_le, method)`
  check fires in `handle_contract_call` before `invoke_native_contract`.

Diff harness: `StructFixed` struct storage contract was
`5 × FAULT` → HALT.

### S6 CallFlags propagation

The runtime previously had only the `WRITE_STATES` flag declared; every
mutating syscall was unconditional. Now the full Neo N3 `CallFlags`
bitmask is declared (ReadStates / WriteStates / AllowCall /
AllowNotify / All), each mutating syscall gates on its bit
(`AllowNotify` for `Runtime.Notify` + `Runtime.Log`, `AllowCall` for
`System.Contract.Call`, etc.), and the caller's flag set is saved per
`CallFrame` and restored on return + on exception unwind so nested calls
don't leak permissions across frames. Compiler-emitted flags are also
validated to fit in `0x0F`.

### M-IR2 logical operators normalize

`||` and `&&` right operands were not always coerced to `Boolean` after
short-circuit evaluation, producing an `Integer`-typed result when the
LHS was already a boolean. Added `Instruction::Convert { Boolean }`
after each right-operand evaluation. +1 structural + behavioral test.

### M-DEV1 NEP-11 `mintToSelf` reverts on bad receiver

Un-deferred `mintToSelf` silently succeeded when the contract's own
`INEP11Receiver.onNEP11Received` reverted, leaving the contract owning
its own token (a real-footgun). New test
(`m_dev1_nep11_mint_to_self_succeeds_and_contract_owns_token`) uses a
reverting receiver as the failure-mode discriminator; the test fails
if the contract ends up owning the token.

---

## 🟢 Safety hardening (silent → loud)

### Local-slot overflow no longer silently collides

`LoweringContext::allocate_local` previously did
`checked_add(1).unwrap_or(self.local_count)`, which on the (impossible)
u16 overflow returned the SAME index for two distinct locals — they'd
share a slot and corrupt IR state silently. Now `.expect("...exceeds
u16::MAX (65 536) locals")` panics with an actionable message instead.
`next_label` got the same treatment for its `usize` counter.

### `CONVERT`-to-Integer errors on `bytes.len() > 32`

Previously the simulator wrapped a >32-byte ByteArray as `ByteArray`
silently — a divergence from real NeoVM that hid
`[2^255, 2^256-1]`-class lowering bugs. Now it returns the same error
real NeoVM does (`MaxSize of Integer is exceeded`), so any future
bytecode that emits an over-sized value faults locally instead of
silently diverging from the real node.

### `unwrap()` hardening

`return_lower.rs::wrap_external_single_array_return_value` had two
`return_types.first().unwrap()` sites after a `len() == 1` guard.
Replaced with an explicit `first_ret_type = &return_types[0]` binding
right after the guard — same behavior, no panics possible.

---

## 🟡 Real-Node test infrastructure (new)

### Neo-Express differential harness

Closes the audit's #1 finding: the simulator accepted bytecode that
real NeoVM rejected. For every probe in `fn cases()`, the same compiled
contract runs in BOTH the in-tree `NeoRuntime` AND a real Neo-Express
3.9.1 node, and the results are diffed. **14/14 PASS** on Neo-Express
3.9.1 across 7 pure methods (`pow_test`, `xor_test`, `shl_test`,
`nested_mod`, `bitwise_complex`, `fn_div`, `pow_wide`, `mul_wide`).

Gated behind `#![cfg(feature = "neoxp-diff")]` + `#[ignore]`; runs in
the dedicated `neoxp-diff` CI job
(`cargo test --features neoxp-diff -- --ignored`).

### Famous-contracts runtime smoke

For every vendored .sol in `third_party/famous-contracts/sources/`
(92 contracts from OpenZeppelin, Uniswap V2/V4, Aave V3, Chainlink,
Safe), compile → deploy to Neo-Express → invoke a representative
read-only method (`name`, `symbol`, `decimals`, `totalSupply`,
`owner`, `paused`, `get`, `view`) → write a markdown report at
`third_party/famous-contracts/RUNTIME_REPORT.md`.

**First-run results**:
- 7/92 compile pass (matches the existing `famous_corpus_vendor_only_compile_floor`)
- 5/7 deploy pass (2 constructor-required contracts fault as expected)
- **1/5 full smoke HALT** — **WETH9** (`@aave/core-v3/contracts/dependencies/weth/WETH9.sol`)
  deployed successfully and `name()` returned `"Wrapped Ether"` on a
  real Neo N3 chain.
- 3/5 "deployed, no smoke" — abstract / library base classes
  (`Initializable` ×2, `MultiSend` ×2) with no zero-arg reads.
- 85 compile FAIL — missing transitive deps (`IERC20.sol`,
  `IERC721.sol`, …) in the leaf-only OZ vendor tree (documented in
  `famous_contracts_compile.rs` as the intended state).

### S7 e2e revert-rollback test

Genuine regression guard — verified by removing the
`restore_storage_snapshot` call and confirming the test fails.

---

## 🔵 Architecture & refactor (Round 4, zero behavior change)

- **−25 LOC net** across 19 files; 5 dedup passes collapsed 3-5
  near-identical definitions into single sources of truth:
  - `canonical_param_type`: 3 duplicates → `crate::utils::canonical_param_type`
  - `method_name_from_signature`: 5 inline implementations → one helper with doctest
  - `BUILTIN_LIBRARY_BASES`: 3 inline `matches!` → the canonical `pub(crate)` const in `ir_context`
  - `MAX_CLIMB = 16`: 2 local consts 36 lines apart → 1 module-level
  - `MAX_DECIMAL_EXPONENT = 1024`: now also reused by `power.rs` (was `MAX_LITERAL_POW_EXP`)
- **`OutputConfig::nef_source()` method** replaces 3 inline
  `config.nef_source_override.unwrap_or(config.input_file)` fallbacks.
- **Dead code removed**: orphaned
  `src/ir/expressions/calls/builtins/helpers.rs` (no `mod helpers;`
  declaration, zero callers), 2 unused `check_*` validators in
  `erc_nep_patterns.rs`, dead `SolidityError::is_recoverable`.
- **Other minor cleanups**: removed 4 dead npm deps + add lean-build
  CI gate (`e8787df`, `7c733bf`); hardened production `unreachable!`
  /`expect` to recoverable errors (`2473e5f`); B2+C2+P3 fix batch
  (restore `from_contract` for tests, fix `multiSigTransfer` API,
  dedup p256 dep listing — `30d6301`); NEP compliance docs verified
  (`77aec5a`).

---

## 📊 Headline numbers

| Metric | v0.23.0 | v0.24.0 |
| --- | --- | --- |
| Test suites green | 49 | **49** |
| Total tests | 1844 | **1885** |
| Diff harness probes (real node) | 0 | **14** |
| Diff harness PASS rate | — | **100 %** |
| Correctness fixes | 4 | **5** |
| Real-node test infra | 5 scripts | **+ diff harness + runtime smoke** |
| Public API breaking changes | 1 (CheckSig) | **0** |

---

## 🚀 How to upgrade

The public-API surface is unchanged from v0.23.0 — this is a
backwards-compatible release. Cargo and npm users will pick up
`v0.24.0` on their next dependency resolution.

**CLI users**: rebuild with `cargo install --path . --version 0.24.0`
or download the prebuilt binary from the
[release page](https://github.com/r3e-network/neo-devpack-solidity/releases/tag/v0.24.0).

**devpack users**: `npm install @neo-devpack-solidity/contracts@0.24.0`.

---

## 🧪 Verification

- `cargo fmt --all -- --check` ✅
- `cargo clippy --all-targets --all-features -- -D warnings` ✅
- `cargo test --workspace` → **1885 passed; 0 failed** ✅
- `cargo test --features neoxp-diff --test neoxp_differential -- --ignored`
  → **14/14 PASS on real Neo-Express 3.9.1** ✅
- `cargo test --features neoxp-diff --test famous_contracts_runtime_smoke -- --ignored`
  → **92 contracts scanned, WETH9 full smoke HALT** ✅
- `cargo build --release --bin neo-solc` → ✅