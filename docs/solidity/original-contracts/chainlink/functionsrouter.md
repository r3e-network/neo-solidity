# FunctionsRouter (Chainlink)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@chainlink/contracts/src/v0.8/functions/v1_0_0/FunctionsRouter.sol`
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
| warning | W200 | function 'isContract' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'safeApprove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ConfirmedOwner' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | VALIDATION_WARNING | abstract contract 'FunctionsSubscriptions' has 5 unimplemented function(s): [_getMaxConsumers, _getSubscriptionDepositDetails, _onlySenderThatAcceptedToS, _onlyRouterOwner, _whenNotPaused] |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'FunctionsSubscriptions' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Context' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W200 | function 'isContract' in 'Pausable' overrides 'Context::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Pausable' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Pausable' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Pausable' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Pausable' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Pausable' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Pausable' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Pausable' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Pausable' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Pausable' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Pausable' overrides 'Context::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Pausable' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Pausable' overrides 'Context::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'Pausable' overrides 'Context::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'Pausable' overrides 'Context::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'Pausable' overrides 'Context::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Pausable' overrides 'Context::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Pausable' overrides 'Context::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'Pausable' overrides 'Context::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'Pausable' overrides 'Context::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'Pausable' overrides 'Context::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'Pausable' overrides 'Context::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'Pausable' overrides 'Context::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Pausable' overrides 'Context::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'Pausable' overrides 'Context::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'Pausable' overrides 'Context::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'Pausable' overrides 'Context::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'Pausable' overrides 'Context::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'Pausable' overrides 'Context::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'Pausable' overrides 'Context::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'Pausable' overrides 'Context::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Pausable' overrides 'Context::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'Pausable' overrides 'Context::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'Pausable' overrides 'Context::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'Pausable' overrides 'Context::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Pausable' overrides 'Context::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'Pausable' overrides 'Context::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'Pausable' overrides 'Context::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'Pausable' overrides 'Context::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Pausable' overrides 'Context::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'Pausable' overrides 'Context::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'Pausable' overrides 'Context::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'Pausable' overrides 'Context::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Pausable' overrides 'Context::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'Pausable' overrides 'Context::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'Pausable' overrides 'Context::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'Pausable' overrides 'Context::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Pausable' overrides 'Context::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'Pausable' overrides 'Context::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Pausable' overrides 'Context::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Pausable' overrides 'Context::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Pausable' overrides 'Context::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'Pausable' overrides 'Context::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'Pausable' overrides 'Context::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'Pausable' overrides 'Context::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'Pausable' overrides 'Context::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'Pausable' overrides 'Context::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'Pausable' overrides 'Context::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'Pausable' overrides 'Context::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'Pausable' overrides 'Context::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'Pausable' overrides 'Context::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'Pausable' overrides 'Context::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'Pausable' overrides 'Context::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'Pausable' overrides 'Context::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'Pausable' overrides 'Context::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'Pausable' overrides 'Context::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'Pausable' overrides 'Context::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Pausable' overrides 'Context::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'Pausable' overrides 'Context::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'Pausable' overrides 'Context::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'Pausable' overrides 'Context::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'Pausable' overrides 'Context::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'Pausable' overrides 'Context::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'Pausable' overrides 'Context::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'Pausable' overrides 'Context::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Pausable' overrides 'Context::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'Pausable' overrides 'Context::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'Pausable' overrides 'Context::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'Pausable' overrides 'Context::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Pausable' overrides 'Context::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'Pausable' overrides 'Context::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Pausable' overrides 'Context::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Pausable' overrides 'Context::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Pausable' overrides 'Context::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Pausable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W200 | function 'isContract' in 'Context' overrides 'FunctionsSubscriptions::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Context' overrides 'FunctionsSubscriptions::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Context' overrides 'FunctionsSubscriptions::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Context' overrides 'FunctionsSubscriptions::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Context' overrides 'FunctionsSubscriptions::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Context' overrides 'FunctionsSubscriptions::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Context' overrides 'FunctionsSubscriptions::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Context' overrides 'FunctionsSubscriptions::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Context' overrides 'FunctionsSubscriptions::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Context' overrides 'FunctionsSubscriptions::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Context' overrides 'FunctionsSubscriptions::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Context' overrides 'FunctionsSubscriptions::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Context' overrides 'FunctionsSubscriptions::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'Context' overrides 'FunctionsSubscriptions::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'Context' overrides 'FunctionsSubscriptions::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'Context' overrides 'FunctionsSubscriptions::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Context' overrides 'FunctionsSubscriptions::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Context' overrides 'FunctionsSubscriptions::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'Context' overrides 'FunctionsSubscriptions::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'Context' overrides 'FunctionsSubscriptions::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'Context' overrides 'FunctionsSubscriptions::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'Context' overrides 'FunctionsSubscriptions::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'Context' overrides 'FunctionsSubscriptions::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Context' overrides 'FunctionsSubscriptions::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'Context' overrides 'FunctionsSubscriptions::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'Context' overrides 'FunctionsSubscriptions::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'Context' overrides 'FunctionsSubscriptions::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'Context' overrides 'FunctionsSubscriptions::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'Context' overrides 'FunctionsSubscriptions::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'Context' overrides 'FunctionsSubscriptions::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'Context' overrides 'FunctionsSubscriptions::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Context' overrides 'FunctionsSubscriptions::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'Context' overrides 'FunctionsSubscriptions::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'Context' overrides 'FunctionsSubscriptions::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'Context' overrides 'FunctionsSubscriptions::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Context' overrides 'FunctionsSubscriptions::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'Context' overrides 'FunctionsSubscriptions::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'Context' overrides 'FunctionsSubscriptions::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'Context' overrides 'FunctionsSubscriptions::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Context' overrides 'FunctionsSubscriptions::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'Context' overrides 'FunctionsSubscriptions::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'Context' overrides 'FunctionsSubscriptions::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'Context' overrides 'FunctionsSubscriptions::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Context' overrides 'FunctionsSubscriptions::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'Context' overrides 'FunctionsSubscriptions::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'Context' overrides 'FunctionsSubscriptions::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'Context' overrides 'FunctionsSubscriptions::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Context' overrides 'FunctionsSubscriptions::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'Context' overrides 'FunctionsSubscriptions::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Context' overrides 'FunctionsSubscriptions::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Context' overrides 'FunctionsSubscriptions::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Context' overrides 'FunctionsSubscriptions::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'Context' overrides 'FunctionsSubscriptions::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'Context' overrides 'FunctionsSubscriptions::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'Context' overrides 'FunctionsSubscriptions::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'Context' overrides 'FunctionsSubscriptions::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'Context' overrides 'FunctionsSubscriptions::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'Context' overrides 'FunctionsSubscriptions::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'Context' overrides 'FunctionsSubscriptions::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'Context' overrides 'FunctionsSubscriptions::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'Context' overrides 'FunctionsSubscriptions::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'Context' overrides 'FunctionsSubscriptions::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'Context' overrides 'FunctionsSubscriptions::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'Context' overrides 'FunctionsSubscriptions::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'Context' overrides 'FunctionsSubscriptions::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'Context' overrides 'FunctionsSubscriptions::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'Context' overrides 'FunctionsSubscriptions::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Context' overrides 'FunctionsSubscriptions::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'Context' overrides 'FunctionsSubscriptions::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'Context' overrides 'FunctionsSubscriptions::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'Context' overrides 'FunctionsSubscriptions::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'Context' overrides 'FunctionsSubscriptions::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'Context' overrides 'FunctionsSubscriptions::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'Context' overrides 'FunctionsSubscriptions::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'Context' overrides 'FunctionsSubscriptions::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Context' overrides 'FunctionsSubscriptions::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'Context' overrides 'FunctionsSubscriptions::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'Context' overrides 'FunctionsSubscriptions::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'Context' overrides 'FunctionsSubscriptions::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Context' overrides 'FunctionsSubscriptions::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'Context' overrides 'FunctionsSubscriptions::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Context' overrides 'FunctionsSubscriptions::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Context' overrides 'FunctionsSubscriptions::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Context' overrides 'FunctionsSubscriptions::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'Pausable' overrides 'Context::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Pausable' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Pausable' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Pausable' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Pausable' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Pausable' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Pausable' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Pausable' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Pausable' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Pausable' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Pausable' overrides 'Context::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Pausable' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Pausable' overrides 'Context::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'Pausable' overrides 'Context::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'Pausable' overrides 'Context::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'Pausable' overrides 'Context::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Pausable' overrides 'Context::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Pausable' overrides 'Context::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'Pausable' overrides 'Context::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'Pausable' overrides 'Context::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'Pausable' overrides 'Context::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'Pausable' overrides 'Context::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'Pausable' overrides 'Context::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Pausable' overrides 'Context::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'Pausable' overrides 'Context::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'Pausable' overrides 'Context::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'Pausable' overrides 'Context::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'Pausable' overrides 'Context::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'Pausable' overrides 'Context::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'Pausable' overrides 'Context::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'Pausable' overrides 'Context::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Pausable' overrides 'Context::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'Pausable' overrides 'Context::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'Pausable' overrides 'Context::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'Pausable' overrides 'Context::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Pausable' overrides 'Context::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'Pausable' overrides 'Context::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'Pausable' overrides 'Context::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'Pausable' overrides 'Context::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Pausable' overrides 'Context::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'Pausable' overrides 'Context::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'Pausable' overrides 'Context::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'Pausable' overrides 'Context::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Pausable' overrides 'Context::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'Pausable' overrides 'Context::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'Pausable' overrides 'Context::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'Pausable' overrides 'Context::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Pausable' overrides 'Context::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'Pausable' overrides 'Context::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Pausable' overrides 'Context::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Pausable' overrides 'Context::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Pausable' overrides 'Context::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'Pausable' overrides 'Context::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'Pausable' overrides 'Context::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'Pausable' overrides 'Context::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'Pausable' overrides 'Context::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'Pausable' overrides 'Context::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'Pausable' overrides 'Context::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'Pausable' overrides 'Context::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'Pausable' overrides 'Context::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'Pausable' overrides 'Context::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'Pausable' overrides 'Context::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'Pausable' overrides 'Context::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'Pausable' overrides 'Context::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'Pausable' overrides 'Context::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'Pausable' overrides 'Context::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'Pausable' overrides 'Context::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Pausable' overrides 'Context::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'Pausable' overrides 'Context::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'Pausable' overrides 'Context::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'Pausable' overrides 'Context::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'Pausable' overrides 'Context::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'Pausable' overrides 'Context::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'Pausable' overrides 'Context::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'Pausable' overrides 'Context::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Pausable' overrides 'Context::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'Pausable' overrides 'Context::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'Pausable' overrides 'Context::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'Pausable' overrides 'Context::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Pausable' overrides 'Context::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'Pausable' overrides 'Context::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Pausable' overrides 'Context::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Pausable' overrides 'Context::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Pausable' overrides 'Context::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Pausable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwnerWithProposal' overrides 'Pausable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'safeApprove' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'FunctionsRouter' overrides 'ConfirmedOwner::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'FunctionsRouter' overrides 'ConfirmedOwner::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'FunctionsRouter' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'FunctionsRouter' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'FunctionsRouter' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'FunctionsRouter' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'FunctionsRouter' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'FunctionsRouter' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'FunctionsRouter' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'FunctionsRouter' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'FunctionsRouter' overrides 'ConfirmedOwner::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'FunctionsRouter' overrides 'ConfirmedOwner::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'FunctionsRouter' overrides 'ConfirmedOwner::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'FunctionsRouter' overrides 'ConfirmedOwner::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'FunctionsRouter' overrides 'ConfirmedOwner::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'FunctionsRouter' overrides 'ConfirmedOwner::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'FunctionsRouter' overrides 'ConfirmedOwner::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'FunctionsRouter' overrides 'ConfirmedOwner::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'FunctionsRouter' overrides 'ConfirmedOwner::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'FunctionsRouter' overrides 'ConfirmedOwner::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'FunctionsRouter' overrides 'ConfirmedOwner::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'FunctionsRouter' overrides 'ConfirmedOwner::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'FunctionsRouter' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'FunctionsRouter' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@chainlink/contracts/src/v0.8/functions/v1_0_0/FunctionsRouter.sol`