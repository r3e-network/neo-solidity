# SignMessageLib (Safe)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@safe-global/safe-contracts/contracts/libraries/SignMessageLib.sol`
- Primary issue: function 'execute': unknown identifier 'Enum' help: check spelling or ensure the variable is declared in the same contract or an imported library
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
| error | IR_GENERATION_ERROR | function 'execute': unknown identifier 'Enum'
  help: check spelling or ensure the variable is declared in the same contract or an imported library |
| error | IR_GENERATION_ERROR | function 'execute': inline assembly is not supported
  help: Neo N3 uses NeoVM opcodes; use NativeCalls.sol for low-level operations |
| error | IR_GENERATION_ERROR | function 'execute': inline assembly is not supported
  help: Neo N3 uses NeoVM opcodes; use NativeCalls.sol for low-level operations |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@safe-global/safe-contracts/contracts/libraries/SignMessageLib.sol`