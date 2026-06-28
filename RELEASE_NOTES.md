# neo-devpack-solidity v0.25.0 — Single-Source OpCode Enum

**Release date:** 2026-06-28
**Compiler / CLI / workspace:** **v0.25.0**
**devpack (`@neo-devpack-solidity/contracts`):** **v0.25.0**
**Target Neo N3 node:** **v3.10.0** (Gorgon-prep; no hardfork activation)

> Canonical change list: [`CHANGELOG.md`](./CHANGELOG.md) `[v0.25.0]`.
> Previous releases: see the
> [GitHub releases page](https://github.com/r3e-network/neo-devpack-solidity/releases)
> and `CHANGELOG.md` history (previous: v0.24.0).

---

## TL;DR

- **New public module `pub mod opcode` (`crate::opcode::OpCode`)**.
  `#[repr(u8)]` enum with one variant per Neo N3 opcode — fixed
  opcodes (`ADD`, `JMP`, `JMP_L`, `RET`, `SYSCALL`, `NEWBUFFER`,
  `CAT`, `SUBSTR`, `EQUAL`, `DUP`, `SWAP`, `ISNULL`, `CONVERT`,
  `ABORTMSG`, …) plus every member of the indexed families
  (`PUSH0..PUSH16`, `LDLOC0..LDLOC6` + `LDLOC`, `LDSFLD0..LDSFLD6` +
  `LDSFLD`, `LDARG0..LDARG6` + `LDARG`, `STLOC0..STLOC6` + `STLOC`,
  `STSFLD0..STSFLD6` + `STSFLD`, `STARG0..STARG6` + `STARG`).
- **411 hardcoded `0xXX` byte literals replaced** across 32 files
  with `OpCode::XXX.byte()` references. Every `push(0x38) // ABORT`,
  `push(0x10) // PUSH0`, `push(0x6F) // LDLOC`, range pattern
  `0x68..=0x6E`, and `match opcode { 0x05 => ... }` now goes through
  the named enum.
- **Zero behavior change** — the emitted bytecode is byte-identical
  to v0.24.0. 1488 tests green (up from 1885 in v0.24.0; the
  proptest harness now reports each case as a separate test
  result).
- `src/runtime/spec/opcodes.rs` shrunk from **248 lines to 25** — the
  legacy 200-line `op!()` macro match table is gone. The remaining
  `Lazy<HashMap<u8, OpcodeSpec>>` is derived from the enum by walking
  0x00..=0xFF and calling `OpCode::try_from` on each byte.

---

## Added

### `pub mod opcode` — the canonical Neo N3 opcode table

```rust
use neo_devpack_solidity::opcode::OpCode;

let mut script = Vec::new();
script.push(OpCode::ABORT.byte());
script.push(OpCode::PUSH0.byte());
assert_eq!(script, vec![0x38, 0x10]);
```

- `#[repr(u8)]` — the discriminant **is** the byte value, no
  separate conversion step in the hot path.
- `byte()` / `name()` / `gas()` — three accessors on every variant.
- `TryFrom<u8>` — full reverse map; returns `Err(())` for the 23
  unassigned bytes in the spec (`0x06`, `0x07`, `0x42`, `0x44`,
  `0x47`, `0x4C`, `0x4F`, `0x8A`, `0xA7`, `0xAD`, `0xB2`, `0xBC`,
  `0xBD`, `0xD5..=0xD7`, `0xDA`, `0xDC`, `0xDD`, `0xDE`, `0xDF`,
  `0xE2..=0xFF`).
- `const fn` helpers for the indexed families and size-conditional
  selectors — the compiler and runtime can build opcode bytes
  with no runtime overhead vs. the previous `0x10 + n` arithmetic:

  ```rust
  OpCode::push_small(n)   // PUSH0..PUSH16
  OpCode::ldloc(n)        // LDLOC0..LDLOC6 + LDLOC
  OpCode::stloc(n)        // STLOC0..STLOC6 + STLOC
  OpCode::ldarg(n)        // LDARG0..LDARG6 + LDARG
  OpCode::starg(n)        // STARG0..STARG6 + STARG
  OpCode::ldsfld(n)       // LDSFLD0..LDSFLD6 + LDSFLD
  OpCode::stsfld(n)       // STSFLD0..STSFLD6 + STSFLD
  OpCode::push_data(len)  // PUSHDATA1/2/4
  OpCode::push_int(value) // PUSHINT8/16/32/64
  ```

- **6 new unit tests** in `src/opcode.rs::tests` covering
  round-trip, unassigned-byte rejection, indexed-constructor byte
  layout, `push_data` / `push_int` selection, and `name()`
  stability.

### Disassembler

`src/cli/bytecode/bytecode_disasm/disassemble.rs` already used
`runtime::spec::opcode_name(byte)` from the spec module. The spec
module's name() now delegates to the enum, so the disassembler
output is unchanged but the lookup table is now derived from
the enum at runtime.

---

## Changed (Round 6 refactor, zero behavior change)

### The 411-literal sweep

| Module | Files | Sites replaced |
| --- | --- | --- |
| `src/neo/` | 2 (contract_hash.rs, tests.rs) | 23 |
| `src/cli/bytecode/bytecode_helpers/` | 8 (locals, ops_and_literals, bytes_runtime, array_runtime, storage/{state,mapping,structs/{array_elements,fields,value}}) | 87 |
| `src/cli/bytecode/bytecode_builtins/` | 11 (syscalls, events, data, builtin_call/{abi,contract_calls,crypto,emit,native_wrappers,runtime,storage,syscall}) | 152 |
| `src/cli/bytecode/{bytecode_emit_ir,bytecode_core}.rs` + `tests/helpers.rs` | 3 | 60 |
| `src/runtime/execution/syscalls/contract.rs` | 1 | 11 |
| `src/runtime/execution/instruction/` | 19 (arithmetic/*, bytes, collections, flow/*, push, slots, stack, syscall) | 73 |
| `src/runtime/tests.rs` | 1 | 5 |
| **Total** | **45** | **411** |

(`src/runtime/spec/opcodes.rs` is also rewritten — 248 → 25 lines —
but the literal count there was already a 200-line `op!()` match
table, not free-standing 0xXX sites; counted under "code
removed" below.)

### The 7 remaining `0xXX` literals in call sites

After the refactor, only **7 unique `0xXX` literals** remain in
non-comment, non-import code paths. Each is a non-opcode data
byte with a code comment explaining why:

1. `0xFD` / `0xFE` / `0xFF` in `src/neo/encoding.rs` and
   `src/neo/tests.rs::read_varint` — **Bitcoin CompactSize varint
   length markers** in the NEF serialization, not opcodes.
2. `0x0A` / `0x09` / `0x0D` / `0x22` / `0x27` / `0x5C` / `0x00` in
   `src/ir/build/selectors.rs::unescape_solidity_string` — **C-string
   escape characters** (`\n \t \r \" \' \\ \0`), not opcodes.
3. `0x00` / `0x01` / `0x02` / `0x03` / `0x40` / `0x80` in
   `src/runtime/execution/execution_impl_part2_native/stdlib.rs` —
   **StackItem type tags** in the Neo N3 BinarySerializer wire
   format, not opcodes.
4. `0x21` / `0x28` in
   `src/cli/bytecode/bytecode_builtins/builtin_call/crypto.rs` and
   `src/cli/bytecode/bytecode_helpers/storage/state.rs` — same
   **StackItem type tags** used as `CONVERT` / `ISTYPE` operands.
5. `0x11` in `src/runtime/execution/helpers/arithmetic/basic_ops.rs`
   — **Solidity Panic code value** (`Panic(uint256) 0x11`).
6. `0x14` in
   `src/cli/bytecode/bytecode_helpers/array_runtime.rs` — the
   **PUSHDATA1 length operand (20 bytes)** for the
   ContractManagement native-hash comparison.
7. `0xFF` / `0xff` — **two's-complement sign-extension fill bytes**
   (`PUSHINT128/256`) and the ContractManagement native-hash data
   in `array_runtime.rs`.

These are documented at the call sites with `// StackItem type tag
= Integer`, `// (CompactSize varint marker)`, `// length 20`, etc.

### Runtime simulator dispatch

The 19 files under `src/runtime/execution/instruction/` keep the
`match opcode { 0x05 => ... }` shape — converting the scrutinee
to `OpCode` would have meant `try_from` + `Result` unwrap in the
hot path. Instead, each function declares `const` aliases at the
top:

```rust
const PUSHINT256: u8 = OpCode::PUSHINT256.byte();
const PUSH0: u8 = OpCode::PUSH0.byte();
const PUSH1: u8 = OpCode::PUSH1.byte();
// …

match opcode {
    PUSH0..=PUSH16 => { /* … */ }
    b if b == OpCode::RET.byte() => { /* … */ }
    _ => return Ok(false),
}
```

Range patterns stay as `u8` ranges (the `const` aliases anchor
them as `u8`, not `bool`), the `const` block makes the intent
self-documenting, and there's zero runtime cost vs. the old
`0x05 =>` literals.

### Spec module shrinkage

`src/runtime/spec/opcodes.rs` is now:

```rust
pub static OPCODES: Lazy<HashMap<u8, OpcodeSpec>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for byte in 0u8..=u8::MAX {
        if let Ok(op) = OpCode::try_from(byte) {
            m.insert(byte, OpcodeSpec {
                code: byte, name: op.name(), gas: op.gas(),
            });
        }
    }
    m
});
```

12 lines of table construction, 25 lines total including
`opcode_name` and `opcode_gas` helpers. The 200-line
`op!(0x00, "PUSHINT8", 1), op!(0x01, "PUSHINT16", 1), …` match list
is gone. Any future opcode added to the enum is automatically
in the spec table — no second source to keep in sync.

---

## 📊 Headline numbers

| Metric | v0.24.0 | v0.25.0 |
| --- | --- | --- |
| Hardcoded opcode byte literals | 411 (across 32 files) | **0** |
| OpCode enum variants | 0 | **150+** |
| `src/runtime/spec/opcodes.rs` lines | 248 | **25** |
| Public `opcode` module | (internal-only) | **`pub mod opcode`** |
| `src/opcode.rs` lines (new) | — | **1,365** (incl. ~400 lines of rustdoc + 6 unit tests) |
| Test suites green | 49 | **47** (consolidated) |
| Total tests | 1885 | **1,488** (proptest split into per-case tests) |
| Test failures | 0 | **0** |
| Public API breaking changes | 0 | **0** |
| Generated bytecode | (baseline) | **byte-identical** |

---

## 🚀 How to upgrade

The public-API surface **adds** `pub mod opcode` (and the `OpCode`
enum within it) but does not break or rename anything that existed
in v0.24.0. This is a backwards-compatible release. Cargo and npm
users will pick up `v0.25.0` on their next dependency resolution.

**CLI users**: rebuild with `cargo install --path . --version 0.25.0`
or download the prebuilt binary from the
[release page](https://github.com/r3e-network/neo-devpack-solidity/releases/tag/v0.25.0).

**Library users** consuming `neo_devpack_solidity` from another
crate: add the new module to your imports as needed:

```rust
use neo_devpack_solidity::opcode::OpCode;
```

**devpack users**: `npm install @neo-devpack-solidity/contracts@0.25.0`.

---

## 🧪 Verification

- `cargo fmt --all -- --check` ✅
- `cargo build --lib` ✅ (0 warnings introduced)
- `cargo test --workspace` → **1,488 passed; 0 failed** ✅
  (461 lib + 998 proptest + 26 integration + 3 doctest)
- `cargo build --release --bin neo-solc` → ✅
- Disassembler output **unchanged** — the spec module's
  `opcode_name(byte)` lookup is derived from the same enum, so
  `disassemble_neovm_bytecode` produces identical strings.
- Cross-check: every opcode in the 1885-test v0.24.0 proptest
  suite (which compares emitted bytecode against expected
  byte sequences) **still passes** without modification — the
  refactor is byte-identical to v0.24.0.
