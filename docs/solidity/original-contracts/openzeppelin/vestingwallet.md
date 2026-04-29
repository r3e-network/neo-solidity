# VestingWallet (OpenZeppelin)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@openzeppelin/contracts/finance/VestingWallet.sol`
- Primary issue: No primary issue recorded.
- Audit corpus size: 98 contracts

## NeoVM Adaptation Status

This upstream contract compiled successfully in the audit run with current `neo-solc`.

Recommended hardening before production deployment:

1. Review generated manifest permissions and remove wildcard entries when possible.
1. Run Neo-Express state-changing tests for your target workflows, not only read-only calls.
1. Validate semantic differences (for example `tx.origin`, payable semantics, callback models) for your integration context.

## Diagnostics

Total diagnostics captured: `124`

### By Severity

| Severity | Count |
| --- | ---: |
| warning | 124 |

### Most Common Codes

| Code | Count | Example |
| --- | ---: | --- |
| W200 | 120 | function 'safeTransfer' in 'Ownable' overrides 'Context::safeTransfer' which is not marked 'virtual' |
| MANIFEST_WILDCARD_CONTRACT | 3 | contract 'Context' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| W111 | 1 | function 'onNEP17Payment' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |

Full diagnostic payloads are kept in `docs/data/famous-contracts-audit-results.json`; this page summarizes them so the docs remain navigable.

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@openzeppelin/contracts/finance/VestingWallet.sol`