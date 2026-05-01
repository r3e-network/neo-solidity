# neo-solidity v0.16.0 + devpack v1.1.0 — Fuzz-Driven Stability Release (Archived)

> **Archived release notes**: this file documents the v0.16.0 release.
> The current compiler version is v0.18.0; for the latest state see
> [`CHANGELOG.md`](./CHANGELOG.md) and the
> [GitHub releases page](https://github.com/r3e-network/neo-solidity/releases).
> The numbers, status claims, and TL;DR below reflect v0.16.0 specifically and
> are kept for archaeology rather than current accuracy.

**Release date:** 2026-04-19
**Compiler / CLI / workspace:** **v0.16.0**
**devpack (`@r3e-network/neo-solidity-devpack`):** **v1.1.0**

---

## TL;DR

- **~90 distinct fixes** landed through continuous fuzz-harness review
  (Tasks #94–#183), covering arithmetic, ABI encoding, events,
  modifiers, try/catch, yul, storage, and runtime bridging.
- **409 fuzz harnesses passing** (0 failed, 1 ignored) — up from 52 at
  the start of the review cycle.
- **Narrow Yul support lands**: `mstore`/`mload`/`return`, `let`/`:=`,
  basic arithmetic, and EIP-1153 transient storage (`tstore` / `tload`).
- **Checked arithmetic is now canonical**: `uint256` and `int256`
  Add/Sub/Mul — plus narrow signed int types — route through BigInt
  with post-op range guards and emit the EVM-canonical
  `Panic(uint256)` envelope.
- **EVM envelope parity**: `abi.encode*` / `abi.decode`, event
  `topic0 = keccak256(signature)`, `Error(string)` for `require`/
  `revert`, and `Panic(0xNN)` for every panic site now match the EVM
  wire shape, so `catch Error(string)` / `catch Panic(uint)` clauses
  match against the canonical data.

---

## By the numbers

| Metric                          | v0.15.0        | v0.16.0        | Delta           |
|---------------------------------|----------------|----------------|-----------------|
| Fuzz harnesses passing          | ~52            | **409**        | +357            |
| Tasks resolved this release     | —              | ~90 (#94–#183) | —               |
| Ignored (known-gap) harnesses   | 0              | 1 (#182)       | +1              |
| Workspace source lines changed  | —              | +2,341 / −1,293| 56 files        |
| IR variants added               | —              | 5              | Substr, NewMap, HasKey, StoreParameter, LoadParameter |
| Fuzz submodules                 | 1 monolith     | 10             | +9              |

```
$ cargo test --test fuzz_tests --release
test result: ok. 409 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
```

---

## Highlights

### Canonical EVM envelopes everywhere

Every panic site — division by zero, modulus by zero, `INT256_MIN / -1`,
integer overflow, unary `-intN.min`, array OOB, `pop()` on empty, `abi.decode`
short-buffer — now routes through `ir/build/panic.rs::emit_panic`, which
emits the 4-byte selector `0x4e487b71` followed by a BE-32 panic code.
Solidity `revert("reason")` and `require(cond, "msg")` both emit the
`Error(string)` envelope. This closes Tasks #103, #107, #108, #131.

Before:

```
throw "Panic: 0x11"
```

After:

```
0x4e487b71 ‖ abi.encode(uint256(0x11))
```

### Narrow Yul + EIP-1153

`assembly { ... }` blocks now lower `mstore`/`mload`/`return`, `let x := ...`,
basic arithmetic, and `tstore`/`tload`. Transient storage uses a
per-function `NEWMAP` local with a `HasKey` guard — unset slots return 0,
matching EIP-1153 semantics. Yul identifiers also resolve to outer
Solidity function parameters via the new `StoreParameter` / `LoadParameter`
IR variants (Task #99, #100, #183).

### msg.value / msg.data EVM surface

Neo N3 has no intrinsic `msg.value`. v0.16.0 threads a per-call
`pending_value` override through `NeoRuntime::override_value` /
`override_sender_and_value`, with a dedicated syscall slot returning the
active value. `msg.data` is synthesised as `selector || abi.encode(args)`
at the start of each public-method execution (Task #112, #113).

### Fuzz suite split

The 24k-line `tests/fuzz_tests.rs` has been reorganised into 10
submodules under `tests/fuzz_tests/`:

- `baseline_tests.rs` (52 original harnesses)
- `batches_18_30.rs`, `batches_31_45.rs`, `batches_46_64.rs`, `batches_66_80.rs`
- `arithmetic_props.rs`, `compiler_props.rs`, `optimizer_props.rs`, `storage_props.rs`
- `task107_catch_panic_tests.rs`
- `common.rs` (shared helpers)

---

## Compatibility notes

The following changes alter observable behaviour for existing contracts.
Review these if you are upgrading a deployed contract:

1. **Integer overflow now reverts.** `unchecked { ... }` blocks remain
   the escape hatch. Contracts that previously relied on silent wrap at
   64-bit on `uint256` operands will now observe `Panic(0x11)`.
2. **`~uint256(x)` / wide bitwise ops no longer wrap at 64-bit.** Results
   match EVM (i.e., `~x == u256::MAX - x`). Any contract that relied on
   the broken behaviour will produce different values.
3. **`delegatecall` is hard-rejected at compile time**, not silently
   routed to `System.Contract.Call`. Migrate to a proxy contract or
   inherit the target. (Task #101.)
4. **Event `topic0` is now `keccak256(signature)`** instead of a
   pre-existing Neo-specific placeholder. Off-chain indexers that
   decoded `topic0` as a raw opaque value must switch to the EVM
   convention.
5. **`catch Panic(uint256)` / `catch Error(string)` now match correctly.**
   If your catch blocks depended on the legacy `"Panic: 0xNN"`
   ByteString shape, they will no longer match — update them to the
   EVM-canonical 4-byte-selector+abi-payload shape.
6. **`uint256(bytes32)` is a bit-identity reinterpret**, not a decode.
   Contracts that implicitly relied on the old behaviour will observe
   different numeric results.
7. **`payable(x)` is a type-only cast**, not a no-op wrap. Manifest
   output is unchanged; source-level analysis may flag different
   warnings.
8. **`block.coinbase` returns `address(0)`** (already in v0.15.0; stable in
   v0.16.0). It does NOT return `getNextBlockValidators()`.
9. **`msg.data` outside `onNEP17Payment`/`onNEP11Payment` callbacks
   produces `selector || abi.encode(args)`** (v0.15.0 change, stable in
   v0.16.0).

---

## Known limitations

- **Task #182** — `struct { T[] dyn; ... } s; return s.dyn.length;` still
  leaks the internal JSON Array shape instead of an EVM-canonical 32-byte BE
  length. Workaround: store the length in a separate `uint256` field
  or return it as a top-level expression. The corresponding fuzz
  harness is marked `#[ignore]`.

Oracle integration remains a stub (requires external oracle service) —
unchanged from v0.15.0.

---

## Upgrade guide (v0.15.0 → v0.16.0)

Most contracts need no source changes. The main things to check:

1. **Arithmetic.** Anywhere you do `a + b`, `a - b`, `a * b`, `++a`,
   `a += b` on `uint256`/`int256` (or narrow signed), confirm you
   understand the post-op range guard. Wrap explicit overflow paths in
   `unchecked { ... }`.
2. **Custom errors / try-catch.** Confirm your catch clauses match the
   canonical EVM envelope. `catch Error(string memory s) { ... }` and
   `catch Panic(uint256 code) { ... }` now work as spec'd.
3. **`delegatecall`.** Any lingering `delegatecall` needs replacement
   with a proxy pattern or target-contract inheritance. The compile
   error points at the offending call site.
4. **Yul / inline assembly.** Narrow yul is now compiled, not warned
   out. If you relied on the v0.15.0 silent-nop for `assembly { ... }`,
   switch to `// solhint-disable-next-line no-inline-assembly` or strip
   the block — the compiler now emits real code for supported
   primitives.
5. **Devpack users** — bump `@r3e-network/neo-solidity-devpack` to
   `^1.1.0`. New `Precompiles.sol` library available for EVM precompile
   routing (`ecrecover`, `sha256`, `ripemd160`, `identity`, `modexp`).

### CLI output sanity check

```
neo-solc --version
# neo-solc 0.16.0
```

---

## What ships

### Core compiler (`neo-solidity` v0.16.0)

- `src/ir/build/panic.rs` — new shared panic envelope emitter.
- `src/ir/ir_types.rs` — 5 new IR variants (`Substr`, `NewMap`,
  `HasKey`, `StoreParameter`, `LoadParameter`).
- `src/ir/statements/assembly.rs` — narrow yul lowering + transient
  storage + yul-to-parameter binding.
- `src/ir/statements/dispatch/return_revert.rs` — multi-value return,
  short-decode panic guard, dynamic-array layout, struct custom-error
  envelopes.
- `src/ir/expressions/dispatch/{binary, unary}.rs` — checked
  arithmetic for `uint256` / `int256` / narrow signed.
- `src/runtime/runtime_parts/runtime_impl/runtime/execution.rs` —
  msg.data synthesis, per-call msg.value and caller overrides,
  self-method dispatch table.
- `src/runtime/execution/execution_impl_part2_contract_call.rs` —
  self-external-call routing through the merged offsets table; virtual
  script hash derivation.
- `src/runtime/execution/helpers/{bitwise, arithmetic, comparison}.rs`
  — wide-BigInt path for all wide operands; EIP-145 shift clamping.
- `src/runtime/execution/execution_impl_part2_native/stdlib.rs` —
  canonical EVM abi.encode*/abi.decode; itoa / atoi / base64*.

### Devpack (`@r3e-network/neo-solidity-devpack` v1.1.0)

- `devpack/libraries/Precompiles.sol` — NEW. EVM precompile routing
  wrappers.
- `devpack/contracts/FrameworkBase.sol` — refreshed for the new
  compiler semantics.
- `devpack/standards/NEP17.sol`, `NEP11.sol` — updated to match the
  compiler's Event-canonical wire format.
- `devpack/contracts/NativeCalls.sol`, `devpack/libraries/{Neo, Runtime,
  Storage}.sol` — aligned with the new runtime bridge.

---

## Acknowledgments

This release was driven by a multi-week fuzz-harness audit. Every task
in this release (#94–#183) resolves a finding raised by an automated
harness, which is documented in the source via `// Task #NNN — …`
comments for future archaeology.

Thanks to everyone who reviewed diagnostics, staged fixes, and kept the
runtime bridge honest while the compiler caught up with the spec.

---

## Links

- [CHANGELOG.md](./CHANGELOG.md) — detailed per-task log.
- [Release notes on GitHub](https://github.com/r3e-network/neo-solidity/releases)
- [Documentation site](https://docs.r3e.network/neo-solidity)
- [SOLIDITY_SUPPORT_MATRIX.md](./docs/SOLIDITY_SUPPORT_MATRIX.md)
