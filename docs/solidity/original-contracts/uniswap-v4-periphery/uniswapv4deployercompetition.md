# UniswapV4DeployerCompetition (Uniswap V4 Periphery)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@uniswap/v4-periphery/src/UniswapV4DeployerCompetition.sol`
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
| warning | W117 | function 'competitionDeadline' appears to be time-sensitive. block.timestamp on Neo N3 is deterministic but can be affected by block production timing. |
| warning | W117 | function 'exclusiveDeployDeadline' appears to be time-sensitive. block.timestamp on Neo N3 is deterministic but can be affected by block production timing. |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'UniswapV4DeployerCompetition' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@uniswap/v4-periphery/src/UniswapV4DeployerCompetition.sol`