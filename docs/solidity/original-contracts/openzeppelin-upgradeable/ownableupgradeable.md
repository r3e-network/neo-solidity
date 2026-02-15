# OwnableUpgradeable (OpenZeppelin Upgradeable)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol`
- Primary issue: inheritance linearization failed for 'OwnableUpgradeable': inconsistent base order
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `inheritance_linearization`
- Need on Neo (from audit): 需要调整继承层次（或扩展编译器的 C3 线性化兼容），避免多重继承顺序冲突

### Migration Playbook: Inheritance linearization conflict

1. Reorder base contracts to satisfy C3 linearization constraints.
1. Split conflicting base behaviors into composition-style helper contracts.
1. Reduce deep diamond inheritance trees before compiling to NeoVM.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | GENERIC_ERROR | inheritance linearization failed for 'OwnableUpgradeable': inconsistent base order |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol`