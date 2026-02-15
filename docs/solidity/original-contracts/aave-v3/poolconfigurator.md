# PoolConfigurator (Aave V3)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@aave/core-v3/contracts/protocol/pool/PoolConfigurator.sol`
- Primary issue: function 'executeInitReserve' parameter 'pool' uses unsupported type 'IPool' function 'executeInitReserve' parameter 'input' uses unsupported type 'ConfiguratorInputTypes.InitReserveInput' function 'executeUpdateAToken' parameter 'cachedPool' uses unsupported type 'IPool' function 'executeUpdateAToken' parameter 'input' uses unsupported type 'ConfiguratorInputTypes.UpdateATokenInput' function 'executeUpdateStableDebtToken' parameter 'cachedPool' uses unsupported type 'IPool' function 'executeUpdateStableDebtToken' parameter 'input' uses unsupported type 'ConfiguratorInputTypes.UpdateDebtTokenInput' function 'executeUpdateVariableDebtToken' parameter 'cachedPool' uses unsupported type 'IPool' function 'executeUpdateVariableDebtToken' parameter 'input' uses unsupported type 'ConfiguratorInputTypes.UpdateDebtTokenInput'
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `unsupported_param_type`
- Need on Neo (from audit): 需要扩展接口/结构体参数类型 lowering（复杂参数序列化），或先重构为基础类型边界

### Migration Playbook: Unsupported parameter type at contract boundary

1. Replace complex interface/struct boundary types with primitive ABI-safe values.
1. Pass opaque bytes payloads and decode internally when interoperability is needed.
1. Keep cross-contract entrypoints narrow and use internal adapters for complex types.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | GENERIC_ERROR | function 'executeInitReserve' parameter 'pool' uses unsupported type 'IPool'
function 'executeInitReserve' parameter 'input' uses unsupported type 'ConfiguratorInputTypes.InitReserveInput'
function 'executeUpdateAToken' parameter 'cachedPool' uses unsupported type 'IPool'
function 'executeUpdateAToken' parameter 'input' uses unsupported type 'ConfiguratorInputTypes.UpdateATokenInput'
function 'executeUpdateStableDebtToken' parameter 'cachedPool' uses unsupported type 'IPool'
function 'executeUpdateStableDebtToken' parameter 'input' uses unsupported type 'ConfiguratorInputTypes.UpdateDebtTokenInput'
function 'executeUpdateVariableDebtToken' parameter 'cachedPool' uses unsupported type 'IPool'
function 'executeUpdateVariableDebtToken' parameter 'input' uses unsupported type 'ConfiguratorInputTypes.UpdateDebtTokenInput' |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@aave/core-v3/contracts/protocol/pool/PoolConfigurator.sol`