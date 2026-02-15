# ERC20PermitUpgradeable (OpenZeppelin Upgradeable)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20PermitUpgradeable.sol`
- Primary issue: overloaded function 'modExp' with 3 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime overloaded function 'tryModExp' with 3 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `abi_overload`
- Need on Neo (from audit): 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度

### Migration Playbook: ABI overload collision on Neo

1. Rename public/external overloads so each exposed method has a unique name.
1. Keep overloaded variants internal/private if overloading is required for code reuse.
1. If upstream API compatibility is required, add a thin adapter layer that maps unique Neo entrypoints to canonical behavior.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | GENERIC_ERROR | overloaded function 'modExp' with 3 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime
overloaded function 'tryModExp' with 3 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20PermitUpgradeable.sol`