# Fuzz Testing

The Neo Solidity compiler uses a multi-layered fuzzing strategy to catch crashes, semantic regressions, and runtime parity issues before they reach production.

## Overview

| Layer | Framework | Target | Cases |
|-------|-----------|--------|-------|
| **Proptest** (Rust) | `proptest` | Compiler properties, runtime verification, storage | 655 |
| **Cargo-fuzz T1** | `libfuzzer` | Parser / compiler crash finding | Continuous |
| **Cargo-fuzz T2** | `libfuzzer` | NEF deserializer crash finding | Continuous |

## Running the Fuzz Suite

### One-Shot Proptest Run

```bash
# Full suite (default: 50 cases per property)
cargo test --test fuzz_tests

# Full suite with more cases (slower, deeper)
PROPTEST_CASES=200 cargo test --test fuzz_tests

# Quick smoke (fewer cases, faster feedback)
PROPTEST_CASES=10 cargo test --test fuzz_tests -- --test-threads=4
```

### Continuous Background Fuzzing

```bash
# Start the continuous fuzz loop (proptest + cargo-fuzz, indefinite)
make test-fuzz-start

# Check status, coverage, and crash count
make test-fuzz-status

# Stop the background fuzzer
make test-fuzz-stop
```

The continuous runner executes in rounds:
1. Proptest suite (default 50 cases per property)
2. Cargo-fuzz T1 — compiler/parser target (5 min)
3. Cargo-fuzz T2 — NEF parser target (5 min)
4. Periodic gate/smoke checks (every N rounds)

### CI / Scheduled Deep Runs

```bash
# Deep run: 100 proptest cases + extended cargo-fuzz
make test-fuzz-continuous

# Or run the suite script directly
bash scripts/run_fuzz_suite.sh deep
```

A GitHub Actions workflow (`.github/workflows/fuzz.yml`) runs the deep suite nightly at 02:00 UTC.

## What the Fuzz Suite Covers

### Compiler Properties (`optimizer_props.rs`)

- **Semantic equivalence**: O0 vs O3 produce identical execution results for pure arithmetic, loops, recursion, internal calls, modifiers, storage+events
- **NEF format validity**: Generated `.nef` files have correct magic header, checksum, and method tokens
- **Manifest correctness**: Generated `.manifest.json` has valid ABI, permissions, and supported standards
- **Determinism**: Same source produces identical bytecode across compilations

### Runtime Verification (`compiler_props.rs`)

- **Neo N3 native contracts**: Oracle, Policy, Ledger, RoleManagement, ContractManagement
- **Precompile helpers**: sha256, ripemd160, identity, and the typed uint256 modexp helper are checked against reference implementations. The modexp helper is not full EIP-198 raw byte-payload parity, and Neo BLS12-381 adaptations for 0x06-0x08 are not Ethereum BN256 precompile parity.
- **EVM globals**: `gasleft()`, `block.timestamp`, `address.balance`, `selfdestruct`
- **ABI encode/decode**: Roundtrip correctness (test-runtime only; see P0 warning)

### Storage Properties (`storage_props.rs`)

- Isolation between accounts
- Roundtrip preservation
- Key ordering
- Overwrite semantics
- Large value handling
- Empty value handling

### Syntax Resilience (`arithmetic_props.rs`)

- Keyword case variants
- Nested ternary with side effects
- Interleaved comments
- Very large hex literals
- Scientific notation extremes
- Escape sequence edge cases
- Empty/whitespace strings

### Edge Features (`compiler_props.rs`)

- User-defined value types
- Nested struct arrays
- Anonymous events
- Try/catch with custom errors
- Enum casts
- Function pointers
- `type(I).interfaceId`
- Global `using for`

### Precompile Runtime Verification (batches_100_105.rs, Batch #100)

- **ecrecover** (0x01): compilation and dispatch verification
- **SHA-256** (0x02): digest matches Rust sha2 reference
- **RIPEMD-160** (0x03): digest matches Rust ripemd reference
- **Identity** (0x04): data passthrough roundtrip
- **ModExp** (0x05): compilation and dispatch verification
- **BLS12-381 adaptations** (0x06-0x08): Neo-native curve helpers only; not Ethereum BN256/alt_bn128 parity
- **BLAKE2f** (0x09): unavailable on Neo N3 and expected to revert if called through the devpack helper

### Neo N3 Syscall Verification (batches_100_105.rs, Batch #101)

- `Runtime.checkWitness` — dispatch and return verification
- `Runtime.getNetwork` — network magic number
- `Runtime.getPlatform` — platform string
- `Runtime.getEntryScriptHash` — script hash verification
- `Storage.find` — iterator-based prefix search

### Solidity Feature Runtime (batches_100_105.rs, Batches #102-#105)

- `unchecked` overflow wrap semantics
- `fallback()` dispatch routing
- `msg.sig` selector verification
- `immutable` write rejection
- `receive()` → `onNEP17Payment` manifest remapping
- NEF/manifest `supportedstandards` auto-detection
- Manifest `groups` and `permissions` shape
- Multi-contract manifest isolation
- NEF bytecode determinism
- `abi.encodePacked` output shape
- `abi.encodeWithSignature` selector prefix
- `bytes.concat` / `string.concat` runtime
- try/catch with string return binding

### Native Contract & OOP (batches_106_110.rs, Batches #106-#110)

- ContractManagement.getContract/hasMethod/getMinimumDeploymentFee
- Policy.getExecFeeFactor/getStoragePrice
- Storage multi-key-type mappings, delete-then-get, nested mappings
- Struct with mixed fields (uint/address/bool)
- Array push/pop/length
- Event emission with indexed params
- Custom error revert with ABI-encoded args
- `require` with string and custom error messages
- `assert` failure (Panic 0x01)
- Inheritance with `super` calls
- Interface implementation and dispatch
- Abstract contract rejection
- `using for` directive
- Constructor chaining across 3 levels
- Empty contract deployment
- Event-only contract
- Long function names (100 chars)
- 50 state variables with auto-getters
- 5 contracts in one source file

## Interpreting Coverage

Cargo-fuzz prints coverage after each run:

```
cov: 7356                  ← edges covered in the target
ft: 7356                   ← features (same as cov for libfuzzer)
corp: 127                  ← corpus size (seed inputs)
```

For the **compiler/parser target (T1)**, coverage grows as the fuzzer discovers new Solidity syntax constructs. Saturation (single-digit gains per round) is expected after several hours.

For the **NEF parser target (T2)**, coverage plateaus early (~83 edges) because the NEF parser is extremely strict — most random bytes are rejected at the magic header check.

## Crash Triage

If the fuzzer finds a crash:

1. **Locate the crash artifact**:
   ```bash
   ls fuzz/corpus/fuzz_target_1/crash-*
   ls fuzz/corpus/fuzz_target_2/crash-*
   ```

2. **Reproduce with the artifact**:
   ```bash
   cargo +nightly fuzz run fuzz_target_1 fuzz/corpus/fuzz_target_1/crash-<hash>
   ```

3. **Minimize the crash input**:
   ```bash
   cargo +nightly fuzz run fuzz_target_1 -minimize-crash=1 -runs=10000 \
     fuzz/corpus/fuzz_target_1/crash-<hash>
   ```

4. **Convert to a regression test** in the appropriate `tests/` module.

## Known Fuzz Gaps

These areas are not yet deeply fuzzed and represent future expansion targets:

- **Differential testing** — compare neo-solc output against a reference EVM compiler
- **Gas oracle differential** — compare embedded runtime gas against Neo-Express
- **Cross-contract call sequences** — multi-contract interaction fuzzing
- **Manifest permission inference** — fuzz dynamic call sites to stress permission analysis
- **Dynamic ABI encoding** — `abi.encode` with dynamic types (string, bytes, arrays) falls back to pseudo-native StdLib which faults on real Neo N3
- **Full NEP-17 transfer flow** — end-to-end token transfer with onNEP17Payment callback chain
- **ContractManagement.deploy/update** — dynamic contract deployment and upgrade lifecycle
