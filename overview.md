# v0.28.1 — Audit-Driven Patch Release

## What was done

All P2 and P3 issues from the comprehensive v0.28.0 audit report have been fixed.

### P2 Fixes (Moderate — Should Fix)

1. **P2-1: Nested EQUAL type-strictness** — `stack_items_equal()` now recursively compares Array/Map elements instead of delegating to `PartialEq` (which ignored `type_tag`). `PartialEq` impl for `ByteArray` also checks `type_tag`.

2. **P2-2: NativeTypes.ContractState type mismatch** — `hash` → `address`, `id` → `int256`, `updateCounter` → `uint256` (matching NeoVM wire format).

3. **P2-3: 20 missing runtime handlers** — All implemented:
   - NEO (5): getAllCandidates, getCandidateVote, getRegisterPrice, setRegisterPrice, unclaimedGas
   - ContractMgmt (4): destroyContract, getContractById, listContracts, setMinimumDeploymentFee
   - StdLib (10): base64UrlEncode/Decode, base58Encode/Decode, base58CheckEncode/Decode, memoryCompare, memorySearch, stringSplit, strLen
   - CryptoLib (1): verifyWithEd25519 (via ed25519-dalek)

4. **P2-4/P2-5: Dead code removal** — Removed `build_storage_entries`, `allocate_iterator`. Fixed VMBridge `eq_stack_items` type_tag check.

### P3 Fixes (Low — Nice to Fix)

5. **P3-1: 7 clippy warnings** — All fixed. `cargo clippy` reports 0 warnings.

6. **P3-2: 10 unused imports** — Removed unused `SyscallsTypes.sol` (3 files) and `NativeTypes.sol` (7 files) imports.

7. **P3-3: 9 unused `using Syscalls for *;`** — Removed from all 9 native library files.

8. **P3-4: Pragma inconsistency** — 3 compat files changed from `^0.8.20` to `^0.8.19`.

9. **P3-5: 14 missing Solidity wrappers** — Added:
   - CryptoLib (7): sha1, bls12381G1Add/Mul/Neg, bls12381G2Add/Mul/Neg
   - Oracle (1): getOracleRequest
   - Syscalls (6): getMsgValue, getNetwork, getAddressVersion, getInvocationCounter, getRandom, burnGas

## Key decisions

- Added `bs58` and `ed25519-dalek` as optional runtime dependencies for base58 and Ed25519 handlers
- Solidity wrappers added to both domain libraries AND the `Syscalls.sol` aggregate for consistency
- NativeNEO.sol and NativeContractMgmt.sol retain their `NativeTypes.sol` import (it IS used there)

## Verification

- `cargo check`: 0 errors, 0 warnings
- `cargo clippy`: 0 warnings
- `cargo test`: **965 tests passed, 0 failed** (same count as v0.28.0 — zero regressions)
