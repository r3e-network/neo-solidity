# UUPSUpgradeable (OpenZeppelin)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol`
- Primary issue: No primary issue recorded.
- Audit corpus size: 98 contracts

## NeoVM Adaptation Status

This upstream contract compiled successfully in the audit run with current `neo-solc`.

Recommended hardening before production deployment:

1. Review generated manifest permissions and remove wildcard entries when possible.
1. Run Neo-Express state-changing tests for your target workflows, not only read-only calls.
1. Validate semantic differences (for example `tx.origin`, payable semantics, callback models) for your integration context.

## Diagnostics

Total diagnostics captured: `13`

### By Severity

| Severity | Count |
| --- | ---: |
| warning | 13 |

### Most Common Codes

| Code | Count | Example |
| --- | ---: | --- |
| INVALID_STORAGE_RETURN | 9 | function 'getAddressSlot' return value 'AddressSlot' uses 'storage' data location (treated as Any) |
| MANIFEST_WILDCARD_CONTRACT | 1 | contract 'UUPSUpgradeable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| VALIDATION_WARNING | 1 | abstract contract 'UUPSUpgradeable' has 1 unimplemented function(s): [_authorizeUpgrade] |
| W111 | 1 | function 'upgradeToAndCall' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| W116 | 1 | function 'upgradeToAndCall' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |

Full diagnostic payloads are kept in `docs/data/famous-contracts-audit-results.json`; this page summarizes them so the docs remain navigable.

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol`