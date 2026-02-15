# TimelockController (OpenZeppelin)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@openzeppelin/contracts/governance/TimelockController.sol`
- Primary issue: function 'sendValue': function call options (`{...}`) are not supported (value); Neo N3 requires explicit NEP-17 transfers (`NativeCalls.gasTransfer` / `NativeCalls.neoTransfer`) + `onNEP17Payment`
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `value_call_options`
- Need on Neo (from audit): 需要把 `{value: ...}` 风格调用改成显式 NEP-17 转账（`NativeCalls.gasTransfer` / `NativeCalls.neoTransfer`）并使用 `onNEP17Payment` 回调

### Migration Playbook: EVM call options with value not supported

1. Replace `call{value: ...}` / `send` / `transfer` with `NativeCalls.gasTransfer` or `NativeCalls.neoTransfer`.
1. Receive funds via NEP callback methods (`onNEP17Payment` / `onNEP11Payment`) instead of `receive()`.
1. Separate transfer side effects from contract call logic to keep manifests least-privilege.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | IR_GENERATION_ERROR | function 'sendValue': function call options (`{...}`) are not supported (value); Neo N3 requires explicit NEP-17 transfers (`NativeCalls.gasTransfer` / `NativeCalls.neoTransfer`) + `onNEP17Payment` |
| error | IR_GENERATION_ERROR | function 'functionCallWithValue': function call options (`{...}`) are not supported (value); Neo N3 requires explicit NEP-17 transfers (`NativeCalls.gasTransfer` / `NativeCalls.neoTransfer`) + `onNEP17Payment` |
| error | IR_GENERATION_ERROR | function 'functionStaticCall': unsupported low-level EVM call 'staticcall'
  help: Neo N3 does not support low-level EVM calls; use NativeCalls.sol for contract-to-contract interactions |
| error | IR_GENERATION_ERROR | function 'functionDelegateCall': unsupported low-level EVM call 'delegatecall'
  help: delegatecall is not available on Neo N3; Neo contracts have isolated storage. Use Syscalls.contractCall() for cross-contract calls |
| error | IR_GENERATION_ERROR | function '_revert': inline assembly is not supported
  help: Neo N3 uses NeoVM opcodes; use NativeCalls.sol for low-level operations |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@openzeppelin/contracts/governance/TimelockController.sol`