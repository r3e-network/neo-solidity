# ProxyAdmin (OpenZeppelin)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol`
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
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | INVALID_STORAGE_RETURN | function 'getAddressSlot' return value 'AddressSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBooleanSlot' return value 'BooleanSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytes32Slot' return value 'Bytes32Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getUint256Slot' return value 'Uint256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getInt256Slot' return value 'Int256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | VALIDATION_WARNING | abstract contract 'Proxy' has 1 unimplemented function(s): [_implementation] |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Proxy' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'IMPLEMENTATION_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADMIN_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEACON_SLOT' detected while merging libraries |
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
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'sendValue' in 'ERC1967Proxy' overrides 'Proxy::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ERC1967Proxy' overrides 'Proxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ERC1967Proxy' overrides 'Proxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ERC1967Proxy' overrides 'Proxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ERC1967Proxy' overrides 'Proxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC1967Proxy' overrides 'Proxy::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ERC1967Proxy' overrides 'Proxy::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ERC1967Proxy' overrides 'Proxy::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'ERC1967Proxy' overrides 'Proxy::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'ERC1967Proxy' overrides 'Proxy::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'ERC1967Proxy' overrides 'Proxy::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'ERC1967Proxy' overrides 'Proxy::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'ERC1967Proxy' overrides 'Proxy::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ERC1967Proxy' overrides 'Proxy::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ERC1967Proxy' overrides 'Proxy::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ERC1967Proxy' overrides 'Proxy::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ERC1967Proxy' overrides 'Proxy::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getImplementation' in 'ERC1967Proxy' overrides 'Proxy::getImplementation' which is not marked 'virtual' |
| warning | W200 | function 'getImplementation' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setImplementation' in 'ERC1967Proxy' overrides 'Proxy::_setImplementation' which is not marked 'virtual' |
| warning | W200 | function '_setImplementation' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeToAndCall' in 'ERC1967Proxy' overrides 'Proxy::upgradeToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeToAndCall' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAdmin' in 'ERC1967Proxy' overrides 'Proxy::getAdmin' which is not marked 'virtual' |
| warning | W200 | function 'getAdmin' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setAdmin' in 'ERC1967Proxy' overrides 'Proxy::_setAdmin' which is not marked 'virtual' |
| warning | W200 | function '_setAdmin' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'changeAdmin' in 'ERC1967Proxy' overrides 'Proxy::changeAdmin' which is not marked 'virtual' |
| warning | W200 | function 'changeAdmin' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBeacon' in 'ERC1967Proxy' overrides 'Proxy::getBeacon' which is not marked 'virtual' |
| warning | W200 | function 'getBeacon' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setBeacon' in 'ERC1967Proxy' overrides 'Proxy::_setBeacon' which is not marked 'virtual' |
| warning | W200 | function '_setBeacon' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'ERC1967Proxy' overrides 'Proxy::upgradeBeaconToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_checkNonPayable' in 'ERC1967Proxy' overrides 'Proxy::_checkNonPayable' which is not marked 'virtual' |
| warning | W200 | function '_checkNonPayable' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ERC1967Proxy' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'IMPLEMENTATION_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADMIN_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEACON_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IMPLEMENTATION_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADMIN_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEACON_SLOT' detected while merging libraries |
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
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'sendValue' in 'ERC1967Proxy' overrides 'Proxy::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ERC1967Proxy' overrides 'Proxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ERC1967Proxy' overrides 'Proxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ERC1967Proxy' overrides 'Proxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ERC1967Proxy' overrides 'Proxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC1967Proxy' overrides 'Proxy::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ERC1967Proxy' overrides 'Proxy::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ERC1967Proxy' overrides 'Proxy::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'ERC1967Proxy' overrides 'Proxy::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'ERC1967Proxy' overrides 'Proxy::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'ERC1967Proxy' overrides 'Proxy::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'ERC1967Proxy' overrides 'Proxy::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'ERC1967Proxy' overrides 'Proxy::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ERC1967Proxy' overrides 'Proxy::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ERC1967Proxy' overrides 'Proxy::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ERC1967Proxy' overrides 'Proxy::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ERC1967Proxy' overrides 'Proxy::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getImplementation' in 'ERC1967Proxy' overrides 'Proxy::getImplementation' which is not marked 'virtual' |
| warning | W200 | function 'getImplementation' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setImplementation' in 'ERC1967Proxy' overrides 'Proxy::_setImplementation' which is not marked 'virtual' |
| warning | W200 | function '_setImplementation' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeToAndCall' in 'ERC1967Proxy' overrides 'Proxy::upgradeToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeToAndCall' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAdmin' in 'ERC1967Proxy' overrides 'Proxy::getAdmin' which is not marked 'virtual' |
| warning | W200 | function 'getAdmin' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setAdmin' in 'ERC1967Proxy' overrides 'Proxy::_setAdmin' which is not marked 'virtual' |
| warning | W200 | function '_setAdmin' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'changeAdmin' in 'ERC1967Proxy' overrides 'Proxy::changeAdmin' which is not marked 'virtual' |
| warning | W200 | function 'changeAdmin' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBeacon' in 'ERC1967Proxy' overrides 'Proxy::getBeacon' which is not marked 'virtual' |
| warning | W200 | function 'getBeacon' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setBeacon' in 'ERC1967Proxy' overrides 'Proxy::_setBeacon' which is not marked 'virtual' |
| warning | W200 | function '_setBeacon' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'ERC1967Proxy' overrides 'Proxy::upgradeBeaconToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_checkNonPayable' in 'ERC1967Proxy' overrides 'Proxy::_checkNonPayable' which is not marked 'virtual' |
| warning | W200 | function '_checkNonPayable' in 'ERC1967Proxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getImplementation' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::getImplementation' which is not marked 'virtual' |
| warning | W200 | function 'getImplementation' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setImplementation' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::_setImplementation' which is not marked 'virtual' |
| warning | W200 | function '_setImplementation' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeToAndCall' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::upgradeToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeToAndCall' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAdmin' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::getAdmin' which is not marked 'virtual' |
| warning | W200 | function 'getAdmin' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setAdmin' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::_setAdmin' which is not marked 'virtual' |
| warning | W200 | function '_setAdmin' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'changeAdmin' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::changeAdmin' which is not marked 'virtual' |
| warning | W200 | function 'changeAdmin' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBeacon' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::getBeacon' which is not marked 'virtual' |
| warning | W200 | function 'getBeacon' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setBeacon' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::_setBeacon' which is not marked 'virtual' |
| warning | W200 | function '_setBeacon' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::upgradeBeaconToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_checkNonPayable' in 'TransparentUpgradeableProxy' overrides 'ERC1967Proxy::_checkNonPayable' which is not marked 'virtual' |
| warning | W200 | function '_checkNonPayable' in 'TransparentUpgradeableProxy' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'TransparentUpgradeableProxy' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
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
| warning | W121 | duplicate constant state variable 'IMPLEMENTATION_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADMIN_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEACON_SLOT' detected while merging libraries |
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
| warning | W200 | function 'sendValue' in 'Ownable' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Ownable' overrides 'Context::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Ownable' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Ownable' overrides 'Context::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'Ownable' overrides 'Context::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'Ownable' overrides 'Context::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'Ownable' overrides 'Context::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'Ownable' overrides 'Context::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'Ownable' overrides 'Context::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'Ownable' overrides 'Context::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'Ownable' overrides 'Context::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'Ownable' overrides 'Context::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'Ownable' overrides 'Context::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getImplementation' in 'Ownable' overrides 'Context::getImplementation' which is not marked 'virtual' |
| warning | W200 | function 'getImplementation' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setImplementation' in 'Ownable' overrides 'Context::_setImplementation' which is not marked 'virtual' |
| warning | W200 | function '_setImplementation' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeToAndCall' in 'Ownable' overrides 'Context::upgradeToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeToAndCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAdmin' in 'Ownable' overrides 'Context::getAdmin' which is not marked 'virtual' |
| warning | W200 | function 'getAdmin' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setAdmin' in 'Ownable' overrides 'Context::_setAdmin' which is not marked 'virtual' |
| warning | W200 | function '_setAdmin' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'changeAdmin' in 'Ownable' overrides 'Context::changeAdmin' which is not marked 'virtual' |
| warning | W200 | function 'changeAdmin' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBeacon' in 'Ownable' overrides 'Context::getBeacon' which is not marked 'virtual' |
| warning | W200 | function 'getBeacon' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setBeacon' in 'Ownable' overrides 'Context::_setBeacon' which is not marked 'virtual' |
| warning | W200 | function '_setBeacon' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'Ownable' overrides 'Context::upgradeBeaconToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_checkNonPayable' in 'Ownable' overrides 'Context::_checkNonPayable' which is not marked 'virtual' |
| warning | W200 | function '_checkNonPayable' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Ownable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'upgradeAndCall' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'IMPLEMENTATION_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADMIN_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEACON_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IMPLEMENTATION_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADMIN_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEACON_SLOT' detected while merging libraries |
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
| warning | W116 | function 'upgradeAndCall' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'sendValue' in 'Ownable' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Ownable' overrides 'Context::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Ownable' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Ownable' overrides 'Context::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'Ownable' overrides 'Context::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'Ownable' overrides 'Context::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'Ownable' overrides 'Context::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'Ownable' overrides 'Context::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'Ownable' overrides 'Context::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'Ownable' overrides 'Context::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'Ownable' overrides 'Context::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'Ownable' overrides 'Context::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'Ownable' overrides 'Context::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getImplementation' in 'Ownable' overrides 'Context::getImplementation' which is not marked 'virtual' |
| warning | W200 | function 'getImplementation' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setImplementation' in 'Ownable' overrides 'Context::_setImplementation' which is not marked 'virtual' |
| warning | W200 | function '_setImplementation' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeToAndCall' in 'Ownable' overrides 'Context::upgradeToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeToAndCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAdmin' in 'Ownable' overrides 'Context::getAdmin' which is not marked 'virtual' |
| warning | W200 | function 'getAdmin' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setAdmin' in 'Ownable' overrides 'Context::_setAdmin' which is not marked 'virtual' |
| warning | W200 | function '_setAdmin' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'changeAdmin' in 'Ownable' overrides 'Context::changeAdmin' which is not marked 'virtual' |
| warning | W200 | function 'changeAdmin' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBeacon' in 'Ownable' overrides 'Context::getBeacon' which is not marked 'virtual' |
| warning | W200 | function 'getBeacon' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setBeacon' in 'Ownable' overrides 'Context::_setBeacon' which is not marked 'virtual' |
| warning | W200 | function '_setBeacon' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'Ownable' overrides 'Context::upgradeBeaconToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_checkNonPayable' in 'Ownable' overrides 'Context::_checkNonPayable' which is not marked 'virtual' |
| warning | W200 | function '_checkNonPayable' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'ProxyAdmin' overrides 'Ownable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ProxyAdmin' overrides 'Ownable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ProxyAdmin' overrides 'Ownable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ProxyAdmin' overrides 'Ownable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ProxyAdmin' overrides 'Ownable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ProxyAdmin' overrides 'Ownable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ProxyAdmin' overrides 'Ownable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ProxyAdmin' overrides 'Ownable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'ProxyAdmin' overrides 'Ownable::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'ProxyAdmin' overrides 'Ownable::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'ProxyAdmin' overrides 'Ownable::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'ProxyAdmin' overrides 'Ownable::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'ProxyAdmin' overrides 'Ownable::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ProxyAdmin' overrides 'Ownable::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'ProxyAdmin' overrides 'Ownable::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ProxyAdmin' overrides 'Ownable::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'ProxyAdmin' overrides 'Ownable::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getImplementation' in 'ProxyAdmin' overrides 'Ownable::getImplementation' which is not marked 'virtual' |
| warning | W200 | function 'getImplementation' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setImplementation' in 'ProxyAdmin' overrides 'Ownable::_setImplementation' which is not marked 'virtual' |
| warning | W200 | function '_setImplementation' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeToAndCall' in 'ProxyAdmin' overrides 'Ownable::upgradeToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeToAndCall' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAdmin' in 'ProxyAdmin' overrides 'Ownable::getAdmin' which is not marked 'virtual' |
| warning | W200 | function 'getAdmin' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setAdmin' in 'ProxyAdmin' overrides 'Ownable::_setAdmin' which is not marked 'virtual' |
| warning | W200 | function '_setAdmin' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'changeAdmin' in 'ProxyAdmin' overrides 'Ownable::changeAdmin' which is not marked 'virtual' |
| warning | W200 | function 'changeAdmin' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBeacon' in 'ProxyAdmin' overrides 'Ownable::getBeacon' which is not marked 'virtual' |
| warning | W200 | function 'getBeacon' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setBeacon' in 'ProxyAdmin' overrides 'Ownable::_setBeacon' which is not marked 'virtual' |
| warning | W200 | function '_setBeacon' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'ProxyAdmin' overrides 'Ownable::upgradeBeaconToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | W200 | function '_checkNonPayable' in 'ProxyAdmin' overrides 'Ownable::_checkNonPayable' which is not marked 'virtual' |
| warning | W200 | function '_checkNonPayable' in 'ProxyAdmin' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ProxyAdmin' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol`