# ProtocolFees (Uniswap V4 Core)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@uniswap/v4-core/src/ProtocolFees.sol`
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
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Owned' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_PROTOCOL_FEE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_0_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'FEE_1_THRESHOLD' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PIPS_DENOMINATOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'DYNAMIC_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVERRIDE_FEE_FLAG' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'REMOVE_OVERRIDE_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_LP_FEE' detected while merging libraries |
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
| warning | W200 | function 'getSyncedCurrency' in 'ProtocolFees' overrides 'Owned::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'ProtocolFees' overrides 'Owned::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ProtocolFees' overrides 'Owned::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ProtocolFees' overrides 'Owned::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ProtocolFees' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'getZeroForOneFee' in 'ProtocolFees' overrides 'Owned::getZeroForOneFee' which is not marked 'virtual' |
| warning | W200 | function 'getZeroForOneFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getOneForZeroFee' in 'ProtocolFees' overrides 'Owned::getOneForZeroFee' which is not marked 'virtual' |
| warning | W200 | function 'getOneForZeroFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isValidProtocolFee' in 'ProtocolFees' overrides 'Owned::isValidProtocolFee' which is not marked 'virtual' |
| warning | W200 | function 'isValidProtocolFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculateSwapFee' in 'ProtocolFees' overrides 'Owned::calculateSwapFee' which is not marked 'virtual' |
| warning | W200 | function 'calculateSwapFee' in 'ProtocolFees' overrides a base function but is not marked 'override' |
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
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ProtocolFees' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@uniswap/v4-core/src/ProtocolFees.sol`