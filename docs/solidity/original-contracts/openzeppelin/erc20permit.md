# ERC20Permit (OpenZeppelin)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol`
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
| warning | INVALID_STORAGE_RETURN | function 'getAddressSlot' return value 'AddressSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBooleanSlot' return value 'BooleanSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytes32Slot' return value 'Bytes32Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getUint256Slot' return value 'Uint256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getInt256Slot' return value 'Int256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
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
| warning | W121 | duplicate constant state variable 'FALLBACK_SENTINEL' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'getAddressSlot' return value 'AddressSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBooleanSlot' return value 'BooleanSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytes32Slot' return value 'Bytes32Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getUint256Slot' return value 'Uint256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getInt256Slot' return value 'Int256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getAddressSlot' return value 'AddressSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getBooleanSlot' return value 'BooleanSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getBytes32Slot' return value 'Bytes32Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getUint256Slot' return value 'Uint256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getInt256Slot' return value 'Int256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | W101 | function 'transfer' has 2 parameters (ERC-20 pattern). NEP-17 requires 4 parameters: transfer(from, to, amount, data). The `from` address is verified via Runtime.checkWitness() and `data` (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |
| warning | W200 | function 'tryRecover' in 'ERC20' overrides 'Context::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20' overrides 'Context::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC20' overrides 'Context::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20' overrides 'Context::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC20' overrides 'Context::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20' overrides 'Context::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'ERC20' overrides 'Context::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC20' overrides 'Context::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC20' overrides 'Context::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC20' overrides 'Context::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC20' overrides 'Context::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC20' overrides 'Context::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC20' overrides 'Context::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC20' overrides 'Context::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC20' overrides 'Context::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC20' overrides 'Context::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC20' overrides 'Context::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC20' overrides 'Context::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC20' overrides 'Context::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC20' overrides 'Context::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC20' overrides 'Context::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC20' overrides 'Context::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC20' overrides 'Context::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC20' overrides 'Context::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC20' overrides 'Context::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC20' overrides 'Context::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC20' overrides 'Context::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC20' overrides 'Context::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC20' overrides 'Context::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC20' overrides 'Context::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC20' overrides 'Context::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC20' overrides 'Context::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC20' overrides 'Context::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC20' overrides 'Context::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC20' overrides 'Context::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC20' overrides 'Context::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC20' overrides 'Context::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC20' overrides 'Context::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC20' overrides 'Context::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC20' overrides 'Context::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC20' overrides 'Context::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC20' overrides 'Context::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC20' overrides 'Context::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC20' overrides 'Context::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC20' overrides 'Context::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC20' overrides 'Context::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC20' overrides 'Context::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC20' overrides 'Context::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC20' overrides 'Context::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC20' overrides 'Context::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC20' overrides 'Context::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC20' overrides 'Context::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC20' overrides 'Context::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC20' overrides 'Context::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC20' overrides 'Context::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC20' overrides 'Context::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC20' overrides 'Context::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC20' overrides 'Context::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC20' overrides 'Context::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC20' overrides 'Context::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC20' overrides 'Context::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC20' overrides 'Context::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC20' overrides 'Context::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC20' overrides 'Context::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC20' overrides 'Context::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC20' overrides 'Context::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC20' overrides 'Context::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC20' overrides 'Context::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC20' overrides 'Context::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC20' overrides 'Context::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC20' overrides 'Context::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC20' overrides 'Context::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC20' overrides 'Context::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC20' overrides 'Context::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC20' overrides 'Context::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC20' overrides 'Context::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC20' overrides 'Context::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC20' overrides 'Context::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC20' overrides 'Context::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC20' overrides 'Context::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC20' overrides 'Context::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC20' overrides 'Context::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC20' overrides 'Context::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC20' overrides 'Context::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC20' overrides 'Context::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC20' overrides 'Context::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC20' overrides 'Context::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC20' overrides 'Context::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC20' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC20' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC20' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC20' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC20' overrides 'Context::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC20' overrides 'Context::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC20' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC20' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC20' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC20' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC20' overrides 'Context::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC20' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC20' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC20' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC20' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC20' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC20' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC20' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC20' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC20' overrides 'Context::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC20' overrides 'Context::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC20' overrides 'Context::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC20' overrides 'Context::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC20' overrides 'Context::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC20' overrides 'Context::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC20' overrides 'Context::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC20' overrides 'Context::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC20' overrides 'Context::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC20' overrides 'Context::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC20' overrides 'Context::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC20' overrides 'Context::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC20' overrides 'Context::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC20' overrides 'Context::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC20' overrides 'Context::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC20' overrides 'Context::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC20' overrides 'Context::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC20' overrides 'Context::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC20' overrides 'Context::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC20' overrides 'Context::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC20' overrides 'Context::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC20' overrides 'Context::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20' overrides 'Context::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20' overrides 'Context::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC20' overrides 'Context::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC20' overrides 'Context::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC20' overrides 'Context::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC20' overrides 'Context::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC20' overrides 'Context::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC20' overrides 'Context::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC20' overrides 'Context::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC20' overrides 'Context::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20' overrides 'Context::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20' overrides 'Context::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20' overrides 'Context::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20' overrides 'Context::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'ERC20' overrides 'Context::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'ERC20' overrides 'Context::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'ERC20' overrides 'Context::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'ERC20' overrides 'Context::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'ERC20' overrides 'Context::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'ERC20' overrides 'Context::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ERC20' overrides 'Context::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ERC20' overrides 'Context::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ERC20' overrides 'Context::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ERC20' overrides 'Context::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortString' in 'ERC20' overrides 'Context::toShortString' which is not marked 'virtual' |
| warning | W200 | function 'toShortString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC20' overrides 'Context::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLength' in 'ERC20' overrides 'Context::byteLength' which is not marked 'virtual' |
| warning | W200 | function 'byteLength' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortStringWithFallback' in 'ERC20' overrides 'Context::toShortStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toShortStringWithFallback' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringWithFallback' in 'ERC20' overrides 'Context::toStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toStringWithFallback' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLengthWithFallback' in 'ERC20' overrides 'Context::byteLengthWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'byteLengthWithFallback' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | INVALID_STORAGE_RETURN | function 'getAddressSlot' return value 'AddressSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBooleanSlot' return value 'BooleanSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytes32Slot' return value 'Bytes32Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getUint256Slot' return value 'Uint256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getInt256Slot' return value 'Int256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | VALIDATION_WARNING | 'using X for *' is supported; all library functions are available but type-specific filtering is not enforced at compile time |
| warning | INVALID_STORAGE_RETURN | function 'getAddressSlot' return value 'AddressSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBooleanSlot' return value 'BooleanSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytes32Slot' return value 'Bytes32Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getUint256Slot' return value 'Uint256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getInt256Slot' return value 'Int256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
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
| warning | W121 | duplicate constant state variable 'FALLBACK_SENTINEL' detected while merging libraries |
| warning | W122 | duplicate state variable '_name' detected while flattening/merging contracts |
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
| warning | W121 | duplicate constant state variable 'FALLBACK_SENTINEL' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'FALLBACK_SENTINEL' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'FALLBACK_SENTINEL' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'getAddressSlot' return value 'AddressSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBooleanSlot' return value 'BooleanSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytes32Slot' return value 'Bytes32Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getUint256Slot' return value 'Uint256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getInt256Slot' return value 'Int256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getAddressSlot' return value 'AddressSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getBooleanSlot' return value 'BooleanSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getBytes32Slot' return value 'Bytes32Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getUint256Slot' return value 'Uint256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getInt256Slot' return value 'Int256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | W101 | function 'transfer' has 2 parameters (ERC-20 pattern). NEP-17 requires 4 parameters: transfer(from, to, amount, data). The `from` address is verified via Runtime.checkWitness() and `data` (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W108 | ERC-2612 permit pattern detected (7-parameter permit function). Neo N3 uses Runtime.checkWitness() for authorization; off-chain signature permits are not needed. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |
| warning | W200 | function 'tryRecover' in 'ERC20' overrides 'Context::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20' overrides 'Context::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC20' overrides 'Context::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20' overrides 'Context::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC20' overrides 'Context::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20' overrides 'Context::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'ERC20' overrides 'Context::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC20' overrides 'Context::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC20' overrides 'Context::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC20' overrides 'Context::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC20' overrides 'Context::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC20' overrides 'Context::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC20' overrides 'Context::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC20' overrides 'Context::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC20' overrides 'Context::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC20' overrides 'Context::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC20' overrides 'Context::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC20' overrides 'Context::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC20' overrides 'Context::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC20' overrides 'Context::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC20' overrides 'Context::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC20' overrides 'Context::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC20' overrides 'Context::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC20' overrides 'Context::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC20' overrides 'Context::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC20' overrides 'Context::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC20' overrides 'Context::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC20' overrides 'Context::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC20' overrides 'Context::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC20' overrides 'Context::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC20' overrides 'Context::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC20' overrides 'Context::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC20' overrides 'Context::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC20' overrides 'Context::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC20' overrides 'Context::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC20' overrides 'Context::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC20' overrides 'Context::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC20' overrides 'Context::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC20' overrides 'Context::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC20' overrides 'Context::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC20' overrides 'Context::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC20' overrides 'Context::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC20' overrides 'Context::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC20' overrides 'Context::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC20' overrides 'Context::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC20' overrides 'Context::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC20' overrides 'Context::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC20' overrides 'Context::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC20' overrides 'Context::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC20' overrides 'Context::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC20' overrides 'Context::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC20' overrides 'Context::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC20' overrides 'Context::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC20' overrides 'Context::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC20' overrides 'Context::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC20' overrides 'Context::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC20' overrides 'Context::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC20' overrides 'Context::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC20' overrides 'Context::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC20' overrides 'Context::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC20' overrides 'Context::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC20' overrides 'Context::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC20' overrides 'Context::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC20' overrides 'Context::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC20' overrides 'Context::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC20' overrides 'Context::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC20' overrides 'Context::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC20' overrides 'Context::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC20' overrides 'Context::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC20' overrides 'Context::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC20' overrides 'Context::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC20' overrides 'Context::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC20' overrides 'Context::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC20' overrides 'Context::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC20' overrides 'Context::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC20' overrides 'Context::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC20' overrides 'Context::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC20' overrides 'Context::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC20' overrides 'Context::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC20' overrides 'Context::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC20' overrides 'Context::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC20' overrides 'Context::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC20' overrides 'Context::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC20' overrides 'Context::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC20' overrides 'Context::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC20' overrides 'Context::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC20' overrides 'Context::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC20' overrides 'Context::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC20' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC20' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC20' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC20' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC20' overrides 'Context::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC20' overrides 'Context::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC20' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC20' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC20' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC20' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC20' overrides 'Context::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC20' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC20' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC20' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC20' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC20' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC20' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC20' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC20' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC20' overrides 'Context::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC20' overrides 'Context::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC20' overrides 'Context::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC20' overrides 'Context::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC20' overrides 'Context::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC20' overrides 'Context::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC20' overrides 'Context::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC20' overrides 'Context::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC20' overrides 'Context::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC20' overrides 'Context::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC20' overrides 'Context::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC20' overrides 'Context::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC20' overrides 'Context::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC20' overrides 'Context::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC20' overrides 'Context::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC20' overrides 'Context::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC20' overrides 'Context::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC20' overrides 'Context::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC20' overrides 'Context::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC20' overrides 'Context::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC20' overrides 'Context::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC20' overrides 'Context::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20' overrides 'Context::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20' overrides 'Context::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC20' overrides 'Context::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC20' overrides 'Context::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC20' overrides 'Context::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC20' overrides 'Context::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC20' overrides 'Context::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC20' overrides 'Context::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC20' overrides 'Context::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC20' overrides 'Context::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20' overrides 'Context::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20' overrides 'Context::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20' overrides 'Context::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20' overrides 'Context::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'ERC20' overrides 'Context::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'ERC20' overrides 'Context::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'ERC20' overrides 'Context::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'ERC20' overrides 'Context::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'ERC20' overrides 'Context::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'ERC20' overrides 'Context::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ERC20' overrides 'Context::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ERC20' overrides 'Context::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ERC20' overrides 'Context::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ERC20' overrides 'Context::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortString' in 'ERC20' overrides 'Context::toShortString' which is not marked 'virtual' |
| warning | W200 | function 'toShortString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC20' overrides 'Context::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLength' in 'ERC20' overrides 'Context::byteLength' which is not marked 'virtual' |
| warning | W200 | function 'byteLength' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortStringWithFallback' in 'ERC20' overrides 'Context::toShortStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toShortStringWithFallback' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringWithFallback' in 'ERC20' overrides 'Context::toStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toStringWithFallback' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLengthWithFallback' in 'ERC20' overrides 'Context::byteLengthWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'byteLengthWithFallback' in 'ERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712' overrides 'ERC20::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712' overrides 'ERC20::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712' overrides 'ERC20::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712' overrides 'ERC20::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712' overrides 'ERC20::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712' overrides 'ERC20::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'EIP712' overrides 'ERC20::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'EIP712' overrides 'ERC20::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'EIP712' overrides 'ERC20::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'EIP712' overrides 'ERC20::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'EIP712' overrides 'ERC20::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'EIP712' overrides 'ERC20::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'EIP712' overrides 'ERC20::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'EIP712' overrides 'ERC20::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'EIP712' overrides 'ERC20::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'EIP712' overrides 'ERC20::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'EIP712' overrides 'ERC20::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'EIP712' overrides 'ERC20::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'EIP712' overrides 'ERC20::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'EIP712' overrides 'ERC20::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'EIP712' overrides 'ERC20::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'EIP712' overrides 'ERC20::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'EIP712' overrides 'ERC20::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'EIP712' overrides 'ERC20::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'EIP712' overrides 'ERC20::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'EIP712' overrides 'ERC20::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'EIP712' overrides 'ERC20::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'EIP712' overrides 'ERC20::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'EIP712' overrides 'ERC20::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'EIP712' overrides 'ERC20::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'EIP712' overrides 'ERC20::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'EIP712' overrides 'ERC20::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'EIP712' overrides 'ERC20::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'EIP712' overrides 'ERC20::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'EIP712' overrides 'ERC20::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'EIP712' overrides 'ERC20::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'EIP712' overrides 'ERC20::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'EIP712' overrides 'ERC20::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'EIP712' overrides 'ERC20::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'EIP712' overrides 'ERC20::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'EIP712' overrides 'ERC20::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'EIP712' overrides 'ERC20::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'EIP712' overrides 'ERC20::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'EIP712' overrides 'ERC20::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'EIP712' overrides 'ERC20::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'EIP712' overrides 'ERC20::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'EIP712' overrides 'ERC20::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'EIP712' overrides 'ERC20::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'EIP712' overrides 'ERC20::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'EIP712' overrides 'ERC20::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'EIP712' overrides 'ERC20::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'EIP712' overrides 'ERC20::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'EIP712' overrides 'ERC20::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'EIP712' overrides 'ERC20::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'EIP712' overrides 'ERC20::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'EIP712' overrides 'ERC20::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'EIP712' overrides 'ERC20::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'EIP712' overrides 'ERC20::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'EIP712' overrides 'ERC20::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'EIP712' overrides 'ERC20::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'EIP712' overrides 'ERC20::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'EIP712' overrides 'ERC20::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'EIP712' overrides 'ERC20::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'EIP712' overrides 'ERC20::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'EIP712' overrides 'ERC20::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'EIP712' overrides 'ERC20::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'EIP712' overrides 'ERC20::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'EIP712' overrides 'ERC20::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'EIP712' overrides 'ERC20::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'EIP712' overrides 'ERC20::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'EIP712' overrides 'ERC20::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'EIP712' overrides 'ERC20::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'EIP712' overrides 'ERC20::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'EIP712' overrides 'ERC20::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'EIP712' overrides 'ERC20::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'EIP712' overrides 'ERC20::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'EIP712' overrides 'ERC20::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'EIP712' overrides 'ERC20::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'EIP712' overrides 'ERC20::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'EIP712' overrides 'ERC20::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'EIP712' overrides 'ERC20::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'EIP712' overrides 'ERC20::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'EIP712' overrides 'ERC20::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'EIP712' overrides 'ERC20::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'EIP712' overrides 'ERC20::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'EIP712' overrides 'ERC20::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'EIP712' overrides 'ERC20::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'EIP712' overrides 'ERC20::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712' overrides 'ERC20::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712' overrides 'ERC20::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'EIP712' overrides 'ERC20::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'EIP712' overrides 'ERC20::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'EIP712' overrides 'ERC20::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'EIP712' overrides 'ERC20::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'EIP712' overrides 'ERC20::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'EIP712' overrides 'ERC20::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'EIP712' overrides 'ERC20::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'EIP712' overrides 'ERC20::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'EIP712' overrides 'ERC20::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'EIP712' overrides 'ERC20::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'EIP712' overrides 'ERC20::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'EIP712' overrides 'ERC20::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'EIP712' overrides 'ERC20::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'EIP712' overrides 'ERC20::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'EIP712' overrides 'ERC20::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'EIP712' overrides 'ERC20::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'EIP712' overrides 'ERC20::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'EIP712' overrides 'ERC20::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'EIP712' overrides 'ERC20::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'EIP712' overrides 'ERC20::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'EIP712' overrides 'ERC20::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'EIP712' overrides 'ERC20::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'EIP712' overrides 'ERC20::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'EIP712' overrides 'ERC20::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'EIP712' overrides 'ERC20::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712' overrides 'ERC20::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712' overrides 'ERC20::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712' overrides 'ERC20::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'EIP712' overrides 'ERC20::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'EIP712' overrides 'ERC20::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'EIP712' overrides 'ERC20::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'EIP712' overrides 'ERC20::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'EIP712' overrides 'ERC20::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'EIP712' overrides 'ERC20::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'EIP712' overrides 'ERC20::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'EIP712' overrides 'ERC20::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'EIP712' overrides 'ERC20::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'EIP712' overrides 'ERC20::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'EIP712' overrides 'ERC20::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'EIP712' overrides 'ERC20::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'EIP712' overrides 'ERC20::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'EIP712' overrides 'ERC20::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712' overrides 'ERC20::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712' overrides 'ERC20::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'EIP712' overrides 'ERC20::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'EIP712' overrides 'ERC20::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'EIP712' overrides 'ERC20::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'EIP712' overrides 'ERC20::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'EIP712' overrides 'ERC20::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'EIP712' overrides 'ERC20::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'EIP712' overrides 'ERC20::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'EIP712' overrides 'ERC20::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712' overrides 'ERC20::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712' overrides 'ERC20::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712' overrides 'ERC20::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712' overrides 'ERC20::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'EIP712' overrides 'ERC20::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'EIP712' overrides 'ERC20::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'EIP712' overrides 'ERC20::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'EIP712' overrides 'ERC20::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'EIP712' overrides 'ERC20::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'EIP712' overrides 'ERC20::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'EIP712' overrides 'ERC20::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'EIP712' overrides 'ERC20::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'EIP712' overrides 'ERC20::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'EIP712' overrides 'ERC20::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortString' in 'EIP712' overrides 'ERC20::toShortString' which is not marked 'virtual' |
| warning | W200 | function 'toShortString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'EIP712' overrides 'ERC20::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLength' in 'EIP712' overrides 'ERC20::byteLength' which is not marked 'virtual' |
| warning | W200 | function 'byteLength' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortStringWithFallback' in 'EIP712' overrides 'ERC20::toShortStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toShortStringWithFallback' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringWithFallback' in 'EIP712' overrides 'ERC20::toStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toStringWithFallback' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLengthWithFallback' in 'EIP712' overrides 'ERC20::byteLengthWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'byteLengthWithFallback' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'Nonces' overrides 'EIP712::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'Nonces' overrides 'EIP712::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'Nonces' overrides 'EIP712::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'Nonces' overrides 'EIP712::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'Nonces' overrides 'EIP712::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'Nonces' overrides 'EIP712::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'Nonces' overrides 'EIP712::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'Nonces' overrides 'EIP712::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'Nonces' overrides 'EIP712::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'Nonces' overrides 'EIP712::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'Nonces' overrides 'EIP712::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Nonces' overrides 'EIP712::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'Nonces' overrides 'EIP712::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'Nonces' overrides 'EIP712::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'Nonces' overrides 'EIP712::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'Nonces' overrides 'EIP712::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'Nonces' overrides 'EIP712::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'Nonces' overrides 'EIP712::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'Nonces' overrides 'EIP712::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Nonces' overrides 'EIP712::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'Nonces' overrides 'EIP712::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'Nonces' overrides 'EIP712::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'Nonces' overrides 'EIP712::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Nonces' overrides 'EIP712::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'Nonces' overrides 'EIP712::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'Nonces' overrides 'EIP712::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'Nonces' overrides 'EIP712::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Nonces' overrides 'EIP712::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'Nonces' overrides 'EIP712::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'Nonces' overrides 'EIP712::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'Nonces' overrides 'EIP712::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Nonces' overrides 'EIP712::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'Nonces' overrides 'EIP712::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'Nonces' overrides 'EIP712::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'Nonces' overrides 'EIP712::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Nonces' overrides 'EIP712::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'Nonces' overrides 'EIP712::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Nonces' overrides 'EIP712::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Nonces' overrides 'EIP712::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Nonces' overrides 'EIP712::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'Nonces' overrides 'EIP712::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'Nonces' overrides 'EIP712::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'Nonces' overrides 'EIP712::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'Nonces' overrides 'EIP712::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'Nonces' overrides 'EIP712::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'Nonces' overrides 'EIP712::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'Nonces' overrides 'EIP712::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'Nonces' overrides 'EIP712::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'Nonces' overrides 'EIP712::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'Nonces' overrides 'EIP712::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'Nonces' overrides 'EIP712::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'Nonces' overrides 'EIP712::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'Nonces' overrides 'EIP712::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'Nonces' overrides 'EIP712::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'Nonces' overrides 'EIP712::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Nonces' overrides 'EIP712::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'Nonces' overrides 'EIP712::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'Nonces' overrides 'EIP712::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'Nonces' overrides 'EIP712::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'Nonces' overrides 'EIP712::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'Nonces' overrides 'EIP712::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'Nonces' overrides 'EIP712::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'Nonces' overrides 'EIP712::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Nonces' overrides 'EIP712::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'Nonces' overrides 'EIP712::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'Nonces' overrides 'EIP712::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'Nonces' overrides 'EIP712::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Nonces' overrides 'EIP712::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'Nonces' overrides 'EIP712::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Nonces' overrides 'EIP712::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Nonces' overrides 'EIP712::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Nonces' overrides 'EIP712::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'Nonces' overrides 'EIP712::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'Nonces' overrides 'EIP712::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'Nonces' overrides 'EIP712::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'Nonces' overrides 'EIP712::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'Nonces' overrides 'EIP712::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'Nonces' overrides 'EIP712::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'Nonces' overrides 'EIP712::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'Nonces' overrides 'EIP712::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'Nonces' overrides 'EIP712::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'Nonces' overrides 'EIP712::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'Nonces' overrides 'EIP712::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'Nonces' overrides 'EIP712::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'Nonces' overrides 'EIP712::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'Nonces' overrides 'EIP712::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'Nonces' overrides 'EIP712::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'Nonces' overrides 'EIP712::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Nonces' overrides 'EIP712::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Nonces' overrides 'EIP712::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'Nonces' overrides 'EIP712::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'Nonces' overrides 'EIP712::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'Nonces' overrides 'EIP712::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'Nonces' overrides 'EIP712::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'Nonces' overrides 'EIP712::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'Nonces' overrides 'EIP712::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'Nonces' overrides 'EIP712::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'Nonces' overrides 'EIP712::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'Nonces' overrides 'EIP712::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'Nonces' overrides 'EIP712::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'Nonces' overrides 'EIP712::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'Nonces' overrides 'EIP712::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'Nonces' overrides 'EIP712::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'Nonces' overrides 'EIP712::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'Nonces' overrides 'EIP712::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'Nonces' overrides 'EIP712::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'Nonces' overrides 'EIP712::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'Nonces' overrides 'EIP712::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'Nonces' overrides 'EIP712::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'Nonces' overrides 'EIP712::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'Nonces' overrides 'EIP712::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'Nonces' overrides 'EIP712::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'Nonces' overrides 'EIP712::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'Nonces' overrides 'EIP712::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'Nonces' overrides 'EIP712::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'Nonces' overrides 'EIP712::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'Nonces' overrides 'EIP712::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'Nonces' overrides 'EIP712::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'Nonces' overrides 'EIP712::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'Nonces' overrides 'EIP712::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'Nonces' overrides 'EIP712::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'Nonces' overrides 'EIP712::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'Nonces' overrides 'EIP712::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'Nonces' overrides 'EIP712::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'Nonces' overrides 'EIP712::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'Nonces' overrides 'EIP712::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'Nonces' overrides 'EIP712::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'Nonces' overrides 'EIP712::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'Nonces' overrides 'EIP712::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'Nonces' overrides 'EIP712::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'Nonces' overrides 'EIP712::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'Nonces' overrides 'EIP712::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'Nonces' overrides 'EIP712::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'Nonces' overrides 'EIP712::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'Nonces' overrides 'EIP712::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'Nonces' overrides 'EIP712::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'Nonces' overrides 'EIP712::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'Nonces' overrides 'EIP712::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'Nonces' overrides 'EIP712::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'Nonces' overrides 'EIP712::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'Nonces' overrides 'EIP712::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'Nonces' overrides 'EIP712::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'Nonces' overrides 'EIP712::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'Nonces' overrides 'EIP712::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'Nonces' overrides 'EIP712::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'Nonces' overrides 'EIP712::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'Nonces' overrides 'EIP712::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'Nonces' overrides 'EIP712::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'Nonces' overrides 'EIP712::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'Nonces' overrides 'EIP712::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'Nonces' overrides 'EIP712::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'Nonces' overrides 'EIP712::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'Nonces' overrides 'EIP712::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'Nonces' overrides 'EIP712::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'Nonces' overrides 'EIP712::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'Nonces' overrides 'EIP712::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortString' in 'Nonces' overrides 'EIP712::toShortString' which is not marked 'virtual' |
| warning | W200 | function 'toShortString' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'Nonces' overrides 'EIP712::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLength' in 'Nonces' overrides 'EIP712::byteLength' which is not marked 'virtual' |
| warning | W200 | function 'byteLength' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortStringWithFallback' in 'Nonces' overrides 'EIP712::toShortStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toShortStringWithFallback' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringWithFallback' in 'Nonces' overrides 'EIP712::toStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toStringWithFallback' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLengthWithFallback' in 'Nonces' overrides 'EIP712::byteLengthWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'byteLengthWithFallback' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC20Permit' overrides 'Nonces::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20Permit' overrides 'Nonces::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC20Permit' overrides 'Nonces::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20Permit' overrides 'Nonces::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC20Permit' overrides 'Nonces::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC20Permit' overrides 'Nonces::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'ERC20Permit' overrides 'Nonces::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC20Permit' overrides 'Nonces::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC20Permit' overrides 'Nonces::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC20Permit' overrides 'Nonces::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC20Permit' overrides 'Nonces::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC20Permit' overrides 'Nonces::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC20Permit' overrides 'Nonces::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC20Permit' overrides 'Nonces::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC20Permit' overrides 'Nonces::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC20Permit' overrides 'Nonces::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC20Permit' overrides 'Nonces::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC20Permit' overrides 'Nonces::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC20Permit' overrides 'Nonces::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC20Permit' overrides 'Nonces::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC20Permit' overrides 'Nonces::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC20Permit' overrides 'Nonces::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC20Permit' overrides 'Nonces::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC20Permit' overrides 'Nonces::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC20Permit' overrides 'Nonces::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC20Permit' overrides 'Nonces::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC20Permit' overrides 'Nonces::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC20Permit' overrides 'Nonces::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC20Permit' overrides 'Nonces::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC20Permit' overrides 'Nonces::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC20Permit' overrides 'Nonces::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC20Permit' overrides 'Nonces::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC20Permit' overrides 'Nonces::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC20Permit' overrides 'Nonces::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC20Permit' overrides 'Nonces::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC20Permit' overrides 'Nonces::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC20Permit' overrides 'Nonces::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC20Permit' overrides 'Nonces::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC20Permit' overrides 'Nonces::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC20Permit' overrides 'Nonces::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC20Permit' overrides 'Nonces::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC20Permit' overrides 'Nonces::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC20Permit' overrides 'Nonces::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC20Permit' overrides 'Nonces::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC20Permit' overrides 'Nonces::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC20Permit' overrides 'Nonces::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC20Permit' overrides 'Nonces::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC20Permit' overrides 'Nonces::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC20Permit' overrides 'Nonces::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC20Permit' overrides 'Nonces::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC20Permit' overrides 'Nonces::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC20Permit' overrides 'Nonces::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC20Permit' overrides 'Nonces::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC20Permit' overrides 'Nonces::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC20Permit' overrides 'Nonces::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC20Permit' overrides 'Nonces::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC20Permit' overrides 'Nonces::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC20Permit' overrides 'Nonces::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC20Permit' overrides 'Nonces::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC20Permit' overrides 'Nonces::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC20Permit' overrides 'Nonces::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC20Permit' overrides 'Nonces::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC20Permit' overrides 'Nonces::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC20Permit' overrides 'Nonces::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC20Permit' overrides 'Nonces::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC20Permit' overrides 'Nonces::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC20Permit' overrides 'Nonces::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC20Permit' overrides 'Nonces::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC20Permit' overrides 'Nonces::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC20Permit' overrides 'Nonces::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC20Permit' overrides 'Nonces::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC20Permit' overrides 'Nonces::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC20Permit' overrides 'Nonces::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC20Permit' overrides 'Nonces::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC20Permit' overrides 'Nonces::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC20Permit' overrides 'Nonces::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC20Permit' overrides 'Nonces::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC20Permit' overrides 'Nonces::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC20Permit' overrides 'Nonces::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC20Permit' overrides 'Nonces::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC20Permit' overrides 'Nonces::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC20Permit' overrides 'Nonces::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC20Permit' overrides 'Nonces::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC20Permit' overrides 'Nonces::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC20Permit' overrides 'Nonces::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC20Permit' overrides 'Nonces::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC20Permit' overrides 'Nonces::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC20Permit' overrides 'Nonces::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC20Permit' overrides 'Nonces::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC20Permit' overrides 'Nonces::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC20Permit' overrides 'Nonces::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC20Permit' overrides 'Nonces::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC20Permit' overrides 'Nonces::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC20Permit' overrides 'Nonces::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC20Permit' overrides 'Nonces::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC20Permit' overrides 'Nonces::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC20Permit' overrides 'Nonces::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC20Permit' overrides 'Nonces::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC20Permit' overrides 'Nonces::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC20Permit' overrides 'Nonces::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC20Permit' overrides 'Nonces::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC20Permit' overrides 'Nonces::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC20Permit' overrides 'Nonces::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC20Permit' overrides 'Nonces::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC20Permit' overrides 'Nonces::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC20Permit' overrides 'Nonces::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC20Permit' overrides 'Nonces::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC20Permit' overrides 'Nonces::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC20Permit' overrides 'Nonces::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC20Permit' overrides 'Nonces::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC20Permit' overrides 'Nonces::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC20Permit' overrides 'Nonces::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC20Permit' overrides 'Nonces::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC20Permit' overrides 'Nonces::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC20Permit' overrides 'Nonces::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20Permit' overrides 'Nonces::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20Permit' overrides 'Nonces::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC20Permit' overrides 'Nonces::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC20Permit' overrides 'Nonces::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC20Permit' overrides 'Nonces::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC20Permit' overrides 'Nonces::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC20Permit' overrides 'Nonces::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC20Permit' overrides 'Nonces::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC20Permit' overrides 'Nonces::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC20Permit' overrides 'Nonces::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC20Permit' overrides 'Nonces::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC20Permit' overrides 'Nonces::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC20Permit' overrides 'Nonces::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC20Permit' overrides 'Nonces::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC20Permit' overrides 'Nonces::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC20Permit' overrides 'Nonces::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC20Permit' overrides 'Nonces::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20Permit' overrides 'Nonces::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20Permit' overrides 'Nonces::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC20Permit' overrides 'Nonces::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC20Permit' overrides 'Nonces::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC20Permit' overrides 'Nonces::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC20Permit' overrides 'Nonces::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC20Permit' overrides 'Nonces::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC20Permit' overrides 'Nonces::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC20Permit' overrides 'Nonces::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC20Permit' overrides 'Nonces::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20Permit' overrides 'Nonces::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20Permit' overrides 'Nonces::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20Permit' overrides 'Nonces::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20Permit' overrides 'Nonces::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'ERC20Permit' overrides 'Nonces::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'ERC20Permit' overrides 'Nonces::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'ERC20Permit' overrides 'Nonces::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'ERC20Permit' overrides 'Nonces::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'ERC20Permit' overrides 'Nonces::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'ERC20Permit' overrides 'Nonces::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ERC20Permit' overrides 'Nonces::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ERC20Permit' overrides 'Nonces::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ERC20Permit' overrides 'Nonces::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ERC20Permit' overrides 'Nonces::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortString' in 'ERC20Permit' overrides 'Nonces::toShortString' which is not marked 'virtual' |
| warning | W200 | function 'toShortString' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC20Permit' overrides 'Nonces::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLength' in 'ERC20Permit' overrides 'Nonces::byteLength' which is not marked 'virtual' |
| warning | W200 | function 'byteLength' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortStringWithFallback' in 'ERC20Permit' overrides 'Nonces::toShortStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toShortStringWithFallback' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringWithFallback' in 'ERC20Permit' overrides 'Nonces::toStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toStringWithFallback' in 'ERC20Permit' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLengthWithFallback' in 'ERC20Permit' overrides 'Nonces::byteLengthWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'byteLengthWithFallback' in 'ERC20Permit' overrides a base function but is not marked 'override' |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol`