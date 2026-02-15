# Account (OpenZeppelin)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@openzeppelin/contracts/account/Account.sol`
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
| warning | RAW | warning: block.basefee auto-mapped to Policy.getFeePerByte() on Neo N3. Neo uses a fixed fee structure, not EIP-1559 base fees. |
| warning | RAW | warning: block.basefee auto-mapped to Policy.getFeePerByte() on Neo N3. Neo uses a fixed fee structure, not EIP-1559 base fees. |
| warning | RAW | warning: block.basefee auto-mapped to Policy.getFeePerByte() on Neo N3. Neo uses a fixed fee structure, not EIP-1559 base fees. |
| warning | VALIDATION_WARNING | abstract contract 'AbstractSigner' has 1 unimplemented function(s): [_rawSignatureValidation] |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'AbstractSigner' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'onNEP17Payment' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'GENERIC' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSERT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDER_OVERFLOW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DIVISION_BY_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ENUM_CONVERSION_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STORAGE_ENCODING_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_ARRAY_POP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARRAY_OUT_OF_BOUNDS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOURCE_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTERNAL_FUNCTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ENTRYPOINT_V07' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ENTRYPOINT_V08' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SIG_VALIDATION_SUCCESS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SIG_VALIDATION_FAILED' detected while merging libraries |
| warning | VALIDATION_WARNING | abstract contract 'Account' has 1 unimplemented function(s): [_rawSignatureValidation] |
| warning | W200 | function 'panic' in 'Account' overrides 'AbstractSigner::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'Account' overrides 'AbstractSigner::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'Account' overrides 'AbstractSigner::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'Account' overrides 'AbstractSigner::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Account' overrides 'AbstractSigner::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'Account' overrides 'AbstractSigner::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'Account' overrides 'AbstractSigner::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'Account' overrides 'AbstractSigner::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'Account' overrides 'AbstractSigner::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'Account' overrides 'AbstractSigner::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'Account' overrides 'AbstractSigner::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'Account' overrides 'AbstractSigner::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Account' overrides 'AbstractSigner::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'Account' overrides 'AbstractSigner::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'Account' overrides 'AbstractSigner::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'Account' overrides 'AbstractSigner::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Account' overrides 'AbstractSigner::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'Account' overrides 'AbstractSigner::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'Account' overrides 'AbstractSigner::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'Account' overrides 'AbstractSigner::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Account' overrides 'AbstractSigner::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'Account' overrides 'AbstractSigner::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'Account' overrides 'AbstractSigner::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'Account' overrides 'AbstractSigner::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Account' overrides 'AbstractSigner::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'Account' overrides 'AbstractSigner::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'Account' overrides 'AbstractSigner::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'Account' overrides 'AbstractSigner::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Account' overrides 'AbstractSigner::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'Account' overrides 'AbstractSigner::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Account' overrides 'AbstractSigner::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Account' overrides 'AbstractSigner::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Account' overrides 'AbstractSigner::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'Account' overrides 'AbstractSigner::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'Account' overrides 'AbstractSigner::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'Account' overrides 'AbstractSigner::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'Account' overrides 'AbstractSigner::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'Account' overrides 'AbstractSigner::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'Account' overrides 'AbstractSigner::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'Account' overrides 'AbstractSigner::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'Account' overrides 'AbstractSigner::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'Account' overrides 'AbstractSigner::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'Account' overrides 'AbstractSigner::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'Account' overrides 'AbstractSigner::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'Account' overrides 'AbstractSigner::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'Account' overrides 'AbstractSigner::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'Account' overrides 'AbstractSigner::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'Account' overrides 'AbstractSigner::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Account' overrides 'AbstractSigner::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'Account' overrides 'AbstractSigner::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'Account' overrides 'AbstractSigner::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'Account' overrides 'AbstractSigner::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'Account' overrides 'AbstractSigner::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'Account' overrides 'AbstractSigner::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'Account' overrides 'AbstractSigner::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'Account' overrides 'AbstractSigner::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Account' overrides 'AbstractSigner::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'Account' overrides 'AbstractSigner::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'Account' overrides 'AbstractSigner::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'Account' overrides 'AbstractSigner::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Account' overrides 'AbstractSigner::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'Account' overrides 'AbstractSigner::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Account' overrides 'AbstractSigner::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Account' overrides 'AbstractSigner::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Account' overrides 'AbstractSigner::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'Account' overrides 'AbstractSigner::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'Account' overrides 'AbstractSigner::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'Account' overrides 'AbstractSigner::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'Account' overrides 'AbstractSigner::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'Account' overrides 'AbstractSigner::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'Account' overrides 'AbstractSigner::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'Account' overrides 'AbstractSigner::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'Account' overrides 'AbstractSigner::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'Account' overrides 'AbstractSigner::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'Account' overrides 'AbstractSigner::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'Account' overrides 'AbstractSigner::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'Account' overrides 'AbstractSigner::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'Account' overrides 'AbstractSigner::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'Account' overrides 'AbstractSigner::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'Account' overrides 'AbstractSigner::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'Account' overrides 'AbstractSigner::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Account' overrides 'AbstractSigner::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Account' overrides 'AbstractSigner::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'Account' overrides 'AbstractSigner::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'Account' overrides 'AbstractSigner::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'Account' overrides 'AbstractSigner::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'Account' overrides 'AbstractSigner::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'Account' overrides 'AbstractSigner::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'Account' overrides 'AbstractSigner::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'Account' overrides 'AbstractSigner::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'Account' overrides 'AbstractSigner::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'Account' overrides 'AbstractSigner::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'Account' overrides 'AbstractSigner::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'Account' overrides 'AbstractSigner::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'Account' overrides 'AbstractSigner::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'Account' overrides 'AbstractSigner::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'Account' overrides 'AbstractSigner::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'Account' overrides 'AbstractSigner::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'Account' overrides 'AbstractSigner::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'Account' overrides 'AbstractSigner::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'Account' overrides 'AbstractSigner::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'emptyBytes' in 'Account' overrides 'AbstractSigner::emptyBytes' which is not marked 'virtual' |
| warning | W200 | function 'emptyBytes' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'emptyString' in 'Account' overrides 'AbstractSigner::emptyString' which is not marked 'virtual' |
| warning | W200 | function 'emptyString' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_1_1' in 'Account' overrides 'AbstractSigner::pack_1_1' which is not marked 'virtual' |
| warning | W200 | function 'pack_1_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_2_2' in 'Account' overrides 'AbstractSigner::pack_2_2' which is not marked 'virtual' |
| warning | W200 | function 'pack_2_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_2_4' in 'Account' overrides 'AbstractSigner::pack_2_4' which is not marked 'virtual' |
| warning | W200 | function 'pack_2_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_2_6' in 'Account' overrides 'AbstractSigner::pack_2_6' which is not marked 'virtual' |
| warning | W200 | function 'pack_2_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_2_8' in 'Account' overrides 'AbstractSigner::pack_2_8' which is not marked 'virtual' |
| warning | W200 | function 'pack_2_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_2_10' in 'Account' overrides 'AbstractSigner::pack_2_10' which is not marked 'virtual' |
| warning | W200 | function 'pack_2_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_2_20' in 'Account' overrides 'AbstractSigner::pack_2_20' which is not marked 'virtual' |
| warning | W200 | function 'pack_2_20' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_2_22' in 'Account' overrides 'AbstractSigner::pack_2_22' which is not marked 'virtual' |
| warning | W200 | function 'pack_2_22' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_4_2' in 'Account' overrides 'AbstractSigner::pack_4_2' which is not marked 'virtual' |
| warning | W200 | function 'pack_4_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_4_4' in 'Account' overrides 'AbstractSigner::pack_4_4' which is not marked 'virtual' |
| warning | W200 | function 'pack_4_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_4_6' in 'Account' overrides 'AbstractSigner::pack_4_6' which is not marked 'virtual' |
| warning | W200 | function 'pack_4_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_4_8' in 'Account' overrides 'AbstractSigner::pack_4_8' which is not marked 'virtual' |
| warning | W200 | function 'pack_4_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_4_12' in 'Account' overrides 'AbstractSigner::pack_4_12' which is not marked 'virtual' |
| warning | W200 | function 'pack_4_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_4_16' in 'Account' overrides 'AbstractSigner::pack_4_16' which is not marked 'virtual' |
| warning | W200 | function 'pack_4_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_4_20' in 'Account' overrides 'AbstractSigner::pack_4_20' which is not marked 'virtual' |
| warning | W200 | function 'pack_4_20' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_4_24' in 'Account' overrides 'AbstractSigner::pack_4_24' which is not marked 'virtual' |
| warning | W200 | function 'pack_4_24' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_4_28' in 'Account' overrides 'AbstractSigner::pack_4_28' which is not marked 'virtual' |
| warning | W200 | function 'pack_4_28' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_6_2' in 'Account' overrides 'AbstractSigner::pack_6_2' which is not marked 'virtual' |
| warning | W200 | function 'pack_6_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_6_4' in 'Account' overrides 'AbstractSigner::pack_6_4' which is not marked 'virtual' |
| warning | W200 | function 'pack_6_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_6_6' in 'Account' overrides 'AbstractSigner::pack_6_6' which is not marked 'virtual' |
| warning | W200 | function 'pack_6_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_6_10' in 'Account' overrides 'AbstractSigner::pack_6_10' which is not marked 'virtual' |
| warning | W200 | function 'pack_6_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_6_16' in 'Account' overrides 'AbstractSigner::pack_6_16' which is not marked 'virtual' |
| warning | W200 | function 'pack_6_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_6_22' in 'Account' overrides 'AbstractSigner::pack_6_22' which is not marked 'virtual' |
| warning | W200 | function 'pack_6_22' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_8_2' in 'Account' overrides 'AbstractSigner::pack_8_2' which is not marked 'virtual' |
| warning | W200 | function 'pack_8_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_8_4' in 'Account' overrides 'AbstractSigner::pack_8_4' which is not marked 'virtual' |
| warning | W200 | function 'pack_8_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_8_8' in 'Account' overrides 'AbstractSigner::pack_8_8' which is not marked 'virtual' |
| warning | W200 | function 'pack_8_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_8_12' in 'Account' overrides 'AbstractSigner::pack_8_12' which is not marked 'virtual' |
| warning | W200 | function 'pack_8_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_8_16' in 'Account' overrides 'AbstractSigner::pack_8_16' which is not marked 'virtual' |
| warning | W200 | function 'pack_8_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_8_20' in 'Account' overrides 'AbstractSigner::pack_8_20' which is not marked 'virtual' |
| warning | W200 | function 'pack_8_20' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_8_24' in 'Account' overrides 'AbstractSigner::pack_8_24' which is not marked 'virtual' |
| warning | W200 | function 'pack_8_24' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_10_2' in 'Account' overrides 'AbstractSigner::pack_10_2' which is not marked 'virtual' |
| warning | W200 | function 'pack_10_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_10_6' in 'Account' overrides 'AbstractSigner::pack_10_6' which is not marked 'virtual' |
| warning | W200 | function 'pack_10_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_10_10' in 'Account' overrides 'AbstractSigner::pack_10_10' which is not marked 'virtual' |
| warning | W200 | function 'pack_10_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_10_12' in 'Account' overrides 'AbstractSigner::pack_10_12' which is not marked 'virtual' |
| warning | W200 | function 'pack_10_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_10_22' in 'Account' overrides 'AbstractSigner::pack_10_22' which is not marked 'virtual' |
| warning | W200 | function 'pack_10_22' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_12_4' in 'Account' overrides 'AbstractSigner::pack_12_4' which is not marked 'virtual' |
| warning | W200 | function 'pack_12_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_12_8' in 'Account' overrides 'AbstractSigner::pack_12_8' which is not marked 'virtual' |
| warning | W200 | function 'pack_12_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_12_10' in 'Account' overrides 'AbstractSigner::pack_12_10' which is not marked 'virtual' |
| warning | W200 | function 'pack_12_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_12_12' in 'Account' overrides 'AbstractSigner::pack_12_12' which is not marked 'virtual' |
| warning | W200 | function 'pack_12_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_12_16' in 'Account' overrides 'AbstractSigner::pack_12_16' which is not marked 'virtual' |
| warning | W200 | function 'pack_12_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_12_20' in 'Account' overrides 'AbstractSigner::pack_12_20' which is not marked 'virtual' |
| warning | W200 | function 'pack_12_20' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_16_4' in 'Account' overrides 'AbstractSigner::pack_16_4' which is not marked 'virtual' |
| warning | W200 | function 'pack_16_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_16_6' in 'Account' overrides 'AbstractSigner::pack_16_6' which is not marked 'virtual' |
| warning | W200 | function 'pack_16_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_16_8' in 'Account' overrides 'AbstractSigner::pack_16_8' which is not marked 'virtual' |
| warning | W200 | function 'pack_16_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_16_12' in 'Account' overrides 'AbstractSigner::pack_16_12' which is not marked 'virtual' |
| warning | W200 | function 'pack_16_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_16_16' in 'Account' overrides 'AbstractSigner::pack_16_16' which is not marked 'virtual' |
| warning | W200 | function 'pack_16_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_20_2' in 'Account' overrides 'AbstractSigner::pack_20_2' which is not marked 'virtual' |
| warning | W200 | function 'pack_20_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_20_4' in 'Account' overrides 'AbstractSigner::pack_20_4' which is not marked 'virtual' |
| warning | W200 | function 'pack_20_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_20_8' in 'Account' overrides 'AbstractSigner::pack_20_8' which is not marked 'virtual' |
| warning | W200 | function 'pack_20_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_20_12' in 'Account' overrides 'AbstractSigner::pack_20_12' which is not marked 'virtual' |
| warning | W200 | function 'pack_20_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_22_2' in 'Account' overrides 'AbstractSigner::pack_22_2' which is not marked 'virtual' |
| warning | W200 | function 'pack_22_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_22_6' in 'Account' overrides 'AbstractSigner::pack_22_6' which is not marked 'virtual' |
| warning | W200 | function 'pack_22_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_22_10' in 'Account' overrides 'AbstractSigner::pack_22_10' which is not marked 'virtual' |
| warning | W200 | function 'pack_22_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_24_4' in 'Account' overrides 'AbstractSigner::pack_24_4' which is not marked 'virtual' |
| warning | W200 | function 'pack_24_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_24_8' in 'Account' overrides 'AbstractSigner::pack_24_8' which is not marked 'virtual' |
| warning | W200 | function 'pack_24_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack_28_4' in 'Account' overrides 'AbstractSigner::pack_28_4' which is not marked 'virtual' |
| warning | W200 | function 'pack_28_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_2_1' in 'Account' overrides 'AbstractSigner::extract_2_1' which is not marked 'virtual' |
| warning | W200 | function 'extract_2_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_2_1' in 'Account' overrides 'AbstractSigner::replace_2_1' which is not marked 'virtual' |
| warning | W200 | function 'replace_2_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_4_1' in 'Account' overrides 'AbstractSigner::extract_4_1' which is not marked 'virtual' |
| warning | W200 | function 'extract_4_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_4_1' in 'Account' overrides 'AbstractSigner::replace_4_1' which is not marked 'virtual' |
| warning | W200 | function 'replace_4_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_4_2' in 'Account' overrides 'AbstractSigner::extract_4_2' which is not marked 'virtual' |
| warning | W200 | function 'extract_4_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_4_2' in 'Account' overrides 'AbstractSigner::replace_4_2' which is not marked 'virtual' |
| warning | W200 | function 'replace_4_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_6_1' in 'Account' overrides 'AbstractSigner::extract_6_1' which is not marked 'virtual' |
| warning | W200 | function 'extract_6_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_6_1' in 'Account' overrides 'AbstractSigner::replace_6_1' which is not marked 'virtual' |
| warning | W200 | function 'replace_6_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_6_2' in 'Account' overrides 'AbstractSigner::extract_6_2' which is not marked 'virtual' |
| warning | W200 | function 'extract_6_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_6_2' in 'Account' overrides 'AbstractSigner::replace_6_2' which is not marked 'virtual' |
| warning | W200 | function 'replace_6_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_6_4' in 'Account' overrides 'AbstractSigner::extract_6_4' which is not marked 'virtual' |
| warning | W200 | function 'extract_6_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_6_4' in 'Account' overrides 'AbstractSigner::replace_6_4' which is not marked 'virtual' |
| warning | W200 | function 'replace_6_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_8_1' in 'Account' overrides 'AbstractSigner::extract_8_1' which is not marked 'virtual' |
| warning | W200 | function 'extract_8_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_8_1' in 'Account' overrides 'AbstractSigner::replace_8_1' which is not marked 'virtual' |
| warning | W200 | function 'replace_8_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_8_2' in 'Account' overrides 'AbstractSigner::extract_8_2' which is not marked 'virtual' |
| warning | W200 | function 'extract_8_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_8_2' in 'Account' overrides 'AbstractSigner::replace_8_2' which is not marked 'virtual' |
| warning | W200 | function 'replace_8_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_8_4' in 'Account' overrides 'AbstractSigner::extract_8_4' which is not marked 'virtual' |
| warning | W200 | function 'extract_8_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_8_4' in 'Account' overrides 'AbstractSigner::replace_8_4' which is not marked 'virtual' |
| warning | W200 | function 'replace_8_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_8_6' in 'Account' overrides 'AbstractSigner::extract_8_6' which is not marked 'virtual' |
| warning | W200 | function 'extract_8_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_8_6' in 'Account' overrides 'AbstractSigner::replace_8_6' which is not marked 'virtual' |
| warning | W200 | function 'replace_8_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_10_1' in 'Account' overrides 'AbstractSigner::extract_10_1' which is not marked 'virtual' |
| warning | W200 | function 'extract_10_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_10_1' in 'Account' overrides 'AbstractSigner::replace_10_1' which is not marked 'virtual' |
| warning | W200 | function 'replace_10_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_10_2' in 'Account' overrides 'AbstractSigner::extract_10_2' which is not marked 'virtual' |
| warning | W200 | function 'extract_10_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_10_2' in 'Account' overrides 'AbstractSigner::replace_10_2' which is not marked 'virtual' |
| warning | W200 | function 'replace_10_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_10_4' in 'Account' overrides 'AbstractSigner::extract_10_4' which is not marked 'virtual' |
| warning | W200 | function 'extract_10_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_10_4' in 'Account' overrides 'AbstractSigner::replace_10_4' which is not marked 'virtual' |
| warning | W200 | function 'replace_10_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_10_6' in 'Account' overrides 'AbstractSigner::extract_10_6' which is not marked 'virtual' |
| warning | W200 | function 'extract_10_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_10_6' in 'Account' overrides 'AbstractSigner::replace_10_6' which is not marked 'virtual' |
| warning | W200 | function 'replace_10_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_10_8' in 'Account' overrides 'AbstractSigner::extract_10_8' which is not marked 'virtual' |
| warning | W200 | function 'extract_10_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_10_8' in 'Account' overrides 'AbstractSigner::replace_10_8' which is not marked 'virtual' |
| warning | W200 | function 'replace_10_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_12_1' in 'Account' overrides 'AbstractSigner::extract_12_1' which is not marked 'virtual' |
| warning | W200 | function 'extract_12_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_12_1' in 'Account' overrides 'AbstractSigner::replace_12_1' which is not marked 'virtual' |
| warning | W200 | function 'replace_12_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_12_2' in 'Account' overrides 'AbstractSigner::extract_12_2' which is not marked 'virtual' |
| warning | W200 | function 'extract_12_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_12_2' in 'Account' overrides 'AbstractSigner::replace_12_2' which is not marked 'virtual' |
| warning | W200 | function 'replace_12_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_12_4' in 'Account' overrides 'AbstractSigner::extract_12_4' which is not marked 'virtual' |
| warning | W200 | function 'extract_12_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_12_4' in 'Account' overrides 'AbstractSigner::replace_12_4' which is not marked 'virtual' |
| warning | W200 | function 'replace_12_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_12_6' in 'Account' overrides 'AbstractSigner::extract_12_6' which is not marked 'virtual' |
| warning | W200 | function 'extract_12_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_12_6' in 'Account' overrides 'AbstractSigner::replace_12_6' which is not marked 'virtual' |
| warning | W200 | function 'replace_12_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_12_8' in 'Account' overrides 'AbstractSigner::extract_12_8' which is not marked 'virtual' |
| warning | W200 | function 'extract_12_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_12_8' in 'Account' overrides 'AbstractSigner::replace_12_8' which is not marked 'virtual' |
| warning | W200 | function 'replace_12_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_12_10' in 'Account' overrides 'AbstractSigner::extract_12_10' which is not marked 'virtual' |
| warning | W200 | function 'extract_12_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_12_10' in 'Account' overrides 'AbstractSigner::replace_12_10' which is not marked 'virtual' |
| warning | W200 | function 'replace_12_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_16_1' in 'Account' overrides 'AbstractSigner::extract_16_1' which is not marked 'virtual' |
| warning | W200 | function 'extract_16_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_16_1' in 'Account' overrides 'AbstractSigner::replace_16_1' which is not marked 'virtual' |
| warning | W200 | function 'replace_16_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_16_2' in 'Account' overrides 'AbstractSigner::extract_16_2' which is not marked 'virtual' |
| warning | W200 | function 'extract_16_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_16_2' in 'Account' overrides 'AbstractSigner::replace_16_2' which is not marked 'virtual' |
| warning | W200 | function 'replace_16_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_16_4' in 'Account' overrides 'AbstractSigner::extract_16_4' which is not marked 'virtual' |
| warning | W200 | function 'extract_16_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_16_4' in 'Account' overrides 'AbstractSigner::replace_16_4' which is not marked 'virtual' |
| warning | W200 | function 'replace_16_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_16_6' in 'Account' overrides 'AbstractSigner::extract_16_6' which is not marked 'virtual' |
| warning | W200 | function 'extract_16_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_16_6' in 'Account' overrides 'AbstractSigner::replace_16_6' which is not marked 'virtual' |
| warning | W200 | function 'replace_16_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_16_8' in 'Account' overrides 'AbstractSigner::extract_16_8' which is not marked 'virtual' |
| warning | W200 | function 'extract_16_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_16_8' in 'Account' overrides 'AbstractSigner::replace_16_8' which is not marked 'virtual' |
| warning | W200 | function 'replace_16_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_16_10' in 'Account' overrides 'AbstractSigner::extract_16_10' which is not marked 'virtual' |
| warning | W200 | function 'extract_16_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_16_10' in 'Account' overrides 'AbstractSigner::replace_16_10' which is not marked 'virtual' |
| warning | W200 | function 'replace_16_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_16_12' in 'Account' overrides 'AbstractSigner::extract_16_12' which is not marked 'virtual' |
| warning | W200 | function 'extract_16_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_16_12' in 'Account' overrides 'AbstractSigner::replace_16_12' which is not marked 'virtual' |
| warning | W200 | function 'replace_16_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_20_1' in 'Account' overrides 'AbstractSigner::extract_20_1' which is not marked 'virtual' |
| warning | W200 | function 'extract_20_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_20_1' in 'Account' overrides 'AbstractSigner::replace_20_1' which is not marked 'virtual' |
| warning | W200 | function 'replace_20_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_20_2' in 'Account' overrides 'AbstractSigner::extract_20_2' which is not marked 'virtual' |
| warning | W200 | function 'extract_20_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_20_2' in 'Account' overrides 'AbstractSigner::replace_20_2' which is not marked 'virtual' |
| warning | W200 | function 'replace_20_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_20_4' in 'Account' overrides 'AbstractSigner::extract_20_4' which is not marked 'virtual' |
| warning | W200 | function 'extract_20_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_20_4' in 'Account' overrides 'AbstractSigner::replace_20_4' which is not marked 'virtual' |
| warning | W200 | function 'replace_20_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_20_6' in 'Account' overrides 'AbstractSigner::extract_20_6' which is not marked 'virtual' |
| warning | W200 | function 'extract_20_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_20_6' in 'Account' overrides 'AbstractSigner::replace_20_6' which is not marked 'virtual' |
| warning | W200 | function 'replace_20_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_20_8' in 'Account' overrides 'AbstractSigner::extract_20_8' which is not marked 'virtual' |
| warning | W200 | function 'extract_20_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_20_8' in 'Account' overrides 'AbstractSigner::replace_20_8' which is not marked 'virtual' |
| warning | W200 | function 'replace_20_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_20_10' in 'Account' overrides 'AbstractSigner::extract_20_10' which is not marked 'virtual' |
| warning | W200 | function 'extract_20_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_20_10' in 'Account' overrides 'AbstractSigner::replace_20_10' which is not marked 'virtual' |
| warning | W200 | function 'replace_20_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_20_12' in 'Account' overrides 'AbstractSigner::extract_20_12' which is not marked 'virtual' |
| warning | W200 | function 'extract_20_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_20_12' in 'Account' overrides 'AbstractSigner::replace_20_12' which is not marked 'virtual' |
| warning | W200 | function 'replace_20_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_20_16' in 'Account' overrides 'AbstractSigner::extract_20_16' which is not marked 'virtual' |
| warning | W200 | function 'extract_20_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_20_16' in 'Account' overrides 'AbstractSigner::replace_20_16' which is not marked 'virtual' |
| warning | W200 | function 'replace_20_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_22_1' in 'Account' overrides 'AbstractSigner::extract_22_1' which is not marked 'virtual' |
| warning | W200 | function 'extract_22_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_22_1' in 'Account' overrides 'AbstractSigner::replace_22_1' which is not marked 'virtual' |
| warning | W200 | function 'replace_22_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_22_2' in 'Account' overrides 'AbstractSigner::extract_22_2' which is not marked 'virtual' |
| warning | W200 | function 'extract_22_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_22_2' in 'Account' overrides 'AbstractSigner::replace_22_2' which is not marked 'virtual' |
| warning | W200 | function 'replace_22_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_22_4' in 'Account' overrides 'AbstractSigner::extract_22_4' which is not marked 'virtual' |
| warning | W200 | function 'extract_22_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_22_4' in 'Account' overrides 'AbstractSigner::replace_22_4' which is not marked 'virtual' |
| warning | W200 | function 'replace_22_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_22_6' in 'Account' overrides 'AbstractSigner::extract_22_6' which is not marked 'virtual' |
| warning | W200 | function 'extract_22_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_22_6' in 'Account' overrides 'AbstractSigner::replace_22_6' which is not marked 'virtual' |
| warning | W200 | function 'replace_22_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_22_8' in 'Account' overrides 'AbstractSigner::extract_22_8' which is not marked 'virtual' |
| warning | W200 | function 'extract_22_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_22_8' in 'Account' overrides 'AbstractSigner::replace_22_8' which is not marked 'virtual' |
| warning | W200 | function 'replace_22_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_22_10' in 'Account' overrides 'AbstractSigner::extract_22_10' which is not marked 'virtual' |
| warning | W200 | function 'extract_22_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_22_10' in 'Account' overrides 'AbstractSigner::replace_22_10' which is not marked 'virtual' |
| warning | W200 | function 'replace_22_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_22_12' in 'Account' overrides 'AbstractSigner::extract_22_12' which is not marked 'virtual' |
| warning | W200 | function 'extract_22_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_22_12' in 'Account' overrides 'AbstractSigner::replace_22_12' which is not marked 'virtual' |
| warning | W200 | function 'replace_22_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_22_16' in 'Account' overrides 'AbstractSigner::extract_22_16' which is not marked 'virtual' |
| warning | W200 | function 'extract_22_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_22_16' in 'Account' overrides 'AbstractSigner::replace_22_16' which is not marked 'virtual' |
| warning | W200 | function 'replace_22_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_22_20' in 'Account' overrides 'AbstractSigner::extract_22_20' which is not marked 'virtual' |
| warning | W200 | function 'extract_22_20' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_22_20' in 'Account' overrides 'AbstractSigner::replace_22_20' which is not marked 'virtual' |
| warning | W200 | function 'replace_22_20' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_24_1' in 'Account' overrides 'AbstractSigner::extract_24_1' which is not marked 'virtual' |
| warning | W200 | function 'extract_24_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_24_1' in 'Account' overrides 'AbstractSigner::replace_24_1' which is not marked 'virtual' |
| warning | W200 | function 'replace_24_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_24_2' in 'Account' overrides 'AbstractSigner::extract_24_2' which is not marked 'virtual' |
| warning | W200 | function 'extract_24_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_24_2' in 'Account' overrides 'AbstractSigner::replace_24_2' which is not marked 'virtual' |
| warning | W200 | function 'replace_24_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_24_4' in 'Account' overrides 'AbstractSigner::extract_24_4' which is not marked 'virtual' |
| warning | W200 | function 'extract_24_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_24_4' in 'Account' overrides 'AbstractSigner::replace_24_4' which is not marked 'virtual' |
| warning | W200 | function 'replace_24_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_24_6' in 'Account' overrides 'AbstractSigner::extract_24_6' which is not marked 'virtual' |
| warning | W200 | function 'extract_24_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_24_6' in 'Account' overrides 'AbstractSigner::replace_24_6' which is not marked 'virtual' |
| warning | W200 | function 'replace_24_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_24_8' in 'Account' overrides 'AbstractSigner::extract_24_8' which is not marked 'virtual' |
| warning | W200 | function 'extract_24_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_24_8' in 'Account' overrides 'AbstractSigner::replace_24_8' which is not marked 'virtual' |
| warning | W200 | function 'replace_24_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_24_10' in 'Account' overrides 'AbstractSigner::extract_24_10' which is not marked 'virtual' |
| warning | W200 | function 'extract_24_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_24_10' in 'Account' overrides 'AbstractSigner::replace_24_10' which is not marked 'virtual' |
| warning | W200 | function 'replace_24_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_24_12' in 'Account' overrides 'AbstractSigner::extract_24_12' which is not marked 'virtual' |
| warning | W200 | function 'extract_24_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_24_12' in 'Account' overrides 'AbstractSigner::replace_24_12' which is not marked 'virtual' |
| warning | W200 | function 'replace_24_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_24_16' in 'Account' overrides 'AbstractSigner::extract_24_16' which is not marked 'virtual' |
| warning | W200 | function 'extract_24_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_24_16' in 'Account' overrides 'AbstractSigner::replace_24_16' which is not marked 'virtual' |
| warning | W200 | function 'replace_24_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_24_20' in 'Account' overrides 'AbstractSigner::extract_24_20' which is not marked 'virtual' |
| warning | W200 | function 'extract_24_20' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_24_20' in 'Account' overrides 'AbstractSigner::replace_24_20' which is not marked 'virtual' |
| warning | W200 | function 'replace_24_20' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_24_22' in 'Account' overrides 'AbstractSigner::extract_24_22' which is not marked 'virtual' |
| warning | W200 | function 'extract_24_22' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_24_22' in 'Account' overrides 'AbstractSigner::replace_24_22' which is not marked 'virtual' |
| warning | W200 | function 'replace_24_22' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_28_1' in 'Account' overrides 'AbstractSigner::extract_28_1' which is not marked 'virtual' |
| warning | W200 | function 'extract_28_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_28_1' in 'Account' overrides 'AbstractSigner::replace_28_1' which is not marked 'virtual' |
| warning | W200 | function 'replace_28_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_28_2' in 'Account' overrides 'AbstractSigner::extract_28_2' which is not marked 'virtual' |
| warning | W200 | function 'extract_28_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_28_2' in 'Account' overrides 'AbstractSigner::replace_28_2' which is not marked 'virtual' |
| warning | W200 | function 'replace_28_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_28_4' in 'Account' overrides 'AbstractSigner::extract_28_4' which is not marked 'virtual' |
| warning | W200 | function 'extract_28_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_28_4' in 'Account' overrides 'AbstractSigner::replace_28_4' which is not marked 'virtual' |
| warning | W200 | function 'replace_28_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_28_6' in 'Account' overrides 'AbstractSigner::extract_28_6' which is not marked 'virtual' |
| warning | W200 | function 'extract_28_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_28_6' in 'Account' overrides 'AbstractSigner::replace_28_6' which is not marked 'virtual' |
| warning | W200 | function 'replace_28_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_28_8' in 'Account' overrides 'AbstractSigner::extract_28_8' which is not marked 'virtual' |
| warning | W200 | function 'extract_28_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_28_8' in 'Account' overrides 'AbstractSigner::replace_28_8' which is not marked 'virtual' |
| warning | W200 | function 'replace_28_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_28_10' in 'Account' overrides 'AbstractSigner::extract_28_10' which is not marked 'virtual' |
| warning | W200 | function 'extract_28_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_28_10' in 'Account' overrides 'AbstractSigner::replace_28_10' which is not marked 'virtual' |
| warning | W200 | function 'replace_28_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_28_12' in 'Account' overrides 'AbstractSigner::extract_28_12' which is not marked 'virtual' |
| warning | W200 | function 'extract_28_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_28_12' in 'Account' overrides 'AbstractSigner::replace_28_12' which is not marked 'virtual' |
| warning | W200 | function 'replace_28_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_28_16' in 'Account' overrides 'AbstractSigner::extract_28_16' which is not marked 'virtual' |
| warning | W200 | function 'extract_28_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_28_16' in 'Account' overrides 'AbstractSigner::replace_28_16' which is not marked 'virtual' |
| warning | W200 | function 'replace_28_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_28_20' in 'Account' overrides 'AbstractSigner::extract_28_20' which is not marked 'virtual' |
| warning | W200 | function 'extract_28_20' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_28_20' in 'Account' overrides 'AbstractSigner::replace_28_20' which is not marked 'virtual' |
| warning | W200 | function 'replace_28_20' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_28_22' in 'Account' overrides 'AbstractSigner::extract_28_22' which is not marked 'virtual' |
| warning | W200 | function 'extract_28_22' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_28_22' in 'Account' overrides 'AbstractSigner::replace_28_22' which is not marked 'virtual' |
| warning | W200 | function 'replace_28_22' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_28_24' in 'Account' overrides 'AbstractSigner::extract_28_24' which is not marked 'virtual' |
| warning | W200 | function 'extract_28_24' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_28_24' in 'Account' overrides 'AbstractSigner::replace_28_24' which is not marked 'virtual' |
| warning | W200 | function 'replace_28_24' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_32_1' in 'Account' overrides 'AbstractSigner::extract_32_1' which is not marked 'virtual' |
| warning | W200 | function 'extract_32_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_32_1' in 'Account' overrides 'AbstractSigner::replace_32_1' which is not marked 'virtual' |
| warning | W200 | function 'replace_32_1' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_32_2' in 'Account' overrides 'AbstractSigner::extract_32_2' which is not marked 'virtual' |
| warning | W200 | function 'extract_32_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_32_2' in 'Account' overrides 'AbstractSigner::replace_32_2' which is not marked 'virtual' |
| warning | W200 | function 'replace_32_2' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_32_4' in 'Account' overrides 'AbstractSigner::extract_32_4' which is not marked 'virtual' |
| warning | W200 | function 'extract_32_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_32_4' in 'Account' overrides 'AbstractSigner::replace_32_4' which is not marked 'virtual' |
| warning | W200 | function 'replace_32_4' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_32_6' in 'Account' overrides 'AbstractSigner::extract_32_6' which is not marked 'virtual' |
| warning | W200 | function 'extract_32_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_32_6' in 'Account' overrides 'AbstractSigner::replace_32_6' which is not marked 'virtual' |
| warning | W200 | function 'replace_32_6' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_32_8' in 'Account' overrides 'AbstractSigner::extract_32_8' which is not marked 'virtual' |
| warning | W200 | function 'extract_32_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_32_8' in 'Account' overrides 'AbstractSigner::replace_32_8' which is not marked 'virtual' |
| warning | W200 | function 'replace_32_8' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_32_10' in 'Account' overrides 'AbstractSigner::extract_32_10' which is not marked 'virtual' |
| warning | W200 | function 'extract_32_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_32_10' in 'Account' overrides 'AbstractSigner::replace_32_10' which is not marked 'virtual' |
| warning | W200 | function 'replace_32_10' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_32_12' in 'Account' overrides 'AbstractSigner::extract_32_12' which is not marked 'virtual' |
| warning | W200 | function 'extract_32_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_32_12' in 'Account' overrides 'AbstractSigner::replace_32_12' which is not marked 'virtual' |
| warning | W200 | function 'replace_32_12' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_32_16' in 'Account' overrides 'AbstractSigner::extract_32_16' which is not marked 'virtual' |
| warning | W200 | function 'extract_32_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_32_16' in 'Account' overrides 'AbstractSigner::replace_32_16' which is not marked 'virtual' |
| warning | W200 | function 'replace_32_16' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_32_20' in 'Account' overrides 'AbstractSigner::extract_32_20' which is not marked 'virtual' |
| warning | W200 | function 'extract_32_20' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_32_20' in 'Account' overrides 'AbstractSigner::replace_32_20' which is not marked 'virtual' |
| warning | W200 | function 'replace_32_20' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_32_22' in 'Account' overrides 'AbstractSigner::extract_32_22' which is not marked 'virtual' |
| warning | W200 | function 'extract_32_22' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_32_22' in 'Account' overrides 'AbstractSigner::replace_32_22' which is not marked 'virtual' |
| warning | W200 | function 'replace_32_22' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_32_24' in 'Account' overrides 'AbstractSigner::extract_32_24' which is not marked 'virtual' |
| warning | W200 | function 'extract_32_24' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_32_24' in 'Account' overrides 'AbstractSigner::replace_32_24' which is not marked 'virtual' |
| warning | W200 | function 'replace_32_24' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'extract_32_28' in 'Account' overrides 'AbstractSigner::extract_32_28' which is not marked 'virtual' |
| warning | W200 | function 'extract_32_28' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'replace_32_28' in 'Account' overrides 'AbstractSigner::replace_32_28' which is not marked 'virtual' |
| warning | W200 | function 'replace_32_28' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseValidationData' in 'Account' overrides 'AbstractSigner::parseValidationData' which is not marked 'virtual' |
| warning | W200 | function 'parseValidationData' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'packValidationData' in 'Account' overrides 'AbstractSigner::packValidationData' which is not marked 'virtual' |
| warning | W200 | function 'packValidationData' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'packValidationData' in 'Account' overrides 'AbstractSigner::packValidationData' which is not marked 'virtual' |
| warning | W200 | function 'packValidationData' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'combineValidationData' in 'Account' overrides 'AbstractSigner::combineValidationData' which is not marked 'virtual' |
| warning | W200 | function 'combineValidationData' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getValidationData' in 'Account' overrides 'AbstractSigner::getValidationData' which is not marked 'virtual' |
| warning | W200 | function 'getValidationData' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hash' in 'Account' overrides 'AbstractSigner::hash' which is not marked 'virtual' |
| warning | W200 | function 'hash' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'factory' in 'Account' overrides 'AbstractSigner::factory' which is not marked 'virtual' |
| warning | W200 | function 'factory' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'factoryData' in 'Account' overrides 'AbstractSigner::factoryData' which is not marked 'virtual' |
| warning | W200 | function 'factoryData' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verificationGasLimit' in 'Account' overrides 'AbstractSigner::verificationGasLimit' which is not marked 'virtual' |
| warning | W200 | function 'verificationGasLimit' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callGasLimit' in 'Account' overrides 'AbstractSigner::callGasLimit' which is not marked 'virtual' |
| warning | W200 | function 'callGasLimit' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxPriorityFeePerGas' in 'Account' overrides 'AbstractSigner::maxPriorityFeePerGas' which is not marked 'virtual' |
| warning | W200 | function 'maxPriorityFeePerGas' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxFeePerGas' in 'Account' overrides 'AbstractSigner::maxFeePerGas' which is not marked 'virtual' |
| warning | W200 | function 'maxFeePerGas' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'gasPrice' in 'Account' overrides 'AbstractSigner::gasPrice' which is not marked 'virtual' |
| warning | W200 | function 'gasPrice' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'paymaster' in 'Account' overrides 'AbstractSigner::paymaster' which is not marked 'virtual' |
| warning | W200 | function 'paymaster' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'paymasterVerificationGasLimit' in 'Account' overrides 'AbstractSigner::paymasterVerificationGasLimit' which is not marked 'virtual' |
| warning | W200 | function 'paymasterVerificationGasLimit' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'paymasterPostOpGasLimit' in 'Account' overrides 'AbstractSigner::paymasterPostOpGasLimit' which is not marked 'virtual' |
| warning | W200 | function 'paymasterPostOpGasLimit' in 'Account' overrides a base function but is not marked 'override' |
| warning | W200 | function 'paymasterData' in 'Account' overrides 'AbstractSigner::paymasterData' which is not marked 'virtual' |
| warning | W200 | function 'paymasterData' in 'Account' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Account' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@openzeppelin/contracts/account/Account.sol`