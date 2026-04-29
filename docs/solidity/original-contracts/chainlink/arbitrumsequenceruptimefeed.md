# ArbitrumSequencerUptimeFeed (Chainlink)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/l2ep/arbitrum/ArbitrumSequencerUptimeFeed.sol`
- Primary issue: No primary issue recorded.
- Audit corpus size: 98 contracts

## NeoVM Adaptation Status

This upstream contract compiled successfully in the audit run with current `neo-solc`.

Recommended hardening before production deployment:

1. Review generated manifest permissions and remove wildcard entries when possible.
1. Run Neo-Express state-changing tests for your target workflows, not only read-only calls.
1. Validate semantic differences (for example `tx.origin`, payable semantics, callback models) for your integration context.

## Diagnostics

Total diagnostics captured: `53`

### By Severity

| Severity | Count |
| --- | ---: |
| warning | 53 |

### Most Common Codes

| Code | Count | Example |
| --- | ---: | --- |
| W200 | 40 | function 'applyL1ToL2Alias' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::applyL1ToL2Alias' which is not marked 'virtual' |
| W121 | 10 | duplicate constant state variable 'offset' detected while merging libraries |
| RAW | 2 | warning: tx.origin has different semantics on Neo N3. Neo uses multi-signature witnesses instead of a single origin. Consider using msg.sender or Runtime.CheckWitness() instead. |
| MANIFEST_WILDCARD_CONTRACT | 1 | contract 'ArbitrumSequencerUptimeFeed' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

Full diagnostic payloads are kept in `docs/data/famous-contracts-audit-results.json`; this page summarizes them so the docs remain navigable.

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/l2ep/arbitrum/ArbitrumSequencerUptimeFeed.sol`