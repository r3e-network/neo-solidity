# AccessManager (OpenZeppelin)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@openzeppelin/contracts/access/manager/AccessManager.sol`
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
| warning | VALIDATION_WARNING | function '_getFullAt' should return 3 values but expression does not match tuple |
| warning | RETURN_TYPE_UNMAPPED | function 'toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | VALIDATION_WARNING | function '_getFullAt' should return 3 values but expression does not match tuple |
| warning | VALIDATION_WARNING | function '__super__getFullAt' should return 3 values but expression does not match tuple |
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
| warning | RETURN_TYPE_UNMAPPED | function 'toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | W200 | function 'sendValue' in 'Multicall' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Multicall' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Multicall' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Multicall' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Multicall' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Multicall' overrides 'Context::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Multicall' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Multicall' overrides 'Context::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'Multicall' overrides 'Context::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'Multicall' overrides 'Context::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'Multicall' overrides 'Context::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'Multicall' overrides 'Context::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Multicall' overrides 'Context::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'Multicall' overrides 'Context::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'Multicall' overrides 'Context::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'Multicall' overrides 'Context::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'Multicall' overrides 'Context::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'Multicall' overrides 'Context::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'Multicall' overrides 'Context::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'Multicall' overrides 'Context::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Multicall' overrides 'Context::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'Multicall' overrides 'Context::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'Multicall' overrides 'Context::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'Multicall' overrides 'Context::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Multicall' overrides 'Context::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'Multicall' overrides 'Context::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'Multicall' overrides 'Context::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'Multicall' overrides 'Context::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Multicall' overrides 'Context::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'Multicall' overrides 'Context::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'Multicall' overrides 'Context::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'Multicall' overrides 'Context::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Multicall' overrides 'Context::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'Multicall' overrides 'Context::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'Multicall' overrides 'Context::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'Multicall' overrides 'Context::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Multicall' overrides 'Context::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'Multicall' overrides 'Context::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Multicall' overrides 'Context::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Multicall' overrides 'Context::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Multicall' overrides 'Context::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'Multicall' overrides 'Context::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'Multicall' overrides 'Context::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'Multicall' overrides 'Context::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'Multicall' overrides 'Context::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'Multicall' overrides 'Context::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'Multicall' overrides 'Context::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'Multicall' overrides 'Context::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'Multicall' overrides 'Context::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'Multicall' overrides 'Context::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'Multicall' overrides 'Context::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'Multicall' overrides 'Context::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'Multicall' overrides 'Context::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'Multicall' overrides 'Context::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'Multicall' overrides 'Context::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'Multicall' overrides 'Context::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Multicall' overrides 'Context::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'Multicall' overrides 'Context::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'Multicall' overrides 'Context::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'Multicall' overrides 'Context::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'Multicall' overrides 'Context::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'Multicall' overrides 'Context::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'Multicall' overrides 'Context::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'Multicall' overrides 'Context::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Multicall' overrides 'Context::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'Multicall' overrides 'Context::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'Multicall' overrides 'Context::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'Multicall' overrides 'Context::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Multicall' overrides 'Context::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'Multicall' overrides 'Context::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Multicall' overrides 'Context::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Multicall' overrides 'Context::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Multicall' overrides 'Context::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'Multicall' overrides 'Context::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'Multicall' overrides 'Context::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'Multicall' overrides 'Context::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'Multicall' overrides 'Context::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'Multicall' overrides 'Context::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'Multicall' overrides 'Context::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'Multicall' overrides 'Context::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'Multicall' overrides 'Context::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'Multicall' overrides 'Context::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'Multicall' overrides 'Context::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'Multicall' overrides 'Context::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'Multicall' overrides 'Context::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'Multicall' overrides 'Context::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'Multicall' overrides 'Context::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'Multicall' overrides 'Context::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'Multicall' overrides 'Context::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Multicall' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Multicall' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'Multicall' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'Multicall' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'Multicall' overrides 'Context::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'Multicall' overrides 'Context::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'Multicall' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'Multicall' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'Multicall' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'Multicall' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'Multicall' overrides 'Context::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'Multicall' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'Multicall' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'Multicall' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'Multicall' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'Multicall' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'Multicall' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'Multicall' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'Multicall' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'Multicall' overrides 'Context::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'Multicall' overrides 'Context::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'Multicall' overrides 'Context::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'Multicall' overrides 'Context::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'Multicall' overrides 'Context::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'Multicall' overrides 'Context::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'Multicall' overrides 'Context::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'Multicall' overrides 'Context::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'Multicall' overrides 'Context::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'Multicall' overrides 'Context::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | VALIDATION_WARNING | function '_getFullAt' should return 3 values but expression does not match tuple |
| warning | VALIDATION_WARNING | function '__super__getFullAt' should return 3 values but expression does not match tuple |
| warning | VALIDATION_WARNING | function 'canCall' should return 2 values but expression does not match tuple |
| warning | W111 | function 'execute' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | VALIDATION_WARNING | function '_canCallExtended' should return 2 values but expression does not match tuple |
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
| warning | RETURN_TYPE_UNMAPPED | function 'toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | W116 | function 'execute' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | VALIDATION_WARNING | 'using X for *' is supported; all library functions are available but type-specific filtering is not enforced at compile time |
| warning | W200 | function 'sendValue' in 'Multicall' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Multicall' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Multicall' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Multicall' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Multicall' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Multicall' overrides 'Context::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Multicall' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Multicall' overrides 'Context::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'Multicall' overrides 'Context::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'Multicall' overrides 'Context::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'Multicall' overrides 'Context::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'Multicall' overrides 'Context::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Multicall' overrides 'Context::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'Multicall' overrides 'Context::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'Multicall' overrides 'Context::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'Multicall' overrides 'Context::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'Multicall' overrides 'Context::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'Multicall' overrides 'Context::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'Multicall' overrides 'Context::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'Multicall' overrides 'Context::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Multicall' overrides 'Context::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'Multicall' overrides 'Context::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'Multicall' overrides 'Context::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'Multicall' overrides 'Context::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Multicall' overrides 'Context::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'Multicall' overrides 'Context::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'Multicall' overrides 'Context::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'Multicall' overrides 'Context::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Multicall' overrides 'Context::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'Multicall' overrides 'Context::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'Multicall' overrides 'Context::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'Multicall' overrides 'Context::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Multicall' overrides 'Context::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'Multicall' overrides 'Context::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'Multicall' overrides 'Context::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'Multicall' overrides 'Context::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Multicall' overrides 'Context::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'Multicall' overrides 'Context::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Multicall' overrides 'Context::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Multicall' overrides 'Context::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Multicall' overrides 'Context::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'Multicall' overrides 'Context::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'Multicall' overrides 'Context::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'Multicall' overrides 'Context::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'Multicall' overrides 'Context::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'Multicall' overrides 'Context::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'Multicall' overrides 'Context::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'Multicall' overrides 'Context::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'Multicall' overrides 'Context::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'Multicall' overrides 'Context::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'Multicall' overrides 'Context::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'Multicall' overrides 'Context::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'Multicall' overrides 'Context::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'Multicall' overrides 'Context::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'Multicall' overrides 'Context::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'Multicall' overrides 'Context::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Multicall' overrides 'Context::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'Multicall' overrides 'Context::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'Multicall' overrides 'Context::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'Multicall' overrides 'Context::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'Multicall' overrides 'Context::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'Multicall' overrides 'Context::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'Multicall' overrides 'Context::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'Multicall' overrides 'Context::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Multicall' overrides 'Context::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'Multicall' overrides 'Context::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'Multicall' overrides 'Context::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'Multicall' overrides 'Context::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Multicall' overrides 'Context::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'Multicall' overrides 'Context::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Multicall' overrides 'Context::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Multicall' overrides 'Context::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Multicall' overrides 'Context::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'Multicall' overrides 'Context::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'Multicall' overrides 'Context::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'Multicall' overrides 'Context::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'Multicall' overrides 'Context::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'Multicall' overrides 'Context::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'Multicall' overrides 'Context::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'Multicall' overrides 'Context::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'Multicall' overrides 'Context::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'Multicall' overrides 'Context::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'Multicall' overrides 'Context::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'Multicall' overrides 'Context::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'Multicall' overrides 'Context::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'Multicall' overrides 'Context::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'Multicall' overrides 'Context::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'Multicall' overrides 'Context::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'Multicall' overrides 'Context::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Multicall' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Multicall' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'Multicall' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'Multicall' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'Multicall' overrides 'Context::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'Multicall' overrides 'Context::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'Multicall' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'Multicall' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'Multicall' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'Multicall' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'Multicall' overrides 'Context::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'Multicall' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'Multicall' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'Multicall' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'Multicall' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'Multicall' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'Multicall' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'Multicall' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'Multicall' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'Multicall' overrides 'Context::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'Multicall' overrides 'Context::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'Multicall' overrides 'Context::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'Multicall' overrides 'Context::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'Multicall' overrides 'Context::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'Multicall' overrides 'Context::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'Multicall' overrides 'Context::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'Multicall' overrides 'Context::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'Multicall' overrides 'Context::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'Multicall' overrides 'Context::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'AccessManager' overrides 'Multicall::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AccessManager' overrides 'Multicall::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AccessManager' overrides 'Multicall::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AccessManager' overrides 'Multicall::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AccessManager' overrides 'Multicall::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AccessManager' overrides 'Multicall::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'AccessManager' overrides 'Multicall::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'AccessManager' overrides 'Multicall::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'AccessManager' overrides 'Multicall::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'AccessManager' overrides 'Multicall::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'AccessManager' overrides 'Multicall::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'AccessManager' overrides 'Multicall::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'AccessManager' overrides 'Multicall::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'AccessManager' overrides 'Multicall::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'AccessManager' overrides 'Multicall::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'AccessManager' overrides 'Multicall::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'AccessManager' overrides 'Multicall::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'AccessManager' overrides 'Multicall::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'AccessManager' overrides 'Multicall::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'AccessManager' overrides 'Multicall::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'AccessManager' overrides 'Multicall::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'AccessManager' overrides 'Multicall::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'AccessManager' overrides 'Multicall::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'AccessManager' overrides 'Multicall::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'AccessManager' overrides 'Multicall::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'AccessManager' overrides 'Multicall::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'AccessManager' overrides 'Multicall::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'AccessManager' overrides 'Multicall::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'AccessManager' overrides 'Multicall::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'AccessManager' overrides 'Multicall::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'AccessManager' overrides 'Multicall::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'AccessManager' overrides 'Multicall::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'AccessManager' overrides 'Multicall::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'AccessManager' overrides 'Multicall::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'AccessManager' overrides 'Multicall::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'AccessManager' overrides 'Multicall::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'AccessManager' overrides 'Multicall::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'AccessManager' overrides 'Multicall::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'AccessManager' overrides 'Multicall::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'AccessManager' overrides 'Multicall::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'AccessManager' overrides 'Multicall::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'AccessManager' overrides 'Multicall::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'AccessManager' overrides 'Multicall::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'AccessManager' overrides 'Multicall::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'AccessManager' overrides 'Multicall::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'AccessManager' overrides 'Multicall::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'AccessManager' overrides 'Multicall::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'AccessManager' overrides 'Multicall::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'AccessManager' overrides 'Multicall::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'AccessManager' overrides 'Multicall::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'AccessManager' overrides 'Multicall::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'AccessManager' overrides 'Multicall::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'AccessManager' overrides 'Multicall::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'AccessManager' overrides 'Multicall::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'AccessManager' overrides 'Multicall::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'AccessManager' overrides 'Multicall::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'AccessManager' overrides 'Multicall::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'AccessManager' overrides 'Multicall::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'AccessManager' overrides 'Multicall::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'AccessManager' overrides 'Multicall::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'AccessManager' overrides 'Multicall::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'AccessManager' overrides 'Multicall::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'AccessManager' overrides 'Multicall::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'AccessManager' overrides 'Multicall::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'AccessManager' overrides 'Multicall::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'AccessManager' overrides 'Multicall::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'AccessManager' overrides 'Multicall::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'AccessManager' overrides 'Multicall::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'AccessManager' overrides 'Multicall::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'AccessManager' overrides 'Multicall::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'AccessManager' overrides 'Multicall::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'AccessManager' overrides 'Multicall::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'AccessManager' overrides 'Multicall::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'AccessManager' overrides 'Multicall::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'AccessManager' overrides 'Multicall::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'AccessManager' overrides 'Multicall::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'AccessManager' overrides 'Multicall::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'AccessManager' overrides 'Multicall::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'AccessManager' overrides 'Multicall::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'AccessManager' overrides 'Multicall::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'AccessManager' overrides 'Multicall::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'AccessManager' overrides 'Multicall::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'AccessManager' overrides 'Multicall::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'AccessManager' overrides 'Multicall::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'AccessManager' overrides 'Multicall::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'AccessManager' overrides 'Multicall::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'AccessManager' overrides 'Multicall::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'AccessManager' overrides 'Multicall::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'AccessManager' overrides 'Multicall::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'AccessManager' overrides 'Multicall::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'AccessManager' overrides 'Multicall::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'AccessManager' overrides 'Multicall::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'AccessManager' overrides 'Multicall::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'AccessManager' overrides 'Multicall::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'AccessManager' overrides 'Multicall::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'AccessManager' overrides 'Multicall::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'AccessManager' overrides 'Multicall::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'AccessManager' overrides 'Multicall::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'AccessManager' overrides 'Multicall::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'AccessManager' overrides 'Multicall::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'AccessManager' overrides 'Multicall::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'AccessManager' overrides 'Multicall::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'AccessManager' overrides 'Multicall::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'AccessManager' overrides 'Multicall::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'AccessManager' overrides 'Multicall::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'AccessManager' overrides 'Multicall::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'AccessManager' overrides 'Multicall::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'AccessManager' overrides 'Multicall::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'AccessManager' overrides 'Multicall::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'AccessManager' overrides 'Multicall::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'AccessManager' overrides 'Multicall::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'AccessManager' overrides 'Multicall::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'AccessManager' overrides 'Multicall::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'AccessManager' overrides 'Multicall::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'AccessManager' overrides 'Multicall::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'AccessManager' overrides 'Multicall::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'AccessManager' overrides 'Multicall::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'AccessManager' overrides 'Multicall::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'AccessManager' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'AccessManager' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@openzeppelin/contracts/access/manager/AccessManager.sol`