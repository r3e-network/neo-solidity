# V4Router (Uniswap V4 Periphery)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@uniswap/v4-periphery/src/V4Router.sol`
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
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ImmutableState' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DONATE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PORTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLEAR_OR_TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWEEP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNWRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BPS_DENOMINATOR' detected while merging libraries |
| warning | VALIDATION_WARNING | abstract contract 'SafeCallback' has 1 unimplemented function(s): [_unlockCallback] |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'SafeCallback' overrides 'ImmutableState::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'SafeCallback' overrides 'ImmutableState::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'SafeCallback' overrides 'ImmutableState::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'SafeCallback' overrides 'ImmutableState::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'SafeCallback' overrides 'ImmutableState::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'SafeCallback' overrides 'ImmutableState::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'SafeCallback' overrides 'ImmutableState::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'SafeCallback' overrides 'ImmutableState::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'SafeCallback' overrides 'ImmutableState::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'SafeCallback' overrides 'ImmutableState::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'SafeCallback' overrides 'ImmutableState::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'SafeCallback' overrides 'ImmutableState::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'SafeCallback' overrides 'ImmutableState::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'SafeCallback' overrides 'ImmutableState::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'SafeCallback' overrides 'ImmutableState::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'SafeCallback' overrides 'ImmutableState::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'SafeCallback' overrides 'ImmutableState::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'SafeCallback' overrides 'ImmutableState::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides 'ImmutableState::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'SafeCallback' overrides 'ImmutableState::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'SafeCallback' overrides 'ImmutableState::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'SafeCallback' overrides 'ImmutableState::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'SafeCallback' overrides 'ImmutableState::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'SafeCallback' overrides 'ImmutableState::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'SafeCallback' overrides 'ImmutableState::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'SafeCallback' overrides 'ImmutableState::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'SafeCallback' overrides 'ImmutableState::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'SafeCallback' overrides 'ImmutableState::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'SafeCallback' overrides 'ImmutableState::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'SafeCallback' overrides 'ImmutableState::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'SafeCallback' overrides 'ImmutableState::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'SafeCallback' overrides 'ImmutableState::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'SafeCallback' overrides 'ImmutableState::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'SafeCallback' overrides 'ImmutableState::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'SafeCallback' overrides 'ImmutableState::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'SafeCallback' overrides 'ImmutableState::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'SafeCallback' overrides 'ImmutableState::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'SafeCallback' overrides 'ImmutableState::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'SafeCallback' overrides 'ImmutableState::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'SafeCallback' overrides 'ImmutableState::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'SafeCallback' overrides 'ImmutableState::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePortion' in 'SafeCallback' overrides 'ImmutableState::calculatePortion' which is not marked 'virtual' |
| warning | W200 | function 'calculatePortion' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'SafeCallback' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DONATE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PORTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLEAR_OR_TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWEEP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNWRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BPS_DENOMINATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DONATE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PORTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLEAR_OR_TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWEEP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNWRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BPS_DENOMINATOR' detected while merging libraries |
| warning | VALIDATION_WARNING | abstract contract 'BaseActionsRouter' has 2 unimplemented function(s): [_handleAction, msgSender] |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'SafeCallback' overrides 'ImmutableState::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'SafeCallback' overrides 'ImmutableState::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'SafeCallback' overrides 'ImmutableState::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'SafeCallback' overrides 'ImmutableState::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'SafeCallback' overrides 'ImmutableState::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'SafeCallback' overrides 'ImmutableState::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'SafeCallback' overrides 'ImmutableState::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'SafeCallback' overrides 'ImmutableState::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'SafeCallback' overrides 'ImmutableState::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'SafeCallback' overrides 'ImmutableState::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'SafeCallback' overrides 'ImmutableState::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'SafeCallback' overrides 'ImmutableState::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'SafeCallback' overrides 'ImmutableState::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'SafeCallback' overrides 'ImmutableState::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'SafeCallback' overrides 'ImmutableState::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'SafeCallback' overrides 'ImmutableState::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'SafeCallback' overrides 'ImmutableState::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'SafeCallback' overrides 'ImmutableState::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides 'ImmutableState::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'SafeCallback' overrides 'ImmutableState::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'SafeCallback' overrides 'ImmutableState::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'SafeCallback' overrides 'ImmutableState::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'SafeCallback' overrides 'ImmutableState::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'SafeCallback' overrides 'ImmutableState::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'SafeCallback' overrides 'ImmutableState::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'SafeCallback' overrides 'ImmutableState::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'SafeCallback' overrides 'ImmutableState::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'SafeCallback' overrides 'ImmutableState::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'SafeCallback' overrides 'ImmutableState::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'SafeCallback' overrides 'ImmutableState::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'SafeCallback' overrides 'ImmutableState::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'SafeCallback' overrides 'ImmutableState::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'SafeCallback' overrides 'ImmutableState::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'SafeCallback' overrides 'ImmutableState::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'SafeCallback' overrides 'ImmutableState::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'SafeCallback' overrides 'ImmutableState::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'SafeCallback' overrides 'ImmutableState::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'SafeCallback' overrides 'ImmutableState::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'SafeCallback' overrides 'ImmutableState::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'SafeCallback' overrides 'ImmutableState::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'SafeCallback' overrides 'ImmutableState::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePortion' in 'SafeCallback' overrides 'ImmutableState::calculatePortion' which is not marked 'virtual' |
| warning | W200 | function 'calculatePortion' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'BaseActionsRouter' overrides 'SafeCallback::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'BaseActionsRouter' overrides 'SafeCallback::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'BaseActionsRouter' overrides 'SafeCallback::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'BaseActionsRouter' overrides 'SafeCallback::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'BaseActionsRouter' overrides 'SafeCallback::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'BaseActionsRouter' overrides 'SafeCallback::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'BaseActionsRouter' overrides 'SafeCallback::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'BaseActionsRouter' overrides 'SafeCallback::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'BaseActionsRouter' overrides 'SafeCallback::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'BaseActionsRouter' overrides 'SafeCallback::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'BaseActionsRouter' overrides 'SafeCallback::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'BaseActionsRouter' overrides 'SafeCallback::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'BaseActionsRouter' overrides 'SafeCallback::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'BaseActionsRouter' overrides 'SafeCallback::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'BaseActionsRouter' overrides 'SafeCallback::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'BaseActionsRouter' overrides 'SafeCallback::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'BaseActionsRouter' overrides 'SafeCallback::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'BaseActionsRouter' overrides 'SafeCallback::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'BaseActionsRouter' overrides 'SafeCallback::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'BaseActionsRouter' overrides 'SafeCallback::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'BaseActionsRouter' overrides 'SafeCallback::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'BaseActionsRouter' overrides 'SafeCallback::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'BaseActionsRouter' overrides 'SafeCallback::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'BaseActionsRouter' overrides 'SafeCallback::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'BaseActionsRouter' overrides 'SafeCallback::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'BaseActionsRouter' overrides 'SafeCallback::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'BaseActionsRouter' overrides 'SafeCallback::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'BaseActionsRouter' overrides 'SafeCallback::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'BaseActionsRouter' overrides 'SafeCallback::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'BaseActionsRouter' overrides 'SafeCallback::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'BaseActionsRouter' overrides 'SafeCallback::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'BaseActionsRouter' overrides 'SafeCallback::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'BaseActionsRouter' overrides 'SafeCallback::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'BaseActionsRouter' overrides 'SafeCallback::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'BaseActionsRouter' overrides 'SafeCallback::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'BaseActionsRouter' overrides 'SafeCallback::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'BaseActionsRouter' overrides 'SafeCallback::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'BaseActionsRouter' overrides 'SafeCallback::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'BaseActionsRouter' overrides 'SafeCallback::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'BaseActionsRouter' overrides 'SafeCallback::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'BaseActionsRouter' overrides 'SafeCallback::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'BaseActionsRouter' overrides 'SafeCallback::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePortion' in 'BaseActionsRouter' overrides 'SafeCallback::calculatePortion' which is not marked 'virtual' |
| warning | W200 | function 'calculatePortion' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'BaseActionsRouter' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DONATE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PORTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLEAR_OR_TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWEEP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNWRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BPS_DENOMINATOR' detected while merging libraries |
| warning | VALIDATION_WARNING | abstract contract 'DeltaResolver' has 1 unimplemented function(s): [_pay] |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'DeltaResolver' overrides 'ImmutableState::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'DeltaResolver' overrides 'ImmutableState::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'DeltaResolver' overrides 'ImmutableState::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'DeltaResolver' overrides 'ImmutableState::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'DeltaResolver' overrides 'ImmutableState::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'DeltaResolver' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'DeltaResolver' overrides 'ImmutableState::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'DeltaResolver' overrides 'ImmutableState::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'DeltaResolver' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'DeltaResolver' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'DeltaResolver' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'DeltaResolver' overrides 'ImmutableState::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'DeltaResolver' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'DeltaResolver' overrides 'ImmutableState::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'DeltaResolver' overrides 'ImmutableState::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'DeltaResolver' overrides 'ImmutableState::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'DeltaResolver' overrides 'ImmutableState::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'DeltaResolver' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'DeltaResolver' overrides 'ImmutableState::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'DeltaResolver' overrides 'ImmutableState::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'DeltaResolver' overrides 'ImmutableState::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'DeltaResolver' overrides 'ImmutableState::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'DeltaResolver' overrides 'ImmutableState::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'DeltaResolver' overrides 'ImmutableState::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'DeltaResolver' overrides 'ImmutableState::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'DeltaResolver' overrides 'ImmutableState::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'DeltaResolver' overrides 'ImmutableState::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'DeltaResolver' overrides 'ImmutableState::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'DeltaResolver' overrides 'ImmutableState::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'DeltaResolver' overrides 'ImmutableState::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'DeltaResolver' overrides 'ImmutableState::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'DeltaResolver' overrides 'ImmutableState::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'DeltaResolver' overrides 'ImmutableState::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'DeltaResolver' overrides 'ImmutableState::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'DeltaResolver' overrides 'ImmutableState::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'DeltaResolver' overrides 'ImmutableState::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'DeltaResolver' overrides 'ImmutableState::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'DeltaResolver' overrides 'ImmutableState::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'DeltaResolver' overrides 'ImmutableState::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'DeltaResolver' overrides 'ImmutableState::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'DeltaResolver' overrides 'ImmutableState::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'DeltaResolver' overrides 'ImmutableState::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'DeltaResolver' overrides 'ImmutableState::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'DeltaResolver' overrides 'ImmutableState::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'DeltaResolver' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'DeltaResolver' overrides 'ImmutableState::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'DeltaResolver' overrides 'ImmutableState::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'DeltaResolver' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'DeltaResolver' overrides 'ImmutableState::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'DeltaResolver' overrides 'ImmutableState::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'DeltaResolver' overrides 'ImmutableState::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'DeltaResolver' overrides 'ImmutableState::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'DeltaResolver' overrides 'ImmutableState::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'DeltaResolver' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'DeltaResolver' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'DeltaResolver' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'DeltaResolver' overrides 'ImmutableState::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'DeltaResolver' overrides 'ImmutableState::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'DeltaResolver' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePortion' in 'DeltaResolver' overrides 'ImmutableState::calculatePortion' which is not marked 'virtual' |
| warning | W200 | function 'calculatePortion' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'DeltaResolver' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DONATE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PORTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLEAR_OR_TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWEEP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNWRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BPS_DENOMINATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DONATE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PORTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLEAR_OR_TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWEEP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNWRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BPS_DENOMINATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DONATE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PORTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLEAR_OR_TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWEEP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNWRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BPS_DENOMINATOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DECREASE_LIQUIDITY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_POSITION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INCREASE_LIQUIDITY_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_POSITION_FROM_DELTAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_IN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT_SINGLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWAP_EXACT_OUT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DONATE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SETTLE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_ALL' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PORTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TAKE_PAIR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLOSE_CURRENCY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CLEAR_OR_TAKE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SWEEP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'WRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNWRAP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MINT_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BURN_6909' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BPS_DENOMINATOR' detected while merging libraries |
| warning | VALIDATION_WARNING | 'using X for *' is supported; all library functions are available but type-specific filtering is not enforced at compile time |
| warning | VALIDATION_WARNING | abstract contract 'V4Router' has 2 unimplemented function(s): [msgSender, _pay] |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'SafeCallback' overrides 'ImmutableState::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'SafeCallback' overrides 'ImmutableState::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'SafeCallback' overrides 'ImmutableState::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'SafeCallback' overrides 'ImmutableState::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'SafeCallback' overrides 'ImmutableState::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'SafeCallback' overrides 'ImmutableState::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'SafeCallback' overrides 'ImmutableState::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'SafeCallback' overrides 'ImmutableState::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'SafeCallback' overrides 'ImmutableState::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'SafeCallback' overrides 'ImmutableState::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'SafeCallback' overrides 'ImmutableState::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'SafeCallback' overrides 'ImmutableState::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'SafeCallback' overrides 'ImmutableState::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'SafeCallback' overrides 'ImmutableState::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'SafeCallback' overrides 'ImmutableState::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'SafeCallback' overrides 'ImmutableState::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'SafeCallback' overrides 'ImmutableState::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'SafeCallback' overrides 'ImmutableState::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides 'ImmutableState::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'SafeCallback' overrides 'ImmutableState::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'SafeCallback' overrides 'ImmutableState::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'SafeCallback' overrides 'ImmutableState::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'SafeCallback' overrides 'ImmutableState::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'SafeCallback' overrides 'ImmutableState::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'SafeCallback' overrides 'ImmutableState::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'SafeCallback' overrides 'ImmutableState::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'SafeCallback' overrides 'ImmutableState::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'SafeCallback' overrides 'ImmutableState::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'SafeCallback' overrides 'ImmutableState::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'SafeCallback' overrides 'ImmutableState::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'SafeCallback' overrides 'ImmutableState::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'SafeCallback' overrides 'ImmutableState::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'SafeCallback' overrides 'ImmutableState::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'SafeCallback' overrides 'ImmutableState::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'SafeCallback' overrides 'ImmutableState::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'SafeCallback' overrides 'ImmutableState::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'SafeCallback' overrides 'ImmutableState::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'SafeCallback' overrides 'ImmutableState::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'SafeCallback' overrides 'ImmutableState::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'SafeCallback' overrides 'ImmutableState::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'SafeCallback' overrides 'ImmutableState::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'SafeCallback' overrides 'ImmutableState::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePortion' in 'SafeCallback' overrides 'ImmutableState::calculatePortion' which is not marked 'virtual' |
| warning | W200 | function 'calculatePortion' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'BaseActionsRouter' overrides 'SafeCallback::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'BaseActionsRouter' overrides 'SafeCallback::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'BaseActionsRouter' overrides 'SafeCallback::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'BaseActionsRouter' overrides 'SafeCallback::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'BaseActionsRouter' overrides 'SafeCallback::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'BaseActionsRouter' overrides 'SafeCallback::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'BaseActionsRouter' overrides 'SafeCallback::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'BaseActionsRouter' overrides 'SafeCallback::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'BaseActionsRouter' overrides 'SafeCallback::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'BaseActionsRouter' overrides 'SafeCallback::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'BaseActionsRouter' overrides 'SafeCallback::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'BaseActionsRouter' overrides 'SafeCallback::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'BaseActionsRouter' overrides 'SafeCallback::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'BaseActionsRouter' overrides 'SafeCallback::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'BaseActionsRouter' overrides 'SafeCallback::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'BaseActionsRouter' overrides 'SafeCallback::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'BaseActionsRouter' overrides 'SafeCallback::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'BaseActionsRouter' overrides 'SafeCallback::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'BaseActionsRouter' overrides 'SafeCallback::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'BaseActionsRouter' overrides 'SafeCallback::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'BaseActionsRouter' overrides 'SafeCallback::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'BaseActionsRouter' overrides 'SafeCallback::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'BaseActionsRouter' overrides 'SafeCallback::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'BaseActionsRouter' overrides 'SafeCallback::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'BaseActionsRouter' overrides 'SafeCallback::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'BaseActionsRouter' overrides 'SafeCallback::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'BaseActionsRouter' overrides 'SafeCallback::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'BaseActionsRouter' overrides 'SafeCallback::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'BaseActionsRouter' overrides 'SafeCallback::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'BaseActionsRouter' overrides 'SafeCallback::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'BaseActionsRouter' overrides 'SafeCallback::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'BaseActionsRouter' overrides 'SafeCallback::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'BaseActionsRouter' overrides 'SafeCallback::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'BaseActionsRouter' overrides 'SafeCallback::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'BaseActionsRouter' overrides 'SafeCallback::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'BaseActionsRouter' overrides 'SafeCallback::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'BaseActionsRouter' overrides 'SafeCallback::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'BaseActionsRouter' overrides 'SafeCallback::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'BaseActionsRouter' overrides 'SafeCallback::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'BaseActionsRouter' overrides 'SafeCallback::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'BaseActionsRouter' overrides 'SafeCallback::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'BaseActionsRouter' overrides 'SafeCallback::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'BaseActionsRouter' overrides 'SafeCallback::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'BaseActionsRouter' overrides 'SafeCallback::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePortion' in 'BaseActionsRouter' overrides 'SafeCallback::calculatePortion' which is not marked 'virtual' |
| warning | W200 | function 'calculatePortion' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'DeltaResolver' overrides 'BaseActionsRouter::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'DeltaResolver' overrides 'BaseActionsRouter::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'DeltaResolver' overrides 'BaseActionsRouter::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'DeltaResolver' overrides 'BaseActionsRouter::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'DeltaResolver' overrides 'BaseActionsRouter::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'DeltaResolver' overrides 'BaseActionsRouter::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'DeltaResolver' overrides 'BaseActionsRouter::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'DeltaResolver' overrides 'BaseActionsRouter::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'DeltaResolver' overrides 'BaseActionsRouter::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'DeltaResolver' overrides 'BaseActionsRouter::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'DeltaResolver' overrides 'BaseActionsRouter::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'DeltaResolver' overrides 'BaseActionsRouter::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'DeltaResolver' overrides 'BaseActionsRouter::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'DeltaResolver' overrides 'BaseActionsRouter::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'DeltaResolver' overrides 'BaseActionsRouter::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'DeltaResolver' overrides 'BaseActionsRouter::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'DeltaResolver' overrides 'BaseActionsRouter::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'DeltaResolver' overrides 'BaseActionsRouter::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'DeltaResolver' overrides 'BaseActionsRouter::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'DeltaResolver' overrides 'BaseActionsRouter::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'DeltaResolver' overrides 'BaseActionsRouter::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'DeltaResolver' overrides 'BaseActionsRouter::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'DeltaResolver' overrides 'BaseActionsRouter::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'DeltaResolver' overrides 'BaseActionsRouter::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'DeltaResolver' overrides 'BaseActionsRouter::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'DeltaResolver' overrides 'BaseActionsRouter::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'DeltaResolver' overrides 'BaseActionsRouter::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'DeltaResolver' overrides 'BaseActionsRouter::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'DeltaResolver' overrides 'BaseActionsRouter::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'DeltaResolver' overrides 'BaseActionsRouter::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'DeltaResolver' overrides 'BaseActionsRouter::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'DeltaResolver' overrides 'BaseActionsRouter::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'DeltaResolver' overrides 'BaseActionsRouter::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'DeltaResolver' overrides 'BaseActionsRouter::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'DeltaResolver' overrides 'BaseActionsRouter::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'DeltaResolver' overrides 'BaseActionsRouter::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'DeltaResolver' overrides 'BaseActionsRouter::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'DeltaResolver' overrides 'BaseActionsRouter::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'DeltaResolver' overrides 'BaseActionsRouter::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'DeltaResolver' overrides 'BaseActionsRouter::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'DeltaResolver' overrides 'BaseActionsRouter::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'DeltaResolver' overrides 'BaseActionsRouter::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePortion' in 'DeltaResolver' overrides 'BaseActionsRouter::calculatePortion' which is not marked 'virtual' |
| warning | W200 | function 'calculatePortion' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'V4Router' overrides 'DeltaResolver::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'V4Router' overrides 'DeltaResolver::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'V4Router' overrides 'DeltaResolver::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'V4Router' overrides 'DeltaResolver::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'V4Router' overrides 'DeltaResolver::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'V4Router' overrides 'DeltaResolver::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'V4Router' overrides 'DeltaResolver::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'V4Router' overrides 'DeltaResolver::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'V4Router' overrides 'DeltaResolver::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'V4Router' overrides 'DeltaResolver::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'V4Router' overrides 'DeltaResolver::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'V4Router' overrides 'DeltaResolver::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'V4Router' overrides 'DeltaResolver::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'V4Router' overrides 'DeltaResolver::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'V4Router' overrides 'DeltaResolver::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'V4Router' overrides 'DeltaResolver::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'V4Router' overrides 'DeltaResolver::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'V4Router' overrides 'DeltaResolver::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'V4Router' overrides 'DeltaResolver::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'V4Router' overrides 'DeltaResolver::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'V4Router' overrides 'DeltaResolver::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'V4Router' overrides 'DeltaResolver::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'V4Router' overrides 'DeltaResolver::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'V4Router' overrides 'DeltaResolver::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'V4Router' overrides 'DeltaResolver::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'V4Router' overrides 'DeltaResolver::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'V4Router' overrides 'DeltaResolver::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'V4Router' overrides 'DeltaResolver::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'V4Router' overrides 'DeltaResolver::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'V4Router' overrides 'DeltaResolver::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'V4Router' overrides 'DeltaResolver::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'V4Router' overrides 'DeltaResolver::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'V4Router' overrides 'DeltaResolver::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'V4Router' overrides 'DeltaResolver::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'V4Router' overrides 'DeltaResolver::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'V4Router' overrides 'DeltaResolver::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'V4Router' overrides 'DeltaResolver::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'V4Router' overrides 'DeltaResolver::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'V4Router' overrides 'DeltaResolver::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'V4Router' overrides 'DeltaResolver::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'V4Router' overrides 'DeltaResolver::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'V4Router' overrides 'DeltaResolver::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'V4Router' overrides 'DeltaResolver::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'V4Router' overrides 'DeltaResolver::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'V4Router' overrides 'DeltaResolver::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'V4Router' overrides 'DeltaResolver::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'V4Router' overrides 'DeltaResolver::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'V4Router' overrides 'DeltaResolver::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'V4Router' overrides 'DeltaResolver::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'V4Router' overrides 'DeltaResolver::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'V4Router' overrides 'DeltaResolver::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'V4Router' overrides 'DeltaResolver::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'V4Router' overrides 'DeltaResolver::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'V4Router' overrides 'DeltaResolver::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'V4Router' overrides 'DeltaResolver::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'V4Router' overrides 'DeltaResolver::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'V4Router' overrides 'DeltaResolver::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'V4Router' overrides 'DeltaResolver::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'V4Router' overrides 'DeltaResolver::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePortion' in 'V4Router' overrides 'DeltaResolver::calculatePortion' which is not marked 'virtual' |
| warning | W200 | function 'calculatePortion' in 'V4Router' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'V4Router' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@uniswap/v4-periphery/src/V4Router.sol`