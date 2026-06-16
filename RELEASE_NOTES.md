# neo-devpack-solidity v0.21.0 + devpack v2.1.0 — Deep Correctness & Conformance Release

**Release date:** 2026-06-16
**Compiler / CLI / workspace:** **v0.21.0**
**devpack (`@neo-devpack-solidity/contracts`):** **v2.1.0**

> Canonical change list: [`CHANGELOG.md`](./CHANGELOG.md) `[v0.21.0]`.
> Previous releases: see the
> [GitHub releases page](https://github.com/r3e-network/neo-devpack-solidity/releases)
> and `CHANGELOG.md` history (previous: v0.20.0 / devpack v2.0.0).

---

## TL;DR

- **`uint256` arithmetic is now conformant for values `>= 2^255`, end to
  end.** Software 256-bit limb routines implement add/sub/mul, checked
  overflow, unsigned compare, divmod, and logical/wrapping shifts entirely
  in `<= 32`-byte NeoVM operations, so a value like `type(uint256).max` no
  longer forms a rejected 33-byte integer and no longer reinterprets as
  negative. `type(uint256).max < 5 == false`, `type(uint256).max >> 1`
  gives `2^255-1`, unchecked `**`/`+=`/`++` wrap mod `2^256`, and checked
  ops Panic(0x11) — matching a real node, with `int256` left on native
  signed operations. The runtime simulator was made faithful to a 32-byte
  two's-complement NeoVM so it can actually validate this class of behavior.
- **Two full adversarial passes landed.** A 25-defect production audit (24
  fixed) and a 12-subsystem systemic best-practice review (33 findings: 32
  fixed, 1 surfaced as a loud compile-time warning). Every defect was
  reproduced against the compiler/runtime/devpack before fixing and is
  covered by regression tests.
- **The ABI codec now covers the two shapes that previously fell back to
  Neo-native bytes.** Nested-dynamic types (`string[]`, `bytes[]`, `T[][]`)
  and dynamic structs (any struct with a dynamic field) encode and decode
  with the full recursive EVM head/tail layout, so cross-contract calldata
  and return-data are byte-exact with EVM/solc. `abi.decode` now follows the
  encoded head offset instead of assuming `0x20`, accepts over-length
  buffers, and faults Panic `0x41` on non-canonical (high-bit-set)
  length/offset slots.
- **Selectors and manifests are canonical.** Selectors, `.selector`, and
  event `topic0` are computed from EVM-ABI-canonical types (struct → tuple,
  enum → `uint8`, `uint` → `uint256`); standard-JSON `methodIdentifiers`,
  `methodMap`, and the `abi` array now emit decodable `tuple`/`components`
  shapes; `ErrorName.selector` hashes the parametrized signature; fixed-size
  arrays (`uint256[3]`) and `bytes20` carry their true ABI type; struct
  overloads colliding to the same tuple shape now error.
- **Runtime VM fidelity improved across stack, item, and storage limits.**
  Global and per-collection `MaxStackSize` (2048), `MaxItemSize` (65535) on
  `NEWBUFFER`/`CAT`, and Neo N3 `Storage.Put` key/value size limits now
  fault exactly where a real node would; minimal `CONVERT` Integer→bytes
  encoding, corrected `PACKMAP` pop order, `MODPOW(-1)` modular inverse, and
  a `SUBSTR` overflow guard close further simulator/real-node gaps.
- **Dispatch is correct and fails loud.** Same-arity overloads resolve by
  argument type instead of insertion order; unresolved member calls,
  return-tuple arity mismatches, and `>255`-local functions are now hard
  compile errors instead of silent mis-dispatch / zero-returns / truncated
  slot indices; function-pointer locals dispatch via `CALLA` (and revert
  cleanly when zero-init).
- **Devpack fund-safety hardened.** NEP-17 self-escrow no longer faults the
  escrow-in leg, zero-amount transfers conform to NEP-17, an
  owner-authorized signer quorum gates the escrowed pool, the oracle
  conditional-transfer callback no longer strands funds on a short result or
  a mis-ordered signature, `multiSigBurn` requires the holder's witness, and
  NEP-26 `onNEP11Payment` takes dynamic `bytes` token IDs.
- **Tooling is more honest.** Standard-JSON parse failures emit one
  structured `ParseError` per diagnostic with byte ranges, aliased imports
  warn (`IMPORT_ALIAS_BY_NAME`), the disassembler decodes the `CALLT`,
  `CONVERT`, and `ISTYPE` operands, and the compiler warns when a literal
  exceeds NeoVM's 32-byte integer limit.

## Verification

- `cargo test`: full unit, e2e, conformance, differential, and
  property/fuzz suites green — including new property tests pinning the
  software 256-bit add/sub/mul/divmod/shift/compare routines against a
  reference 256-bit model across the `[2^255, 2^256-1]` range, and ABI
  round-trip tests for nested-dynamic and dynamic-struct encode/decode.
- `cargo clippy --all-targets`: clean. `cargo fmt --check`: clean.
- neo-express on-chain smoke tests remain the real-chain ground truth
  (deploy, constructor args, CALLT, permissions, low-level/external calls,
  structs, upgrade lifecycle, witness guards, oracle relay); the native NEP
  Transfer / iterator / Basilisk-notification validations from v0.20.0
  continue to pass.
- Every fixed defect from both review passes ships with a regression test;
  the two surfaced-but-not-silenced items (the `uint256` literal
  representation warning and the `IMPORT_ALIAS_BY_NAME` warning) are
  asserted as warnings rather than hard failures.

## Breaking changes

- **devpack NEP-26 receiver shape.** `INEP26Receiver.onNEP11Payment`
  tokenId is now a dynamic `bytes` (was `bytes32`), matching
  `INEP11Receiver` and the ByteString token IDs the NEP-11 base passes. A
  contract implementing the old `bytes32` signature must update its
  parameter type to be invoked correctly; a non-32-byte id no longer
  mis-encodes.
- **Standard-JSON error shape.** Parse failures now emit one
  `type:"ParseError"` entry per diagnostic, each with
  `sourceLocation:{file,start,end}` byte offsets, instead of a single
  `type:"Generic"` blob. Tooling that pattern-matched the old single
  `Generic` error must read the per-diagnostic `ParseError` array.
- **New hard compile errors.** Unresolved member calls (`inner.member(args)`
  matching no resolution branch), functions exceeding NeoVM's 255 local-slot
  limit, explicit return-tuple arity mismatches, same-tuple-shape struct
  overloads, and out-of-range `bytesN` (`bytes0`, `bytes33`+) now fail the
  build. These previously compiled silently (member calls returned `0` and
  dropped arguments; `>255` locals truncated slot indices; mismatches were
  warnings), producing NEFs that mis-dispatched or contradicted their own
  ABI. Code relying on those silent behaviors must be corrected.
- **`bytes20` and fixed-size-array selectors changed.** `bytes20` now
  canonicalizes to `bytes20` (was `address`) and `uint256[3]`-style
  parameters now canonicalize to `T[N]` (was `T[]`), so the keccak selector,
  `interfaceId`, and `abi.encodeWithSelector` payload for any function taking
  these types change to the solc/ethers-correct values. Off-chain callers
  hard-coding the old (incorrect) selectors must recompute them.
- **`msg.value` now lowers to `0`.** Neo N3 has no EVM-style attached call
  value; the prior `System.Runtime.GetMsgValue` syscall FAULTed on a real
  node. Contracts that read `msg.value` must instead take received amounts
  from the `amount` argument of `onNEP17Payment`/`onNEP11Payment`.
- **Aliased imports warn.** `import {A as B}` and `import * as NS` now emit
  an `IMPORT_ALIAS_BY_NAME` warning (resolution and codegen unchanged), since
  aliases resolve by the underlying symbol name and a name collision in the
  import closure could bind to the wrong declaration. Builds treating
  warnings as errors must allowlist this code.

## Known limitations

- `uint256` software arithmetic is scoped to genuinely-typed `uint256`;
  signed `int256` deliberately keeps native NeoVM operations (`-x-1` NOT,
  arithmetic shift, signed divmod), so the two integer widths follow
  different code paths by design.
- The bundled emulator, now made faithful to a 32-byte two's-complement
  NeoVM with the stack/item/storage-limit checks above, remains a
  development tool rather than consensus-grade: gas is approximated and
  exception-unwinding parity is partial (see
  `docs/NEO_VM_PARITY_TODO.md`). neo-express smoke tests are the real-chain
  ground truth.
- Modular-arithmetic sign conventions follow NeoVM's C#-style truncated
  remainder (`MODMUL`/`MODPOW`, `%`), which differs from the Euclidean
  non-negative form some EVM tooling assumes for negative operands; this is
  intentional NeoVM parity, not an EVM match.
- The minimal `CONVERT` Integer→ByteString/Buffer encoding is scoped to the
  Solidity-observable CONVERT path; the internal storage/map-key byte helper
  keeps its own fixed encoding, so the two are not interchangeable at the
  byte level.
