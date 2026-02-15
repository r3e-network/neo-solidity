# VRFCoordinatorV2_5 (Chainlink)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/vrf/dev/VRFCoordinatorV2_5.sol`
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
| warning | RAW | warning: blockhash() auto-mapped to Ledger.getBlockHash() on Neo N3. Returns the block hash for the given index. |
| warning | RAW | warning: blockhash() auto-mapped to Ledger.getBlockHash() on Neo N3. Returns the block hash for the given index. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: blockhash() auto-mapped to Ledger.getBlockHash() on Neo N3. Returns the block hash for the given index. |
| warning | W121 | duplicate constant state variable 'EXTRA_ARGS_V1_TAG' detected while merging libraries |
| warning | W200 | function '_add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_argsToBytes' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_argsToBytes' which is not marked 'virtual' |
| warning | W200 | function '_argsToBytes' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W111 | function 'fundSubscriptionWithNative' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'EXTRA_ARGS_V1_TAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EXTRA_ARGS_V1_TAG' detected while merging libraries |
| warning | RETURN_TYPE_UNMAPPED | function 's_config' returns 'Any', which may not map cleanly to Neo manifest types |
| warning | W116 | function 'fundSubscriptionWithNative' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function '_add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_argsToBytes' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_argsToBytes' which is not marked 'virtual' |
| warning | W200 | function '_argsToBytes' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'SubscriptionAPI' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'SubscriptionAPI' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'SubscriptionAPI' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'SubscriptionAPI' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'SubscriptionAPI' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'SubscriptionAPI' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'SubscriptionAPI' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'SubscriptionAPI' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'SubscriptionAPI' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'SubscriptionAPI' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'SubscriptionAPI' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'SubscriptionAPI' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'SubscriptionAPI' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'SubscriptionAPI' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'SubscriptionAPI' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'SubscriptionAPI' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'SubscriptionAPI' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'SubscriptionAPI' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function '_argsToBytes' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_argsToBytes' which is not marked 'virtual' |
| warning | W200 | function '_argsToBytes' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'SubscriptionAPI' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'EXTRA_ARGS_V1_TAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EXTRA_ARGS_V1_TAG' detected while merging libraries |
| warning | VALIDATION_WARNING | abstract contract 'VRFConsumerBaseV2Plus' has 1 unimplemented function(s): [fulfillRandomWords] |
| warning | W200 | function '_add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_argsToBytes' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_argsToBytes' which is not marked 'virtual' |
| warning | W200 | function '_argsToBytes' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W200 | function '_argsToBytes' in 'VRFConsumerBaseV2Plus' overrides 'ConfirmedOwner::_argsToBytes' which is not marked 'virtual' |
| warning | W200 | function '_argsToBytes' in 'VRFConsumerBaseV2Plus' overrides a base function but is not marked 'override' |
| warning | W111 | function 'fundSubscriptionWithNative' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'EXTRA_ARGS_V1_TAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EXTRA_ARGS_V1_TAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EXTRA_ARGS_V1_TAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EXTRA_ARGS_V1_TAG' detected while merging libraries |
| warning | RETURN_TYPE_UNMAPPED | function 's_config' returns 'Any', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 's_provingKeys' returns 'Any', which may not map cleanly to Neo manifest types |
| warning | W116 | function 'fundSubscriptionWithNative' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function '_add' in 'ConfirmedOwnerWithProposal' overrides 'VRF::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'ConfirmedOwnerWithProposal' overrides 'VRF::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'ConfirmedOwnerWithProposal' overrides 'VRF::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'ConfirmedOwnerWithProposal' overrides 'VRF::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'ConfirmedOwnerWithProposal' overrides 'VRF::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'ConfirmedOwnerWithProposal' overrides 'VRF::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwnerWithProposal' overrides 'VRF::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwnerWithProposal' overrides 'VRF::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwnerWithProposal' overrides 'VRF::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwnerWithProposal' overrides 'VRF::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwnerWithProposal' overrides 'VRF::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwnerWithProposal' overrides 'VRF::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwnerWithProposal' overrides 'VRF::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwnerWithProposal' overrides 'VRF::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwnerWithProposal' overrides 'VRF::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwnerWithProposal' overrides 'VRF::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwnerWithProposal' overrides 'VRF::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwnerWithProposal' overrides 'VRF::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwnerWithProposal' overrides 'VRF::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwnerWithProposal' overrides 'VRF::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwnerWithProposal' overrides 'VRF::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwnerWithProposal' overrides 'VRF::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwnerWithProposal' overrides 'VRF::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwnerWithProposal' overrides 'VRF::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function '_argsToBytes' in 'ConfirmedOwnerWithProposal' overrides 'VRF::_argsToBytes' which is not marked 'virtual' |
| warning | W200 | function '_argsToBytes' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_argsToBytes' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_argsToBytes' which is not marked 'virtual' |
| warning | W200 | function '_argsToBytes' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'SubscriptionAPI' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'SubscriptionAPI' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'SubscriptionAPI' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'SubscriptionAPI' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'SubscriptionAPI' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'SubscriptionAPI' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'SubscriptionAPI' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'SubscriptionAPI' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'SubscriptionAPI' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'SubscriptionAPI' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'SubscriptionAPI' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'SubscriptionAPI' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'SubscriptionAPI' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'SubscriptionAPI' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'SubscriptionAPI' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'SubscriptionAPI' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'SubscriptionAPI' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'SubscriptionAPI' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function '_argsToBytes' in 'SubscriptionAPI' overrides 'ConfirmedOwner::_argsToBytes' which is not marked 'virtual' |
| warning | W200 | function '_argsToBytes' in 'SubscriptionAPI' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | W200 | function '_argsToBytes' in 'VRFCoordinatorV2_5' overrides 'SubscriptionAPI::_argsToBytes' which is not marked 'virtual' |
| warning | W200 | function '_argsToBytes' in 'VRFCoordinatorV2_5' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'VRFCoordinatorV2_5' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/vrf/dev/VRFCoordinatorV2_5.sol`