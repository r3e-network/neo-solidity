# UUPSUpgradeable (OpenZeppelin Upgradeable)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol`
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
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getAddressSlot' return value 'AddressSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBooleanSlot' return value 'BooleanSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytes32Slot' return value 'Bytes32Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getUint256Slot' return value 'Uint256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getInt256Slot' return value 'Int256Slot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getStringSlot' return value 'StringSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function 'getBytesSlot' return value 'BytesSlot' uses 'storage' data location (treated as Any) |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Initializable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'upgradeToAndCall' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'IMPLEMENTATION_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADMIN_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEACON_SLOT' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function '_getInitializableStorage' return value 'InitializableStorage' uses 'storage' data location (treated as Any) |
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
| warning | W116 | function 'upgradeToAndCall' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | VALIDATION_WARNING | abstract contract 'UUPSUpgradeable' has 1 unimplemented function(s): [_authorizeUpgrade] |
| warning | W200 | function 'upgradeToAndCall' in 'UUPSUpgradeable' overrides 'Initializable::upgradeToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeToAndCall' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'UUPSUpgradeable' overrides 'Initializable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'UUPSUpgradeable' overrides 'Initializable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'UUPSUpgradeable' overrides 'Initializable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'UUPSUpgradeable' overrides 'Initializable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'UUPSUpgradeable' overrides 'Initializable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'UUPSUpgradeable' overrides 'Initializable::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'UUPSUpgradeable' overrides 'Initializable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'UUPSUpgradeable' overrides 'Initializable::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'UUPSUpgradeable' overrides 'Initializable::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'UUPSUpgradeable' overrides 'Initializable::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'UUPSUpgradeable' overrides 'Initializable::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'UUPSUpgradeable' overrides 'Initializable::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'UUPSUpgradeable' overrides 'Initializable::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'UUPSUpgradeable' overrides 'Initializable::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'UUPSUpgradeable' overrides 'Initializable::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'UUPSUpgradeable' overrides 'Initializable::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'UUPSUpgradeable' overrides 'Initializable::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getImplementation' in 'UUPSUpgradeable' overrides 'Initializable::getImplementation' which is not marked 'virtual' |
| warning | W200 | function 'getImplementation' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setImplementation' in 'UUPSUpgradeable' overrides 'Initializable::_setImplementation' which is not marked 'virtual' |
| warning | W200 | function '_setImplementation' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAdmin' in 'UUPSUpgradeable' overrides 'Initializable::getAdmin' which is not marked 'virtual' |
| warning | W200 | function 'getAdmin' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setAdmin' in 'UUPSUpgradeable' overrides 'Initializable::_setAdmin' which is not marked 'virtual' |
| warning | W200 | function '_setAdmin' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'changeAdmin' in 'UUPSUpgradeable' overrides 'Initializable::changeAdmin' which is not marked 'virtual' |
| warning | W200 | function 'changeAdmin' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBeacon' in 'UUPSUpgradeable' overrides 'Initializable::getBeacon' which is not marked 'virtual' |
| warning | W200 | function 'getBeacon' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setBeacon' in 'UUPSUpgradeable' overrides 'Initializable::_setBeacon' which is not marked 'virtual' |
| warning | W200 | function '_setBeacon' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'UUPSUpgradeable' overrides 'Initializable::upgradeBeaconToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_checkNonPayable' in 'UUPSUpgradeable' overrides 'Initializable::_checkNonPayable' which is not marked 'virtual' |
| warning | W200 | function '_checkNonPayable' in 'UUPSUpgradeable' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'UUPSUpgradeable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol`