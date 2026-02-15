# DataFeedsCache (Chainlink)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@chainlink/contracts/src/v0.8/data-feeds/DataFeedsCache.sol`
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
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ConfirmedOwnerWithProposal' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W200 | function 'sendValue' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'forceApprove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::forceApprove' which is not marked 'virtual' |
| warning | W200 | function 'forceApprove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturnBool' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_callOptionalReturnBool' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturnBool' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ConfirmedOwner' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W200 | function 'sendValue' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'forceApprove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::forceApprove' which is not marked 'virtual' |
| warning | W200 | function 'forceApprove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturnBool' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_callOptionalReturnBool' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturnBool' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'OwnerIsCreator' overrides 'ConfirmedOwner::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'OwnerIsCreator' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'OwnerIsCreator' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'OwnerIsCreator' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'OwnerIsCreator' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'OwnerIsCreator' overrides 'ConfirmedOwner::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'OwnerIsCreator' overrides 'ConfirmedOwner::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'OwnerIsCreator' overrides 'ConfirmedOwner::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'OwnerIsCreator' overrides 'ConfirmedOwner::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'OwnerIsCreator' overrides 'ConfirmedOwner::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'OwnerIsCreator' overrides 'ConfirmedOwner::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'OwnerIsCreator' overrides 'ConfirmedOwner::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'forceApprove' in 'OwnerIsCreator' overrides 'ConfirmedOwner::forceApprove' which is not marked 'virtual' |
| warning | W200 | function 'forceApprove' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'OwnerIsCreator' overrides 'ConfirmedOwner::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturnBool' in 'OwnerIsCreator' overrides 'ConfirmedOwner::_callOptionalReturnBool' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturnBool' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'OwnerIsCreator' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W200 | function 'sendValue' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'forceApprove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::forceApprove' which is not marked 'virtual' |
| warning | W200 | function 'forceApprove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturnBool' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_callOptionalReturnBool' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturnBool' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'OwnerIsCreator' overrides 'ConfirmedOwner::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'OwnerIsCreator' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'OwnerIsCreator' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'OwnerIsCreator' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'OwnerIsCreator' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'OwnerIsCreator' overrides 'ConfirmedOwner::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'OwnerIsCreator' overrides 'ConfirmedOwner::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'OwnerIsCreator' overrides 'ConfirmedOwner::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'OwnerIsCreator' overrides 'ConfirmedOwner::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'OwnerIsCreator' overrides 'ConfirmedOwner::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'OwnerIsCreator' overrides 'ConfirmedOwner::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'OwnerIsCreator' overrides 'ConfirmedOwner::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'forceApprove' in 'OwnerIsCreator' overrides 'ConfirmedOwner::forceApprove' which is not marked 'virtual' |
| warning | W200 | function 'forceApprove' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'OwnerIsCreator' overrides 'ConfirmedOwner::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturnBool' in 'OwnerIsCreator' overrides 'ConfirmedOwner::_callOptionalReturnBool' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturnBool' in 'OwnerIsCreator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'DataFeedsCache' overrides 'OwnerIsCreator::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'DataFeedsCache' overrides 'OwnerIsCreator::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'DataFeedsCache' overrides 'OwnerIsCreator::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'DataFeedsCache' overrides 'OwnerIsCreator::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'DataFeedsCache' overrides 'OwnerIsCreator::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'DataFeedsCache' overrides 'OwnerIsCreator::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'DataFeedsCache' overrides 'OwnerIsCreator::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'DataFeedsCache' overrides 'OwnerIsCreator::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'DataFeedsCache' overrides 'OwnerIsCreator::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'DataFeedsCache' overrides 'OwnerIsCreator::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'DataFeedsCache' overrides 'OwnerIsCreator::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'DataFeedsCache' overrides 'OwnerIsCreator::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function 'forceApprove' in 'DataFeedsCache' overrides 'OwnerIsCreator::forceApprove' which is not marked 'virtual' |
| warning | W200 | function 'forceApprove' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'DataFeedsCache' overrides 'OwnerIsCreator::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturnBool' in 'DataFeedsCache' overrides 'OwnerIsCreator::_callOptionalReturnBool' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturnBool' in 'DataFeedsCache' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'DataFeedsCache' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@chainlink/contracts/src/v0.8/data-feeds/DataFeedsCache.sol`