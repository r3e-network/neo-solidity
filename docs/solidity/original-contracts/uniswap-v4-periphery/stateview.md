# StateView (Uniswap V4 Periphery)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@uniswap/v4-periphery/src/lens/StateView.sol`
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
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | INVALID_STORAGE_RETURN | function '__super_get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | W200 | function 'revertWith' in 'StateView' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'StateView' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'StateView' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'StateView' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'StateView' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'StateView' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'StateView' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'StateView' overrides 'ImmutableState::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'StateView' overrides 'ImmutableState::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'StateView' overrides 'ImmutableState::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'StateView' overrides 'ImmutableState::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'StateView' overrides 'ImmutableState::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'StateView' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'StateView' overrides 'ImmutableState::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'StateView' overrides 'ImmutableState::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'StateView' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'StateView' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'StateView' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'StateView' overrides 'ImmutableState::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'StateView' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'StateView' overrides 'ImmutableState::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'StateView' overrides 'ImmutableState::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'StateView' overrides 'ImmutableState::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'StateView' overrides 'ImmutableState::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'StateView' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'StateView' overrides 'ImmutableState::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'StateView' overrides 'ImmutableState::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'StateView' overrides 'ImmutableState::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'StateView' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'StateView' overrides 'ImmutableState::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'StateView' overrides 'ImmutableState::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'StateView' overrides 'ImmutableState::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'StateView' overrides 'ImmutableState::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'StateView' overrides 'ImmutableState::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'StateView' overrides 'ImmutableState::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'StateView' overrides 'ImmutableState::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'StateView' overrides 'ImmutableState::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'StateView' overrides 'ImmutableState::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'StateView' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'StateView' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'StateView' overrides 'ImmutableState::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'StateView' overrides 'ImmutableState::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'StateView' overrides 'ImmutableState::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'StateView' overrides 'ImmutableState::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'StateView' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'StateView' overrides 'ImmutableState::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'StateView' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'StateView' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@uniswap/v4-periphery/src/lens/StateView.sol`