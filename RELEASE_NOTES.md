# neo-devpack-solidity v0.20.0 + devpack v2.0.0 — Correctness & Conformance Release

**Release date:** 2026-06-11
**Compiler / CLI / workspace:** **v0.20.0**
**devpack (`@neo-devpack-solidity/contracts`):** **v2.0.0** (breaking — see below)

> Canonical change list: [`CHANGELOG.md`](./CHANGELOG.md) `[v0.20.0]`.
> Previous releases: see the
> [GitHub releases page](https://github.com/r3e-network/neo-devpack-solidity/releases)
> and `CHANGELOG.md` history.

---

## TL;DR

- **A full adversarial review pass fixed 26 independently verified
  correctness findings** across ABI encoding/decoding, storage soundness,
  manifest generation, devpack APIs, and compiler robustness — every
  finding reproduced against the compiler before fixing, every fix covered
  by regression tests.
- **Token contracts are now ecosystem-conformant on real Neo N3.**
  NEP-17/NEP-11 `Transfer` events emit **native NEP notifications**
  (`[from, to, amount(, tokenId)]`, zero address → `Null`) that wallets,
  explorers, and trackers can read; all other events declare their true
  EVM wire shape so they pass HF_Basilisk notification validation
  (previously **every `emit` faulted on nodes ≥ 3.6**).
- **NEP-11 deep conformance:** ByteString token IDs (≤ 64 bytes),
  `tokensOf`/`tokens` return real NeoVM iterators (manifest
  `InteropInterface`, like the official C# devpack), `data` parameters are
  `Any` — the devpack NEP-11 base now **passes neo-express's strict
  deploy-time NEP-11 standard check**, validated on-chain end-to-end
  (deploy → mint → native Transfer notification → iterator traversal).
- **The devpack tells the truth.** 111 documented-but-uncallable library
  functions, the fictional `Storage.*Local` API (would FAULT on every real
  node), silently miscompiling helpers (`batchPut` → single `Put`, …), and
  the misleading `hasRole` are gone; a probe test keeps the intrinsic
  whitelist and the compiler's lowerings in lockstep so this class of gap
  cannot reappear.
- **Manifests are sound:** permissions cover `catch`-handler calls,
  `safe`-flag analysis follows function pointers, NEP standards are only
  claimed (or accepted, when declared) on real conformance, distinct-arity
  overloads keep their Solidity names (ERC-20 `transfer(to, amount)` is
  callable), and the NEP-17 `data` parameter is manifest type `Any`.
- **Compiler robustness:** no more OOM-abort on huge constant folds at
  default `-O2`, no more parser stack overflow on deeply nested input.
- **Structural cleanup:** the dead Yul frontend (lexer/parser/semantic/
  optimizer — never used by the real solang-based pipeline) and
  `.dead_modules/` are deleted; all 156 `include!()` fragments under
  `src/runtime` are real Rust modules now.

## Verification

- `cargo test`: **1,758 tests, 0 failures** (unit, e2e, conformance,
  differential, property/fuzz harnesses).
- `cargo clippy --all-targets`: clean. `cargo fmt --check`: clean.
- **23/23 neo-express on-chain smoke tests** (deploy, constructor args,
  CALLT, permissions, low-level/external calls, structs, upgrade
  lifecycle, witness guards, oracle relay).
- New on-chain validations this release: committed transactions show
  native `Transfer` state `[from|Null, to, amount(, tokenId)]`; custom
  events HALT under Basilisk notification checks; `tokens()`/`tokensOf()`
  return `InteropInterface` stack items on a real chain.

## Breaking changes

- **devpack v2.0.0**: `Runtime.sol`/`Storage.sol`/`Neo.sol` pruned to the
  intrinsic surface the compiler actually lowers; `Storage.*Local` and
  `hasRole` removed; NEP-11 token IDs are `bytes` (was `bytes32`),
  `tokensOf`/`tokens` return `Syscalls.Iterator` (was `bytes32[]`), and
  `data` parameters are `Any`. Existing NEP-11 deployments using the old
  base need a storage migration on upgrade.
- **Event wire format**: standard-signature `Transfer` events emit native
  NEP notifications; all other manifests now declare the EVM wire shape.
  Anything pinned to the old (faulting) manifest declarations must
  recompile.
- **Overload naming**: distinct-arity overloads keep their original names
  in the manifest; callers using mangled names like
  `transfer(address,uint256)` must migrate on recompile.
- **Rust API**: public modules `lexer`, `parser`, `semantic`, `optimizer`,
  `codegen` removed; `interop_id_bytes` moved to
  `neo_devpack_solidity::interop`.

## Known limitations

- NEP-11 `properties` returns serialized `bytes` (manifest `ByteArray`),
  not the spec's `Map` — Solidity has no construct producing a NeoVM Map
  return. Documented in `devpack/standards/STANDARDS_MAPPING.md`.
- The bundled emulator remains a development tool, not consensus-grade:
  see `docs/NEO_VM_PARITY_TODO.md` (gas approximation, exception-unwinding
  parity). neo-express smoke tests are the real-chain ground truth.
- `include!()`-based file structure remains outside `src/runtime`
  (`src/ir`, `src/cli`, `src/frontend`); same modularization is a planned
  follow-up.
