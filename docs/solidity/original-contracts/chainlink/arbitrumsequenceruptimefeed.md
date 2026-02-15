# ArbitrumSequencerUptimeFeed (Chainlink)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@chainlink/contracts/src/v0.8/l2ep/arbitrum/ArbitrumSequencerUptimeFeed.sol`
- Primary issue: duplicate state variable 'offset'
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `duplicate_state_var`
- Need on Neo (from audit): 需要修复状态变量命名冲突解析（编译器语义分析）或在源码层拆分冲突字段

### Migration Playbook: Duplicate state variable symbol

1. Rename colliding state variables across inheritance hierarchy.
1. Consolidate duplicated storage slots into one authoritative declaration.
1. Review overshadowing patterns introduced by upgrades/merges.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'offset' |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@chainlink/contracts/src/v0.8/l2ep/arbitrum/ArbitrumSequencerUptimeFeed.sol`