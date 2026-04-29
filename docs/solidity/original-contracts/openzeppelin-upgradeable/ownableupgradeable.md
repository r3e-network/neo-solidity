# OwnableUpgradeable (OpenZeppelin Upgradeable)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol`
- Primary issue: No primary issue recorded.
- Audit corpus size: 98 contracts

## NeoVM Adaptation Status

This upstream contract compiled successfully in the audit run with current `neo-solc`.

Recommended hardening before production deployment:

1. Review generated manifest permissions and remove wildcard entries when possible.
1. Run Neo-Express state-changing tests for your target workflows, not only read-only calls.
1. Validate semantic differences (for example `tx.origin`, payable semantics, callback models) for your integration context.

## Diagnostics

Total diagnostics captured: `4`

### By Severity

| Severity | Count |
| --- | ---: |
| warning | 4 |

### Most Common Codes

| Code | Count | Example |
| --- | ---: | --- |
| INVALID_STORAGE_RETURN | 4 | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |

Full diagnostic payloads are kept in `docs/data/famous-contracts-audit-results.json`; this page summarizes them so the docs remain navigable.

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol`