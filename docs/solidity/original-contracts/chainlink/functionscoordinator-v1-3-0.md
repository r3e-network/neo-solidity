# FunctionsCoordinator_v1_3_0 (Chainlink)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/functions/v1_3_0/FunctionsCoordinator.sol`
- Primary issue: No primary issue recorded.
- Audit corpus size: 98 contracts

## NeoVM Adaptation Status

This upstream contract compiled successfully in the audit run with current `neo-solc`.

Recommended hardening before production deployment:

1. Review generated manifest permissions and remove wildcard entries when possible.
1. Run Neo-Express state-changing tests for your target workflows, not only read-only calls.
1. Validate semantic differences (for example `tx.origin`, payable semantics, callback models) for your integration context.

## Diagnostics

Total diagnostics captured: `1864`

### By Severity

| Severity | Count |
| --- | ---: |
| warning | 1864 |

### Most Common Codes

| Code | Count | Example |
| --- | ---: | --- |
| W200 | 1474 | function 'toUint248' in 'FunctionsBilling' overrides 'Routable::toUint248' which is not marked 'virtual' |
| W121 | 374 | duplicate constant state variable 'L2_TO_L1_MESSAGE_PASSER' detected while merging libraries |
| RAW | 11 | warning: block.basefee auto-mapped to Policy.getFeePerByte() on Neo N3. Neo uses a fixed fee structure, not EIP-1559 base fees. |
| VALIDATION_WARNING | 3 | abstract contract 'FunctionsBilling' has 3 unimplemented function(s): [_getTransmitters, _onlyOwner, _owner] |
| MANIFEST_WILDCARD_CONTRACT | 2 | contract 'FunctionsBilling' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

Full diagnostic payloads are kept in `docs/data/famous-contracts-audit-results.json`; this page summarizes them so the docs remain navigable.

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `/Users/jinghuiliao/git/neo-solidity/third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/functions/v1_3_0/FunctionsCoordinator.sol`