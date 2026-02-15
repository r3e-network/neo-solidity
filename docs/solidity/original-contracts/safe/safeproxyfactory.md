# SafeProxyFactory (Safe)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@safe-global/safe-contracts/contracts/proxies/SafeProxyFactory.sol`
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
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'SafeProxyFactory' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@safe-global/safe-contracts/contracts/proxies/SafeProxyFactory.sol`