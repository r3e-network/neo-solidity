# VRFConsumerBaseV2 (Chainlink)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/vrf/VRFConsumerBaseV2.sol`
- Primary issue: No primary issue recorded.
- Audit corpus size: 98 contracts

## NeoVM Adaptation Status

This upstream contract compiled successfully in the audit run with current `neo-solc`.

Recommended hardening before production deployment:

1. Review generated manifest permissions and remove wildcard entries when possible.
1. Run Neo-Express state-changing tests for your target workflows, not only read-only calls.
1. Validate semantic differences (for example `tx.origin`, payable semantics, callback models) for your integration context.

## Diagnostics

Total diagnostics captured: `1`

### By Severity

| Severity | Count |
| --- | ---: |
| warning | 1 |

### Most Common Codes

| Code | Count | Example |
| --- | ---: | --- |
| VALIDATION_WARNING | 1 | abstract contract 'VRFConsumerBaseV2' has 1 unimplemented function(s): [fulfillRandomWords] |

Full diagnostic payloads are kept in `docs/data/famous-contracts-audit-results.json`; this page summarizes them so the docs remain navigable.

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/vrf/VRFConsumerBaseV2.sol`