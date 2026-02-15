# ArbitrumSequencerUptimeFeed (Chainlink)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/l2ep/arbitrum/ArbitrumSequencerUptimeFeed.sol`
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
| warning | RAW | warning: tx.origin has different semantics on Neo N3. Neo uses multi-signature witnesses instead of a single origin. Consider using msg.sender or Runtime.CheckWitness() instead. |
| warning | RAW | warning: tx.origin has different semantics on Neo N3. Neo uses multi-signature witnesses instead of a single origin. Consider using msg.sender or Runtime.CheckWitness() instead. |
| warning | W121 | duplicate constant state variable 'offset' detected while merging libraries |
| warning | W200 | function 'applyL1ToL2Alias' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::applyL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'applyL1ToL2Alias' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'undoL1ToL2Alias' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::undoL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'undoL1ToL2Alias' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W121 | duplicate constant state variable 'offset' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'offset' detected while merging libraries |
| warning | W200 | function 'applyL1ToL2Alias' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::applyL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'applyL1ToL2Alias' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'undoL1ToL2Alias' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::undoL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'undoL1ToL2Alias' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyL1ToL2Alias' in 'SimpleWriteAccessController' overrides 'ConfirmedOwner::applyL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'applyL1ToL2Alias' in 'SimpleWriteAccessController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'undoL1ToL2Alias' in 'SimpleWriteAccessController' overrides 'ConfirmedOwner::undoL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'undoL1ToL2Alias' in 'SimpleWriteAccessController' overrides a base function but is not marked 'override' |
| warning | W121 | duplicate constant state variable 'offset' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'offset' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'offset' detected while merging libraries |
| warning | W200 | function 'applyL1ToL2Alias' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::applyL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'applyL1ToL2Alias' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'undoL1ToL2Alias' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::undoL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'undoL1ToL2Alias' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyL1ToL2Alias' in 'SimpleWriteAccessController' overrides 'ConfirmedOwner::applyL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'applyL1ToL2Alias' in 'SimpleWriteAccessController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'undoL1ToL2Alias' in 'SimpleWriteAccessController' overrides 'ConfirmedOwner::undoL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'undoL1ToL2Alias' in 'SimpleWriteAccessController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyL1ToL2Alias' in 'SimpleReadAccessController' overrides 'SimpleWriteAccessController::applyL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'applyL1ToL2Alias' in 'SimpleReadAccessController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'undoL1ToL2Alias' in 'SimpleReadAccessController' overrides 'SimpleWriteAccessController::undoL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'undoL1ToL2Alias' in 'SimpleReadAccessController' overrides a base function but is not marked 'override' |
| warning | W121 | duplicate constant state variable 'offset' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'offset' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'offset' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'offset' detected while merging libraries |
| warning | W200 | function 'applyL1ToL2Alias' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::applyL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'applyL1ToL2Alias' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'undoL1ToL2Alias' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::undoL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'undoL1ToL2Alias' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyL1ToL2Alias' in 'SimpleWriteAccessController' overrides 'ConfirmedOwner::applyL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'applyL1ToL2Alias' in 'SimpleWriteAccessController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'undoL1ToL2Alias' in 'SimpleWriteAccessController' overrides 'ConfirmedOwner::undoL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'undoL1ToL2Alias' in 'SimpleWriteAccessController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyL1ToL2Alias' in 'SimpleReadAccessController' overrides 'SimpleWriteAccessController::applyL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'applyL1ToL2Alias' in 'SimpleReadAccessController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'undoL1ToL2Alias' in 'SimpleReadAccessController' overrides 'SimpleWriteAccessController::undoL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'undoL1ToL2Alias' in 'SimpleReadAccessController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyL1ToL2Alias' in 'ArbitrumSequencerUptimeFeed' overrides 'SimpleReadAccessController::applyL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'applyL1ToL2Alias' in 'ArbitrumSequencerUptimeFeed' overrides a base function but is not marked 'override' |
| warning | W200 | function 'undoL1ToL2Alias' in 'ArbitrumSequencerUptimeFeed' overrides 'SimpleReadAccessController::undoL1ToL2Alias' which is not marked 'virtual' |
| warning | W200 | function 'undoL1ToL2Alias' in 'ArbitrumSequencerUptimeFeed' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ArbitrumSequencerUptimeFeed' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/l2ep/arbitrum/ArbitrumSequencerUptimeFeed.sol`