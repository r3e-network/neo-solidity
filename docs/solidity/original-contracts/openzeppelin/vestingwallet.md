# VestingWallet (OpenZeppelin)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@openzeppelin/contracts/finance/VestingWallet.sol`
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
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Context' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W200 | function 'safeTransfer' in 'Ownable' overrides 'Context::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'Ownable' overrides 'Context::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySafeTransfer' in 'Ownable' overrides 'Context::trySafeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'trySafeTransfer' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySafeTransferFrom' in 'Ownable' overrides 'Context::trySafeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'trySafeTransferFrom' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Ownable' overrides 'Context::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Ownable' overrides 'Context::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'forceApprove' in 'Ownable' overrides 'Context::forceApprove' which is not marked 'virtual' |
| warning | W200 | function 'forceApprove' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transferAndCallRelaxed' in 'Ownable' overrides 'Context::transferAndCallRelaxed' which is not marked 'virtual' |
| warning | W200 | function 'transferAndCallRelaxed' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transferFromAndCallRelaxed' in 'Ownable' overrides 'Context::transferFromAndCallRelaxed' which is not marked 'virtual' |
| warning | W200 | function 'transferFromAndCallRelaxed' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'approveAndCallRelaxed' in 'Ownable' overrides 'Context::approveAndCallRelaxed' which is not marked 'virtual' |
| warning | W200 | function 'approveAndCallRelaxed' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'Ownable' overrides 'Context::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturnBool' in 'Ownable' overrides 'Context::_callOptionalReturnBool' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturnBool' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Ownable' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Ownable' overrides 'Context::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Ownable' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Ownable' overrides 'Context::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Ownable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'onNEP17Payment' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W200 | function 'safeTransfer' in 'Ownable' overrides 'Context::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'Ownable' overrides 'Context::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySafeTransfer' in 'Ownable' overrides 'Context::trySafeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'trySafeTransfer' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySafeTransferFrom' in 'Ownable' overrides 'Context::trySafeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'trySafeTransferFrom' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Ownable' overrides 'Context::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Ownable' overrides 'Context::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'forceApprove' in 'Ownable' overrides 'Context::forceApprove' which is not marked 'virtual' |
| warning | W200 | function 'forceApprove' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transferAndCallRelaxed' in 'Ownable' overrides 'Context::transferAndCallRelaxed' which is not marked 'virtual' |
| warning | W200 | function 'transferAndCallRelaxed' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transferFromAndCallRelaxed' in 'Ownable' overrides 'Context::transferFromAndCallRelaxed' which is not marked 'virtual' |
| warning | W200 | function 'transferFromAndCallRelaxed' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'approveAndCallRelaxed' in 'Ownable' overrides 'Context::approveAndCallRelaxed' which is not marked 'virtual' |
| warning | W200 | function 'approveAndCallRelaxed' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'Ownable' overrides 'Context::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturnBool' in 'Ownable' overrides 'Context::_callOptionalReturnBool' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturnBool' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Ownable' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Ownable' overrides 'Context::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Ownable' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Ownable' overrides 'Context::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'VestingWallet' overrides 'Ownable::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'VestingWallet' overrides 'Ownable::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySafeTransfer' in 'VestingWallet' overrides 'Ownable::trySafeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'trySafeTransfer' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySafeTransferFrom' in 'VestingWallet' overrides 'Ownable::trySafeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'trySafeTransferFrom' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'VestingWallet' overrides 'Ownable::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'VestingWallet' overrides 'Ownable::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'forceApprove' in 'VestingWallet' overrides 'Ownable::forceApprove' which is not marked 'virtual' |
| warning | W200 | function 'forceApprove' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transferAndCallRelaxed' in 'VestingWallet' overrides 'Ownable::transferAndCallRelaxed' which is not marked 'virtual' |
| warning | W200 | function 'transferAndCallRelaxed' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transferFromAndCallRelaxed' in 'VestingWallet' overrides 'Ownable::transferFromAndCallRelaxed' which is not marked 'virtual' |
| warning | W200 | function 'transferFromAndCallRelaxed' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'approveAndCallRelaxed' in 'VestingWallet' overrides 'Ownable::approveAndCallRelaxed' which is not marked 'virtual' |
| warning | W200 | function 'approveAndCallRelaxed' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'VestingWallet' overrides 'Ownable::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturnBool' in 'VestingWallet' overrides 'Ownable::_callOptionalReturnBool' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturnBool' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'VestingWallet' overrides 'Ownable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'VestingWallet' overrides 'Ownable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'VestingWallet' overrides 'Ownable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'VestingWallet' overrides 'Ownable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'VestingWallet' overrides 'Ownable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'VestingWallet' overrides 'Ownable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'VestingWallet' overrides 'Ownable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'VestingWallet' overrides 'Ownable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'VestingWallet' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'VestingWallet' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@openzeppelin/contracts/finance/VestingWallet.sol`