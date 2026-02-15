# ACLManager (Aave V3)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@aave/core-v3/contracts/protocol/configuration/ACLManager.sol`
- Primary issue: overloaded function 'CALLER_NOT_POOL_ADMIN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `abi_overload`
- Need on Neo (from audit): 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度

### Migration Playbook: ABI overload collision on Neo

1. Rename public/external overloads so each exposed method has a unique name.
1. Keep overloaded variants internal/private if overloading is required for code reuse.
1. If upstream API compatibility is required, add a thin adapter layer that maps unique Neo entrypoints to canonical behavior.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_POOL_ADMIN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_POOL_ADMIN()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_EMERGENCY_ADMIN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_EMERGENCY_ADMIN()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_RISK_OR_POOL_ADMIN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_RISK_OR_POOL_ADMIN()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_BRIDGE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_BRIDGE()' |
| error | VALIDATION_ERROR | overloaded function 'ADDRESSES_PROVIDER_NOT_REGISTERED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'ADDRESSES_PROVIDER_NOT_REGISTERED()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_ADDRESSES_PROVIDER_ID' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_ADDRESSES_PROVIDER_ID()' |
| error | VALIDATION_ERROR | overloaded function 'NOT_CONTRACT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NOT_CONTRACT()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_POOL_CONFIGURATOR' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_POOL_CONFIGURATOR()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_ATOKEN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_ATOKEN()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_ADDRESSES_PROVIDER' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_ADDRESSES_PROVIDER()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_FLASHLOAN_EXECUTOR_RETURN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_FLASHLOAN_EXECUTOR_RETURN()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_ALREADY_ADDED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_ALREADY_ADDED()' |
| error | VALIDATION_ERROR | overloaded function 'NO_MORE_RESERVES_ALLOWED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NO_MORE_RESERVES_ALLOWED()' |
| error | VALIDATION_ERROR | overloaded function 'EMODE_CATEGORY_RESERVED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'EMODE_CATEGORY_RESERVED()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_EMODE_CATEGORY_ASSIGNMENT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_EMODE_CATEGORY_ASSIGNMENT()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_LIQUIDITY_NOT_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_LIQUIDITY_NOT_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'FLASHLOAN_PREMIUM_INVALID' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'FLASHLOAN_PREMIUM_INVALID()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_RESERVE_PARAMS' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_RESERVE_PARAMS()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_EMODE_CATEGORY_PARAMS' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_EMODE_CATEGORY_PARAMS()' |
| error | VALIDATION_ERROR | overloaded function 'BRIDGE_PROTOCOL_FEE_INVALID' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'BRIDGE_PROTOCOL_FEE_INVALID()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_MUST_BE_POOL' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_MUST_BE_POOL()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_MINT_AMOUNT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_MINT_AMOUNT()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_BURN_AMOUNT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_BURN_AMOUNT()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_AMOUNT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_AMOUNT()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_INACTIVE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_INACTIVE()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_FROZEN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_FROZEN()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_PAUSED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_PAUSED()' |
| error | VALIDATION_ERROR | overloaded function 'BORROWING_NOT_ENABLED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'BORROWING_NOT_ENABLED()' |
| error | VALIDATION_ERROR | overloaded function 'STABLE_BORROWING_NOT_ENABLED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'STABLE_BORROWING_NOT_ENABLED()' |
| error | VALIDATION_ERROR | overloaded function 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NOT_ENOUGH_AVAILABLE_USER_BALANCE()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_INTEREST_RATE_MODE_SELECTED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_INTEREST_RATE_MODE_SELECTED()' |
| error | VALIDATION_ERROR | overloaded function 'COLLATERAL_BALANCE_IS_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'COLLATERAL_BALANCE_IS_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD()' |
| error | VALIDATION_ERROR | overloaded function 'COLLATERAL_CANNOT_COVER_NEW_BORROW' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'COLLATERAL_CANNOT_COVER_NEW_BORROW()' |
| error | VALIDATION_ERROR | overloaded function 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'COLLATERAL_SAME_AS_BORROWING_CURRENCY()' |
| error | VALIDATION_ERROR | overloaded function 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE()' |
| error | VALIDATION_ERROR | overloaded function 'NO_DEBT_OF_SELECTED_TYPE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NO_DEBT_OF_SELECTED_TYPE()' |
| error | VALIDATION_ERROR | overloaded function 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF()' |
| error | VALIDATION_ERROR | overloaded function 'NO_OUTSTANDING_STABLE_DEBT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NO_OUTSTANDING_STABLE_DEBT()' |
| error | VALIDATION_ERROR | overloaded function 'NO_OUTSTANDING_VARIABLE_DEBT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NO_OUTSTANDING_VARIABLE_DEBT()' |
| error | VALIDATION_ERROR | overloaded function 'UNDERLYING_BALANCE_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'UNDERLYING_BALANCE_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET()' |
| error | VALIDATION_ERROR | overloaded function 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD()' |
| error | VALIDATION_ERROR | overloaded function 'COLLATERAL_CANNOT_BE_LIQUIDATED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'COLLATERAL_CANNOT_BE_LIQUIDATED()' |
| error | VALIDATION_ERROR | overloaded function 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER()' |
| error | VALIDATION_ERROR | overloaded function 'INCONSISTENT_FLASHLOAN_PARAMS' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INCONSISTENT_FLASHLOAN_PARAMS()' |
| error | VALIDATION_ERROR | overloaded function 'BORROW_CAP_EXCEEDED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'BORROW_CAP_EXCEEDED()' |
| error | VALIDATION_ERROR | overloaded function 'SUPPLY_CAP_EXCEEDED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'SUPPLY_CAP_EXCEEDED()' |
| error | VALIDATION_ERROR | overloaded function 'UNBACKED_MINT_CAP_EXCEEDED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'UNBACKED_MINT_CAP_EXCEEDED()' |
| error | VALIDATION_ERROR | overloaded function 'DEBT_CEILING_EXCEEDED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'DEBT_CEILING_EXCEEDED()' |
| error | VALIDATION_ERROR | overloaded function 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'STABLE_DEBT_NOT_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'STABLE_DEBT_NOT_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'VARIABLE_DEBT_SUPPLY_NOT_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'LTV_VALIDATION_FAILED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'LTV_VALIDATION_FAILED()' |
| error | VALIDATION_ERROR | overloaded function 'INCONSISTENT_EMODE_CATEGORY' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INCONSISTENT_EMODE_CATEGORY()' |
| error | VALIDATION_ERROR | overloaded function 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'PRICE_ORACLE_SENTINEL_CHECK_FAILED()' |
| error | VALIDATION_ERROR | overloaded function 'ASSET_NOT_BORROWABLE_IN_ISOLATION' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'ASSET_NOT_BORROWABLE_IN_ISOLATION()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_ALREADY_INITIALIZED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_ALREADY_INITIALIZED()' |
| error | VALIDATION_ERROR | overloaded function 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_LTV' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_LTV()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_LIQ_THRESHOLD' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_LIQ_THRESHOLD()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_LIQ_BONUS' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_LIQ_BONUS()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_DECIMALS' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_DECIMALS()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_RESERVE_FACTOR' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_RESERVE_FACTOR()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_BORROW_CAP' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_BORROW_CAP()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_SUPPLY_CAP' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_SUPPLY_CAP()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_LIQUIDATION_PROTOCOL_FEE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_LIQUIDATION_PROTOCOL_FEE()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_EMODE_CATEGORY' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_EMODE_CATEGORY()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_UNBACKED_MINT_CAP' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_UNBACKED_MINT_CAP()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_DEBT_CEILING' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_DEBT_CEILING()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_RESERVE_INDEX' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_RESERVE_INDEX()' |
| error | VALIDATION_ERROR | overloaded function 'ACL_ADMIN_CANNOT_BE_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'ACL_ADMIN_CANNOT_BE_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'INCONSISTENT_PARAMS_LENGTH' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INCONSISTENT_PARAMS_LENGTH()' |
| error | VALIDATION_ERROR | overloaded function 'ZERO_ADDRESS_NOT_VALID' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'ZERO_ADDRESS_NOT_VALID()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_EXPIRATION' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_EXPIRATION()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_SIGNATURE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_SIGNATURE()' |
| error | VALIDATION_ERROR | overloaded function 'OPERATION_NOT_SUPPORTED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'OPERATION_NOT_SUPPORTED()' |
| error | VALIDATION_ERROR | overloaded function 'DEBT_CEILING_NOT_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'DEBT_CEILING_NOT_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'ASSET_NOT_LISTED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'ASSET_NOT_LISTED()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_OPTIMAL_USAGE_RATIO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_OPTIMAL_USAGE_RATIO()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()' |
| error | VALIDATION_ERROR | overloaded function 'UNDERLYING_CANNOT_BE_RESCUED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'UNDERLYING_CANNOT_BE_RESCUED()' |
| error | VALIDATION_ERROR | overloaded function 'ADDRESSES_PROVIDER_ALREADY_ADDED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'ADDRESSES_PROVIDER_ALREADY_ADDED()' |
| error | VALIDATION_ERROR | overloaded function 'POOL_ADDRESSES_DO_NOT_MATCH' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'POOL_ADDRESSES_DO_NOT_MATCH()' |
| error | VALIDATION_ERROR | overloaded function 'STABLE_BORROWING_ENABLED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'STABLE_BORROWING_ENABLED()' |
| error | VALIDATION_ERROR | overloaded function 'SILOED_BORROWING_VIOLATION' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'SILOED_BORROWING_VIOLATION()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_DEBT_NOT_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_DEBT_NOT_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'FLASHLOAN_DISABLED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'FLASHLOAN_DISABLED()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_POOL_ADMIN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_POOL_ADMIN()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_EMERGENCY_ADMIN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_EMERGENCY_ADMIN()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_RISK_OR_POOL_ADMIN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_RISK_OR_POOL_ADMIN()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_BRIDGE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_BRIDGE()' |
| error | VALIDATION_ERROR | overloaded function 'ADDRESSES_PROVIDER_NOT_REGISTERED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'ADDRESSES_PROVIDER_NOT_REGISTERED()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_ADDRESSES_PROVIDER_ID' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_ADDRESSES_PROVIDER_ID()' |
| error | VALIDATION_ERROR | overloaded function 'NOT_CONTRACT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NOT_CONTRACT()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_POOL_CONFIGURATOR' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_POOL_CONFIGURATOR()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_NOT_ATOKEN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_NOT_ATOKEN()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_ADDRESSES_PROVIDER' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_ADDRESSES_PROVIDER()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_FLASHLOAN_EXECUTOR_RETURN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_FLASHLOAN_EXECUTOR_RETURN()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_ALREADY_ADDED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_ALREADY_ADDED()' |
| error | VALIDATION_ERROR | overloaded function 'NO_MORE_RESERVES_ALLOWED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NO_MORE_RESERVES_ALLOWED()' |
| error | VALIDATION_ERROR | overloaded function 'EMODE_CATEGORY_RESERVED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'EMODE_CATEGORY_RESERVED()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_EMODE_CATEGORY_ASSIGNMENT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_EMODE_CATEGORY_ASSIGNMENT()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_LIQUIDITY_NOT_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_LIQUIDITY_NOT_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'FLASHLOAN_PREMIUM_INVALID' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'FLASHLOAN_PREMIUM_INVALID()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_RESERVE_PARAMS' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_RESERVE_PARAMS()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_EMODE_CATEGORY_PARAMS' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_EMODE_CATEGORY_PARAMS()' |
| error | VALIDATION_ERROR | overloaded function 'BRIDGE_PROTOCOL_FEE_INVALID' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'BRIDGE_PROTOCOL_FEE_INVALID()' |
| error | VALIDATION_ERROR | overloaded function 'CALLER_MUST_BE_POOL' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'CALLER_MUST_BE_POOL()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_MINT_AMOUNT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_MINT_AMOUNT()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_BURN_AMOUNT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_BURN_AMOUNT()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_AMOUNT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_AMOUNT()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_INACTIVE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_INACTIVE()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_FROZEN' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_FROZEN()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_PAUSED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_PAUSED()' |
| error | VALIDATION_ERROR | overloaded function 'BORROWING_NOT_ENABLED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'BORROWING_NOT_ENABLED()' |
| error | VALIDATION_ERROR | overloaded function 'STABLE_BORROWING_NOT_ENABLED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'STABLE_BORROWING_NOT_ENABLED()' |
| error | VALIDATION_ERROR | overloaded function 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NOT_ENOUGH_AVAILABLE_USER_BALANCE()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_INTEREST_RATE_MODE_SELECTED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_INTEREST_RATE_MODE_SELECTED()' |
| error | VALIDATION_ERROR | overloaded function 'COLLATERAL_BALANCE_IS_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'COLLATERAL_BALANCE_IS_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD()' |
| error | VALIDATION_ERROR | overloaded function 'COLLATERAL_CANNOT_COVER_NEW_BORROW' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'COLLATERAL_CANNOT_COVER_NEW_BORROW()' |
| error | VALIDATION_ERROR | overloaded function 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'COLLATERAL_SAME_AS_BORROWING_CURRENCY()' |
| error | VALIDATION_ERROR | overloaded function 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE()' |
| error | VALIDATION_ERROR | overloaded function 'NO_DEBT_OF_SELECTED_TYPE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NO_DEBT_OF_SELECTED_TYPE()' |
| error | VALIDATION_ERROR | overloaded function 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF()' |
| error | VALIDATION_ERROR | overloaded function 'NO_OUTSTANDING_STABLE_DEBT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NO_OUTSTANDING_STABLE_DEBT()' |
| error | VALIDATION_ERROR | overloaded function 'NO_OUTSTANDING_VARIABLE_DEBT' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'NO_OUTSTANDING_VARIABLE_DEBT()' |
| error | VALIDATION_ERROR | overloaded function 'UNDERLYING_BALANCE_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'UNDERLYING_BALANCE_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET()' |
| error | VALIDATION_ERROR | overloaded function 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD()' |
| error | VALIDATION_ERROR | overloaded function 'COLLATERAL_CANNOT_BE_LIQUIDATED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'COLLATERAL_CANNOT_BE_LIQUIDATED()' |
| error | VALIDATION_ERROR | overloaded function 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER()' |
| error | VALIDATION_ERROR | overloaded function 'INCONSISTENT_FLASHLOAN_PARAMS' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INCONSISTENT_FLASHLOAN_PARAMS()' |
| error | VALIDATION_ERROR | overloaded function 'BORROW_CAP_EXCEEDED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'BORROW_CAP_EXCEEDED()' |
| error | VALIDATION_ERROR | overloaded function 'SUPPLY_CAP_EXCEEDED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'SUPPLY_CAP_EXCEEDED()' |
| error | VALIDATION_ERROR | overloaded function 'UNBACKED_MINT_CAP_EXCEEDED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'UNBACKED_MINT_CAP_EXCEEDED()' |
| error | VALIDATION_ERROR | overloaded function 'DEBT_CEILING_EXCEEDED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'DEBT_CEILING_EXCEEDED()' |
| error | VALIDATION_ERROR | overloaded function 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'STABLE_DEBT_NOT_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'STABLE_DEBT_NOT_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'VARIABLE_DEBT_SUPPLY_NOT_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'LTV_VALIDATION_FAILED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'LTV_VALIDATION_FAILED()' |
| error | VALIDATION_ERROR | overloaded function 'INCONSISTENT_EMODE_CATEGORY' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INCONSISTENT_EMODE_CATEGORY()' |
| error | VALIDATION_ERROR | overloaded function 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'PRICE_ORACLE_SENTINEL_CHECK_FAILED()' |
| error | VALIDATION_ERROR | overloaded function 'ASSET_NOT_BORROWABLE_IN_ISOLATION' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'ASSET_NOT_BORROWABLE_IN_ISOLATION()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_ALREADY_INITIALIZED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_ALREADY_INITIALIZED()' |
| error | VALIDATION_ERROR | overloaded function 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_LTV' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_LTV()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_LIQ_THRESHOLD' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_LIQ_THRESHOLD()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_LIQ_BONUS' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_LIQ_BONUS()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_DECIMALS' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_DECIMALS()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_RESERVE_FACTOR' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_RESERVE_FACTOR()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_BORROW_CAP' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_BORROW_CAP()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_SUPPLY_CAP' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_SUPPLY_CAP()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_LIQUIDATION_PROTOCOL_FEE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_LIQUIDATION_PROTOCOL_FEE()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_EMODE_CATEGORY' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_EMODE_CATEGORY()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_UNBACKED_MINT_CAP' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_UNBACKED_MINT_CAP()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_DEBT_CEILING' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_DEBT_CEILING()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_RESERVE_INDEX' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_RESERVE_INDEX()' |
| error | VALIDATION_ERROR | overloaded function 'ACL_ADMIN_CANNOT_BE_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'ACL_ADMIN_CANNOT_BE_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'INCONSISTENT_PARAMS_LENGTH' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INCONSISTENT_PARAMS_LENGTH()' |
| error | VALIDATION_ERROR | overloaded function 'ZERO_ADDRESS_NOT_VALID' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'ZERO_ADDRESS_NOT_VALID()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_EXPIRATION' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_EXPIRATION()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_SIGNATURE' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_SIGNATURE()' |
| error | VALIDATION_ERROR | overloaded function 'OPERATION_NOT_SUPPORTED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'OPERATION_NOT_SUPPORTED()' |
| error | VALIDATION_ERROR | overloaded function 'DEBT_CEILING_NOT_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'DEBT_CEILING_NOT_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'ASSET_NOT_LISTED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'ASSET_NOT_LISTED()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_OPTIMAL_USAGE_RATIO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_OPTIMAL_USAGE_RATIO()' |
| error | VALIDATION_ERROR | overloaded function 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()' |
| error | VALIDATION_ERROR | overloaded function 'UNDERLYING_CANNOT_BE_RESCUED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'UNDERLYING_CANNOT_BE_RESCUED()' |
| error | VALIDATION_ERROR | overloaded function 'ADDRESSES_PROVIDER_ALREADY_ADDED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'ADDRESSES_PROVIDER_ALREADY_ADDED()' |
| error | VALIDATION_ERROR | overloaded function 'POOL_ADDRESSES_DO_NOT_MATCH' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'POOL_ADDRESSES_DO_NOT_MATCH()' |
| error | VALIDATION_ERROR | overloaded function 'STABLE_BORROWING_ENABLED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'STABLE_BORROWING_ENABLED()' |
| error | VALIDATION_ERROR | overloaded function 'SILOED_BORROWING_VIOLATION' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'SILOED_BORROWING_VIOLATION()' |
| error | VALIDATION_ERROR | overloaded function 'RESERVE_DEBT_NOT_ZERO' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'RESERVE_DEBT_NOT_ZERO()' |
| error | VALIDATION_ERROR | overloaded function 'FLASHLOAN_DISABLED' with 0 parameter(s) is not supported; Neo ABI dispatches by name and argument count only, so overloads that differ only in parameter types cannot be distinguished at runtime |
| error | DUPLICATE_SIGNATURE | duplicate function signature 'FLASHLOAN_DISABLED()' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_POOL_ADMIN' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_EMERGENCY_ADMIN' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_BRIDGE' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_ADDRESSES_PROVIDER_ID' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'NOT_CONTRACT' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_POOL_CONFIGURATOR' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_ATOKEN' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_ADDRESSES_PROVIDER' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'RESERVE_ALREADY_ADDED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'NO_MORE_RESERVES_ALLOWED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'EMODE_CATEGORY_RESERVED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'RESERVE_LIQUIDITY_NOT_ZERO' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'FLASHLOAN_PREMIUM_INVALID' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_RESERVE_PARAMS' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_EMODE_CATEGORY_PARAMS' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'BRIDGE_PROTOCOL_FEE_INVALID' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'CALLER_MUST_BE_POOL' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_MINT_AMOUNT' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_BURN_AMOUNT' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_AMOUNT' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'RESERVE_INACTIVE' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'RESERVE_FROZEN' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'RESERVE_PAUSED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'BORROWING_NOT_ENABLED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'STABLE_BORROWING_NOT_ENABLED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'COLLATERAL_BALANCE_IS_ZERO' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'NO_DEBT_OF_SELECTED_TYPE' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'NO_OUTSTANDING_STABLE_DEBT' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'NO_OUTSTANDING_VARIABLE_DEBT' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'UNDERLYING_BALANCE_ZERO' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INCONSISTENT_FLASHLOAN_PARAMS' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'BORROW_CAP_EXCEEDED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'SUPPLY_CAP_EXCEEDED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'UNBACKED_MINT_CAP_EXCEEDED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'DEBT_CEILING_EXCEEDED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'STABLE_DEBT_NOT_ZERO' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'LTV_VALIDATION_FAILED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INCONSISTENT_EMODE_CATEGORY' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'RESERVE_ALREADY_INITIALIZED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_LTV' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_LIQ_THRESHOLD' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_LIQ_BONUS' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_DECIMALS' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_RESERVE_FACTOR' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_BORROW_CAP' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_SUPPLY_CAP' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_EMODE_CATEGORY' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_UNBACKED_MINT_CAP' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_DEBT_CEILING' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_RESERVE_INDEX' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'ACL_ADMIN_CANNOT_BE_ZERO' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INCONSISTENT_PARAMS_LENGTH' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'ZERO_ADDRESS_NOT_VALID' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_EXPIRATION' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_SIGNATURE' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'OPERATION_NOT_SUPPORTED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'DEBT_CEILING_NOT_ZERO' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'ASSET_NOT_LISTED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_OPTIMAL_USAGE_RATIO' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'UNDERLYING_CANNOT_BE_RESCUED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'POOL_ADDRESSES_DO_NOT_MATCH' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'STABLE_BORROWING_ENABLED' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'SILOED_BORROWING_VIOLATION' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'RESERVE_DEBT_NOT_ZERO' conflicts with a function of the same name |
| error | VALIDATION_ERROR | public state variable 'FLASHLOAN_DISABLED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable '_HEX_SYMBOLS' |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_POOL_ADMIN' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_POOL_ADMIN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_EMERGENCY_ADMIN' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_EMERGENCY_ADMIN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_BRIDGE' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_BRIDGE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' |
| error | VALIDATION_ERROR | public state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_ADDRESSES_PROVIDER_ID' |
| error | VALIDATION_ERROR | public state variable 'INVALID_ADDRESSES_PROVIDER_ID' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NOT_CONTRACT' |
| error | VALIDATION_ERROR | public state variable 'NOT_CONTRACT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_POOL_CONFIGURATOR' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_POOL_CONFIGURATOR' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_ATOKEN' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_ATOKEN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_ADDRESSES_PROVIDER' |
| error | VALIDATION_ERROR | public state variable 'INVALID_ADDRESSES_PROVIDER' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' |
| error | VALIDATION_ERROR | public state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_ALREADY_ADDED' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_ALREADY_ADDED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NO_MORE_RESERVES_ALLOWED' |
| error | VALIDATION_ERROR | public state variable 'NO_MORE_RESERVES_ALLOWED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'EMODE_CATEGORY_RESERVED' |
| error | VALIDATION_ERROR | public state variable 'EMODE_CATEGORY_RESERVED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' |
| error | VALIDATION_ERROR | public state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_LIQUIDITY_NOT_ZERO' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_LIQUIDITY_NOT_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'FLASHLOAN_PREMIUM_INVALID' |
| error | VALIDATION_ERROR | public state variable 'FLASHLOAN_PREMIUM_INVALID' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_RESERVE_PARAMS' |
| error | VALIDATION_ERROR | public state variable 'INVALID_RESERVE_PARAMS' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_EMODE_CATEGORY_PARAMS' |
| error | VALIDATION_ERROR | public state variable 'INVALID_EMODE_CATEGORY_PARAMS' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'BRIDGE_PROTOCOL_FEE_INVALID' |
| error | VALIDATION_ERROR | public state variable 'BRIDGE_PROTOCOL_FEE_INVALID' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_MUST_BE_POOL' |
| error | VALIDATION_ERROR | public state variable 'CALLER_MUST_BE_POOL' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_MINT_AMOUNT' |
| error | VALIDATION_ERROR | public state variable 'INVALID_MINT_AMOUNT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_BURN_AMOUNT' |
| error | VALIDATION_ERROR | public state variable 'INVALID_BURN_AMOUNT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_AMOUNT' |
| error | VALIDATION_ERROR | public state variable 'INVALID_AMOUNT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_INACTIVE' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_INACTIVE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_FROZEN' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_FROZEN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_PAUSED' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_PAUSED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'BORROWING_NOT_ENABLED' |
| error | VALIDATION_ERROR | public state variable 'BORROWING_NOT_ENABLED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'STABLE_BORROWING_NOT_ENABLED' |
| error | VALIDATION_ERROR | public state variable 'STABLE_BORROWING_NOT_ENABLED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' |
| error | VALIDATION_ERROR | public state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' |
| error | VALIDATION_ERROR | public state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'COLLATERAL_BALANCE_IS_ZERO' |
| error | VALIDATION_ERROR | public state variable 'COLLATERAL_BALANCE_IS_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' |
| error | VALIDATION_ERROR | public state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' |
| error | VALIDATION_ERROR | public state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' |
| error | VALIDATION_ERROR | public state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' |
| error | VALIDATION_ERROR | public state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NO_DEBT_OF_SELECTED_TYPE' |
| error | VALIDATION_ERROR | public state variable 'NO_DEBT_OF_SELECTED_TYPE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' |
| error | VALIDATION_ERROR | public state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NO_OUTSTANDING_STABLE_DEBT' |
| error | VALIDATION_ERROR | public state variable 'NO_OUTSTANDING_STABLE_DEBT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NO_OUTSTANDING_VARIABLE_DEBT' |
| error | VALIDATION_ERROR | public state variable 'NO_OUTSTANDING_VARIABLE_DEBT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'UNDERLYING_BALANCE_ZERO' |
| error | VALIDATION_ERROR | public state variable 'UNDERLYING_BALANCE_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' |
| error | VALIDATION_ERROR | public state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' |
| error | VALIDATION_ERROR | public state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' |
| error | VALIDATION_ERROR | public state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' |
| error | VALIDATION_ERROR | public state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INCONSISTENT_FLASHLOAN_PARAMS' |
| error | VALIDATION_ERROR | public state variable 'INCONSISTENT_FLASHLOAN_PARAMS' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'BORROW_CAP_EXCEEDED' |
| error | VALIDATION_ERROR | public state variable 'BORROW_CAP_EXCEEDED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'SUPPLY_CAP_EXCEEDED' |
| error | VALIDATION_ERROR | public state variable 'SUPPLY_CAP_EXCEEDED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'UNBACKED_MINT_CAP_EXCEEDED' |
| error | VALIDATION_ERROR | public state variable 'UNBACKED_MINT_CAP_EXCEEDED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'DEBT_CEILING_EXCEEDED' |
| error | VALIDATION_ERROR | public state variable 'DEBT_CEILING_EXCEEDED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' |
| error | VALIDATION_ERROR | public state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'STABLE_DEBT_NOT_ZERO' |
| error | VALIDATION_ERROR | public state variable 'STABLE_DEBT_NOT_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' |
| error | VALIDATION_ERROR | public state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'LTV_VALIDATION_FAILED' |
| error | VALIDATION_ERROR | public state variable 'LTV_VALIDATION_FAILED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INCONSISTENT_EMODE_CATEGORY' |
| error | VALIDATION_ERROR | public state variable 'INCONSISTENT_EMODE_CATEGORY' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' |
| error | VALIDATION_ERROR | public state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' |
| error | VALIDATION_ERROR | public state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_ALREADY_INITIALIZED' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_ALREADY_INITIALIZED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' |
| error | VALIDATION_ERROR | public state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_LTV' |
| error | VALIDATION_ERROR | public state variable 'INVALID_LTV' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_LIQ_THRESHOLD' |
| error | VALIDATION_ERROR | public state variable 'INVALID_LIQ_THRESHOLD' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_LIQ_BONUS' |
| error | VALIDATION_ERROR | public state variable 'INVALID_LIQ_BONUS' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_DECIMALS' |
| error | VALIDATION_ERROR | public state variable 'INVALID_DECIMALS' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_RESERVE_FACTOR' |
| error | VALIDATION_ERROR | public state variable 'INVALID_RESERVE_FACTOR' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_BORROW_CAP' |
| error | VALIDATION_ERROR | public state variable 'INVALID_BORROW_CAP' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_SUPPLY_CAP' |
| error | VALIDATION_ERROR | public state variable 'INVALID_SUPPLY_CAP' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' |
| error | VALIDATION_ERROR | public state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_EMODE_CATEGORY' |
| error | VALIDATION_ERROR | public state variable 'INVALID_EMODE_CATEGORY' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_UNBACKED_MINT_CAP' |
| error | VALIDATION_ERROR | public state variable 'INVALID_UNBACKED_MINT_CAP' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_DEBT_CEILING' |
| error | VALIDATION_ERROR | public state variable 'INVALID_DEBT_CEILING' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_RESERVE_INDEX' |
| error | VALIDATION_ERROR | public state variable 'INVALID_RESERVE_INDEX' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'ACL_ADMIN_CANNOT_BE_ZERO' |
| error | VALIDATION_ERROR | public state variable 'ACL_ADMIN_CANNOT_BE_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INCONSISTENT_PARAMS_LENGTH' |
| error | VALIDATION_ERROR | public state variable 'INCONSISTENT_PARAMS_LENGTH' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'ZERO_ADDRESS_NOT_VALID' |
| error | VALIDATION_ERROR | public state variable 'ZERO_ADDRESS_NOT_VALID' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_EXPIRATION' |
| error | VALIDATION_ERROR | public state variable 'INVALID_EXPIRATION' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_SIGNATURE' |
| error | VALIDATION_ERROR | public state variable 'INVALID_SIGNATURE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'OPERATION_NOT_SUPPORTED' |
| error | VALIDATION_ERROR | public state variable 'OPERATION_NOT_SUPPORTED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'DEBT_CEILING_NOT_ZERO' |
| error | VALIDATION_ERROR | public state variable 'DEBT_CEILING_NOT_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'ASSET_NOT_LISTED' |
| error | VALIDATION_ERROR | public state variable 'ASSET_NOT_LISTED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_OPTIMAL_USAGE_RATIO' |
| error | VALIDATION_ERROR | public state variable 'INVALID_OPTIMAL_USAGE_RATIO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' |
| error | VALIDATION_ERROR | public state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'UNDERLYING_CANNOT_BE_RESCUED' |
| error | VALIDATION_ERROR | public state variable 'UNDERLYING_CANNOT_BE_RESCUED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' |
| error | VALIDATION_ERROR | public state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'POOL_ADDRESSES_DO_NOT_MATCH' |
| error | VALIDATION_ERROR | public state variable 'POOL_ADDRESSES_DO_NOT_MATCH' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'STABLE_BORROWING_ENABLED' |
| error | VALIDATION_ERROR | public state variable 'STABLE_BORROWING_ENABLED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'SILOED_BORROWING_VIOLATION' |
| error | VALIDATION_ERROR | public state variable 'SILOED_BORROWING_VIOLATION' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_DEBT_NOT_ZERO' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_DEBT_NOT_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'FLASHLOAN_DISABLED' |
| error | VALIDATION_ERROR | public state variable 'FLASHLOAN_DISABLED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable '_HEX_SYMBOLS' |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_POOL_ADMIN' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_POOL_ADMIN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_EMERGENCY_ADMIN' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_EMERGENCY_ADMIN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_POOL_OR_EMERGENCY_ADMIN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_RISK_OR_POOL_ADMIN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_BRIDGE' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_BRIDGE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' |
| error | VALIDATION_ERROR | public state variable 'ADDRESSES_PROVIDER_NOT_REGISTERED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_ADDRESSES_PROVIDER_ID' |
| error | VALIDATION_ERROR | public state variable 'INVALID_ADDRESSES_PROVIDER_ID' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NOT_CONTRACT' |
| error | VALIDATION_ERROR | public state variable 'NOT_CONTRACT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_POOL_CONFIGURATOR' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_POOL_CONFIGURATOR' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_NOT_ATOKEN' |
| error | VALIDATION_ERROR | public state variable 'CALLER_NOT_ATOKEN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_ADDRESSES_PROVIDER' |
| error | VALIDATION_ERROR | public state variable 'INVALID_ADDRESSES_PROVIDER' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' |
| error | VALIDATION_ERROR | public state variable 'INVALID_FLASHLOAN_EXECUTOR_RETURN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_ALREADY_ADDED' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_ALREADY_ADDED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NO_MORE_RESERVES_ALLOWED' |
| error | VALIDATION_ERROR | public state variable 'NO_MORE_RESERVES_ALLOWED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'EMODE_CATEGORY_RESERVED' |
| error | VALIDATION_ERROR | public state variable 'EMODE_CATEGORY_RESERVED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' |
| error | VALIDATION_ERROR | public state variable 'INVALID_EMODE_CATEGORY_ASSIGNMENT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_LIQUIDITY_NOT_ZERO' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_LIQUIDITY_NOT_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'FLASHLOAN_PREMIUM_INVALID' |
| error | VALIDATION_ERROR | public state variable 'FLASHLOAN_PREMIUM_INVALID' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_RESERVE_PARAMS' |
| error | VALIDATION_ERROR | public state variable 'INVALID_RESERVE_PARAMS' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_EMODE_CATEGORY_PARAMS' |
| error | VALIDATION_ERROR | public state variable 'INVALID_EMODE_CATEGORY_PARAMS' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'BRIDGE_PROTOCOL_FEE_INVALID' |
| error | VALIDATION_ERROR | public state variable 'BRIDGE_PROTOCOL_FEE_INVALID' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'CALLER_MUST_BE_POOL' |
| error | VALIDATION_ERROR | public state variable 'CALLER_MUST_BE_POOL' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_MINT_AMOUNT' |
| error | VALIDATION_ERROR | public state variable 'INVALID_MINT_AMOUNT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_BURN_AMOUNT' |
| error | VALIDATION_ERROR | public state variable 'INVALID_BURN_AMOUNT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_AMOUNT' |
| error | VALIDATION_ERROR | public state variable 'INVALID_AMOUNT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_INACTIVE' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_INACTIVE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_FROZEN' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_FROZEN' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_PAUSED' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_PAUSED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'BORROWING_NOT_ENABLED' |
| error | VALIDATION_ERROR | public state variable 'BORROWING_NOT_ENABLED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'STABLE_BORROWING_NOT_ENABLED' |
| error | VALIDATION_ERROR | public state variable 'STABLE_BORROWING_NOT_ENABLED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' |
| error | VALIDATION_ERROR | public state variable 'NOT_ENOUGH_AVAILABLE_USER_BALANCE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' |
| error | VALIDATION_ERROR | public state variable 'INVALID_INTEREST_RATE_MODE_SELECTED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'COLLATERAL_BALANCE_IS_ZERO' |
| error | VALIDATION_ERROR | public state variable 'COLLATERAL_BALANCE_IS_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' |
| error | VALIDATION_ERROR | public state variable 'HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' |
| error | VALIDATION_ERROR | public state variable 'COLLATERAL_CANNOT_COVER_NEW_BORROW' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' |
| error | VALIDATION_ERROR | public state variable 'COLLATERAL_SAME_AS_BORROWING_CURRENCY' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' |
| error | VALIDATION_ERROR | public state variable 'AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NO_DEBT_OF_SELECTED_TYPE' |
| error | VALIDATION_ERROR | public state variable 'NO_DEBT_OF_SELECTED_TYPE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' |
| error | VALIDATION_ERROR | public state variable 'NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NO_OUTSTANDING_STABLE_DEBT' |
| error | VALIDATION_ERROR | public state variable 'NO_OUTSTANDING_STABLE_DEBT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'NO_OUTSTANDING_VARIABLE_DEBT' |
| error | VALIDATION_ERROR | public state variable 'NO_OUTSTANDING_VARIABLE_DEBT' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'UNDERLYING_BALANCE_ZERO' |
| error | VALIDATION_ERROR | public state variable 'UNDERLYING_BALANCE_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' |
| error | VALIDATION_ERROR | public state variable 'INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' |
| error | VALIDATION_ERROR | public state variable 'HEALTH_FACTOR_NOT_BELOW_THRESHOLD' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' |
| error | VALIDATION_ERROR | public state variable 'COLLATERAL_CANNOT_BE_LIQUIDATED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' |
| error | VALIDATION_ERROR | public state variable 'SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INCONSISTENT_FLASHLOAN_PARAMS' |
| error | VALIDATION_ERROR | public state variable 'INCONSISTENT_FLASHLOAN_PARAMS' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'BORROW_CAP_EXCEEDED' |
| error | VALIDATION_ERROR | public state variable 'BORROW_CAP_EXCEEDED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'SUPPLY_CAP_EXCEEDED' |
| error | VALIDATION_ERROR | public state variable 'SUPPLY_CAP_EXCEEDED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'UNBACKED_MINT_CAP_EXCEEDED' |
| error | VALIDATION_ERROR | public state variable 'UNBACKED_MINT_CAP_EXCEEDED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'DEBT_CEILING_EXCEEDED' |
| error | VALIDATION_ERROR | public state variable 'DEBT_CEILING_EXCEEDED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' |
| error | VALIDATION_ERROR | public state variable 'UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'STABLE_DEBT_NOT_ZERO' |
| error | VALIDATION_ERROR | public state variable 'STABLE_DEBT_NOT_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' |
| error | VALIDATION_ERROR | public state variable 'VARIABLE_DEBT_SUPPLY_NOT_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'LTV_VALIDATION_FAILED' |
| error | VALIDATION_ERROR | public state variable 'LTV_VALIDATION_FAILED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INCONSISTENT_EMODE_CATEGORY' |
| error | VALIDATION_ERROR | public state variable 'INCONSISTENT_EMODE_CATEGORY' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' |
| error | VALIDATION_ERROR | public state variable 'PRICE_ORACLE_SENTINEL_CHECK_FAILED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' |
| error | VALIDATION_ERROR | public state variable 'ASSET_NOT_BORROWABLE_IN_ISOLATION' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_ALREADY_INITIALIZED' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_ALREADY_INITIALIZED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' |
| error | VALIDATION_ERROR | public state variable 'USER_IN_ISOLATION_MODE_OR_LTV_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_LTV' |
| error | VALIDATION_ERROR | public state variable 'INVALID_LTV' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_LIQ_THRESHOLD' |
| error | VALIDATION_ERROR | public state variable 'INVALID_LIQ_THRESHOLD' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_LIQ_BONUS' |
| error | VALIDATION_ERROR | public state variable 'INVALID_LIQ_BONUS' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_DECIMALS' |
| error | VALIDATION_ERROR | public state variable 'INVALID_DECIMALS' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_RESERVE_FACTOR' |
| error | VALIDATION_ERROR | public state variable 'INVALID_RESERVE_FACTOR' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_BORROW_CAP' |
| error | VALIDATION_ERROR | public state variable 'INVALID_BORROW_CAP' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_SUPPLY_CAP' |
| error | VALIDATION_ERROR | public state variable 'INVALID_SUPPLY_CAP' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' |
| error | VALIDATION_ERROR | public state variable 'INVALID_LIQUIDATION_PROTOCOL_FEE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_EMODE_CATEGORY' |
| error | VALIDATION_ERROR | public state variable 'INVALID_EMODE_CATEGORY' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_UNBACKED_MINT_CAP' |
| error | VALIDATION_ERROR | public state variable 'INVALID_UNBACKED_MINT_CAP' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_DEBT_CEILING' |
| error | VALIDATION_ERROR | public state variable 'INVALID_DEBT_CEILING' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_RESERVE_INDEX' |
| error | VALIDATION_ERROR | public state variable 'INVALID_RESERVE_INDEX' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'ACL_ADMIN_CANNOT_BE_ZERO' |
| error | VALIDATION_ERROR | public state variable 'ACL_ADMIN_CANNOT_BE_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INCONSISTENT_PARAMS_LENGTH' |
| error | VALIDATION_ERROR | public state variable 'INCONSISTENT_PARAMS_LENGTH' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'ZERO_ADDRESS_NOT_VALID' |
| error | VALIDATION_ERROR | public state variable 'ZERO_ADDRESS_NOT_VALID' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_EXPIRATION' |
| error | VALIDATION_ERROR | public state variable 'INVALID_EXPIRATION' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_SIGNATURE' |
| error | VALIDATION_ERROR | public state variable 'INVALID_SIGNATURE' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'OPERATION_NOT_SUPPORTED' |
| error | VALIDATION_ERROR | public state variable 'OPERATION_NOT_SUPPORTED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'DEBT_CEILING_NOT_ZERO' |
| error | VALIDATION_ERROR | public state variable 'DEBT_CEILING_NOT_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'ASSET_NOT_LISTED' |
| error | VALIDATION_ERROR | public state variable 'ASSET_NOT_LISTED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_OPTIMAL_USAGE_RATIO' |
| error | VALIDATION_ERROR | public state variable 'INVALID_OPTIMAL_USAGE_RATIO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' |
| error | VALIDATION_ERROR | public state variable 'INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'UNDERLYING_CANNOT_BE_RESCUED' |
| error | VALIDATION_ERROR | public state variable 'UNDERLYING_CANNOT_BE_RESCUED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' |
| error | VALIDATION_ERROR | public state variable 'ADDRESSES_PROVIDER_ALREADY_ADDED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'POOL_ADDRESSES_DO_NOT_MATCH' |
| error | VALIDATION_ERROR | public state variable 'POOL_ADDRESSES_DO_NOT_MATCH' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'STABLE_BORROWING_ENABLED' |
| error | VALIDATION_ERROR | public state variable 'STABLE_BORROWING_ENABLED' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'SILOED_BORROWING_VIOLATION' |
| error | VALIDATION_ERROR | public state variable 'SILOED_BORROWING_VIOLATION' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'RESERVE_DEBT_NOT_ZERO' |
| error | VALIDATION_ERROR | public state variable 'RESERVE_DEBT_NOT_ZERO' conflicts with a function of the same name |
| error | DUPLICATE_STATE_VARIABLE | duplicate state variable 'FLASHLOAN_DISABLED' |
| error | VALIDATION_ERROR | public state variable 'FLASHLOAN_DISABLED' conflicts with a function of the same name |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@aave/core-v3/contracts/protocol/configuration/ACLManager.sol`