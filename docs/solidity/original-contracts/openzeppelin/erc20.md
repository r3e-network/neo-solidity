# ERC20 (OpenZeppelin)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@openzeppelin/contracts/token/ERC20/ERC20.sol`
- Primary issue: state variable '_balances' has unsupported type 'mapping(address account =&gt; uint256)'
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `named_mapping`
- Need on Neo (from audit): 需要编译器补齐命名 `mapping(address key =&gt; T)` 语法 lowering，或改写为 `mapping(address =&gt; T)`

### Migration Playbook: Named mapping syntax/shape unsupported in current pipeline

1. Rewrite to plain mapping declarations (for example `mapping(address => uint256)`).
1. Flatten nested mapping wrappers where possible to reduce type complexity.
1. Track compiler updates for full named mapping lowering and migrate back if desired.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | UNSUPPORTED_STATE_TYPE | state variable '_balances' has unsupported type 'mapping(address account =&gt; uint256)' |
| error | UNSUPPORTED_STATE_TYPE | state variable '_allowances' has unsupported type 'mapping(address account =&gt; mapping(address spender =&gt; uint256))' |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@openzeppelin/contracts/token/ERC20/ERC20.sol`