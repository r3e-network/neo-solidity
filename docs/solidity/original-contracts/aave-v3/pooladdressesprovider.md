# PoolAddressesProvider (Aave V3)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@aave/core-v3/contracts/protocol/configuration/PoolAddressesProvider.sol`
- Primary issue: function 'isContract': inline assembly is not supported help: Neo N3 uses NeoVM opcodes; use NativeCalls.sol for low-level operations
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `assembly`
- Need on Neo (from audit): 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写

### Migration Playbook: Inline assembly not supported

1. Replace assembly with high-level Solidity and devpack intrinsics (`Syscalls`, `NativeCalls`, `Runtime`).
1. For low-level call/value movement, use explicit Neo-native APIs instead of EVM opcodes.
1. Isolate assembly-heavy modules and rewrite them first as Neo-specific utility contracts.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | IR_GENERATION_ERROR | function 'isContract': inline assembly is not supported
  help: Neo N3 uses NeoVM opcodes; use NativeCalls.sol for low-level operations |
| error | IR_GENERATION_ERROR | function 'sendValue': function call options (`{...}`) are not supported (value); Neo N3 requires explicit NEP-17 transfers (`NativeCalls.gasTransfer` / `NativeCalls.neoTransfer`) + `onNEP17Payment` |
| error | IR_GENERATION_ERROR | function 'functionCallWithValue': function call options (`{...}`) are not supported (value); Neo N3 requires explicit NEP-17 transfers (`NativeCalls.gasTransfer` / `NativeCalls.neoTransfer`) + `onNEP17Payment` |
| error | IR_GENERATION_ERROR | function 'functionStaticCall': unsupported low-level EVM call 'staticcall'
  help: Neo N3 does not support low-level EVM calls; use NativeCalls.sol for contract-to-contract interactions |
| error | IR_GENERATION_ERROR | function 'functionDelegateCall': unsupported low-level EVM call 'delegatecall'
  help: delegatecall is not available on Neo N3; Neo contracts have isolated storage. Use Syscalls.contractCall() for cross-contract calls |
| error | IR_GENERATION_ERROR | function 'verifyCallResult': inline assembly is not supported
  help: Neo N3 uses NeoVM opcodes; use NativeCalls.sol for low-level operations |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@aave/core-v3/contracts/protocol/configuration/PoolAddressesProvider.sol`