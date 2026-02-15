# Governor (OpenZeppelin)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@openzeppelin/contracts/governance/Governor.sol`
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
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'EIP712' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | INVALID_STORAGE_RETURN | function 'getAddressSlot' return value 'AddressSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBooleanSlot' return value 'BooleanSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytes32Slot' return value 'Bytes32Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getUint256Slot' return value 'Uint256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getInt256Slot' return value 'Int256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ERC165' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | INVALID_STORAGE_RETURN | function 'getAddressSlot' return value 'AddressSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBooleanSlot' return value 'BooleanSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytes32Slot' return value 'Bytes32Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getUint256Slot' return value 'Uint256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getInt256Slot' return value 'Int256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Context' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | INVALID_STORAGE_RETURN | function 'getAddressSlot' return value 'AddressSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBooleanSlot' return value 'BooleanSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytes32Slot' return value 'Bytes32Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getUint256Slot' return value 'Uint256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getInt256Slot' return value 'Int256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Nonces' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
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
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W116 | function 'execute' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'relay' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W117 | function 'proposalDeadline' appears to be time-sensitive. block.timestamp on Neo N3 is deterministic but can be affected by block production timing. |
| warning | VALIDATION_WARNING | abstract contract 'Governor' has 9 unimplemented function(s): [_quorumReached, _voteSucceeded, _getVotes, _countVote, clock, CLOCK_MODE, votingDelay, votingPeriod, quorum] |
| warning | W200 | function 'panic' in 'ERC165' overrides 'Context::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC165' overrides 'Context::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC165' overrides 'Context::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC165' overrides 'Context::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC165' overrides 'Context::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC165' overrides 'Context::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC165' overrides 'Context::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC165' overrides 'Context::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC165' overrides 'Context::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC165' overrides 'Context::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC165' overrides 'Context::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC165' overrides 'Context::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC165' overrides 'Context::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC165' overrides 'Context::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC165' overrides 'Context::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC165' overrides 'Context::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC165' overrides 'Context::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC165' overrides 'Context::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC165' overrides 'Context::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC165' overrides 'Context::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC165' overrides 'Context::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC165' overrides 'Context::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC165' overrides 'Context::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC165' overrides 'Context::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC165' overrides 'Context::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC165' overrides 'Context::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC165' overrides 'Context::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC165' overrides 'Context::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC165' overrides 'Context::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC165' overrides 'Context::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC165' overrides 'Context::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC165' overrides 'Context::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC165' overrides 'Context::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC165' overrides 'Context::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC165' overrides 'Context::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC165' overrides 'Context::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC165' overrides 'Context::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC165' overrides 'Context::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC165' overrides 'Context::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC165' overrides 'Context::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC165' overrides 'Context::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC165' overrides 'Context::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC165' overrides 'Context::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC165' overrides 'Context::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC165' overrides 'Context::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC165' overrides 'Context::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC165' overrides 'Context::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC165' overrides 'Context::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC165' overrides 'Context::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC165' overrides 'Context::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC165' overrides 'Context::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC165' overrides 'Context::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC165' overrides 'Context::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC165' overrides 'Context::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC165' overrides 'Context::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC165' overrides 'Context::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC165' overrides 'Context::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC165' overrides 'Context::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC165' overrides 'Context::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC165' overrides 'Context::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC165' overrides 'Context::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC165' overrides 'Context::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC165' overrides 'Context::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC165' overrides 'Context::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC165' overrides 'Context::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC165' overrides 'Context::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC165' overrides 'Context::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC165' overrides 'Context::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC165' overrides 'Context::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC165' overrides 'Context::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC165' overrides 'Context::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC165' overrides 'Context::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC165' overrides 'Context::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC165' overrides 'Context::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC165' overrides 'Context::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC165' overrides 'Context::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC165' overrides 'Context::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC165' overrides 'Context::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC165' overrides 'Context::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC165' overrides 'Context::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC165' overrides 'Context::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC165' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC165' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC165' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC165' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC165' overrides 'Context::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC165' overrides 'Context::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC165' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC165' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC165' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC165' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC165' overrides 'Context::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC165' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC165' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC165' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC165' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC165' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC165' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC165' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC165' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC165' overrides 'Context::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC165' overrides 'Context::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC165' overrides 'Context::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC165' overrides 'Context::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC165' overrides 'Context::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC165' overrides 'Context::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC165' overrides 'Context::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC165' overrides 'Context::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC165' overrides 'Context::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC165' overrides 'Context::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC165' overrides 'Context::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC165' overrides 'Context::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC165' overrides 'Context::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC165' overrides 'Context::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC165' overrides 'Context::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC165' overrides 'Context::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC165' overrides 'Context::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC165' overrides 'Context::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC165' overrides 'Context::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC165' overrides 'Context::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC165' overrides 'Context::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC165' overrides 'Context::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165' overrides 'Context::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165' overrides 'Context::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC165' overrides 'Context::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC165' overrides 'Context::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC165' overrides 'Context::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC165' overrides 'Context::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC165' overrides 'Context::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC165' overrides 'Context::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC165' overrides 'Context::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC165' overrides 'Context::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165' overrides 'Context::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165' overrides 'Context::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165' overrides 'Context::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165' overrides 'Context::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'ERC165' overrides 'Context::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'ERC165' overrides 'Context::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'ERC165' overrides 'Context::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'ERC165' overrides 'Context::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'ERC165' overrides 'Context::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'ERC165' overrides 'Context::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ERC165' overrides 'Context::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ERC165' overrides 'Context::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ERC165' overrides 'Context::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ERC165' overrides 'Context::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortString' in 'ERC165' overrides 'Context::toShortString' which is not marked 'virtual' |
| warning | W200 | function 'toShortString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC165' overrides 'Context::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLength' in 'ERC165' overrides 'Context::byteLength' which is not marked 'virtual' |
| warning | W200 | function 'byteLength' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortStringWithFallback' in 'ERC165' overrides 'Context::toShortStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toShortStringWithFallback' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringWithFallback' in 'ERC165' overrides 'Context::toStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toStringWithFallback' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLengthWithFallback' in 'ERC165' overrides 'Context::byteLengthWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'byteLengthWithFallback' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC165' overrides 'Context::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC165' overrides 'Context::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC165' overrides 'Context::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC165' overrides 'Context::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'ERC165' overrides 'Context::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'ERC165' overrides 'Context::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'ERC165' overrides 'Context::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ERC165' overrides 'Context::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'ERC165' overrides 'Context::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ERC165' overrides 'Context::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'ERC165' overrides 'Context::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ERC165' overrides 'Context::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'ERC165' overrides 'Context::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165' overrides 'Context::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ERC165' overrides 'Context::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165' overrides 'Context::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'ERC165' overrides 'Context::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'ERC165' overrides 'Context::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'ERC165' overrides 'Context::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'ERC165' overrides 'Context::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'ERC165' overrides 'Context::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'ERC165' overrides 'Context::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'ERC165' overrides 'Context::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'ERC165' overrides 'Context::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'ERC165' overrides 'Context::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'ERC165' overrides 'Context::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'ERC165' overrides 'Context::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'ERC165' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ERC165' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ERC165' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ERC165' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ERC165' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC165' overrides 'Context::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ERC165' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ERC165' overrides 'Context::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'EIP712' overrides 'ERC165::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'EIP712' overrides 'ERC165::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'EIP712' overrides 'ERC165::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'EIP712' overrides 'ERC165::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'EIP712' overrides 'ERC165::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'EIP712' overrides 'ERC165::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'EIP712' overrides 'ERC165::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'EIP712' overrides 'ERC165::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'EIP712' overrides 'ERC165::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'EIP712' overrides 'ERC165::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'EIP712' overrides 'ERC165::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'EIP712' overrides 'ERC165::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'EIP712' overrides 'ERC165::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'EIP712' overrides 'ERC165::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'EIP712' overrides 'ERC165::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'EIP712' overrides 'ERC165::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'EIP712' overrides 'ERC165::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'EIP712' overrides 'ERC165::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'EIP712' overrides 'ERC165::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'EIP712' overrides 'ERC165::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'EIP712' overrides 'ERC165::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'EIP712' overrides 'ERC165::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'EIP712' overrides 'ERC165::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'EIP712' overrides 'ERC165::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'EIP712' overrides 'ERC165::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'EIP712' overrides 'ERC165::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'EIP712' overrides 'ERC165::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'EIP712' overrides 'ERC165::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'EIP712' overrides 'ERC165::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'EIP712' overrides 'ERC165::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'EIP712' overrides 'ERC165::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'EIP712' overrides 'ERC165::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'EIP712' overrides 'ERC165::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'EIP712' overrides 'ERC165::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'EIP712' overrides 'ERC165::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'EIP712' overrides 'ERC165::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'EIP712' overrides 'ERC165::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'EIP712' overrides 'ERC165::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'EIP712' overrides 'ERC165::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'EIP712' overrides 'ERC165::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'EIP712' overrides 'ERC165::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'EIP712' overrides 'ERC165::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'EIP712' overrides 'ERC165::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'EIP712' overrides 'ERC165::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'EIP712' overrides 'ERC165::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'EIP712' overrides 'ERC165::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'EIP712' overrides 'ERC165::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'EIP712' overrides 'ERC165::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'EIP712' overrides 'ERC165::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'EIP712' overrides 'ERC165::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'EIP712' overrides 'ERC165::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'EIP712' overrides 'ERC165::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'EIP712' overrides 'ERC165::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'EIP712' overrides 'ERC165::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'EIP712' overrides 'ERC165::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'EIP712' overrides 'ERC165::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'EIP712' overrides 'ERC165::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'EIP712' overrides 'ERC165::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'EIP712' overrides 'ERC165::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'EIP712' overrides 'ERC165::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'EIP712' overrides 'ERC165::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'EIP712' overrides 'ERC165::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'EIP712' overrides 'ERC165::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'EIP712' overrides 'ERC165::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'EIP712' overrides 'ERC165::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'EIP712' overrides 'ERC165::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'EIP712' overrides 'ERC165::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'EIP712' overrides 'ERC165::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'EIP712' overrides 'ERC165::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'EIP712' overrides 'ERC165::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'EIP712' overrides 'ERC165::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'EIP712' overrides 'ERC165::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'EIP712' overrides 'ERC165::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'EIP712' overrides 'ERC165::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'EIP712' overrides 'ERC165::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'EIP712' overrides 'ERC165::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'EIP712' overrides 'ERC165::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'EIP712' overrides 'ERC165::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'EIP712' overrides 'ERC165::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'EIP712' overrides 'ERC165::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'EIP712' overrides 'ERC165::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712' overrides 'ERC165::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712' overrides 'ERC165::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'EIP712' overrides 'ERC165::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'EIP712' overrides 'ERC165::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'EIP712' overrides 'ERC165::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'EIP712' overrides 'ERC165::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'EIP712' overrides 'ERC165::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'EIP712' overrides 'ERC165::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'EIP712' overrides 'ERC165::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'EIP712' overrides 'ERC165::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'EIP712' overrides 'ERC165::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'EIP712' overrides 'ERC165::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'EIP712' overrides 'ERC165::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'EIP712' overrides 'ERC165::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'EIP712' overrides 'ERC165::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'EIP712' overrides 'ERC165::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'EIP712' overrides 'ERC165::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'EIP712' overrides 'ERC165::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'EIP712' overrides 'ERC165::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'EIP712' overrides 'ERC165::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'EIP712' overrides 'ERC165::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'EIP712' overrides 'ERC165::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'EIP712' overrides 'ERC165::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'EIP712' overrides 'ERC165::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'EIP712' overrides 'ERC165::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'EIP712' overrides 'ERC165::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'EIP712' overrides 'ERC165::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712' overrides 'ERC165::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712' overrides 'ERC165::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'EIP712' overrides 'ERC165::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'EIP712' overrides 'ERC165::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'EIP712' overrides 'ERC165::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'EIP712' overrides 'ERC165::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'EIP712' overrides 'ERC165::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'EIP712' overrides 'ERC165::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'EIP712' overrides 'ERC165::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'EIP712' overrides 'ERC165::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'EIP712' overrides 'ERC165::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'EIP712' overrides 'ERC165::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'EIP712' overrides 'ERC165::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'EIP712' overrides 'ERC165::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'EIP712' overrides 'ERC165::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'EIP712' overrides 'ERC165::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'EIP712' overrides 'ERC165::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712' overrides 'ERC165::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712' overrides 'ERC165::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'EIP712' overrides 'ERC165::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'EIP712' overrides 'ERC165::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'EIP712' overrides 'ERC165::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'EIP712' overrides 'ERC165::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'EIP712' overrides 'ERC165::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'EIP712' overrides 'ERC165::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'EIP712' overrides 'ERC165::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'EIP712' overrides 'ERC165::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712' overrides 'ERC165::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712' overrides 'ERC165::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712' overrides 'ERC165::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712' overrides 'ERC165::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'EIP712' overrides 'ERC165::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'EIP712' overrides 'ERC165::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'EIP712' overrides 'ERC165::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'EIP712' overrides 'ERC165::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'EIP712' overrides 'ERC165::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'EIP712' overrides 'ERC165::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'EIP712' overrides 'ERC165::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'EIP712' overrides 'ERC165::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'EIP712' overrides 'ERC165::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'EIP712' overrides 'ERC165::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortString' in 'EIP712' overrides 'ERC165::toShortString' which is not marked 'virtual' |
| warning | W200 | function 'toShortString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'EIP712' overrides 'ERC165::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLength' in 'EIP712' overrides 'ERC165::byteLength' which is not marked 'virtual' |
| warning | W200 | function 'byteLength' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortStringWithFallback' in 'EIP712' overrides 'ERC165::toShortStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toShortStringWithFallback' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringWithFallback' in 'EIP712' overrides 'ERC165::toStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toStringWithFallback' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLengthWithFallback' in 'EIP712' overrides 'ERC165::byteLengthWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'byteLengthWithFallback' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712' overrides 'ERC165::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712' overrides 'ERC165::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712' overrides 'ERC165::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712' overrides 'ERC165::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'EIP712' overrides 'ERC165::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'EIP712' overrides 'ERC165::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'EIP712' overrides 'ERC165::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'EIP712' overrides 'ERC165::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'EIP712' overrides 'ERC165::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'EIP712' overrides 'ERC165::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'EIP712' overrides 'ERC165::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'EIP712' overrides 'ERC165::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'EIP712' overrides 'ERC165::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712' overrides 'ERC165::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'EIP712' overrides 'ERC165::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712' overrides 'ERC165::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'EIP712' overrides 'ERC165::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'EIP712' overrides 'ERC165::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'EIP712' overrides 'ERC165::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'EIP712' overrides 'ERC165::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'EIP712' overrides 'ERC165::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'EIP712' overrides 'ERC165::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'EIP712' overrides 'ERC165::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'EIP712' overrides 'ERC165::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'EIP712' overrides 'ERC165::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'EIP712' overrides 'ERC165::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'EIP712' overrides 'ERC165::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'EIP712' overrides 'ERC165::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'EIP712' overrides 'ERC165::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'EIP712' overrides 'ERC165::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'EIP712' overrides 'ERC165::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'EIP712' overrides 'ERC165::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'EIP712' overrides 'ERC165::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'EIP712' overrides 'ERC165::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'EIP712' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'EIP712' overrides 'ERC165::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'EIP712' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'indexOf' in 'Nonces' overrides 'EIP712::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'Nonces' overrides 'EIP712::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'Nonces' overrides 'EIP712::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'Nonces' overrides 'EIP712::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'Nonces' overrides 'EIP712::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'Nonces' overrides 'EIP712::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'Nonces' overrides 'EIP712::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'Nonces' overrides 'EIP712::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'Nonces' overrides 'EIP712::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'Nonces' overrides 'EIP712::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'Nonces' overrides 'EIP712::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'Nonces' overrides 'EIP712::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'Nonces' overrides 'EIP712::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'Nonces' overrides 'EIP712::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'Nonces' overrides 'EIP712::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'Nonces' overrides 'EIP712::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'Nonces' overrides 'EIP712::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'Nonces' overrides 'EIP712::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'Nonces' overrides 'EIP712::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'Nonces' overrides 'EIP712::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Nonces' overrides 'EIP712::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Nonces' overrides 'EIP712::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Nonces' overrides 'EIP712::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Nonces' overrides 'EIP712::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Nonces' overrides 'EIP712::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Nonces' overrides 'EIP712::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Nonces' overrides 'EIP712::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Nonces' overrides 'EIP712::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Nonces' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'Governor' overrides 'Nonces::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'Governor' overrides 'Nonces::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'Governor' overrides 'Nonces::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'Governor' overrides 'Nonces::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'Governor' overrides 'Nonces::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Governor' overrides 'Nonces::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'Governor' overrides 'Nonces::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'Governor' overrides 'Nonces::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'Governor' overrides 'Nonces::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'Governor' overrides 'Nonces::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'Governor' overrides 'Nonces::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'Governor' overrides 'Nonces::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'Governor' overrides 'Nonces::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Governor' overrides 'Nonces::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'Governor' overrides 'Nonces::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'Governor' overrides 'Nonces::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'Governor' overrides 'Nonces::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Governor' overrides 'Nonces::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'Governor' overrides 'Nonces::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'Governor' overrides 'Nonces::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'Governor' overrides 'Nonces::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Governor' overrides 'Nonces::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'Governor' overrides 'Nonces::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'Governor' overrides 'Nonces::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'Governor' overrides 'Nonces::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Governor' overrides 'Nonces::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'Governor' overrides 'Nonces::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'Governor' overrides 'Nonces::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'Governor' overrides 'Nonces::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Governor' overrides 'Nonces::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'Governor' overrides 'Nonces::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Governor' overrides 'Nonces::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Governor' overrides 'Nonces::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Governor' overrides 'Nonces::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'Governor' overrides 'Nonces::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'Governor' overrides 'Nonces::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'Governor' overrides 'Nonces::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'Governor' overrides 'Nonces::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'Governor' overrides 'Nonces::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'Governor' overrides 'Nonces::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'Governor' overrides 'Nonces::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'Governor' overrides 'Nonces::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'Governor' overrides 'Nonces::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'Governor' overrides 'Nonces::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'Governor' overrides 'Nonces::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'Governor' overrides 'Nonces::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'Governor' overrides 'Nonces::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'Governor' overrides 'Nonces::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'Governor' overrides 'Nonces::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Governor' overrides 'Nonces::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'Governor' overrides 'Nonces::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'Governor' overrides 'Nonces::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'Governor' overrides 'Nonces::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'Governor' overrides 'Nonces::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'Governor' overrides 'Nonces::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'Governor' overrides 'Nonces::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'Governor' overrides 'Nonces::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Governor' overrides 'Nonces::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'Governor' overrides 'Nonces::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'Governor' overrides 'Nonces::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'Governor' overrides 'Nonces::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Governor' overrides 'Nonces::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'Governor' overrides 'Nonces::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Governor' overrides 'Nonces::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Governor' overrides 'Nonces::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Governor' overrides 'Nonces::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'Governor' overrides 'Nonces::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'Governor' overrides 'Nonces::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'Governor' overrides 'Nonces::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'Governor' overrides 'Nonces::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'Governor' overrides 'Nonces::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'Governor' overrides 'Nonces::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'Governor' overrides 'Nonces::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'Governor' overrides 'Nonces::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'Governor' overrides 'Nonces::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'Governor' overrides 'Nonces::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'Governor' overrides 'Nonces::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'Governor' overrides 'Nonces::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'Governor' overrides 'Nonces::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'Governor' overrides 'Nonces::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'Governor' overrides 'Nonces::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'Governor' overrides 'Nonces::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Governor' overrides 'Nonces::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Governor' overrides 'Nonces::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'Governor' overrides 'Nonces::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'Governor' overrides 'Nonces::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'Governor' overrides 'Nonces::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'Governor' overrides 'Nonces::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'Governor' overrides 'Nonces::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'Governor' overrides 'Nonces::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'Governor' overrides 'Nonces::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'Governor' overrides 'Nonces::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'Governor' overrides 'Nonces::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'Governor' overrides 'Nonces::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'Governor' overrides 'Nonces::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'Governor' overrides 'Nonces::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'Governor' overrides 'Nonces::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'Governor' overrides 'Nonces::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'Governor' overrides 'Nonces::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'Governor' overrides 'Nonces::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'Governor' overrides 'Nonces::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'Governor' overrides 'Nonces::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'Governor' overrides 'Nonces::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'Governor' overrides 'Nonces::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'Governor' overrides 'Nonces::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'Governor' overrides 'Nonces::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'Governor' overrides 'Nonces::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'Governor' overrides 'Nonces::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'Governor' overrides 'Nonces::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'Governor' overrides 'Nonces::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'Governor' overrides 'Nonces::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'Governor' overrides 'Nonces::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'Governor' overrides 'Nonces::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'Governor' overrides 'Nonces::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'Governor' overrides 'Nonces::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'Governor' overrides 'Nonces::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'Governor' overrides 'Nonces::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'Governor' overrides 'Nonces::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'Governor' overrides 'Nonces::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'Governor' overrides 'Nonces::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'Governor' overrides 'Nonces::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'Governor' overrides 'Nonces::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'Governor' overrides 'Nonces::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'Governor' overrides 'Nonces::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'Governor' overrides 'Nonces::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'Governor' overrides 'Nonces::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'Governor' overrides 'Nonces::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'Governor' overrides 'Nonces::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'Governor' overrides 'Nonces::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'Governor' overrides 'Nonces::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'Governor' overrides 'Nonces::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'Governor' overrides 'Nonces::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'Governor' overrides 'Nonces::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'Governor' overrides 'Nonces::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'Governor' overrides 'Nonces::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'Governor' overrides 'Nonces::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toEthSignedMessageHash' in 'Governor' overrides 'Nonces::toEthSignedMessageHash' which is not marked 'virtual' |
| warning | W200 | function 'toEthSignedMessageHash' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'Governor' overrides 'Nonces::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'Governor' overrides 'Nonces::toDataWithIntendedValidatorHash' which is not marked 'virtual' |
| warning | W200 | function 'toDataWithIntendedValidatorHash' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toTypedDataHash' in 'Governor' overrides 'Nonces::toTypedDataHash' which is not marked 'virtual' |
| warning | W200 | function 'toTypedDataHash' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'Governor' overrides 'Nonces::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'Governor' overrides 'Nonces::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'Governor' overrides 'Nonces::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'Governor' overrides 'Nonces::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'Governor' overrides 'Nonces::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'Governor' overrides 'Nonces::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'Governor' overrides 'Nonces::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'Governor' overrides 'Nonces::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'Governor' overrides 'Nonces::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortString' in 'Governor' overrides 'Nonces::toShortString' which is not marked 'virtual' |
| warning | W200 | function 'toShortString' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'Governor' overrides 'Nonces::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLength' in 'Governor' overrides 'Nonces::byteLength' which is not marked 'virtual' |
| warning | W200 | function 'byteLength' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toShortStringWithFallback' in 'Governor' overrides 'Nonces::toShortStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toShortStringWithFallback' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringWithFallback' in 'Governor' overrides 'Nonces::toStringWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'toStringWithFallback' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'byteLengthWithFallback' in 'Governor' overrides 'Nonces::byteLengthWithFallback' which is not marked 'virtual' |
| warning | W200 | function 'byteLengthWithFallback' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'Governor' overrides 'Nonces::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'Governor' overrides 'Nonces::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'Governor' overrides 'Nonces::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'Governor' overrides 'Nonces::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryRecover' in 'Governor' overrides 'Nonces::tryRecover' which is not marked 'virtual' |
| warning | W200 | function 'tryRecover' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'recover' in 'Governor' overrides 'Nonces::recover' which is not marked 'virtual' |
| warning | W200 | function 'recover' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function '_throwError' in 'Governor' overrides 'Nonces::_throwError' which is not marked 'virtual' |
| warning | W200 | function '_throwError' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'Governor' overrides 'Nonces::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'indexOf' in 'Governor' overrides 'Nonces::indexOf' which is not marked 'virtual' |
| warning | W200 | function 'indexOf' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'Governor' overrides 'Nonces::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lastIndexOf' in 'Governor' overrides 'Nonces::lastIndexOf' which is not marked 'virtual' |
| warning | W200 | function 'lastIndexOf' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'Governor' overrides 'Nonces::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'slice' in 'Governor' overrides 'Nonces::slice' which is not marked 'virtual' |
| warning | W200 | function 'slice' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'Governor' overrides 'Nonces::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'Governor' overrides 'Nonces::isValidERC1271SignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidERC1271SignatureNow' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidSignatureNow' in 'Governor' overrides 'Nonces::isValidSignatureNow' which is not marked 'virtual' |
| warning | W200 | function 'isValidSignatureNow' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'areValidSignaturesNow' in 'Governor' overrides 'Nonces::areValidSignaturesNow' which is not marked 'virtual' |
| warning | W200 | function 'areValidSignaturesNow' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushBack' in 'Governor' overrides 'Nonces::pushBack' which is not marked 'virtual' |
| warning | W200 | function 'pushBack' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popBack' in 'Governor' overrides 'Nonces::popBack' which is not marked 'virtual' |
| warning | W200 | function 'popBack' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'pushFront' in 'Governor' overrides 'Nonces::pushFront' which is not marked 'virtual' |
| warning | W200 | function 'pushFront' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'popFront' in 'Governor' overrides 'Nonces::popFront' which is not marked 'virtual' |
| warning | W200 | function 'popFront' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'front' in 'Governor' overrides 'Nonces::front' which is not marked 'virtual' |
| warning | W200 | function 'front' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'back' in 'Governor' overrides 'Nonces::back' which is not marked 'virtual' |
| warning | W200 | function 'back' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'at' in 'Governor' overrides 'Nonces::at' which is not marked 'virtual' |
| warning | W200 | function 'at' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clear' in 'Governor' overrides 'Nonces::clear' which is not marked 'virtual' |
| warning | W200 | function 'clear' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'length' in 'Governor' overrides 'Nonces::length' which is not marked 'virtual' |
| warning | W200 | function 'length' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'empty' in 'Governor' overrides 'Nonces::empty' which is not marked 'virtual' |
| warning | W200 | function 'empty' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Governor' overrides 'Nonces::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Governor' overrides 'Nonces::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Governor' overrides 'Nonces::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Governor' overrides 'Nonces::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Governor' overrides 'Nonces::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Governor' overrides 'Nonces::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Governor' overrides 'Nonces::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Governor' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Governor' overrides 'Nonces::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Governor' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Governor' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@openzeppelin/contracts/governance/Governor.sol`