# L2Pool (Aave V3)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@aave/core-v3/contracts/protocol/pool/L2Pool.sol`
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
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | VALIDATION_WARNING | abstract contract 'VersionedInitializable' has 1 unimplemented function(s): [getRevision] |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'VersionedInitializable' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Context' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ISOLATED_COLLATERAL_SUPPLIER_ROLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEFAULT_LIQUIDATION_CLOSE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LIQUIDATION_CLOSE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_FACTOR_HF_THRESHOLD' detected while merging libraries |
| warning | W101 | function 'transfer' has 2 parameters (ERC-20 pattern). NEP-17 requires 4 parameters: transfer(from, to, amount, data). The `from` address is verified via Runtime.checkWitness() and `data` (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |
| warning | W200 | function 'setLtv' in 'IncentivizedERC20' overrides 'Context::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'IncentivizedERC20' overrides 'Context::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'IncentivizedERC20' overrides 'Context::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'IncentivizedERC20' overrides 'Context::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'IncentivizedERC20' overrides 'Context::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'IncentivizedERC20' overrides 'Context::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'IncentivizedERC20' overrides 'Context::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'IncentivizedERC20' overrides 'Context::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'IncentivizedERC20' overrides 'Context::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'IncentivizedERC20' overrides 'Context::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'IncentivizedERC20' overrides 'Context::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'IncentivizedERC20' overrides 'Context::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'IncentivizedERC20' overrides 'Context::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'IncentivizedERC20' overrides 'Context::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'IncentivizedERC20' overrides 'Context::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'IncentivizedERC20' overrides 'Context::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'IncentivizedERC20' overrides 'Context::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'IncentivizedERC20' overrides 'Context::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'IncentivizedERC20' overrides 'Context::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'IncentivizedERC20' overrides 'Context::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'IncentivizedERC20' overrides 'Context::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'IncentivizedERC20' overrides 'Context::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'IncentivizedERC20' overrides 'Context::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'IncentivizedERC20' overrides 'Context::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'IncentivizedERC20' overrides 'Context::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'IncentivizedERC20' overrides 'Context::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'IncentivizedERC20' overrides 'Context::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'IncentivizedERC20' overrides 'Context::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'IncentivizedERC20' overrides 'Context::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'IncentivizedERC20' overrides 'Context::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'IncentivizedERC20' overrides 'Context::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'IncentivizedERC20' overrides 'Context::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'IncentivizedERC20' overrides 'Context::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'IncentivizedERC20' overrides 'Context::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'IncentivizedERC20' overrides 'Context::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'IncentivizedERC20' overrides 'Context::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'IncentivizedERC20' overrides 'Context::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'IncentivizedERC20' overrides 'Context::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'IncentivizedERC20' overrides 'Context::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'IncentivizedERC20' overrides 'Context::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'IncentivizedERC20' overrides 'Context::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'IncentivizedERC20' overrides 'Context::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'IncentivizedERC20' overrides 'Context::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLastTransferResult' in 'IncentivizedERC20' overrides 'Context::getLastTransferResult' which is not marked 'virtual' |
| warning | W200 | function 'getLastTransferResult' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'IncentivizedERC20' overrides 'Context::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'IncentivizedERC20' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'IncentivizedERC20' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'IncentivizedERC20' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'IncentivizedERC20' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'IncentivizedERC20' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'IncentivizedERC20' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'IncentivizedERC20' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'IncentivizedERC20' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'IncentivizedERC20' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'IncentivizedERC20' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadMul' in 'IncentivizedERC20' overrides 'Context::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'IncentivizedERC20' overrides 'Context::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'IncentivizedERC20' overrides 'Context::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'IncentivizedERC20' overrides 'Context::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'IncentivizedERC20' overrides 'Context::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'IncentivizedERC20' overrides 'Context::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'IncentivizedERC20' overrides 'Context::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'IncentivizedERC20' overrides 'Context::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'IncentivizedERC20' overrides 'Context::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'IncentivizedERC20' overrides 'Context::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'IncentivizedERC20' overrides 'Context::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'IncentivizedERC20' overrides 'Context::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'IncentivizedERC20' overrides 'Context::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'IncentivizedERC20' overrides 'Context::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'IncentivizedERC20' overrides 'Context::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'IncentivizedERC20' overrides 'Context::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'IncentivizedERC20' overrides 'Context::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'IncentivizedERC20' overrides 'Context::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'IncentivizedERC20' overrides 'Context::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'IncentivizedERC20' overrides 'Context::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'IncentivizedERC20' overrides 'Context::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'IncentivizedERC20' overrides 'Context::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'IncentivizedERC20' overrides 'Context::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'IncentivizedERC20' overrides 'Context::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'IncentivizedERC20' overrides 'Context::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNormalizedIncome' in 'IncentivizedERC20' overrides 'Context::getNormalizedIncome' which is not marked 'virtual' |
| warning | W200 | function 'getNormalizedIncome' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNormalizedDebt' in 'IncentivizedERC20' overrides 'Context::getNormalizedDebt' which is not marked 'virtual' |
| warning | W200 | function 'getNormalizedDebt' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateState' in 'IncentivizedERC20' overrides 'Context::updateState' which is not marked 'virtual' |
| warning | W200 | function 'updateState' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'cumulateToLiquidityIndex' in 'IncentivizedERC20' overrides 'Context::cumulateToLiquidityIndex' which is not marked 'virtual' |
| warning | W200 | function 'cumulateToLiquidityIndex' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'init' in 'IncentivizedERC20' overrides 'Context::init' which is not marked 'virtual' |
| warning | W200 | function 'init' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateInterestRates' in 'IncentivizedERC20' overrides 'Context::updateInterestRates' which is not marked 'virtual' |
| warning | W200 | function 'updateInterestRates' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_accrueToTreasury' in 'IncentivizedERC20' overrides 'Context::_accrueToTreasury' which is not marked 'virtual' |
| warning | W200 | function '_accrueToTreasury' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_updateIndexes' in 'IncentivizedERC20' overrides 'Context::_updateIndexes' which is not marked 'virtual' |
| warning | W200 | function '_updateIndexes' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'cache' in 'IncentivizedERC20' overrides 'Context::cache' which is not marked 'virtual' |
| warning | W200 | function 'cache' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowing' in 'IncentivizedERC20' overrides 'Context::setBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowing' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUsingAsCollateral' in 'IncentivizedERC20' overrides 'Context::setUsingAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'setUsingAsCollateral' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralOrBorrowing' in 'IncentivizedERC20' overrides 'Context::isUsingAsCollateralOrBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralOrBorrowing' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowing' in 'IncentivizedERC20' overrides 'Context::isBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowing' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateral' in 'IncentivizedERC20' overrides 'Context::isUsingAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateral' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralOne' in 'IncentivizedERC20' overrides 'Context::isUsingAsCollateralOne' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralOne' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralAny' in 'IncentivizedERC20' overrides 'Context::isUsingAsCollateralAny' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralAny' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowingOne' in 'IncentivizedERC20' overrides 'Context::isBorrowingOne' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowingOne' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowingAny' in 'IncentivizedERC20' overrides 'Context::isBorrowingAny' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowingAny' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isEmpty' in 'IncentivizedERC20' overrides 'Context::isEmpty' which is not marked 'virtual' |
| warning | W200 | function 'isEmpty' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getIsolationModeState' in 'IncentivizedERC20' overrides 'Context::getIsolationModeState' which is not marked 'virtual' |
| warning | W200 | function 'getIsolationModeState' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowingState' in 'IncentivizedERC20' overrides 'Context::getSiloedBorrowingState' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowingState' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFirstAssetIdByMask' in 'IncentivizedERC20' overrides 'Context::_getFirstAssetIdByMask' which is not marked 'virtual' |
| warning | W200 | function '_getFirstAssetIdByMask' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSetUserEMode' in 'IncentivizedERC20' overrides 'Context::executeSetUserEMode' which is not marked 'virtual' |
| warning | W200 | function 'executeSetUserEMode' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeConfiguration' in 'IncentivizedERC20' overrides 'Context::getEModeConfiguration' which is not marked 'virtual' |
| warning | W200 | function 'getEModeConfiguration' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isInEModeCategory' in 'IncentivizedERC20' overrides 'Context::isInEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'isInEModeCategory' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateUserAccountData' in 'IncentivizedERC20' overrides 'Context::calculateUserAccountData' which is not marked 'virtual' |
| warning | W200 | function 'calculateUserAccountData' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateAvailableBorrows' in 'IncentivizedERC20' overrides 'Context::calculateAvailableBorrows' which is not marked 'virtual' |
| warning | W200 | function 'calculateAvailableBorrows' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getUserDebtInBaseCurrency' in 'IncentivizedERC20' overrides 'Context::_getUserDebtInBaseCurrency' which is not marked 'virtual' |
| warning | W200 | function '_getUserDebtInBaseCurrency' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getUserBalanceInBaseCurrency' in 'IncentivizedERC20' overrides 'Context::_getUserBalanceInBaseCurrency' which is not marked 'virtual' |
| warning | W200 | function '_getUserBalanceInBaseCurrency' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSupply' in 'IncentivizedERC20' overrides 'Context::validateSupply' which is not marked 'virtual' |
| warning | W200 | function 'validateSupply' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateWithdraw' in 'IncentivizedERC20' overrides 'Context::validateWithdraw' which is not marked 'virtual' |
| warning | W200 | function 'validateWithdraw' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateBorrow' in 'IncentivizedERC20' overrides 'Context::validateBorrow' which is not marked 'virtual' |
| warning | W200 | function 'validateBorrow' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateRepay' in 'IncentivizedERC20' overrides 'Context::validateRepay' which is not marked 'virtual' |
| warning | W200 | function 'validateRepay' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSwapRateMode' in 'IncentivizedERC20' overrides 'Context::validateSwapRateMode' which is not marked 'virtual' |
| warning | W200 | function 'validateSwapRateMode' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateRebalanceStableBorrowRate' in 'IncentivizedERC20' overrides 'Context::validateRebalanceStableBorrowRate' which is not marked 'virtual' |
| warning | W200 | function 'validateRebalanceStableBorrowRate' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSetUseReserveAsCollateral' in 'IncentivizedERC20' overrides 'Context::validateSetUseReserveAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateSetUseReserveAsCollateral' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateFlashloan' in 'IncentivizedERC20' overrides 'Context::validateFlashloan' which is not marked 'virtual' |
| warning | W200 | function 'validateFlashloan' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateFlashloanSimple' in 'IncentivizedERC20' overrides 'Context::validateFlashloanSimple' which is not marked 'virtual' |
| warning | W200 | function 'validateFlashloanSimple' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateLiquidationCall' in 'IncentivizedERC20' overrides 'Context::validateLiquidationCall' which is not marked 'virtual' |
| warning | W200 | function 'validateLiquidationCall' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHealthFactor' in 'IncentivizedERC20' overrides 'Context::validateHealthFactor' which is not marked 'virtual' |
| warning | W200 | function 'validateHealthFactor' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHFAndLtv' in 'IncentivizedERC20' overrides 'Context::validateHFAndLtv' which is not marked 'virtual' |
| warning | W200 | function 'validateHFAndLtv' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateTransfer' in 'IncentivizedERC20' overrides 'Context::validateTransfer' which is not marked 'virtual' |
| warning | W200 | function 'validateTransfer' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateDropReserve' in 'IncentivizedERC20' overrides 'Context::validateDropReserve' which is not marked 'virtual' |
| warning | W200 | function 'validateDropReserve' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSetUserEMode' in 'IncentivizedERC20' overrides 'Context::validateSetUserEMode' which is not marked 'virtual' |
| warning | W200 | function 'validateSetUserEMode' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateUseAsCollateral' in 'IncentivizedERC20' overrides 'Context::validateUseAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateUseAsCollateral' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateAutomaticUseAsCollateral' in 'IncentivizedERC20' overrides 'Context::validateAutomaticUseAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateAutomaticUseAsCollateral' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeInitReserve' in 'IncentivizedERC20' overrides 'Context::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRescueTokens' in 'IncentivizedERC20' overrides 'Context::executeRescueTokens' which is not marked 'virtual' |
| warning | W200 | function 'executeRescueTokens' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeMintToTreasury' in 'IncentivizedERC20' overrides 'Context::executeMintToTreasury' which is not marked 'virtual' |
| warning | W200 | function 'executeMintToTreasury' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeResetIsolationModeTotalDebt' in 'IncentivizedERC20' overrides 'Context::executeResetIsolationModeTotalDebt' which is not marked 'virtual' |
| warning | W200 | function 'executeResetIsolationModeTotalDebt' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeDropReserve' in 'IncentivizedERC20' overrides 'Context::executeDropReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeDropReserve' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeGetUserAccountData' in 'IncentivizedERC20' overrides 'Context::executeGetUserAccountData' which is not marked 'virtual' |
| warning | W200 | function 'executeGetUserAccountData' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSupply' in 'IncentivizedERC20' overrides 'Context::executeSupply' which is not marked 'virtual' |
| warning | W200 | function 'executeSupply' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeWithdraw' in 'IncentivizedERC20' overrides 'Context::executeWithdraw' which is not marked 'virtual' |
| warning | W200 | function 'executeWithdraw' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFinalizeTransfer' in 'IncentivizedERC20' overrides 'Context::executeFinalizeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'executeFinalizeTransfer' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUseReserveAsCollateral' in 'IncentivizedERC20' overrides 'Context::executeUseReserveAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'executeUseReserveAsCollateral' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUserCurrentDebt' in 'IncentivizedERC20' overrides 'Context::getUserCurrentDebt' which is not marked 'virtual' |
| warning | W200 | function 'getUserCurrentDebt' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateIsolatedDebtIfIsolated' in 'IncentivizedERC20' overrides 'Context::updateIsolatedDebtIfIsolated' which is not marked 'virtual' |
| warning | W200 | function 'updateIsolatedDebtIfIsolated' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeBorrow' in 'IncentivizedERC20' overrides 'Context::executeBorrow' which is not marked 'virtual' |
| warning | W200 | function 'executeBorrow' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRepay' in 'IncentivizedERC20' overrides 'Context::executeRepay' which is not marked 'virtual' |
| warning | W200 | function 'executeRepay' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRebalanceStableBorrowRate' in 'IncentivizedERC20' overrides 'Context::executeRebalanceStableBorrowRate' which is not marked 'virtual' |
| warning | W200 | function 'executeRebalanceStableBorrowRate' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSwapBorrowRateMode' in 'IncentivizedERC20' overrides 'Context::executeSwapBorrowRateMode' which is not marked 'virtual' |
| warning | W200 | function 'executeSwapBorrowRateMode' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFlashLoan' in 'IncentivizedERC20' overrides 'Context::executeFlashLoan' which is not marked 'virtual' |
| warning | W200 | function 'executeFlashLoan' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFlashLoanSimple' in 'IncentivizedERC20' overrides 'Context::executeFlashLoanSimple' which is not marked 'virtual' |
| warning | W200 | function 'executeFlashLoanSimple' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_handleFlashLoanRepayment' in 'IncentivizedERC20' overrides 'Context::_handleFlashLoanRepayment' which is not marked 'virtual' |
| warning | W200 | function '_handleFlashLoanRepayment' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeLiquidationCall' in 'IncentivizedERC20' overrides 'Context::executeLiquidationCall' which is not marked 'virtual' |
| warning | W200 | function 'executeLiquidationCall' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_burnCollateralATokens' in 'IncentivizedERC20' overrides 'Context::_burnCollateralATokens' which is not marked 'virtual' |
| warning | W200 | function '_burnCollateralATokens' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_liquidateATokens' in 'IncentivizedERC20' overrides 'Context::_liquidateATokens' which is not marked 'virtual' |
| warning | W200 | function '_liquidateATokens' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_burnDebtTokens' in 'IncentivizedERC20' overrides 'Context::_burnDebtTokens' which is not marked 'virtual' |
| warning | W200 | function '_burnDebtTokens' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_calculateDebt' in 'IncentivizedERC20' overrides 'Context::_calculateDebt' which is not marked 'virtual' |
| warning | W200 | function '_calculateDebt' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getConfigurationData' in 'IncentivizedERC20' overrides 'Context::_getConfigurationData' which is not marked 'virtual' |
| warning | W200 | function '_getConfigurationData' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function '_calculateAvailableCollateralToLiquidate' in 'IncentivizedERC20' overrides 'Context::_calculateAvailableCollateralToLiquidate' which is not marked 'virtual' |
| warning | W200 | function '_calculateAvailableCollateralToLiquidate' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeMintUnbacked' in 'IncentivizedERC20' overrides 'Context::executeMintUnbacked' which is not marked 'virtual' |
| warning | W200 | function 'executeMintUnbacked' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeBackUnbacked' in 'IncentivizedERC20' overrides 'Context::executeBackUnbacked' which is not marked 'virtual' |
| warning | W200 | function 'executeBackUnbacked' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSupplyParams' in 'IncentivizedERC20' overrides 'Context::decodeSupplyParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSupplyParams' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSupplyWithPermitParams' in 'IncentivizedERC20' overrides 'Context::decodeSupplyWithPermitParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSupplyWithPermitParams' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeWithdrawParams' in 'IncentivizedERC20' overrides 'Context::decodeWithdrawParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeWithdrawParams' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBorrowParams' in 'IncentivizedERC20' overrides 'Context::decodeBorrowParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBorrowParams' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRepayParams' in 'IncentivizedERC20' overrides 'Context::decodeRepayParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRepayParams' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRepayWithPermitParams' in 'IncentivizedERC20' overrides 'Context::decodeRepayWithPermitParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRepayWithPermitParams' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapBorrowRateModeParams' in 'IncentivizedERC20' overrides 'Context::decodeSwapBorrowRateModeParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapBorrowRateModeParams' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRebalanceStableBorrowRateParams' in 'IncentivizedERC20' overrides 'Context::decodeRebalanceStableBorrowRateParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRebalanceStableBorrowRateParams' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSetUserUseReserveAsCollateralParams' in 'IncentivizedERC20' overrides 'Context::decodeSetUserUseReserveAsCollateralParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSetUserUseReserveAsCollateralParams' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeLiquidationCallParams' in 'IncentivizedERC20' overrides 'Context::decodeLiquidationCallParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeLiquidationCallParams' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'IncentivizedERC20' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'PoolStorage' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ISOLATED_COLLATERAL_SUPPLIER_ROLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEFAULT_LIQUIDATION_CLOSE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LIQUIDATION_CLOSE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_FACTOR_HF_THRESHOLD' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ISOLATED_COLLATERAL_SUPPLIER_ROLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEFAULT_LIQUIDATION_CLOSE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LIQUIDATION_CLOSE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_FACTOR_HF_THRESHOLD' detected while merging libraries |
| warning | W200 | function 'setLtv' in 'PoolStorage' overrides 'VersionedInitializable::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'PoolStorage' overrides 'VersionedInitializable::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'PoolStorage' overrides 'VersionedInitializable::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'PoolStorage' overrides 'VersionedInitializable::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'PoolStorage' overrides 'VersionedInitializable::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'PoolStorage' overrides 'VersionedInitializable::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'PoolStorage' overrides 'VersionedInitializable::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'PoolStorage' overrides 'VersionedInitializable::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'PoolStorage' overrides 'VersionedInitializable::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'PoolStorage' overrides 'VersionedInitializable::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'PoolStorage' overrides 'VersionedInitializable::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'PoolStorage' overrides 'VersionedInitializable::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'PoolStorage' overrides 'VersionedInitializable::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'PoolStorage' overrides 'VersionedInitializable::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'PoolStorage' overrides 'VersionedInitializable::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'PoolStorage' overrides 'VersionedInitializable::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'PoolStorage' overrides 'VersionedInitializable::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'PoolStorage' overrides 'VersionedInitializable::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'PoolStorage' overrides 'VersionedInitializable::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'PoolStorage' overrides 'VersionedInitializable::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'PoolStorage' overrides 'VersionedInitializable::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'PoolStorage' overrides 'VersionedInitializable::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'PoolStorage' overrides 'VersionedInitializable::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'PoolStorage' overrides 'VersionedInitializable::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'PoolStorage' overrides 'VersionedInitializable::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'PoolStorage' overrides 'VersionedInitializable::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'PoolStorage' overrides 'VersionedInitializable::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'PoolStorage' overrides 'VersionedInitializable::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'PoolStorage' overrides 'VersionedInitializable::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'PoolStorage' overrides 'VersionedInitializable::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'PoolStorage' overrides 'VersionedInitializable::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'PoolStorage' overrides 'VersionedInitializable::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'PoolStorage' overrides 'VersionedInitializable::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'PoolStorage' overrides 'VersionedInitializable::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'PoolStorage' overrides 'VersionedInitializable::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'PoolStorage' overrides 'VersionedInitializable::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'PoolStorage' overrides 'VersionedInitializable::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'PoolStorage' overrides 'VersionedInitializable::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'PoolStorage' overrides 'VersionedInitializable::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'PoolStorage' overrides 'VersionedInitializable::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'PoolStorage' overrides 'VersionedInitializable::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'PoolStorage' overrides 'VersionedInitializable::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'PoolStorage' overrides 'VersionedInitializable::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLastTransferResult' in 'PoolStorage' overrides 'VersionedInitializable::getLastTransferResult' which is not marked 'virtual' |
| warning | W200 | function 'getLastTransferResult' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'PoolStorage' overrides 'VersionedInitializable::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'PoolStorage' overrides 'VersionedInitializable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'PoolStorage' overrides 'VersionedInitializable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'PoolStorage' overrides 'VersionedInitializable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'PoolStorage' overrides 'VersionedInitializable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'PoolStorage' overrides 'VersionedInitializable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'PoolStorage' overrides 'VersionedInitializable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'PoolStorage' overrides 'VersionedInitializable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'PoolStorage' overrides 'VersionedInitializable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'PoolStorage' overrides 'VersionedInitializable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'PoolStorage' overrides 'VersionedInitializable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadMul' in 'PoolStorage' overrides 'VersionedInitializable::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'PoolStorage' overrides 'VersionedInitializable::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'PoolStorage' overrides 'VersionedInitializable::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'PoolStorage' overrides 'VersionedInitializable::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'PoolStorage' overrides 'VersionedInitializable::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'PoolStorage' overrides 'VersionedInitializable::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'PoolStorage' overrides 'VersionedInitializable::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'PoolStorage' overrides 'VersionedInitializable::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'PoolStorage' overrides 'VersionedInitializable::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'PoolStorage' overrides 'VersionedInitializable::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'PoolStorage' overrides 'VersionedInitializable::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'PoolStorage' overrides 'VersionedInitializable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'PoolStorage' overrides 'VersionedInitializable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'PoolStorage' overrides 'VersionedInitializable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'PoolStorage' overrides 'VersionedInitializable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'PoolStorage' overrides 'VersionedInitializable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'PoolStorage' overrides 'VersionedInitializable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'PoolStorage' overrides 'VersionedInitializable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'PoolStorage' overrides 'VersionedInitializable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'PoolStorage' overrides 'VersionedInitializable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'PoolStorage' overrides 'VersionedInitializable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'PoolStorage' overrides 'VersionedInitializable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'PoolStorage' overrides 'VersionedInitializable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'PoolStorage' overrides 'VersionedInitializable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'PoolStorage' overrides 'VersionedInitializable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNormalizedIncome' in 'PoolStorage' overrides 'VersionedInitializable::getNormalizedIncome' which is not marked 'virtual' |
| warning | W200 | function 'getNormalizedIncome' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNormalizedDebt' in 'PoolStorage' overrides 'VersionedInitializable::getNormalizedDebt' which is not marked 'virtual' |
| warning | W200 | function 'getNormalizedDebt' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateState' in 'PoolStorage' overrides 'VersionedInitializable::updateState' which is not marked 'virtual' |
| warning | W200 | function 'updateState' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'cumulateToLiquidityIndex' in 'PoolStorage' overrides 'VersionedInitializable::cumulateToLiquidityIndex' which is not marked 'virtual' |
| warning | W200 | function 'cumulateToLiquidityIndex' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'init' in 'PoolStorage' overrides 'VersionedInitializable::init' which is not marked 'virtual' |
| warning | W200 | function 'init' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateInterestRates' in 'PoolStorage' overrides 'VersionedInitializable::updateInterestRates' which is not marked 'virtual' |
| warning | W200 | function 'updateInterestRates' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_accrueToTreasury' in 'PoolStorage' overrides 'VersionedInitializable::_accrueToTreasury' which is not marked 'virtual' |
| warning | W200 | function '_accrueToTreasury' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_updateIndexes' in 'PoolStorage' overrides 'VersionedInitializable::_updateIndexes' which is not marked 'virtual' |
| warning | W200 | function '_updateIndexes' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'cache' in 'PoolStorage' overrides 'VersionedInitializable::cache' which is not marked 'virtual' |
| warning | W200 | function 'cache' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowing' in 'PoolStorage' overrides 'VersionedInitializable::setBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowing' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUsingAsCollateral' in 'PoolStorage' overrides 'VersionedInitializable::setUsingAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'setUsingAsCollateral' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralOrBorrowing' in 'PoolStorage' overrides 'VersionedInitializable::isUsingAsCollateralOrBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralOrBorrowing' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowing' in 'PoolStorage' overrides 'VersionedInitializable::isBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowing' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateral' in 'PoolStorage' overrides 'VersionedInitializable::isUsingAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateral' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralOne' in 'PoolStorage' overrides 'VersionedInitializable::isUsingAsCollateralOne' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralOne' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralAny' in 'PoolStorage' overrides 'VersionedInitializable::isUsingAsCollateralAny' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralAny' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowingOne' in 'PoolStorage' overrides 'VersionedInitializable::isBorrowingOne' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowingOne' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowingAny' in 'PoolStorage' overrides 'VersionedInitializable::isBorrowingAny' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowingAny' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isEmpty' in 'PoolStorage' overrides 'VersionedInitializable::isEmpty' which is not marked 'virtual' |
| warning | W200 | function 'isEmpty' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getIsolationModeState' in 'PoolStorage' overrides 'VersionedInitializable::getIsolationModeState' which is not marked 'virtual' |
| warning | W200 | function 'getIsolationModeState' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowingState' in 'PoolStorage' overrides 'VersionedInitializable::getSiloedBorrowingState' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowingState' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFirstAssetIdByMask' in 'PoolStorage' overrides 'VersionedInitializable::_getFirstAssetIdByMask' which is not marked 'virtual' |
| warning | W200 | function '_getFirstAssetIdByMask' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSetUserEMode' in 'PoolStorage' overrides 'VersionedInitializable::executeSetUserEMode' which is not marked 'virtual' |
| warning | W200 | function 'executeSetUserEMode' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeConfiguration' in 'PoolStorage' overrides 'VersionedInitializable::getEModeConfiguration' which is not marked 'virtual' |
| warning | W200 | function 'getEModeConfiguration' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isInEModeCategory' in 'PoolStorage' overrides 'VersionedInitializable::isInEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'isInEModeCategory' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateUserAccountData' in 'PoolStorage' overrides 'VersionedInitializable::calculateUserAccountData' which is not marked 'virtual' |
| warning | W200 | function 'calculateUserAccountData' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateAvailableBorrows' in 'PoolStorage' overrides 'VersionedInitializable::calculateAvailableBorrows' which is not marked 'virtual' |
| warning | W200 | function 'calculateAvailableBorrows' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getUserDebtInBaseCurrency' in 'PoolStorage' overrides 'VersionedInitializable::_getUserDebtInBaseCurrency' which is not marked 'virtual' |
| warning | W200 | function '_getUserDebtInBaseCurrency' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getUserBalanceInBaseCurrency' in 'PoolStorage' overrides 'VersionedInitializable::_getUserBalanceInBaseCurrency' which is not marked 'virtual' |
| warning | W200 | function '_getUserBalanceInBaseCurrency' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSupply' in 'PoolStorage' overrides 'VersionedInitializable::validateSupply' which is not marked 'virtual' |
| warning | W200 | function 'validateSupply' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateWithdraw' in 'PoolStorage' overrides 'VersionedInitializable::validateWithdraw' which is not marked 'virtual' |
| warning | W200 | function 'validateWithdraw' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateBorrow' in 'PoolStorage' overrides 'VersionedInitializable::validateBorrow' which is not marked 'virtual' |
| warning | W200 | function 'validateBorrow' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateRepay' in 'PoolStorage' overrides 'VersionedInitializable::validateRepay' which is not marked 'virtual' |
| warning | W200 | function 'validateRepay' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSwapRateMode' in 'PoolStorage' overrides 'VersionedInitializable::validateSwapRateMode' which is not marked 'virtual' |
| warning | W200 | function 'validateSwapRateMode' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateRebalanceStableBorrowRate' in 'PoolStorage' overrides 'VersionedInitializable::validateRebalanceStableBorrowRate' which is not marked 'virtual' |
| warning | W200 | function 'validateRebalanceStableBorrowRate' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSetUseReserveAsCollateral' in 'PoolStorage' overrides 'VersionedInitializable::validateSetUseReserveAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateSetUseReserveAsCollateral' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateFlashloan' in 'PoolStorage' overrides 'VersionedInitializable::validateFlashloan' which is not marked 'virtual' |
| warning | W200 | function 'validateFlashloan' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateFlashloanSimple' in 'PoolStorage' overrides 'VersionedInitializable::validateFlashloanSimple' which is not marked 'virtual' |
| warning | W200 | function 'validateFlashloanSimple' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateLiquidationCall' in 'PoolStorage' overrides 'VersionedInitializable::validateLiquidationCall' which is not marked 'virtual' |
| warning | W200 | function 'validateLiquidationCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHealthFactor' in 'PoolStorage' overrides 'VersionedInitializable::validateHealthFactor' which is not marked 'virtual' |
| warning | W200 | function 'validateHealthFactor' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHFAndLtv' in 'PoolStorage' overrides 'VersionedInitializable::validateHFAndLtv' which is not marked 'virtual' |
| warning | W200 | function 'validateHFAndLtv' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateTransfer' in 'PoolStorage' overrides 'VersionedInitializable::validateTransfer' which is not marked 'virtual' |
| warning | W200 | function 'validateTransfer' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateDropReserve' in 'PoolStorage' overrides 'VersionedInitializable::validateDropReserve' which is not marked 'virtual' |
| warning | W200 | function 'validateDropReserve' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSetUserEMode' in 'PoolStorage' overrides 'VersionedInitializable::validateSetUserEMode' which is not marked 'virtual' |
| warning | W200 | function 'validateSetUserEMode' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateUseAsCollateral' in 'PoolStorage' overrides 'VersionedInitializable::validateUseAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateUseAsCollateral' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateAutomaticUseAsCollateral' in 'PoolStorage' overrides 'VersionedInitializable::validateAutomaticUseAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateAutomaticUseAsCollateral' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeInitReserve' in 'PoolStorage' overrides 'VersionedInitializable::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRescueTokens' in 'PoolStorage' overrides 'VersionedInitializable::executeRescueTokens' which is not marked 'virtual' |
| warning | W200 | function 'executeRescueTokens' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeMintToTreasury' in 'PoolStorage' overrides 'VersionedInitializable::executeMintToTreasury' which is not marked 'virtual' |
| warning | W200 | function 'executeMintToTreasury' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeResetIsolationModeTotalDebt' in 'PoolStorage' overrides 'VersionedInitializable::executeResetIsolationModeTotalDebt' which is not marked 'virtual' |
| warning | W200 | function 'executeResetIsolationModeTotalDebt' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeDropReserve' in 'PoolStorage' overrides 'VersionedInitializable::executeDropReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeDropReserve' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeGetUserAccountData' in 'PoolStorage' overrides 'VersionedInitializable::executeGetUserAccountData' which is not marked 'virtual' |
| warning | W200 | function 'executeGetUserAccountData' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSupply' in 'PoolStorage' overrides 'VersionedInitializable::executeSupply' which is not marked 'virtual' |
| warning | W200 | function 'executeSupply' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeWithdraw' in 'PoolStorage' overrides 'VersionedInitializable::executeWithdraw' which is not marked 'virtual' |
| warning | W200 | function 'executeWithdraw' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFinalizeTransfer' in 'PoolStorage' overrides 'VersionedInitializable::executeFinalizeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'executeFinalizeTransfer' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUseReserveAsCollateral' in 'PoolStorage' overrides 'VersionedInitializable::executeUseReserveAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'executeUseReserveAsCollateral' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUserCurrentDebt' in 'PoolStorage' overrides 'VersionedInitializable::getUserCurrentDebt' which is not marked 'virtual' |
| warning | W200 | function 'getUserCurrentDebt' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateIsolatedDebtIfIsolated' in 'PoolStorage' overrides 'VersionedInitializable::updateIsolatedDebtIfIsolated' which is not marked 'virtual' |
| warning | W200 | function 'updateIsolatedDebtIfIsolated' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeBorrow' in 'PoolStorage' overrides 'VersionedInitializable::executeBorrow' which is not marked 'virtual' |
| warning | W200 | function 'executeBorrow' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRepay' in 'PoolStorage' overrides 'VersionedInitializable::executeRepay' which is not marked 'virtual' |
| warning | W200 | function 'executeRepay' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRebalanceStableBorrowRate' in 'PoolStorage' overrides 'VersionedInitializable::executeRebalanceStableBorrowRate' which is not marked 'virtual' |
| warning | W200 | function 'executeRebalanceStableBorrowRate' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSwapBorrowRateMode' in 'PoolStorage' overrides 'VersionedInitializable::executeSwapBorrowRateMode' which is not marked 'virtual' |
| warning | W200 | function 'executeSwapBorrowRateMode' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFlashLoan' in 'PoolStorage' overrides 'VersionedInitializable::executeFlashLoan' which is not marked 'virtual' |
| warning | W200 | function 'executeFlashLoan' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFlashLoanSimple' in 'PoolStorage' overrides 'VersionedInitializable::executeFlashLoanSimple' which is not marked 'virtual' |
| warning | W200 | function 'executeFlashLoanSimple' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_handleFlashLoanRepayment' in 'PoolStorage' overrides 'VersionedInitializable::_handleFlashLoanRepayment' which is not marked 'virtual' |
| warning | W200 | function '_handleFlashLoanRepayment' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeLiquidationCall' in 'PoolStorage' overrides 'VersionedInitializable::executeLiquidationCall' which is not marked 'virtual' |
| warning | W200 | function 'executeLiquidationCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_burnCollateralATokens' in 'PoolStorage' overrides 'VersionedInitializable::_burnCollateralATokens' which is not marked 'virtual' |
| warning | W200 | function '_burnCollateralATokens' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_liquidateATokens' in 'PoolStorage' overrides 'VersionedInitializable::_liquidateATokens' which is not marked 'virtual' |
| warning | W200 | function '_liquidateATokens' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_burnDebtTokens' in 'PoolStorage' overrides 'VersionedInitializable::_burnDebtTokens' which is not marked 'virtual' |
| warning | W200 | function '_burnDebtTokens' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_calculateDebt' in 'PoolStorage' overrides 'VersionedInitializable::_calculateDebt' which is not marked 'virtual' |
| warning | W200 | function '_calculateDebt' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getConfigurationData' in 'PoolStorage' overrides 'VersionedInitializable::_getConfigurationData' which is not marked 'virtual' |
| warning | W200 | function '_getConfigurationData' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_calculateAvailableCollateralToLiquidate' in 'PoolStorage' overrides 'VersionedInitializable::_calculateAvailableCollateralToLiquidate' which is not marked 'virtual' |
| warning | W200 | function '_calculateAvailableCollateralToLiquidate' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeMintUnbacked' in 'PoolStorage' overrides 'VersionedInitializable::executeMintUnbacked' which is not marked 'virtual' |
| warning | W200 | function 'executeMintUnbacked' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeBackUnbacked' in 'PoolStorage' overrides 'VersionedInitializable::executeBackUnbacked' which is not marked 'virtual' |
| warning | W200 | function 'executeBackUnbacked' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSupplyParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeSupplyParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSupplyParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSupplyWithPermitParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeSupplyWithPermitParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSupplyWithPermitParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeWithdrawParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeWithdrawParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeWithdrawParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBorrowParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeBorrowParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBorrowParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRepayParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeRepayParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRepayParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRepayWithPermitParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeRepayWithPermitParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRepayWithPermitParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapBorrowRateModeParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeSwapBorrowRateModeParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapBorrowRateModeParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRebalanceStableBorrowRateParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeRebalanceStableBorrowRateParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRebalanceStableBorrowRateParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSetUserUseReserveAsCollateralParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeSetUserUseReserveAsCollateralParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSetUserUseReserveAsCollateralParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeLiquidationCallParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeLiquidationCallParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeLiquidationCallParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLtv' in 'Pool' overrides 'PoolStorage::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'Pool' overrides 'PoolStorage::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'Pool' overrides 'PoolStorage::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'Pool' overrides 'PoolStorage::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'Pool' overrides 'PoolStorage::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'Pool' overrides 'PoolStorage::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'Pool' overrides 'PoolStorage::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'Pool' overrides 'PoolStorage::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'Pool' overrides 'PoolStorage::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'Pool' overrides 'PoolStorage::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'Pool' overrides 'PoolStorage::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'Pool' overrides 'PoolStorage::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'Pool' overrides 'PoolStorage::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'Pool' overrides 'PoolStorage::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'Pool' overrides 'PoolStorage::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'Pool' overrides 'PoolStorage::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'Pool' overrides 'PoolStorage::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'Pool' overrides 'PoolStorage::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'Pool' overrides 'PoolStorage::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'Pool' overrides 'PoolStorage::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'Pool' overrides 'PoolStorage::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'Pool' overrides 'PoolStorage::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'Pool' overrides 'PoolStorage::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'Pool' overrides 'PoolStorage::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'Pool' overrides 'PoolStorage::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'Pool' overrides 'PoolStorage::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'Pool' overrides 'PoolStorage::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'Pool' overrides 'PoolStorage::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'Pool' overrides 'PoolStorage::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'Pool' overrides 'PoolStorage::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'Pool' overrides 'PoolStorage::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'Pool' overrides 'PoolStorage::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'Pool' overrides 'PoolStorage::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'Pool' overrides 'PoolStorage::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'Pool' overrides 'PoolStorage::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'Pool' overrides 'PoolStorage::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'Pool' overrides 'PoolStorage::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'Pool' overrides 'PoolStorage::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'Pool' overrides 'PoolStorage::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'Pool' overrides 'PoolStorage::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'Pool' overrides 'PoolStorage::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'Pool' overrides 'PoolStorage::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'Pool' overrides 'PoolStorage::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLastTransferResult' in 'Pool' overrides 'PoolStorage::getLastTransferResult' which is not marked 'virtual' |
| warning | W200 | function 'getLastTransferResult' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'Pool' overrides 'PoolStorage::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Pool' overrides 'PoolStorage::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Pool' overrides 'PoolStorage::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Pool' overrides 'PoolStorage::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Pool' overrides 'PoolStorage::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Pool' overrides 'PoolStorage::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Pool' overrides 'PoolStorage::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Pool' overrides 'PoolStorage::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Pool' overrides 'PoolStorage::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Pool' overrides 'PoolStorage::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Pool' overrides 'PoolStorage::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadMul' in 'Pool' overrides 'PoolStorage::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'Pool' overrides 'PoolStorage::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'Pool' overrides 'PoolStorage::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'Pool' overrides 'PoolStorage::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'Pool' overrides 'PoolStorage::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'Pool' overrides 'PoolStorage::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'Pool' overrides 'PoolStorage::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Pool' overrides 'PoolStorage::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Pool' overrides 'PoolStorage::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'Pool' overrides 'PoolStorage::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'Pool' overrides 'PoolStorage::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Pool' overrides 'PoolStorage::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Pool' overrides 'PoolStorage::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Pool' overrides 'PoolStorage::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Pool' overrides 'PoolStorage::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Pool' overrides 'PoolStorage::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Pool' overrides 'PoolStorage::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Pool' overrides 'PoolStorage::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Pool' overrides 'PoolStorage::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Pool' overrides 'PoolStorage::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Pool' overrides 'PoolStorage::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Pool' overrides 'PoolStorage::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Pool' overrides 'PoolStorage::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Pool' overrides 'PoolStorage::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Pool' overrides 'PoolStorage::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNormalizedIncome' in 'Pool' overrides 'PoolStorage::getNormalizedIncome' which is not marked 'virtual' |
| warning | W200 | function 'getNormalizedIncome' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNormalizedDebt' in 'Pool' overrides 'PoolStorage::getNormalizedDebt' which is not marked 'virtual' |
| warning | W200 | function 'getNormalizedDebt' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateState' in 'Pool' overrides 'PoolStorage::updateState' which is not marked 'virtual' |
| warning | W200 | function 'updateState' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'cumulateToLiquidityIndex' in 'Pool' overrides 'PoolStorage::cumulateToLiquidityIndex' which is not marked 'virtual' |
| warning | W200 | function 'cumulateToLiquidityIndex' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'init' in 'Pool' overrides 'PoolStorage::init' which is not marked 'virtual' |
| warning | W200 | function 'init' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateInterestRates' in 'Pool' overrides 'PoolStorage::updateInterestRates' which is not marked 'virtual' |
| warning | W200 | function 'updateInterestRates' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_accrueToTreasury' in 'Pool' overrides 'PoolStorage::_accrueToTreasury' which is not marked 'virtual' |
| warning | W200 | function '_accrueToTreasury' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_updateIndexes' in 'Pool' overrides 'PoolStorage::_updateIndexes' which is not marked 'virtual' |
| warning | W200 | function '_updateIndexes' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'cache' in 'Pool' overrides 'PoolStorage::cache' which is not marked 'virtual' |
| warning | W200 | function 'cache' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowing' in 'Pool' overrides 'PoolStorage::setBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowing' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUsingAsCollateral' in 'Pool' overrides 'PoolStorage::setUsingAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'setUsingAsCollateral' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralOrBorrowing' in 'Pool' overrides 'PoolStorage::isUsingAsCollateralOrBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralOrBorrowing' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowing' in 'Pool' overrides 'PoolStorage::isBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowing' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateral' in 'Pool' overrides 'PoolStorage::isUsingAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateral' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralOne' in 'Pool' overrides 'PoolStorage::isUsingAsCollateralOne' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralOne' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralAny' in 'Pool' overrides 'PoolStorage::isUsingAsCollateralAny' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralAny' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowingOne' in 'Pool' overrides 'PoolStorage::isBorrowingOne' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowingOne' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowingAny' in 'Pool' overrides 'PoolStorage::isBorrowingAny' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowingAny' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isEmpty' in 'Pool' overrides 'PoolStorage::isEmpty' which is not marked 'virtual' |
| warning | W200 | function 'isEmpty' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getIsolationModeState' in 'Pool' overrides 'PoolStorage::getIsolationModeState' which is not marked 'virtual' |
| warning | W200 | function 'getIsolationModeState' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowingState' in 'Pool' overrides 'PoolStorage::getSiloedBorrowingState' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowingState' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFirstAssetIdByMask' in 'Pool' overrides 'PoolStorage::_getFirstAssetIdByMask' which is not marked 'virtual' |
| warning | W200 | function '_getFirstAssetIdByMask' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSetUserEMode' in 'Pool' overrides 'PoolStorage::executeSetUserEMode' which is not marked 'virtual' |
| warning | W200 | function 'executeSetUserEMode' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeConfiguration' in 'Pool' overrides 'PoolStorage::getEModeConfiguration' which is not marked 'virtual' |
| warning | W200 | function 'getEModeConfiguration' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isInEModeCategory' in 'Pool' overrides 'PoolStorage::isInEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'isInEModeCategory' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateUserAccountData' in 'Pool' overrides 'PoolStorage::calculateUserAccountData' which is not marked 'virtual' |
| warning | W200 | function 'calculateUserAccountData' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateAvailableBorrows' in 'Pool' overrides 'PoolStorage::calculateAvailableBorrows' which is not marked 'virtual' |
| warning | W200 | function 'calculateAvailableBorrows' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getUserDebtInBaseCurrency' in 'Pool' overrides 'PoolStorage::_getUserDebtInBaseCurrency' which is not marked 'virtual' |
| warning | W200 | function '_getUserDebtInBaseCurrency' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getUserBalanceInBaseCurrency' in 'Pool' overrides 'PoolStorage::_getUserBalanceInBaseCurrency' which is not marked 'virtual' |
| warning | W200 | function '_getUserBalanceInBaseCurrency' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSupply' in 'Pool' overrides 'PoolStorage::validateSupply' which is not marked 'virtual' |
| warning | W200 | function 'validateSupply' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateWithdraw' in 'Pool' overrides 'PoolStorage::validateWithdraw' which is not marked 'virtual' |
| warning | W200 | function 'validateWithdraw' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateBorrow' in 'Pool' overrides 'PoolStorage::validateBorrow' which is not marked 'virtual' |
| warning | W200 | function 'validateBorrow' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateRepay' in 'Pool' overrides 'PoolStorage::validateRepay' which is not marked 'virtual' |
| warning | W200 | function 'validateRepay' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSwapRateMode' in 'Pool' overrides 'PoolStorage::validateSwapRateMode' which is not marked 'virtual' |
| warning | W200 | function 'validateSwapRateMode' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateRebalanceStableBorrowRate' in 'Pool' overrides 'PoolStorage::validateRebalanceStableBorrowRate' which is not marked 'virtual' |
| warning | W200 | function 'validateRebalanceStableBorrowRate' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSetUseReserveAsCollateral' in 'Pool' overrides 'PoolStorage::validateSetUseReserveAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateSetUseReserveAsCollateral' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateFlashloan' in 'Pool' overrides 'PoolStorage::validateFlashloan' which is not marked 'virtual' |
| warning | W200 | function 'validateFlashloan' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateFlashloanSimple' in 'Pool' overrides 'PoolStorage::validateFlashloanSimple' which is not marked 'virtual' |
| warning | W200 | function 'validateFlashloanSimple' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateLiquidationCall' in 'Pool' overrides 'PoolStorage::validateLiquidationCall' which is not marked 'virtual' |
| warning | W200 | function 'validateLiquidationCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHealthFactor' in 'Pool' overrides 'PoolStorage::validateHealthFactor' which is not marked 'virtual' |
| warning | W200 | function 'validateHealthFactor' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHFAndLtv' in 'Pool' overrides 'PoolStorage::validateHFAndLtv' which is not marked 'virtual' |
| warning | W200 | function 'validateHFAndLtv' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateTransfer' in 'Pool' overrides 'PoolStorage::validateTransfer' which is not marked 'virtual' |
| warning | W200 | function 'validateTransfer' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateDropReserve' in 'Pool' overrides 'PoolStorage::validateDropReserve' which is not marked 'virtual' |
| warning | W200 | function 'validateDropReserve' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSetUserEMode' in 'Pool' overrides 'PoolStorage::validateSetUserEMode' which is not marked 'virtual' |
| warning | W200 | function 'validateSetUserEMode' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateUseAsCollateral' in 'Pool' overrides 'PoolStorage::validateUseAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateUseAsCollateral' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateAutomaticUseAsCollateral' in 'Pool' overrides 'PoolStorage::validateAutomaticUseAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateAutomaticUseAsCollateral' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeInitReserve' in 'Pool' overrides 'PoolStorage::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRescueTokens' in 'Pool' overrides 'PoolStorage::executeRescueTokens' which is not marked 'virtual' |
| warning | W200 | function 'executeRescueTokens' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeMintToTreasury' in 'Pool' overrides 'PoolStorage::executeMintToTreasury' which is not marked 'virtual' |
| warning | W200 | function 'executeMintToTreasury' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeResetIsolationModeTotalDebt' in 'Pool' overrides 'PoolStorage::executeResetIsolationModeTotalDebt' which is not marked 'virtual' |
| warning | W200 | function 'executeResetIsolationModeTotalDebt' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeDropReserve' in 'Pool' overrides 'PoolStorage::executeDropReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeDropReserve' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeGetUserAccountData' in 'Pool' overrides 'PoolStorage::executeGetUserAccountData' which is not marked 'virtual' |
| warning | W200 | function 'executeGetUserAccountData' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSupply' in 'Pool' overrides 'PoolStorage::executeSupply' which is not marked 'virtual' |
| warning | W200 | function 'executeSupply' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeWithdraw' in 'Pool' overrides 'PoolStorage::executeWithdraw' which is not marked 'virtual' |
| warning | W200 | function 'executeWithdraw' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFinalizeTransfer' in 'Pool' overrides 'PoolStorage::executeFinalizeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'executeFinalizeTransfer' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUseReserveAsCollateral' in 'Pool' overrides 'PoolStorage::executeUseReserveAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'executeUseReserveAsCollateral' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUserCurrentDebt' in 'Pool' overrides 'PoolStorage::getUserCurrentDebt' which is not marked 'virtual' |
| warning | W200 | function 'getUserCurrentDebt' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateIsolatedDebtIfIsolated' in 'Pool' overrides 'PoolStorage::updateIsolatedDebtIfIsolated' which is not marked 'virtual' |
| warning | W200 | function 'updateIsolatedDebtIfIsolated' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeBorrow' in 'Pool' overrides 'PoolStorage::executeBorrow' which is not marked 'virtual' |
| warning | W200 | function 'executeBorrow' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRepay' in 'Pool' overrides 'PoolStorage::executeRepay' which is not marked 'virtual' |
| warning | W200 | function 'executeRepay' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRebalanceStableBorrowRate' in 'Pool' overrides 'PoolStorage::executeRebalanceStableBorrowRate' which is not marked 'virtual' |
| warning | W200 | function 'executeRebalanceStableBorrowRate' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSwapBorrowRateMode' in 'Pool' overrides 'PoolStorage::executeSwapBorrowRateMode' which is not marked 'virtual' |
| warning | W200 | function 'executeSwapBorrowRateMode' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFlashLoan' in 'Pool' overrides 'PoolStorage::executeFlashLoan' which is not marked 'virtual' |
| warning | W200 | function 'executeFlashLoan' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFlashLoanSimple' in 'Pool' overrides 'PoolStorage::executeFlashLoanSimple' which is not marked 'virtual' |
| warning | W200 | function 'executeFlashLoanSimple' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_handleFlashLoanRepayment' in 'Pool' overrides 'PoolStorage::_handleFlashLoanRepayment' which is not marked 'virtual' |
| warning | W200 | function '_handleFlashLoanRepayment' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeLiquidationCall' in 'Pool' overrides 'PoolStorage::executeLiquidationCall' which is not marked 'virtual' |
| warning | W200 | function 'executeLiquidationCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_burnCollateralATokens' in 'Pool' overrides 'PoolStorage::_burnCollateralATokens' which is not marked 'virtual' |
| warning | W200 | function '_burnCollateralATokens' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_liquidateATokens' in 'Pool' overrides 'PoolStorage::_liquidateATokens' which is not marked 'virtual' |
| warning | W200 | function '_liquidateATokens' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_burnDebtTokens' in 'Pool' overrides 'PoolStorage::_burnDebtTokens' which is not marked 'virtual' |
| warning | W200 | function '_burnDebtTokens' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_calculateDebt' in 'Pool' overrides 'PoolStorage::_calculateDebt' which is not marked 'virtual' |
| warning | W200 | function '_calculateDebt' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getConfigurationData' in 'Pool' overrides 'PoolStorage::_getConfigurationData' which is not marked 'virtual' |
| warning | W200 | function '_getConfigurationData' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_calculateAvailableCollateralToLiquidate' in 'Pool' overrides 'PoolStorage::_calculateAvailableCollateralToLiquidate' which is not marked 'virtual' |
| warning | W200 | function '_calculateAvailableCollateralToLiquidate' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeMintUnbacked' in 'Pool' overrides 'PoolStorage::executeMintUnbacked' which is not marked 'virtual' |
| warning | W200 | function 'executeMintUnbacked' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeBackUnbacked' in 'Pool' overrides 'PoolStorage::executeBackUnbacked' which is not marked 'virtual' |
| warning | W200 | function 'executeBackUnbacked' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSupplyParams' in 'Pool' overrides 'PoolStorage::decodeSupplyParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSupplyParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSupplyWithPermitParams' in 'Pool' overrides 'PoolStorage::decodeSupplyWithPermitParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSupplyWithPermitParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeWithdrawParams' in 'Pool' overrides 'PoolStorage::decodeWithdrawParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeWithdrawParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBorrowParams' in 'Pool' overrides 'PoolStorage::decodeBorrowParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBorrowParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRepayParams' in 'Pool' overrides 'PoolStorage::decodeRepayParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRepayParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRepayWithPermitParams' in 'Pool' overrides 'PoolStorage::decodeRepayWithPermitParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRepayWithPermitParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapBorrowRateModeParams' in 'Pool' overrides 'PoolStorage::decodeSwapBorrowRateModeParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapBorrowRateModeParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRebalanceStableBorrowRateParams' in 'Pool' overrides 'PoolStorage::decodeRebalanceStableBorrowRateParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRebalanceStableBorrowRateParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSetUserUseReserveAsCollateralParams' in 'Pool' overrides 'PoolStorage::decodeSetUserUseReserveAsCollateralParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSetUserUseReserveAsCollateralParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeLiquidationCallParams' in 'Pool' overrides 'PoolStorage::decodeLiquidationCallParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeLiquidationCallParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Pool' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ISOLATED_COLLATERAL_SUPPLIER_ROLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEFAULT_LIQUIDATION_CLOSE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LIQUIDATION_CLOSE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_FACTOR_HF_THRESHOLD' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ISOLATED_COLLATERAL_SUPPLIER_ROLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEFAULT_LIQUIDATION_CLOSE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LIQUIDATION_CLOSE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_FACTOR_HF_THRESHOLD' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_PERCENTAGE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BORROWING_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'COLLATERAL_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEALTH_FACTOR_LIQUIDATION_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ISOLATED_COLLATERAL_SUPPLIER_ROLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEFAULT_LIQUIDATION_CLOSE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LIQUIDATION_CLOSE_FACTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_FACTOR_HF_THRESHOLD' detected while merging libraries |
| warning | W200 | function 'setLtv' in 'PoolStorage' overrides 'VersionedInitializable::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'PoolStorage' overrides 'VersionedInitializable::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'PoolStorage' overrides 'VersionedInitializable::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'PoolStorage' overrides 'VersionedInitializable::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'PoolStorage' overrides 'VersionedInitializable::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'PoolStorage' overrides 'VersionedInitializable::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'PoolStorage' overrides 'VersionedInitializable::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'PoolStorage' overrides 'VersionedInitializable::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'PoolStorage' overrides 'VersionedInitializable::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'PoolStorage' overrides 'VersionedInitializable::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'PoolStorage' overrides 'VersionedInitializable::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'PoolStorage' overrides 'VersionedInitializable::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'PoolStorage' overrides 'VersionedInitializable::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'PoolStorage' overrides 'VersionedInitializable::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'PoolStorage' overrides 'VersionedInitializable::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'PoolStorage' overrides 'VersionedInitializable::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'PoolStorage' overrides 'VersionedInitializable::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'PoolStorage' overrides 'VersionedInitializable::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'PoolStorage' overrides 'VersionedInitializable::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'PoolStorage' overrides 'VersionedInitializable::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'PoolStorage' overrides 'VersionedInitializable::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'PoolStorage' overrides 'VersionedInitializable::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'PoolStorage' overrides 'VersionedInitializable::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'PoolStorage' overrides 'VersionedInitializable::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'PoolStorage' overrides 'VersionedInitializable::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'PoolStorage' overrides 'VersionedInitializable::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'PoolStorage' overrides 'VersionedInitializable::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'PoolStorage' overrides 'VersionedInitializable::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'PoolStorage' overrides 'VersionedInitializable::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'PoolStorage' overrides 'VersionedInitializable::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'PoolStorage' overrides 'VersionedInitializable::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'PoolStorage' overrides 'VersionedInitializable::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'PoolStorage' overrides 'VersionedInitializable::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'PoolStorage' overrides 'VersionedInitializable::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'PoolStorage' overrides 'VersionedInitializable::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'PoolStorage' overrides 'VersionedInitializable::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'PoolStorage' overrides 'VersionedInitializable::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'PoolStorage' overrides 'VersionedInitializable::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'PoolStorage' overrides 'VersionedInitializable::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'PoolStorage' overrides 'VersionedInitializable::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'PoolStorage' overrides 'VersionedInitializable::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'PoolStorage' overrides 'VersionedInitializable::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'PoolStorage' overrides 'VersionedInitializable::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLastTransferResult' in 'PoolStorage' overrides 'VersionedInitializable::getLastTransferResult' which is not marked 'virtual' |
| warning | W200 | function 'getLastTransferResult' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'PoolStorage' overrides 'VersionedInitializable::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'PoolStorage' overrides 'VersionedInitializable::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'PoolStorage' overrides 'VersionedInitializable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'PoolStorage' overrides 'VersionedInitializable::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'PoolStorage' overrides 'VersionedInitializable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'PoolStorage' overrides 'VersionedInitializable::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'PoolStorage' overrides 'VersionedInitializable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'PoolStorage' overrides 'VersionedInitializable::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'PoolStorage' overrides 'VersionedInitializable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'PoolStorage' overrides 'VersionedInitializable::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'PoolStorage' overrides 'VersionedInitializable::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadMul' in 'PoolStorage' overrides 'VersionedInitializable::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'PoolStorage' overrides 'VersionedInitializable::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'PoolStorage' overrides 'VersionedInitializable::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'PoolStorage' overrides 'VersionedInitializable::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'PoolStorage' overrides 'VersionedInitializable::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'PoolStorage' overrides 'VersionedInitializable::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'PoolStorage' overrides 'VersionedInitializable::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'PoolStorage' overrides 'VersionedInitializable::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'PoolStorage' overrides 'VersionedInitializable::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'PoolStorage' overrides 'VersionedInitializable::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'PoolStorage' overrides 'VersionedInitializable::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'PoolStorage' overrides 'VersionedInitializable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'PoolStorage' overrides 'VersionedInitializable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'PoolStorage' overrides 'VersionedInitializable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'PoolStorage' overrides 'VersionedInitializable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'PoolStorage' overrides 'VersionedInitializable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'PoolStorage' overrides 'VersionedInitializable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'PoolStorage' overrides 'VersionedInitializable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'PoolStorage' overrides 'VersionedInitializable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'PoolStorage' overrides 'VersionedInitializable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'PoolStorage' overrides 'VersionedInitializable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'PoolStorage' overrides 'VersionedInitializable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'PoolStorage' overrides 'VersionedInitializable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'PoolStorage' overrides 'VersionedInitializable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'PoolStorage' overrides 'VersionedInitializable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNormalizedIncome' in 'PoolStorage' overrides 'VersionedInitializable::getNormalizedIncome' which is not marked 'virtual' |
| warning | W200 | function 'getNormalizedIncome' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNormalizedDebt' in 'PoolStorage' overrides 'VersionedInitializable::getNormalizedDebt' which is not marked 'virtual' |
| warning | W200 | function 'getNormalizedDebt' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateState' in 'PoolStorage' overrides 'VersionedInitializable::updateState' which is not marked 'virtual' |
| warning | W200 | function 'updateState' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'cumulateToLiquidityIndex' in 'PoolStorage' overrides 'VersionedInitializable::cumulateToLiquidityIndex' which is not marked 'virtual' |
| warning | W200 | function 'cumulateToLiquidityIndex' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'init' in 'PoolStorage' overrides 'VersionedInitializable::init' which is not marked 'virtual' |
| warning | W200 | function 'init' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateInterestRates' in 'PoolStorage' overrides 'VersionedInitializable::updateInterestRates' which is not marked 'virtual' |
| warning | W200 | function 'updateInterestRates' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_accrueToTreasury' in 'PoolStorage' overrides 'VersionedInitializable::_accrueToTreasury' which is not marked 'virtual' |
| warning | W200 | function '_accrueToTreasury' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_updateIndexes' in 'PoolStorage' overrides 'VersionedInitializable::_updateIndexes' which is not marked 'virtual' |
| warning | W200 | function '_updateIndexes' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'cache' in 'PoolStorage' overrides 'VersionedInitializable::cache' which is not marked 'virtual' |
| warning | W200 | function 'cache' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowing' in 'PoolStorage' overrides 'VersionedInitializable::setBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowing' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUsingAsCollateral' in 'PoolStorage' overrides 'VersionedInitializable::setUsingAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'setUsingAsCollateral' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralOrBorrowing' in 'PoolStorage' overrides 'VersionedInitializable::isUsingAsCollateralOrBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralOrBorrowing' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowing' in 'PoolStorage' overrides 'VersionedInitializable::isBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowing' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateral' in 'PoolStorage' overrides 'VersionedInitializable::isUsingAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateral' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralOne' in 'PoolStorage' overrides 'VersionedInitializable::isUsingAsCollateralOne' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralOne' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralAny' in 'PoolStorage' overrides 'VersionedInitializable::isUsingAsCollateralAny' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralAny' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowingOne' in 'PoolStorage' overrides 'VersionedInitializable::isBorrowingOne' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowingOne' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowingAny' in 'PoolStorage' overrides 'VersionedInitializable::isBorrowingAny' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowingAny' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isEmpty' in 'PoolStorage' overrides 'VersionedInitializable::isEmpty' which is not marked 'virtual' |
| warning | W200 | function 'isEmpty' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getIsolationModeState' in 'PoolStorage' overrides 'VersionedInitializable::getIsolationModeState' which is not marked 'virtual' |
| warning | W200 | function 'getIsolationModeState' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowingState' in 'PoolStorage' overrides 'VersionedInitializable::getSiloedBorrowingState' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowingState' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFirstAssetIdByMask' in 'PoolStorage' overrides 'VersionedInitializable::_getFirstAssetIdByMask' which is not marked 'virtual' |
| warning | W200 | function '_getFirstAssetIdByMask' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSetUserEMode' in 'PoolStorage' overrides 'VersionedInitializable::executeSetUserEMode' which is not marked 'virtual' |
| warning | W200 | function 'executeSetUserEMode' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeConfiguration' in 'PoolStorage' overrides 'VersionedInitializable::getEModeConfiguration' which is not marked 'virtual' |
| warning | W200 | function 'getEModeConfiguration' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isInEModeCategory' in 'PoolStorage' overrides 'VersionedInitializable::isInEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'isInEModeCategory' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateUserAccountData' in 'PoolStorage' overrides 'VersionedInitializable::calculateUserAccountData' which is not marked 'virtual' |
| warning | W200 | function 'calculateUserAccountData' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateAvailableBorrows' in 'PoolStorage' overrides 'VersionedInitializable::calculateAvailableBorrows' which is not marked 'virtual' |
| warning | W200 | function 'calculateAvailableBorrows' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getUserDebtInBaseCurrency' in 'PoolStorage' overrides 'VersionedInitializable::_getUserDebtInBaseCurrency' which is not marked 'virtual' |
| warning | W200 | function '_getUserDebtInBaseCurrency' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getUserBalanceInBaseCurrency' in 'PoolStorage' overrides 'VersionedInitializable::_getUserBalanceInBaseCurrency' which is not marked 'virtual' |
| warning | W200 | function '_getUserBalanceInBaseCurrency' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSupply' in 'PoolStorage' overrides 'VersionedInitializable::validateSupply' which is not marked 'virtual' |
| warning | W200 | function 'validateSupply' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateWithdraw' in 'PoolStorage' overrides 'VersionedInitializable::validateWithdraw' which is not marked 'virtual' |
| warning | W200 | function 'validateWithdraw' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateBorrow' in 'PoolStorage' overrides 'VersionedInitializable::validateBorrow' which is not marked 'virtual' |
| warning | W200 | function 'validateBorrow' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateRepay' in 'PoolStorage' overrides 'VersionedInitializable::validateRepay' which is not marked 'virtual' |
| warning | W200 | function 'validateRepay' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSwapRateMode' in 'PoolStorage' overrides 'VersionedInitializable::validateSwapRateMode' which is not marked 'virtual' |
| warning | W200 | function 'validateSwapRateMode' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateRebalanceStableBorrowRate' in 'PoolStorage' overrides 'VersionedInitializable::validateRebalanceStableBorrowRate' which is not marked 'virtual' |
| warning | W200 | function 'validateRebalanceStableBorrowRate' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSetUseReserveAsCollateral' in 'PoolStorage' overrides 'VersionedInitializable::validateSetUseReserveAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateSetUseReserveAsCollateral' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateFlashloan' in 'PoolStorage' overrides 'VersionedInitializable::validateFlashloan' which is not marked 'virtual' |
| warning | W200 | function 'validateFlashloan' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateFlashloanSimple' in 'PoolStorage' overrides 'VersionedInitializable::validateFlashloanSimple' which is not marked 'virtual' |
| warning | W200 | function 'validateFlashloanSimple' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateLiquidationCall' in 'PoolStorage' overrides 'VersionedInitializable::validateLiquidationCall' which is not marked 'virtual' |
| warning | W200 | function 'validateLiquidationCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHealthFactor' in 'PoolStorage' overrides 'VersionedInitializable::validateHealthFactor' which is not marked 'virtual' |
| warning | W200 | function 'validateHealthFactor' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHFAndLtv' in 'PoolStorage' overrides 'VersionedInitializable::validateHFAndLtv' which is not marked 'virtual' |
| warning | W200 | function 'validateHFAndLtv' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateTransfer' in 'PoolStorage' overrides 'VersionedInitializable::validateTransfer' which is not marked 'virtual' |
| warning | W200 | function 'validateTransfer' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateDropReserve' in 'PoolStorage' overrides 'VersionedInitializable::validateDropReserve' which is not marked 'virtual' |
| warning | W200 | function 'validateDropReserve' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSetUserEMode' in 'PoolStorage' overrides 'VersionedInitializable::validateSetUserEMode' which is not marked 'virtual' |
| warning | W200 | function 'validateSetUserEMode' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateUseAsCollateral' in 'PoolStorage' overrides 'VersionedInitializable::validateUseAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateUseAsCollateral' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateAutomaticUseAsCollateral' in 'PoolStorage' overrides 'VersionedInitializable::validateAutomaticUseAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateAutomaticUseAsCollateral' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeInitReserve' in 'PoolStorage' overrides 'VersionedInitializable::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRescueTokens' in 'PoolStorage' overrides 'VersionedInitializable::executeRescueTokens' which is not marked 'virtual' |
| warning | W200 | function 'executeRescueTokens' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeMintToTreasury' in 'PoolStorage' overrides 'VersionedInitializable::executeMintToTreasury' which is not marked 'virtual' |
| warning | W200 | function 'executeMintToTreasury' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeResetIsolationModeTotalDebt' in 'PoolStorage' overrides 'VersionedInitializable::executeResetIsolationModeTotalDebt' which is not marked 'virtual' |
| warning | W200 | function 'executeResetIsolationModeTotalDebt' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeDropReserve' in 'PoolStorage' overrides 'VersionedInitializable::executeDropReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeDropReserve' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeGetUserAccountData' in 'PoolStorage' overrides 'VersionedInitializable::executeGetUserAccountData' which is not marked 'virtual' |
| warning | W200 | function 'executeGetUserAccountData' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSupply' in 'PoolStorage' overrides 'VersionedInitializable::executeSupply' which is not marked 'virtual' |
| warning | W200 | function 'executeSupply' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeWithdraw' in 'PoolStorage' overrides 'VersionedInitializable::executeWithdraw' which is not marked 'virtual' |
| warning | W200 | function 'executeWithdraw' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFinalizeTransfer' in 'PoolStorage' overrides 'VersionedInitializable::executeFinalizeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'executeFinalizeTransfer' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUseReserveAsCollateral' in 'PoolStorage' overrides 'VersionedInitializable::executeUseReserveAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'executeUseReserveAsCollateral' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUserCurrentDebt' in 'PoolStorage' overrides 'VersionedInitializable::getUserCurrentDebt' which is not marked 'virtual' |
| warning | W200 | function 'getUserCurrentDebt' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateIsolatedDebtIfIsolated' in 'PoolStorage' overrides 'VersionedInitializable::updateIsolatedDebtIfIsolated' which is not marked 'virtual' |
| warning | W200 | function 'updateIsolatedDebtIfIsolated' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeBorrow' in 'PoolStorage' overrides 'VersionedInitializable::executeBorrow' which is not marked 'virtual' |
| warning | W200 | function 'executeBorrow' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRepay' in 'PoolStorage' overrides 'VersionedInitializable::executeRepay' which is not marked 'virtual' |
| warning | W200 | function 'executeRepay' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRebalanceStableBorrowRate' in 'PoolStorage' overrides 'VersionedInitializable::executeRebalanceStableBorrowRate' which is not marked 'virtual' |
| warning | W200 | function 'executeRebalanceStableBorrowRate' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSwapBorrowRateMode' in 'PoolStorage' overrides 'VersionedInitializable::executeSwapBorrowRateMode' which is not marked 'virtual' |
| warning | W200 | function 'executeSwapBorrowRateMode' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFlashLoan' in 'PoolStorage' overrides 'VersionedInitializable::executeFlashLoan' which is not marked 'virtual' |
| warning | W200 | function 'executeFlashLoan' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFlashLoanSimple' in 'PoolStorage' overrides 'VersionedInitializable::executeFlashLoanSimple' which is not marked 'virtual' |
| warning | W200 | function 'executeFlashLoanSimple' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_handleFlashLoanRepayment' in 'PoolStorage' overrides 'VersionedInitializable::_handleFlashLoanRepayment' which is not marked 'virtual' |
| warning | W200 | function '_handleFlashLoanRepayment' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeLiquidationCall' in 'PoolStorage' overrides 'VersionedInitializable::executeLiquidationCall' which is not marked 'virtual' |
| warning | W200 | function 'executeLiquidationCall' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_burnCollateralATokens' in 'PoolStorage' overrides 'VersionedInitializable::_burnCollateralATokens' which is not marked 'virtual' |
| warning | W200 | function '_burnCollateralATokens' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_liquidateATokens' in 'PoolStorage' overrides 'VersionedInitializable::_liquidateATokens' which is not marked 'virtual' |
| warning | W200 | function '_liquidateATokens' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_burnDebtTokens' in 'PoolStorage' overrides 'VersionedInitializable::_burnDebtTokens' which is not marked 'virtual' |
| warning | W200 | function '_burnDebtTokens' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_calculateDebt' in 'PoolStorage' overrides 'VersionedInitializable::_calculateDebt' which is not marked 'virtual' |
| warning | W200 | function '_calculateDebt' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getConfigurationData' in 'PoolStorage' overrides 'VersionedInitializable::_getConfigurationData' which is not marked 'virtual' |
| warning | W200 | function '_getConfigurationData' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_calculateAvailableCollateralToLiquidate' in 'PoolStorage' overrides 'VersionedInitializable::_calculateAvailableCollateralToLiquidate' which is not marked 'virtual' |
| warning | W200 | function '_calculateAvailableCollateralToLiquidate' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeMintUnbacked' in 'PoolStorage' overrides 'VersionedInitializable::executeMintUnbacked' which is not marked 'virtual' |
| warning | W200 | function 'executeMintUnbacked' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeBackUnbacked' in 'PoolStorage' overrides 'VersionedInitializable::executeBackUnbacked' which is not marked 'virtual' |
| warning | W200 | function 'executeBackUnbacked' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSupplyParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeSupplyParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSupplyParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSupplyWithPermitParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeSupplyWithPermitParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSupplyWithPermitParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeWithdrawParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeWithdrawParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeWithdrawParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBorrowParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeBorrowParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBorrowParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRepayParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeRepayParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRepayParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRepayWithPermitParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeRepayWithPermitParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRepayWithPermitParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapBorrowRateModeParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeSwapBorrowRateModeParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapBorrowRateModeParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRebalanceStableBorrowRateParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeRebalanceStableBorrowRateParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRebalanceStableBorrowRateParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSetUserUseReserveAsCollateralParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeSetUserUseReserveAsCollateralParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSetUserUseReserveAsCollateralParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeLiquidationCallParams' in 'PoolStorage' overrides 'VersionedInitializable::decodeLiquidationCallParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeLiquidationCallParams' in 'PoolStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLtv' in 'Pool' overrides 'PoolStorage::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'Pool' overrides 'PoolStorage::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'Pool' overrides 'PoolStorage::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'Pool' overrides 'PoolStorage::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'Pool' overrides 'PoolStorage::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'Pool' overrides 'PoolStorage::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'Pool' overrides 'PoolStorage::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'Pool' overrides 'PoolStorage::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'Pool' overrides 'PoolStorage::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'Pool' overrides 'PoolStorage::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'Pool' overrides 'PoolStorage::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'Pool' overrides 'PoolStorage::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'Pool' overrides 'PoolStorage::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'Pool' overrides 'PoolStorage::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'Pool' overrides 'PoolStorage::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'Pool' overrides 'PoolStorage::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'Pool' overrides 'PoolStorage::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'Pool' overrides 'PoolStorage::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'Pool' overrides 'PoolStorage::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'Pool' overrides 'PoolStorage::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'Pool' overrides 'PoolStorage::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'Pool' overrides 'PoolStorage::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'Pool' overrides 'PoolStorage::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'Pool' overrides 'PoolStorage::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'Pool' overrides 'PoolStorage::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'Pool' overrides 'PoolStorage::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'Pool' overrides 'PoolStorage::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'Pool' overrides 'PoolStorage::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'Pool' overrides 'PoolStorage::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'Pool' overrides 'PoolStorage::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'Pool' overrides 'PoolStorage::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'Pool' overrides 'PoolStorage::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'Pool' overrides 'PoolStorage::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'Pool' overrides 'PoolStorage::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'Pool' overrides 'PoolStorage::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'Pool' overrides 'PoolStorage::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'Pool' overrides 'PoolStorage::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'Pool' overrides 'PoolStorage::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'Pool' overrides 'PoolStorage::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'Pool' overrides 'PoolStorage::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'Pool' overrides 'PoolStorage::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'Pool' overrides 'PoolStorage::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'Pool' overrides 'PoolStorage::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLastTransferResult' in 'Pool' overrides 'PoolStorage::getLastTransferResult' which is not marked 'virtual' |
| warning | W200 | function 'getLastTransferResult' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'Pool' overrides 'PoolStorage::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'Pool' overrides 'PoolStorage::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Pool' overrides 'PoolStorage::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Pool' overrides 'PoolStorage::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Pool' overrides 'PoolStorage::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Pool' overrides 'PoolStorage::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Pool' overrides 'PoolStorage::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Pool' overrides 'PoolStorage::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Pool' overrides 'PoolStorage::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Pool' overrides 'PoolStorage::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Pool' overrides 'PoolStorage::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadMul' in 'Pool' overrides 'PoolStorage::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'Pool' overrides 'PoolStorage::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'Pool' overrides 'PoolStorage::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'Pool' overrides 'PoolStorage::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'Pool' overrides 'PoolStorage::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'Pool' overrides 'PoolStorage::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'Pool' overrides 'PoolStorage::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Pool' overrides 'PoolStorage::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Pool' overrides 'PoolStorage::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'Pool' overrides 'PoolStorage::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'Pool' overrides 'PoolStorage::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Pool' overrides 'PoolStorage::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Pool' overrides 'PoolStorage::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Pool' overrides 'PoolStorage::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Pool' overrides 'PoolStorage::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Pool' overrides 'PoolStorage::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Pool' overrides 'PoolStorage::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Pool' overrides 'PoolStorage::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Pool' overrides 'PoolStorage::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Pool' overrides 'PoolStorage::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Pool' overrides 'PoolStorage::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Pool' overrides 'PoolStorage::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Pool' overrides 'PoolStorage::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Pool' overrides 'PoolStorage::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Pool' overrides 'PoolStorage::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNormalizedIncome' in 'Pool' overrides 'PoolStorage::getNormalizedIncome' which is not marked 'virtual' |
| warning | W200 | function 'getNormalizedIncome' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNormalizedDebt' in 'Pool' overrides 'PoolStorage::getNormalizedDebt' which is not marked 'virtual' |
| warning | W200 | function 'getNormalizedDebt' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateState' in 'Pool' overrides 'PoolStorage::updateState' which is not marked 'virtual' |
| warning | W200 | function 'updateState' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'cumulateToLiquidityIndex' in 'Pool' overrides 'PoolStorage::cumulateToLiquidityIndex' which is not marked 'virtual' |
| warning | W200 | function 'cumulateToLiquidityIndex' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'init' in 'Pool' overrides 'PoolStorage::init' which is not marked 'virtual' |
| warning | W200 | function 'init' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateInterestRates' in 'Pool' overrides 'PoolStorage::updateInterestRates' which is not marked 'virtual' |
| warning | W200 | function 'updateInterestRates' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_accrueToTreasury' in 'Pool' overrides 'PoolStorage::_accrueToTreasury' which is not marked 'virtual' |
| warning | W200 | function '_accrueToTreasury' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_updateIndexes' in 'Pool' overrides 'PoolStorage::_updateIndexes' which is not marked 'virtual' |
| warning | W200 | function '_updateIndexes' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'cache' in 'Pool' overrides 'PoolStorage::cache' which is not marked 'virtual' |
| warning | W200 | function 'cache' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowing' in 'Pool' overrides 'PoolStorage::setBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowing' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUsingAsCollateral' in 'Pool' overrides 'PoolStorage::setUsingAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'setUsingAsCollateral' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralOrBorrowing' in 'Pool' overrides 'PoolStorage::isUsingAsCollateralOrBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralOrBorrowing' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowing' in 'Pool' overrides 'PoolStorage::isBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowing' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateral' in 'Pool' overrides 'PoolStorage::isUsingAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateral' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralOne' in 'Pool' overrides 'PoolStorage::isUsingAsCollateralOne' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralOne' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralAny' in 'Pool' overrides 'PoolStorage::isUsingAsCollateralAny' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralAny' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowingOne' in 'Pool' overrides 'PoolStorage::isBorrowingOne' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowingOne' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowingAny' in 'Pool' overrides 'PoolStorage::isBorrowingAny' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowingAny' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isEmpty' in 'Pool' overrides 'PoolStorage::isEmpty' which is not marked 'virtual' |
| warning | W200 | function 'isEmpty' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getIsolationModeState' in 'Pool' overrides 'PoolStorage::getIsolationModeState' which is not marked 'virtual' |
| warning | W200 | function 'getIsolationModeState' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowingState' in 'Pool' overrides 'PoolStorage::getSiloedBorrowingState' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowingState' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFirstAssetIdByMask' in 'Pool' overrides 'PoolStorage::_getFirstAssetIdByMask' which is not marked 'virtual' |
| warning | W200 | function '_getFirstAssetIdByMask' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSetUserEMode' in 'Pool' overrides 'PoolStorage::executeSetUserEMode' which is not marked 'virtual' |
| warning | W200 | function 'executeSetUserEMode' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeConfiguration' in 'Pool' overrides 'PoolStorage::getEModeConfiguration' which is not marked 'virtual' |
| warning | W200 | function 'getEModeConfiguration' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isInEModeCategory' in 'Pool' overrides 'PoolStorage::isInEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'isInEModeCategory' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateUserAccountData' in 'Pool' overrides 'PoolStorage::calculateUserAccountData' which is not marked 'virtual' |
| warning | W200 | function 'calculateUserAccountData' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateAvailableBorrows' in 'Pool' overrides 'PoolStorage::calculateAvailableBorrows' which is not marked 'virtual' |
| warning | W200 | function 'calculateAvailableBorrows' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getUserDebtInBaseCurrency' in 'Pool' overrides 'PoolStorage::_getUserDebtInBaseCurrency' which is not marked 'virtual' |
| warning | W200 | function '_getUserDebtInBaseCurrency' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getUserBalanceInBaseCurrency' in 'Pool' overrides 'PoolStorage::_getUserBalanceInBaseCurrency' which is not marked 'virtual' |
| warning | W200 | function '_getUserBalanceInBaseCurrency' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSupply' in 'Pool' overrides 'PoolStorage::validateSupply' which is not marked 'virtual' |
| warning | W200 | function 'validateSupply' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateWithdraw' in 'Pool' overrides 'PoolStorage::validateWithdraw' which is not marked 'virtual' |
| warning | W200 | function 'validateWithdraw' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateBorrow' in 'Pool' overrides 'PoolStorage::validateBorrow' which is not marked 'virtual' |
| warning | W200 | function 'validateBorrow' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateRepay' in 'Pool' overrides 'PoolStorage::validateRepay' which is not marked 'virtual' |
| warning | W200 | function 'validateRepay' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSwapRateMode' in 'Pool' overrides 'PoolStorage::validateSwapRateMode' which is not marked 'virtual' |
| warning | W200 | function 'validateSwapRateMode' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateRebalanceStableBorrowRate' in 'Pool' overrides 'PoolStorage::validateRebalanceStableBorrowRate' which is not marked 'virtual' |
| warning | W200 | function 'validateRebalanceStableBorrowRate' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSetUseReserveAsCollateral' in 'Pool' overrides 'PoolStorage::validateSetUseReserveAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateSetUseReserveAsCollateral' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateFlashloan' in 'Pool' overrides 'PoolStorage::validateFlashloan' which is not marked 'virtual' |
| warning | W200 | function 'validateFlashloan' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateFlashloanSimple' in 'Pool' overrides 'PoolStorage::validateFlashloanSimple' which is not marked 'virtual' |
| warning | W200 | function 'validateFlashloanSimple' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateLiquidationCall' in 'Pool' overrides 'PoolStorage::validateLiquidationCall' which is not marked 'virtual' |
| warning | W200 | function 'validateLiquidationCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHealthFactor' in 'Pool' overrides 'PoolStorage::validateHealthFactor' which is not marked 'virtual' |
| warning | W200 | function 'validateHealthFactor' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHFAndLtv' in 'Pool' overrides 'PoolStorage::validateHFAndLtv' which is not marked 'virtual' |
| warning | W200 | function 'validateHFAndLtv' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateTransfer' in 'Pool' overrides 'PoolStorage::validateTransfer' which is not marked 'virtual' |
| warning | W200 | function 'validateTransfer' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateDropReserve' in 'Pool' overrides 'PoolStorage::validateDropReserve' which is not marked 'virtual' |
| warning | W200 | function 'validateDropReserve' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSetUserEMode' in 'Pool' overrides 'PoolStorage::validateSetUserEMode' which is not marked 'virtual' |
| warning | W200 | function 'validateSetUserEMode' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateUseAsCollateral' in 'Pool' overrides 'PoolStorage::validateUseAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateUseAsCollateral' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateAutomaticUseAsCollateral' in 'Pool' overrides 'PoolStorage::validateAutomaticUseAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateAutomaticUseAsCollateral' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeInitReserve' in 'Pool' overrides 'PoolStorage::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRescueTokens' in 'Pool' overrides 'PoolStorage::executeRescueTokens' which is not marked 'virtual' |
| warning | W200 | function 'executeRescueTokens' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeMintToTreasury' in 'Pool' overrides 'PoolStorage::executeMintToTreasury' which is not marked 'virtual' |
| warning | W200 | function 'executeMintToTreasury' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeResetIsolationModeTotalDebt' in 'Pool' overrides 'PoolStorage::executeResetIsolationModeTotalDebt' which is not marked 'virtual' |
| warning | W200 | function 'executeResetIsolationModeTotalDebt' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeDropReserve' in 'Pool' overrides 'PoolStorage::executeDropReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeDropReserve' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeGetUserAccountData' in 'Pool' overrides 'PoolStorage::executeGetUserAccountData' which is not marked 'virtual' |
| warning | W200 | function 'executeGetUserAccountData' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSupply' in 'Pool' overrides 'PoolStorage::executeSupply' which is not marked 'virtual' |
| warning | W200 | function 'executeSupply' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeWithdraw' in 'Pool' overrides 'PoolStorage::executeWithdraw' which is not marked 'virtual' |
| warning | W200 | function 'executeWithdraw' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFinalizeTransfer' in 'Pool' overrides 'PoolStorage::executeFinalizeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'executeFinalizeTransfer' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUseReserveAsCollateral' in 'Pool' overrides 'PoolStorage::executeUseReserveAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'executeUseReserveAsCollateral' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUserCurrentDebt' in 'Pool' overrides 'PoolStorage::getUserCurrentDebt' which is not marked 'virtual' |
| warning | W200 | function 'getUserCurrentDebt' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateIsolatedDebtIfIsolated' in 'Pool' overrides 'PoolStorage::updateIsolatedDebtIfIsolated' which is not marked 'virtual' |
| warning | W200 | function 'updateIsolatedDebtIfIsolated' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeBorrow' in 'Pool' overrides 'PoolStorage::executeBorrow' which is not marked 'virtual' |
| warning | W200 | function 'executeBorrow' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRepay' in 'Pool' overrides 'PoolStorage::executeRepay' which is not marked 'virtual' |
| warning | W200 | function 'executeRepay' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRebalanceStableBorrowRate' in 'Pool' overrides 'PoolStorage::executeRebalanceStableBorrowRate' which is not marked 'virtual' |
| warning | W200 | function 'executeRebalanceStableBorrowRate' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSwapBorrowRateMode' in 'Pool' overrides 'PoolStorage::executeSwapBorrowRateMode' which is not marked 'virtual' |
| warning | W200 | function 'executeSwapBorrowRateMode' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFlashLoan' in 'Pool' overrides 'PoolStorage::executeFlashLoan' which is not marked 'virtual' |
| warning | W200 | function 'executeFlashLoan' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFlashLoanSimple' in 'Pool' overrides 'PoolStorage::executeFlashLoanSimple' which is not marked 'virtual' |
| warning | W200 | function 'executeFlashLoanSimple' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_handleFlashLoanRepayment' in 'Pool' overrides 'PoolStorage::_handleFlashLoanRepayment' which is not marked 'virtual' |
| warning | W200 | function '_handleFlashLoanRepayment' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeLiquidationCall' in 'Pool' overrides 'PoolStorage::executeLiquidationCall' which is not marked 'virtual' |
| warning | W200 | function 'executeLiquidationCall' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_burnCollateralATokens' in 'Pool' overrides 'PoolStorage::_burnCollateralATokens' which is not marked 'virtual' |
| warning | W200 | function '_burnCollateralATokens' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_liquidateATokens' in 'Pool' overrides 'PoolStorage::_liquidateATokens' which is not marked 'virtual' |
| warning | W200 | function '_liquidateATokens' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_burnDebtTokens' in 'Pool' overrides 'PoolStorage::_burnDebtTokens' which is not marked 'virtual' |
| warning | W200 | function '_burnDebtTokens' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_calculateDebt' in 'Pool' overrides 'PoolStorage::_calculateDebt' which is not marked 'virtual' |
| warning | W200 | function '_calculateDebt' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getConfigurationData' in 'Pool' overrides 'PoolStorage::_getConfigurationData' which is not marked 'virtual' |
| warning | W200 | function '_getConfigurationData' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_calculateAvailableCollateralToLiquidate' in 'Pool' overrides 'PoolStorage::_calculateAvailableCollateralToLiquidate' which is not marked 'virtual' |
| warning | W200 | function '_calculateAvailableCollateralToLiquidate' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeMintUnbacked' in 'Pool' overrides 'PoolStorage::executeMintUnbacked' which is not marked 'virtual' |
| warning | W200 | function 'executeMintUnbacked' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeBackUnbacked' in 'Pool' overrides 'PoolStorage::executeBackUnbacked' which is not marked 'virtual' |
| warning | W200 | function 'executeBackUnbacked' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSupplyParams' in 'Pool' overrides 'PoolStorage::decodeSupplyParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSupplyParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSupplyWithPermitParams' in 'Pool' overrides 'PoolStorage::decodeSupplyWithPermitParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSupplyWithPermitParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeWithdrawParams' in 'Pool' overrides 'PoolStorage::decodeWithdrawParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeWithdrawParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBorrowParams' in 'Pool' overrides 'PoolStorage::decodeBorrowParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBorrowParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRepayParams' in 'Pool' overrides 'PoolStorage::decodeRepayParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRepayParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRepayWithPermitParams' in 'Pool' overrides 'PoolStorage::decodeRepayWithPermitParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRepayWithPermitParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapBorrowRateModeParams' in 'Pool' overrides 'PoolStorage::decodeSwapBorrowRateModeParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapBorrowRateModeParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRebalanceStableBorrowRateParams' in 'Pool' overrides 'PoolStorage::decodeRebalanceStableBorrowRateParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRebalanceStableBorrowRateParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSetUserUseReserveAsCollateralParams' in 'Pool' overrides 'PoolStorage::decodeSetUserUseReserveAsCollateralParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSetUserUseReserveAsCollateralParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeLiquidationCallParams' in 'Pool' overrides 'PoolStorage::decodeLiquidationCallParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeLiquidationCallParams' in 'Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLtv' in 'L2Pool' overrides 'Pool::setLtv' which is not marked 'virtual' |
| warning | W200 | function 'setLtv' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLtv' in 'L2Pool' overrides 'Pool::getLtv' which is not marked 'virtual' |
| warning | W200 | function 'getLtv' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationThreshold' in 'L2Pool' overrides 'Pool::setLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationThreshold' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationThreshold' in 'L2Pool' overrides 'Pool::getLiquidationThreshold' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationThreshold' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationBonus' in 'L2Pool' overrides 'Pool::setLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationBonus' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationBonus' in 'L2Pool' overrides 'Pool::getLiquidationBonus' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationBonus' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDecimals' in 'L2Pool' overrides 'Pool::setDecimals' which is not marked 'virtual' |
| warning | W200 | function 'setDecimals' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDecimals' in 'L2Pool' overrides 'Pool::getDecimals' which is not marked 'virtual' |
| warning | W200 | function 'getDecimals' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setActive' in 'L2Pool' overrides 'Pool::setActive' which is not marked 'virtual' |
| warning | W200 | function 'setActive' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getActive' in 'L2Pool' overrides 'Pool::getActive' which is not marked 'virtual' |
| warning | W200 | function 'getActive' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFrozen' in 'L2Pool' overrides 'Pool::setFrozen' which is not marked 'virtual' |
| warning | W200 | function 'setFrozen' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFrozen' in 'L2Pool' overrides 'Pool::getFrozen' which is not marked 'virtual' |
| warning | W200 | function 'getFrozen' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setPaused' in 'L2Pool' overrides 'Pool::setPaused' which is not marked 'virtual' |
| warning | W200 | function 'setPaused' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPaused' in 'L2Pool' overrides 'Pool::getPaused' which is not marked 'virtual' |
| warning | W200 | function 'getPaused' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowableInIsolation' in 'L2Pool' overrides 'Pool::setBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowableInIsolation' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowableInIsolation' in 'L2Pool' overrides 'Pool::getBorrowableInIsolation' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowableInIsolation' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSiloedBorrowing' in 'L2Pool' overrides 'Pool::setSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setSiloedBorrowing' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowing' in 'L2Pool' overrides 'Pool::getSiloedBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowing' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowingEnabled' in 'L2Pool' overrides 'Pool::setBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowingEnabled' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowingEnabled' in 'L2Pool' overrides 'Pool::getBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowingEnabled' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'L2Pool' overrides 'Pool::setStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setStableRateBorrowingEnabled' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'L2Pool' overrides 'Pool::getStableRateBorrowingEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getStableRateBorrowingEnabled' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setReserveFactor' in 'L2Pool' overrides 'Pool::setReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'setReserveFactor' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getReserveFactor' in 'L2Pool' overrides 'Pool::getReserveFactor' which is not marked 'virtual' |
| warning | W200 | function 'getReserveFactor' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowCap' in 'L2Pool' overrides 'Pool::setBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowCap' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getBorrowCap' in 'L2Pool' overrides 'Pool::getBorrowCap' which is not marked 'virtual' |
| warning | W200 | function 'getBorrowCap' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSupplyCap' in 'L2Pool' overrides 'Pool::setSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'setSupplyCap' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSupplyCap' in 'L2Pool' overrides 'Pool::getSupplyCap' which is not marked 'virtual' |
| warning | W200 | function 'getSupplyCap' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setDebtCeiling' in 'L2Pool' overrides 'Pool::setDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'setDebtCeiling' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDebtCeiling' in 'L2Pool' overrides 'Pool::getDebtCeiling' which is not marked 'virtual' |
| warning | W200 | function 'getDebtCeiling' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'L2Pool' overrides 'Pool::setLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setLiquidationProtocolFee' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'L2Pool' overrides 'Pool::getLiquidationProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidationProtocolFee' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnbackedMintCap' in 'L2Pool' overrides 'Pool::setUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'setUnbackedMintCap' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnbackedMintCap' in 'L2Pool' overrides 'Pool::getUnbackedMintCap' which is not marked 'virtual' |
| warning | W200 | function 'getUnbackedMintCap' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setEModeCategory' in 'L2Pool' overrides 'Pool::setEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'setEModeCategory' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeCategory' in 'L2Pool' overrides 'Pool::getEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'getEModeCategory' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setFlashLoanEnabled' in 'L2Pool' overrides 'Pool::setFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'setFlashLoanEnabled' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlashLoanEnabled' in 'L2Pool' overrides 'Pool::getFlashLoanEnabled' which is not marked 'virtual' |
| warning | W200 | function 'getFlashLoanEnabled' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFlags' in 'L2Pool' overrides 'Pool::getFlags' which is not marked 'virtual' |
| warning | W200 | function 'getFlags' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getParams' in 'L2Pool' overrides 'Pool::getParams' which is not marked 'virtual' |
| warning | W200 | function 'getParams' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getCaps' in 'L2Pool' overrides 'Pool::getCaps' which is not marked 'virtual' |
| warning | W200 | function 'getCaps' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransfer' in 'L2Pool' overrides 'Pool::safeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'safeTransfer' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'safeTransferFrom' in 'L2Pool' overrides 'Pool::safeTransferFrom' which is not marked 'virtual' |
| warning | W200 | function 'safeTransferFrom' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLastTransferResult' in 'L2Pool' overrides 'Pool::getLastTransferResult' which is not marked 'virtual' |
| warning | W200 | function 'getLastTransferResult' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isContract' in 'L2Pool' overrides 'Pool::isContract' which is not marked 'virtual' |
| warning | W200 | function 'isContract' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'L2Pool' overrides 'Pool::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'L2Pool' overrides 'Pool::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'L2Pool' overrides 'Pool::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'L2Pool' overrides 'Pool::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'L2Pool' overrides 'Pool::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'L2Pool' overrides 'Pool::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'L2Pool' overrides 'Pool::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'L2Pool' overrides 'Pool::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'L2Pool' overrides 'Pool::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'L2Pool' overrides 'Pool::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadMul' in 'L2Pool' overrides 'Pool::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'L2Pool' overrides 'Pool::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'L2Pool' overrides 'Pool::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'L2Pool' overrides 'Pool::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'L2Pool' overrides 'Pool::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'L2Pool' overrides 'Pool::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'L2Pool' overrides 'Pool::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'L2Pool' overrides 'Pool::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'L2Pool' overrides 'Pool::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentMul' in 'L2Pool' overrides 'Pool::percentMul' which is not marked 'virtual' |
| warning | W200 | function 'percentMul' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'percentDiv' in 'L2Pool' overrides 'Pool::percentDiv' which is not marked 'virtual' |
| warning | W200 | function 'percentDiv' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'L2Pool' overrides 'Pool::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'L2Pool' overrides 'Pool::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'L2Pool' overrides 'Pool::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'L2Pool' overrides 'Pool::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'L2Pool' overrides 'Pool::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'L2Pool' overrides 'Pool::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'L2Pool' overrides 'Pool::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'L2Pool' overrides 'Pool::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'L2Pool' overrides 'Pool::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'L2Pool' overrides 'Pool::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'L2Pool' overrides 'Pool::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'L2Pool' overrides 'Pool::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'L2Pool' overrides 'Pool::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'L2Pool' overrides 'Pool::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNormalizedIncome' in 'L2Pool' overrides 'Pool::getNormalizedIncome' which is not marked 'virtual' |
| warning | W200 | function 'getNormalizedIncome' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNormalizedDebt' in 'L2Pool' overrides 'Pool::getNormalizedDebt' which is not marked 'virtual' |
| warning | W200 | function 'getNormalizedDebt' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateState' in 'L2Pool' overrides 'Pool::updateState' which is not marked 'virtual' |
| warning | W200 | function 'updateState' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'cumulateToLiquidityIndex' in 'L2Pool' overrides 'Pool::cumulateToLiquidityIndex' which is not marked 'virtual' |
| warning | W200 | function 'cumulateToLiquidityIndex' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'init' in 'L2Pool' overrides 'Pool::init' which is not marked 'virtual' |
| warning | W200 | function 'init' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateInterestRates' in 'L2Pool' overrides 'Pool::updateInterestRates' which is not marked 'virtual' |
| warning | W200 | function 'updateInterestRates' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_accrueToTreasury' in 'L2Pool' overrides 'Pool::_accrueToTreasury' which is not marked 'virtual' |
| warning | W200 | function '_accrueToTreasury' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_updateIndexes' in 'L2Pool' overrides 'Pool::_updateIndexes' which is not marked 'virtual' |
| warning | W200 | function '_updateIndexes' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'cache' in 'L2Pool' overrides 'Pool::cache' which is not marked 'virtual' |
| warning | W200 | function 'cache' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setBorrowing' in 'L2Pool' overrides 'Pool::setBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'setBorrowing' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUsingAsCollateral' in 'L2Pool' overrides 'Pool::setUsingAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'setUsingAsCollateral' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralOrBorrowing' in 'L2Pool' overrides 'Pool::isUsingAsCollateralOrBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralOrBorrowing' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowing' in 'L2Pool' overrides 'Pool::isBorrowing' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowing' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateral' in 'L2Pool' overrides 'Pool::isUsingAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateral' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralOne' in 'L2Pool' overrides 'Pool::isUsingAsCollateralOne' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralOne' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUsingAsCollateralAny' in 'L2Pool' overrides 'Pool::isUsingAsCollateralAny' which is not marked 'virtual' |
| warning | W200 | function 'isUsingAsCollateralAny' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowingOne' in 'L2Pool' overrides 'Pool::isBorrowingOne' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowingOne' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isBorrowingAny' in 'L2Pool' overrides 'Pool::isBorrowingAny' which is not marked 'virtual' |
| warning | W200 | function 'isBorrowingAny' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isEmpty' in 'L2Pool' overrides 'Pool::isEmpty' which is not marked 'virtual' |
| warning | W200 | function 'isEmpty' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getIsolationModeState' in 'L2Pool' overrides 'Pool::getIsolationModeState' which is not marked 'virtual' |
| warning | W200 | function 'getIsolationModeState' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSiloedBorrowingState' in 'L2Pool' overrides 'Pool::getSiloedBorrowingState' which is not marked 'virtual' |
| warning | W200 | function 'getSiloedBorrowingState' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getFirstAssetIdByMask' in 'L2Pool' overrides 'Pool::_getFirstAssetIdByMask' which is not marked 'virtual' |
| warning | W200 | function '_getFirstAssetIdByMask' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSetUserEMode' in 'L2Pool' overrides 'Pool::executeSetUserEMode' which is not marked 'virtual' |
| warning | W200 | function 'executeSetUserEMode' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getEModeConfiguration' in 'L2Pool' overrides 'Pool::getEModeConfiguration' which is not marked 'virtual' |
| warning | W200 | function 'getEModeConfiguration' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isInEModeCategory' in 'L2Pool' overrides 'Pool::isInEModeCategory' which is not marked 'virtual' |
| warning | W200 | function 'isInEModeCategory' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateUserAccountData' in 'L2Pool' overrides 'Pool::calculateUserAccountData' which is not marked 'virtual' |
| warning | W200 | function 'calculateUserAccountData' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateAvailableBorrows' in 'L2Pool' overrides 'Pool::calculateAvailableBorrows' which is not marked 'virtual' |
| warning | W200 | function 'calculateAvailableBorrows' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getUserDebtInBaseCurrency' in 'L2Pool' overrides 'Pool::_getUserDebtInBaseCurrency' which is not marked 'virtual' |
| warning | W200 | function '_getUserDebtInBaseCurrency' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getUserBalanceInBaseCurrency' in 'L2Pool' overrides 'Pool::_getUserBalanceInBaseCurrency' which is not marked 'virtual' |
| warning | W200 | function '_getUserBalanceInBaseCurrency' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSupply' in 'L2Pool' overrides 'Pool::validateSupply' which is not marked 'virtual' |
| warning | W200 | function 'validateSupply' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateWithdraw' in 'L2Pool' overrides 'Pool::validateWithdraw' which is not marked 'virtual' |
| warning | W200 | function 'validateWithdraw' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateBorrow' in 'L2Pool' overrides 'Pool::validateBorrow' which is not marked 'virtual' |
| warning | W200 | function 'validateBorrow' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateRepay' in 'L2Pool' overrides 'Pool::validateRepay' which is not marked 'virtual' |
| warning | W200 | function 'validateRepay' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSwapRateMode' in 'L2Pool' overrides 'Pool::validateSwapRateMode' which is not marked 'virtual' |
| warning | W200 | function 'validateSwapRateMode' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateRebalanceStableBorrowRate' in 'L2Pool' overrides 'Pool::validateRebalanceStableBorrowRate' which is not marked 'virtual' |
| warning | W200 | function 'validateRebalanceStableBorrowRate' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSetUseReserveAsCollateral' in 'L2Pool' overrides 'Pool::validateSetUseReserveAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateSetUseReserveAsCollateral' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateFlashloan' in 'L2Pool' overrides 'Pool::validateFlashloan' which is not marked 'virtual' |
| warning | W200 | function 'validateFlashloan' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateFlashloanSimple' in 'L2Pool' overrides 'Pool::validateFlashloanSimple' which is not marked 'virtual' |
| warning | W200 | function 'validateFlashloanSimple' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateLiquidationCall' in 'L2Pool' overrides 'Pool::validateLiquidationCall' which is not marked 'virtual' |
| warning | W200 | function 'validateLiquidationCall' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHealthFactor' in 'L2Pool' overrides 'Pool::validateHealthFactor' which is not marked 'virtual' |
| warning | W200 | function 'validateHealthFactor' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHFAndLtv' in 'L2Pool' overrides 'Pool::validateHFAndLtv' which is not marked 'virtual' |
| warning | W200 | function 'validateHFAndLtv' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateTransfer' in 'L2Pool' overrides 'Pool::validateTransfer' which is not marked 'virtual' |
| warning | W200 | function 'validateTransfer' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateDropReserve' in 'L2Pool' overrides 'Pool::validateDropReserve' which is not marked 'virtual' |
| warning | W200 | function 'validateDropReserve' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateSetUserEMode' in 'L2Pool' overrides 'Pool::validateSetUserEMode' which is not marked 'virtual' |
| warning | W200 | function 'validateSetUserEMode' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateUseAsCollateral' in 'L2Pool' overrides 'Pool::validateUseAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateUseAsCollateral' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateAutomaticUseAsCollateral' in 'L2Pool' overrides 'Pool::validateAutomaticUseAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'validateAutomaticUseAsCollateral' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeInitReserve' in 'L2Pool' overrides 'Pool::executeInitReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeInitReserve' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRescueTokens' in 'L2Pool' overrides 'Pool::executeRescueTokens' which is not marked 'virtual' |
| warning | W200 | function 'executeRescueTokens' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeMintToTreasury' in 'L2Pool' overrides 'Pool::executeMintToTreasury' which is not marked 'virtual' |
| warning | W200 | function 'executeMintToTreasury' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeResetIsolationModeTotalDebt' in 'L2Pool' overrides 'Pool::executeResetIsolationModeTotalDebt' which is not marked 'virtual' |
| warning | W200 | function 'executeResetIsolationModeTotalDebt' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeDropReserve' in 'L2Pool' overrides 'Pool::executeDropReserve' which is not marked 'virtual' |
| warning | W200 | function 'executeDropReserve' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeGetUserAccountData' in 'L2Pool' overrides 'Pool::executeGetUserAccountData' which is not marked 'virtual' |
| warning | W200 | function 'executeGetUserAccountData' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSupply' in 'L2Pool' overrides 'Pool::executeSupply' which is not marked 'virtual' |
| warning | W200 | function 'executeSupply' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeWithdraw' in 'L2Pool' overrides 'Pool::executeWithdraw' which is not marked 'virtual' |
| warning | W200 | function 'executeWithdraw' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFinalizeTransfer' in 'L2Pool' overrides 'Pool::executeFinalizeTransfer' which is not marked 'virtual' |
| warning | W200 | function 'executeFinalizeTransfer' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeUseReserveAsCollateral' in 'L2Pool' overrides 'Pool::executeUseReserveAsCollateral' which is not marked 'virtual' |
| warning | W200 | function 'executeUseReserveAsCollateral' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUserCurrentDebt' in 'L2Pool' overrides 'Pool::getUserCurrentDebt' which is not marked 'virtual' |
| warning | W200 | function 'getUserCurrentDebt' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateIsolatedDebtIfIsolated' in 'L2Pool' overrides 'Pool::updateIsolatedDebtIfIsolated' which is not marked 'virtual' |
| warning | W200 | function 'updateIsolatedDebtIfIsolated' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeBorrow' in 'L2Pool' overrides 'Pool::executeBorrow' which is not marked 'virtual' |
| warning | W200 | function 'executeBorrow' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRepay' in 'L2Pool' overrides 'Pool::executeRepay' which is not marked 'virtual' |
| warning | W200 | function 'executeRepay' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeRebalanceStableBorrowRate' in 'L2Pool' overrides 'Pool::executeRebalanceStableBorrowRate' which is not marked 'virtual' |
| warning | W200 | function 'executeRebalanceStableBorrowRate' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeSwapBorrowRateMode' in 'L2Pool' overrides 'Pool::executeSwapBorrowRateMode' which is not marked 'virtual' |
| warning | W200 | function 'executeSwapBorrowRateMode' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFlashLoan' in 'L2Pool' overrides 'Pool::executeFlashLoan' which is not marked 'virtual' |
| warning | W200 | function 'executeFlashLoan' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeFlashLoanSimple' in 'L2Pool' overrides 'Pool::executeFlashLoanSimple' which is not marked 'virtual' |
| warning | W200 | function 'executeFlashLoanSimple' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_handleFlashLoanRepayment' in 'L2Pool' overrides 'Pool::_handleFlashLoanRepayment' which is not marked 'virtual' |
| warning | W200 | function '_handleFlashLoanRepayment' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeLiquidationCall' in 'L2Pool' overrides 'Pool::executeLiquidationCall' which is not marked 'virtual' |
| warning | W200 | function 'executeLiquidationCall' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_burnCollateralATokens' in 'L2Pool' overrides 'Pool::_burnCollateralATokens' which is not marked 'virtual' |
| warning | W200 | function '_burnCollateralATokens' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_liquidateATokens' in 'L2Pool' overrides 'Pool::_liquidateATokens' which is not marked 'virtual' |
| warning | W200 | function '_liquidateATokens' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_burnDebtTokens' in 'L2Pool' overrides 'Pool::_burnDebtTokens' which is not marked 'virtual' |
| warning | W200 | function '_burnDebtTokens' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_calculateDebt' in 'L2Pool' overrides 'Pool::_calculateDebt' which is not marked 'virtual' |
| warning | W200 | function '_calculateDebt' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getConfigurationData' in 'L2Pool' overrides 'Pool::_getConfigurationData' which is not marked 'virtual' |
| warning | W200 | function '_getConfigurationData' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function '_calculateAvailableCollateralToLiquidate' in 'L2Pool' overrides 'Pool::_calculateAvailableCollateralToLiquidate' which is not marked 'virtual' |
| warning | W200 | function '_calculateAvailableCollateralToLiquidate' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeMintUnbacked' in 'L2Pool' overrides 'Pool::executeMintUnbacked' which is not marked 'virtual' |
| warning | W200 | function 'executeMintUnbacked' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'executeBackUnbacked' in 'L2Pool' overrides 'Pool::executeBackUnbacked' which is not marked 'virtual' |
| warning | W200 | function 'executeBackUnbacked' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSupplyParams' in 'L2Pool' overrides 'Pool::decodeSupplyParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSupplyParams' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSupplyWithPermitParams' in 'L2Pool' overrides 'Pool::decodeSupplyWithPermitParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSupplyWithPermitParams' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeWithdrawParams' in 'L2Pool' overrides 'Pool::decodeWithdrawParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeWithdrawParams' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBorrowParams' in 'L2Pool' overrides 'Pool::decodeBorrowParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBorrowParams' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRepayParams' in 'L2Pool' overrides 'Pool::decodeRepayParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRepayParams' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRepayWithPermitParams' in 'L2Pool' overrides 'Pool::decodeRepayWithPermitParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRepayWithPermitParams' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapBorrowRateModeParams' in 'L2Pool' overrides 'Pool::decodeSwapBorrowRateModeParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapBorrowRateModeParams' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeRebalanceStableBorrowRateParams' in 'L2Pool' overrides 'Pool::decodeRebalanceStableBorrowRateParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeRebalanceStableBorrowRateParams' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSetUserUseReserveAsCollateralParams' in 'L2Pool' overrides 'Pool::decodeSetUserUseReserveAsCollateralParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSetUserUseReserveAsCollateralParams' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeLiquidationCallParams' in 'L2Pool' overrides 'Pool::decodeLiquidationCallParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeLiquidationCallParams' in 'L2Pool' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'L2Pool' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@aave/core-v3/contracts/protocol/pool/L2Pool.sol`