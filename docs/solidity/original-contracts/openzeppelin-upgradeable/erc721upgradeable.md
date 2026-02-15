# ERC721Upgradeable (OpenZeppelin Upgradeable)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol`
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
| error | RAW | [info][NEP-11] NEP-11 `Transfer` event has 3 parameter(s), expected 4. |
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Initializable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
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
| warning | W200 | function 'checkOnERC721Received' in 'ContextUpgradeable' overrides 'Initializable::checkOnERC721Received' which is not marked 'virtual' |
| warning | W200 | function 'checkOnERC721Received' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
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
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ContextUpgradeable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
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
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W200 | function 'checkOnERC721Received' in 'ERC165Upgradeable' overrides 'Initializable::checkOnERC721Received' which is not marked 'virtual' |
| warning | W200 | function 'checkOnERC721Received' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
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
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ERC165Upgradeable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
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
| warning | INVALID_STORAGE_RETURN | function '_getERC721Storage' return value 'ERC721Storage' uses 'storage' data location (treated as Any) |
| warning | W104 | function 'transferFrom' with 3 parameters (ERC-721 pattern) detected. NEP-11 uses transfer(to, tokenId, data) with 3 parameters instead. Authorization is via Runtime.checkWitness(owner), not msg.sender. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W114 | NFT contract (has ownerOf) but missing onNEP11Payment callback. Other contracts cannot send NFTs to this contract. |
| warning | W200 | function 'checkOnERC721Received' in 'ContextUpgradeable' overrides 'Initializable::checkOnERC721Received' which is not marked 'virtual' |
| warning | W200 | function 'checkOnERC721Received' in 'ContextUpgradeable' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'checkOnERC721Received' in 'ERC165Upgradeable' overrides 'ContextUpgradeable::checkOnERC721Received' which is not marked 'virtual' |
| warning | W200 | function 'checkOnERC721Received' in 'ERC165Upgradeable' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'checkOnERC721Received' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::checkOnERC721Received' which is not marked 'virtual' |
| warning | W200 | function 'checkOnERC721Received' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC721Upgradeable' overrides 'ERC165Upgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC721Upgradeable' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ERC721Upgradeable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol`