# ERC20Capped (OpenZeppelin)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@openzeppelin/contracts/token/ERC20/extensions/ERC20Capped.sol`
- Primary issue: modifier/constructor argument mismatch: expected 2, got 0
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `ctor_modifier_mismatch`
- Need on Neo (from audit): 需要修复构造器/修饰器参数传递路径，或扩展编译器对复杂构造器链的 lowering

### Migration Playbook: Constructor/modifier argument mismatch

1. Align constructor arguments through the full inheritance chain.
1. Avoid hidden parameter propagation through modifiers in constructors.
1. Move complex initialization into explicit `initialize` routines when practical.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | GENERIC_ERROR | modifier/constructor argument mismatch: expected 2, got 0 |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@openzeppelin/contracts/token/ERC20/extensions/ERC20Capped.sol`