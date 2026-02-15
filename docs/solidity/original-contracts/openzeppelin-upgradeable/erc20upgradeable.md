# ERC20Upgradeable (OpenZeppelin Upgradeable)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol`
- Primary issue: No primary issue recorded.
- Audit corpus size: 98 contracts

## NeoVM Adaptation Status

This upstream contract compiled successfully in the audit run with current `neo-solc`.

Recommended hardening before production deployment:

1. Review generated manifest permissions and remove wildcard entries when possible.
1. Run Neo-Express state-changing tests for your target workflows, not only read-only calls.
1. Validate semantic differences (for example `tx.origin`, payable semantics, callback models) for your integration context.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | RAW | [info][NEP-17] NEP-17 `transfer` method has 2 parameter(s), spec expects 4. See STANDARDS_MAPPING.md for details. |
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '_getERC20Storage' return value 'ERC20Storage' uses 'storage' data location (treated as Any) |
| warning | W101 | function 'transfer' has 2 parameters (ERC-20 pattern). NEP-17 requires 4 parameters: transfer(from, to, amount, data). The `from` address is verified via Runtime.checkWitness() and `data` (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol`