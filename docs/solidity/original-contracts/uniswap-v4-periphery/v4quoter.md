# V4Quoter (Uniswap V4 Periphery)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@uniswap/v4-periphery/src/lens/V4Quoter.sol`
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
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ImmutableState' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOLS_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_GROWTH_GLOBAL0_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDITY_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICKS_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_BITMAP_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POSITIONS_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
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
| warning | W200 | function 'mulDiv' in 'SafeCallback' overrides 'ImmutableState::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'SafeCallback' overrides 'ImmutableState::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'SafeCallback' overrides 'ImmutableState::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'SafeCallback' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'SafeCallback' overrides 'ImmutableState::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'SafeCallback' overrides 'ImmutableState::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'SafeCallback' overrides 'ImmutableState::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'SafeCallback' overrides 'ImmutableState::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'SafeCallback' overrides 'ImmutableState::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'SafeCallback' overrides 'ImmutableState::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'SafeCallback' overrides 'ImmutableState::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'SafeCallback' overrides 'ImmutableState::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'SafeCallback' overrides 'ImmutableState::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'SafeCallback' overrides 'ImmutableState::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'SafeCallback' overrides 'ImmutableState::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'SafeCallback' overrides 'ImmutableState::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'SafeCallback' overrides 'ImmutableState::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'SafeCallback' overrides 'ImmutableState::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides 'ImmutableState::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'SafeCallback' overrides 'ImmutableState::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'SafeCallback' overrides 'ImmutableState::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'SafeCallback' overrides 'ImmutableState::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertQuote' in 'SafeCallback' overrides 'ImmutableState::revertQuote' which is not marked 'virtual' |
| warning | W200 | function 'revertQuote' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleReason' in 'SafeCallback' overrides 'ImmutableState::bubbleReason' which is not marked 'virtual' |
| warning | W200 | function 'bubbleReason' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseQuoteAmount' in 'SafeCallback' overrides 'ImmutableState::parseQuoteAmount' which is not marked 'virtual' |
| warning | W200 | function 'parseQuoteAmount' in 'SafeCallback' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'set' in 'SafeCallback' overrides 'ImmutableState::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'SafeCallback' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'SafeCallback' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOLS_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_GROWTH_GLOBAL0_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDITY_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICKS_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_BITMAP_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POSITIONS_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOLS_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_GROWTH_GLOBAL0_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDITY_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICKS_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_BITMAP_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POSITIONS_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | VALIDATION_WARNING | 'using X for *' is supported; all library functions are available but type-specific filtering is not enforced at compile time |
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
| warning | W200 | function 'mulDiv' in 'SafeCallback' overrides 'ImmutableState::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'SafeCallback' overrides 'ImmutableState::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'SafeCallback' overrides 'ImmutableState::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'SafeCallback' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'SafeCallback' overrides 'ImmutableState::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'SafeCallback' overrides 'ImmutableState::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'SafeCallback' overrides 'ImmutableState::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'SafeCallback' overrides 'ImmutableState::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'SafeCallback' overrides 'ImmutableState::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'SafeCallback' overrides 'ImmutableState::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'SafeCallback' overrides 'ImmutableState::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'SafeCallback' overrides 'ImmutableState::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'SafeCallback' overrides 'ImmutableState::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'SafeCallback' overrides 'ImmutableState::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'SafeCallback' overrides 'ImmutableState::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'SafeCallback' overrides 'ImmutableState::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'SafeCallback' overrides 'ImmutableState::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'SafeCallback' overrides 'ImmutableState::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides 'ImmutableState::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'SafeCallback' overrides 'ImmutableState::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'SafeCallback' overrides 'ImmutableState::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'SafeCallback' overrides 'ImmutableState::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertQuote' in 'SafeCallback' overrides 'ImmutableState::revertQuote' which is not marked 'virtual' |
| warning | W200 | function 'revertQuote' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleReason' in 'SafeCallback' overrides 'ImmutableState::bubbleReason' which is not marked 'virtual' |
| warning | W200 | function 'bubbleReason' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseQuoteAmount' in 'SafeCallback' overrides 'ImmutableState::parseQuoteAmount' which is not marked 'virtual' |
| warning | W200 | function 'parseQuoteAmount' in 'SafeCallback' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'set' in 'SafeCallback' overrides 'ImmutableState::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'SafeCallback' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'BaseV4Quoter' overrides 'SafeCallback::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'BaseV4Quoter' overrides 'SafeCallback::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'BaseV4Quoter' overrides 'SafeCallback::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'BaseV4Quoter' overrides 'SafeCallback::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'BaseV4Quoter' overrides 'SafeCallback::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'BaseV4Quoter' overrides 'SafeCallback::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'BaseV4Quoter' overrides 'SafeCallback::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'BaseV4Quoter' overrides 'SafeCallback::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'BaseV4Quoter' overrides 'SafeCallback::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'BaseV4Quoter' overrides 'SafeCallback::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'BaseV4Quoter' overrides 'SafeCallback::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'BaseV4Quoter' overrides 'SafeCallback::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'BaseV4Quoter' overrides 'SafeCallback::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'BaseV4Quoter' overrides 'SafeCallback::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'BaseV4Quoter' overrides 'SafeCallback::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'BaseV4Quoter' overrides 'SafeCallback::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'BaseV4Quoter' overrides 'SafeCallback::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'BaseV4Quoter' overrides 'SafeCallback::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'BaseV4Quoter' overrides 'SafeCallback::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'BaseV4Quoter' overrides 'SafeCallback::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'BaseV4Quoter' overrides 'SafeCallback::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'BaseV4Quoter' overrides 'SafeCallback::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'BaseV4Quoter' overrides 'SafeCallback::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'BaseV4Quoter' overrides 'SafeCallback::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'BaseV4Quoter' overrides 'SafeCallback::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'BaseV4Quoter' overrides 'SafeCallback::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'BaseV4Quoter' overrides 'SafeCallback::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'BaseV4Quoter' overrides 'SafeCallback::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'BaseV4Quoter' overrides 'SafeCallback::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'BaseV4Quoter' overrides 'SafeCallback::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'BaseV4Quoter' overrides 'SafeCallback::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'BaseV4Quoter' overrides 'SafeCallback::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'BaseV4Quoter' overrides 'SafeCallback::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'BaseV4Quoter' overrides 'SafeCallback::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'BaseV4Quoter' overrides 'SafeCallback::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'BaseV4Quoter' overrides 'SafeCallback::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'BaseV4Quoter' overrides 'SafeCallback::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'BaseV4Quoter' overrides 'SafeCallback::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'BaseV4Quoter' overrides 'SafeCallback::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'BaseV4Quoter' overrides 'SafeCallback::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'BaseV4Quoter' overrides 'SafeCallback::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'BaseV4Quoter' overrides 'SafeCallback::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertQuote' in 'BaseV4Quoter' overrides 'SafeCallback::revertQuote' which is not marked 'virtual' |
| warning | W200 | function 'revertQuote' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleReason' in 'BaseV4Quoter' overrides 'SafeCallback::bubbleReason' which is not marked 'virtual' |
| warning | W200 | function 'bubbleReason' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseQuoteAmount' in 'BaseV4Quoter' overrides 'SafeCallback::parseQuoteAmount' which is not marked 'virtual' |
| warning | W200 | function 'parseQuoteAmount' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'BaseV4Quoter' overrides 'SafeCallback::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'BaseV4Quoter' overrides 'SafeCallback::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'BaseV4Quoter' overrides 'SafeCallback::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'BaseV4Quoter' overrides 'SafeCallback::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'BaseV4Quoter' overrides 'SafeCallback::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'BaseV4Quoter' overrides 'SafeCallback::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'BaseV4Quoter' overrides 'SafeCallback::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'BaseV4Quoter' overrides 'SafeCallback::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'BaseV4Quoter' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOLS_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_GROWTH_GLOBAL0_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDITY_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICKS_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_BITMAP_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POSITIONS_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOLS_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_GROWTH_GLOBAL0_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDITY_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICKS_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_BITMAP_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POSITIONS_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q128' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POOLS_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_GROWTH_GLOBAL0_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LIQUIDITY_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICKS_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_BITMAP_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'POSITIONS_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | VALIDATION_WARNING | 'using X for *' is supported; all library functions are available but type-specific filtering is not enforced at compile time |
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
| warning | W200 | function 'mulDiv' in 'SafeCallback' overrides 'ImmutableState::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'SafeCallback' overrides 'ImmutableState::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'SafeCallback' overrides 'ImmutableState::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'SafeCallback' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'SafeCallback' overrides 'ImmutableState::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'SafeCallback' overrides 'ImmutableState::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'SafeCallback' overrides 'ImmutableState::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'SafeCallback' overrides 'ImmutableState::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'SafeCallback' overrides 'ImmutableState::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'SafeCallback' overrides 'ImmutableState::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'SafeCallback' overrides 'ImmutableState::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'SafeCallback' overrides 'ImmutableState::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'SafeCallback' overrides 'ImmutableState::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'SafeCallback' overrides 'ImmutableState::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'SafeCallback' overrides 'ImmutableState::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'SafeCallback' overrides 'ImmutableState::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'SafeCallback' overrides 'ImmutableState::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'SafeCallback' overrides 'ImmutableState::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides 'ImmutableState::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'SafeCallback' overrides 'ImmutableState::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'SafeCallback' overrides 'ImmutableState::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'SafeCallback' overrides 'ImmutableState::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertQuote' in 'SafeCallback' overrides 'ImmutableState::revertQuote' which is not marked 'virtual' |
| warning | W200 | function 'revertQuote' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleReason' in 'SafeCallback' overrides 'ImmutableState::bubbleReason' which is not marked 'virtual' |
| warning | W200 | function 'bubbleReason' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseQuoteAmount' in 'SafeCallback' overrides 'ImmutableState::parseQuoteAmount' which is not marked 'virtual' |
| warning | W200 | function 'parseQuoteAmount' in 'SafeCallback' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'set' in 'SafeCallback' overrides 'ImmutableState::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'SafeCallback' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides 'SafeCallback::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'BaseV4Quoter' overrides 'SafeCallback::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'BaseV4Quoter' overrides 'SafeCallback::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'BaseV4Quoter' overrides 'SafeCallback::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'BaseV4Quoter' overrides 'SafeCallback::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'BaseV4Quoter' overrides 'SafeCallback::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'BaseV4Quoter' overrides 'SafeCallback::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'BaseV4Quoter' overrides 'SafeCallback::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'BaseV4Quoter' overrides 'SafeCallback::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'BaseV4Quoter' overrides 'SafeCallback::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'BaseV4Quoter' overrides 'SafeCallback::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'BaseV4Quoter' overrides 'SafeCallback::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'BaseV4Quoter' overrides 'SafeCallback::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'BaseV4Quoter' overrides 'SafeCallback::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'BaseV4Quoter' overrides 'SafeCallback::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'BaseV4Quoter' overrides 'SafeCallback::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'BaseV4Quoter' overrides 'SafeCallback::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'BaseV4Quoter' overrides 'SafeCallback::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'BaseV4Quoter' overrides 'SafeCallback::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'BaseV4Quoter' overrides 'SafeCallback::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'BaseV4Quoter' overrides 'SafeCallback::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'BaseV4Quoter' overrides 'SafeCallback::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'BaseV4Quoter' overrides 'SafeCallback::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'BaseV4Quoter' overrides 'SafeCallback::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'BaseV4Quoter' overrides 'SafeCallback::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'BaseV4Quoter' overrides 'SafeCallback::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'BaseV4Quoter' overrides 'SafeCallback::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'BaseV4Quoter' overrides 'SafeCallback::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'BaseV4Quoter' overrides 'SafeCallback::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'BaseV4Quoter' overrides 'SafeCallback::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'BaseV4Quoter' overrides 'SafeCallback::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'BaseV4Quoter' overrides 'SafeCallback::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'BaseV4Quoter' overrides 'SafeCallback::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'BaseV4Quoter' overrides 'SafeCallback::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'BaseV4Quoter' overrides 'SafeCallback::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'BaseV4Quoter' overrides 'SafeCallback::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'BaseV4Quoter' overrides 'SafeCallback::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'BaseV4Quoter' overrides 'SafeCallback::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'BaseV4Quoter' overrides 'SafeCallback::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'BaseV4Quoter' overrides 'SafeCallback::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'BaseV4Quoter' overrides 'SafeCallback::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'BaseV4Quoter' overrides 'SafeCallback::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'BaseV4Quoter' overrides 'SafeCallback::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertQuote' in 'BaseV4Quoter' overrides 'SafeCallback::revertQuote' which is not marked 'virtual' |
| warning | W200 | function 'revertQuote' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleReason' in 'BaseV4Quoter' overrides 'SafeCallback::bubbleReason' which is not marked 'virtual' |
| warning | W200 | function 'bubbleReason' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseQuoteAmount' in 'BaseV4Quoter' overrides 'SafeCallback::parseQuoteAmount' which is not marked 'virtual' |
| warning | W200 | function 'parseQuoteAmount' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'BaseV4Quoter' overrides 'SafeCallback::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'BaseV4Quoter' overrides 'SafeCallback::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'BaseV4Quoter' overrides 'SafeCallback::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'BaseV4Quoter' overrides 'SafeCallback::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'BaseV4Quoter' overrides 'SafeCallback::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'BaseV4Quoter' overrides 'SafeCallback::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'BaseV4Quoter' overrides 'SafeCallback::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'BaseV4Quoter' overrides 'SafeCallback::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'BaseV4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides 'BaseV4Quoter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides 'BaseV4Quoter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides 'BaseV4Quoter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides 'BaseV4Quoter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides 'BaseV4Quoter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides 'BaseV4Quoter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides 'BaseV4Quoter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'V4Quoter' overrides 'BaseV4Quoter::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'V4Quoter' overrides 'BaseV4Quoter::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'V4Quoter' overrides 'BaseV4Quoter::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'V4Quoter' overrides 'BaseV4Quoter::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'V4Quoter' overrides 'BaseV4Quoter::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'V4Quoter' overrides 'BaseV4Quoter::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'V4Quoter' overrides 'BaseV4Quoter::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'V4Quoter' overrides 'BaseV4Quoter::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'V4Quoter' overrides 'BaseV4Quoter::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'V4Quoter' overrides 'BaseV4Quoter::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'V4Quoter' overrides 'BaseV4Quoter::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'V4Quoter' overrides 'BaseV4Quoter::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'V4Quoter' overrides 'BaseV4Quoter::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'V4Quoter' overrides 'BaseV4Quoter::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'V4Quoter' overrides 'BaseV4Quoter::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'V4Quoter' overrides 'BaseV4Quoter::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'V4Quoter' overrides 'BaseV4Quoter::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'V4Quoter' overrides 'BaseV4Quoter::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'V4Quoter' overrides 'BaseV4Quoter::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'V4Quoter' overrides 'BaseV4Quoter::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'V4Quoter' overrides 'BaseV4Quoter::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'V4Quoter' overrides 'BaseV4Quoter::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'V4Quoter' overrides 'BaseV4Quoter::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'V4Quoter' overrides 'BaseV4Quoter::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'V4Quoter' overrides 'BaseV4Quoter::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'V4Quoter' overrides 'BaseV4Quoter::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'V4Quoter' overrides 'BaseV4Quoter::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'V4Quoter' overrides 'BaseV4Quoter::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'V4Quoter' overrides 'BaseV4Quoter::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'V4Quoter' overrides 'BaseV4Quoter::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'V4Quoter' overrides 'BaseV4Quoter::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'V4Quoter' overrides 'BaseV4Quoter::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'V4Quoter' overrides 'BaseV4Quoter::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'V4Quoter' overrides 'BaseV4Quoter::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'V4Quoter' overrides 'BaseV4Quoter::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'V4Quoter' overrides 'BaseV4Quoter::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'V4Quoter' overrides 'BaseV4Quoter::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'V4Quoter' overrides 'BaseV4Quoter::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'V4Quoter' overrides 'BaseV4Quoter::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseSelector' in 'V4Quoter' overrides 'BaseV4Quoter::parseSelector' which is not marked 'virtual' |
| warning | W200 | function 'parseSelector' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseFee' in 'V4Quoter' overrides 'BaseV4Quoter::parseFee' which is not marked 'virtual' |
| warning | W200 | function 'parseFee' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseReturnDelta' in 'V4Quoter' overrides 'BaseV4Quoter::parseReturnDelta' which is not marked 'virtual' |
| warning | W200 | function 'parseReturnDelta' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertQuote' in 'V4Quoter' overrides 'BaseV4Quoter::revertQuote' which is not marked 'virtual' |
| warning | W200 | function 'revertQuote' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleReason' in 'V4Quoter' overrides 'BaseV4Quoter::bubbleReason' which is not marked 'virtual' |
| warning | W200 | function 'bubbleReason' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseQuoteAmount' in 'V4Quoter' overrides 'BaseV4Quoter::parseQuoteAmount' which is not marked 'virtual' |
| warning | W200 | function 'parseQuoteAmount' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'V4Quoter' overrides 'BaseV4Quoter::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'V4Quoter' overrides 'BaseV4Quoter::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'V4Quoter' overrides 'BaseV4Quoter::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'V4Quoter' overrides 'BaseV4Quoter::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'V4Quoter' overrides 'BaseV4Quoter::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'V4Quoter' overrides 'BaseV4Quoter::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'V4Quoter' overrides 'BaseV4Quoter::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'V4Quoter' overrides 'BaseV4Quoter::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'V4Quoter' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'V4Quoter' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@uniswap/v4-periphery/src/lens/V4Quoter.sol`