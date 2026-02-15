# Operator (Chainlink)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/operatorforwarder/Operator.sol`
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
| warning | VALIDATION_WARNING | abstract contract 'AuthorizedReceiver' has 1 unimplemented function(s): [_canSetAuthorizedSenders] |
| warning | VALIDATION_WARNING | abstract contract 'LinkTokenReceiver' has 2 unimplemented function(s): [getChainlinkToken, _validateTokenTransferAction] |
| warning | W111 | function 'distributeFunds' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W116 | function 'distributeFunds' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'toUint248' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwnerWithProposal' overrides 'AuthorizedReceiver::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwnerWithProposal' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'toUint248' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'LinkTokenReceiver' overrides 'ConfirmedOwner::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'LinkTokenReceiver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'Operator' overrides 'LinkTokenReceiver::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'Operator' overrides 'LinkTokenReceiver::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'Operator' overrides 'LinkTokenReceiver::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Operator' overrides 'LinkTokenReceiver::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'Operator' overrides 'LinkTokenReceiver::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'Operator' overrides 'LinkTokenReceiver::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'Operator' overrides 'LinkTokenReceiver::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'Operator' overrides 'LinkTokenReceiver::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'Operator' overrides 'LinkTokenReceiver::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'Operator' overrides 'LinkTokenReceiver::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'Operator' overrides 'LinkTokenReceiver::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Operator' overrides 'LinkTokenReceiver::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'Operator' overrides 'LinkTokenReceiver::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'Operator' overrides 'LinkTokenReceiver::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'Operator' overrides 'LinkTokenReceiver::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Operator' overrides 'LinkTokenReceiver::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'Operator' overrides 'LinkTokenReceiver::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'Operator' overrides 'LinkTokenReceiver::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'Operator' overrides 'LinkTokenReceiver::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Operator' overrides 'LinkTokenReceiver::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'Operator' overrides 'LinkTokenReceiver::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'Operator' overrides 'LinkTokenReceiver::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'Operator' overrides 'LinkTokenReceiver::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Operator' overrides 'LinkTokenReceiver::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'Operator' overrides 'LinkTokenReceiver::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'Operator' overrides 'LinkTokenReceiver::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'Operator' overrides 'LinkTokenReceiver::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Operator' overrides 'LinkTokenReceiver::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'Operator' overrides 'LinkTokenReceiver::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Operator' overrides 'LinkTokenReceiver::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Operator' overrides 'LinkTokenReceiver::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Operator' overrides 'LinkTokenReceiver::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'Operator' overrides 'LinkTokenReceiver::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'Operator' overrides 'LinkTokenReceiver::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'Operator' overrides 'LinkTokenReceiver::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'Operator' overrides 'LinkTokenReceiver::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'Operator' overrides 'LinkTokenReceiver::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'Operator' overrides 'LinkTokenReceiver::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'Operator' overrides 'LinkTokenReceiver::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'Operator' overrides 'LinkTokenReceiver::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'Operator' overrides 'LinkTokenReceiver::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'Operator' overrides 'LinkTokenReceiver::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'Operator' overrides 'LinkTokenReceiver::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'Operator' overrides 'LinkTokenReceiver::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'Operator' overrides 'LinkTokenReceiver::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'Operator' overrides 'LinkTokenReceiver::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'Operator' overrides 'LinkTokenReceiver::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Operator' overrides 'LinkTokenReceiver::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'Operator' overrides 'LinkTokenReceiver::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'Operator' overrides 'LinkTokenReceiver::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'Operator' overrides 'LinkTokenReceiver::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'Operator' overrides 'LinkTokenReceiver::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'Operator' overrides 'LinkTokenReceiver::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'Operator' overrides 'LinkTokenReceiver::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'Operator' overrides 'LinkTokenReceiver::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Operator' overrides 'LinkTokenReceiver::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'Operator' overrides 'LinkTokenReceiver::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'Operator' overrides 'LinkTokenReceiver::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'Operator' overrides 'LinkTokenReceiver::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Operator' overrides 'LinkTokenReceiver::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'Operator' overrides 'LinkTokenReceiver::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Operator' overrides 'LinkTokenReceiver::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Operator' overrides 'LinkTokenReceiver::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Operator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Operator' overrides 'LinkTokenReceiver::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Operator' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Operator' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/operatorforwarder/Operator.sol`