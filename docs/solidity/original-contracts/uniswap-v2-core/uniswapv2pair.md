# UniswapV2Pair (Uniswap V2 Core)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@uniswap/v2-core/contracts/UniswapV2Pair.sol`
- Primary issue: Unsupported Solidity version: =0.5.16
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `solidity_version`
- Need on Neo (from audit): 需要将源码迁移到 Solidity 0.8.x 范围并处理破坏性变更

### Migration Playbook: Solidity version outside compiler support range

1. Upgrade source pragmas and syntax to Solidity 0.8.x.
1. Replace legacy patterns (for example legacy SafeMath flows and old constructor style) with 0.8-native code.
1. Re-run audit after each migration step to isolate non-version blockers.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | GENERIC_ERROR | Unsupported Solidity version: =0.5.16 |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@uniswap/v2-core/contracts/UniswapV2Pair.sol`