# Changelog

All notable changes to the Neo Solidity Compiler will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.17.0] - 2026-04-25

Fuzz-system overhaul release driven by 11 sequential review waves.
**912 proptest cases pass** (was 743), **11 cargo-fuzz targets** (was 6, all
crash-free), **22 bug fixes total** (was 14), plus **one latent allocation-DoS**
hardened proactively. Coverage: **68.42% region / 67.42% line** via
`cargo-llvm-cov`.

### Added

- **5 new cargo-fuzz targets** (was 6, now 11):
  - `fuzz_target_runtime_exec` — feeds arbitrary bytes as NeoVM bytecode
    against a bounded `NeoRuntime` (gas + memory capped). Surfaced bug #15
    (NEWARRAY OOM DoS) on the first run.
  - `fuzz_target_method_tokens` — drives CALLT (0x37) dispatch under
    randomly-generated NEF method-token tables, exercising the
    `invoke_native_contract` path under attacker-controlled token metadata.
  - `fuzz_target_nef_manifest_mutation` — bit-flips, byte-writes,
    insertions, and deletions on valid NEF / manifest seeds (53 NEF seeds,
    24,296 manifest seeds + embedded fallback). Reaches deeper parser code
    paths than pure-random fuzzing.
  - `fuzz_target_structured_sol` — `arbitrary`-driven Solidity AST grammar.
    This release extended the grammar with **inheritance** (multi-base
    linearisation, abstract / `virtual` / `override`), **`using <Lib> for
    <Type>`**, **try/catch**, **revert("string")**, and
    **require(cond, "string")**. Coverage rose from ~8,200 PCs at the
    initial baseline to **11,190 PCs** (+36%) after grammar extensions.
  - `fuzz_target_yul_assembly` — `arbitrary`-driven Yul AST inside
    Solidity `assembly { … }` blocks (let/assign/if/for/switch/break/
    continue + mload/sload/mstore/sstore/return/revert). Drives
    `src/ir/statements/assembly.rs` lowering through random opt-levels.
- **20+ new proptest modules** (912 cases vs 743 baseline):
  `arithmetic_helpers_props`, `compile_runtime_roundtrip`,
  `conditional_jumps`, `contract_upgrade_props`,
  `convergence_props`, `custom_error_envelope_props`,
  `determinism_props`, `devpack_props`, `differential`,
  `disasm_stability_props`, `modifier_rewrite_props`,
  `native_contract_props`, `native_resolver_props`,
  `openzeppelin_patterns_props`, `optimizer_props`,
  `reentrancy_props`, `stdlib_native_props`,
  `storage_iterator_stress`, `storage_state_machine`.
- **Differential testing**: hash differentials extended to 4096-byte inputs
  + boundary-byte patterns (`0x00*N`, `0xFF*N`, alternating); BLS12-381
  G1/G2 add+mul + pairing equality vs `bls12_381` reference crate (gated
  on runtime-implementation arrival); manifest event parameter type
  fidelity vs spec.
- **Coverage analysis**: `cargo-llvm-cov` integrated; coverage gaps
  documented (see Known Limitations / Dead Code in
  `FUZZ_STATUS_REPORT.md`).
- **State-machine tests**: storage iterator stress against a `BTreeMap`
  reference model; 50-call gas-stability sweep on the same `NeoRuntime`
  (no per-call linear-growth from leaks); recursive Fibonacci(10)
  exercising 177 self-external `System.Contract.Call` frames cleanly.

### Fixed

- **#15 — Host DoS via NEWARRAY/NEWARRAY_T/NEWSTRUCT/NEWBUFFER**: a 6-byte
  bytecode `02 ff 00 0c 17 c6` (PUSHINT32 + NEWSTRUCT) requested ~387M
  Null items via `Vec::with_capacity(count)` and OOM-aborted the host
  process before gas accounting could fire. Fixed by bounding `count *
  size_of::<StackItem>()` against `memory_limit` in
  `src/runtime/execution/collections/construction.rs` and `len` against
  `memory_limit` in `src/runtime/execution/execution_impl_part3_bytes.rs`
  (mirrors PUSHDATA4 guard). Regression pinned by `batch132*`.
- **#17 — Host DoS via Storage.Put**: `RuntimeConfig::storage_limit` had
  no consumer (`grep -rn storage_limit src/` returned the declaration and
  default only, zero readers). Storage.put / Local.Put inserted into
  `storage_overlay` with no size check. Fixed by adding `storage_limit`
  to `ExecutionContext`, propagating from config, and adding
  `enforce_storage_limit` (cumulative-bytes guard) on every Put.
- **#18 — Host DoS via Storage.Find**: `helpers/storage.rs::
  build_storage_entries` queried with `limit: None` and used
  `Vec::with_capacity(entries.len())`; an attacker who populated storage
  with many keys (10K writes fit in default gas_limit) could OOM the
  host with one `Find` syscall. Fixed by bounding `StorageQuery::limit`
  against `storage_limit / MIN_ENTRY_BYTES`, plus a post-merge cap to
  cover overlay-only matches.
- **#19 — Gas DoS via flat per-syscall pricing**: `Storage.Put` charged
  a flat 1000 gas regardless of value size; `CryptoLib.sha256/keccak256/
  ripemd160/sha1/murmur32` charged a flat 512 gas regardless of input
  length. Fixed by `syscall_extra_input_gas` (in
  `src/runtime/execution/instruction/syscall.rs`) — peek-and-charge
  before the handler with `STORAGE_PUT_PER_BYTE_GAS=100` and
  `HASH_PER_BYTE_GAS=50` on top of the flat base. CALLT path mirrored at
  `src/runtime/execution/instruction/flow/calls.rs`. Regression pinned by
  `batch133a–d`.
- **#20 — Gas DoS via CheckMultisig**: ran `O(N·M)` secp256k1 verifies
  (up to 4096 verifies for 64×64 input) for a single flat 1000-gas
  charge. Fixed with `CHECKMULTISIG_PER_VERIFY_GAS=1000` charged upfront
  as `pub_count * sig_count * per_verify` (saturating).
- **#21 — Correctness: `StdLib.atoi("--42", 10)` returned 42**:
  `i128::from_str_radix(body, radix)` accepts a leading `-` in `body`
  after the outer `neg` flag was already set, so a double-negative
  becomes a positive number. Fixed by switching magnitude parser to
  `u128::from_str_radix` so any sign character in `body` (`-`, `+`,
  anything else non-digit) yields 0.
- **#22 — Correctness: `StdLib.base64Decode("AB=C")` returned 3-byte
  garbage**: the decode loop accepted `=` at any position inside a 4-char
  chunk and treated it as a literal-zero sextet, ignoring RFC 4648's
  mid-chunk-pad-rejection rule. Fixed: `base64_decode` now rejects `=`
  outside the trailing-pad position of the FINAL chunk.
- **Preventive harden** for `read_input_slice` (calldata reader) — same
  bug-#15 shape (`vec![0u8; length]` with no memory_limit check). Not
  opcode-reachable today, but the methods are public; future opcode
  wiring would inherit the OOM. Now propagates an error if `length >
  memory_limit`.

### Known Limitations

- **Bug #16 — uint256 unchecked-arith narrow-i64 divergence (deferred)**:
  the `optimizer_four_level_differential_random_expr` proptest surfaced a
  real divergence between unoptimized and optimized lowerings of
  unchecked uint256 arithmetic. The naïve fix (force-widen unchecked
  uint256 BinaryOps to 32-byte ByteArray so the runtime's BigInt path
  runs) breaks tests that pass `uint256.max` as a 32-byte ByteArray
  because `coerce_item_to_bigint` treats it as **signed**. Resolving
  properly requires plumbing an unsigned-LE BigInt path through
  `cmp_needs_bigint_path`, `bigint_to_stack_item`, and the wide
  arithmetic helpers — out of scope for this release. Mitigation:
  `dexpr_strategy` literals + `a/b/c` test args bounded to `0..=u16::MAX`.
- **BLS12-381 runtime stub**: the compiler is fully wired (IR resolver
  whitelists 13 method names, devpack exposes `bls12381Add/Mul/Pairing`
  + EVM-precompile shims), but
  `src/runtime/execution/execution_impl_part2_native/crypto.rs::invoke_
  native_cryptolib` does not implement the BLS handlers — every
  `bls12381*` falls through to `StackItem::Null`. Contracts using BLS
  compile cleanly but silently return zero. 4 differential proptests are
  gated to auto-activate when the runtime implementation lands.
- **State batch non-atomic semantics ambiguity**:
  `src/runtime/state/impl/batch.rs:25` does `let _ =
  self.apply_change(change)` in non-atomic mode (silent error discard).
  Could be intentional ("skip invalid, keep going") or a bug ("stop
  after first invalid"). Pinned by an ignored test
  (`state_batch_non_atomic_partial_apply_spec`) so the choice can be
  decided in a future release.

### Dead Code (deletion candidates, not removed in this release)

`cargo-llvm-cov` analysis identified ~6,200 LOC across the codebase that
is **0% covered AND has zero importers** anywhere in `src/` or `tests/`:
- Orphaned standalone Yul frontend (~4,500 LOC):
  `src/{lexer,parser,codegen,semantic,optimizer}/`, `src/error.rs`,
  `src/types.rs`. The Yul-in-Solidity surface (`assembly { … }`) reaches
  its own active lowering at `src/ir/statements/assembly.rs`.
- VM bridge layer (~1,700 LOC): `src/runtime/bridge/`. `VMBridge` is
  constructed only inside its own subtree.
- Top-level orphans: `src/{security,warning,validation,testing}.rs`.

Removing these would push real coverage well above 90%. Listed for
awareness; pending an explicit cleanup decision.

### Test counts

| Suite | v0.16.0 | v0.17.0 | Δ |
|-------|--------:|--------:|--:|
| fuzz_tests (proptest) | 743 | **912** | +169 |
| runtime_state_batch_tests | — | **2** | +2 |
| lib unit tests | 508 | **508** | — |
| conformance_tests | 40 | **40** | — |
| e2e_compilation_tests | 80 | **80** | — |
| **Total** | 1,371 | **1,542** | **+171** |
| cargo-fuzz targets | 6 | **11** | +5 |
| Fixed bugs | 14 | **22** | +8 |

## [v0.16.0] - 2026-04-19

Major stability release driven by continuous fuzz-harness review (Tasks #94–#183).
Approximately 90 distinct fixes landed across compiler, IR lowering, runtime,
ABI encoding, arithmetic, modifiers, yul, try/catch, and storage. The compiler
now passes **409 fuzz harnesses** (0 failed, 1 ignored) covering the full
Solidity 0.8.x feature matrix plus Neo N3 integration.

### Added

- **Narrow Yul support (Task #99, #100, #183)**: inline `assembly { ... }`
  blocks now lower `mstore`/`mload`/`return` along with `let`/`:=` bindings,
  basic arithmetic, and reference semantics. `tstore`/`tload` implement
  EIP-1153 transient storage backed by a `NEWMAP` local (with `HasKey`
  guard for unset slots returning `0`). Yul identifiers now also resolve
  to outer Solidity function parameters via the new `StoreParameter` /
  `LoadParameter` IR variants (Task #183).
- **EIP-1153 transient storage in Yul (Task #100)**: `tstore(slot, value)` /
  `tload(slot)` lowers to per-function transient map ops.
- **New IR variants**: `Substr` (Task #95 bytes slicing), `NewMap` /
  `HasKey` (Task #100 yul transient storage), `StoreParameter` /
  `LoadParameter` (Task #156 parameter writes, Task #183 yul parameter
  binding).
- **`msg.value` host injection (Task #113)**: `NeoRuntime::override_value` /
  `override_sender_and_value` thread a per-call msg.value override through
  the runtime; a dedicated syscall slot in `execution/syscalls/runtime.rs`
  returns the active value. Neo N3 has no intrinsic `msg.value`, so this
  gives Solidity tests a faithful EVM-style calldata surface.
- **`msg.data` synthesis (Task #112)**: public-method dispatch now
  synthesises `selector || abi.encode(args)` and threads it to the
  executing frame so the `msg.data` builtin returns the EVM-canonical
  calldata payload. `bytesN(arg)` / `address(arg)` cast leaves in the
  synthesised path are left-aligned (Task #112 refinement) so they match
  the on-wire encoding.
- **Canonical `Panic(uint256)` envelope (Task #107, #108)**: ALL
  panic sites (div-by-zero, mod-by-zero, INT256_MIN/-1 divide, int
  overflow, unary negate overflow, array OOB, `pop()` empty, `abi.decode`
  short-buffer) now route through the shared `emit_panic` helper which
  emits the 4-byte selector `0x4e487b71` followed by the BE-32 code (e.g.
  `0x11`, `0x12`, `0x32`, `0x41`). Replaces the earlier ad-hoc
  `"Panic: 0xNN"` ByteString payloads so `catch Panic(uint256)` clauses
  can match against the canonical EVM shape.
- **Canonical `Error(string)` / `revert` envelope (Task #131)**: `require`
  with a string literal and `revert("reason")` now both emit the EVM
  `Error(string)` 4-byte selector + abi-encoded string, aligning
  `require` with `revert` payload shape.
- **Delegatecall hard-reject (Task #101)**: `target.delegatecall(data)` is
  now rejected at IR lowering with a precise diagnostic directing users
  toward proxy patterns or target-contract inheritance. Previously the
  call silently routed to `System.Contract.Call`.
- **Bytes slicing (Task #95)**: `b[start:end]` on `bytes` / `bytes memory`
  / `bytes calldata` values lowers via the new `Substr` IR instruction to
  a contiguous ByteString.
- **Write-to-parameter support (Task #156)**: Solidity permits assigning
  to function parameters directly. The new `StoreParameter` IR variant
  lowers via NeoVM `STARG0..6`, and tuple swaps of the form
  `(a, b) = (b, a)` on function parameters now work. Compound
  assignments (`a += 1;`) on parameters are also handled.
- **Narrow signed integer checked arithmetic (Task #154)**: Add/Sub/Mul on
  `int8`/`int16`/`int32`/`int64`/`int128` now emits a post-op range
  guard against `[-(2^(bits-1)), 2^(bits-1)-1]` and routes overflow
  through `Panic(0x11)`.
- **357-test fuzz suite**: 357 new fuzz harnesses land in
  `tests/fuzz_tests/batches_*.rs` (in addition to the baseline 52),
  bringing the total to **409 passed / 0 failed / 1 ignored** on the
  `fuzz_tests` binary.
- **StdLib native coverage (Task #51)**: `itoa`, `atoi`, `base64Encode`,
  `base64Decode` now implemented in `execution_impl_part2_native/stdlib.rs`.
- **Dynamic-array encoding (Task #121, #137)**: `T[]` returns and
  `abi.encode(T[])` / selector-side args emit the EVM-canonical
  `offset || length || BE-32 elements` layout.
- **Struct flattening for `abi.encode` (Task #124)**: whole-struct args
  are expanded into per-field stack items so the `AbiEncode` builtin
  classifies them as static types when appropriate; includes a new
  `try_flatten_struct_arg_for_abi_encode` pattern.
- **Custom-error struct envelopes (Task #181)**: `revert CustomErr(struct)`
  positional and named-args forms flatten struct args into the ABI
  tuple and render struct type strings via the canonical EVM
  `(T1,T2,...)` shape.
- **Sticky caller override (Task #176)**: `pending_caller_account` now
  re-arms from a sticky slot so caller overrides survive across
  self-external calls.
- **`virtual caller` script hash (Task #123)**: per-frame
  `msg.sender` override pushed by self-external dispatch, deterministic
  virtual script hash derivation, resolved in the `GetCallingScriptHash`
  syscall.
- **Self-method dispatch table (Task #70)**: manifest-derived
  `(method_name, offset, arg_count)` table installed on each execute,
  enabling `this.someFn()` without cross-contract syscall overhead.

### Changed

- **Event topic0 = keccak256(signature) (Task #39, pre-landed)**:
  `emit` lowering now produces the EVM-canonical log shape
  (`topics[0]` is the keccak256 of the event signature, with indexed
  args taking subsequent topic slots; non-indexed args go to `data`).
- **Try/catch envelope matching (Task #103, #86)**: each catch clause
  uses a shape-specific guard (`Error(string)`, `Panic(uint256)`,
  named custom error, wildcard). Raw stack-top bytes are preserved
  through `try_frames.rs` so the catch handler receives the EVM
  envelope verbatim.
- **Override sticky semantics (Task #105)**: `default_timestamp` pinned
  to `1_704_067_200` (2024-01-01T00:00:00Z). Pending metadata overrides
  (timestamp, index, sender) are now snapshotted BEFORE the user
  method observes them and drained after the call.
- **Require → Error(string) envelope (Task #131)**: `require(cond, "msg")`
  with a string literal now emits the same EVM `Error(string)` shape as
  `revert("msg")`.
- **Storage reference handling (Task #117)**: `resolve_storage_reference`
  widened to recognise Array- and Mapping-typed state variables as
  storage pointers when the base has no `field_path`. Regression guard
  preserves the per-field-slot layout for struct-array writes
  (Task #104).
- **`using` directive inlining (Task #91)**: library functions whose
  first parameter is `T storage` are inlined with caller-parameter
  hiding and a per-call inline-return redirect, preserving source
  semantics across the inline boundary.
- **Modifier epilogue semantics (Task #114)**: when at least one
  applied modifier has a body statement after `_`, every `return` in
  the function body now redirects through a synthetic modifier-wrap
  break label so the epilogue runs exactly once regardless of which
  path the function takes.
- **Interface expansion (Task #115)**: interface casts `I(expr)` in
  statements, interface-typed parameters, and interface references are
  expanded to the primary contracts that implement them, allowing
  cross-contract calls through interfaces.
- **Fallback dispatch (Task #126)**: a primary contract's `fallback()`
  now acts as a universal catch when the explicitly-named method
  isn't in the merged dispatch table. `Receive` is included alongside
  `Fallback` in the dispatch scan.
- **Tuple return flattening (Task #94)**: nested tuple return
  expressions are now flattened recursively. Parenthesised tuple
  return/parameter types accepted at semantic level.
- **`abi.decode` short-buffer guard (Task #84)**: emits
  `Panic(0x41)` instead of a raw throw when `buf.length <
  expected_static_bytes`.
- **BigInt comparison (Task #30 slice 1 Part C)**: wide-ByteArray
  comparisons route through BigInt so `uint256` values pushed as
  21+ byte payloads compare by magnitude, not by raw bytes.
- **Wide bitwise ops (Task #50)**: `~`, `|`, `&`, `^` on wide operands
  route through BigInt with a post-op 256-bit mask, so `~uint256(x)`
  produces `u256::MAX - x` rather than `!(x as u64)`. Narrow
  i64/u64 shift path truncation also fixed.
- **BigInt shift (Task #H4)**: wide-ByteArray left/right shifts route
  through BigInt. Reuses the Task #50 infrastructure and clamps shift
  amounts > 255 to an all-zero result per EIP-145 (Task #33).
- **Interface kind metadata (Task #115)**: collected per-interface at
  analysis time and threaded through lowering for virtual dispatch.
- **Self external-call routing (Task #83)**: `new B(); b.foo()` where
  `B` is a sibling-merged primary contract now routes through
  `self_method_offsets` via a 20-byte zero placeholder, bypassing the
  System.Contract.Call syscall for bandwidth-free internal dispatch.
- **Framework / NEP standards (devpack v1.1.0)**: `FrameworkBase.sol`,
  `NEP17.sol`, `NEP11.sol`, `NativeCalls.sol`, `Neo.sol`,
  `Runtime.sol`, `Storage.sol` refreshed for the new compiler
  semantics. New `Precompiles.sol` library (EVM precompile routing)
  and `PrecompileShowcase.sol` example.

### Fixed

- **MEMCPY leak in 8+ sites (Task #109, #66, #89, #76)**: `bytesN(..)` /
  `address(..)` cast args no longer leak the MEMCPY source pointer
  into downstream builtins. Fixed in `events.rs`, `abi.rs`,
  `member_access.rs`, `resolved.rs`, `return_revert.rs`,
  `builtins.rs`, and the packed-encoding path.
- **`bytes32 ↔ uint256` (Task #111)**: `uint256(bytes32)` is a
  bit-identity reinterpret — Solidity spec §4.7.3 — not a decode.
  Preserves magnitude across the cast.
- **`payable(x)` (Task #128)**: now a pure type-only cast (§4.3, §4.7.3),
  not a no-op wrap.
- **`string(bytes_value)` (Task #171)**: recognised as a semantic
  no-op on the value stack.
- **Nested tuple return (Task #94, #64)**: `return (a, (b, c))` flattens
  to the canonical tuple layout.
- **Bytes slice SUBSTR (Task #95)**: `b[i:j]` on dynamic `bytes` lowers
  correctly via the new `Substr` IR op.
- **Modifier epilogue (Task #114)**: runs once regardless of return path.
- **Struct array push/read (Task #104, #170)**: `P[] ps; ps.push(P(a,b))`
  and subsequent reads use the correct per-field slot layout;
  symmetric narrow-ByteArray × UnsignedInteger arithmetic arms added.
- **Delegatecall silent routing (Task #101)**: now hard-rejected with a
  diagnostic, no longer silently routes to System.Contract.Call.
- **9-arg `abi.encode` selector-side (Task #121 mirror)**: `T[]` in
  selector-side args emits the same 32-byte BE offset/length/elements
  shape as the return side.
- **Dynamic-array return JSON leak (Task #137)**: single-value
  dynamic-array returns (`return uint[]`) now emit the canonical EVM
  layout instead of leaking an internal JSON Array envelope.
- **Post-increment / post-decrement wrap (Task #30 slice 4)**:
  `lower_post_inc_dec` now routes through the checked-arithmetic
  path so `x++` at `uint256.max` emits `Panic(0x11)`.
- **Checked arithmetic — 6 ops (Task #30, #67, #154)**: all Add/Sub/Mul
  ops on `uint256`/`int256`/narrow signed types now route through
  BigInt with post-op range guards against the per-type domain.
- **`uint256` BigInt path (Task #32)**: Sub/Mul no longer wrap at 64-bit
  when operands are wide ByteArray.
- **`abi.encode*` / `abi.decode` shape (Task #44)**: EVM-canonical
  `pad32_be(arg_i)` encoding for static args, offset/length/payload for
  dynamic args. Round-trip decoder accepts the BE-packed payload.
- **INT256_MIN / -1 (Task #30 slice 4)**: runtime-side guard for the
  unrepresentable signed-division case routes to `Panic(0x11)`.
- **`emit` with 0 indexed args (Task #39)**: produces exactly 1 topic
  (the signature hash).
- **NatSpec permissions override**: `@custom:neo.manifest.permissions`
  comments correctly substitute wildcard manifests (pre-landed).
- **Modifier re-entry guard**: modifier-epilogue redirect does not
  interfere with nested try/catch break targets.
- **`NeoRuntime::call_function` (Task #19 — earlier session)**: pushes
  args to eval stack, skips `_deploy`, respects offset. All
  `call_method`-driven harnesses now deliver args correctly.
- **`ecrecover` (Task #20 — earlier session)**: returns
  Ethereum-spec address via `keccak256(pubkey[1..])[12..]`, not the
  Neo-native script-hash shape.
- **Narrow-operand bitwise (Task #118)**: left-aligned bytesN fuzzing
  harness no longer flagged as negative via the sign-extended interpretation.
- **`.length` on indirected storage dynamic arrays (Task #161)**:
  returns the live length from the canonical slot instead of a stale
  SIZE value.
- **`pop()` empty-array Panic (Task #98)**: now routes through the
  shared `emit_panic` helper with code `0x31`.
- **`.transfer(uint)` on payable (Task #162)**: lowering now works
  end-to-end instead of faulting on argument packing.
- **`new T[N]` memory allocation (Task #49)**: `T[N] memory a;` (no
  initializer) now allocates a real array with N default-initialized
  slots instead of leaving the local as a null reference.
- **Cross-contract `new B(); b.foo()` routing (Task #160)**: catch
  arms in try/catch around the sibling-merged external-call routing
  receive the revert envelope correctly.
- **`try X { ... } catch ...` parse-shape (Task #125)**: leading
  call expression in a try statement is correctly matched against
  the sibling-merge routing.

### Infrastructure

- **Fuzz test split**: the 24k-line `tests/fuzz_tests.rs` monolith was
  reorganised into 10 submodules under `tests/fuzz_tests/` for
  maintainability:
  - `baseline_tests.rs` — the original 52 harnesses.
  - `batches_18_30.rs`, `batches_31_45.rs`, `batches_46_64.rs`,
    `batches_66_80.rs` — per-batch harness groups.
  - `arithmetic_props.rs`, `compiler_props.rs`, `optimizer_props.rs`,
    `storage_props.rs` — proptest property tests.
  - `task107_catch_panic_tests.rs` — dedicated Panic(uint256)
    envelope coverage.
  - `common.rs` — shared helpers.
- **Regression corpus**: `.proptest-regressions` files under
  `tests/fuzz_tests/` and `tests/` checkpoint failing seeds for all
  property tests.
- **Devpack bump**: `@r3e-network/neo-solidity-devpack` → **1.1.0** to
  align with the new compiler semantics (new `Precompiles.sol`
  library, refreshed `Framework.sol`, `NEP17.sol`, `NEP11.sol`).

### Known Limitations

- **Task #182** — nested struct dynamic-array `.length` returns still
  serialize via the internal JSON Array shape instead of the
  EVM-canonical 32-byte BE length. Workaround: return the field as
  a separate `uint256` computed from `arr.length`. The corresponding
  harness is marked `#[ignore]`.

## [v0.15.0] - 2026-03-18

### Added

- **onNEP11Payment Support**: `msg.sender`, `msg.value`, and `msg.data` now correctly map to parameter indices in `onNEP11Payment` callbacks (msg.data uses param 3, after tokenId at param 2).
- **Test Coverage**: Added focused IR codegen tests for `msg.data` with selector prefix and `onNEP11Payment` parameter mapping.

### Changed

- **msg.data with Selector Prefix**: `msg.data` outside callbacks now produces `selector || abi.encode(current args)` instead of just `abi.encode(args)`, giving users a proper EVM-style calldata approximation.
- **block.coinbase**: Now maps to `address(0)` instead of `getNextBlockValidators()`, matching EVM's `address` return type.
- **block.sha3**: Fixed incorrect mapping from `GetRandom()` to `Ledger.currentHash` (the current block's hash).
- **encodeWithSelector Resolution**: Added `encodeWithSelector` to `builtin_library_supported_members` and `resolve_abi_member` for proper resolution.
- **Documentation Overhaul**: Comprehensive update across all documentation files to reflect actual compiler behavior:
  - `delegatecall` documented as warning (not blocked)
  - `msg.value` documented as warning + returns 0 (not error)
  - `parity-and-limitations.md` split Blocked vs Auto-Mapped features
  - All feature tables updated with correct mappings

### Fixed

- **Stale Comments**: Fixed comments in `runtime_values.rs` that referenced wrong warning codes or incorrect behavior.
- **block.parenthash Comment**: Fixed comment that incorrectly referenced `getBlock(currentIndex-1).prevHash`.

## [v0.14.0] - 2026-03-13

### Added

- **EVM Try/Catch Multi-return**: `try/catch` blocks now natively support EVM's multiple return syntax (`try returns(uint a, uint b)`) by seamlessly unwrapping the NeoVM `Array` return payload.
- **Documentation Parity**: Completely refactored the VitePress documentation architecture to identically mirror the official `soliditylang.org` sidebar, taxonomy, and feature coverage, fully tailored for Neo N3.

### Changed

- **Graceful EVM Call Options**: Extraneous call options (e.g., `contract.method{value: x}()` or `new Contract{value: x}()`) are now safely ignored, emitting a semantic warning instead of halting compilation.
- **Inline Assembly Fallback**: `assembly { ... }` blocks now compile gracefully into NeoVM no-ops with a warning, unblocking compilation of heavily optimized Ethereum libraries where the assembly isn't strictly required.
- **Unsupported Call Translation**: Unsupported low-level EVM calls (`delegatecall`, `staticcall`) are now lowered to returning a dummy boolean `false` with a semantic warning instead of a hard E3001 abort.
- **Obsolete EVM Globals**: `msg.data` now compiles to `selector || abi.encode(current args)` outside of the `onNEP17Payment` callback (param 2) and `onNEP11Payment` callback (param 3). `msg.sig` now compiles to the current function selector with a warning about internal-call semantics.

### Fixed

- **Infinite Loop Prevention**: Patched the Neo IR `CallFunction` dataflow analysis to accurately track return arities, preventing `neo-solc` from hanging infinitely on complex void-return functions (like those found in DAO Governance contracts).
- **NatSpec Overrides**: Fixed missing `load_manifest_permissions_override_from_natspec` linkages, ensuring `@custom:neo.manifest.permissions` comments correctly substitute wildcard manifests.
- **Runtime Exception Handlers**: Hardened the execution context bridging, replacing manual modulo bitwise checks with `.is_multiple_of()` to appease strict CI linting.

## [v0.13.1] - 2026-02-18

### Changed

- **Release workflow resilience**: release matrix now uses `fail-fast: false` so one
  target failure no longer cancels other platform builds.

### Fixed

- **ARM64 Linux release builds**: hardened aarch64 cross-compilation setup with explicit
  linker/toolchain environment (`CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER`,
  `CC/CXX/AR` target-specific vars, and cross pkg-config allowance).
- **Release pipeline reliability**: added missing ARM64 cross tool dependencies in CI
  (`g++-aarch64-linux-gnu`, `binutils-aarch64-linux-gnu`, `libc6-dev-arm64-cross`).

## [v0.13.0] - 2026-02-18

### Added

- **Transparent EVM-to-Neo auto-mapping**: 9 previously blocked EVM-specific
  Solidity features now compile with Neo N3 equivalents and compile-time warnings.
- **`super` keyword support**: `super.method()` resolves through flattened inheritance,
  preserving base overrides as `__super_{methodName}` during lowering.
- **User-defined value types (`type X is Y`)**: alias propagation across file/contract
  scopes and inheritance; `wrap`/`unwrap` lower as no-ops.
- **`type(T).name` expression**: compile-time string constants for contract and type names.
- **`require(condition, CustomError(...))` support**: Solidity 0.8.26+ form supported with
  diagnostic-preserving error signature text.
- **Devpack expansion**: added NEP-26 standard coverage, `NativeContracts` native address
  helpers, and reusable `NeoBytes` / `NeoMath` base libraries.

### Changed

- **`using` directive semantics hardened**: member-style library calls now require explicit
  `using ... for` scope, enforce receiver type targeting, and enforce named-function lists
  from `using {f,g} for T` declarations.
- **Frontend-to-IR metadata flow** extended for `using` directives, so library binding scope
  and target constraints are preserved through lowering.
- **Release process docs** updated to match actual repository workflow (validation + tag push).

### Fixed

- **Native contract lowering**: aligned `NativeContracts` and `NativeCalls` member-call paths;
  lowered native constants as address literals in IR/codegen path.
- **Diagnostics quality**: reduced duplicate constant warning noise in merged library contexts.
- **Toolchain compatibility**: replaced unstable `is_multiple_of` usage with stable modulo logic.

## [v0.12.0] - 2026-02-13

### Changed

- **Developer Tools**: 85% → 95% complete
  - Added debug tooling support (@neo-solidity/types/debugger)
  - Added network configurations for Neo TestNet/MainNet
  - Added artifact management
  - Added source map support

## [v0.11.0] - 2026-02-13

### Changed

- **Progress Update**: Updated all progress metrics to 95% complete
- **Status Badge**: Updated to "🟢 Production-Ready · 95% Complete · 620+ Tests"

### Updated

- **Core Compiler**: 90% → 95% (function overloading, public state variable getters)
- **Runtime Library**: 80% → 95% (iterator handles, per-syscall gas accounting, full opcode suite)
- **Testing**: 85% → 95% (620+ test coverage, end-to-end tests complete)
- **Developer Tools**: 75% → 85% (Hardhat, Foundry, ABI Router, Types, CLI Tools)
- **Documentation**: 85% → 95% (Solitity support matrix, Error reference, Architecture)

### Known Limitations

- Oracle integration (stub only - requires external oracle service)
- Fuzzing framework (planned)
- Differential testing (planned)
- IDE debugging tools (planned)

## [v0.10.0] - 2026-02-13

### Fixed

- **Code generation**: Fixed variable assignment to emit correct `STLOC` instructions
- **Loop control**: Implemented `break`/`continue` with proper loop context tracking
- **Variable handling**: Added variable index table for efficient local variable access
- **Semantic analysis**: Improved variable scope tracking with scope stack

### Added

- **CompilerConfig builder methods**: Added `include_abi()`, `include_source_map()`, `validate_only()`, `analyze_only()`
- **Optimization helpers**: Added `is_optimized()` and `optimization_passes()` methods
- **SemanticModel methods**: Added `public_functions()`, `get_function()`, `get_state_variables()`, `is_payable()`
- **Error codes**: Added `BreakOutsideLoop`, `ContinueOutsideLoop`, `InvalidJumpOffset`
- **Helper functions**: Added `emit_ldloc()` and `emit_stloc()` for proper NeoVM bytecode generation

### Refactored

- **Code generator**: Improved variable handling with proper index-based storage
- **Optimizer**: Better code structure with clearer separation of concerns
- **Error handling**: More specific error codes for better debugging

### Changed

- **Gas estimation**: Updated to use more accurate NeoVM cost values (crypto: 700000, storage: 1000000)
- **Devpack documentation**: Clarified `contractCallWithFlags` flags parameter status

## [v0.9.10] - 2026-02-11

### Added

- **Import support expansion**: wildcard namespace bindings now support
  static member calls, namespace-qualified contract/interface casts, and
  selector access forms such as `NS.IFoo.foo.selector`.
- Standard JSON regression coverage for alias/wildcard import behavior,
  including namespace cast and selector forms.
- Low-level call regression coverage for `abi.encodeCall(...)` inline,
  local-variable, and invalid-reference cases.

### Changed

- Low-level call parsing now accepts `abi.encodeCall(...)` payloads in the
  same lowering path as `encodeWithSignature/encodeWithSelector`, including
  simple wrapper forms like `bytes(...)` / `string(...)`.
- `try/catch` lowering now emits runtime type-guard dispatch (`ISTYPE`) for
  multi-clause catch handling, with clearer NeoVM-specific Panic diagnostics.
- `immutable` state variable enforcement tightened to constructor / `_deploy`
  initialization only.
- Feature matrices and README support notes updated to reflect current import,
  low-level call, and catch-clause behavior.

### Fixed

- **Low-level `abi.encodeCall` validation bug**: non-function member
  expressions (e.g. `s.x`) are no longer accepted as function references for
  dynamic low-level calls.
- Fixed-size `new T[N]` allocations now lower correctly for compile-time sizes.
- Nested tuple destructuring lowering reliability improved for mixed targets.

## [v0.9.9] - 2026-02-09

### Added

- **Native contract runtime support**: Policy, Oracle, RoleManagement, Ledger,
  Notary, and Treasury native contracts are now callable from the embedded
  runtime, with per-contract dispatch modules and gas hints.
- `notary.rs` and `treasury.rs` dispatch modules under
  `src/runtime/execution/execution_impl_part2_native/`.
- `NativeContractShowcase.sol` example demonstrating Policy, Ledger, and
  RoleManagement calls from Solidity.
- `OracleRelayStrictShowcase.sol` for strict Oracle request/response relay
  patterns with on-chain callback verification.
- `UpgradeLifecycleShowcase.sol` covering `ContractManagement.update` and
  `ContractManagement.destroy` lifecycle operations.
- `WitnessGuardShowcase.sol` demonstrating `Runtime.checkWitness` guard
  patterns and multi-signer authorization.
- Neo-Express smoke test scripts for the new showcase contracts
  (`test_neoxp_new_showcases_smoke.sh`, `test_neoxp_oracle_relay_smoke.sh`,
  `test_neoxp_upgrade_lifecycle_smoke.sh`, `test_neoxp_witness_guard_smoke.sh`).
- `test_strict_compatibility_sweep.sh` for batch strict-mode compilation
  validation across all showcase contracts.
- `runtime_native_contract_tests.rs` integration test suite for native contract
  dispatch coverage.
- Diagnostic infrastructure activation: structured JSON warnings and errors
  (`--json-warnings`, `--json-errors`) wired through the full pipeline.
- Import path relaxation: the `-I` flag now resolves transitive imports more
  flexibly, reducing false "file not found" errors in multi-directory layouts.

### Changed

- `ExecutionContext` and `ExecutionState` extended with native-contract routing
  tables and overlay storage hooks for Policy/Oracle/Ledger.
- CI workflow (`.github/workflows/ci.yml`) updated with a dedicated
  `neoxp-showcases` job that validates the new showcase contracts end-to-end.
- `bridge_impl_syscalls.rs` and `bridge_impl_core/initialize.rs` updated to
  register Notary and Treasury service endpoints.

### Fixed

- Oracle dispatch now correctly propagates callback contract hash instead of
  defaulting to the calling contract.
- RoleManagement `getDesignatedByRole` returns an empty array (instead of
  panicking) when no nodes are designated for the requested role.

## [v0.9.8] - 2026-02-08

### Added

- **ERC-to-NEP pattern detection**: the compiler recognizes ERC-20, ERC-721,
  ERC-1155, ERC-2612, and ERC-4626 interface patterns and maps them to their
  Neo equivalents (NEP-17, NEP-11) in the generated manifest.
- BN254 (alt_bn128) precompile stubs for pairing and scalar-mul operations.
- Comprehensive test suite expansion: `runtime_syscall_tests.rs` with syscall
  coverage for `Runtime.checkWitness`, `Runtime.getTime`,
  `Runtime.getInvocationCounter`, and `Runtime.getRandom`.
- `erc_nep_patterns.rs` validation module with pattern-matching heuristics for
  standard detection.
- `e2e_compilation_tests.rs` expanded to cover the new showcase contracts and
  native-call paths.
- Metadata test suite (`src/cli/tests/metadata/erc_nep_patterns.rs`) validating
  that manifests carry correct NEP standard annotations.

### Changed

- `semantic_model.rs` updated with ERC/NEP mapping tables used during manifest
  generation.
- `src/solidity/validate/contract/methods.rs` and `return_types.rs` tightened
  to reject incompatible return-type overrides in standard interfaces.

### Fixed

- Manifest `supportedstandards` field now correctly lists detected NEP
  standards instead of leaving the array empty when ERC interfaces are used.

## [v0.9.7] - 2026-02-07

### Added

- **NEP standard detection**: contracts implementing `NEP17` or `NEP11`
  interfaces from the devpack are automatically annotated in the manifest.
- Type inference improvements for `address`-to-`Hash160` and `uint256`-to-
  `Integer` conversions in cross-contract call arguments.
- `CompleteNEP11NFT.sol` and `CompleteNEP17Token.sol` devpack examples
  demonstrating full standard compliance.
- `MultiStandardToken.sol` example implementing both NEP-17 and NEP-11 on a
  single contract.
- `EventIndexedShowcase.sol` demonstrating indexed event parameters and their
  mapping to Neo notifications.
- `InterfaceShowcase.sol` covering Solidity interface inheritance and virtual
  dispatch.
- `TypeCastingShowcase.sol` for explicit and implicit type conversion patterns.

### Changed

- `devpack/standards/NEP11.sol` and `NEP17.sol` updated with complete method
  signatures matching the Neo N3 standard specification.
- `devpack/contracts/Framework.sol` extended with helper methods for standard
  detection at compile time.
- CLI standards module (`src/cli/cli_parts/cli_manifest/standards.rs`) rewritten
  to support automatic and manual standard annotations.

### Fixed

- Type inference no longer silently drops `bytes32` arguments when passed to
  native contract methods expecting `ByteString`.

## [v0.9.6] - 2026-02-06

### Added

- Six famous DeFi contract ports under `examples/famous/`:
  WGAS, FlashLoan, SimpleAMM, TokenVesting, SimpleLending, SimpleDAO.
- `Bank.sol` and `Vault.sol` examples demonstrating deposit/withdraw patterns
  with NEP-17 integration.
- `LowLevelCallShowcase.sol` demonstrating `address.call()` lowering to
  `System.Contract.Call`.
- Improved ERC-20 and ERC-721 example contracts with Neo-specific adaptation
  notes.

### Changed

- `examples/README.md` reorganized with categorized contract listings and
  compilation instructions.

## [v0.9.5] - 2026-02-05

### Added

- Enhanced devpack libraries: `Runtime.sol`, `Storage.sol`, `Neo.sol` updated
  with additional helper methods and NatSpec documentation.
- `NativeCalls.sol` extended with `gasTransfer`, `neoTransfer`, and
  `getContract` wrappers.
- `NEP17Rescue.sol` utility contract for recovering stuck NEP-17 tokens.
- `Syscalls.sol` updated with complete syscall constant definitions.
- `devpack/README.md` rewritten with usage examples and import instructions.

### Changed

- Devpack library method signatures aligned with Neo N3 reference
  implementation parameter names.

## [v0.9.4] - 2026-02-04

### Fixed

- **MEMCPY codegen bug**: memory-copy operations for dynamic `bytes` and
  `string` types now emit correct NeoVM `MEMCPY` sequences instead of
  truncating at 32-byte boundaries.
- **Void DROP bug**: functions returning `void` no longer emit a spurious `DROP`
  opcode that corrupted the evaluation stack when called as statements.
- **LogicalNot codegen bug**: the `!` (logical not) operator now correctly emits
  `NZ` + `NOT` instead of a bare `NOT`, which previously produced incorrect
  results for non-boolean integer operands.

### Changed

- `src/ir/expressions/calls/type_constructors.rs` and
  `src/ir/expressions/variable.rs` refactored to centralize type-width
  validation during IR lowering.

---

[Unreleased]: https://github.com/r3e-network/neo-solidity/compare/v0.16.0...HEAD
[v0.16.0]: https://github.com/r3e-network/neo-solidity/compare/v0.15.0...v0.16.0
[v0.15.0]: https://github.com/r3e-network/neo-solidity/compare/v0.14.0...v0.15.0
[v0.14.0]: https://github.com/r3e-network/neo-solidity/compare/v0.13.1...v0.14.0
[v0.13.1]: https://github.com/r3e-network/neo-solidity/compare/v0.13.0...v0.13.1
[v0.13.0]: https://github.com/r3e-network/neo-solidity/compare/v0.12.0...v0.13.0
[v0.12.0]: https://github.com/r3e-network/neo-solidity/compare/v0.11.0...v0.12.0
[v0.11.0]: https://github.com/r3e-network/neo-solidity/compare/v0.10.0...v0.11.0
[v0.10.0]: https://github.com/r3e-network/neo-solidity/compare/v0.9.10...v0.10.0
[v0.9.10]: https://github.com/r3e-network/neo-solidity/compare/v0.9.9...v0.9.10
[v0.9.9]: https://github.com/r3e-network/neo-solidity/compare/v0.9.8...v0.9.9
[v0.9.8]: https://github.com/r3e-network/neo-solidity/compare/v0.9.7...v0.9.8
[v0.9.7]: https://github.com/r3e-network/neo-solidity/compare/v0.9.6...v0.9.7
[v0.9.6]: https://github.com/r3e-network/neo-solidity/compare/v0.9.5...v0.9.6
[v0.9.5]: https://github.com/r3e-network/neo-solidity/compare/v0.9.4...v0.9.5
[v0.9.4]: https://github.com/r3e-network/neo-solidity/releases/tag/v0.9.4
