# ERC20PermitUpgradeable (OpenZeppelin Upgradeable)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20PermitUpgradeable.sol`
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
| error | RAW | [info][NEP-17] NEP-17 `transfer` method has 2 parameter(s), spec expects 4. See STANDARDS_MAPPING.md for details. |
| error | RAW | [info][NEP-17] NEP-17 `transfer` method has 2 parameter(s), spec expects 4. See STANDARDS_MAPPING.md for details. |
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
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
| warning | INVALID_STORAGE_RETURN | function '_getERC20Storage' return value 'ERC20Storage' uses 'storage' data location (treated as Any) |
| warning | W101 | function 'transfer' has 2 parameters (ERC-20 pattern). NEP-17 requires 4 parameters: transfer(from, to, amount, data). The `from` address is verified via Runtime.checkWitness() and `data` (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |
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
| warning | W200 | function 'tryRecover' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
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
| warning | INVALID_STORAGE_RETURN | function '_getEIP712Storage' return value 'EIP712Storage' uses 'storage' data location (treated as Any) |
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
| warning | INVALID_STORAGE_RETURN | function '_getNoncesStorage' return value 'NoncesStorage' uses 'storage' data location (treated as Any) |
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
| warning | INVALID_STORAGE_RETURN | function '_getERC20Storage' return value 'ERC20Storage' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '_getEIP712Storage' return value 'EIP712Storage' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '_getNoncesStorage' return value 'NoncesStorage' uses 'storage' data location (treated as Any) |
| warning | W101 | function 'transfer' has 2 parameters (ERC-20 pattern). NEP-17 requires 4 parameters: transfer(from, to, amount, data). The `from` address is verified via Runtime.checkWitness() and `data` (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W108 | ERC-2612 permit pattern detected (7-parameter permit function). Neo N3 uses Runtime.checkWitness() for authorization; off-chain signature permits are not needed. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |
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
| warning | W200 | function 'tryRecover' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'ERC20Upgradeable' overrides 'ContextUpgradeable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'ERC20Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'EIP712Upgradeable' overrides 'ERC20Upgradeable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'EIP712Upgradeable' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'tryRecover' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'ERC20PermitUpgradeable' overrides 'NoncesUpgradeable::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'ERC20PermitUpgradeable' overrides a base function but is not marked 'override' |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20PermitUpgradeable.sol`