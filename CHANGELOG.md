# Changelog

All notable changes to the Neo Solidity Compiler will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Transparent EVM-to-Neo auto-mapping**: 9 previously blocked EVM-specific
  Solidity features now compile transparently with automatic Neo N3 equivalents
  and compile-time warnings. Developers no longer need to modify their Solidity
  code for these features:
  - `block.coinbase` → `address(0)` (dBFT has no miner)
  - `block.difficulty` / `block.prevrandao` → `Runtime.getRandom()`
  - `block.gaslimit` → `Policy.getExecFeeFactor()`
  - `block.basefee` → `Policy.getFeePerByte()`
  - `tx.gasprice` → `Policy.getFeePerByte()`
  - `gasleft()` → `System.Runtime.GasLeft` syscall
  - `blockhash(n)` → `Ledger.getBlockHash(n)`
  - `selfdestruct(addr)` → `ContractManagement.destroy()` (addr argument dropped)
  - `address.codehash` → contract script hash (non-contract returns `bytes32(0)`)

- **`super` keyword support**: `super.method()` calls now resolve correctly
  through inheritance flattening. Overridden base methods are preserved as
  `__super_{methodName}` and resolved during IR lowering. Supports multi-level
  inheritance with proper C3 linearization deduplication.
- **User-defined value types (`type X is Y`)**: Transparent type aliases where
  `wrap`/`unwrap` compile to no-ops on NeoVM. Supported at both file-level and
  contract-level, with proper propagation through inheritance chains.
- `SuperShowcase.sol` and `SuperError.sol` examples demonstrating `super`
  keyword usage patterns.
- `UserDefinedTypeError.sol` example now compiles successfully with `type Price
is uint256` and `Price.wrap(...)`.
- **`type(T).name` expression**: `type(ContractName).name` and
  `type(uint256).name` now resolve to compile-time string constants on NeoVM.
  Completes `type(...)` expression support alongside `.min`, `.max`, and
  `.interfaceId`.
- `TypeNameShowcase.sol` example demonstrating `type(T).name` patterns.
- **`require(condition, CustomError(...))` support**: Solidity 0.8.26+ syntax
  now compiles correctly. Error name and argument count are preserved in the
  NeoVM THROW message for diagnostics (e.g., `"InsufficientBalance(2 args)"`).
- `RequireCustomErrorShowcase.sol` example demonstrating the three `require`
  forms: plain condition, string message, and custom error.

- `LoweringContext` extended with `super_method_map` for `super` keyword
  resolution during IR lowering.
- `ContractIR` and `ContractMetadata` extended with `type_aliases` field for
  user-defined value type support.
- `NeoType::from_solidity_with_aliases` added as alias-aware type resolution
  wrapper around `NeoType::from_solidity`.
- Inheritance flattening (`flatten.rs`) now merges type aliases and preserves
  `__super_` methods through the C3 linearization chain.

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
