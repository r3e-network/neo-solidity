# SafeL2 (Safe)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@safe-global/safe-contracts/contracts/SafeL2.sol`
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
| warning | RAW | warning: tx.origin has different semantics on Neo N3. Neo uses multi-signature witnesses instead of a single origin. Consider using msg.sender or Runtime.CheckWitness() instead. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.origin has different semantics on Neo N3. Neo uses multi-signature witnesses instead of a single origin. Consider using msg.sender or Runtime.CheckWitness() instead. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | W200 | function 'mul' in 'Executor' overrides 'SelfAuthorized::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'Executor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'Executor' overrides 'SelfAuthorized::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'Executor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Executor' overrides 'SelfAuthorized::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Executor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'Executor' overrides 'SelfAuthorized::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'Executor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'ModuleManager' overrides 'Executor::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'ModuleManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'ModuleManager' overrides 'Executor::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'ModuleManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ModuleManager' overrides 'Executor::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ModuleManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ModuleManager' overrides 'Executor::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ModuleManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'OwnerManager' overrides 'SelfAuthorized::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'OwnerManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'OwnerManager' overrides 'SelfAuthorized::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'OwnerManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'OwnerManager' overrides 'SelfAuthorized::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'OwnerManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'OwnerManager' overrides 'SelfAuthorized::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'OwnerManager' overrides a base function but is not marked 'override' |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W200 | function 'mul' in 'FallbackManager' overrides 'SelfAuthorized::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'FallbackManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'FallbackManager' overrides 'SelfAuthorized::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'FallbackManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'FallbackManager' overrides 'SelfAuthorized::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'FallbackManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'FallbackManager' overrides 'SelfAuthorized::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'FallbackManager' overrides a base function but is not marked 'override' |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W200 | function 'mul' in 'GuardManager' overrides 'SelfAuthorized::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'GuardManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'GuardManager' overrides 'SelfAuthorized::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'GuardManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'GuardManager' overrides 'SelfAuthorized::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'GuardManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'GuardManager' overrides 'SelfAuthorized::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'GuardManager' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'GuardManager' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'onNEP17Payment' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | VALIDATION_WARNING | abstract contract 'ISignatureValidator' has 1 unimplemented function(s): [isValidSignature] |
| warning | W200 | function 'mul' in 'ISignatureValidator' overrides 'ISignatureValidatorConstants::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'ISignatureValidator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'ISignatureValidator' overrides 'ISignatureValidatorConstants::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'ISignatureValidator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ISignatureValidator' overrides 'ISignatureValidatorConstants::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ISignatureValidator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ISignatureValidator' overrides 'ISignatureValidatorConstants::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ISignatureValidator' overrides a base function but is not marked 'override' |
| warning | W111 | function 'onNEP17Payment' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'execTransaction' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W105 | function 'fallback' has no effect on Neo N3. The contract already defines onNEP17Payment which is the correct Neo callback for receiving tokens. |
| warning | W116 | function 'execTransaction' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'mul' in 'NativeCurrencyPaymentFallback' overrides 'Singleton::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'NativeCurrencyPaymentFallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'NativeCurrencyPaymentFallback' overrides 'Singleton::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'NativeCurrencyPaymentFallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'NativeCurrencyPaymentFallback' overrides 'Singleton::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'NativeCurrencyPaymentFallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'NativeCurrencyPaymentFallback' overrides 'Singleton::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'NativeCurrencyPaymentFallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'SelfAuthorized' overrides 'NativeCurrencyPaymentFallback::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'SelfAuthorized' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'SelfAuthorized' overrides 'NativeCurrencyPaymentFallback::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'SelfAuthorized' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'SelfAuthorized' overrides 'NativeCurrencyPaymentFallback::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'SelfAuthorized' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'SelfAuthorized' overrides 'NativeCurrencyPaymentFallback::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'SelfAuthorized' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'Executor' overrides 'SelfAuthorized::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'Executor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'Executor' overrides 'SelfAuthorized::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'Executor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Executor' overrides 'SelfAuthorized::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Executor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'Executor' overrides 'SelfAuthorized::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'Executor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'ModuleManager' overrides 'Executor::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'ModuleManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'ModuleManager' overrides 'Executor::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'ModuleManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ModuleManager' overrides 'Executor::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ModuleManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ModuleManager' overrides 'Executor::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ModuleManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'OwnerManager' overrides 'ModuleManager::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'OwnerManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'OwnerManager' overrides 'ModuleManager::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'OwnerManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'OwnerManager' overrides 'ModuleManager::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'OwnerManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'OwnerManager' overrides 'ModuleManager::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'OwnerManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'SignatureDecoder' overrides 'OwnerManager::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'SignatureDecoder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'SignatureDecoder' overrides 'OwnerManager::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'SignatureDecoder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'SignatureDecoder' overrides 'OwnerManager::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'SignatureDecoder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'SignatureDecoder' overrides 'OwnerManager::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'SignatureDecoder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'SecuredTokenTransfer' overrides 'SignatureDecoder::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'SecuredTokenTransfer' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'SecuredTokenTransfer' overrides 'SignatureDecoder::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'SecuredTokenTransfer' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'SecuredTokenTransfer' overrides 'SignatureDecoder::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'SecuredTokenTransfer' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'SecuredTokenTransfer' overrides 'SignatureDecoder::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'SecuredTokenTransfer' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'ISignatureValidatorConstants' overrides 'SecuredTokenTransfer::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'ISignatureValidatorConstants' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'ISignatureValidatorConstants' overrides 'SecuredTokenTransfer::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'ISignatureValidatorConstants' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ISignatureValidatorConstants' overrides 'SecuredTokenTransfer::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ISignatureValidatorConstants' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ISignatureValidatorConstants' overrides 'SecuredTokenTransfer::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ISignatureValidatorConstants' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'FallbackManager' overrides 'ISignatureValidatorConstants::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'FallbackManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'FallbackManager' overrides 'ISignatureValidatorConstants::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'FallbackManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'FallbackManager' overrides 'ISignatureValidatorConstants::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'FallbackManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'FallbackManager' overrides 'ISignatureValidatorConstants::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'FallbackManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'StorageAccessible' overrides 'FallbackManager::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'StorageAccessible' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'StorageAccessible' overrides 'FallbackManager::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'StorageAccessible' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'StorageAccessible' overrides 'FallbackManager::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'StorageAccessible' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'StorageAccessible' overrides 'FallbackManager::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'StorageAccessible' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'GuardManager' overrides 'StorageAccessible::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'GuardManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'GuardManager' overrides 'StorageAccessible::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'GuardManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'GuardManager' overrides 'StorageAccessible::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'GuardManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'GuardManager' overrides 'StorageAccessible::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'GuardManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'Safe' overrides 'GuardManager::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'Safe' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'Safe' overrides 'GuardManager::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'Safe' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Safe' overrides 'GuardManager::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Safe' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'Safe' overrides 'GuardManager::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'Safe' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Safe' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'onNEP17Payment' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'execTransaction' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function '__super_execTransaction' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W105 | function 'fallback' has no effect on Neo N3. The contract already defines onNEP17Payment which is the correct Neo callback for receiving tokens. |
| warning | W116 | function 'execTransaction' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'mul' in 'NativeCurrencyPaymentFallback' overrides 'Singleton::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'NativeCurrencyPaymentFallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'NativeCurrencyPaymentFallback' overrides 'Singleton::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'NativeCurrencyPaymentFallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'NativeCurrencyPaymentFallback' overrides 'Singleton::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'NativeCurrencyPaymentFallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'NativeCurrencyPaymentFallback' overrides 'Singleton::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'NativeCurrencyPaymentFallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'SelfAuthorized' overrides 'NativeCurrencyPaymentFallback::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'SelfAuthorized' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'SelfAuthorized' overrides 'NativeCurrencyPaymentFallback::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'SelfAuthorized' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'SelfAuthorized' overrides 'NativeCurrencyPaymentFallback::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'SelfAuthorized' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'SelfAuthorized' overrides 'NativeCurrencyPaymentFallback::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'SelfAuthorized' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'Executor' overrides 'SelfAuthorized::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'Executor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'Executor' overrides 'SelfAuthorized::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'Executor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Executor' overrides 'SelfAuthorized::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Executor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'Executor' overrides 'SelfAuthorized::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'Executor' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'ModuleManager' overrides 'Executor::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'ModuleManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'ModuleManager' overrides 'Executor::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'ModuleManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ModuleManager' overrides 'Executor::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ModuleManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ModuleManager' overrides 'Executor::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ModuleManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'OwnerManager' overrides 'ModuleManager::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'OwnerManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'OwnerManager' overrides 'ModuleManager::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'OwnerManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'OwnerManager' overrides 'ModuleManager::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'OwnerManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'OwnerManager' overrides 'ModuleManager::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'OwnerManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'SignatureDecoder' overrides 'OwnerManager::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'SignatureDecoder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'SignatureDecoder' overrides 'OwnerManager::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'SignatureDecoder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'SignatureDecoder' overrides 'OwnerManager::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'SignatureDecoder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'SignatureDecoder' overrides 'OwnerManager::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'SignatureDecoder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'SecuredTokenTransfer' overrides 'SignatureDecoder::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'SecuredTokenTransfer' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'SecuredTokenTransfer' overrides 'SignatureDecoder::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'SecuredTokenTransfer' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'SecuredTokenTransfer' overrides 'SignatureDecoder::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'SecuredTokenTransfer' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'SecuredTokenTransfer' overrides 'SignatureDecoder::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'SecuredTokenTransfer' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'ISignatureValidatorConstants' overrides 'SecuredTokenTransfer::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'ISignatureValidatorConstants' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'ISignatureValidatorConstants' overrides 'SecuredTokenTransfer::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'ISignatureValidatorConstants' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'ISignatureValidatorConstants' overrides 'SecuredTokenTransfer::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'ISignatureValidatorConstants' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ISignatureValidatorConstants' overrides 'SecuredTokenTransfer::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ISignatureValidatorConstants' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'FallbackManager' overrides 'ISignatureValidatorConstants::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'FallbackManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'FallbackManager' overrides 'ISignatureValidatorConstants::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'FallbackManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'FallbackManager' overrides 'ISignatureValidatorConstants::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'FallbackManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'FallbackManager' overrides 'ISignatureValidatorConstants::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'FallbackManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'StorageAccessible' overrides 'FallbackManager::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'StorageAccessible' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'StorageAccessible' overrides 'FallbackManager::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'StorageAccessible' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'StorageAccessible' overrides 'FallbackManager::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'StorageAccessible' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'StorageAccessible' overrides 'FallbackManager::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'StorageAccessible' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'GuardManager' overrides 'StorageAccessible::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'GuardManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'GuardManager' overrides 'StorageAccessible::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'GuardManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'GuardManager' overrides 'StorageAccessible::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'GuardManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'GuardManager' overrides 'StorageAccessible::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'GuardManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'Safe' overrides 'GuardManager::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'Safe' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'Safe' overrides 'GuardManager::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'Safe' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'Safe' overrides 'GuardManager::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'Safe' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'Safe' overrides 'GuardManager::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'Safe' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'SafeL2' overrides 'Safe::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'SafeL2' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'SafeL2' overrides 'Safe::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'SafeL2' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add' in 'SafeL2' overrides 'Safe::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'SafeL2' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'SafeL2' overrides 'Safe::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'SafeL2' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'SafeL2' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@safe-global/safe-contracts/contracts/SafeL2.sol`