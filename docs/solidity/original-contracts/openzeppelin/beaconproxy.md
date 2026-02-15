# BeaconProxy (OpenZeppelin)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@openzeppelin/contracts/proxy/beacon/BeaconProxy.sol`
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
| warning | W200 | function 'sendValue' in 'BeaconProxy' overrides 'Proxy::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'BeaconProxy' overrides 'Proxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'BeaconProxy' overrides 'Proxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'BeaconProxy' overrides 'Proxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'BeaconProxy' overrides 'Proxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'BeaconProxy' overrides 'Proxy::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'BeaconProxy' overrides 'Proxy::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'BeaconProxy' overrides 'Proxy::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAddressSlot' in 'BeaconProxy' overrides 'Proxy::getAddressSlot' which is not marked 'virtual' |
| warning | W200 | function 'getAddressSlot' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBooleanSlot' in 'BeaconProxy' overrides 'Proxy::getBooleanSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBooleanSlot' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytes32Slot' in 'BeaconProxy' overrides 'Proxy::getBytes32Slot' which is not marked 'virtual' |
| warning | W200 | function 'getBytes32Slot' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUint256Slot' in 'BeaconProxy' overrides 'Proxy::getUint256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getUint256Slot' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInt256Slot' in 'BeaconProxy' overrides 'Proxy::getInt256Slot' which is not marked 'virtual' |
| warning | W200 | function 'getInt256Slot' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'BeaconProxy' overrides 'Proxy::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStringSlot' in 'BeaconProxy' overrides 'Proxy::getStringSlot' which is not marked 'virtual' |
| warning | W200 | function 'getStringSlot' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'BeaconProxy' overrides 'Proxy::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBytesSlot' in 'BeaconProxy' overrides 'Proxy::getBytesSlot' which is not marked 'virtual' |
| warning | W200 | function 'getBytesSlot' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getImplementation' in 'BeaconProxy' overrides 'Proxy::getImplementation' which is not marked 'virtual' |
| warning | W200 | function 'getImplementation' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setImplementation' in 'BeaconProxy' overrides 'Proxy::_setImplementation' which is not marked 'virtual' |
| warning | W200 | function '_setImplementation' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeToAndCall' in 'BeaconProxy' overrides 'Proxy::upgradeToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeToAndCall' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAdmin' in 'BeaconProxy' overrides 'Proxy::getAdmin' which is not marked 'virtual' |
| warning | W200 | function 'getAdmin' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setAdmin' in 'BeaconProxy' overrides 'Proxy::_setAdmin' which is not marked 'virtual' |
| warning | W200 | function '_setAdmin' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'changeAdmin' in 'BeaconProxy' overrides 'Proxy::changeAdmin' which is not marked 'virtual' |
| warning | W200 | function 'changeAdmin' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBeacon' in 'BeaconProxy' overrides 'Proxy::getBeacon' which is not marked 'virtual' |
| warning | W200 | function 'getBeacon' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_setBeacon' in 'BeaconProxy' overrides 'Proxy::_setBeacon' which is not marked 'virtual' |
| warning | W200 | function '_setBeacon' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'BeaconProxy' overrides 'Proxy::upgradeBeaconToAndCall' which is not marked 'virtual' |
| warning | W200 | function 'upgradeBeaconToAndCall' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_checkNonPayable' in 'BeaconProxy' overrides 'Proxy::_checkNonPayable' which is not marked 'virtual' |
| warning | W200 | function '_checkNonPayable' in 'BeaconProxy' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'BeaconProxy' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@openzeppelin/contracts/proxy/beacon/BeaconProxy.sol`