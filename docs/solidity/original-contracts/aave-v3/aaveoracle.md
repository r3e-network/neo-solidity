# AaveOracle (Aave V3)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@aave/core-v3/contracts/misc/AaveOracle.sol`
- Primary issue: function '_setAssetsSources': unknown identifier 'Errors' help: check spelling or ensure the variable is declared in the same contract or an imported library
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `name_resolution`
- Need on Neo (from audit): 需要补齐对应语义 lowering（命名解析/继承展平），或对源码做 Neo 兼容重写

### Migration Playbook: Name resolution / symbol flattening gap

1. Fully qualify symbol access and reduce implicit inheritance lookups.
1. Refactor ambiguous symbols into explicit library/internal calls.
1. Minimize cross-file wildcard imports to simplify resolution.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | IR_GENERATION_ERROR | function '_setAssetsSources': unknown identifier 'Errors'
  help: check spelling or ensure the variable is declared in the same contract or an imported library |
| error | IR_GENERATION_ERROR | function 'getAssetPrice': unsupported external/library call 'latestAnswer' |
| error | IR_GENERATION_ERROR | function '_onlyAssetListingOrPoolAdmins': unsupported function call 'IACLManager'
  help: check spelling or ensure the function is declared in the same contract |
| error | IR_GENERATION_ERROR | function '_onlyAssetListingOrPoolAdmins': unsupported external/library call 'isAssetListingAdmin' |
| error | IR_GENERATION_ERROR | function '_onlyAssetListingOrPoolAdmins': unknown identifier 'Errors'
  help: check spelling or ensure the variable is declared in the same contract or an imported library |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@aave/core-v3/contracts/misc/AaveOracle.sol`