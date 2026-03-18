# Changelog

All notable changes to the Neo Solidity Compiler will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/r3e-network/neo-solidity/compare/v0.9.10...HEAD
[v0.9.10]: https://github.com/r3e-network/neo-solidity/compare/v0.9.9...v0.9.10
[v0.9.9]: https://github.com/r3e-network/neo-solidity/compare/v0.9.8...v0.9.9
[v0.9.8]: https://github.com/r3e-network/neo-solidity/compare/v0.9.7...v0.9.8
[v0.9.7]: https://github.com/r3e-network/neo-solidity/compare/v0.9.6...v0.9.7
[v0.9.6]: https://github.com/r3e-network/neo-solidity/compare/v0.9.5...v0.9.6
[v0.9.5]: https://github.com/r3e-network/neo-solidity/compare/v0.9.4...v0.9.5
[v0.9.4]: https://github.com/r3e-network/neo-solidity/releases/tag/v0.9.4
