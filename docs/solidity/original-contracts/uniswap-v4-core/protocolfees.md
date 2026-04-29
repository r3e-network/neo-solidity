# ProtocolFees (Uniswap V4 Core)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@uniswap/v4-core/src/ProtocolFees.sol`
- Primary issue: No primary issue recorded.
- Audit corpus size: 98 contracts

## NeoVM Adaptation Status

This upstream contract compiled successfully in the audit run with current `neo-solc`.

Recommended hardening before production deployment:

1. Review generated manifest permissions and remove wildcard entries when possible.
1. Run Neo-Express state-changing tests for your target workflows, not only read-only calls.
1. Validate semantic differences (for example `tx.origin`, payable semantics, callback models) for your integration context.

## Diagnostics

Total diagnostics captured: `218`

### By Severity

| Severity | Count |
| --- | ---: |
| warning | 218 |

### Most Common Codes

| Code | Count | Example |
| --- | ---: | --- |
| W200 | 180 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| W121 | 31 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| INVALID_STORAGE_RETURN | 4 | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| MANIFEST_WILDCARD_CONTRACT | 2 | contract 'Owned' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| VALIDATION_WARNING | 1 | abstract contract 'ProtocolFees' has 2 unimplemented function(s): [_isUnlocked, _getPool] |

Full diagnostic payloads are kept in `docs/data/famous-contracts-audit-results.json`; this page summarizes them so the docs remain navigable.

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@uniswap/v4-core/src/ProtocolFees.sol`