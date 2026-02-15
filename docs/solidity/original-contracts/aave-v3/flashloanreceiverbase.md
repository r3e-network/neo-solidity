# FlashLoanReceiverBase (Aave V3)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@aave/core-v3/contracts/flashloan/base/FlashLoanReceiverBase.sol`
- Primary issue: function 'constructor': unsupported function call 'IPool' help: check spelling or ensure the function is declared in the same contract
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `other`
- Need on Neo (from audit): 需要扩展 neo-solidity 对该语义的 IR lowering，或用 Neo 等价模式重写该模块

### Migration Playbook: General compiler compatibility gap

1. Use diagnostics to isolate the minimal failing construct.
1. Refactor toward Neo-native patterns (`Runtime`, `Syscalls`, `NativeCalls`).
1. Open a focused compiler issue with a minimized reproducer when behavior should be supported.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | IR_GENERATION_ERROR | function 'constructor': unsupported function call 'IPool'
  help: check spelling or ensure the function is declared in the same contract |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@aave/core-v3/contracts/flashloan/base/FlashLoanReceiverBase.sol`