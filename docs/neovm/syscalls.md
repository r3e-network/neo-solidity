# Syscalls

The compiler/runtime implement NeoVM syscall surfaces used by generated contracts and devpack intrinsics.

## Implemented categories

- Storage: context/get/put/delete/find + iterator operations
- Runtime: trigger/platform/network/time/gas/calling/executing script hashes, notify/log/checkWitness
- Crypto: SHA256/RIPEMD160/Keccak256/Murmur32/Hash160/Hash256/signature checks
- Contract: call/getContract/call flags/account creation helpers
- Oracle/Policy/ContractManagement integration paths

Reference: [`docs/RUNTIME_SPEC.md`](../RUNTIME_SPEC.md).

## Devpack syscall wrapper

Use `devpack/contracts/Syscalls.sol` for Solidity-facing signatures and helper structs.

Important notes:

- Many platform features are native-contract methods reached via `System.Contract.Call`.
- Some API-compatible helpers accept extra parameters reserved for forward compatibility.

## Gas and parity notes

The embedded runtime uses syscall gas hint tables and provides high fidelity for compilation/testing, while documenting remaining precision/parity gaps in:

- [Runtime Spec](/reference/runtime)
- [Parity and Limitations](/reference/parity-limitations)
