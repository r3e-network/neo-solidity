# PoolAddressesProvider (Aave V3)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@aave/core-v3/contracts/protocol/configuration/PoolAddressesProvider.sol`
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
| warning | W200 | function 'isContract' in 'Ownable' overrides 'Context::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Ownable' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Ownable' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | VALIDATION_WARNING | abstract contract 'Proxy' has 1 unimplemented function(s): [_implementation] |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'isContract' in 'BaseUpgradeabilityProxy' overrides 'Proxy::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'BaseUpgradeabilityProxy' overrides 'Proxy::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'BaseUpgradeabilityProxy' overrides 'Proxy::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'initialize' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'initialize' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'isContract' in 'BaseUpgradeabilityProxy' overrides 'Proxy::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'BaseUpgradeabilityProxy' overrides 'Proxy::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'BaseUpgradeabilityProxy' overrides 'Proxy::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'upgradeToAndCall' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'upgradeToAndCall' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'isContract' in 'BaseUpgradeabilityProxy' overrides 'Proxy::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'BaseUpgradeabilityProxy' overrides 'Proxy::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'BaseUpgradeabilityProxy' overrides 'Proxy::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'upgradeToAndCall' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'initialize' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'upgradeToAndCall' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'initialize' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'isContract' in 'BaseUpgradeabilityProxy' overrides 'Proxy::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'BaseUpgradeabilityProxy' overrides 'Proxy::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides 'Proxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'BaseUpgradeabilityProxy' overrides 'Proxy::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'Ownable' overrides 'Context::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Ownable' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Ownable' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Ownable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'PoolAddressesProvider' overrides 'Ownable::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'PoolAddressesProvider' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'PoolAddressesProvider' overrides 'Ownable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'PoolAddressesProvider' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'PoolAddressesProvider' overrides 'Ownable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'PoolAddressesProvider' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'PoolAddressesProvider' overrides 'Ownable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'PoolAddressesProvider' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'PoolAddressesProvider' overrides 'Ownable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'PoolAddressesProvider' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'PoolAddressesProvider' overrides 'Ownable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'PoolAddressesProvider' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'PoolAddressesProvider' overrides 'Ownable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'PoolAddressesProvider' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'PoolAddressesProvider' overrides 'Ownable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'PoolAddressesProvider' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'PoolAddressesProvider' overrides 'Ownable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'PoolAddressesProvider' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'PoolAddressesProvider' overrides 'Ownable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'PoolAddressesProvider' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'PoolAddressesProvider' overrides 'Ownable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'PoolAddressesProvider' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'PoolAddressesProvider' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@aave/core-v3/contracts/protocol/configuration/PoolAddressesProvider.sol`