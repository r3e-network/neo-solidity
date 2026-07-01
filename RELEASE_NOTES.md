# neo-devpack-solidity v0.26.0 — neo-test Dev Environment & Deep Correctness

**Release date:** 2026-07-01
**Compiler / CLI / workspace:** **v0.26.0**
**devpack (`@neo-devpack-solidity/contracts`):** **v0.26.0**
**Target Neo N3 node:** **v3.10.0** (Gorgon-prep; no hardfork activation)

> Canonical change list: [`CHANGELOG.md`](./CHANGELOG.md) `[v0.26.0]`.
> Previous releases: see the
> [GitHub releases page](https://github.com/r3e-network/neo-devpack-solidity/releases)
> and `CHANGELOG.md` history (previous: v0.25.0).

---

## TL;DR

- **`neo-test`** — a native, Foundry-style Solidity **test runner** that
  executes `test*` / `testFail*` / `setUp()` directly on the in-tree
  NeoVM, with per-test state isolation, cross-contract `new`, decoded
  `revert` / `Panic` reasons, `console.log` and gas reporting. Bundles
  `neo-std/{Test,console,Vm}.sol`, wired into `neo-forge test`. See
  [`docs/TESTING.md`](./docs/TESTING.md).
- **Foundry cheatcodes** — `vm.prank` / `startPrank` / `stopPrank` /
  `warp` / `roll` / `deal` / `label` / `assume` and `vm.expectRevert()`
  (via the HEVM address; no compiler change).
- **A large correctness wave**, dogfooded through `neo-test` and
  cargo-fuzz/proptest: arithmetic wrap/overflow, `bytesN` byte-order at
  every binding site, storage aliasing & deep-copy, low-level `.call`
  returndata, ABI encode/decode fidelity, event/inheritance merge, and
  `try/catch` semantics — **1952 tests across 54 targets, 0 failures.**

---

## 🧪 The headline: `neo-test`

Neo now has a first-class, Foundry-shaped testing story that runs on the
same NeoVM the compiler targets — no external node required for the inner
loop:

- Write tests in Solidity: `function testTransfer() public { ... }`,
  `testFail*` for expected-revert, `setUp()` for per-test fixtures.
- **Per-test state isolation** and **cross-contract `new`** so multi-
  contract flows (Vault/deposit, factory patterns) work out of the box.
- **Decoded failures** — `revert("msg")`, `require`, custom errors and
  `Panic(0xNN)` come back as human-readable reasons, not raw bytes.
- **`console.log` / `console.logBytes`**, **gas accounting**, and
  **value-rich assertions** (`assertEq failed: 3 != 5`).
- **Cheatcodes** through the standard HEVM address: `vm.prank`,
  `startPrank`/`stopPrank`, `warp`, `roll`, `deal`, `label`, `assume`,
  and `vm.expectRevert()`.

`neo-forge test` now delegates to `neo-test`. The bundled `neo-std`
(`Test.sol`, `console.sol`, `Vm.sol`) ships in-tree via `include_str!`.

---

## 🔧 Correctness fixes (highlights)

Grouped; full list in [`CHANGELOG.md`](./CHANGELOG.md) `[v0.26.0]`. Many
were found by dogfooding `neo-test` and by the fuzz harness, then
verified against a real `neoxp` node.

- **Arithmetic** — unchecked `-type(intN).min` / `0 - min` now WRAP
  two's-complement (previously produced the out-of-range `+2^(N-1)`,
  which also faulted NeoVM's 256-bit integer); `addmod(a,b,0)` /
  `mulmod(a,b,0)` Panic(0x12); `addmod` full-width carry; `int256`/
  `uint256 **` via soft-arith magnitude (catchable Panic / wrap, no
  33-byte fault); `mapping(K => uintN)` value width.
- **`bytesN` / byte semantics** — integer-backed `bytesN` literals are
  big-endian at *every* binding site (scalar, struct field, array push,
  mapping key, call arg, return, `constant`, multi-return tuple, indexed
  event topic); `bytesN` bitwise read-back via `uint256(...)`; `b[i]` is
  a `bytes1`; `intN(bytesN)` endianness; `bytesN` constant comparison
  (fixed a byte-reversed access-control check).
- **Storage** — storage `struct` passed by reference to an internal
  function aliases the slot; `T[] memory m = storageArr` deep-copies;
  storage `bytes` element write `data[i] = v` persists; `bytes`
  push/pop.
- **ABI & low-level calls** — internal call to a public array/`bytes`/
  `string` returner decodes the ABI blob; low-level `.call` returndata
  is the raw EVM ABI envelope (no Neo serialize framing), a no-reason
  revert is empty, and `(bool ok, )` reports `false` on revert; strict
  `abi.decode` type fidelity; `abi.encode`/`encodePacked` `ByteString`;
  partial tuple destructure from a cross-contract call.
- **Events / inheritance / merge** — a contract keeps its own event
  declarations when another references it by type; a `new`-deployed
  contract's public method reaches its own concrete internal helper
  (was dropped → `PUSH0` → write lost); cross-contract `msg.sender` for
  `new`-deployed callees.
- **`try/catch` & control flow** — bounded `TRY` nesting (VM-DoS fix) +
  uncatchable `ExecutionEngineLimits`; `catch Panic`/`catch Error`
  matching against real-node faults; balanced `TRY` frame on every catch
  exit; modifier/base-constructor args evaluated exactly once.
- **Type resolution & devpack** — integer-literal overload resolution;
  `address.balance`/`.code`/`.codehash` inference; NeoVM `EQUAL`
  type-strictness; devpack `multiSigMint` authorization, stake-reward
  settlement, NEP-11 oracle/curation/minter-royalty paths.

### Known limitations (tracked for a follow-up)

A handful of deep sim-fidelity / model-level items are documented but not
yet fixed: `abi.encodeWithSignature` used as raw bytes vs. the Neo
method-name `.call` optimization; `abi.encode` of fixed-size arrays
(`uintN[K]`) using dynamic layout; nested struct-array-member storage
keys; inherited-getter dispatch and inherited base-constructor state when
a contract is instantiated via `new` in the in-tree simulator.

---

## 📊 Headline numbers

| Metric | v0.25.0 | v0.26.0 |
| --- | --- | --- |
| Native Solidity test runner | — | **`neo-test`** |
| Foundry cheatcodes | — | **9** (prank/warp/roll/deal/label/assume/expectRevert/…) |
| Test suites green | 47 | **54** |
| Total tests | 1,488 | **1,952** |
| Test failures | 0 | **0** |
| Backlog compiler bugs fixed this cycle | — | **30+** |

---

## 🚀 How to upgrade

Backwards-compatible release — no public-API breaks or renames. Cargo and
npm users pick up `v0.26.0` on their next dependency resolution.

**CLI users**: rebuild with `cargo install --path . --version 0.26.0`
or download the prebuilt binary from the
[release page](https://github.com/r3e-network/neo-devpack-solidity/releases/tag/v0.26.0).

**Library users** consuming `neo_devpack_solidity` from another crate:
pick up `0.26.0` via `cargo update`.

**devpack users**: `npm install @neo-devpack-solidity/contracts@0.26.0`.

**Test authors**: write Solidity tests and run `neo-test <file>.sol`
(or `neo-forge test`). See [`docs/TESTING.md`](./docs/TESTING.md).

---

## 🧪 Verification

- `cargo test --release` ✅ **1,952 passed / 54 targets / 0 failed**
  (integration + lib unit + cargo-fuzz/proptest)
- `cargo fmt --all -- --check` ✅
- `cargo clippy` ✅
