# PoolManager (Uniswap V4 Core)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@uniswap/v4-core/src/PoolManager.sol`
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
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'NoDelegateCall' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Owned' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DYNAMIC_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVERRIDE_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REMOVE_OVERRIDE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ALL_HOOK_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SWAP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_160_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROTOCOL_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LP_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_0_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_1_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PIPS_DENOMINATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '_getPool' return value 'Pool.State' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | VALIDATION_WARNING | abstract contract 'ProtocolFees' has 2 unimplemented function(s): [_isUnlocked, _getPool] |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ProtocolFees' overrides 'Owned::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'ProtocolFees' overrides 'Owned::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'ProtocolFees' overrides 'Owned::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'ProtocolFees' overrides 'Owned::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'ProtocolFees' overrides 'Owned::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ProtocolFees' overrides 'Owned::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'ProtocolFees' overrides 'Owned::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ProtocolFees' overrides 'Owned::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ProtocolFees' overrides 'Owned::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ProtocolFees' overrides 'Owned::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ProtocolFees' overrides 'Owned::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ProtocolFees' overrides 'Owned::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ProtocolFees' overrides 'Owned::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'ProtocolFees' overrides 'Owned::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'ProtocolFees' overrides 'Owned::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'ProtocolFees' overrides 'Owned::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ProtocolFees' overrides 'Owned::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ProtocolFees' overrides 'Owned::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isDynamicFee' in 'ProtocolFees' overrides 'Owned::isDynamicFee' which is not marked 'virtual' |
| warning | W200 | function 'isDynamicFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValid' in 'ProtocolFees' overrides 'Owned::isValid' which is not marked 'virtual' |
| warning | W200 | function 'isValid' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validate' in 'ProtocolFees' overrides 'Owned::validate' which is not marked 'virtual' |
| warning | W200 | function 'validate' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInitialLPFee' in 'ProtocolFees' overrides 'Owned::getInitialLPFee' which is not marked 'virtual' |
| warning | W200 | function 'getInitialLPFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isOverride' in 'ProtocolFees' overrides 'Owned::isOverride' which is not marked 'virtual' |
| warning | W200 | function 'isOverride' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlag' in 'ProtocolFees' overrides 'Owned::removeOverrideFlag' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlag' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'ProtocolFees' overrides 'Owned::removeOverrideFlagAndValidate' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'ProtocolFees' overrides 'Owned::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'ProtocolFees' overrides 'Owned::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'ProtocolFees' overrides 'Owned::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHookPermissions' in 'ProtocolFees' overrides 'Owned::validateHookPermissions' which is not marked 'virtual' |
| warning | W200 | function 'validateHookPermissions' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidHookAddress' in 'ProtocolFees' overrides 'Owned::isValidHookAddress' which is not marked 'virtual' |
| warning | W200 | function 'isValidHookAddress' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHook' in 'ProtocolFees' overrides 'Owned::callHook' which is not marked 'virtual' |
| warning | W200 | function 'callHook' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHookWithReturnDelta' in 'ProtocolFees' overrides 'Owned::callHookWithReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'callHookWithReturnDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'noSelfCall' in 'ProtocolFees' overrides 'Owned::noSelfCall' which is not marked 'virtual' |
| warning | W200 | function 'noSelfCall' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeInitialize' in 'ProtocolFees' overrides 'Owned::beforeInitialize' which is not marked 'virtual' |
| warning | W200 | function 'beforeInitialize' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterInitialize' in 'ProtocolFees' overrides 'Owned::afterInitialize' which is not marked 'virtual' |
| warning | W200 | function 'afterInitialize' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeModifyLiquidity' in 'ProtocolFees' overrides 'Owned::beforeModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'beforeModifyLiquidity' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterModifyLiquidity' in 'ProtocolFees' overrides 'Owned::afterModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'afterModifyLiquidity' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeSwap' in 'ProtocolFees' overrides 'Owned::beforeSwap' which is not marked 'virtual' |
| warning | W200 | function 'beforeSwap' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterSwap' in 'ProtocolFees' overrides 'Owned::afterSwap' which is not marked 'virtual' |
| warning | W200 | function 'afterSwap' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeDonate' in 'ProtocolFees' overrides 'Owned::beforeDonate' which is not marked 'virtual' |
| warning | W200 | function 'beforeDonate' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterDonate' in 'ProtocolFees' overrides 'Owned::afterDonate' which is not marked 'virtual' |
| warning | W200 | function 'afterDonate' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasPermission' in 'ProtocolFees' overrides 'Owned::hasPermission' which is not marked 'virtual' |
| warning | W200 | function 'hasPermission' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'ProtocolFees' overrides 'Owned::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'ProtocolFees' overrides 'Owned::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'compress' in 'ProtocolFees' overrides 'Owned::compress' which is not marked 'virtual' |
| warning | W200 | function 'compress' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'position' in 'ProtocolFees' overrides 'Owned::position' which is not marked 'virtual' |
| warning | W200 | function 'position' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'flipTick' in 'ProtocolFees' overrides 'Owned::flipTick' which is not marked 'virtual' |
| warning | W200 | function 'flipTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'ProtocolFees' overrides 'Owned::nextInitializedTickWithinOneWord' which is not marked 'virtual' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ProtocolFees' overrides 'Owned::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'ProtocolFees' overrides 'Owned::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'ProtocolFees' overrides 'Owned::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ProtocolFees' overrides 'Owned::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'ProtocolFees' overrides 'Owned::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'ProtocolFees' overrides 'Owned::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'divRoundingUp' in 'ProtocolFees' overrides 'Owned::divRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'divRoundingUp' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'simpleMulDiv' in 'ProtocolFees' overrides 'Owned::simpleMulDiv' which is not marked 'virtual' |
| warning | W200 | function 'simpleMulDiv' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'ProtocolFees' overrides 'Owned::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'ProtocolFees' overrides 'Owned::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ProtocolFees' overrides 'Owned::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ProtocolFees' overrides 'Owned::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'ProtocolFees' overrides 'Owned::getNextSqrtPriceFromAmount0RoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'ProtocolFees' overrides 'Owned::getNextSqrtPriceFromAmount1RoundingDown' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'ProtocolFees' overrides 'Owned::getNextSqrtPriceFromInput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'ProtocolFees' overrides 'Owned::getNextSqrtPriceFromOutput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'ProtocolFees' overrides 'Owned::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'absDiff' in 'ProtocolFees' overrides 'Owned::absDiff' which is not marked 'virtual' |
| warning | W200 | function 'absDiff' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'ProtocolFees' overrides 'Owned::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'ProtocolFees' overrides 'Owned::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'ProtocolFees' overrides 'Owned::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceTarget' in 'ProtocolFees' overrides 'Owned::getSqrtPriceTarget' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceTarget' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'computeSwapStep' in 'ProtocolFees' overrides 'Owned::computeSwapStep' which is not marked 'virtual' |
| warning | W200 | function 'computeSwapStep' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrtPriceX96' in 'ProtocolFees' overrides 'Owned::sqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'sqrtPriceX96' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tick' in 'ProtocolFees' overrides 'Owned::tick' which is not marked 'virtual' |
| warning | W200 | function 'tick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'protocolFee' in 'ProtocolFees' overrides 'Owned::protocolFee' which is not marked 'virtual' |
| warning | W200 | function 'protocolFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lpFee' in 'ProtocolFees' overrides 'Owned::lpFee' which is not marked 'virtual' |
| warning | W200 | function 'lpFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSqrtPriceX96' in 'ProtocolFees' overrides 'Owned::setSqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'setSqrtPriceX96' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setTick' in 'ProtocolFees' overrides 'Owned::setTick' which is not marked 'virtual' |
| warning | W200 | function 'setTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'ProtocolFees' overrides 'Owned::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLpFee' in 'ProtocolFees' overrides 'Owned::setLpFee' which is not marked 'virtual' |
| warning | W200 | function 'setLpFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getZeroForOneFee' in 'ProtocolFees' overrides 'Owned::getZeroForOneFee' which is not marked 'virtual' |
| warning | W200 | function 'getZeroForOneFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getOneForZeroFee' in 'ProtocolFees' overrides 'Owned::getOneForZeroFee' which is not marked 'virtual' |
| warning | W200 | function 'getOneForZeroFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidProtocolFee' in 'ProtocolFees' overrides 'Owned::isValidProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'isValidProtocolFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateSwapFee' in 'ProtocolFees' overrides 'Owned::calculateSwapFee' which is not marked 'virtual' |
| warning | W200 | function 'calculateSwapFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkTicks' in 'ProtocolFees' overrides 'Owned::checkTicks' which is not marked 'virtual' |
| warning | W200 | function 'checkTicks' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'ProtocolFees' overrides 'Owned::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'ProtocolFees' overrides 'Owned::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLPFee' in 'ProtocolFees' overrides 'Owned::setLPFee' which is not marked 'virtual' |
| warning | W200 | function 'setLPFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modifyLiquidity' in 'ProtocolFees' overrides 'Owned::modifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'modifyLiquidity' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'swap' in 'ProtocolFees' overrides 'Owned::swap' which is not marked 'virtual' |
| warning | W200 | function 'swap' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'donate' in 'ProtocolFees' overrides 'Owned::donate' which is not marked 'virtual' |
| warning | W200 | function 'donate' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'ProtocolFees' overrides 'Owned::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateTick' in 'ProtocolFees' overrides 'Owned::updateTick' which is not marked 'virtual' |
| warning | W200 | function 'updateTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'ProtocolFees' overrides 'Owned::tickSpacingToMaxLiquidityPerTick' which is not marked 'virtual' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkPoolInitialized' in 'ProtocolFees' overrides 'Owned::checkPoolInitialized' which is not marked 'virtual' |
| warning | W200 | function 'checkPoolInitialized' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clearTick' in 'ProtocolFees' overrides 'Owned::clearTick' which is not marked 'virtual' |
| warning | W200 | function 'clearTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'crossTick' in 'ProtocolFees' overrides 'Owned::crossTick' which is not marked 'virtual' |
| warning | W200 | function 'crossTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'ProtocolFees' overrides 'Owned::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'ProtocolFees' overrides 'Owned::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ProtocolFees' overrides 'Owned::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ProtocolFees' overrides 'Owned::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'ProtocolFees' overrides 'Owned::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'ProtocolFees' overrides 'Owned::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'ProtocolFees' overrides 'Owned::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function '_computeSlot' in 'ProtocolFees' overrides 'Owned::_computeSlot' which is not marked 'virtual' |
| warning | W200 | function '_computeSlot' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDelta' in 'ProtocolFees' overrides 'Owned::getDelta' which is not marked 'virtual' |
| warning | W200 | function 'getDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyDelta' in 'ProtocolFees' overrides 'Owned::applyDelta' which is not marked 'virtual' |
| warning | W200 | function 'applyDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'ProtocolFees' overrides 'Owned::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'ProtocolFees' overrides 'Owned::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'ProtocolFees' overrides 'Owned::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ProtocolFees' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W123 | public state variable 'balanceOf' conflicts with a function of the same name |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | W102 | function 'transfer' has 3 parameters, but NEP-17 requires 4: transfer(from, to, amount, data). The `data` parameter (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ERC6909' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W123 | public state variable 'balanceOf' conflicts with a function of the same name |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DYNAMIC_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVERRIDE_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REMOVE_OVERRIDE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ALL_HOOK_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SWAP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_160_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROTOCOL_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LP_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_0_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_1_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PIPS_DENOMINATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | W102 | function 'transfer' has 3 parameters, but NEP-17 requires 4: transfer(from, to, amount, data). The `data` parameter (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ERC6909Claims' overrides 'ERC6909::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'ERC6909Claims' overrides 'ERC6909::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'ERC6909Claims' overrides 'ERC6909::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'ERC6909Claims' overrides 'ERC6909::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'ERC6909Claims' overrides 'ERC6909::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ERC6909Claims' overrides 'ERC6909::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'ERC6909Claims' overrides 'ERC6909::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC6909Claims' overrides 'ERC6909::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC6909Claims' overrides 'ERC6909::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC6909Claims' overrides 'ERC6909::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC6909Claims' overrides 'ERC6909::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC6909Claims' overrides 'ERC6909::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC6909Claims' overrides 'ERC6909::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'ERC6909Claims' overrides 'ERC6909::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'ERC6909Claims' overrides 'ERC6909::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'ERC6909Claims' overrides 'ERC6909::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ERC6909Claims' overrides 'ERC6909::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ERC6909Claims' overrides 'ERC6909::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isDynamicFee' in 'ERC6909Claims' overrides 'ERC6909::isDynamicFee' which is not marked 'virtual' |
| warning | W200 | function 'isDynamicFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValid' in 'ERC6909Claims' overrides 'ERC6909::isValid' which is not marked 'virtual' |
| warning | W200 | function 'isValid' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validate' in 'ERC6909Claims' overrides 'ERC6909::validate' which is not marked 'virtual' |
| warning | W200 | function 'validate' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInitialLPFee' in 'ERC6909Claims' overrides 'ERC6909::getInitialLPFee' which is not marked 'virtual' |
| warning | W200 | function 'getInitialLPFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isOverride' in 'ERC6909Claims' overrides 'ERC6909::isOverride' which is not marked 'virtual' |
| warning | W200 | function 'isOverride' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlag' in 'ERC6909Claims' overrides 'ERC6909::removeOverrideFlag' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlag' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'ERC6909Claims' overrides 'ERC6909::removeOverrideFlagAndValidate' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'ERC6909Claims' overrides 'ERC6909::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'ERC6909Claims' overrides 'ERC6909::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'ERC6909Claims' overrides 'ERC6909::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHookPermissions' in 'ERC6909Claims' overrides 'ERC6909::validateHookPermissions' which is not marked 'virtual' |
| warning | W200 | function 'validateHookPermissions' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidHookAddress' in 'ERC6909Claims' overrides 'ERC6909::isValidHookAddress' which is not marked 'virtual' |
| warning | W200 | function 'isValidHookAddress' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHook' in 'ERC6909Claims' overrides 'ERC6909::callHook' which is not marked 'virtual' |
| warning | W200 | function 'callHook' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHookWithReturnDelta' in 'ERC6909Claims' overrides 'ERC6909::callHookWithReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'callHookWithReturnDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'noSelfCall' in 'ERC6909Claims' overrides 'ERC6909::noSelfCall' which is not marked 'virtual' |
| warning | W200 | function 'noSelfCall' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeInitialize' in 'ERC6909Claims' overrides 'ERC6909::beforeInitialize' which is not marked 'virtual' |
| warning | W200 | function 'beforeInitialize' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterInitialize' in 'ERC6909Claims' overrides 'ERC6909::afterInitialize' which is not marked 'virtual' |
| warning | W200 | function 'afterInitialize' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeModifyLiquidity' in 'ERC6909Claims' overrides 'ERC6909::beforeModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'beforeModifyLiquidity' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterModifyLiquidity' in 'ERC6909Claims' overrides 'ERC6909::afterModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'afterModifyLiquidity' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeSwap' in 'ERC6909Claims' overrides 'ERC6909::beforeSwap' which is not marked 'virtual' |
| warning | W200 | function 'beforeSwap' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterSwap' in 'ERC6909Claims' overrides 'ERC6909::afterSwap' which is not marked 'virtual' |
| warning | W200 | function 'afterSwap' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeDonate' in 'ERC6909Claims' overrides 'ERC6909::beforeDonate' which is not marked 'virtual' |
| warning | W200 | function 'beforeDonate' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterDonate' in 'ERC6909Claims' overrides 'ERC6909::afterDonate' which is not marked 'virtual' |
| warning | W200 | function 'afterDonate' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasPermission' in 'ERC6909Claims' overrides 'ERC6909::hasPermission' which is not marked 'virtual' |
| warning | W200 | function 'hasPermission' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'ERC6909Claims' overrides 'ERC6909::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'ERC6909Claims' overrides 'ERC6909::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'compress' in 'ERC6909Claims' overrides 'ERC6909::compress' which is not marked 'virtual' |
| warning | W200 | function 'compress' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'position' in 'ERC6909Claims' overrides 'ERC6909::position' which is not marked 'virtual' |
| warning | W200 | function 'position' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'flipTick' in 'ERC6909Claims' overrides 'ERC6909::flipTick' which is not marked 'virtual' |
| warning | W200 | function 'flipTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'ERC6909Claims' overrides 'ERC6909::nextInitializedTickWithinOneWord' which is not marked 'virtual' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC6909Claims' overrides 'ERC6909::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'ERC6909Claims' overrides 'ERC6909::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'ERC6909Claims' overrides 'ERC6909::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ERC6909Claims' overrides 'ERC6909::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'ERC6909Claims' overrides 'ERC6909::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'ERC6909Claims' overrides 'ERC6909::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'divRoundingUp' in 'ERC6909Claims' overrides 'ERC6909::divRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'divRoundingUp' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'simpleMulDiv' in 'ERC6909Claims' overrides 'ERC6909::simpleMulDiv' which is not marked 'virtual' |
| warning | W200 | function 'simpleMulDiv' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'ERC6909Claims' overrides 'ERC6909::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'ERC6909Claims' overrides 'ERC6909::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ERC6909Claims' overrides 'ERC6909::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ERC6909Claims' overrides 'ERC6909::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'ERC6909Claims' overrides 'ERC6909::getNextSqrtPriceFromAmount0RoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'ERC6909Claims' overrides 'ERC6909::getNextSqrtPriceFromAmount1RoundingDown' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'ERC6909Claims' overrides 'ERC6909::getNextSqrtPriceFromInput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'ERC6909Claims' overrides 'ERC6909::getNextSqrtPriceFromOutput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'ERC6909Claims' overrides 'ERC6909::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'absDiff' in 'ERC6909Claims' overrides 'ERC6909::absDiff' which is not marked 'virtual' |
| warning | W200 | function 'absDiff' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'ERC6909Claims' overrides 'ERC6909::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'ERC6909Claims' overrides 'ERC6909::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'ERC6909Claims' overrides 'ERC6909::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceTarget' in 'ERC6909Claims' overrides 'ERC6909::getSqrtPriceTarget' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceTarget' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'computeSwapStep' in 'ERC6909Claims' overrides 'ERC6909::computeSwapStep' which is not marked 'virtual' |
| warning | W200 | function 'computeSwapStep' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrtPriceX96' in 'ERC6909Claims' overrides 'ERC6909::sqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'sqrtPriceX96' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tick' in 'ERC6909Claims' overrides 'ERC6909::tick' which is not marked 'virtual' |
| warning | W200 | function 'tick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'protocolFee' in 'ERC6909Claims' overrides 'ERC6909::protocolFee' which is not marked 'virtual' |
| warning | W200 | function 'protocolFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lpFee' in 'ERC6909Claims' overrides 'ERC6909::lpFee' which is not marked 'virtual' |
| warning | W200 | function 'lpFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSqrtPriceX96' in 'ERC6909Claims' overrides 'ERC6909::setSqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'setSqrtPriceX96' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setTick' in 'ERC6909Claims' overrides 'ERC6909::setTick' which is not marked 'virtual' |
| warning | W200 | function 'setTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'ERC6909Claims' overrides 'ERC6909::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLpFee' in 'ERC6909Claims' overrides 'ERC6909::setLpFee' which is not marked 'virtual' |
| warning | W200 | function 'setLpFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getZeroForOneFee' in 'ERC6909Claims' overrides 'ERC6909::getZeroForOneFee' which is not marked 'virtual' |
| warning | W200 | function 'getZeroForOneFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getOneForZeroFee' in 'ERC6909Claims' overrides 'ERC6909::getOneForZeroFee' which is not marked 'virtual' |
| warning | W200 | function 'getOneForZeroFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidProtocolFee' in 'ERC6909Claims' overrides 'ERC6909::isValidProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'isValidProtocolFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateSwapFee' in 'ERC6909Claims' overrides 'ERC6909::calculateSwapFee' which is not marked 'virtual' |
| warning | W200 | function 'calculateSwapFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkTicks' in 'ERC6909Claims' overrides 'ERC6909::checkTicks' which is not marked 'virtual' |
| warning | W200 | function 'checkTicks' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'ERC6909Claims' overrides 'ERC6909::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'ERC6909Claims' overrides 'ERC6909::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLPFee' in 'ERC6909Claims' overrides 'ERC6909::setLPFee' which is not marked 'virtual' |
| warning | W200 | function 'setLPFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modifyLiquidity' in 'ERC6909Claims' overrides 'ERC6909::modifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'modifyLiquidity' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'swap' in 'ERC6909Claims' overrides 'ERC6909::swap' which is not marked 'virtual' |
| warning | W200 | function 'swap' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'donate' in 'ERC6909Claims' overrides 'ERC6909::donate' which is not marked 'virtual' |
| warning | W200 | function 'donate' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'ERC6909Claims' overrides 'ERC6909::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateTick' in 'ERC6909Claims' overrides 'ERC6909::updateTick' which is not marked 'virtual' |
| warning | W200 | function 'updateTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'ERC6909Claims' overrides 'ERC6909::tickSpacingToMaxLiquidityPerTick' which is not marked 'virtual' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkPoolInitialized' in 'ERC6909Claims' overrides 'ERC6909::checkPoolInitialized' which is not marked 'virtual' |
| warning | W200 | function 'checkPoolInitialized' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clearTick' in 'ERC6909Claims' overrides 'ERC6909::clearTick' which is not marked 'virtual' |
| warning | W200 | function 'clearTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'crossTick' in 'ERC6909Claims' overrides 'ERC6909::crossTick' which is not marked 'virtual' |
| warning | W200 | function 'crossTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC6909Claims' overrides 'ERC6909::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'ERC6909Claims' overrides 'ERC6909::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ERC6909Claims' overrides 'ERC6909::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ERC6909Claims' overrides 'ERC6909::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'ERC6909Claims' overrides 'ERC6909::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'ERC6909Claims' overrides 'ERC6909::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'ERC6909Claims' overrides 'ERC6909::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function '_computeSlot' in 'ERC6909Claims' overrides 'ERC6909::_computeSlot' which is not marked 'virtual' |
| warning | W200 | function '_computeSlot' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDelta' in 'ERC6909Claims' overrides 'ERC6909::getDelta' which is not marked 'virtual' |
| warning | W200 | function 'getDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyDelta' in 'ERC6909Claims' overrides 'ERC6909::applyDelta' which is not marked 'virtual' |
| warning | W200 | function 'applyDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'ERC6909Claims' overrides 'ERC6909::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'ERC6909Claims' overrides 'ERC6909::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'ERC6909Claims' overrides 'ERC6909::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ERC6909Claims' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W130 | overloaded function 'extsload' with 1 parameter(s) uses Neo overload mangling; external callers must invoke the generated Neo method names |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Extsload' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W130 | overloaded function 'exttload' with 1 parameter(s) uses Neo overload mangling; external callers must invoke the generated Neo method names |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Exttload' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W130 | overloaded function 'extsload' with 1 parameter(s) uses Neo overload mangling; external callers must invoke the generated Neo method names |
| warning | W130 | overloaded function 'exttload' with 1 parameter(s) uses Neo overload mangling; external callers must invoke the generated Neo method names |
| warning | W111 | function 'settle' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'settleFor' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DYNAMIC_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVERRIDE_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REMOVE_OVERRIDE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ALL_HOOK_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SWAP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_160_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROTOCOL_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LP_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_0_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_1_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PIPS_DENOMINATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DYNAMIC_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVERRIDE_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REMOVE_OVERRIDE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ALL_HOOK_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SWAP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_160_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROTOCOL_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LP_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_0_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_1_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PIPS_DENOMINATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W123 | public state variable 'balanceOf' conflicts with a function of the same name |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DYNAMIC_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVERRIDE_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REMOVE_OVERRIDE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ALL_HOOK_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SWAP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_160_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROTOCOL_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LP_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_0_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_1_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PIPS_DENOMINATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DYNAMIC_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVERRIDE_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REMOVE_OVERRIDE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ALL_HOOK_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SWAP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_160_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROTOCOL_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LP_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_0_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_1_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PIPS_DENOMINATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DYNAMIC_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVERRIDE_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REMOVE_OVERRIDE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ALL_HOOK_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SWAP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_160_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROTOCOL_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LP_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_0_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_1_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PIPS_DENOMINATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DYNAMIC_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVERRIDE_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REMOVE_OVERRIDE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ALL_HOOK_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SWAP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_160_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROTOCOL_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LP_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_0_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_1_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PIPS_DENOMINATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DYNAMIC_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVERRIDE_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REMOVE_OVERRIDE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ALL_HOOK_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_INITIALIZE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_DONATE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BEFORE_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_SWAP_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_ADD_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SWAP_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_160_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROTOCOL_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LP_FEE_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_0_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_1_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PIPS_DENOMINATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '_getPool' return value 'Pool.State' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | W102 | function 'transfer' has 3 parameters, but NEP-17 requires 4: transfer(from, to, amount, data). The `data` parameter (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |
| warning | W116 | function 'settle' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'settleFor' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | VALIDATION_WARNING | 'using X for *' is supported; all library functions are available but type-specific filtering is not enforced at compile time |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides 'Owned::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ProtocolFees' overrides 'Owned::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'ProtocolFees' overrides 'Owned::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'ProtocolFees' overrides 'Owned::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'ProtocolFees' overrides 'Owned::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'ProtocolFees' overrides 'Owned::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ProtocolFees' overrides 'Owned::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'ProtocolFees' overrides 'Owned::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ProtocolFees' overrides 'Owned::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ProtocolFees' overrides 'Owned::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ProtocolFees' overrides 'Owned::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ProtocolFees' overrides 'Owned::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ProtocolFees' overrides 'Owned::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ProtocolFees' overrides 'Owned::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'ProtocolFees' overrides 'Owned::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'ProtocolFees' overrides 'Owned::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'ProtocolFees' overrides 'Owned::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ProtocolFees' overrides 'Owned::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ProtocolFees' overrides 'Owned::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isDynamicFee' in 'ProtocolFees' overrides 'Owned::isDynamicFee' which is not marked 'virtual' |
| warning | W200 | function 'isDynamicFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValid' in 'ProtocolFees' overrides 'Owned::isValid' which is not marked 'virtual' |
| warning | W200 | function 'isValid' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validate' in 'ProtocolFees' overrides 'Owned::validate' which is not marked 'virtual' |
| warning | W200 | function 'validate' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInitialLPFee' in 'ProtocolFees' overrides 'Owned::getInitialLPFee' which is not marked 'virtual' |
| warning | W200 | function 'getInitialLPFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isOverride' in 'ProtocolFees' overrides 'Owned::isOverride' which is not marked 'virtual' |
| warning | W200 | function 'isOverride' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlag' in 'ProtocolFees' overrides 'Owned::removeOverrideFlag' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlag' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'ProtocolFees' overrides 'Owned::removeOverrideFlagAndValidate' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'ProtocolFees' overrides 'Owned::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'ProtocolFees' overrides 'Owned::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'ProtocolFees' overrides 'Owned::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHookPermissions' in 'ProtocolFees' overrides 'Owned::validateHookPermissions' which is not marked 'virtual' |
| warning | W200 | function 'validateHookPermissions' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidHookAddress' in 'ProtocolFees' overrides 'Owned::isValidHookAddress' which is not marked 'virtual' |
| warning | W200 | function 'isValidHookAddress' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHook' in 'ProtocolFees' overrides 'Owned::callHook' which is not marked 'virtual' |
| warning | W200 | function 'callHook' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHookWithReturnDelta' in 'ProtocolFees' overrides 'Owned::callHookWithReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'callHookWithReturnDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'noSelfCall' in 'ProtocolFees' overrides 'Owned::noSelfCall' which is not marked 'virtual' |
| warning | W200 | function 'noSelfCall' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeInitialize' in 'ProtocolFees' overrides 'Owned::beforeInitialize' which is not marked 'virtual' |
| warning | W200 | function 'beforeInitialize' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterInitialize' in 'ProtocolFees' overrides 'Owned::afterInitialize' which is not marked 'virtual' |
| warning | W200 | function 'afterInitialize' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeModifyLiquidity' in 'ProtocolFees' overrides 'Owned::beforeModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'beforeModifyLiquidity' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterModifyLiquidity' in 'ProtocolFees' overrides 'Owned::afterModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'afterModifyLiquidity' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeSwap' in 'ProtocolFees' overrides 'Owned::beforeSwap' which is not marked 'virtual' |
| warning | W200 | function 'beforeSwap' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterSwap' in 'ProtocolFees' overrides 'Owned::afterSwap' which is not marked 'virtual' |
| warning | W200 | function 'afterSwap' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeDonate' in 'ProtocolFees' overrides 'Owned::beforeDonate' which is not marked 'virtual' |
| warning | W200 | function 'beforeDonate' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterDonate' in 'ProtocolFees' overrides 'Owned::afterDonate' which is not marked 'virtual' |
| warning | W200 | function 'afterDonate' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasPermission' in 'ProtocolFees' overrides 'Owned::hasPermission' which is not marked 'virtual' |
| warning | W200 | function 'hasPermission' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'ProtocolFees' overrides 'Owned::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'ProtocolFees' overrides 'Owned::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'compress' in 'ProtocolFees' overrides 'Owned::compress' which is not marked 'virtual' |
| warning | W200 | function 'compress' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'position' in 'ProtocolFees' overrides 'Owned::position' which is not marked 'virtual' |
| warning | W200 | function 'position' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'flipTick' in 'ProtocolFees' overrides 'Owned::flipTick' which is not marked 'virtual' |
| warning | W200 | function 'flipTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'ProtocolFees' overrides 'Owned::nextInitializedTickWithinOneWord' which is not marked 'virtual' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ProtocolFees' overrides 'Owned::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'ProtocolFees' overrides 'Owned::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'ProtocolFees' overrides 'Owned::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ProtocolFees' overrides 'Owned::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'ProtocolFees' overrides 'Owned::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'ProtocolFees' overrides 'Owned::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'divRoundingUp' in 'ProtocolFees' overrides 'Owned::divRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'divRoundingUp' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'simpleMulDiv' in 'ProtocolFees' overrides 'Owned::simpleMulDiv' which is not marked 'virtual' |
| warning | W200 | function 'simpleMulDiv' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'ProtocolFees' overrides 'Owned::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'ProtocolFees' overrides 'Owned::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ProtocolFees' overrides 'Owned::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ProtocolFees' overrides 'Owned::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'ProtocolFees' overrides 'Owned::getNextSqrtPriceFromAmount0RoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'ProtocolFees' overrides 'Owned::getNextSqrtPriceFromAmount1RoundingDown' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'ProtocolFees' overrides 'Owned::getNextSqrtPriceFromInput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'ProtocolFees' overrides 'Owned::getNextSqrtPriceFromOutput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'ProtocolFees' overrides 'Owned::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'absDiff' in 'ProtocolFees' overrides 'Owned::absDiff' which is not marked 'virtual' |
| warning | W200 | function 'absDiff' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'ProtocolFees' overrides 'Owned::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'ProtocolFees' overrides 'Owned::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'ProtocolFees' overrides 'Owned::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceTarget' in 'ProtocolFees' overrides 'Owned::getSqrtPriceTarget' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceTarget' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'computeSwapStep' in 'ProtocolFees' overrides 'Owned::computeSwapStep' which is not marked 'virtual' |
| warning | W200 | function 'computeSwapStep' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrtPriceX96' in 'ProtocolFees' overrides 'Owned::sqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'sqrtPriceX96' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tick' in 'ProtocolFees' overrides 'Owned::tick' which is not marked 'virtual' |
| warning | W200 | function 'tick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'protocolFee' in 'ProtocolFees' overrides 'Owned::protocolFee' which is not marked 'virtual' |
| warning | W200 | function 'protocolFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lpFee' in 'ProtocolFees' overrides 'Owned::lpFee' which is not marked 'virtual' |
| warning | W200 | function 'lpFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSqrtPriceX96' in 'ProtocolFees' overrides 'Owned::setSqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'setSqrtPriceX96' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setTick' in 'ProtocolFees' overrides 'Owned::setTick' which is not marked 'virtual' |
| warning | W200 | function 'setTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'ProtocolFees' overrides 'Owned::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLpFee' in 'ProtocolFees' overrides 'Owned::setLpFee' which is not marked 'virtual' |
| warning | W200 | function 'setLpFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getZeroForOneFee' in 'ProtocolFees' overrides 'Owned::getZeroForOneFee' which is not marked 'virtual' |
| warning | W200 | function 'getZeroForOneFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getOneForZeroFee' in 'ProtocolFees' overrides 'Owned::getOneForZeroFee' which is not marked 'virtual' |
| warning | W200 | function 'getOneForZeroFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidProtocolFee' in 'ProtocolFees' overrides 'Owned::isValidProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'isValidProtocolFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateSwapFee' in 'ProtocolFees' overrides 'Owned::calculateSwapFee' which is not marked 'virtual' |
| warning | W200 | function 'calculateSwapFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkTicks' in 'ProtocolFees' overrides 'Owned::checkTicks' which is not marked 'virtual' |
| warning | W200 | function 'checkTicks' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'ProtocolFees' overrides 'Owned::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'ProtocolFees' overrides 'Owned::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLPFee' in 'ProtocolFees' overrides 'Owned::setLPFee' which is not marked 'virtual' |
| warning | W200 | function 'setLPFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modifyLiquidity' in 'ProtocolFees' overrides 'Owned::modifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'modifyLiquidity' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'swap' in 'ProtocolFees' overrides 'Owned::swap' which is not marked 'virtual' |
| warning | W200 | function 'swap' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'donate' in 'ProtocolFees' overrides 'Owned::donate' which is not marked 'virtual' |
| warning | W200 | function 'donate' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'ProtocolFees' overrides 'Owned::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateTick' in 'ProtocolFees' overrides 'Owned::updateTick' which is not marked 'virtual' |
| warning | W200 | function 'updateTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'ProtocolFees' overrides 'Owned::tickSpacingToMaxLiquidityPerTick' which is not marked 'virtual' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkPoolInitialized' in 'ProtocolFees' overrides 'Owned::checkPoolInitialized' which is not marked 'virtual' |
| warning | W200 | function 'checkPoolInitialized' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clearTick' in 'ProtocolFees' overrides 'Owned::clearTick' which is not marked 'virtual' |
| warning | W200 | function 'clearTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'crossTick' in 'ProtocolFees' overrides 'Owned::crossTick' which is not marked 'virtual' |
| warning | W200 | function 'crossTick' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'ProtocolFees' overrides 'Owned::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'ProtocolFees' overrides 'Owned::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ProtocolFees' overrides 'Owned::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ProtocolFees' overrides 'Owned::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'ProtocolFees' overrides 'Owned::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'ProtocolFees' overrides 'Owned::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'ProtocolFees' overrides 'Owned::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function '_computeSlot' in 'ProtocolFees' overrides 'Owned::_computeSlot' which is not marked 'virtual' |
| warning | W200 | function '_computeSlot' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDelta' in 'ProtocolFees' overrides 'Owned::getDelta' which is not marked 'virtual' |
| warning | W200 | function 'getDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyDelta' in 'ProtocolFees' overrides 'Owned::applyDelta' which is not marked 'virtual' |
| warning | W200 | function 'applyDelta' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'ProtocolFees' overrides 'Owned::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'ProtocolFees' overrides 'Owned::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'ProtocolFees' overrides 'Owned::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides 'ProtocolFees::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides 'ProtocolFees::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides 'ProtocolFees::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides 'ProtocolFees::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides 'ProtocolFees::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides 'ProtocolFees::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides 'ProtocolFees::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'NoDelegateCall' overrides 'ProtocolFees::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'NoDelegateCall' overrides 'ProtocolFees::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'NoDelegateCall' overrides 'ProtocolFees::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'NoDelegateCall' overrides 'ProtocolFees::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'NoDelegateCall' overrides 'ProtocolFees::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'NoDelegateCall' overrides 'ProtocolFees::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'NoDelegateCall' overrides 'ProtocolFees::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'NoDelegateCall' overrides 'ProtocolFees::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'NoDelegateCall' overrides 'ProtocolFees::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'NoDelegateCall' overrides 'ProtocolFees::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'NoDelegateCall' overrides 'ProtocolFees::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'NoDelegateCall' overrides 'ProtocolFees::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'NoDelegateCall' overrides 'ProtocolFees::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'NoDelegateCall' overrides 'ProtocolFees::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'NoDelegateCall' overrides 'ProtocolFees::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'NoDelegateCall' overrides 'ProtocolFees::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'NoDelegateCall' overrides 'ProtocolFees::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'NoDelegateCall' overrides 'ProtocolFees::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isDynamicFee' in 'NoDelegateCall' overrides 'ProtocolFees::isDynamicFee' which is not marked 'virtual' |
| warning | W200 | function 'isDynamicFee' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValid' in 'NoDelegateCall' overrides 'ProtocolFees::isValid' which is not marked 'virtual' |
| warning | W200 | function 'isValid' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validate' in 'NoDelegateCall' overrides 'ProtocolFees::validate' which is not marked 'virtual' |
| warning | W200 | function 'validate' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInitialLPFee' in 'NoDelegateCall' overrides 'ProtocolFees::getInitialLPFee' which is not marked 'virtual' |
| warning | W200 | function 'getInitialLPFee' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isOverride' in 'NoDelegateCall' overrides 'ProtocolFees::isOverride' which is not marked 'virtual' |
| warning | W200 | function 'isOverride' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlag' in 'NoDelegateCall' overrides 'ProtocolFees::removeOverrideFlag' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlag' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'NoDelegateCall' overrides 'ProtocolFees::removeOverrideFlagAndValidate' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'NoDelegateCall' overrides 'ProtocolFees::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'NoDelegateCall' overrides 'ProtocolFees::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'NoDelegateCall' overrides 'ProtocolFees::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHookPermissions' in 'NoDelegateCall' overrides 'ProtocolFees::validateHookPermissions' which is not marked 'virtual' |
| warning | W200 | function 'validateHookPermissions' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidHookAddress' in 'NoDelegateCall' overrides 'ProtocolFees::isValidHookAddress' which is not marked 'virtual' |
| warning | W200 | function 'isValidHookAddress' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHook' in 'NoDelegateCall' overrides 'ProtocolFees::callHook' which is not marked 'virtual' |
| warning | W200 | function 'callHook' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHookWithReturnDelta' in 'NoDelegateCall' overrides 'ProtocolFees::callHookWithReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'callHookWithReturnDelta' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'noSelfCall' in 'NoDelegateCall' overrides 'ProtocolFees::noSelfCall' which is not marked 'virtual' |
| warning | W200 | function 'noSelfCall' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeInitialize' in 'NoDelegateCall' overrides 'ProtocolFees::beforeInitialize' which is not marked 'virtual' |
| warning | W200 | function 'beforeInitialize' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterInitialize' in 'NoDelegateCall' overrides 'ProtocolFees::afterInitialize' which is not marked 'virtual' |
| warning | W200 | function 'afterInitialize' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeModifyLiquidity' in 'NoDelegateCall' overrides 'ProtocolFees::beforeModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'beforeModifyLiquidity' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterModifyLiquidity' in 'NoDelegateCall' overrides 'ProtocolFees::afterModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'afterModifyLiquidity' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeSwap' in 'NoDelegateCall' overrides 'ProtocolFees::beforeSwap' which is not marked 'virtual' |
| warning | W200 | function 'beforeSwap' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterSwap' in 'NoDelegateCall' overrides 'ProtocolFees::afterSwap' which is not marked 'virtual' |
| warning | W200 | function 'afterSwap' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeDonate' in 'NoDelegateCall' overrides 'ProtocolFees::beforeDonate' which is not marked 'virtual' |
| warning | W200 | function 'beforeDonate' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterDonate' in 'NoDelegateCall' overrides 'ProtocolFees::afterDonate' which is not marked 'virtual' |
| warning | W200 | function 'afterDonate' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasPermission' in 'NoDelegateCall' overrides 'ProtocolFees::hasPermission' which is not marked 'virtual' |
| warning | W200 | function 'hasPermission' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'NoDelegateCall' overrides 'ProtocolFees::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'NoDelegateCall' overrides 'ProtocolFees::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'compress' in 'NoDelegateCall' overrides 'ProtocolFees::compress' which is not marked 'virtual' |
| warning | W200 | function 'compress' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'position' in 'NoDelegateCall' overrides 'ProtocolFees::position' which is not marked 'virtual' |
| warning | W200 | function 'position' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'flipTick' in 'NoDelegateCall' overrides 'ProtocolFees::flipTick' which is not marked 'virtual' |
| warning | W200 | function 'flipTick' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'NoDelegateCall' overrides 'ProtocolFees::nextInitializedTickWithinOneWord' which is not marked 'virtual' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'NoDelegateCall' overrides 'ProtocolFees::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'NoDelegateCall' overrides 'ProtocolFees::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'NoDelegateCall' overrides 'ProtocolFees::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'NoDelegateCall' overrides 'ProtocolFees::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'NoDelegateCall' overrides 'ProtocolFees::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'NoDelegateCall' overrides 'ProtocolFees::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'divRoundingUp' in 'NoDelegateCall' overrides 'ProtocolFees::divRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'divRoundingUp' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'simpleMulDiv' in 'NoDelegateCall' overrides 'ProtocolFees::simpleMulDiv' which is not marked 'virtual' |
| warning | W200 | function 'simpleMulDiv' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'NoDelegateCall' overrides 'ProtocolFees::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'NoDelegateCall' overrides 'ProtocolFees::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'NoDelegateCall' overrides 'ProtocolFees::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'NoDelegateCall' overrides 'ProtocolFees::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'NoDelegateCall' overrides 'ProtocolFees::getNextSqrtPriceFromAmount0RoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'NoDelegateCall' overrides 'ProtocolFees::getNextSqrtPriceFromAmount1RoundingDown' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'NoDelegateCall' overrides 'ProtocolFees::getNextSqrtPriceFromInput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'NoDelegateCall' overrides 'ProtocolFees::getNextSqrtPriceFromOutput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'NoDelegateCall' overrides 'ProtocolFees::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'absDiff' in 'NoDelegateCall' overrides 'ProtocolFees::absDiff' which is not marked 'virtual' |
| warning | W200 | function 'absDiff' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'NoDelegateCall' overrides 'ProtocolFees::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'NoDelegateCall' overrides 'ProtocolFees::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'NoDelegateCall' overrides 'ProtocolFees::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceTarget' in 'NoDelegateCall' overrides 'ProtocolFees::getSqrtPriceTarget' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceTarget' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'computeSwapStep' in 'NoDelegateCall' overrides 'ProtocolFees::computeSwapStep' which is not marked 'virtual' |
| warning | W200 | function 'computeSwapStep' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrtPriceX96' in 'NoDelegateCall' overrides 'ProtocolFees::sqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'sqrtPriceX96' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tick' in 'NoDelegateCall' overrides 'ProtocolFees::tick' which is not marked 'virtual' |
| warning | W200 | function 'tick' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'protocolFee' in 'NoDelegateCall' overrides 'ProtocolFees::protocolFee' which is not marked 'virtual' |
| warning | W200 | function 'protocolFee' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lpFee' in 'NoDelegateCall' overrides 'ProtocolFees::lpFee' which is not marked 'virtual' |
| warning | W200 | function 'lpFee' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSqrtPriceX96' in 'NoDelegateCall' overrides 'ProtocolFees::setSqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'setSqrtPriceX96' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setTick' in 'NoDelegateCall' overrides 'ProtocolFees::setTick' which is not marked 'virtual' |
| warning | W200 | function 'setTick' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'NoDelegateCall' overrides 'ProtocolFees::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLpFee' in 'NoDelegateCall' overrides 'ProtocolFees::setLpFee' which is not marked 'virtual' |
| warning | W200 | function 'setLpFee' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getZeroForOneFee' in 'NoDelegateCall' overrides 'ProtocolFees::getZeroForOneFee' which is not marked 'virtual' |
| warning | W200 | function 'getZeroForOneFee' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getOneForZeroFee' in 'NoDelegateCall' overrides 'ProtocolFees::getOneForZeroFee' which is not marked 'virtual' |
| warning | W200 | function 'getOneForZeroFee' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidProtocolFee' in 'NoDelegateCall' overrides 'ProtocolFees::isValidProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'isValidProtocolFee' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateSwapFee' in 'NoDelegateCall' overrides 'ProtocolFees::calculateSwapFee' which is not marked 'virtual' |
| warning | W200 | function 'calculateSwapFee' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkTicks' in 'NoDelegateCall' overrides 'ProtocolFees::checkTicks' which is not marked 'virtual' |
| warning | W200 | function 'checkTicks' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'NoDelegateCall' overrides 'ProtocolFees::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'NoDelegateCall' overrides 'ProtocolFees::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLPFee' in 'NoDelegateCall' overrides 'ProtocolFees::setLPFee' which is not marked 'virtual' |
| warning | W200 | function 'setLPFee' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modifyLiquidity' in 'NoDelegateCall' overrides 'ProtocolFees::modifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'modifyLiquidity' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'swap' in 'NoDelegateCall' overrides 'ProtocolFees::swap' which is not marked 'virtual' |
| warning | W200 | function 'swap' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'donate' in 'NoDelegateCall' overrides 'ProtocolFees::donate' which is not marked 'virtual' |
| warning | W200 | function 'donate' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'NoDelegateCall' overrides 'ProtocolFees::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateTick' in 'NoDelegateCall' overrides 'ProtocolFees::updateTick' which is not marked 'virtual' |
| warning | W200 | function 'updateTick' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'NoDelegateCall' overrides 'ProtocolFees::tickSpacingToMaxLiquidityPerTick' which is not marked 'virtual' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkPoolInitialized' in 'NoDelegateCall' overrides 'ProtocolFees::checkPoolInitialized' which is not marked 'virtual' |
| warning | W200 | function 'checkPoolInitialized' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clearTick' in 'NoDelegateCall' overrides 'ProtocolFees::clearTick' which is not marked 'virtual' |
| warning | W200 | function 'clearTick' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'crossTick' in 'NoDelegateCall' overrides 'ProtocolFees::crossTick' which is not marked 'virtual' |
| warning | W200 | function 'crossTick' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'NoDelegateCall' overrides 'ProtocolFees::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'NoDelegateCall' overrides 'ProtocolFees::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'NoDelegateCall' overrides 'ProtocolFees::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'NoDelegateCall' overrides 'ProtocolFees::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'NoDelegateCall' overrides 'ProtocolFees::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'NoDelegateCall' overrides 'ProtocolFees::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'NoDelegateCall' overrides 'ProtocolFees::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function '_computeSlot' in 'NoDelegateCall' overrides 'ProtocolFees::_computeSlot' which is not marked 'virtual' |
| warning | W200 | function '_computeSlot' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDelta' in 'NoDelegateCall' overrides 'ProtocolFees::getDelta' which is not marked 'virtual' |
| warning | W200 | function 'getDelta' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyDelta' in 'NoDelegateCall' overrides 'ProtocolFees::applyDelta' which is not marked 'virtual' |
| warning | W200 | function 'applyDelta' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'NoDelegateCall' overrides 'ProtocolFees::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'NoDelegateCall' overrides 'ProtocolFees::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'NoDelegateCall' overrides 'ProtocolFees::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'NoDelegateCall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides 'NoDelegateCall::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides 'NoDelegateCall::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides 'NoDelegateCall::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides 'NoDelegateCall::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides 'NoDelegateCall::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides 'NoDelegateCall::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides 'NoDelegateCall::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ERC6909' overrides 'NoDelegateCall::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'ERC6909' overrides 'NoDelegateCall::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'ERC6909' overrides 'NoDelegateCall::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'ERC6909' overrides 'NoDelegateCall::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'ERC6909' overrides 'NoDelegateCall::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ERC6909' overrides 'NoDelegateCall::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'ERC6909' overrides 'NoDelegateCall::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC6909' overrides 'NoDelegateCall::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC6909' overrides 'NoDelegateCall::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC6909' overrides 'NoDelegateCall::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC6909' overrides 'NoDelegateCall::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC6909' overrides 'NoDelegateCall::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC6909' overrides 'NoDelegateCall::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'ERC6909' overrides 'NoDelegateCall::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'ERC6909' overrides 'NoDelegateCall::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'ERC6909' overrides 'NoDelegateCall::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ERC6909' overrides 'NoDelegateCall::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ERC6909' overrides 'NoDelegateCall::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isDynamicFee' in 'ERC6909' overrides 'NoDelegateCall::isDynamicFee' which is not marked 'virtual' |
| warning | W200 | function 'isDynamicFee' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValid' in 'ERC6909' overrides 'NoDelegateCall::isValid' which is not marked 'virtual' |
| warning | W200 | function 'isValid' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validate' in 'ERC6909' overrides 'NoDelegateCall::validate' which is not marked 'virtual' |
| warning | W200 | function 'validate' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInitialLPFee' in 'ERC6909' overrides 'NoDelegateCall::getInitialLPFee' which is not marked 'virtual' |
| warning | W200 | function 'getInitialLPFee' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isOverride' in 'ERC6909' overrides 'NoDelegateCall::isOverride' which is not marked 'virtual' |
| warning | W200 | function 'isOverride' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlag' in 'ERC6909' overrides 'NoDelegateCall::removeOverrideFlag' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlag' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'ERC6909' overrides 'NoDelegateCall::removeOverrideFlagAndValidate' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'ERC6909' overrides 'NoDelegateCall::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'ERC6909' overrides 'NoDelegateCall::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'ERC6909' overrides 'NoDelegateCall::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHookPermissions' in 'ERC6909' overrides 'NoDelegateCall::validateHookPermissions' which is not marked 'virtual' |
| warning | W200 | function 'validateHookPermissions' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidHookAddress' in 'ERC6909' overrides 'NoDelegateCall::isValidHookAddress' which is not marked 'virtual' |
| warning | W200 | function 'isValidHookAddress' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHook' in 'ERC6909' overrides 'NoDelegateCall::callHook' which is not marked 'virtual' |
| warning | W200 | function 'callHook' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHookWithReturnDelta' in 'ERC6909' overrides 'NoDelegateCall::callHookWithReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'callHookWithReturnDelta' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'noSelfCall' in 'ERC6909' overrides 'NoDelegateCall::noSelfCall' which is not marked 'virtual' |
| warning | W200 | function 'noSelfCall' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeInitialize' in 'ERC6909' overrides 'NoDelegateCall::beforeInitialize' which is not marked 'virtual' |
| warning | W200 | function 'beforeInitialize' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterInitialize' in 'ERC6909' overrides 'NoDelegateCall::afterInitialize' which is not marked 'virtual' |
| warning | W200 | function 'afterInitialize' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeModifyLiquidity' in 'ERC6909' overrides 'NoDelegateCall::beforeModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'beforeModifyLiquidity' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterModifyLiquidity' in 'ERC6909' overrides 'NoDelegateCall::afterModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'afterModifyLiquidity' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeSwap' in 'ERC6909' overrides 'NoDelegateCall::beforeSwap' which is not marked 'virtual' |
| warning | W200 | function 'beforeSwap' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterSwap' in 'ERC6909' overrides 'NoDelegateCall::afterSwap' which is not marked 'virtual' |
| warning | W200 | function 'afterSwap' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeDonate' in 'ERC6909' overrides 'NoDelegateCall::beforeDonate' which is not marked 'virtual' |
| warning | W200 | function 'beforeDonate' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterDonate' in 'ERC6909' overrides 'NoDelegateCall::afterDonate' which is not marked 'virtual' |
| warning | W200 | function 'afterDonate' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasPermission' in 'ERC6909' overrides 'NoDelegateCall::hasPermission' which is not marked 'virtual' |
| warning | W200 | function 'hasPermission' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'ERC6909' overrides 'NoDelegateCall::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'ERC6909' overrides 'NoDelegateCall::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'compress' in 'ERC6909' overrides 'NoDelegateCall::compress' which is not marked 'virtual' |
| warning | W200 | function 'compress' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'position' in 'ERC6909' overrides 'NoDelegateCall::position' which is not marked 'virtual' |
| warning | W200 | function 'position' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'flipTick' in 'ERC6909' overrides 'NoDelegateCall::flipTick' which is not marked 'virtual' |
| warning | W200 | function 'flipTick' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'ERC6909' overrides 'NoDelegateCall::nextInitializedTickWithinOneWord' which is not marked 'virtual' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC6909' overrides 'NoDelegateCall::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'ERC6909' overrides 'NoDelegateCall::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'ERC6909' overrides 'NoDelegateCall::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ERC6909' overrides 'NoDelegateCall::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'ERC6909' overrides 'NoDelegateCall::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'ERC6909' overrides 'NoDelegateCall::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'divRoundingUp' in 'ERC6909' overrides 'NoDelegateCall::divRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'divRoundingUp' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'simpleMulDiv' in 'ERC6909' overrides 'NoDelegateCall::simpleMulDiv' which is not marked 'virtual' |
| warning | W200 | function 'simpleMulDiv' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'ERC6909' overrides 'NoDelegateCall::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'ERC6909' overrides 'NoDelegateCall::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ERC6909' overrides 'NoDelegateCall::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ERC6909' overrides 'NoDelegateCall::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'ERC6909' overrides 'NoDelegateCall::getNextSqrtPriceFromAmount0RoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'ERC6909' overrides 'NoDelegateCall::getNextSqrtPriceFromAmount1RoundingDown' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'ERC6909' overrides 'NoDelegateCall::getNextSqrtPriceFromInput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'ERC6909' overrides 'NoDelegateCall::getNextSqrtPriceFromOutput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'ERC6909' overrides 'NoDelegateCall::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'absDiff' in 'ERC6909' overrides 'NoDelegateCall::absDiff' which is not marked 'virtual' |
| warning | W200 | function 'absDiff' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'ERC6909' overrides 'NoDelegateCall::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'ERC6909' overrides 'NoDelegateCall::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'ERC6909' overrides 'NoDelegateCall::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceTarget' in 'ERC6909' overrides 'NoDelegateCall::getSqrtPriceTarget' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceTarget' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'computeSwapStep' in 'ERC6909' overrides 'NoDelegateCall::computeSwapStep' which is not marked 'virtual' |
| warning | W200 | function 'computeSwapStep' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrtPriceX96' in 'ERC6909' overrides 'NoDelegateCall::sqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'sqrtPriceX96' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tick' in 'ERC6909' overrides 'NoDelegateCall::tick' which is not marked 'virtual' |
| warning | W200 | function 'tick' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'protocolFee' in 'ERC6909' overrides 'NoDelegateCall::protocolFee' which is not marked 'virtual' |
| warning | W200 | function 'protocolFee' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lpFee' in 'ERC6909' overrides 'NoDelegateCall::lpFee' which is not marked 'virtual' |
| warning | W200 | function 'lpFee' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSqrtPriceX96' in 'ERC6909' overrides 'NoDelegateCall::setSqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'setSqrtPriceX96' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setTick' in 'ERC6909' overrides 'NoDelegateCall::setTick' which is not marked 'virtual' |
| warning | W200 | function 'setTick' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'ERC6909' overrides 'NoDelegateCall::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLpFee' in 'ERC6909' overrides 'NoDelegateCall::setLpFee' which is not marked 'virtual' |
| warning | W200 | function 'setLpFee' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getZeroForOneFee' in 'ERC6909' overrides 'NoDelegateCall::getZeroForOneFee' which is not marked 'virtual' |
| warning | W200 | function 'getZeroForOneFee' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getOneForZeroFee' in 'ERC6909' overrides 'NoDelegateCall::getOneForZeroFee' which is not marked 'virtual' |
| warning | W200 | function 'getOneForZeroFee' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidProtocolFee' in 'ERC6909' overrides 'NoDelegateCall::isValidProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'isValidProtocolFee' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateSwapFee' in 'ERC6909' overrides 'NoDelegateCall::calculateSwapFee' which is not marked 'virtual' |
| warning | W200 | function 'calculateSwapFee' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkTicks' in 'ERC6909' overrides 'NoDelegateCall::checkTicks' which is not marked 'virtual' |
| warning | W200 | function 'checkTicks' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'ERC6909' overrides 'NoDelegateCall::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'ERC6909' overrides 'NoDelegateCall::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLPFee' in 'ERC6909' overrides 'NoDelegateCall::setLPFee' which is not marked 'virtual' |
| warning | W200 | function 'setLPFee' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modifyLiquidity' in 'ERC6909' overrides 'NoDelegateCall::modifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'modifyLiquidity' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'swap' in 'ERC6909' overrides 'NoDelegateCall::swap' which is not marked 'virtual' |
| warning | W200 | function 'swap' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'donate' in 'ERC6909' overrides 'NoDelegateCall::donate' which is not marked 'virtual' |
| warning | W200 | function 'donate' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'ERC6909' overrides 'NoDelegateCall::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateTick' in 'ERC6909' overrides 'NoDelegateCall::updateTick' which is not marked 'virtual' |
| warning | W200 | function 'updateTick' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'ERC6909' overrides 'NoDelegateCall::tickSpacingToMaxLiquidityPerTick' which is not marked 'virtual' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkPoolInitialized' in 'ERC6909' overrides 'NoDelegateCall::checkPoolInitialized' which is not marked 'virtual' |
| warning | W200 | function 'checkPoolInitialized' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clearTick' in 'ERC6909' overrides 'NoDelegateCall::clearTick' which is not marked 'virtual' |
| warning | W200 | function 'clearTick' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'crossTick' in 'ERC6909' overrides 'NoDelegateCall::crossTick' which is not marked 'virtual' |
| warning | W200 | function 'crossTick' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC6909' overrides 'NoDelegateCall::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'ERC6909' overrides 'NoDelegateCall::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ERC6909' overrides 'NoDelegateCall::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ERC6909' overrides 'NoDelegateCall::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'ERC6909' overrides 'NoDelegateCall::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'ERC6909' overrides 'NoDelegateCall::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'ERC6909' overrides 'NoDelegateCall::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function '_computeSlot' in 'ERC6909' overrides 'NoDelegateCall::_computeSlot' which is not marked 'virtual' |
| warning | W200 | function '_computeSlot' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDelta' in 'ERC6909' overrides 'NoDelegateCall::getDelta' which is not marked 'virtual' |
| warning | W200 | function 'getDelta' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyDelta' in 'ERC6909' overrides 'NoDelegateCall::applyDelta' which is not marked 'virtual' |
| warning | W200 | function 'applyDelta' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'ERC6909' overrides 'NoDelegateCall::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'ERC6909' overrides 'NoDelegateCall::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'ERC6909' overrides 'NoDelegateCall::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'ERC6909' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides 'ERC6909::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ERC6909Claims' overrides 'ERC6909::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'ERC6909Claims' overrides 'ERC6909::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'ERC6909Claims' overrides 'ERC6909::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'ERC6909Claims' overrides 'ERC6909::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'ERC6909Claims' overrides 'ERC6909::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ERC6909Claims' overrides 'ERC6909::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'ERC6909Claims' overrides 'ERC6909::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC6909Claims' overrides 'ERC6909::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC6909Claims' overrides 'ERC6909::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC6909Claims' overrides 'ERC6909::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC6909Claims' overrides 'ERC6909::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC6909Claims' overrides 'ERC6909::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC6909Claims' overrides 'ERC6909::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'ERC6909Claims' overrides 'ERC6909::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'ERC6909Claims' overrides 'ERC6909::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'ERC6909Claims' overrides 'ERC6909::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ERC6909Claims' overrides 'ERC6909::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ERC6909Claims' overrides 'ERC6909::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isDynamicFee' in 'ERC6909Claims' overrides 'ERC6909::isDynamicFee' which is not marked 'virtual' |
| warning | W200 | function 'isDynamicFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValid' in 'ERC6909Claims' overrides 'ERC6909::isValid' which is not marked 'virtual' |
| warning | W200 | function 'isValid' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validate' in 'ERC6909Claims' overrides 'ERC6909::validate' which is not marked 'virtual' |
| warning | W200 | function 'validate' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInitialLPFee' in 'ERC6909Claims' overrides 'ERC6909::getInitialLPFee' which is not marked 'virtual' |
| warning | W200 | function 'getInitialLPFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isOverride' in 'ERC6909Claims' overrides 'ERC6909::isOverride' which is not marked 'virtual' |
| warning | W200 | function 'isOverride' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlag' in 'ERC6909Claims' overrides 'ERC6909::removeOverrideFlag' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlag' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'ERC6909Claims' overrides 'ERC6909::removeOverrideFlagAndValidate' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'ERC6909Claims' overrides 'ERC6909::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'ERC6909Claims' overrides 'ERC6909::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'ERC6909Claims' overrides 'ERC6909::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHookPermissions' in 'ERC6909Claims' overrides 'ERC6909::validateHookPermissions' which is not marked 'virtual' |
| warning | W200 | function 'validateHookPermissions' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidHookAddress' in 'ERC6909Claims' overrides 'ERC6909::isValidHookAddress' which is not marked 'virtual' |
| warning | W200 | function 'isValidHookAddress' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHook' in 'ERC6909Claims' overrides 'ERC6909::callHook' which is not marked 'virtual' |
| warning | W200 | function 'callHook' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHookWithReturnDelta' in 'ERC6909Claims' overrides 'ERC6909::callHookWithReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'callHookWithReturnDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'noSelfCall' in 'ERC6909Claims' overrides 'ERC6909::noSelfCall' which is not marked 'virtual' |
| warning | W200 | function 'noSelfCall' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeInitialize' in 'ERC6909Claims' overrides 'ERC6909::beforeInitialize' which is not marked 'virtual' |
| warning | W200 | function 'beforeInitialize' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterInitialize' in 'ERC6909Claims' overrides 'ERC6909::afterInitialize' which is not marked 'virtual' |
| warning | W200 | function 'afterInitialize' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeModifyLiquidity' in 'ERC6909Claims' overrides 'ERC6909::beforeModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'beforeModifyLiquidity' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterModifyLiquidity' in 'ERC6909Claims' overrides 'ERC6909::afterModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'afterModifyLiquidity' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeSwap' in 'ERC6909Claims' overrides 'ERC6909::beforeSwap' which is not marked 'virtual' |
| warning | W200 | function 'beforeSwap' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterSwap' in 'ERC6909Claims' overrides 'ERC6909::afterSwap' which is not marked 'virtual' |
| warning | W200 | function 'afterSwap' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeDonate' in 'ERC6909Claims' overrides 'ERC6909::beforeDonate' which is not marked 'virtual' |
| warning | W200 | function 'beforeDonate' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterDonate' in 'ERC6909Claims' overrides 'ERC6909::afterDonate' which is not marked 'virtual' |
| warning | W200 | function 'afterDonate' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasPermission' in 'ERC6909Claims' overrides 'ERC6909::hasPermission' which is not marked 'virtual' |
| warning | W200 | function 'hasPermission' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'ERC6909Claims' overrides 'ERC6909::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'ERC6909Claims' overrides 'ERC6909::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'compress' in 'ERC6909Claims' overrides 'ERC6909::compress' which is not marked 'virtual' |
| warning | W200 | function 'compress' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'position' in 'ERC6909Claims' overrides 'ERC6909::position' which is not marked 'virtual' |
| warning | W200 | function 'position' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'flipTick' in 'ERC6909Claims' overrides 'ERC6909::flipTick' which is not marked 'virtual' |
| warning | W200 | function 'flipTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'ERC6909Claims' overrides 'ERC6909::nextInitializedTickWithinOneWord' which is not marked 'virtual' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC6909Claims' overrides 'ERC6909::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'ERC6909Claims' overrides 'ERC6909::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'ERC6909Claims' overrides 'ERC6909::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ERC6909Claims' overrides 'ERC6909::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'ERC6909Claims' overrides 'ERC6909::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'ERC6909Claims' overrides 'ERC6909::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'divRoundingUp' in 'ERC6909Claims' overrides 'ERC6909::divRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'divRoundingUp' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'simpleMulDiv' in 'ERC6909Claims' overrides 'ERC6909::simpleMulDiv' which is not marked 'virtual' |
| warning | W200 | function 'simpleMulDiv' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'ERC6909Claims' overrides 'ERC6909::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'ERC6909Claims' overrides 'ERC6909::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ERC6909Claims' overrides 'ERC6909::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ERC6909Claims' overrides 'ERC6909::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'ERC6909Claims' overrides 'ERC6909::getNextSqrtPriceFromAmount0RoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'ERC6909Claims' overrides 'ERC6909::getNextSqrtPriceFromAmount1RoundingDown' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'ERC6909Claims' overrides 'ERC6909::getNextSqrtPriceFromInput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'ERC6909Claims' overrides 'ERC6909::getNextSqrtPriceFromOutput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'ERC6909Claims' overrides 'ERC6909::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'absDiff' in 'ERC6909Claims' overrides 'ERC6909::absDiff' which is not marked 'virtual' |
| warning | W200 | function 'absDiff' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'ERC6909Claims' overrides 'ERC6909::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'ERC6909Claims' overrides 'ERC6909::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'ERC6909Claims' overrides 'ERC6909::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceTarget' in 'ERC6909Claims' overrides 'ERC6909::getSqrtPriceTarget' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceTarget' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'computeSwapStep' in 'ERC6909Claims' overrides 'ERC6909::computeSwapStep' which is not marked 'virtual' |
| warning | W200 | function 'computeSwapStep' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrtPriceX96' in 'ERC6909Claims' overrides 'ERC6909::sqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'sqrtPriceX96' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tick' in 'ERC6909Claims' overrides 'ERC6909::tick' which is not marked 'virtual' |
| warning | W200 | function 'tick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'protocolFee' in 'ERC6909Claims' overrides 'ERC6909::protocolFee' which is not marked 'virtual' |
| warning | W200 | function 'protocolFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lpFee' in 'ERC6909Claims' overrides 'ERC6909::lpFee' which is not marked 'virtual' |
| warning | W200 | function 'lpFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSqrtPriceX96' in 'ERC6909Claims' overrides 'ERC6909::setSqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'setSqrtPriceX96' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setTick' in 'ERC6909Claims' overrides 'ERC6909::setTick' which is not marked 'virtual' |
| warning | W200 | function 'setTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'ERC6909Claims' overrides 'ERC6909::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLpFee' in 'ERC6909Claims' overrides 'ERC6909::setLpFee' which is not marked 'virtual' |
| warning | W200 | function 'setLpFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getZeroForOneFee' in 'ERC6909Claims' overrides 'ERC6909::getZeroForOneFee' which is not marked 'virtual' |
| warning | W200 | function 'getZeroForOneFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getOneForZeroFee' in 'ERC6909Claims' overrides 'ERC6909::getOneForZeroFee' which is not marked 'virtual' |
| warning | W200 | function 'getOneForZeroFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidProtocolFee' in 'ERC6909Claims' overrides 'ERC6909::isValidProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'isValidProtocolFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateSwapFee' in 'ERC6909Claims' overrides 'ERC6909::calculateSwapFee' which is not marked 'virtual' |
| warning | W200 | function 'calculateSwapFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkTicks' in 'ERC6909Claims' overrides 'ERC6909::checkTicks' which is not marked 'virtual' |
| warning | W200 | function 'checkTicks' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'ERC6909Claims' overrides 'ERC6909::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'ERC6909Claims' overrides 'ERC6909::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLPFee' in 'ERC6909Claims' overrides 'ERC6909::setLPFee' which is not marked 'virtual' |
| warning | W200 | function 'setLPFee' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modifyLiquidity' in 'ERC6909Claims' overrides 'ERC6909::modifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'modifyLiquidity' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'swap' in 'ERC6909Claims' overrides 'ERC6909::swap' which is not marked 'virtual' |
| warning | W200 | function 'swap' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'donate' in 'ERC6909Claims' overrides 'ERC6909::donate' which is not marked 'virtual' |
| warning | W200 | function 'donate' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'ERC6909Claims' overrides 'ERC6909::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateTick' in 'ERC6909Claims' overrides 'ERC6909::updateTick' which is not marked 'virtual' |
| warning | W200 | function 'updateTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'ERC6909Claims' overrides 'ERC6909::tickSpacingToMaxLiquidityPerTick' which is not marked 'virtual' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkPoolInitialized' in 'ERC6909Claims' overrides 'ERC6909::checkPoolInitialized' which is not marked 'virtual' |
| warning | W200 | function 'checkPoolInitialized' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clearTick' in 'ERC6909Claims' overrides 'ERC6909::clearTick' which is not marked 'virtual' |
| warning | W200 | function 'clearTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'crossTick' in 'ERC6909Claims' overrides 'ERC6909::crossTick' which is not marked 'virtual' |
| warning | W200 | function 'crossTick' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC6909Claims' overrides 'ERC6909::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'ERC6909Claims' overrides 'ERC6909::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ERC6909Claims' overrides 'ERC6909::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ERC6909Claims' overrides 'ERC6909::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'ERC6909Claims' overrides 'ERC6909::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'ERC6909Claims' overrides 'ERC6909::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'ERC6909Claims' overrides 'ERC6909::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function '_computeSlot' in 'ERC6909Claims' overrides 'ERC6909::_computeSlot' which is not marked 'virtual' |
| warning | W200 | function '_computeSlot' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDelta' in 'ERC6909Claims' overrides 'ERC6909::getDelta' which is not marked 'virtual' |
| warning | W200 | function 'getDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyDelta' in 'ERC6909Claims' overrides 'ERC6909::applyDelta' which is not marked 'virtual' |
| warning | W200 | function 'applyDelta' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'ERC6909Claims' overrides 'ERC6909::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'ERC6909Claims' overrides 'ERC6909::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'ERC6909Claims' overrides 'ERC6909::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'ERC6909Claims' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides 'ERC6909Claims::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides 'ERC6909Claims::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides 'ERC6909Claims::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides 'ERC6909Claims::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides 'ERC6909Claims::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides 'ERC6909Claims::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides 'ERC6909Claims::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'Extsload' overrides 'ERC6909Claims::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'Extsload' overrides 'ERC6909Claims::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'Extsload' overrides 'ERC6909Claims::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'Extsload' overrides 'ERC6909Claims::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'Extsload' overrides 'ERC6909Claims::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'Extsload' overrides 'ERC6909Claims::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'Extsload' overrides 'ERC6909Claims::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Extsload' overrides 'ERC6909Claims::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Extsload' overrides 'ERC6909Claims::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Extsload' overrides 'ERC6909Claims::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Extsload' overrides 'ERC6909Claims::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Extsload' overrides 'ERC6909Claims::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Extsload' overrides 'ERC6909Claims::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'Extsload' overrides 'ERC6909Claims::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'Extsload' overrides 'ERC6909Claims::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'Extsload' overrides 'ERC6909Claims::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'Extsload' overrides 'ERC6909Claims::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'Extsload' overrides 'ERC6909Claims::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isDynamicFee' in 'Extsload' overrides 'ERC6909Claims::isDynamicFee' which is not marked 'virtual' |
| warning | W200 | function 'isDynamicFee' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValid' in 'Extsload' overrides 'ERC6909Claims::isValid' which is not marked 'virtual' |
| warning | W200 | function 'isValid' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validate' in 'Extsload' overrides 'ERC6909Claims::validate' which is not marked 'virtual' |
| warning | W200 | function 'validate' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInitialLPFee' in 'Extsload' overrides 'ERC6909Claims::getInitialLPFee' which is not marked 'virtual' |
| warning | W200 | function 'getInitialLPFee' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isOverride' in 'Extsload' overrides 'ERC6909Claims::isOverride' which is not marked 'virtual' |
| warning | W200 | function 'isOverride' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlag' in 'Extsload' overrides 'ERC6909Claims::removeOverrideFlag' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlag' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'Extsload' overrides 'ERC6909Claims::removeOverrideFlagAndValidate' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'Extsload' overrides 'ERC6909Claims::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'Extsload' overrides 'ERC6909Claims::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'Extsload' overrides 'ERC6909Claims::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHookPermissions' in 'Extsload' overrides 'ERC6909Claims::validateHookPermissions' which is not marked 'virtual' |
| warning | W200 | function 'validateHookPermissions' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidHookAddress' in 'Extsload' overrides 'ERC6909Claims::isValidHookAddress' which is not marked 'virtual' |
| warning | W200 | function 'isValidHookAddress' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHook' in 'Extsload' overrides 'ERC6909Claims::callHook' which is not marked 'virtual' |
| warning | W200 | function 'callHook' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHookWithReturnDelta' in 'Extsload' overrides 'ERC6909Claims::callHookWithReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'callHookWithReturnDelta' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'noSelfCall' in 'Extsload' overrides 'ERC6909Claims::noSelfCall' which is not marked 'virtual' |
| warning | W200 | function 'noSelfCall' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeInitialize' in 'Extsload' overrides 'ERC6909Claims::beforeInitialize' which is not marked 'virtual' |
| warning | W200 | function 'beforeInitialize' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterInitialize' in 'Extsload' overrides 'ERC6909Claims::afterInitialize' which is not marked 'virtual' |
| warning | W200 | function 'afterInitialize' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeModifyLiquidity' in 'Extsload' overrides 'ERC6909Claims::beforeModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'beforeModifyLiquidity' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterModifyLiquidity' in 'Extsload' overrides 'ERC6909Claims::afterModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'afterModifyLiquidity' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeSwap' in 'Extsload' overrides 'ERC6909Claims::beforeSwap' which is not marked 'virtual' |
| warning | W200 | function 'beforeSwap' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterSwap' in 'Extsload' overrides 'ERC6909Claims::afterSwap' which is not marked 'virtual' |
| warning | W200 | function 'afterSwap' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeDonate' in 'Extsload' overrides 'ERC6909Claims::beforeDonate' which is not marked 'virtual' |
| warning | W200 | function 'beforeDonate' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterDonate' in 'Extsload' overrides 'ERC6909Claims::afterDonate' which is not marked 'virtual' |
| warning | W200 | function 'afterDonate' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasPermission' in 'Extsload' overrides 'ERC6909Claims::hasPermission' which is not marked 'virtual' |
| warning | W200 | function 'hasPermission' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'Extsload' overrides 'ERC6909Claims::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'Extsload' overrides 'ERC6909Claims::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'compress' in 'Extsload' overrides 'ERC6909Claims::compress' which is not marked 'virtual' |
| warning | W200 | function 'compress' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'position' in 'Extsload' overrides 'ERC6909Claims::position' which is not marked 'virtual' |
| warning | W200 | function 'position' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'flipTick' in 'Extsload' overrides 'ERC6909Claims::flipTick' which is not marked 'virtual' |
| warning | W200 | function 'flipTick' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'Extsload' overrides 'ERC6909Claims::nextInitializedTickWithinOneWord' which is not marked 'virtual' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Extsload' overrides 'ERC6909Claims::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'Extsload' overrides 'ERC6909Claims::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'Extsload' overrides 'ERC6909Claims::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'Extsload' overrides 'ERC6909Claims::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'Extsload' overrides 'ERC6909Claims::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'Extsload' overrides 'ERC6909Claims::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'divRoundingUp' in 'Extsload' overrides 'ERC6909Claims::divRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'divRoundingUp' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'simpleMulDiv' in 'Extsload' overrides 'ERC6909Claims::simpleMulDiv' which is not marked 'virtual' |
| warning | W200 | function 'simpleMulDiv' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'Extsload' overrides 'ERC6909Claims::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'Extsload' overrides 'ERC6909Claims::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'Extsload' overrides 'ERC6909Claims::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'Extsload' overrides 'ERC6909Claims::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'Extsload' overrides 'ERC6909Claims::getNextSqrtPriceFromAmount0RoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'Extsload' overrides 'ERC6909Claims::getNextSqrtPriceFromAmount1RoundingDown' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'Extsload' overrides 'ERC6909Claims::getNextSqrtPriceFromInput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'Extsload' overrides 'ERC6909Claims::getNextSqrtPriceFromOutput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'Extsload' overrides 'ERC6909Claims::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'absDiff' in 'Extsload' overrides 'ERC6909Claims::absDiff' which is not marked 'virtual' |
| warning | W200 | function 'absDiff' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'Extsload' overrides 'ERC6909Claims::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'Extsload' overrides 'ERC6909Claims::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'Extsload' overrides 'ERC6909Claims::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceTarget' in 'Extsload' overrides 'ERC6909Claims::getSqrtPriceTarget' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceTarget' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'computeSwapStep' in 'Extsload' overrides 'ERC6909Claims::computeSwapStep' which is not marked 'virtual' |
| warning | W200 | function 'computeSwapStep' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrtPriceX96' in 'Extsload' overrides 'ERC6909Claims::sqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'sqrtPriceX96' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tick' in 'Extsload' overrides 'ERC6909Claims::tick' which is not marked 'virtual' |
| warning | W200 | function 'tick' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'protocolFee' in 'Extsload' overrides 'ERC6909Claims::protocolFee' which is not marked 'virtual' |
| warning | W200 | function 'protocolFee' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lpFee' in 'Extsload' overrides 'ERC6909Claims::lpFee' which is not marked 'virtual' |
| warning | W200 | function 'lpFee' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSqrtPriceX96' in 'Extsload' overrides 'ERC6909Claims::setSqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'setSqrtPriceX96' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setTick' in 'Extsload' overrides 'ERC6909Claims::setTick' which is not marked 'virtual' |
| warning | W200 | function 'setTick' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'Extsload' overrides 'ERC6909Claims::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLpFee' in 'Extsload' overrides 'ERC6909Claims::setLpFee' which is not marked 'virtual' |
| warning | W200 | function 'setLpFee' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getZeroForOneFee' in 'Extsload' overrides 'ERC6909Claims::getZeroForOneFee' which is not marked 'virtual' |
| warning | W200 | function 'getZeroForOneFee' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getOneForZeroFee' in 'Extsload' overrides 'ERC6909Claims::getOneForZeroFee' which is not marked 'virtual' |
| warning | W200 | function 'getOneForZeroFee' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidProtocolFee' in 'Extsload' overrides 'ERC6909Claims::isValidProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'isValidProtocolFee' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateSwapFee' in 'Extsload' overrides 'ERC6909Claims::calculateSwapFee' which is not marked 'virtual' |
| warning | W200 | function 'calculateSwapFee' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkTicks' in 'Extsload' overrides 'ERC6909Claims::checkTicks' which is not marked 'virtual' |
| warning | W200 | function 'checkTicks' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'Extsload' overrides 'ERC6909Claims::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'Extsload' overrides 'ERC6909Claims::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLPFee' in 'Extsload' overrides 'ERC6909Claims::setLPFee' which is not marked 'virtual' |
| warning | W200 | function 'setLPFee' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modifyLiquidity' in 'Extsload' overrides 'ERC6909Claims::modifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'modifyLiquidity' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'swap' in 'Extsload' overrides 'ERC6909Claims::swap' which is not marked 'virtual' |
| warning | W200 | function 'swap' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'donate' in 'Extsload' overrides 'ERC6909Claims::donate' which is not marked 'virtual' |
| warning | W200 | function 'donate' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'Extsload' overrides 'ERC6909Claims::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateTick' in 'Extsload' overrides 'ERC6909Claims::updateTick' which is not marked 'virtual' |
| warning | W200 | function 'updateTick' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'Extsload' overrides 'ERC6909Claims::tickSpacingToMaxLiquidityPerTick' which is not marked 'virtual' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkPoolInitialized' in 'Extsload' overrides 'ERC6909Claims::checkPoolInitialized' which is not marked 'virtual' |
| warning | W200 | function 'checkPoolInitialized' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clearTick' in 'Extsload' overrides 'ERC6909Claims::clearTick' which is not marked 'virtual' |
| warning | W200 | function 'clearTick' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'crossTick' in 'Extsload' overrides 'ERC6909Claims::crossTick' which is not marked 'virtual' |
| warning | W200 | function 'crossTick' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'Extsload' overrides 'ERC6909Claims::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'Extsload' overrides 'ERC6909Claims::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'Extsload' overrides 'ERC6909Claims::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'Extsload' overrides 'ERC6909Claims::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'Extsload' overrides 'ERC6909Claims::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'Extsload' overrides 'ERC6909Claims::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'Extsload' overrides 'ERC6909Claims::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function '_computeSlot' in 'Extsload' overrides 'ERC6909Claims::_computeSlot' which is not marked 'virtual' |
| warning | W200 | function '_computeSlot' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDelta' in 'Extsload' overrides 'ERC6909Claims::getDelta' which is not marked 'virtual' |
| warning | W200 | function 'getDelta' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyDelta' in 'Extsload' overrides 'ERC6909Claims::applyDelta' which is not marked 'virtual' |
| warning | W200 | function 'applyDelta' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'Extsload' overrides 'ERC6909Claims::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'Extsload' overrides 'ERC6909Claims::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'Extsload' overrides 'ERC6909Claims::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'Extsload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides 'Extsload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides 'Extsload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides 'Extsload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides 'Extsload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides 'Extsload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides 'Extsload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides 'Extsload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'Exttload' overrides 'Extsload::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'Exttload' overrides 'Extsload::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'Exttload' overrides 'Extsload::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'Exttload' overrides 'Extsload::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'Exttload' overrides 'Extsload::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'Exttload' overrides 'Extsload::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'Exttload' overrides 'Extsload::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Exttload' overrides 'Extsload::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Exttload' overrides 'Extsload::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Exttload' overrides 'Extsload::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Exttload' overrides 'Extsload::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Exttload' overrides 'Extsload::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Exttload' overrides 'Extsload::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'Exttload' overrides 'Extsload::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'Exttload' overrides 'Extsload::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'Exttload' overrides 'Extsload::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'Exttload' overrides 'Extsload::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'Exttload' overrides 'Extsload::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isDynamicFee' in 'Exttload' overrides 'Extsload::isDynamicFee' which is not marked 'virtual' |
| warning | W200 | function 'isDynamicFee' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValid' in 'Exttload' overrides 'Extsload::isValid' which is not marked 'virtual' |
| warning | W200 | function 'isValid' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validate' in 'Exttload' overrides 'Extsload::validate' which is not marked 'virtual' |
| warning | W200 | function 'validate' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInitialLPFee' in 'Exttload' overrides 'Extsload::getInitialLPFee' which is not marked 'virtual' |
| warning | W200 | function 'getInitialLPFee' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isOverride' in 'Exttload' overrides 'Extsload::isOverride' which is not marked 'virtual' |
| warning | W200 | function 'isOverride' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlag' in 'Exttload' overrides 'Extsload::removeOverrideFlag' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlag' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'Exttload' overrides 'Extsload::removeOverrideFlagAndValidate' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'Exttload' overrides 'Extsload::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'Exttload' overrides 'Extsload::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'Exttload' overrides 'Extsload::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHookPermissions' in 'Exttload' overrides 'Extsload::validateHookPermissions' which is not marked 'virtual' |
| warning | W200 | function 'validateHookPermissions' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidHookAddress' in 'Exttload' overrides 'Extsload::isValidHookAddress' which is not marked 'virtual' |
| warning | W200 | function 'isValidHookAddress' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHook' in 'Exttload' overrides 'Extsload::callHook' which is not marked 'virtual' |
| warning | W200 | function 'callHook' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHookWithReturnDelta' in 'Exttload' overrides 'Extsload::callHookWithReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'callHookWithReturnDelta' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'noSelfCall' in 'Exttload' overrides 'Extsload::noSelfCall' which is not marked 'virtual' |
| warning | W200 | function 'noSelfCall' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeInitialize' in 'Exttload' overrides 'Extsload::beforeInitialize' which is not marked 'virtual' |
| warning | W200 | function 'beforeInitialize' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterInitialize' in 'Exttload' overrides 'Extsload::afterInitialize' which is not marked 'virtual' |
| warning | W200 | function 'afterInitialize' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeModifyLiquidity' in 'Exttload' overrides 'Extsload::beforeModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'beforeModifyLiquidity' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterModifyLiquidity' in 'Exttload' overrides 'Extsload::afterModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'afterModifyLiquidity' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeSwap' in 'Exttload' overrides 'Extsload::beforeSwap' which is not marked 'virtual' |
| warning | W200 | function 'beforeSwap' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterSwap' in 'Exttload' overrides 'Extsload::afterSwap' which is not marked 'virtual' |
| warning | W200 | function 'afterSwap' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeDonate' in 'Exttload' overrides 'Extsload::beforeDonate' which is not marked 'virtual' |
| warning | W200 | function 'beforeDonate' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterDonate' in 'Exttload' overrides 'Extsload::afterDonate' which is not marked 'virtual' |
| warning | W200 | function 'afterDonate' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasPermission' in 'Exttload' overrides 'Extsload::hasPermission' which is not marked 'virtual' |
| warning | W200 | function 'hasPermission' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'Exttload' overrides 'Extsload::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'Exttload' overrides 'Extsload::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'compress' in 'Exttload' overrides 'Extsload::compress' which is not marked 'virtual' |
| warning | W200 | function 'compress' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'position' in 'Exttload' overrides 'Extsload::position' which is not marked 'virtual' |
| warning | W200 | function 'position' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'flipTick' in 'Exttload' overrides 'Extsload::flipTick' which is not marked 'virtual' |
| warning | W200 | function 'flipTick' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'Exttload' overrides 'Extsload::nextInitializedTickWithinOneWord' which is not marked 'virtual' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Exttload' overrides 'Extsload::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'Exttload' overrides 'Extsload::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'Exttload' overrides 'Extsload::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'Exttload' overrides 'Extsload::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'Exttload' overrides 'Extsload::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'Exttload' overrides 'Extsload::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'divRoundingUp' in 'Exttload' overrides 'Extsload::divRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'divRoundingUp' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'simpleMulDiv' in 'Exttload' overrides 'Extsload::simpleMulDiv' which is not marked 'virtual' |
| warning | W200 | function 'simpleMulDiv' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'Exttload' overrides 'Extsload::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'Exttload' overrides 'Extsload::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'Exttload' overrides 'Extsload::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'Exttload' overrides 'Extsload::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'Exttload' overrides 'Extsload::getNextSqrtPriceFromAmount0RoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'Exttload' overrides 'Extsload::getNextSqrtPriceFromAmount1RoundingDown' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'Exttload' overrides 'Extsload::getNextSqrtPriceFromInput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'Exttload' overrides 'Extsload::getNextSqrtPriceFromOutput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'Exttload' overrides 'Extsload::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'absDiff' in 'Exttload' overrides 'Extsload::absDiff' which is not marked 'virtual' |
| warning | W200 | function 'absDiff' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'Exttload' overrides 'Extsload::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'Exttload' overrides 'Extsload::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'Exttload' overrides 'Extsload::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceTarget' in 'Exttload' overrides 'Extsload::getSqrtPriceTarget' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceTarget' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'computeSwapStep' in 'Exttload' overrides 'Extsload::computeSwapStep' which is not marked 'virtual' |
| warning | W200 | function 'computeSwapStep' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrtPriceX96' in 'Exttload' overrides 'Extsload::sqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'sqrtPriceX96' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tick' in 'Exttload' overrides 'Extsload::tick' which is not marked 'virtual' |
| warning | W200 | function 'tick' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'protocolFee' in 'Exttload' overrides 'Extsload::protocolFee' which is not marked 'virtual' |
| warning | W200 | function 'protocolFee' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lpFee' in 'Exttload' overrides 'Extsload::lpFee' which is not marked 'virtual' |
| warning | W200 | function 'lpFee' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSqrtPriceX96' in 'Exttload' overrides 'Extsload::setSqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'setSqrtPriceX96' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setTick' in 'Exttload' overrides 'Extsload::setTick' which is not marked 'virtual' |
| warning | W200 | function 'setTick' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'Exttload' overrides 'Extsload::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLpFee' in 'Exttload' overrides 'Extsload::setLpFee' which is not marked 'virtual' |
| warning | W200 | function 'setLpFee' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getZeroForOneFee' in 'Exttload' overrides 'Extsload::getZeroForOneFee' which is not marked 'virtual' |
| warning | W200 | function 'getZeroForOneFee' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getOneForZeroFee' in 'Exttload' overrides 'Extsload::getOneForZeroFee' which is not marked 'virtual' |
| warning | W200 | function 'getOneForZeroFee' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidProtocolFee' in 'Exttload' overrides 'Extsload::isValidProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'isValidProtocolFee' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateSwapFee' in 'Exttload' overrides 'Extsload::calculateSwapFee' which is not marked 'virtual' |
| warning | W200 | function 'calculateSwapFee' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkTicks' in 'Exttload' overrides 'Extsload::checkTicks' which is not marked 'virtual' |
| warning | W200 | function 'checkTicks' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'Exttload' overrides 'Extsload::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'Exttload' overrides 'Extsload::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLPFee' in 'Exttload' overrides 'Extsload::setLPFee' which is not marked 'virtual' |
| warning | W200 | function 'setLPFee' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modifyLiquidity' in 'Exttload' overrides 'Extsload::modifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'modifyLiquidity' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'swap' in 'Exttload' overrides 'Extsload::swap' which is not marked 'virtual' |
| warning | W200 | function 'swap' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'donate' in 'Exttload' overrides 'Extsload::donate' which is not marked 'virtual' |
| warning | W200 | function 'donate' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'Exttload' overrides 'Extsload::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateTick' in 'Exttload' overrides 'Extsload::updateTick' which is not marked 'virtual' |
| warning | W200 | function 'updateTick' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'Exttload' overrides 'Extsload::tickSpacingToMaxLiquidityPerTick' which is not marked 'virtual' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkPoolInitialized' in 'Exttload' overrides 'Extsload::checkPoolInitialized' which is not marked 'virtual' |
| warning | W200 | function 'checkPoolInitialized' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clearTick' in 'Exttload' overrides 'Extsload::clearTick' which is not marked 'virtual' |
| warning | W200 | function 'clearTick' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'crossTick' in 'Exttload' overrides 'Extsload::crossTick' which is not marked 'virtual' |
| warning | W200 | function 'crossTick' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'Exttload' overrides 'Extsload::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'Exttload' overrides 'Extsload::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'Exttload' overrides 'Extsload::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'Exttload' overrides 'Extsload::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'Exttload' overrides 'Extsload::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'Exttload' overrides 'Extsload::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'Exttload' overrides 'Extsload::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function '_computeSlot' in 'Exttload' overrides 'Extsload::_computeSlot' which is not marked 'virtual' |
| warning | W200 | function '_computeSlot' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDelta' in 'Exttload' overrides 'Extsload::getDelta' which is not marked 'virtual' |
| warning | W200 | function 'getDelta' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyDelta' in 'Exttload' overrides 'Extsload::applyDelta' which is not marked 'virtual' |
| warning | W200 | function 'applyDelta' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'Exttload' overrides 'Extsload::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'Exttload' overrides 'Extsload::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'Exttload' overrides 'Extsload::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'Exttload' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides 'Exttload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides 'Exttload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides 'Exttload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides 'Exttload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides 'Exttload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides 'Exttload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides 'Exttload::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'PoolManager' overrides 'Exttload::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'PoolManager' overrides 'Exttload::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'PoolManager' overrides 'Exttload::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'PoolManager' overrides 'Exttload::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'PoolManager' overrides 'Exttload::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'PoolManager' overrides 'Exttload::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'PoolManager' overrides 'Exttload::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'PoolManager' overrides 'Exttload::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'PoolManager' overrides 'Exttload::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'PoolManager' overrides 'Exttload::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'PoolManager' overrides 'Exttload::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'PoolManager' overrides 'Exttload::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'PoolManager' overrides 'Exttload::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'PoolManager' overrides 'Exttload::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'PoolManager' overrides 'Exttload::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'PoolManager' overrides 'Exttload::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'PoolManager' overrides 'Exttload::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'PoolManager' overrides 'Exttload::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isDynamicFee' in 'PoolManager' overrides 'Exttload::isDynamicFee' which is not marked 'virtual' |
| warning | W200 | function 'isDynamicFee' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValid' in 'PoolManager' overrides 'Exttload::isValid' which is not marked 'virtual' |
| warning | W200 | function 'isValid' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validate' in 'PoolManager' overrides 'Exttload::validate' which is not marked 'virtual' |
| warning | W200 | function 'validate' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getInitialLPFee' in 'PoolManager' overrides 'Exttload::getInitialLPFee' which is not marked 'virtual' |
| warning | W200 | function 'getInitialLPFee' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isOverride' in 'PoolManager' overrides 'Exttload::isOverride' which is not marked 'virtual' |
| warning | W200 | function 'isOverride' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlag' in 'PoolManager' overrides 'Exttload::removeOverrideFlag' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlag' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'PoolManager' overrides 'Exttload::removeOverrideFlagAndValidate' which is not marked 'virtual' |
| warning | W200 | function 'removeOverrideFlagAndValidate' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'PoolManager' overrides 'Exttload::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'PoolManager' overrides 'Exttload::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'PoolManager' overrides 'Exttload::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateHookPermissions' in 'PoolManager' overrides 'Exttload::validateHookPermissions' which is not marked 'virtual' |
| warning | W200 | function 'validateHookPermissions' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidHookAddress' in 'PoolManager' overrides 'Exttload::isValidHookAddress' which is not marked 'virtual' |
| warning | W200 | function 'isValidHookAddress' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHook' in 'PoolManager' overrides 'Exttload::callHook' which is not marked 'virtual' |
| warning | W200 | function 'callHook' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'callHookWithReturnDelta' in 'PoolManager' overrides 'Exttload::callHookWithReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'callHookWithReturnDelta' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'noSelfCall' in 'PoolManager' overrides 'Exttload::noSelfCall' which is not marked 'virtual' |
| warning | W200 | function 'noSelfCall' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeInitialize' in 'PoolManager' overrides 'Exttload::beforeInitialize' which is not marked 'virtual' |
| warning | W200 | function 'beforeInitialize' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterInitialize' in 'PoolManager' overrides 'Exttload::afterInitialize' which is not marked 'virtual' |
| warning | W200 | function 'afterInitialize' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeModifyLiquidity' in 'PoolManager' overrides 'Exttload::beforeModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'beforeModifyLiquidity' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterModifyLiquidity' in 'PoolManager' overrides 'Exttload::afterModifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'afterModifyLiquidity' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeSwap' in 'PoolManager' overrides 'Exttload::beforeSwap' which is not marked 'virtual' |
| warning | W200 | function 'beforeSwap' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterSwap' in 'PoolManager' overrides 'Exttload::afterSwap' which is not marked 'virtual' |
| warning | W200 | function 'afterSwap' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'beforeDonate' in 'PoolManager' overrides 'Exttload::beforeDonate' which is not marked 'virtual' |
| warning | W200 | function 'beforeDonate' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'afterDonate' in 'PoolManager' overrides 'Exttload::afterDonate' which is not marked 'virtual' |
| warning | W200 | function 'afterDonate' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasPermission' in 'PoolManager' overrides 'Exttload::hasPermission' which is not marked 'virtual' |
| warning | W200 | function 'hasPermission' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'PoolManager' overrides 'Exttload::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'PoolManager' overrides 'Exttload::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'compress' in 'PoolManager' overrides 'Exttload::compress' which is not marked 'virtual' |
| warning | W200 | function 'compress' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'position' in 'PoolManager' overrides 'Exttload::position' which is not marked 'virtual' |
| warning | W200 | function 'position' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'flipTick' in 'PoolManager' overrides 'Exttload::flipTick' which is not marked 'virtual' |
| warning | W200 | function 'flipTick' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'PoolManager' overrides 'Exttload::nextInitializedTickWithinOneWord' which is not marked 'virtual' |
| warning | W200 | function 'nextInitializedTickWithinOneWord' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'PoolManager' overrides 'Exttload::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'PoolManager' overrides 'Exttload::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'PoolManager' overrides 'Exttload::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'PoolManager' overrides 'Exttload::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'PoolManager' overrides 'Exttload::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'PoolManager' overrides 'Exttload::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'divRoundingUp' in 'PoolManager' overrides 'Exttload::divRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'divRoundingUp' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'simpleMulDiv' in 'PoolManager' overrides 'Exttload::simpleMulDiv' which is not marked 'virtual' |
| warning | W200 | function 'simpleMulDiv' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'PoolManager' overrides 'Exttload::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'PoolManager' overrides 'Exttload::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'PoolManager' overrides 'Exttload::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'PoolManager' overrides 'Exttload::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'PoolManager' overrides 'Exttload::getNextSqrtPriceFromAmount0RoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount0RoundingUp' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'PoolManager' overrides 'Exttload::getNextSqrtPriceFromAmount1RoundingDown' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromAmount1RoundingDown' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'PoolManager' overrides 'Exttload::getNextSqrtPriceFromInput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromInput' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'PoolManager' overrides 'Exttload::getNextSqrtPriceFromOutput' which is not marked 'virtual' |
| warning | W200 | function 'getNextSqrtPriceFromOutput' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'PoolManager' overrides 'Exttload::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'absDiff' in 'PoolManager' overrides 'Exttload::absDiff' which is not marked 'virtual' |
| warning | W200 | function 'absDiff' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'PoolManager' overrides 'Exttload::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount0Delta' in 'PoolManager' overrides 'Exttload::getAmount0Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount0Delta' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getAmount1Delta' in 'PoolManager' overrides 'Exttload::getAmount1Delta' which is not marked 'virtual' |
| warning | W200 | function 'getAmount1Delta' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceTarget' in 'PoolManager' overrides 'Exttload::getSqrtPriceTarget' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceTarget' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'computeSwapStep' in 'PoolManager' overrides 'Exttload::computeSwapStep' which is not marked 'virtual' |
| warning | W200 | function 'computeSwapStep' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrtPriceX96' in 'PoolManager' overrides 'Exttload::sqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'sqrtPriceX96' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tick' in 'PoolManager' overrides 'Exttload::tick' which is not marked 'virtual' |
| warning | W200 | function 'tick' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'protocolFee' in 'PoolManager' overrides 'Exttload::protocolFee' which is not marked 'virtual' |
| warning | W200 | function 'protocolFee' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lpFee' in 'PoolManager' overrides 'Exttload::lpFee' which is not marked 'virtual' |
| warning | W200 | function 'lpFee' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSqrtPriceX96' in 'PoolManager' overrides 'Exttload::setSqrtPriceX96' which is not marked 'virtual' |
| warning | W200 | function 'setSqrtPriceX96' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setTick' in 'PoolManager' overrides 'Exttload::setTick' which is not marked 'virtual' |
| warning | W200 | function 'setTick' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'PoolManager' overrides 'Exttload::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLpFee' in 'PoolManager' overrides 'Exttload::setLpFee' which is not marked 'virtual' |
| warning | W200 | function 'setLpFee' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getZeroForOneFee' in 'PoolManager' overrides 'Exttload::getZeroForOneFee' which is not marked 'virtual' |
| warning | W200 | function 'getZeroForOneFee' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getOneForZeroFee' in 'PoolManager' overrides 'Exttload::getOneForZeroFee' which is not marked 'virtual' |
| warning | W200 | function 'getOneForZeroFee' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidProtocolFee' in 'PoolManager' overrides 'Exttload::isValidProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'isValidProtocolFee' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateSwapFee' in 'PoolManager' overrides 'Exttload::calculateSwapFee' which is not marked 'virtual' |
| warning | W200 | function 'calculateSwapFee' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkTicks' in 'PoolManager' overrides 'Exttload::checkTicks' which is not marked 'virtual' |
| warning | W200 | function 'checkTicks' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'PoolManager' overrides 'Exttload::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setProtocolFee' in 'PoolManager' overrides 'Exttload::setProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'setProtocolFee' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setLPFee' in 'PoolManager' overrides 'Exttload::setLPFee' which is not marked 'virtual' |
| warning | W200 | function 'setLPFee' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modifyLiquidity' in 'PoolManager' overrides 'Exttload::modifyLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'modifyLiquidity' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'swap' in 'PoolManager' overrides 'Exttload::swap' which is not marked 'virtual' |
| warning | W200 | function 'swap' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'donate' in 'PoolManager' overrides 'Exttload::donate' which is not marked 'virtual' |
| warning | W200 | function 'donate' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'PoolManager' overrides 'Exttload::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'updateTick' in 'PoolManager' overrides 'Exttload::updateTick' which is not marked 'virtual' |
| warning | W200 | function 'updateTick' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'PoolManager' overrides 'Exttload::tickSpacingToMaxLiquidityPerTick' which is not marked 'virtual' |
| warning | W200 | function 'tickSpacingToMaxLiquidityPerTick' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkPoolInitialized' in 'PoolManager' overrides 'Exttload::checkPoolInitialized' which is not marked 'virtual' |
| warning | W200 | function 'checkPoolInitialized' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'clearTick' in 'PoolManager' overrides 'Exttload::clearTick' which is not marked 'virtual' |
| warning | W200 | function 'clearTick' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'crossTick' in 'PoolManager' overrides 'Exttload::crossTick' which is not marked 'virtual' |
| warning | W200 | function 'crossTick' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'PoolManager' overrides 'Exttload::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'PoolManager' overrides 'Exttload::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'PoolManager' overrides 'Exttload::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'PoolManager' overrides 'Exttload::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'PoolManager' overrides 'Exttload::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'PoolManager' overrides 'Exttload::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'PoolManager' overrides 'Exttload::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function '_computeSlot' in 'PoolManager' overrides 'Exttload::_computeSlot' which is not marked 'virtual' |
| warning | W200 | function '_computeSlot' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getDelta' in 'PoolManager' overrides 'Exttload::getDelta' which is not marked 'virtual' |
| warning | W200 | function 'getDelta' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'applyDelta' in 'PoolManager' overrides 'Exttload::applyDelta' which is not marked 'virtual' |
| warning | W200 | function 'applyDelta' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'PoolManager' overrides 'Exttload::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'PoolManager' overrides 'Exttload::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'PoolManager' overrides 'Exttload::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'PoolManager' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'PoolManager' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@uniswap/v4-core/src/PoolManager.sol`