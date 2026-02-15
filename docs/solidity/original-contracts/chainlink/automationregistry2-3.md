# AutomationRegistry2_3 (Chainlink)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/automation/v2_3/AutomationRegistry2_3.sol`
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
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.origin has different semantics on Neo N3. Neo uses multi-signature witnesses instead of a single origin. Consider using msg.sender or Runtime.CheckWitness() instead. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.origin has different semantics on Neo N3. Neo uses multi-signature witnesses instead of a single origin. Consider using msg.sender or Runtime.CheckWitness() instead. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.origin has different semantics on Neo N3. Neo uses multi-signature witnesses instead of a single origin. Consider using msg.sender or Runtime.CheckWitness() instead. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.origin has different semantics on Neo N3. Neo uses multi-signature witnesses instead of a single origin. Consider using msg.sender or Runtime.CheckWitness() instead. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.origin has different semantics on Neo N3. Neo uses multi-signature witnesses instead of a single origin. Consider using msg.sender or Runtime.CheckWitness() instead. |
| warning | VALIDATION_WARNING | abstract contract 'OCR2Abstract' has 4 unimplemented function(s): [setConfig, latestConfigDetails, latestConfigDigestAndEpoch, transmit] |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'OCR2Abstract' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Chainable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ConfirmedOwnerWithProposal' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
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
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ConfirmedOwner' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | VALIDATION_WARNING | function '_performUpkeep' should return 2 values but expression does not match tuple |
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
| warning | W200 | function 'toUint248' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'AutomationRegistryBase2_3' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'AutomationForwarder' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | VALIDATION_WARNING | function '_performUpkeep' should return 2 values but expression does not match tuple |
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
| warning | W200 | function 'toUint248' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryLogicC2_3' overrides 'AutomationRegistryBase2_3::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryLogicC2_3' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'AutomationRegistryLogicC2_3' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | VALIDATION_WARNING | function '_performUpkeep' should return 2 values but expression does not match tuple |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'addFunds' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'addFunds' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
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
| warning | W200 | function 'toUint248' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'Chainable' overrides 'AutomationRegistryBase2_3::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Chainable' overrides 'AutomationRegistryBase2_3::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Chainable' overrides 'AutomationRegistryBase2_3::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Chainable' overrides 'AutomationRegistryBase2_3::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Chainable' overrides 'AutomationRegistryBase2_3::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'Chainable' overrides 'AutomationRegistryBase2_3::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'Chainable' overrides 'AutomationRegistryBase2_3::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'Chainable' overrides 'AutomationRegistryBase2_3::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'Chainable' overrides 'AutomationRegistryBase2_3::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'Chainable' overrides 'AutomationRegistryBase2_3::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'Chainable' overrides 'AutomationRegistryBase2_3::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Chainable' overrides 'AutomationRegistryBase2_3::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'Chainable' overrides 'AutomationRegistryBase2_3::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'Chainable' overrides 'AutomationRegistryBase2_3::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'Chainable' overrides 'AutomationRegistryBase2_3::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'Chainable' overrides 'AutomationRegistryBase2_3::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'Chainable' overrides 'AutomationRegistryBase2_3::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Chainable' overrides 'AutomationRegistryBase2_3::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'Chainable' overrides 'AutomationRegistryBase2_3::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'Chainable' overrides 'AutomationRegistryBase2_3::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'Chainable' overrides 'AutomationRegistryBase2_3::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'Chainable' overrides 'AutomationRegistryBase2_3::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'Chainable' overrides 'AutomationRegistryBase2_3::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Chainable' overrides 'AutomationRegistryBase2_3::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'Chainable' overrides 'AutomationRegistryBase2_3::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'Chainable' overrides 'AutomationRegistryBase2_3::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'Chainable' overrides 'AutomationRegistryBase2_3::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'Chainable' overrides 'AutomationRegistryBase2_3::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'Chainable' overrides 'AutomationRegistryBase2_3::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'Chainable' overrides 'AutomationRegistryBase2_3::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'Chainable' overrides 'AutomationRegistryBase2_3::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'Chainable' overrides 'AutomationRegistryBase2_3::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Chainable' overrides 'AutomationRegistryBase2_3::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Chainable' overrides 'AutomationRegistryBase2_3::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'Chainable' overrides 'AutomationRegistryBase2_3::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'Chainable' overrides 'AutomationRegistryBase2_3::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryLogicB2_3' overrides 'Chainable::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryLogicB2_3' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'AutomationRegistryLogicB2_3' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | VALIDATION_WARNING | function '_performUpkeep' should return 2 values but expression does not match tuple |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
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
| warning | W200 | function 'toUint248' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Chainable' overrides 'AutomationRegistryBase2_3::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Chainable' overrides 'AutomationRegistryBase2_3::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'Chainable' overrides 'AutomationRegistryBase2_3::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Chainable' overrides 'AutomationRegistryBase2_3::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Chainable' overrides 'AutomationRegistryBase2_3::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Chainable' overrides 'AutomationRegistryBase2_3::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Chainable' overrides 'AutomationRegistryBase2_3::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Chainable' overrides 'AutomationRegistryBase2_3::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'Chainable' overrides 'AutomationRegistryBase2_3::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'Chainable' overrides 'AutomationRegistryBase2_3::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'Chainable' overrides 'AutomationRegistryBase2_3::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'Chainable' overrides 'AutomationRegistryBase2_3::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'Chainable' overrides 'AutomationRegistryBase2_3::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'Chainable' overrides 'AutomationRegistryBase2_3::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Chainable' overrides 'AutomationRegistryBase2_3::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'Chainable' overrides 'AutomationRegistryBase2_3::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'Chainable' overrides 'AutomationRegistryBase2_3::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'Chainable' overrides 'AutomationRegistryBase2_3::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'Chainable' overrides 'AutomationRegistryBase2_3::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'Chainable' overrides 'AutomationRegistryBase2_3::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Chainable' overrides 'AutomationRegistryBase2_3::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'Chainable' overrides 'AutomationRegistryBase2_3::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'Chainable' overrides 'AutomationRegistryBase2_3::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'Chainable' overrides 'AutomationRegistryBase2_3::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'Chainable' overrides 'AutomationRegistryBase2_3::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'Chainable' overrides 'AutomationRegistryBase2_3::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Chainable' overrides 'AutomationRegistryBase2_3::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'Chainable' overrides 'AutomationRegistryBase2_3::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'Chainable' overrides 'AutomationRegistryBase2_3::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'Chainable' overrides 'AutomationRegistryBase2_3::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'Chainable' overrides 'AutomationRegistryBase2_3::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'Chainable' overrides 'AutomationRegistryBase2_3::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'Chainable' overrides 'AutomationRegistryBase2_3::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'Chainable' overrides 'AutomationRegistryBase2_3::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'Chainable' overrides 'AutomationRegistryBase2_3::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Chainable' overrides 'AutomationRegistryBase2_3::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Chainable' overrides 'AutomationRegistryBase2_3::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'Chainable' overrides 'AutomationRegistryBase2_3::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'Chainable' overrides 'AutomationRegistryBase2_3::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryLogicA2_3' overrides 'Chainable::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryLogicA2_3' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'AutomationRegistryLogicA2_3' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | VALIDATION_WARNING | function '_performUpkeep' should return 2 values but expression does not match tuple |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
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
| warning | W200 | function 'toUint248' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryBase2_3' overrides 'ConfirmedOwner::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistryBase2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'OCR2Abstract' overrides 'AutomationRegistryBase2_3::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'Chainable' overrides 'OCR2Abstract::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'Chainable' overrides 'OCR2Abstract::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'Chainable' overrides 'OCR2Abstract::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Chainable' overrides 'OCR2Abstract::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'Chainable' overrides 'OCR2Abstract::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'Chainable' overrides 'OCR2Abstract::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'Chainable' overrides 'OCR2Abstract::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'Chainable' overrides 'OCR2Abstract::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'Chainable' overrides 'OCR2Abstract::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'Chainable' overrides 'OCR2Abstract::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'Chainable' overrides 'OCR2Abstract::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Chainable' overrides 'OCR2Abstract::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'Chainable' overrides 'OCR2Abstract::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'Chainable' overrides 'OCR2Abstract::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'Chainable' overrides 'OCR2Abstract::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Chainable' overrides 'OCR2Abstract::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'Chainable' overrides 'OCR2Abstract::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'Chainable' overrides 'OCR2Abstract::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'Chainable' overrides 'OCR2Abstract::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Chainable' overrides 'OCR2Abstract::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'Chainable' overrides 'OCR2Abstract::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'Chainable' overrides 'OCR2Abstract::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'Chainable' overrides 'OCR2Abstract::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Chainable' overrides 'OCR2Abstract::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'Chainable' overrides 'OCR2Abstract::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'Chainable' overrides 'OCR2Abstract::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'Chainable' overrides 'OCR2Abstract::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Chainable' overrides 'OCR2Abstract::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'Chainable' overrides 'OCR2Abstract::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Chainable' overrides 'OCR2Abstract::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Chainable' overrides 'OCR2Abstract::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Chainable' overrides 'OCR2Abstract::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'Chainable' overrides 'OCR2Abstract::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'Chainable' overrides 'OCR2Abstract::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'Chainable' overrides 'OCR2Abstract::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'Chainable' overrides 'OCR2Abstract::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'Chainable' overrides 'OCR2Abstract::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'Chainable' overrides 'OCR2Abstract::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'Chainable' overrides 'OCR2Abstract::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'Chainable' overrides 'OCR2Abstract::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'Chainable' overrides 'OCR2Abstract::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'Chainable' overrides 'OCR2Abstract::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'Chainable' overrides 'OCR2Abstract::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'Chainable' overrides 'OCR2Abstract::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'Chainable' overrides 'OCR2Abstract::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'Chainable' overrides 'OCR2Abstract::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'Chainable' overrides 'OCR2Abstract::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Chainable' overrides 'OCR2Abstract::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'Chainable' overrides 'OCR2Abstract::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'Chainable' overrides 'OCR2Abstract::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'Chainable' overrides 'OCR2Abstract::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'Chainable' overrides 'OCR2Abstract::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'Chainable' overrides 'OCR2Abstract::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'Chainable' overrides 'OCR2Abstract::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'Chainable' overrides 'OCR2Abstract::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Chainable' overrides 'OCR2Abstract::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'Chainable' overrides 'OCR2Abstract::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'Chainable' overrides 'OCR2Abstract::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'Chainable' overrides 'OCR2Abstract::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Chainable' overrides 'OCR2Abstract::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'Chainable' overrides 'OCR2Abstract::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Chainable' overrides 'OCR2Abstract::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Chainable' overrides 'OCR2Abstract::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Chainable' overrides 'OCR2Abstract::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'Chainable' overrides 'OCR2Abstract::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Chainable' overrides 'OCR2Abstract::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Chainable' overrides 'OCR2Abstract::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Chainable' overrides 'OCR2Abstract::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Chainable' overrides 'OCR2Abstract::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Chainable' overrides 'OCR2Abstract::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Chainable' overrides 'OCR2Abstract::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Chainable' overrides 'OCR2Abstract::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Chainable' overrides 'OCR2Abstract::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Chainable' overrides 'OCR2Abstract::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Chainable' overrides 'OCR2Abstract::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Chainable' overrides 'OCR2Abstract::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Chainable' overrides 'OCR2Abstract::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'Chainable' overrides 'OCR2Abstract::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'Chainable' overrides 'OCR2Abstract::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'Chainable' overrides 'OCR2Abstract::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'Chainable' overrides 'OCR2Abstract::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'Chainable' overrides 'OCR2Abstract::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'Chainable' overrides 'OCR2Abstract::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Chainable' overrides 'OCR2Abstract::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'Chainable' overrides 'OCR2Abstract::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'Chainable' overrides 'OCR2Abstract::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'Chainable' overrides 'OCR2Abstract::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'Chainable' overrides 'OCR2Abstract::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'Chainable' overrides 'OCR2Abstract::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Chainable' overrides 'OCR2Abstract::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'Chainable' overrides 'OCR2Abstract::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'Chainable' overrides 'OCR2Abstract::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'Chainable' overrides 'OCR2Abstract::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'Chainable' overrides 'OCR2Abstract::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'Chainable' overrides 'OCR2Abstract::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Chainable' overrides 'OCR2Abstract::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'Chainable' overrides 'OCR2Abstract::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'Chainable' overrides 'OCR2Abstract::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'Chainable' overrides 'OCR2Abstract::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'Chainable' overrides 'OCR2Abstract::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'Chainable' overrides 'OCR2Abstract::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'Chainable' overrides 'OCR2Abstract::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'Chainable' overrides 'OCR2Abstract::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'Chainable' overrides 'OCR2Abstract::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Chainable' overrides 'OCR2Abstract::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Chainable' overrides 'OCR2Abstract::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'Chainable' overrides 'OCR2Abstract::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'Chainable' overrides 'OCR2Abstract::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'Chainable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'AutomationRegistry2_3' overrides 'Chainable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'AutomationRegistry2_3' overrides 'Chainable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'AutomationRegistry2_3' overrides 'Chainable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'AutomationRegistry2_3' overrides 'Chainable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'AutomationRegistry2_3' overrides 'Chainable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'AutomationRegistry2_3' overrides 'Chainable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'AutomationRegistry2_3' overrides 'Chainable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'AutomationRegistry2_3' overrides 'Chainable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'AutomationRegistry2_3' overrides 'Chainable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'AutomationRegistry2_3' overrides 'Chainable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'AutomationRegistry2_3' overrides 'Chainable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'AutomationRegistry2_3' overrides 'Chainable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'AutomationRegistry2_3' overrides 'Chainable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'AutomationRegistry2_3' overrides 'Chainable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'AutomationRegistry2_3' overrides 'Chainable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'AutomationRegistry2_3' overrides 'Chainable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'AutomationRegistry2_3' overrides 'Chainable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'AutomationRegistry2_3' overrides 'Chainable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'AutomationRegistry2_3' overrides 'Chainable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'AutomationRegistry2_3' overrides 'Chainable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'AutomationRegistry2_3' overrides 'Chainable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'AutomationRegistry2_3' overrides 'Chainable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'AutomationRegistry2_3' overrides 'Chainable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'AutomationRegistry2_3' overrides 'Chainable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'AutomationRegistry2_3' overrides 'Chainable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'AutomationRegistry2_3' overrides 'Chainable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'AutomationRegistry2_3' overrides 'Chainable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'AutomationRegistry2_3' overrides 'Chainable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'AutomationRegistry2_3' overrides 'Chainable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'AutomationRegistry2_3' overrides 'Chainable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'AutomationRegistry2_3' overrides 'Chainable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'AutomationRegistry2_3' overrides 'Chainable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'AutomationRegistry2_3' overrides 'Chainable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'AutomationRegistry2_3' overrides 'Chainable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'AutomationRegistry2_3' overrides 'Chainable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'AutomationRegistry2_3' overrides 'Chainable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'AutomationRegistry2_3' overrides 'Chainable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'AutomationRegistry2_3' overrides 'Chainable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'AutomationRegistry2_3' overrides 'Chainable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'AutomationRegistry2_3' overrides 'Chainable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'AutomationRegistry2_3' overrides 'Chainable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'AutomationRegistry2_3' overrides 'Chainable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'AutomationRegistry2_3' overrides 'Chainable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'AutomationRegistry2_3' overrides 'Chainable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'AutomationRegistry2_3' overrides 'Chainable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'AutomationRegistry2_3' overrides 'Chainable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'AutomationRegistry2_3' overrides 'Chainable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'AutomationRegistry2_3' overrides 'Chainable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'AutomationRegistry2_3' overrides 'Chainable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'AutomationRegistry2_3' overrides 'Chainable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'AutomationRegistry2_3' overrides 'Chainable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'AutomationRegistry2_3' overrides 'Chainable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'AutomationRegistry2_3' overrides 'Chainable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'AutomationRegistry2_3' overrides 'Chainable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'AutomationRegistry2_3' overrides 'Chainable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'AutomationRegistry2_3' overrides 'Chainable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'AutomationRegistry2_3' overrides 'Chainable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'AutomationRegistry2_3' overrides 'Chainable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'AutomationRegistry2_3' overrides 'Chainable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'AutomationRegistry2_3' overrides 'Chainable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'AutomationRegistry2_3' overrides 'Chainable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'AutomationRegistry2_3' overrides 'Chainable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'AutomationRegistry2_3' overrides 'Chainable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'AutomationRegistry2_3' overrides 'Chainable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'AutomationRegistry2_3' overrides 'Chainable::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'AutomationRegistry2_3' overrides 'Chainable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistry2_3' overrides 'Chainable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AutomationRegistry2_3' overrides 'Chainable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistry2_3' overrides 'Chainable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistry2_3' overrides 'Chainable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistry2_3' overrides 'Chainable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistry2_3' overrides 'Chainable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistry2_3' overrides 'Chainable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistry2_3' overrides 'Chainable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistry2_3' overrides 'Chainable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistry2_3' overrides 'Chainable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'AutomationRegistry2_3' overrides 'Chainable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_add' in 'AutomationRegistry2_3' overrides 'Chainable::_add' which is not marked 'virtual' |
| warning | W200 | function '_add' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_remove' in 'AutomationRegistry2_3' overrides 'Chainable::_remove' which is not marked 'virtual' |
| warning | W200 | function '_remove' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_contains' in 'AutomationRegistry2_3' overrides 'Chainable::_contains' which is not marked 'virtual' |
| warning | W200 | function '_contains' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_length' in 'AutomationRegistry2_3' overrides 'Chainable::_length' which is not marked 'virtual' |
| warning | W200 | function '_length' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_at' in 'AutomationRegistry2_3' overrides 'Chainable::_at' which is not marked 'virtual' |
| warning | W200 | function '_at' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_values' in 'AutomationRegistry2_3' overrides 'Chainable::_values' which is not marked 'virtual' |
| warning | W200 | function '_values' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistry2_3' overrides 'Chainable::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistry2_3' overrides 'Chainable::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistry2_3' overrides 'Chainable::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistry2_3' overrides 'Chainable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistry2_3' overrides 'Chainable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistry2_3' overrides 'Chainable::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistry2_3' overrides 'Chainable::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistry2_3' overrides 'Chainable::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistry2_3' overrides 'Chainable::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistry2_3' overrides 'Chainable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistry2_3' overrides 'Chainable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistry2_3' overrides 'Chainable::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'AutomationRegistry2_3' overrides 'Chainable::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'remove' in 'AutomationRegistry2_3' overrides 'Chainable::remove' which is not marked 'virtual' |
| warning | W200 | function 'remove' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'contains' in 'AutomationRegistry2_3' overrides 'Chainable::contains' which is not marked 'virtual' |
| warning | W200 | function 'contains' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'AutomationRegistry2_3' overrides 'Chainable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'AutomationRegistry2_3' overrides 'Chainable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'values' in 'AutomationRegistry2_3' overrides 'Chainable::values' which is not marked 'virtual' |
| warning | W200 | function 'values' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistry2_3' overrides 'Chainable::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistry2_3' overrides 'Chainable::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistry2_3' overrides 'Chainable::safeApprove' which is not marked 'virtual' |
| warning | W200 | function 'safeApprove' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistry2_3' overrides 'Chainable::safeIncreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeIncreaseAllowance' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistry2_3' overrides 'Chainable::safeDecreaseAllowance' which is not marked 'virtual' |
| warning | W200 | function 'safeDecreaseAllowance' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safePermit' in 'AutomationRegistry2_3' overrides 'Chainable::safePermit' which is not marked 'virtual' |
| warning | W200 | function 'safePermit' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistry2_3' overrides 'Chainable::_callOptionalReturn' which is not marked 'virtual' |
| warning | W200 | function '_callOptionalReturn' in 'AutomationRegistry2_3' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'AutomationRegistry2_3' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/automation/v2_3/AutomationRegistry2_3.sol`