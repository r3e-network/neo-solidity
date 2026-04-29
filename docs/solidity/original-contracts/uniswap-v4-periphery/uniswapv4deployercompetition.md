# UniswapV4DeployerCompetition (Uniswap V4 Periphery)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@uniswap/v4-periphery/src/UniswapV4DeployerCompetition.sol`
- Primary issue: No primary issue recorded.
- Audit corpus size: 98 contracts

## NeoVM Adaptation Status

This upstream contract compiled successfully in the audit run with current `neo-solc`.

Recommended hardening before production deployment:

1. Review generated manifest permissions and remove wildcard entries when possible.
1. Run Neo-Express state-changing tests for your target workflows, not only read-only calls.
1. Validate semantic differences (for example `tx.origin`, payable semantics, callback models) for your integration context.

## Diagnostics

Total diagnostics captured: `3`

### By Severity

| Severity | Count |
| --- | ---: |
| warning | 3 |

### Most Common Codes

| Code | Count | Example |
| --- | ---: | --- |
| W117 | 2 | function 'competitionDeadline' appears to be time-sensitive. block.timestamp on Neo N3 is deterministic but can be affected by block production timing. |
| MANIFEST_WILDCARD_CONTRACT | 1 | contract 'UniswapV4DeployerCompetition' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

Full diagnostic payloads are kept in `docs/data/famous-contracts-audit-results.json`; this page summarizes them so the docs remain navigable.

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@uniswap/v4-periphery/src/UniswapV4DeployerCompetition.sol`