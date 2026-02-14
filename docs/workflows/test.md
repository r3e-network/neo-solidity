# Test Contracts

The Neo Solidity project uses a layered testing strategy that validates the compiler at every level: unit tests for individual components, integration tests for the pipeline, E2E compilation tests for real contracts, conformance tests against reference implementations, and Neo-Express smoke tests for on-chain deployment behavior.

## Testing Philosophy

The test suite is organized in layers of increasing scope:

| Layer                | What It Tests                                                                  | Count       | Command                                   |
| -------------------- | ------------------------------------------------------------------------------ | ----------- | ----------------------------------------- |
| Unit tests           | Individual compiler components (lexer, parser, semantic, IR, codegen, runtime) | 400+        | `cargo test --workspace`                  |
| E2E compilation      | Full pipeline from Solidity source to NEF + manifest                           | 74 tests    | `cargo test --test e2e_compilation_tests` |
| Conformance          | Output correctness against reference Neo implementations                       | 32 vectors  | `cargo test --test conformance_tests`     |
| Strict compatibility | Compilation of all devpack and example contracts                               | ~50 files   | `make test-compile-strict`                |
| Neo-Express smoke    | Deploy + invoke on a real local Neo chain                                      | 16+ scripts | `make test-deploy-smoke-full`             |
| Tooling              | TypeScript packages (Hardhat, Foundry, ABI router)                             | varies      | `make tooling-test`                       |
| C# runtime           | EVM-compatible runtime primitives                                              | varies      | `make runtime-test`                       |

Each layer catches different classes of bugs. Unit tests catch logic errors in individual passes. E2E tests catch integration issues between passes. Conformance tests catch behavioral divergence from Neo N3. Smoke tests catch deployment and invocation failures that only manifest on a real chain.

::: tip Quick Feedback Loop
During development, `cargo test --workspace` is the fastest way to validate changes. It runs all Rust tests (unit + integration + E2E + conformance) in under 30 seconds.
:::

## Quick Validation

For fast feedback during development:

```bash
# Run all Rust tests (unit + integration + E2E + conformance)
make test
# or equivalently:
cargo test --workspace
```

For a quick compilation sanity check (verifies NEF magic bytes and manifest structure):

```bash
bash examples/test_compilation.sh
```

## Unit Tests

Unit tests cover individual compiler components in isolation. They live in `tests/` and inline `#[cfg(test)]` modules.

```bash
# Run all unit tests
cargo test --workspace

# Run tests for a specific module
cargo test lexer_tests
cargo test parser_tests
cargo test semantic_tests
cargo test runtime_tests

# Run with output visible (useful for debugging)
cargo test -- --nocapture

# Run a specific test by name
cargo test test_simple_storage_compilation

# Run tests in parallel (default) or single-threaded
cargo test -- --test-threads=1
```

### Runtime Tests

The embedded NeoVM runtime has extensive unit tests covering opcodes, syscalls, storage, iterators, and gas accounting:

```bash
# All runtime tests
cargo test runtime_

# Specific runtime areas
cargo test runtime_arithmetic
cargo test runtime_storage
cargo test runtime_crypto
```

## E2E Compilation Tests

The E2E test suite compiles every example contract through the full pipeline and validates the output:

```bash
cargo test --test e2e_compilation_tests
```

This runs 74 tests that:

1. Parse each Solidity source file
2. Run it through the full compilation pipeline
3. Verify the NEF is valid (magic bytes, checksum)
4. Verify the manifest is valid JSON with correct ABI structure
5. Check that method signatures match the Solidity source

To run with parallel threads for faster execution:

```bash
cargo test --test e2e_compilation_tests -- --test-threads=4
```

## Conformance Tests

Conformance tests compare compiler output against reference Neo N3 implementations:

```bash
cargo test --test conformance_tests
```

The suite includes 32 test vectors with a 93.8% pass rate. Each vector specifies:

- Input Solidity source
- Expected NeoVM opcodes or behavior
- Expected manifest structure

Failing conformance tests indicate behavioral divergence from the Neo N3 reference. These are tracked in `docs/NEO_VM_PARITY_TODO.md`.

::: warning Conformance Gaps
A 93.8% pass rate means some edge cases diverge from the Neo N3 reference. Check the parity TODO before relying on behavior covered by failing vectors.
:::

## Strict Compatibility Compilation Sweep

This sweep compiles all devpack libraries and example contracts with strict settings to catch regressions:

```bash
make test-compile-strict
# or:
bash examples/test_strict_compatibility_sweep.sh
```

The sweep:

1. Compiles every `.sol` file in `devpack/` and `examples/new/`
2. Uses strict compiler flags
3. Reports any unexpected warnings or errors
4. Fails if `STRICT_SWEEP_FAIL_ON_UNEXPECTED_WARNINGS=1` is set (used in CI)

## Neo-Express Smoke Tests

Smoke tests deploy contracts to a real local Neo-Express chain and invoke methods to validate on-chain behavior. Each script is self-contained: it creates a fresh chain, deploys, invokes, and cleans up.

### Prerequisites

- Neo-Express installed (see [Installation](/getting-started/installation))
- `jq` and `hexdump` available
- `neo-solc` built (scripts auto-build if needed)

### Individual Smoke Tests

```bash
# Basic deploy + invoke (storage, events, native calls)
make test-deploy-smoke

# Constructor arguments via _deploy(data, update)
make test-deploy-constructor-smoke

# Contract update lifecycle (deploy v1, update to v2)
make test-deploy-update-smoke

# Manifest permissions for native contracts (StdLib, CryptoLib)
make test-deploy-permissions-smoke

# abi.encode / abi.decode round-trip (StdLib.serialize/deserialize)
make test-deploy-encoding-smoke

# Revert reason propagation (abort message)
make test-deploy-abortmsg-smoke

# Low-level call (System.Contract.Call)
make test-deploy-lowlevel-call-smoke

# Low-level call failure handling
make test-deploy-lowlevel-call-failure-smoke

# External member-call dispatch
make test-deploy-external-call-smoke

# View/ReadOnly external calls
make test-deploy-view-readonly-call-smoke

# Compound assignment operators (+=, -=, etc.)
make test-deploy-compound-assignment-smoke

# Struct array element access
make test-deploy-struct-array-element-smoke

# Nested struct storage
make test-deploy-nested-struct-smoke

# Delete operator
make test-deploy-delete-smoke

# New showcase contracts (UpgradeLifecycle, WitnessGuard, OracleRelay)
make test-deploy-new-showcases-smoke
```

### Run All Smoke Tests

```bash
make test-deploy-smoke-full
```

This runs all 16+ individual smoke tests sequentially. Each test is independent and creates its own temporary chain.

### Famous DeFi Contract Smoke Tests

The repository also includes smoke tests for ports of iconic DeFi protocols:

```bash
# Wrapped GAS (WETH9-style)
make test-deploy-wgas-smoke

# Flash loan (Aave V2-style)
make test-deploy-flashloan-smoke

# AMM (Uniswap V2-style)
make test-deploy-amm-smoke

# Token vesting (OpenZeppelin-style)
make test-deploy-vesting-smoke

# Lending (Compound-style)
make test-deploy-lending-smoke

# DAO (Governor-style)
make test-deploy-dao-smoke

# All famous contract tests
make test-deploy-famous-all
```

### What Each Smoke Test Validates

The basic deploy smoke test (`test_neoxp_deploy.sh`) is representative of the pattern:

1. Resolves `neo-solc` binary (builds if needed)
2. Resolves `neoxp` binary (installs if needed)
3. Creates a temporary working directory
4. Writes a test contract inline
5. Compiles with `neo-solc`
6. Creates a fresh Neo-Express chain
7. Transfers GAS to the deployer
8. Deploys the contract
9. Invokes `sender()` and verifies it returns the deployer's script hash
10. Invokes `set(7)` and verifies the `ValueSet` event fires
11. Invokes `get()` and verifies it returns `7`
12. Invokes `height()` and verifies it returns an integer (tests native contract calls)
13. Cleans up the temporary directory

## Workspace-Wide Test Suites

### Core Tests

```bash
# Rust tests only
make test

# Rust tests + tooling tests + tooling lint
make test-all

# Rust tests + tooling tests + C# runtime tests
make test-all-full
```

### Tooling Tests

```bash
# TypeScript package tests (Hardhat, Foundry, ABI router, CLI tools)
make tooling-test

# TypeScript linting
make tooling-lint
```

### C# Runtime Tests

```bash
# Requires .NET SDK
make runtime-test
```

## Code Coverage

Generate coverage reports using `cargo-tarpaulin`:

```bash
# Install tarpaulin (one-time)
cargo install cargo-tarpaulin

# Generate HTML coverage report
make coverage
# Report is at coverage/tarpaulin-report.html

# Generate LCOV for CI
make coverage-ci
# Report is at coverage/lcov.info

# Check minimum coverage threshold (70%)
make check-coverage
```

## CI/CD Pipeline Setup

### Recommended CI Sequence

The recommended CI pipeline runs these steps in order:

```bash
# 1. Code formatting check
cargo fmt --all -- --check

# 2. Linting
cargo clippy --all-targets --all-features -- -D warnings

# 3. Release build
cargo build --release

# 4. Full test suite
cargo test --workspace --all-features

# 5. E2E compilation tests (parallel)
cargo test --test e2e_compilation_tests -- --test-threads=4

# 6. Strict compatibility sweep
STRICT_SWEEP_FAIL_ON_UNEXPECTED_WARNINGS=1 make test-compile-strict

# 7. Neo-Express smoke tests (requires .NET)
make test-deploy-smoke-full
```

### One-Command CI Gate

The `production-gate` target runs all of the above:

```bash
make production-gate
```

See [Production Readiness](/workflows/production) for details.

### GitHub Actions Example

The repository's CI workflow (`.github/workflows/ci.yml`) includes:

- Standard Rust CI (fmt, clippy, test) on Ubuntu
- E2E compilation tests
- A dedicated `neoxp-showcases` job that:
    - Installs Rust + .NET 8 + `jq`
    - Runs `examples/test_neoxp_new_showcases_smoke.sh`
    - Validates `UpgradeLifecycleShowcase`, `WitnessGuardShowcase`, and `OracleRelayStrictShowcase` end-to-end

## Writing Your Own Tests

### Adding a Compilation Test

1. Create a new Solidity file in `examples/` or `examples/new/`.
2. The E2E test suite automatically picks up files in these directories.
3. Run `cargo test --test e2e_compilation_tests` to verify.

### Adding a Smoke Test

1. Create a new script in `examples/` following the pattern of existing scripts.
2. The script should:
    - Create a temporary directory and clean up on exit
    - Resolve `neo-solc` and `neoxp` binaries
    - Write a test contract inline
    - Compile, deploy, invoke, and validate
3. Add a Make target in the `Makefile`.
4. Add the target to `test-deploy-smoke-full` dependencies.

### Adding a Unit Test

Add `#[test]` functions in the relevant module or create a new test file in `tests/`:

```rust
#[test]
fn test_my_feature() {
    let source = r#"
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.20;
        contract Test {
            function foo() public pure returns (uint256) {
                return 42;
            }
        }
    "#;
    // Use the compiler API to compile and validate
    // ...
}
```

## Test Summary

| Command                                   | Scope               | Duration | Requires                |
| ----------------------------------------- | ------------------- | -------- | ----------------------- |
| `cargo test --workspace`                  | All Rust tests      | ~30s     | Rust                    |
| `cargo test --test e2e_compilation_tests` | E2E compilation     | ~10s     | Rust                    |
| `cargo test --test conformance_tests`     | Conformance vectors | ~5s      | Rust                    |
| `make test-compile-strict`                | Strict sweep        | ~20s     | Rust                    |
| `make test-deploy-smoke`                  | Basic deploy        | ~30s     | Rust, .NET, jq          |
| `make test-deploy-smoke-full`             | All smoke tests     | ~5min    | Rust, .NET, jq          |
| `make test-all`                           | Rust + tooling      | ~1min    | Rust, Node.js           |
| `make production-gate`                    | Everything          | ~8min    | Rust, Node.js, .NET, jq |

## Related Pages

- [Production Readiness](/workflows/production) -- Full pre-deployment gate.
- [Compile Workflow](/workflows/compile) -- CLI reference.
- [Deploy Workflow](/workflows/deploy) -- Deployment patterns.
- [Installation](/getting-started/installation) -- Set up the test environment.
