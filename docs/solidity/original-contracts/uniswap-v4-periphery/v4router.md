# V4Router (Uniswap V4 Periphery)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@uniswap/v4-periphery/src/V4Router.sol`
- Primary issue: Error resolving imports: import cycle detected: node_modules/@uniswap/v4-periphery/src/V4Router.sol -&gt; node_modules/@uniswap/v4-core/src/interfaces/IPoolManager.sol -&gt; node_modules/@uniswap/v4-core/src/types/PoolKey.sol -&gt; node_modules/@uniswap/v4-core/src/interfaces/IHooks.sol -&gt; node_modules/@uniswap/v4-core/src/types/PoolKey.sol
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `import_cycle`
- Need on Neo (from audit): 需要编译器导入解析支持循环依赖图，或对上游源码做解环拆分

### Migration Playbook: Import cycle in source graph

1. Break cyclic dependencies by extracting interfaces and shared structs to leaf modules.
1. Split contract logic into acyclic layers (`interfaces` -> `base` -> `impl`).
1. Avoid barrel imports that re-export modules participating in cycles.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | RAW | Error resolving imports: import cycle detected: node_modules/@uniswap/v4-periphery/src/V4Router.sol -&gt; node_modules/@uniswap/v4-core/src/interfaces/IPoolManager.sol -&gt; node_modules/@uniswap/v4-core/src/types/PoolKey.sol -&gt; node_modules/@uniswap/v4-core/src/interfaces/IHooks.sol -&gt; node_modules/@uniswap/v4-core/src/types/PoolKey.sol |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@uniswap/v4-periphery/src/V4Router.sol`