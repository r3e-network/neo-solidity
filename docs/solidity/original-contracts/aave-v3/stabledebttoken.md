# StableDebtToken (Aave V3)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@aave/core-v3/contracts/protocol/tokenization/StableDebtToken.sol`
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
| warning | VALIDATION_WARNING | abstract contract 'VersionedInitializable' has 1 unimplemented function(s): [getRevision] |
| warning | VALIDATION_WARNING | abstract contract 'EIP712Base' has 1 unimplemented function(s): [_EIP712BaseId] |
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
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
| warning | VALIDATION_WARNING | abstract contract 'DebtTokenBase' has 2 unimplemented function(s): [getRevision, _EIP712BaseId] |
| warning | W200 | function 'wadMul' in 'EIP712Base' overrides 'VersionedInitializable::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'EIP712Base' overrides 'VersionedInitializable::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'EIP712Base' overrides 'VersionedInitializable::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'EIP712Base' overrides 'VersionedInitializable::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'EIP712Base' overrides 'VersionedInitializable::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'EIP712Base' overrides 'VersionedInitializable::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'EIP712Base' overrides 'VersionedInitializable::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'EIP712Base' overrides 'VersionedInitializable::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'EIP712Base' overrides 'VersionedInitializable::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'EIP712Base' overrides 'VersionedInitializable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'EIP712Base' overrides 'VersionedInitializable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'EIP712Base' overrides 'VersionedInitializable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'EIP712Base' overrides 'VersionedInitializable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'EIP712Base' overrides 'VersionedInitializable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'EIP712Base' overrides 'VersionedInitializable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'EIP712Base' overrides 'VersionedInitializable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'EIP712Base' overrides 'VersionedInitializable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'EIP712Base' overrides 'VersionedInitializable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'EIP712Base' overrides 'VersionedInitializable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'EIP712Base' overrides 'VersionedInitializable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'EIP712Base' overrides 'VersionedInitializable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'EIP712Base' overrides 'VersionedInitializable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'EIP712Base' overrides 'VersionedInitializable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadMul' in 'Context' overrides 'EIP712Base::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'Context' overrides 'EIP712Base::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'Context' overrides 'EIP712Base::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'Context' overrides 'EIP712Base::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'Context' overrides 'EIP712Base::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'Context' overrides 'EIP712Base::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'Context' overrides 'EIP712Base::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Context' overrides 'EIP712Base::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Context' overrides 'EIP712Base::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Context' overrides 'EIP712Base::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Context' overrides 'EIP712Base::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Context' overrides 'EIP712Base::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Context' overrides 'EIP712Base::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Context' overrides 'EIP712Base::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Context' overrides 'EIP712Base::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Context' overrides 'EIP712Base::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Context' overrides 'EIP712Base::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Context' overrides 'EIP712Base::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Context' overrides 'EIP712Base::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Context' overrides 'EIP712Base::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Context' overrides 'EIP712Base::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Context' overrides 'EIP712Base::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Context' overrides 'EIP712Base::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadMul' in 'DebtTokenBase' overrides 'Context::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'DebtTokenBase' overrides 'Context::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'DebtTokenBase' overrides 'Context::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'DebtTokenBase' overrides 'Context::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'DebtTokenBase' overrides 'Context::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'DebtTokenBase' overrides 'Context::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'DebtTokenBase' overrides 'Context::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'DebtTokenBase' overrides 'Context::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'DebtTokenBase' overrides 'Context::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'DebtTokenBase' overrides 'Context::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'DebtTokenBase' overrides 'Context::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'DebtTokenBase' overrides 'Context::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'DebtTokenBase' overrides 'Context::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'DebtTokenBase' overrides 'Context::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'DebtTokenBase' overrides 'Context::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'DebtTokenBase' overrides 'Context::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'DebtTokenBase' overrides 'Context::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'DebtTokenBase' overrides 'Context::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'DebtTokenBase' overrides 'Context::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'DebtTokenBase' overrides 'Context::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'DebtTokenBase' overrides 'Context::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'DebtTokenBase' overrides 'Context::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'DebtTokenBase' overrides 'Context::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
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
| warning | W101 | function 'transfer' has 2 parameters (ERC-20 pattern). NEP-17 requires 4 parameters: transfer(from, to, amount, data). The `from` address is verified via Runtime.checkWitness() and `data` (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |
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
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'IncentivizedERC20' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_WAD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HALF_RAY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WAD_RAY_RATIO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SECONDS_PER_YEAR' detected while merging libraries |
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
| warning | W101 | function 'transfer' has 2 parameters (ERC-20 pattern). NEP-17 requires 4 parameters: transfer(from, to, amount, data). The `from` address is verified via Runtime.checkWitness() and `data` (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |
| warning | W200 | function 'wadMul' in 'EIP712Base' overrides 'VersionedInitializable::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'EIP712Base' overrides 'VersionedInitializable::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'EIP712Base' overrides 'VersionedInitializable::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'EIP712Base' overrides 'VersionedInitializable::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'EIP712Base' overrides 'VersionedInitializable::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'EIP712Base' overrides 'VersionedInitializable::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'EIP712Base' overrides 'VersionedInitializable::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'EIP712Base' overrides 'VersionedInitializable::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'EIP712Base' overrides 'VersionedInitializable::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'EIP712Base' overrides 'VersionedInitializable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'EIP712Base' overrides 'VersionedInitializable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'EIP712Base' overrides 'VersionedInitializable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'EIP712Base' overrides 'VersionedInitializable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'EIP712Base' overrides 'VersionedInitializable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'EIP712Base' overrides 'VersionedInitializable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'EIP712Base' overrides 'VersionedInitializable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'EIP712Base' overrides 'VersionedInitializable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'EIP712Base' overrides 'VersionedInitializable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'EIP712Base' overrides 'VersionedInitializable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'EIP712Base' overrides 'VersionedInitializable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'EIP712Base' overrides 'VersionedInitializable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'EIP712Base' overrides 'VersionedInitializable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'EIP712Base' overrides 'VersionedInitializable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'EIP712Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadMul' in 'Context' overrides 'EIP712Base::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'Context' overrides 'EIP712Base::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'Context' overrides 'EIP712Base::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'Context' overrides 'EIP712Base::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'Context' overrides 'EIP712Base::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'Context' overrides 'EIP712Base::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'Context' overrides 'EIP712Base::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Context' overrides 'EIP712Base::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Context' overrides 'EIP712Base::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Context' overrides 'EIP712Base::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Context' overrides 'EIP712Base::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Context' overrides 'EIP712Base::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Context' overrides 'EIP712Base::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Context' overrides 'EIP712Base::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Context' overrides 'EIP712Base::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Context' overrides 'EIP712Base::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Context' overrides 'EIP712Base::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Context' overrides 'EIP712Base::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Context' overrides 'EIP712Base::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Context' overrides 'EIP712Base::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Context' overrides 'EIP712Base::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Context' overrides 'EIP712Base::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Context' overrides 'EIP712Base::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Context' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadMul' in 'DebtTokenBase' overrides 'Context::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'DebtTokenBase' overrides 'Context::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'DebtTokenBase' overrides 'Context::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'DebtTokenBase' overrides 'Context::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'DebtTokenBase' overrides 'Context::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'DebtTokenBase' overrides 'Context::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'DebtTokenBase' overrides 'Context::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'DebtTokenBase' overrides 'Context::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'DebtTokenBase' overrides 'Context::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'DebtTokenBase' overrides 'Context::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'DebtTokenBase' overrides 'Context::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'DebtTokenBase' overrides 'Context::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'DebtTokenBase' overrides 'Context::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'DebtTokenBase' overrides 'Context::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'DebtTokenBase' overrides 'Context::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'DebtTokenBase' overrides 'Context::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'DebtTokenBase' overrides 'Context::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'DebtTokenBase' overrides 'Context::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'DebtTokenBase' overrides 'Context::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'DebtTokenBase' overrides 'Context::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'DebtTokenBase' overrides 'Context::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'DebtTokenBase' overrides 'Context::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'DebtTokenBase' overrides 'Context::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'DebtTokenBase' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadMul' in 'IncentivizedERC20' overrides 'DebtTokenBase::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'IncentivizedERC20' overrides 'DebtTokenBase::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'IncentivizedERC20' overrides 'DebtTokenBase::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'IncentivizedERC20' overrides 'DebtTokenBase::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'IncentivizedERC20' overrides 'DebtTokenBase::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'IncentivizedERC20' overrides 'DebtTokenBase::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'IncentivizedERC20' overrides 'DebtTokenBase::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'IncentivizedERC20' overrides 'DebtTokenBase::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'IncentivizedERC20' overrides 'DebtTokenBase::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'IncentivizedERC20' overrides 'DebtTokenBase::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'IncentivizedERC20' overrides 'DebtTokenBase::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'IncentivizedERC20' overrides 'DebtTokenBase::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'IncentivizedERC20' overrides 'DebtTokenBase::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'IncentivizedERC20' overrides 'DebtTokenBase::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'IncentivizedERC20' overrides 'DebtTokenBase::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'IncentivizedERC20' overrides 'DebtTokenBase::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'IncentivizedERC20' overrides 'DebtTokenBase::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'IncentivizedERC20' overrides 'DebtTokenBase::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'IncentivizedERC20' overrides 'DebtTokenBase::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'IncentivizedERC20' overrides 'DebtTokenBase::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'IncentivizedERC20' overrides 'DebtTokenBase::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'IncentivizedERC20' overrides 'DebtTokenBase::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'IncentivizedERC20' overrides 'DebtTokenBase::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'IncentivizedERC20' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadMul' in 'StableDebtToken' overrides 'IncentivizedERC20::wadMul' which is not marked 'virtual' |
| warning | W200 | function 'wadMul' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadDiv' in 'StableDebtToken' overrides 'IncentivizedERC20::wadDiv' which is not marked 'virtual' |
| warning | W200 | function 'wadDiv' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayMul' in 'StableDebtToken' overrides 'IncentivizedERC20::rayMul' which is not marked 'virtual' |
| warning | W200 | function 'rayMul' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayDiv' in 'StableDebtToken' overrides 'IncentivizedERC20::rayDiv' which is not marked 'virtual' |
| warning | W200 | function 'rayDiv' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'rayToWad' in 'StableDebtToken' overrides 'IncentivizedERC20::rayToWad' which is not marked 'virtual' |
| warning | W200 | function 'rayToWad' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'wadToRay' in 'StableDebtToken' overrides 'IncentivizedERC20::wadToRay' which is not marked 'virtual' |
| warning | W200 | function 'wadToRay' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateLinearInterest' in 'StableDebtToken' overrides 'IncentivizedERC20::calculateLinearInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateLinearInterest' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'StableDebtToken' overrides 'IncentivizedERC20::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateCompoundedInterest' in 'StableDebtToken' overrides 'IncentivizedERC20::calculateCompoundedInterest' which is not marked 'virtual' |
| warning | W200 | function 'calculateCompoundedInterest' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'StableDebtToken' overrides 'IncentivizedERC20::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'StableDebtToken' overrides 'IncentivizedERC20::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'StableDebtToken' overrides 'IncentivizedERC20::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'StableDebtToken' overrides 'IncentivizedERC20::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'StableDebtToken' overrides 'IncentivizedERC20::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'StableDebtToken' overrides 'IncentivizedERC20::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'StableDebtToken' overrides 'IncentivizedERC20::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'StableDebtToken' overrides 'IncentivizedERC20::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'StableDebtToken' overrides 'IncentivizedERC20::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'StableDebtToken' overrides 'IncentivizedERC20::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'StableDebtToken' overrides 'IncentivizedERC20::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'StableDebtToken' overrides 'IncentivizedERC20::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'StableDebtToken' overrides 'IncentivizedERC20::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'StableDebtToken' overrides 'IncentivizedERC20::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'StableDebtToken' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'StableDebtToken' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@aave/core-v3/contracts/protocol/tokenization/StableDebtToken.sol`