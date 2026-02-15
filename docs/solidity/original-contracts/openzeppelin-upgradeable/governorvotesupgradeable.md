# GovernorVotesUpgradeable (OpenZeppelin Upgradeable)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@openzeppelin/contracts-upgradeable/governance/extensions/GovernorVotesUpgradeable.sol`
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
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
| warning | RETURN_TYPE_UNMAPPED | function 'toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Initializable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
| warning | RETURN_TYPE_UNMAPPED | function 'toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | INVALID_STORAGE_RETURN | function '_getEIP712Storage' return value 'EIP712Storage' uses 'storage' data location (treated as Any) |
| warning | RETURN_TYPE_UNMAPPED | function '__super_toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | W200 | function 'panic' in 'EIP712Upgradeable' overrides 'Initializable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'EIP712Upgradeable' overrides 'Initializable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'EIP712Upgradeable' overrides 'Initializable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'EIP712Upgradeable' overrides 'Initializable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'EIP712Upgradeable' overrides 'Initializable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'EIP712Upgradeable' overrides 'Initializable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'EIP712Upgradeable' overrides 'Initializable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'EIP712Upgradeable' overrides 'Initializable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'EIP712Upgradeable' overrides 'Initializable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'EIP712Upgradeable' overrides 'Initializable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'EIP712Upgradeable' overrides 'Initializable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'EIP712Upgradeable' overrides 'Initializable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'EIP712Upgradeable' overrides 'Initializable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'EIP712Upgradeable' overrides 'Initializable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'EIP712Upgradeable' overrides 'Initializable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'EIP712Upgradeable' overrides 'Initializable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'EIP712Upgradeable' overrides 'Initializable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'EIP712Upgradeable' overrides 'Initializable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'EIP712Upgradeable' overrides 'Initializable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'EIP712Upgradeable' overrides 'Initializable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'EIP712Upgradeable' overrides 'Initializable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'EIP712Upgradeable' overrides 'Initializable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'EIP712Upgradeable' overrides 'Initializable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'EIP712Upgradeable' overrides 'Initializable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'EIP712Upgradeable' overrides 'Initializable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'EIP712Upgradeable' overrides 'Initializable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'EIP712Upgradeable' overrides 'Initializable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'EIP712Upgradeable' overrides 'Initializable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'EIP712Upgradeable' overrides 'Initializable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'EIP712Upgradeable' overrides 'Initializable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'EIP712Upgradeable' overrides 'Initializable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'EIP712Upgradeable' overrides 'Initializable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'EIP712Upgradeable' overrides 'Initializable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'EIP712Upgradeable' overrides 'Initializable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'EIP712Upgradeable' overrides 'Initializable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'EIP712Upgradeable' overrides 'Initializable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'EIP712Upgradeable' overrides 'Initializable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'EIP712Upgradeable' overrides 'Initializable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'EIP712Upgradeable' overrides 'Initializable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'EIP712Upgradeable' overrides 'Initializable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'EIP712Upgradeable' overrides 'Initializable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'EIP712Upgradeable' overrides 'Initializable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'EIP712Upgradeable' overrides 'Initializable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'EIP712Upgradeable' overrides 'Initializable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'EIP712Upgradeable' overrides 'Initializable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'EIP712Upgradeable' overrides 'Initializable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'EIP712Upgradeable' overrides 'Initializable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'EIP712Upgradeable' overrides 'Initializable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'EIP712Upgradeable' overrides 'Initializable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'EIP712Upgradeable' overrides 'Initializable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'EIP712Upgradeable' overrides 'Initializable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'EIP712Upgradeable' overrides 'Initializable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'EIP712Upgradeable' overrides 'Initializable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'EIP712Upgradeable' overrides 'Initializable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'EIP712Upgradeable' overrides 'Initializable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'EIP712Upgradeable' overrides 'Initializable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'EIP712Upgradeable' overrides 'Initializable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'EIP712Upgradeable' overrides 'Initializable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'EIP712Upgradeable' overrides 'Initializable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'EIP712Upgradeable' overrides 'Initializable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'EIP712Upgradeable' overrides 'Initializable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'EIP712Upgradeable' overrides 'Initializable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'EIP712Upgradeable' overrides 'Initializable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'EIP712Upgradeable' overrides 'Initializable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'EIP712Upgradeable' overrides 'Initializable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'EIP712Upgradeable' overrides 'Initializable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'EIP712Upgradeable' overrides 'Initializable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'EIP712Upgradeable' overrides 'Initializable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'EIP712Upgradeable' overrides 'Initializable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'EIP712Upgradeable' overrides 'Initializable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'EIP712Upgradeable' overrides 'Initializable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'EIP712Upgradeable' overrides 'Initializable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'EIP712Upgradeable' overrides 'Initializable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'EIP712Upgradeable' overrides 'Initializable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'EIP712Upgradeable' overrides 'Initializable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'EIP712Upgradeable' overrides 'Initializable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides 'Initializable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides 'Initializable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides 'Initializable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides 'Initializable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'EIP712Upgradeable' overrides 'Initializable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides 'Initializable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides 'Initializable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides 'Initializable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides 'Initializable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'EIP712Upgradeable' overrides 'Initializable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'EIP712Upgradeable' overrides 'Initializable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides 'Initializable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides 'Initializable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides 'Initializable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides 'Initializable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'EIP712Upgradeable' overrides 'Initializable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides 'Initializable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides 'Initializable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides 'Initializable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides 'Initializable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides 'Initializable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides 'Initializable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides 'Initializable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides 'Initializable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'EIP712Upgradeable' overrides 'Initializable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides 'Initializable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides 'Initializable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides 'Initializable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides 'Initializable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'EIP712Upgradeable' overrides 'Initializable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'EIP712Upgradeable' overrides 'Initializable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'EIP712Upgradeable' overrides 'Initializable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'EIP712Upgradeable' overrides 'Initializable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'EIP712Upgradeable' overrides 'Initializable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides 'Initializable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides 'Initializable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides 'Initializable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides 'Initializable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'EIP712Upgradeable' overrides 'Initializable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides 'Initializable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides 'Initializable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides 'Initializable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides 'Initializable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'EIP712Upgradeable' overrides 'Initializable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides 'Initializable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides 'Initializable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides 'Initializable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides 'Initializable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'EIP712Upgradeable' overrides 'Initializable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides 'Initializable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides 'Initializable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides 'Initializable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides 'Initializable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'EIP712Upgradeable' overrides 'Initializable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'EIP712Upgradeable' overrides 'Initializable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'EIP712Upgradeable' overrides 'Initializable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides 'Initializable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides 'Initializable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides 'Initializable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides 'Initializable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'EIP712Upgradeable' overrides 'Initializable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'EIP712Upgradeable' overrides 'Initializable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'EIP712Upgradeable' overrides 'Initializable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'EIP712Upgradeable' overrides 'Initializable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'EIP712Upgradeable' overrides 'Initializable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'EIP712Upgradeable' overrides 'Initializable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'EIP712Upgradeable' overrides 'Initializable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'EIP712Upgradeable' overrides 'Initializable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712Upgradeable' overrides 'Initializable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'EIP712Upgradeable' overrides 'Initializable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712Upgradeable' overrides 'Initializable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'EIP712Upgradeable' overrides 'Initializable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'EIP712Upgradeable' overrides 'Initializable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'EIP712Upgradeable' overrides 'Initializable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'EIP712Upgradeable' overrides 'Initializable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'EIP712Upgradeable' overrides 'Initializable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'EIP712Upgradeable' overrides 'Initializable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'EIP712Upgradeable' overrides 'Initializable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'EIP712Upgradeable' overrides 'Initializable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'EIP712Upgradeable' overrides 'Initializable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'EIP712Upgradeable' overrides 'Initializable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'EIP712Upgradeable' overrides 'Initializable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'EIP712Upgradeable' overrides 'Initializable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'EIP712Upgradeable' overrides 'Initializable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'EIP712Upgradeable' overrides 'Initializable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'EIP712Upgradeable' overrides 'Initializable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'EIP712Upgradeable' overrides 'Initializable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'EIP712Upgradeable' overrides 'Initializable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'EIP712Upgradeable' overrides 'Initializable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'EIP712Upgradeable' overrides 'Initializable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'EIP712Upgradeable' overrides 'Initializable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'EIP712Upgradeable' overrides 'Initializable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'EIP712Upgradeable' overrides 'Initializable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'EIP712Upgradeable' overrides 'Initializable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'EIP712Upgradeable' overrides 'Initializable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'EIP712Upgradeable' overrides 'Initializable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'EIP712Upgradeable' overrides 'Initializable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'EIP712Upgradeable' overrides 'Initializable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'EIP712Upgradeable' overrides 'Initializable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'EIP712Upgradeable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
| warning | RETURN_TYPE_UNMAPPED | function 'toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W200 | function 'panic' in 'ERC165Upgradeable' overrides 'Initializable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC165Upgradeable' overrides 'Initializable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC165Upgradeable' overrides 'Initializable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC165Upgradeable' overrides 'Initializable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC165Upgradeable' overrides 'Initializable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC165Upgradeable' overrides 'Initializable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC165Upgradeable' overrides 'Initializable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC165Upgradeable' overrides 'Initializable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC165Upgradeable' overrides 'Initializable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC165Upgradeable' overrides 'Initializable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC165Upgradeable' overrides 'Initializable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC165Upgradeable' overrides 'Initializable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC165Upgradeable' overrides 'Initializable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC165Upgradeable' overrides 'Initializable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC165Upgradeable' overrides 'Initializable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC165Upgradeable' overrides 'Initializable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC165Upgradeable' overrides 'Initializable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC165Upgradeable' overrides 'Initializable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC165Upgradeable' overrides 'Initializable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC165Upgradeable' overrides 'Initializable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC165Upgradeable' overrides 'Initializable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC165Upgradeable' overrides 'Initializable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC165Upgradeable' overrides 'Initializable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC165Upgradeable' overrides 'Initializable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC165Upgradeable' overrides 'Initializable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC165Upgradeable' overrides 'Initializable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC165Upgradeable' overrides 'Initializable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC165Upgradeable' overrides 'Initializable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC165Upgradeable' overrides 'Initializable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC165Upgradeable' overrides 'Initializable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC165Upgradeable' overrides 'Initializable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC165Upgradeable' overrides 'Initializable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC165Upgradeable' overrides 'Initializable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC165Upgradeable' overrides 'Initializable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC165Upgradeable' overrides 'Initializable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC165Upgradeable' overrides 'Initializable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC165Upgradeable' overrides 'Initializable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC165Upgradeable' overrides 'Initializable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC165Upgradeable' overrides 'Initializable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC165Upgradeable' overrides 'Initializable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC165Upgradeable' overrides 'Initializable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC165Upgradeable' overrides 'Initializable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC165Upgradeable' overrides 'Initializable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC165Upgradeable' overrides 'Initializable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC165Upgradeable' overrides 'Initializable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC165Upgradeable' overrides 'Initializable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC165Upgradeable' overrides 'Initializable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC165Upgradeable' overrides 'Initializable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC165Upgradeable' overrides 'Initializable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC165Upgradeable' overrides 'Initializable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC165Upgradeable' overrides 'Initializable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC165Upgradeable' overrides 'Initializable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC165Upgradeable' overrides 'Initializable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC165Upgradeable' overrides 'Initializable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC165Upgradeable' overrides 'Initializable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC165Upgradeable' overrides 'Initializable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC165Upgradeable' overrides 'Initializable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC165Upgradeable' overrides 'Initializable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC165Upgradeable' overrides 'Initializable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC165Upgradeable' overrides 'Initializable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC165Upgradeable' overrides 'Initializable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC165Upgradeable' overrides 'Initializable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC165Upgradeable' overrides 'Initializable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC165Upgradeable' overrides 'Initializable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC165Upgradeable' overrides 'Initializable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC165Upgradeable' overrides 'Initializable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC165Upgradeable' overrides 'Initializable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC165Upgradeable' overrides 'Initializable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC165Upgradeable' overrides 'Initializable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC165Upgradeable' overrides 'Initializable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC165Upgradeable' overrides 'Initializable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC165Upgradeable' overrides 'Initializable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC165Upgradeable' overrides 'Initializable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC165Upgradeable' overrides 'Initializable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC165Upgradeable' overrides 'Initializable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC165Upgradeable' overrides 'Initializable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC165Upgradeable' overrides 'Initializable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC165Upgradeable' overrides 'Initializable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC165Upgradeable' overrides 'Initializable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC165Upgradeable' overrides 'Initializable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC165Upgradeable' overrides 'Initializable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC165Upgradeable' overrides 'Initializable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC165Upgradeable' overrides 'Initializable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC165Upgradeable' overrides 'Initializable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC165Upgradeable' overrides 'Initializable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC165Upgradeable' overrides 'Initializable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC165Upgradeable' overrides 'Initializable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC165Upgradeable' overrides 'Initializable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC165Upgradeable' overrides 'Initializable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC165Upgradeable' overrides 'Initializable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC165Upgradeable' overrides 'Initializable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC165Upgradeable' overrides 'Initializable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC165Upgradeable' overrides 'Initializable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC165Upgradeable' overrides 'Initializable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC165Upgradeable' overrides 'Initializable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC165Upgradeable' overrides 'Initializable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC165Upgradeable' overrides 'Initializable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC165Upgradeable' overrides 'Initializable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC165Upgradeable' overrides 'Initializable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC165Upgradeable' overrides 'Initializable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC165Upgradeable' overrides 'Initializable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC165Upgradeable' overrides 'Initializable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC165Upgradeable' overrides 'Initializable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC165Upgradeable' overrides 'Initializable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC165Upgradeable' overrides 'Initializable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC165Upgradeable' overrides 'Initializable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC165Upgradeable' overrides 'Initializable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC165Upgradeable' overrides 'Initializable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC165Upgradeable' overrides 'Initializable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC165Upgradeable' overrides 'Initializable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC165Upgradeable' overrides 'Initializable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC165Upgradeable' overrides 'Initializable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC165Upgradeable' overrides 'Initializable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC165Upgradeable' overrides 'Initializable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC165Upgradeable' overrides 'Initializable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC165Upgradeable' overrides 'Initializable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC165Upgradeable' overrides 'Initializable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC165Upgradeable' overrides 'Initializable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC165Upgradeable' overrides 'Initializable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC165Upgradeable' overrides 'Initializable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC165Upgradeable' overrides 'Initializable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC165Upgradeable' overrides 'Initializable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165Upgradeable' overrides 'Initializable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165Upgradeable' overrides 'Initializable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC165Upgradeable' overrides 'Initializable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC165Upgradeable' overrides 'Initializable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC165Upgradeable' overrides 'Initializable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC165Upgradeable' overrides 'Initializable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC165Upgradeable' overrides 'Initializable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC165Upgradeable' overrides 'Initializable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC165Upgradeable' overrides 'Initializable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC165Upgradeable' overrides 'Initializable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165Upgradeable' overrides 'Initializable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165Upgradeable' overrides 'Initializable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165Upgradeable' overrides 'Initializable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165Upgradeable' overrides 'Initializable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'ERC165Upgradeable' overrides 'Initializable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'ERC165Upgradeable' overrides 'Initializable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ERC165Upgradeable' overrides 'Initializable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ERC165Upgradeable' overrides 'Initializable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ERC165Upgradeable' overrides 'Initializable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ERC165Upgradeable' overrides 'Initializable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ERC165Upgradeable' overrides 'Initializable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ERC165Upgradeable' overrides 'Initializable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165Upgradeable' overrides 'Initializable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ERC165Upgradeable' overrides 'Initializable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165Upgradeable' overrides 'Initializable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'ERC165Upgradeable' overrides 'Initializable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'ERC165Upgradeable' overrides 'Initializable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'ERC165Upgradeable' overrides 'Initializable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'ERC165Upgradeable' overrides 'Initializable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'ERC165Upgradeable' overrides 'Initializable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'ERC165Upgradeable' overrides 'Initializable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'ERC165Upgradeable' overrides 'Initializable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ERC165Upgradeable' overrides 'Initializable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'ERC165Upgradeable' overrides 'Initializable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ERC165Upgradeable' overrides 'Initializable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'ERC165Upgradeable' overrides 'Initializable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'ERC165Upgradeable' overrides 'Initializable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ERC165Upgradeable' overrides 'Initializable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ERC165Upgradeable' overrides 'Initializable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ERC165Upgradeable' overrides 'Initializable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ERC165Upgradeable' overrides 'Initializable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC165Upgradeable' overrides 'Initializable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ERC165Upgradeable' overrides 'Initializable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ERC165Upgradeable' overrides 'Initializable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'ERC165Upgradeable' overrides 'Initializable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'ERC165Upgradeable' overrides 'Initializable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'ERC165Upgradeable' overrides 'Initializable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'ERC165Upgradeable' overrides 'Initializable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'ERC165Upgradeable' overrides 'Initializable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ERC165Upgradeable' overrides 'Initializable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'ERC165Upgradeable' overrides 'Initializable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'ERC165Upgradeable' overrides 'Initializable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'ERC165Upgradeable' overrides 'Initializable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ERC165Upgradeable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
| warning | RETURN_TYPE_UNMAPPED | function 'toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | W200 | function 'panic' in 'ContextUpgradeable' overrides 'Initializable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ContextUpgradeable' overrides 'Initializable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ContextUpgradeable' overrides 'Initializable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ContextUpgradeable' overrides 'Initializable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ContextUpgradeable' overrides 'Initializable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ContextUpgradeable' overrides 'Initializable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ContextUpgradeable' overrides 'Initializable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ContextUpgradeable' overrides 'Initializable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ContextUpgradeable' overrides 'Initializable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ContextUpgradeable' overrides 'Initializable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ContextUpgradeable' overrides 'Initializable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ContextUpgradeable' overrides 'Initializable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ContextUpgradeable' overrides 'Initializable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ContextUpgradeable' overrides 'Initializable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ContextUpgradeable' overrides 'Initializable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ContextUpgradeable' overrides 'Initializable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ContextUpgradeable' overrides 'Initializable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ContextUpgradeable' overrides 'Initializable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ContextUpgradeable' overrides 'Initializable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ContextUpgradeable' overrides 'Initializable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ContextUpgradeable' overrides 'Initializable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ContextUpgradeable' overrides 'Initializable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ContextUpgradeable' overrides 'Initializable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ContextUpgradeable' overrides 'Initializable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ContextUpgradeable' overrides 'Initializable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ContextUpgradeable' overrides 'Initializable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ContextUpgradeable' overrides 'Initializable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ContextUpgradeable' overrides 'Initializable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ContextUpgradeable' overrides 'Initializable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ContextUpgradeable' overrides 'Initializable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ContextUpgradeable' overrides 'Initializable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ContextUpgradeable' overrides 'Initializable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ContextUpgradeable' overrides 'Initializable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ContextUpgradeable' overrides 'Initializable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ContextUpgradeable' overrides 'Initializable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ContextUpgradeable' overrides 'Initializable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ContextUpgradeable' overrides 'Initializable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ContextUpgradeable' overrides 'Initializable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ContextUpgradeable' overrides 'Initializable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ContextUpgradeable' overrides 'Initializable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ContextUpgradeable' overrides 'Initializable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ContextUpgradeable' overrides 'Initializable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ContextUpgradeable' overrides 'Initializable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ContextUpgradeable' overrides 'Initializable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ContextUpgradeable' overrides 'Initializable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ContextUpgradeable' overrides 'Initializable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ContextUpgradeable' overrides 'Initializable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ContextUpgradeable' overrides 'Initializable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ContextUpgradeable' overrides 'Initializable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ContextUpgradeable' overrides 'Initializable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ContextUpgradeable' overrides 'Initializable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ContextUpgradeable' overrides 'Initializable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ContextUpgradeable' overrides 'Initializable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ContextUpgradeable' overrides 'Initializable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ContextUpgradeable' overrides 'Initializable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ContextUpgradeable' overrides 'Initializable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ContextUpgradeable' overrides 'Initializable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ContextUpgradeable' overrides 'Initializable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ContextUpgradeable' overrides 'Initializable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ContextUpgradeable' overrides 'Initializable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ContextUpgradeable' overrides 'Initializable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ContextUpgradeable' overrides 'Initializable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ContextUpgradeable' overrides 'Initializable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ContextUpgradeable' overrides 'Initializable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ContextUpgradeable' overrides 'Initializable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ContextUpgradeable' overrides 'Initializable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ContextUpgradeable' overrides 'Initializable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ContextUpgradeable' overrides 'Initializable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ContextUpgradeable' overrides 'Initializable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ContextUpgradeable' overrides 'Initializable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ContextUpgradeable' overrides 'Initializable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ContextUpgradeable' overrides 'Initializable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ContextUpgradeable' overrides 'Initializable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ContextUpgradeable' overrides 'Initializable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ContextUpgradeable' overrides 'Initializable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ContextUpgradeable' overrides 'Initializable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ContextUpgradeable' overrides 'Initializable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ContextUpgradeable' overrides 'Initializable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ContextUpgradeable' overrides 'Initializable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ContextUpgradeable' overrides 'Initializable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ContextUpgradeable' overrides 'Initializable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ContextUpgradeable' overrides 'Initializable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ContextUpgradeable' overrides 'Initializable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ContextUpgradeable' overrides 'Initializable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ContextUpgradeable' overrides 'Initializable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ContextUpgradeable' overrides 'Initializable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ContextUpgradeable' overrides 'Initializable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ContextUpgradeable' overrides 'Initializable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ContextUpgradeable' overrides 'Initializable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ContextUpgradeable' overrides 'Initializable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ContextUpgradeable' overrides 'Initializable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ContextUpgradeable' overrides 'Initializable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ContextUpgradeable' overrides 'Initializable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ContextUpgradeable' overrides 'Initializable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ContextUpgradeable' overrides 'Initializable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ContextUpgradeable' overrides 'Initializable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ContextUpgradeable' overrides 'Initializable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ContextUpgradeable' overrides 'Initializable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ContextUpgradeable' overrides 'Initializable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ContextUpgradeable' overrides 'Initializable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ContextUpgradeable' overrides 'Initializable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ContextUpgradeable' overrides 'Initializable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ContextUpgradeable' overrides 'Initializable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ContextUpgradeable' overrides 'Initializable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ContextUpgradeable' overrides 'Initializable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ContextUpgradeable' overrides 'Initializable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ContextUpgradeable' overrides 'Initializable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ContextUpgradeable' overrides 'Initializable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ContextUpgradeable' overrides 'Initializable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ContextUpgradeable' overrides 'Initializable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ContextUpgradeable' overrides 'Initializable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ContextUpgradeable' overrides 'Initializable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ContextUpgradeable' overrides 'Initializable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ContextUpgradeable' overrides 'Initializable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ContextUpgradeable' overrides 'Initializable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ContextUpgradeable' overrides 'Initializable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ContextUpgradeable' overrides 'Initializable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ContextUpgradeable' overrides 'Initializable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ContextUpgradeable' overrides 'Initializable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ContextUpgradeable' overrides 'Initializable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ContextUpgradeable' overrides 'Initializable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ContextUpgradeable' overrides 'Initializable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ContextUpgradeable' overrides 'Initializable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ContextUpgradeable' overrides 'Initializable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ContextUpgradeable' overrides 'Initializable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ContextUpgradeable' overrides 'Initializable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ContextUpgradeable' overrides 'Initializable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ContextUpgradeable' overrides 'Initializable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ContextUpgradeable' overrides 'Initializable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ContextUpgradeable' overrides 'Initializable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ContextUpgradeable' overrides 'Initializable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ContextUpgradeable' overrides 'Initializable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ContextUpgradeable' overrides 'Initializable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ContextUpgradeable' overrides 'Initializable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ContextUpgradeable' overrides 'Initializable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ContextUpgradeable' overrides 'Initializable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'ContextUpgradeable' overrides 'Initializable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'ContextUpgradeable' overrides 'Initializable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ContextUpgradeable' overrides 'Initializable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ContextUpgradeable' overrides 'Initializable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ContextUpgradeable' overrides 'Initializable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ContextUpgradeable' overrides 'Initializable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ContextUpgradeable' overrides 'Initializable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ContextUpgradeable' overrides 'Initializable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ContextUpgradeable' overrides 'Initializable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ContextUpgradeable' overrides 'Initializable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ContextUpgradeable' overrides 'Initializable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'ContextUpgradeable' overrides 'Initializable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'ContextUpgradeable' overrides 'Initializable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'ContextUpgradeable' overrides 'Initializable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'ContextUpgradeable' overrides 'Initializable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'ContextUpgradeable' overrides 'Initializable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'ContextUpgradeable' overrides 'Initializable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'ContextUpgradeable' overrides 'Initializable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ContextUpgradeable' overrides 'Initializable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'ContextUpgradeable' overrides 'Initializable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ContextUpgradeable' overrides 'Initializable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'ContextUpgradeable' overrides 'Initializable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'ContextUpgradeable' overrides 'Initializable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ContextUpgradeable' overrides 'Initializable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ContextUpgradeable' overrides 'Initializable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ContextUpgradeable' overrides 'Initializable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ContextUpgradeable' overrides 'Initializable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ContextUpgradeable' overrides 'Initializable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ContextUpgradeable' overrides 'Initializable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ContextUpgradeable' overrides 'Initializable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'ContextUpgradeable' overrides 'Initializable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'ContextUpgradeable' overrides 'Initializable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'ContextUpgradeable' overrides 'Initializable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'ContextUpgradeable' overrides 'Initializable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'ContextUpgradeable' overrides 'Initializable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ContextUpgradeable' overrides 'Initializable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'ContextUpgradeable' overrides 'Initializable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'ContextUpgradeable' overrides 'Initializable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'ContextUpgradeable' overrides 'Initializable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ContextUpgradeable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
| warning | RETURN_TYPE_UNMAPPED | function 'toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | INVALID_STORAGE_RETURN | function '_getNoncesStorage' return value 'NoncesStorage' uses 'storage' data location (treated as Any) |
| warning | RETURN_TYPE_UNMAPPED | function '__super_toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | W200 | function 'panic' in 'NoncesUpgradeable' overrides 'Initializable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'NoncesUpgradeable' overrides 'Initializable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'NoncesUpgradeable' overrides 'Initializable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'NoncesUpgradeable' overrides 'Initializable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'NoncesUpgradeable' overrides 'Initializable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'NoncesUpgradeable' overrides 'Initializable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'NoncesUpgradeable' overrides 'Initializable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'NoncesUpgradeable' overrides 'Initializable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'NoncesUpgradeable' overrides 'Initializable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'NoncesUpgradeable' overrides 'Initializable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'NoncesUpgradeable' overrides 'Initializable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'NoncesUpgradeable' overrides 'Initializable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'NoncesUpgradeable' overrides 'Initializable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'NoncesUpgradeable' overrides 'Initializable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'NoncesUpgradeable' overrides 'Initializable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'NoncesUpgradeable' overrides 'Initializable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'NoncesUpgradeable' overrides 'Initializable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'NoncesUpgradeable' overrides 'Initializable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'NoncesUpgradeable' overrides 'Initializable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'NoncesUpgradeable' overrides 'Initializable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'NoncesUpgradeable' overrides 'Initializable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'NoncesUpgradeable' overrides 'Initializable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'NoncesUpgradeable' overrides 'Initializable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'NoncesUpgradeable' overrides 'Initializable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'NoncesUpgradeable' overrides 'Initializable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'NoncesUpgradeable' overrides 'Initializable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'NoncesUpgradeable' overrides 'Initializable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'NoncesUpgradeable' overrides 'Initializable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'NoncesUpgradeable' overrides 'Initializable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'NoncesUpgradeable' overrides 'Initializable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'NoncesUpgradeable' overrides 'Initializable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'NoncesUpgradeable' overrides 'Initializable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'NoncesUpgradeable' overrides 'Initializable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'NoncesUpgradeable' overrides 'Initializable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'NoncesUpgradeable' overrides 'Initializable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'NoncesUpgradeable' overrides 'Initializable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'NoncesUpgradeable' overrides 'Initializable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'NoncesUpgradeable' overrides 'Initializable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'NoncesUpgradeable' overrides 'Initializable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'NoncesUpgradeable' overrides 'Initializable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'NoncesUpgradeable' overrides 'Initializable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'NoncesUpgradeable' overrides 'Initializable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'NoncesUpgradeable' overrides 'Initializable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'NoncesUpgradeable' overrides 'Initializable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'NoncesUpgradeable' overrides 'Initializable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'NoncesUpgradeable' overrides 'Initializable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'NoncesUpgradeable' overrides 'Initializable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'NoncesUpgradeable' overrides 'Initializable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'NoncesUpgradeable' overrides 'Initializable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'NoncesUpgradeable' overrides 'Initializable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'NoncesUpgradeable' overrides 'Initializable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'NoncesUpgradeable' overrides 'Initializable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'NoncesUpgradeable' overrides 'Initializable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'NoncesUpgradeable' overrides 'Initializable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'NoncesUpgradeable' overrides 'Initializable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'NoncesUpgradeable' overrides 'Initializable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'NoncesUpgradeable' overrides 'Initializable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'NoncesUpgradeable' overrides 'Initializable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'NoncesUpgradeable' overrides 'Initializable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'NoncesUpgradeable' overrides 'Initializable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'NoncesUpgradeable' overrides 'Initializable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'NoncesUpgradeable' overrides 'Initializable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'NoncesUpgradeable' overrides 'Initializable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'NoncesUpgradeable' overrides 'Initializable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'NoncesUpgradeable' overrides 'Initializable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'NoncesUpgradeable' overrides 'Initializable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'NoncesUpgradeable' overrides 'Initializable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'NoncesUpgradeable' overrides 'Initializable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'NoncesUpgradeable' overrides 'Initializable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'NoncesUpgradeable' overrides 'Initializable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'NoncesUpgradeable' overrides 'Initializable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'NoncesUpgradeable' overrides 'Initializable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'NoncesUpgradeable' overrides 'Initializable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'NoncesUpgradeable' overrides 'Initializable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'NoncesUpgradeable' overrides 'Initializable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'NoncesUpgradeable' overrides 'Initializable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'NoncesUpgradeable' overrides 'Initializable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'NoncesUpgradeable' overrides 'Initializable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'NoncesUpgradeable' overrides 'Initializable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'NoncesUpgradeable' overrides 'Initializable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'NoncesUpgradeable' overrides 'Initializable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'NoncesUpgradeable' overrides 'Initializable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'NoncesUpgradeable' overrides 'Initializable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'NoncesUpgradeable' overrides 'Initializable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'NoncesUpgradeable' overrides 'Initializable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'NoncesUpgradeable' overrides 'Initializable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'NoncesUpgradeable' overrides 'Initializable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'NoncesUpgradeable' overrides 'Initializable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'NoncesUpgradeable' overrides 'Initializable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'NoncesUpgradeable' overrides 'Initializable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'NoncesUpgradeable' overrides 'Initializable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'NoncesUpgradeable' overrides 'Initializable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'NoncesUpgradeable' overrides 'Initializable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'NoncesUpgradeable' overrides 'Initializable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'NoncesUpgradeable' overrides 'Initializable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'NoncesUpgradeable' overrides 'Initializable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'NoncesUpgradeable' overrides 'Initializable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'NoncesUpgradeable' overrides 'Initializable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'NoncesUpgradeable' overrides 'Initializable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'NoncesUpgradeable' overrides 'Initializable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'NoncesUpgradeable' overrides 'Initializable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'NoncesUpgradeable' overrides 'Initializable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'NoncesUpgradeable' overrides 'Initializable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'NoncesUpgradeable' overrides 'Initializable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'NoncesUpgradeable' overrides 'Initializable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'NoncesUpgradeable' overrides 'Initializable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'NoncesUpgradeable' overrides 'Initializable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'NoncesUpgradeable' overrides 'Initializable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'NoncesUpgradeable' overrides 'Initializable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'NoncesUpgradeable' overrides 'Initializable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'NoncesUpgradeable' overrides 'Initializable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'NoncesUpgradeable' overrides 'Initializable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'NoncesUpgradeable' overrides 'Initializable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'NoncesUpgradeable' overrides 'Initializable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'NoncesUpgradeable' overrides 'Initializable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'NoncesUpgradeable' overrides 'Initializable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'NoncesUpgradeable' overrides 'Initializable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'NoncesUpgradeable' overrides 'Initializable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'NoncesUpgradeable' overrides 'Initializable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'NoncesUpgradeable' overrides 'Initializable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'NoncesUpgradeable' overrides 'Initializable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'NoncesUpgradeable' overrides 'Initializable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'NoncesUpgradeable' overrides 'Initializable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'NoncesUpgradeable' overrides 'Initializable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'NoncesUpgradeable' overrides 'Initializable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'NoncesUpgradeable' overrides 'Initializable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'NoncesUpgradeable' overrides 'Initializable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'NoncesUpgradeable' overrides 'Initializable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'NoncesUpgradeable' overrides 'Initializable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'NoncesUpgradeable' overrides 'Initializable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'NoncesUpgradeable' overrides 'Initializable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'NoncesUpgradeable' overrides 'Initializable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'NoncesUpgradeable' overrides 'Initializable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'NoncesUpgradeable' overrides 'Initializable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'NoncesUpgradeable' overrides 'Initializable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'NoncesUpgradeable' overrides 'Initializable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'NoncesUpgradeable' overrides 'Initializable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'NoncesUpgradeable' overrides 'Initializable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'NoncesUpgradeable' overrides 'Initializable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'NoncesUpgradeable' overrides 'Initializable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'NoncesUpgradeable' overrides 'Initializable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'NoncesUpgradeable' overrides 'Initializable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'NoncesUpgradeable' overrides 'Initializable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'NoncesUpgradeable' overrides 'Initializable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'NoncesUpgradeable' overrides 'Initializable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'NoncesUpgradeable' overrides 'Initializable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'NoncesUpgradeable' overrides 'Initializable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'NoncesUpgradeable' overrides 'Initializable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'NoncesUpgradeable' overrides 'Initializable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'NoncesUpgradeable' overrides 'Initializable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'NoncesUpgradeable' overrides 'Initializable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'NoncesUpgradeable' overrides 'Initializable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'NoncesUpgradeable' overrides 'Initializable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'NoncesUpgradeable' overrides 'Initializable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'NoncesUpgradeable' overrides 'Initializable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'NoncesUpgradeable' overrides 'Initializable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'NoncesUpgradeable' overrides 'Initializable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'NoncesUpgradeable' overrides 'Initializable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'NoncesUpgradeable' overrides 'Initializable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'NoncesUpgradeable' overrides 'Initializable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'NoncesUpgradeable' overrides 'Initializable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'NoncesUpgradeable' overrides 'Initializable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'NoncesUpgradeable' overrides 'Initializable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'NoncesUpgradeable' overrides 'Initializable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'NoncesUpgradeable' overrides 'Initializable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'NoncesUpgradeable' overrides 'Initializable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'NoncesUpgradeable' overrides 'Initializable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'NoncesUpgradeable' overrides 'Initializable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'NoncesUpgradeable' overrides 'Initializable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'NoncesUpgradeable' overrides 'Initializable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'NoncesUpgradeable' overrides 'Initializable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'NoncesUpgradeable' overrides 'Initializable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'NoncesUpgradeable' overrides 'Initializable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'NoncesUpgradeable' overrides 'Initializable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'NoncesUpgradeable' overrides 'Initializable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'NoncesUpgradeable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | VALIDATION_WARNING | function '_getFullAt' should return 3 values but expression does not match tuple |
| warning | VALIDATION_WARNING | function '__super__getFullAt' should return 3 values but expression does not match tuple |
| warning | W111 | function 'onNEP17Payment' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'execute' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'relay' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
| warning | RETURN_TYPE_UNMAPPED | function 'toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | INVALID_STORAGE_RETURN | function '_getEIP712Storage' return value 'EIP712Storage' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '_getNoncesStorage' return value 'NoncesStorage' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '_getGovernorStorage' return value 'GovernorStorage' uses 'storage' data location (treated as Any) |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W116 | function 'execute' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'relay' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W117 | function 'proposalDeadline' appears to be time-sensitive. block.timestamp on Neo N3 is deterministic but can be affected by block production timing. |
| warning | VALIDATION_WARNING | abstract contract 'GovernorUpgradeable' has 9 unimplemented function(s): [_quorumReached, _voteSucceeded, _getVotes, _countVote, clock, CLOCK_MODE, votingDelay, votingPeriod, quorum] |
| warning | W200 | function 'panic' in 'ContextUpgradeable' overrides 'Initializable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ContextUpgradeable' overrides 'Initializable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ContextUpgradeable' overrides 'Initializable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ContextUpgradeable' overrides 'Initializable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ContextUpgradeable' overrides 'Initializable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ContextUpgradeable' overrides 'Initializable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ContextUpgradeable' overrides 'Initializable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ContextUpgradeable' overrides 'Initializable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ContextUpgradeable' overrides 'Initializable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ContextUpgradeable' overrides 'Initializable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ContextUpgradeable' overrides 'Initializable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ContextUpgradeable' overrides 'Initializable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ContextUpgradeable' overrides 'Initializable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ContextUpgradeable' overrides 'Initializable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ContextUpgradeable' overrides 'Initializable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ContextUpgradeable' overrides 'Initializable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ContextUpgradeable' overrides 'Initializable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ContextUpgradeable' overrides 'Initializable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ContextUpgradeable' overrides 'Initializable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ContextUpgradeable' overrides 'Initializable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ContextUpgradeable' overrides 'Initializable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ContextUpgradeable' overrides 'Initializable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ContextUpgradeable' overrides 'Initializable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ContextUpgradeable' overrides 'Initializable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ContextUpgradeable' overrides 'Initializable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ContextUpgradeable' overrides 'Initializable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ContextUpgradeable' overrides 'Initializable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ContextUpgradeable' overrides 'Initializable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ContextUpgradeable' overrides 'Initializable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ContextUpgradeable' overrides 'Initializable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ContextUpgradeable' overrides 'Initializable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ContextUpgradeable' overrides 'Initializable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ContextUpgradeable' overrides 'Initializable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ContextUpgradeable' overrides 'Initializable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ContextUpgradeable' overrides 'Initializable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ContextUpgradeable' overrides 'Initializable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ContextUpgradeable' overrides 'Initializable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ContextUpgradeable' overrides 'Initializable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ContextUpgradeable' overrides 'Initializable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ContextUpgradeable' overrides 'Initializable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ContextUpgradeable' overrides 'Initializable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ContextUpgradeable' overrides 'Initializable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ContextUpgradeable' overrides 'Initializable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ContextUpgradeable' overrides 'Initializable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ContextUpgradeable' overrides 'Initializable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ContextUpgradeable' overrides 'Initializable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ContextUpgradeable' overrides 'Initializable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ContextUpgradeable' overrides 'Initializable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ContextUpgradeable' overrides 'Initializable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ContextUpgradeable' overrides 'Initializable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ContextUpgradeable' overrides 'Initializable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ContextUpgradeable' overrides 'Initializable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ContextUpgradeable' overrides 'Initializable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ContextUpgradeable' overrides 'Initializable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ContextUpgradeable' overrides 'Initializable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ContextUpgradeable' overrides 'Initializable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ContextUpgradeable' overrides 'Initializable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ContextUpgradeable' overrides 'Initializable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ContextUpgradeable' overrides 'Initializable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ContextUpgradeable' overrides 'Initializable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ContextUpgradeable' overrides 'Initializable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ContextUpgradeable' overrides 'Initializable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ContextUpgradeable' overrides 'Initializable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ContextUpgradeable' overrides 'Initializable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ContextUpgradeable' overrides 'Initializable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ContextUpgradeable' overrides 'Initializable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ContextUpgradeable' overrides 'Initializable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ContextUpgradeable' overrides 'Initializable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ContextUpgradeable' overrides 'Initializable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ContextUpgradeable' overrides 'Initializable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ContextUpgradeable' overrides 'Initializable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ContextUpgradeable' overrides 'Initializable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ContextUpgradeable' overrides 'Initializable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ContextUpgradeable' overrides 'Initializable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ContextUpgradeable' overrides 'Initializable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ContextUpgradeable' overrides 'Initializable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ContextUpgradeable' overrides 'Initializable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ContextUpgradeable' overrides 'Initializable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ContextUpgradeable' overrides 'Initializable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ContextUpgradeable' overrides 'Initializable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ContextUpgradeable' overrides 'Initializable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ContextUpgradeable' overrides 'Initializable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ContextUpgradeable' overrides 'Initializable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ContextUpgradeable' overrides 'Initializable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ContextUpgradeable' overrides 'Initializable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ContextUpgradeable' overrides 'Initializable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ContextUpgradeable' overrides 'Initializable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ContextUpgradeable' overrides 'Initializable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ContextUpgradeable' overrides 'Initializable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ContextUpgradeable' overrides 'Initializable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ContextUpgradeable' overrides 'Initializable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ContextUpgradeable' overrides 'Initializable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ContextUpgradeable' overrides 'Initializable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ContextUpgradeable' overrides 'Initializable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ContextUpgradeable' overrides 'Initializable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ContextUpgradeable' overrides 'Initializable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ContextUpgradeable' overrides 'Initializable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ContextUpgradeable' overrides 'Initializable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ContextUpgradeable' overrides 'Initializable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ContextUpgradeable' overrides 'Initializable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ContextUpgradeable' overrides 'Initializable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ContextUpgradeable' overrides 'Initializable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ContextUpgradeable' overrides 'Initializable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ContextUpgradeable' overrides 'Initializable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ContextUpgradeable' overrides 'Initializable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ContextUpgradeable' overrides 'Initializable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ContextUpgradeable' overrides 'Initializable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ContextUpgradeable' overrides 'Initializable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ContextUpgradeable' overrides 'Initializable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ContextUpgradeable' overrides 'Initializable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ContextUpgradeable' overrides 'Initializable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ContextUpgradeable' overrides 'Initializable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ContextUpgradeable' overrides 'Initializable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ContextUpgradeable' overrides 'Initializable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ContextUpgradeable' overrides 'Initializable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ContextUpgradeable' overrides 'Initializable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ContextUpgradeable' overrides 'Initializable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ContextUpgradeable' overrides 'Initializable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ContextUpgradeable' overrides 'Initializable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ContextUpgradeable' overrides 'Initializable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ContextUpgradeable' overrides 'Initializable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ContextUpgradeable' overrides 'Initializable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ContextUpgradeable' overrides 'Initializable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ContextUpgradeable' overrides 'Initializable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ContextUpgradeable' overrides 'Initializable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ContextUpgradeable' overrides 'Initializable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ContextUpgradeable' overrides 'Initializable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ContextUpgradeable' overrides 'Initializable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ContextUpgradeable' overrides 'Initializable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ContextUpgradeable' overrides 'Initializable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ContextUpgradeable' overrides 'Initializable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ContextUpgradeable' overrides 'Initializable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ContextUpgradeable' overrides 'Initializable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ContextUpgradeable' overrides 'Initializable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ContextUpgradeable' overrides 'Initializable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ContextUpgradeable' overrides 'Initializable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'ContextUpgradeable' overrides 'Initializable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'ContextUpgradeable' overrides 'Initializable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ContextUpgradeable' overrides 'Initializable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ContextUpgradeable' overrides 'Initializable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ContextUpgradeable' overrides 'Initializable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ContextUpgradeable' overrides 'Initializable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ContextUpgradeable' overrides 'Initializable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ContextUpgradeable' overrides 'Initializable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ContextUpgradeable' overrides 'Initializable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ContextUpgradeable' overrides 'Initializable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ContextUpgradeable' overrides 'Initializable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'ContextUpgradeable' overrides 'Initializable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'ContextUpgradeable' overrides 'Initializable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'ContextUpgradeable' overrides 'Initializable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'ContextUpgradeable' overrides 'Initializable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'ContextUpgradeable' overrides 'Initializable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'ContextUpgradeable' overrides 'Initializable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'ContextUpgradeable' overrides 'Initializable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ContextUpgradeable' overrides 'Initializable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'ContextUpgradeable' overrides 'Initializable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ContextUpgradeable' overrides 'Initializable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'ContextUpgradeable' overrides 'Initializable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'ContextUpgradeable' overrides 'Initializable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ContextUpgradeable' overrides 'Initializable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ContextUpgradeable' overrides 'Initializable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ContextUpgradeable' overrides 'Initializable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ContextUpgradeable' overrides 'Initializable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ContextUpgradeable' overrides 'Initializable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ContextUpgradeable' overrides 'Initializable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ContextUpgradeable' overrides 'Initializable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'ContextUpgradeable' overrides 'Initializable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'ContextUpgradeable' overrides 'Initializable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'ContextUpgradeable' overrides 'Initializable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'ContextUpgradeable' overrides 'Initializable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'ContextUpgradeable' overrides 'Initializable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ContextUpgradeable' overrides 'Initializable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'ContextUpgradeable' overrides 'Initializable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'ContextUpgradeable' overrides 'Initializable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'ContextUpgradeable' overrides 'Initializable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'GovernorUpgradeable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | VALIDATION_WARNING | function '_getFullAt' should return 3 values but expression does not match tuple |
| warning | VALIDATION_WARNING | function '__super__getFullAt' should return 3 values but expression does not match tuple |
| warning | W111 | function 'onNEP17Payment' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'execute' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'relay' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
| warning | RETURN_TYPE_UNMAPPED | function 'toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_toDelay' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_withUpdate' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function '__super_pack' returns 'Delay', which may not map cleanly to Neo manifest types |
| warning | INVALID_STORAGE_RETURN | function '_getEIP712Storage' return value 'EIP712Storage' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '_getNoncesStorage' return value 'NoncesStorage' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '_getGovernorStorage' return value 'GovernorStorage' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '_getGovernorVotesStorage' return value 'GovernorVotesStorage' uses 'storage' data location (treated as Any) |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W116 | function 'execute' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'relay' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W117 | function 'proposalDeadline' appears to be time-sensitive. block.timestamp on Neo N3 is deterministic but can be affected by block production timing. |
| warning | VALIDATION_WARNING | abstract contract 'GovernorVotesUpgradeable' has 6 unimplemented function(s): [_quorumReached, _voteSucceeded, _countVote, votingDelay, votingPeriod, quorum] |
| warning | W200 | function 'panic' in 'ContextUpgradeable' overrides 'Initializable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ContextUpgradeable' overrides 'Initializable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ContextUpgradeable' overrides 'Initializable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ContextUpgradeable' overrides 'Initializable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ContextUpgradeable' overrides 'Initializable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ContextUpgradeable' overrides 'Initializable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ContextUpgradeable' overrides 'Initializable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ContextUpgradeable' overrides 'Initializable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ContextUpgradeable' overrides 'Initializable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ContextUpgradeable' overrides 'Initializable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ContextUpgradeable' overrides 'Initializable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ContextUpgradeable' overrides 'Initializable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ContextUpgradeable' overrides 'Initializable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ContextUpgradeable' overrides 'Initializable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ContextUpgradeable' overrides 'Initializable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ContextUpgradeable' overrides 'Initializable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ContextUpgradeable' overrides 'Initializable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ContextUpgradeable' overrides 'Initializable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ContextUpgradeable' overrides 'Initializable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ContextUpgradeable' overrides 'Initializable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ContextUpgradeable' overrides 'Initializable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ContextUpgradeable' overrides 'Initializable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ContextUpgradeable' overrides 'Initializable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ContextUpgradeable' overrides 'Initializable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ContextUpgradeable' overrides 'Initializable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ContextUpgradeable' overrides 'Initializable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ContextUpgradeable' overrides 'Initializable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ContextUpgradeable' overrides 'Initializable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ContextUpgradeable' overrides 'Initializable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ContextUpgradeable' overrides 'Initializable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ContextUpgradeable' overrides 'Initializable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ContextUpgradeable' overrides 'Initializable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ContextUpgradeable' overrides 'Initializable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ContextUpgradeable' overrides 'Initializable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ContextUpgradeable' overrides 'Initializable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ContextUpgradeable' overrides 'Initializable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ContextUpgradeable' overrides 'Initializable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ContextUpgradeable' overrides 'Initializable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ContextUpgradeable' overrides 'Initializable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ContextUpgradeable' overrides 'Initializable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ContextUpgradeable' overrides 'Initializable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ContextUpgradeable' overrides 'Initializable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ContextUpgradeable' overrides 'Initializable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ContextUpgradeable' overrides 'Initializable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ContextUpgradeable' overrides 'Initializable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ContextUpgradeable' overrides 'Initializable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ContextUpgradeable' overrides 'Initializable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ContextUpgradeable' overrides 'Initializable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ContextUpgradeable' overrides 'Initializable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ContextUpgradeable' overrides 'Initializable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ContextUpgradeable' overrides 'Initializable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ContextUpgradeable' overrides 'Initializable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ContextUpgradeable' overrides 'Initializable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ContextUpgradeable' overrides 'Initializable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ContextUpgradeable' overrides 'Initializable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ContextUpgradeable' overrides 'Initializable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ContextUpgradeable' overrides 'Initializable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ContextUpgradeable' overrides 'Initializable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ContextUpgradeable' overrides 'Initializable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ContextUpgradeable' overrides 'Initializable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ContextUpgradeable' overrides 'Initializable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ContextUpgradeable' overrides 'Initializable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ContextUpgradeable' overrides 'Initializable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ContextUpgradeable' overrides 'Initializable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ContextUpgradeable' overrides 'Initializable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ContextUpgradeable' overrides 'Initializable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ContextUpgradeable' overrides 'Initializable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ContextUpgradeable' overrides 'Initializable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ContextUpgradeable' overrides 'Initializable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ContextUpgradeable' overrides 'Initializable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ContextUpgradeable' overrides 'Initializable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ContextUpgradeable' overrides 'Initializable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ContextUpgradeable' overrides 'Initializable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ContextUpgradeable' overrides 'Initializable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ContextUpgradeable' overrides 'Initializable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ContextUpgradeable' overrides 'Initializable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ContextUpgradeable' overrides 'Initializable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ContextUpgradeable' overrides 'Initializable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ContextUpgradeable' overrides 'Initializable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ContextUpgradeable' overrides 'Initializable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ContextUpgradeable' overrides 'Initializable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ContextUpgradeable' overrides 'Initializable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ContextUpgradeable' overrides 'Initializable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ContextUpgradeable' overrides 'Initializable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ContextUpgradeable' overrides 'Initializable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ContextUpgradeable' overrides 'Initializable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ContextUpgradeable' overrides 'Initializable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ContextUpgradeable' overrides 'Initializable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ContextUpgradeable' overrides 'Initializable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ContextUpgradeable' overrides 'Initializable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ContextUpgradeable' overrides 'Initializable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ContextUpgradeable' overrides 'Initializable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ContextUpgradeable' overrides 'Initializable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ContextUpgradeable' overrides 'Initializable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ContextUpgradeable' overrides 'Initializable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ContextUpgradeable' overrides 'Initializable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ContextUpgradeable' overrides 'Initializable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ContextUpgradeable' overrides 'Initializable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ContextUpgradeable' overrides 'Initializable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ContextUpgradeable' overrides 'Initializable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ContextUpgradeable' overrides 'Initializable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ContextUpgradeable' overrides 'Initializable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ContextUpgradeable' overrides 'Initializable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ContextUpgradeable' overrides 'Initializable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ContextUpgradeable' overrides 'Initializable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ContextUpgradeable' overrides 'Initializable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ContextUpgradeable' overrides 'Initializable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ContextUpgradeable' overrides 'Initializable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides 'Initializable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ContextUpgradeable' overrides 'Initializable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ContextUpgradeable' overrides 'Initializable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ContextUpgradeable' overrides 'Initializable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ContextUpgradeable' overrides 'Initializable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ContextUpgradeable' overrides 'Initializable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ContextUpgradeable' overrides 'Initializable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ContextUpgradeable' overrides 'Initializable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ContextUpgradeable' overrides 'Initializable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ContextUpgradeable' overrides 'Initializable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ContextUpgradeable' overrides 'Initializable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ContextUpgradeable' overrides 'Initializable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ContextUpgradeable' overrides 'Initializable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ContextUpgradeable' overrides 'Initializable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ContextUpgradeable' overrides 'Initializable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ContextUpgradeable' overrides 'Initializable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ContextUpgradeable' overrides 'Initializable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ContextUpgradeable' overrides 'Initializable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ContextUpgradeable' overrides 'Initializable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ContextUpgradeable' overrides 'Initializable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ContextUpgradeable' overrides 'Initializable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ContextUpgradeable' overrides 'Initializable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ContextUpgradeable' overrides 'Initializable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ContextUpgradeable' overrides 'Initializable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ContextUpgradeable' overrides 'Initializable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ContextUpgradeable' overrides 'Initializable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ContextUpgradeable' overrides 'Initializable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ContextUpgradeable' overrides 'Initializable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ContextUpgradeable' overrides 'Initializable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'ContextUpgradeable' overrides 'Initializable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides 'Initializable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides 'Initializable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'ContextUpgradeable' overrides 'Initializable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ContextUpgradeable' overrides 'Initializable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ContextUpgradeable' overrides 'Initializable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ContextUpgradeable' overrides 'Initializable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ContextUpgradeable' overrides 'Initializable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ContextUpgradeable' overrides 'Initializable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ContextUpgradeable' overrides 'Initializable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ContextUpgradeable' overrides 'Initializable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ContextUpgradeable' overrides 'Initializable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ContextUpgradeable' overrides 'Initializable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'ContextUpgradeable' overrides 'Initializable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'ContextUpgradeable' overrides 'Initializable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'ContextUpgradeable' overrides 'Initializable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'ContextUpgradeable' overrides 'Initializable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'ContextUpgradeable' overrides 'Initializable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'ContextUpgradeable' overrides 'Initializable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'ContextUpgradeable' overrides 'Initializable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ContextUpgradeable' overrides 'Initializable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'ContextUpgradeable' overrides 'Initializable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ContextUpgradeable' overrides 'Initializable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'ContextUpgradeable' overrides 'Initializable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'ContextUpgradeable' overrides 'Initializable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ContextUpgradeable' overrides 'Initializable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ContextUpgradeable' overrides 'Initializable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ContextUpgradeable' overrides 'Initializable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ContextUpgradeable' overrides 'Initializable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ContextUpgradeable' overrides 'Initializable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ContextUpgradeable' overrides 'Initializable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ContextUpgradeable' overrides 'Initializable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'ContextUpgradeable' overrides 'Initializable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'ContextUpgradeable' overrides 'Initializable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'ContextUpgradeable' overrides 'Initializable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'ContextUpgradeable' overrides 'Initializable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'ContextUpgradeable' overrides 'Initializable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ContextUpgradeable' overrides 'Initializable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'ContextUpgradeable' overrides 'Initializable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'ContextUpgradeable' overrides 'Initializable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'ContextUpgradeable' overrides 'Initializable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'EIP712Upgradeable' overrides 'ERC165Upgradeable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'NoncesUpgradeable' overrides 'EIP712Upgradeable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'NoncesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'GovernorUpgradeable' overrides 'NoncesUpgradeable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'GovernorUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'timestamp' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::timestamp' which is not marked 'virtual' |
| warning | W200 | function 'timestamp' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'blockNumber' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::blockNumber' which is not marked 'virtual' |
| warning | W200 | function 'blockNumber' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDelay' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::toDelay' which is not marked 'virtual' |
| warning | W200 | function 'toDelay' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFullAt' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::_getFullAt' which is not marked 'virtual' |
| warning | W200 | function '_getFullAt' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFull' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::getFull' which is not marked 'virtual' |
| warning | W200 | function 'getFull' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'withUpdate' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::withUpdate' which is not marked 'virtual' |
| warning | W200 | function 'withUpdate' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unpack' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::unpack' which is not marked 'virtual' |
| warning | W200 | function 'unpack' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pack' in 'GovernorVotesUpgradeable' overrides 'GovernorUpgradeable::pack' which is not marked 'virtual' |
| warning | W200 | function 'pack' in 'GovernorVotesUpgradeable' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'GovernorVotesUpgradeable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@openzeppelin/contracts-upgradeable/governance/extensions/GovernorVotesUpgradeable.sol`