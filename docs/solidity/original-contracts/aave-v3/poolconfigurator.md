# PoolConfigurator (Aave V3)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@aave/core-v3/contracts/protocol/pool/PoolConfigurator.sol`
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
| warning | VALIDATION_WARNING | abstract contract 'VersionedInitializable' has 1 unimplemented function(s): [getRevision] |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'VersionedInitializable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | VALIDATION_WARNING | abstract contract 'Proxy' has 1 unimplemented function(s): [_implementation] |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Proxy' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_CONTRACT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_CONFIGURATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ATOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_MORE_RESERVES_ALLOWED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_RESERVED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_LIQUIDITY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_PREMIUM_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BRIDGE_PROTOCOL_FEE_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_MUST_BE_POOL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_MINT_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BURN_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_INACTIVE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FROZEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_PAUSED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_BALANCE_IS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_DEBT_OF_SELECTED_TYPE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_STABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_VARIABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_BALANCE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_FLASHLOAN_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_VALIDATION_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_INITIALIZED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_INDEX' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACL_ADMIN_CANNOT_BE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_PARAMS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_ADDRESS_NOT_VALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EXPIRATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SIGNATURE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPERATION_NOT_SUPPORTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_LISTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_USAGE_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CANNOT_BE_RESCUED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOL_ADDRESSES_DO_NOT_MATCH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_VIOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_DISABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECIMALS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACTIVE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FROZEN_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PAUSED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DECIMALS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_ACTIVE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_FROZEN_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_PAUSED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_RESERVES_COUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'setLtv' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'BaseUpgradeabilityProxy' overrides 'Proxy::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'BaseUpgradeabilityProxy' overrides 'Proxy::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'executeInitReserve' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateAToken' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeUpdateAToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateAToken' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeUpdateStableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeUpdateVariableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_initTokenWithProxy' in 'BaseUpgradeabilityProxy' overrides 'Proxy::_initTokenWithProxy' which is not marked 'virtual' |
| warning | W200 | function '_initTokenWithProxy' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_upgradeTokenImplementation' in 'BaseUpgradeabilityProxy' overrides 'Proxy::_upgradeTokenImplementation' which is not marked 'virtual' |
| warning | W200 | function '_upgradeTokenImplementation' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'BaseUpgradeabilityProxy' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'initialize' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_CONTRACT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_CONFIGURATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ATOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_MORE_RESERVES_ALLOWED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_RESERVED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_LIQUIDITY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_PREMIUM_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BRIDGE_PROTOCOL_FEE_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_MUST_BE_POOL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_MINT_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BURN_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_INACTIVE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FROZEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_PAUSED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_BALANCE_IS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_DEBT_OF_SELECTED_TYPE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_STABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_VARIABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_BALANCE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_FLASHLOAN_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_VALIDATION_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_INITIALIZED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_INDEX' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACL_ADMIN_CANNOT_BE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_PARAMS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_ADDRESS_NOT_VALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EXPIRATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SIGNATURE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPERATION_NOT_SUPPORTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_LISTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_USAGE_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CANNOT_BE_RESCUED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOL_ADDRESSES_DO_NOT_MATCH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_VIOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_DISABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECIMALS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACTIVE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FROZEN_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PAUSED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DECIMALS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_ACTIVE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_FROZEN_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_PAUSED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_RESERVES_COUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_CONTRACT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_CONFIGURATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ATOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_MORE_RESERVES_ALLOWED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_RESERVED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_LIQUIDITY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_PREMIUM_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BRIDGE_PROTOCOL_FEE_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_MUST_BE_POOL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_MINT_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BURN_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_INACTIVE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FROZEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_PAUSED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_BALANCE_IS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_DEBT_OF_SELECTED_TYPE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_STABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_VARIABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_BALANCE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_FLASHLOAN_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_VALIDATION_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_INITIALIZED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_INDEX' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACL_ADMIN_CANNOT_BE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_PARAMS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_ADDRESS_NOT_VALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EXPIRATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SIGNATURE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPERATION_NOT_SUPPORTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_LISTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_USAGE_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CANNOT_BE_RESCUED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOL_ADDRESSES_DO_NOT_MATCH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_VIOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_DISABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECIMALS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACTIVE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FROZEN_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PAUSED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DECIMALS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_ACTIVE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_FROZEN_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_PAUSED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_RESERVES_COUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'initialize' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'setLtv' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'BaseUpgradeabilityProxy' overrides 'Proxy::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'BaseUpgradeabilityProxy' overrides 'Proxy::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'executeInitReserve' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateAToken' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeUpdateAToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateAToken' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeUpdateStableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeUpdateVariableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_initTokenWithProxy' in 'BaseUpgradeabilityProxy' overrides 'Proxy::_initTokenWithProxy' which is not marked 'virtual' |
| warning | W200 | function '_initTokenWithProxy' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_upgradeTokenImplementation' in 'BaseUpgradeabilityProxy' overrides 'Proxy::_upgradeTokenImplementation' which is not marked 'virtual' |
| warning | W200 | function '_upgradeTokenImplementation' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLtv' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'executeInitReserve' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateAToken' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::executeUpdateAToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateAToken' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::executeUpdateStableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::executeUpdateVariableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_initTokenWithProxy' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::_initTokenWithProxy' which is not marked 'virtual' |
| warning | W200 | function '_initTokenWithProxy' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_upgradeTokenImplementation' in 'InitializableUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::_upgradeTokenImplementation' which is not marked 'virtual' |
| warning | W200 | function '_upgradeTokenImplementation' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'InitializableUpgradeabilityProxy' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'upgradeToAndCall' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_CONTRACT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_CONFIGURATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ATOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_MORE_RESERVES_ALLOWED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_RESERVED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_LIQUIDITY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_PREMIUM_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BRIDGE_PROTOCOL_FEE_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_MUST_BE_POOL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_MINT_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BURN_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_INACTIVE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FROZEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_PAUSED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_BALANCE_IS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_DEBT_OF_SELECTED_TYPE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_STABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_VARIABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_BALANCE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_FLASHLOAN_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_VALIDATION_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_INITIALIZED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_INDEX' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACL_ADMIN_CANNOT_BE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_PARAMS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_ADDRESS_NOT_VALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EXPIRATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SIGNATURE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPERATION_NOT_SUPPORTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_LISTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_USAGE_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CANNOT_BE_RESCUED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOL_ADDRESSES_DO_NOT_MATCH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_VIOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_DISABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECIMALS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACTIVE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FROZEN_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PAUSED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DECIMALS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_ACTIVE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_FROZEN_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_PAUSED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_RESERVES_COUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_CONTRACT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_CONFIGURATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ATOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_MORE_RESERVES_ALLOWED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_RESERVED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_LIQUIDITY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_PREMIUM_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BRIDGE_PROTOCOL_FEE_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_MUST_BE_POOL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_MINT_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BURN_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_INACTIVE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FROZEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_PAUSED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_BALANCE_IS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_DEBT_OF_SELECTED_TYPE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_STABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_VARIABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_BALANCE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_FLASHLOAN_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_VALIDATION_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_INITIALIZED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_INDEX' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACL_ADMIN_CANNOT_BE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_PARAMS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_ADDRESS_NOT_VALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EXPIRATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SIGNATURE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPERATION_NOT_SUPPORTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_LISTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_USAGE_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CANNOT_BE_RESCUED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOL_ADDRESSES_DO_NOT_MATCH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_VIOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_DISABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECIMALS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACTIVE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FROZEN_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PAUSED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DECIMALS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_ACTIVE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_FROZEN_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_PAUSED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_RESERVES_COUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'upgradeToAndCall' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'setLtv' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'BaseUpgradeabilityProxy' overrides 'Proxy::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'BaseUpgradeabilityProxy' overrides 'Proxy::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'executeInitReserve' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateAToken' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeUpdateAToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateAToken' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeUpdateStableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeUpdateVariableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_initTokenWithProxy' in 'BaseUpgradeabilityProxy' overrides 'Proxy::_initTokenWithProxy' which is not marked 'virtual' |
| warning | W200 | function '_initTokenWithProxy' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_upgradeTokenImplementation' in 'BaseUpgradeabilityProxy' overrides 'Proxy::_upgradeTokenImplementation' which is not marked 'virtual' |
| warning | W200 | function '_upgradeTokenImplementation' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLtv' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'executeInitReserve' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateAToken' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::executeUpdateAToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateAToken' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::executeUpdateStableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::executeUpdateVariableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_initTokenWithProxy' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::_initTokenWithProxy' which is not marked 'virtual' |
| warning | W200 | function '_initTokenWithProxy' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_upgradeTokenImplementation' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::_upgradeTokenImplementation' which is not marked 'virtual' |
| warning | W200 | function '_upgradeTokenImplementation' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'BaseImmutableAdminUpgradeabilityProxy' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'fallback' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'upgradeToAndCall' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'initialize' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_CONTRACT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_CONFIGURATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ATOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_MORE_RESERVES_ALLOWED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_RESERVED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_LIQUIDITY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_PREMIUM_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BRIDGE_PROTOCOL_FEE_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_MUST_BE_POOL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_MINT_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BURN_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_INACTIVE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FROZEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_PAUSED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_BALANCE_IS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_DEBT_OF_SELECTED_TYPE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_STABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_VARIABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_BALANCE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_FLASHLOAN_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_VALIDATION_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_INITIALIZED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_INDEX' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACL_ADMIN_CANNOT_BE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_PARAMS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_ADDRESS_NOT_VALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EXPIRATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SIGNATURE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPERATION_NOT_SUPPORTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_LISTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_USAGE_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CANNOT_BE_RESCUED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOL_ADDRESSES_DO_NOT_MATCH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_VIOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_DISABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECIMALS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACTIVE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FROZEN_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PAUSED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DECIMALS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_ACTIVE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_FROZEN_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_PAUSED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_RESERVES_COUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_CONTRACT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_CONFIGURATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ATOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_MORE_RESERVES_ALLOWED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_RESERVED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_LIQUIDITY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_PREMIUM_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BRIDGE_PROTOCOL_FEE_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_MUST_BE_POOL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_MINT_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BURN_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_INACTIVE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FROZEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_PAUSED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_BALANCE_IS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_DEBT_OF_SELECTED_TYPE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_STABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_VARIABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_BALANCE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_FLASHLOAN_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_VALIDATION_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_INITIALIZED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_INDEX' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACL_ADMIN_CANNOT_BE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_PARAMS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_ADDRESS_NOT_VALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EXPIRATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SIGNATURE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPERATION_NOT_SUPPORTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_LISTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_USAGE_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CANNOT_BE_RESCUED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOL_ADDRESSES_DO_NOT_MATCH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_VIOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_DISABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECIMALS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACTIVE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FROZEN_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PAUSED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DECIMALS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_ACTIVE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_FROZEN_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_PAUSED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_RESERVES_COUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_CONTRACT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_CONFIGURATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ATOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_MORE_RESERVES_ALLOWED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_RESERVED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_LIQUIDITY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_PREMIUM_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BRIDGE_PROTOCOL_FEE_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_MUST_BE_POOL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_MINT_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BURN_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_INACTIVE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FROZEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_PAUSED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_BALANCE_IS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_DEBT_OF_SELECTED_TYPE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_STABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_VARIABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_BALANCE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_FLASHLOAN_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_VALIDATION_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_INITIALIZED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_INDEX' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACL_ADMIN_CANNOT_BE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_PARAMS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_ADDRESS_NOT_VALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EXPIRATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SIGNATURE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPERATION_NOT_SUPPORTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_LISTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_USAGE_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CANNOT_BE_RESCUED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOL_ADDRESSES_DO_NOT_MATCH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_VIOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_DISABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECIMALS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACTIVE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FROZEN_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PAUSED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DECIMALS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_ACTIVE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_FROZEN_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_PAUSED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_RESERVES_COUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_CONTRACT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_CONFIGURATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ATOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_MORE_RESERVES_ALLOWED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_RESERVED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_LIQUIDITY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_PREMIUM_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BRIDGE_PROTOCOL_FEE_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_MUST_BE_POOL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_MINT_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BURN_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_INACTIVE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FROZEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_PAUSED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_BALANCE_IS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_DEBT_OF_SELECTED_TYPE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_STABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_VARIABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_BALANCE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_FLASHLOAN_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_VALIDATION_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_INITIALIZED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_INDEX' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACL_ADMIN_CANNOT_BE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_PARAMS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_ADDRESS_NOT_VALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EXPIRATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SIGNATURE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPERATION_NOT_SUPPORTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_LISTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_USAGE_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CANNOT_BE_RESCUED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOL_ADDRESSES_DO_NOT_MATCH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_VIOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_DISABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECIMALS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACTIVE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FROZEN_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PAUSED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DECIMALS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_ACTIVE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_FROZEN_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_PAUSED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_RESERVES_COUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W105 | function 'fallback' has no effect on Neo N3. Use onNEP17Payment(address from, uint256 amount, bytes data) to handle incoming token payments. |
| warning | W116 | function 'fallback' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'upgradeToAndCall' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'initialize' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'setLtv' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'BaseUpgradeabilityProxy' overrides 'Proxy::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'BaseUpgradeabilityProxy' overrides 'Proxy::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'BaseUpgradeabilityProxy' overrides 'Proxy::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'executeInitReserve' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateAToken' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeUpdateAToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateAToken' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeUpdateStableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'BaseUpgradeabilityProxy' overrides 'Proxy::executeUpdateVariableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_initTokenWithProxy' in 'BaseUpgradeabilityProxy' overrides 'Proxy::_initTokenWithProxy' which is not marked 'virtual' |
| warning | W200 | function '_initTokenWithProxy' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_upgradeTokenImplementation' in 'BaseUpgradeabilityProxy' overrides 'Proxy::_upgradeTokenImplementation' which is not marked 'virtual' |
| warning | W200 | function '_upgradeTokenImplementation' in 'BaseUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLtv' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'executeInitReserve' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateAToken' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::executeUpdateAToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateAToken' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::executeUpdateStableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::executeUpdateVariableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_initTokenWithProxy' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::_initTokenWithProxy' which is not marked 'virtual' |
| warning | W200 | function '_initTokenWithProxy' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_upgradeTokenImplementation' in 'BaseImmutableAdminUpgradeabilityProxy' overrides 'BaseUpgradeabilityProxy::_upgradeTokenImplementation' which is not marked 'virtual' |
| warning | W200 | function '_upgradeTokenImplementation' in 'BaseImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLtv' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'executeInitReserve' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateAToken' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::executeUpdateAToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateAToken' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::executeUpdateStableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::executeUpdateVariableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_initTokenWithProxy' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::_initTokenWithProxy' which is not marked 'virtual' |
| warning | W200 | function '_initTokenWithProxy' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_upgradeTokenImplementation' in 'InitializableUpgradeabilityProxy' overrides 'BaseImmutableAdminUpgradeabilityProxy::_upgradeTokenImplementation' which is not marked 'virtual' |
| warning | W200 | function '_upgradeTokenImplementation' in 'InitializableUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLtv' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'executeInitReserve' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateAToken' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::executeUpdateAToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateAToken' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::executeUpdateStableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::executeUpdateVariableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_initTokenWithProxy' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::_initTokenWithProxy' which is not marked 'virtual' |
| warning | W200 | function '_initTokenWithProxy' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | W200 | function '_upgradeTokenImplementation' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides 'InitializableUpgradeabilityProxy::_upgradeTokenImplementation' which is not marked 'virtual' |
| warning | W200 | function '_upgradeTokenImplementation' in 'InitializableImmutableAdminUpgradeabilityProxy' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'InitializableImmutableAdminUpgradeabilityProxy' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_CONTRACT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_POOL_CONFIGURATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_NOT_ATOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_ADDRESSES_PROVIDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_MORE_RESERVES_ALLOWED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_RESERVED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_LIQUIDITY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_PREMIUM_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BRIDGE_PROTOCOL_FEE_INVALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CALLER_MUST_BE_POOL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_MINT_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BURN_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_AMOUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_INACTIVE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FROZEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_PAUSED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_NOT_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_BALANCE_IS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_DEBT_OF_SELECTED_TYPE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_STABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NO_OUTSTANDING_VARIABLE_DEBT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_BALANCE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_FLASHLOAN_PARAMS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_EXCEEDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_VALIDATION_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_ALREADY_INITIALIZED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQ_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_RESERVE_INDEX' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACL_ADMIN_CANNOT_BE_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCONSISTENT_PARAMS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_ADDRESS_NOT_VALID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_EXPIRATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_SIGNATURE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPERATION_NOT_SUPPORTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSET_NOT_LISTED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_USAGE_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDERLYING_CANNOT_BE_RESCUED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOL_ADDRESSES_DO_NOT_MATCH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_VIOLATION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DEBT_NOT_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_DISABLED' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LTV_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECIMALS_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ACTIVE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FROZEN_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PAUSED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_THRESHOLD_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_BONUS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_DECIMALS_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_ACTIVE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_FROZEN_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STABLE_BORROWING_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_PAUSED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWABLE_IN_ISOLATION_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SILOED_BORROWING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FLASHLOAN_ENABLED_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVE_FACTOR_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROW_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SUPPLY_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDATION_PROTOCOL_FEE_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMODE_CATEGORY_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNBACKED_MINT_CAP_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_START_BIT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LTV' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_BONUS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_RESERVE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_BORROW_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_SUPPLY_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_LIQUIDATION_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_EMODE_CATEGORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_UNBACKED_MINT_CAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_VALID_DEBT_CEILING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEBT_CEILING_DECIMALS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_RESERVES_COUNT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W200 | function 'setLtv' in 'PoolConfigurator' overrides 'VersionedInitializable::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'PoolConfigurator' overrides 'VersionedInitializable::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'PoolConfigurator' overrides 'VersionedInitializable::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'PoolConfigurator' overrides 'VersionedInitializable::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'PoolConfigurator' overrides 'VersionedInitializable::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'PoolConfigurator' overrides 'VersionedInitializable::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'PoolConfigurator' overrides 'VersionedInitializable::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'PoolConfigurator' overrides 'VersionedInitializable::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'PoolConfigurator' overrides 'VersionedInitializable::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'PoolConfigurator' overrides 'VersionedInitializable::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'PoolConfigurator' overrides 'VersionedInitializable::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'PoolConfigurator' overrides 'VersionedInitializable::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'PoolConfigurator' overrides 'VersionedInitializable::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'PoolConfigurator' overrides 'VersionedInitializable::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'PoolConfigurator' overrides 'VersionedInitializable::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'PoolConfigurator' overrides 'VersionedInitializable::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'PoolConfigurator' overrides 'VersionedInitializable::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'PoolConfigurator' overrides 'VersionedInitializable::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'PoolConfigurator' overrides 'VersionedInitializable::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'PoolConfigurator' overrides 'VersionedInitializable::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'PoolConfigurator' overrides 'VersionedInitializable::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'PoolConfigurator' overrides 'VersionedInitializable::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'PoolConfigurator' overrides 'VersionedInitializable::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'PoolConfigurator' overrides 'VersionedInitializable::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'PoolConfigurator' overrides 'VersionedInitializable::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'PoolConfigurator' overrides 'VersionedInitializable::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'PoolConfigurator' overrides 'VersionedInitializable::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'PoolConfigurator' overrides 'VersionedInitializable::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'PoolConfigurator' overrides 'VersionedInitializable::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'PoolConfigurator' overrides 'VersionedInitializable::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'PoolConfigurator' overrides 'VersionedInitializable::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'PoolConfigurator' overrides 'VersionedInitializable::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'PoolConfigurator' overrides 'VersionedInitializable::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'PoolConfigurator' overrides 'VersionedInitializable::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'PoolConfigurator' overrides 'VersionedInitializable::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'PoolConfigurator' overrides 'VersionedInitializable::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'PoolConfigurator' overrides 'VersionedInitializable::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'PoolConfigurator' overrides 'VersionedInitializable::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'PoolConfigurator' overrides 'VersionedInitializable::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'PoolConfigurator' overrides 'VersionedInitializable::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'PoolConfigurator' overrides 'VersionedInitializable::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'PoolConfigurator' overrides 'VersionedInitializable::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'PoolConfigurator' overrides 'VersionedInitializable::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'PoolConfigurator' overrides 'VersionedInitializable::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'PoolConfigurator' overrides 'VersionedInitializable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'PoolConfigurator' overrides 'VersionedInitializable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'PoolConfigurator' overrides 'VersionedInitializable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'PoolConfigurator' overrides 'VersionedInitializable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'PoolConfigurator' overrides 'VersionedInitializable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'PoolConfigurator' overrides 'VersionedInitializable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'PoolConfigurator' overrides 'VersionedInitializable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'PoolConfigurator' overrides 'VersionedInitializable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'PoolConfigurator' overrides 'VersionedInitializable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'PoolConfigurator' overrides 'VersionedInitializable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeInitReserve' in 'PoolConfigurator' overrides 'VersionedInitializable::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateAToken' in 'PoolConfigurator' overrides 'VersionedInitializable::executeUpdateAToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateAToken' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'PoolConfigurator' overrides 'VersionedInitializable::executeUpdateStableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateStableDebtToken' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'PoolConfigurator' overrides 'VersionedInitializable::executeUpdateVariableDebtToken' which is not marked 'virtual' |
| warning | W200 | function 'executeUpdateVariableDebtToken' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function '_initTokenWithProxy' in 'PoolConfigurator' overrides 'VersionedInitializable::_initTokenWithProxy' which is not marked 'virtual' |
| warning | W200 | function '_initTokenWithProxy' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | W200 | function '_upgradeTokenImplementation' in 'PoolConfigurator' overrides 'VersionedInitializable::_upgradeTokenImplementation' which is not marked 'virtual' |
| warning | W200 | function '_upgradeTokenImplementation' in 'PoolConfigurator' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'PoolConfigurator' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@aave/core-v3/contracts/protocol/pool/PoolConfigurator.sol`