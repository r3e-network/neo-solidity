# FunctionsRouter (Chainlink)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@chainlink/contracts/src/v0.8/functions/v1_0_0/FunctionsRouter.sol`
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
| error | IR_GENERATION_ERROR | function 'safeTransfer': abi.encodeWithSelector is only supported for Neo contract calls (inline it into `address.call(...)` / `address.staticcall(...)`, or assign it to a local `bytes` variable that is later passed to those calls). Raw EVM calldata bytes are not supported on Neo N3; use `Syscalls.contractCall` / `Syscalls.contractCallWithFlags` / `NativeCalls.*` helpers when possible |
| error | IR_GENERATION_ERROR | function 'safeTransferFrom': abi.encodeWithSelector is only supported for Neo contract calls (inline it into `address.call(...)` / `address.staticcall(...)`, or assign it to a local `bytes` variable that is later passed to those calls). Raw EVM calldata bytes are not supported on Neo N3; use `Syscalls.contractCall` / `Syscalls.contractCallWithFlags` / `NativeCalls.*` helpers when possible |
| error | IR_GENERATION_ERROR | function 'safeApprove': abi.encodeWithSelector is only supported for Neo contract calls (inline it into `address.call(...)` / `address.staticcall(...)`, or assign it to a local `bytes` variable that is later passed to those calls). Raw EVM calldata bytes are not supported on Neo N3; use `Syscalls.contractCall` / `Syscalls.contractCallWithFlags` / `NativeCalls.*` helpers when possible |
| error | IR_GENERATION_ERROR | function 'safeIncreaseAllowance': abi.encodeWithSelector is only supported for Neo contract calls (inline it into `address.call(...)` / `address.staticcall(...)`, or assign it to a local `bytes` variable that is later passed to those calls). Raw EVM calldata bytes are not supported on Neo N3; use `Syscalls.contractCall` / `Syscalls.contractCallWithFlags` / `NativeCalls.*` helpers when possible |
| error | IR_GENERATION_ERROR | function 'safeDecreaseAllowance': abi.encodeWithSelector is only supported for Neo contract calls (inline it into `address.call(...)` / `address.staticcall(...)`, or assign it to a local `bytes` variable that is later passed to those calls). Raw EVM calldata bytes are not supported on Neo N3; use `Syscalls.contractCall` / `Syscalls.contractCallWithFlags` / `NativeCalls.*` helpers when possible |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@chainlink/contracts/src/v0.8/functions/v1_0_0/FunctionsRouter.sol`