# PoolConfigurator (Aave V3)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@aave/core-v3/contracts/protocol/pool/PoolConfigurator.sol`
- Primary issue: No primary issue recorded.
- Audit corpus size: 98 contracts

## NeoVM Adaptation Status

This upstream contract compiled successfully in the audit run with current `neo-solc`.

Recommended hardening before production deployment:

1. Review generated manifest permissions and remove wildcard entries when possible.
1. Run Neo-Express state-changing tests for your target workflows, not only read-only calls.
1. Validate semantic differences (for example `tx.origin`, payable semantics, callback models) for your integration context.

## Diagnostics

Total diagnostics captured: `2652`

### By Severity

| Severity | Count |
| --- | ---: |
| warning | 2652 |

### Most Common Codes

| Code | Count | Example |
| --- | ---: | --- |
| W121 | 1420 | duplicate constant state variable 'CALLER_NOT_POOL_ADMIN' detected while merging libraries |
| W200 | 1200 | function 'setLtv' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLtv' which is not marked 'virtual' |
| W111 | 9 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| W116 | 9 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| MANIFEST_WILDCARD_CONTRACT | 7 | contract 'VersionedInitializable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| W105 | 5 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| VALIDATION_WARNING | 2 | abstract contract 'VersionedInitializable' has 1 unimplemented function(s): [getRevision] |

Full diagnostic payloads are kept in `docs/data/famous-contracts-audit-results.json`; this page summarizes them so the docs remain navigable.

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@aave/core-v3/contracts/protocol/pool/PoolConfigurator.sol`