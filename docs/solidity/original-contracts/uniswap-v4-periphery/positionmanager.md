# PositionManager (Uniswap V4 Periphery)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@uniswap/v4-periphery/src/PositionManager.sol`
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
| error | RAW | [info][NEP-11] NEP-11 `Transfer` event has 3 parameter(s), expected 4. |
| error | RAW | [info][NEP-11] NEP-11 `Transfer` event has 3 parameter(s), expected 4. |
| error | RAW | [info][NEP-11] NEP-11 `Transfer` event has 3 parameter(s), expected 4. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | W104 | function 'transferFrom' with 3 parameters (ERC-721 pattern) detected. NEP-11 uses transfer(to, tokenId, data) with 3 parameters instead. Authorization is via Runtime.checkWitness(owner), not msg.sender. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W114 | NFT contract (has ownerOf) but missing onNEP11Payment callback. Other contracts cannot send NFTs to this contract. |
| warning | VALIDATION_WARNING | abstract contract 'ERC721' has 1 unimplemented function(s): [tokenURI] |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ERC721' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ERC721TokenReceiver' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'EIP712_v4' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'revokeNonce' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | W116 | function 'revokeNonce' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'UnorderedNonce' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'revokeNonce' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'permit' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'permitForAll' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | W104 | function 'transferFrom' with 3 parameters (ERC-721 pattern) detected. NEP-11 uses transfer(to, tokenId, data) with 3 parameters instead. Authorization is via Runtime.checkWitness(owner), not msg.sender. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W114 | NFT contract (has ownerOf) but missing onNEP11Payment callback. Other contracts cannot send NFTs to this contract. |
| warning | W116 | function 'revokeNonce' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'permit' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'permitForAll' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | VALIDATION_WARNING | abstract contract 'ERC721Permit_v4' has 1 unimplemented function(s): [tokenURI] |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'EIP712_v4' overrides 'ERC721::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'EIP712_v4' overrides 'ERC721::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'EIP712_v4' overrides 'ERC721::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'EIP712_v4' overrides 'ERC721::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'EIP712_v4' overrides 'ERC721::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'EIP712_v4' overrides 'ERC721::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'EIP712_v4' overrides 'ERC721::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'EIP712_v4' overrides 'ERC721::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'EIP712_v4' overrides 'ERC721::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'EIP712_v4' overrides 'ERC721::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'EIP712_v4' overrides 'ERC721::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'EIP712_v4' overrides 'ERC721::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'EIP712_v4' overrides 'ERC721::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'EIP712_v4' overrides 'ERC721::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'EIP712_v4' overrides 'ERC721::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'EIP712_v4' overrides 'ERC721::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'EIP712_v4' overrides 'ERC721::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'EIP712_v4' overrides 'ERC721::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712_v4' overrides 'ERC721::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'EIP712_v4' overrides 'ERC721::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'EIP712_v4' overrides 'ERC721::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'EIP712_v4' overrides 'ERC721::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'EIP712_v4' overrides 'ERC721::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'EIP712_v4' overrides 'ERC721::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'EIP712_v4' overrides 'ERC721::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'EIP712_v4' overrides 'ERC721::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'EIP712_v4' overrides 'ERC721::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'EIP712_v4' overrides 'ERC721::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'EIP712_v4' overrides 'ERC721::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'EIP712_v4' overrides 'ERC721::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'EIP712_v4' overrides 'ERC721::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'EIP712_v4' overrides 'ERC721::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'EIP712_v4' overrides 'ERC721::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'EIP712_v4' overrides 'ERC721::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'EIP712_v4' overrides 'ERC721::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'EIP712_v4' overrides 'ERC721::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'EIP712_v4' overrides 'ERC721::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'EIP712_v4' overrides 'ERC721::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'EIP712_v4' overrides 'ERC721::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'EIP712_v4' overrides 'ERC721::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'EIP712_v4' overrides 'ERC721::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'EIP712_v4' overrides 'ERC721::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'EIP712_v4' overrides 'ERC721::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'EIP712_v4' overrides 'ERC721::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'EIP712_v4' overrides 'ERC721::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'EIP712_v4' overrides 'ERC721::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'EIP712_v4' overrides 'ERC721::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'EIP712_v4' overrides 'ERC721::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'EIP712_v4' overrides 'ERC721::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'EIP712_v4' overrides 'ERC721::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'EIP712_v4' overrides 'ERC721::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'EIP712_v4' overrides 'ERC721::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'EIP712_v4' overrides 'ERC721::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'EIP712_v4' overrides 'ERC721::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'EIP712_v4' overrides 'ERC721::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'EIP712_v4' overrides 'ERC721::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'EIP712_v4' overrides 'ERC721::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'EIP712_v4' overrides 'ERC721::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'EIP712_v4' overrides 'ERC721::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'EIP712_v4' overrides 'ERC721::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'EIP712_v4' overrides 'ERC721::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'EIP712_v4' overrides 'ERC721::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'EIP712_v4' overrides 'ERC721::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'EIP712_v4' overrides 'ERC721::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'EIP712_v4' overrides 'ERC721::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'EIP712_v4' overrides 'ERC721::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'EIP712_v4' overrides 'ERC721::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'EIP712_v4' overrides 'ERC721::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'EIP712_v4' overrides 'ERC721::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'EIP712_v4' overrides 'ERC721::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'EIP712_v4' overrides 'ERC721::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'EIP712_v4' overrides 'ERC721::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'EIP712_v4' overrides 'ERC721::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'EIP712_v4' overrides 'ERC721::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'EIP712_v4' overrides 'ERC721::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'EIP712_v4' overrides 'ERC721::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'EIP712_v4' overrides 'ERC721::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'EIP712_v4' overrides 'ERC721::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'EIP712_v4' overrides 'ERC721::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'EIP712_v4' overrides 'ERC721::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'EIP712_v4' overrides 'ERC721::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'EIP712_v4' overrides 'ERC721::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'EIP712_v4' overrides 'ERC721::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'EIP712_v4' overrides 'ERC721::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'EIP712_v4' overrides 'ERC721::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'EIP712_v4' overrides 'ERC721::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'EIP712_v4' overrides 'ERC721::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'EIP712_v4' overrides 'ERC721::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'EIP712_v4' overrides 'ERC721::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'EIP712_v4' overrides 'ERC721::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'EIP712_v4' overrides 'ERC721::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'EIP712_v4' overrides 'ERC721::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'EIP712_v4' overrides 'ERC721::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'EIP712_v4' overrides 'ERC721::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'EIP712_v4' overrides 'ERC721::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'EIP712_v4' overrides 'ERC721::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'UnorderedNonce' overrides 'EIP712_v4::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'UnorderedNonce' overrides 'EIP712_v4::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'UnorderedNonce' overrides 'EIP712_v4::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'UnorderedNonce' overrides 'EIP712_v4::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'UnorderedNonce' overrides 'EIP712_v4::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'UnorderedNonce' overrides 'EIP712_v4::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'UnorderedNonce' overrides 'EIP712_v4::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'UnorderedNonce' overrides 'EIP712_v4::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'UnorderedNonce' overrides 'EIP712_v4::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'UnorderedNonce' overrides 'EIP712_v4::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'UnorderedNonce' overrides 'EIP712_v4::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'UnorderedNonce' overrides 'EIP712_v4::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'UnorderedNonce' overrides 'EIP712_v4::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'UnorderedNonce' overrides 'EIP712_v4::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'UnorderedNonce' overrides 'EIP712_v4::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'UnorderedNonce' overrides 'EIP712_v4::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'UnorderedNonce' overrides 'EIP712_v4::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'UnorderedNonce' overrides 'EIP712_v4::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'UnorderedNonce' overrides 'EIP712_v4::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'UnorderedNonce' overrides 'EIP712_v4::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'UnorderedNonce' overrides 'EIP712_v4::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'UnorderedNonce' overrides 'EIP712_v4::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'UnorderedNonce' overrides 'EIP712_v4::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'UnorderedNonce' overrides 'EIP712_v4::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'UnorderedNonce' overrides 'EIP712_v4::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'UnorderedNonce' overrides 'EIP712_v4::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'UnorderedNonce' overrides 'EIP712_v4::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'UnorderedNonce' overrides 'EIP712_v4::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'UnorderedNonce' overrides 'EIP712_v4::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'UnorderedNonce' overrides 'EIP712_v4::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'UnorderedNonce' overrides 'EIP712_v4::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'UnorderedNonce' overrides 'EIP712_v4::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'UnorderedNonce' overrides 'EIP712_v4::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'UnorderedNonce' overrides 'EIP712_v4::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'UnorderedNonce' overrides 'EIP712_v4::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'UnorderedNonce' overrides 'EIP712_v4::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'UnorderedNonce' overrides 'EIP712_v4::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'UnorderedNonce' overrides 'EIP712_v4::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'UnorderedNonce' overrides 'EIP712_v4::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'UnorderedNonce' overrides 'EIP712_v4::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'UnorderedNonce' overrides 'EIP712_v4::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'UnorderedNonce' overrides 'EIP712_v4::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'UnorderedNonce' overrides 'EIP712_v4::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'UnorderedNonce' overrides 'EIP712_v4::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'UnorderedNonce' overrides 'EIP712_v4::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'UnorderedNonce' overrides 'EIP712_v4::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'UnorderedNonce' overrides 'EIP712_v4::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'UnorderedNonce' overrides 'EIP712_v4::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'UnorderedNonce' overrides 'EIP712_v4::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'UnorderedNonce' overrides 'EIP712_v4::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'UnorderedNonce' overrides 'EIP712_v4::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'UnorderedNonce' overrides 'EIP712_v4::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'UnorderedNonce' overrides 'EIP712_v4::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'UnorderedNonce' overrides 'EIP712_v4::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'UnorderedNonce' overrides 'EIP712_v4::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'UnorderedNonce' overrides 'EIP712_v4::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'UnorderedNonce' overrides 'EIP712_v4::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'UnorderedNonce' overrides 'EIP712_v4::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'UnorderedNonce' overrides 'EIP712_v4::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'UnorderedNonce' overrides 'EIP712_v4::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'UnorderedNonce' overrides 'EIP712_v4::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'UnorderedNonce' overrides 'EIP712_v4::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'UnorderedNonce' overrides 'EIP712_v4::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'UnorderedNonce' overrides 'EIP712_v4::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'UnorderedNonce' overrides 'EIP712_v4::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'UnorderedNonce' overrides 'EIP712_v4::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'UnorderedNonce' overrides 'EIP712_v4::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'UnorderedNonce' overrides 'EIP712_v4::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'UnorderedNonce' overrides 'EIP712_v4::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'UnorderedNonce' overrides 'EIP712_v4::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'UnorderedNonce' overrides 'EIP712_v4::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'UnorderedNonce' overrides 'EIP712_v4::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'UnorderedNonce' overrides 'EIP712_v4::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'UnorderedNonce' overrides 'EIP712_v4::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'UnorderedNonce' overrides 'EIP712_v4::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'UnorderedNonce' overrides 'EIP712_v4::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'UnorderedNonce' overrides 'EIP712_v4::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'UnorderedNonce' overrides 'EIP712_v4::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'UnorderedNonce' overrides 'EIP712_v4::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'ERC721Permit_v4' overrides 'UnorderedNonce::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'ERC721Permit_v4' overrides 'UnorderedNonce::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'ERC721Permit_v4' overrides 'UnorderedNonce::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'ERC721Permit_v4' overrides 'UnorderedNonce::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'ERC721Permit_v4' overrides 'UnorderedNonce::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'ERC721Permit_v4' overrides 'UnorderedNonce::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'ERC721Permit_v4' overrides 'UnorderedNonce::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC721Permit_v4' overrides 'UnorderedNonce::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'ERC721Permit_v4' overrides 'UnorderedNonce::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'ERC721Permit_v4' overrides 'UnorderedNonce::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ERC721Permit_v4' overrides 'UnorderedNonce::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'ERC721Permit_v4' overrides 'UnorderedNonce::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'ERC721Permit_v4' overrides 'UnorderedNonce::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'ERC721Permit_v4' overrides 'UnorderedNonce::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'ERC721Permit_v4' overrides 'UnorderedNonce::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'ERC721Permit_v4' overrides 'UnorderedNonce::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'ERC721Permit_v4' overrides 'UnorderedNonce::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ERC721Permit_v4' overrides 'UnorderedNonce::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'ERC721Permit_v4' overrides 'UnorderedNonce::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'ERC721Permit_v4' overrides 'UnorderedNonce::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'ERC721Permit_v4' overrides 'UnorderedNonce::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'ERC721Permit_v4' overrides 'UnorderedNonce::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'ERC721Permit_v4' overrides 'UnorderedNonce::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'ERC721Permit_v4' overrides 'UnorderedNonce::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'ERC721Permit_v4' overrides 'UnorderedNonce::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'ERC721Permit_v4' overrides 'UnorderedNonce::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'ERC721Permit_v4' overrides 'UnorderedNonce::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'ERC721Permit_v4' overrides 'UnorderedNonce::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'ERC721Permit_v4' overrides 'UnorderedNonce::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'ERC721Permit_v4' overrides 'UnorderedNonce::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'ERC721Permit_v4' overrides 'UnorderedNonce::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'ERC721Permit_v4' overrides 'UnorderedNonce::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'ERC721Permit_v4' overrides 'UnorderedNonce::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'ERC721Permit_v4' overrides 'UnorderedNonce::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'ERC721Permit_v4' overrides 'UnorderedNonce::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'ERC721Permit_v4' overrides 'UnorderedNonce::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'ERC721Permit_v4' overrides 'UnorderedNonce::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'ERC721Permit_v4' overrides 'UnorderedNonce::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'ERC721Permit_v4' overrides 'UnorderedNonce::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'ERC721Permit_v4' overrides 'UnorderedNonce::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ERC721Permit_v4' overrides 'UnorderedNonce::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'ERC721Permit_v4' overrides 'UnorderedNonce::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'ERC721Permit_v4' overrides 'UnorderedNonce::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ERC721Permit_v4' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ReentrancyLock' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'multicall' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | W116 | function 'multicall' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Multicall_v4' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ImmutableState' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'initializePool' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | W116 | function 'initializePool' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'PoolInitializer_v4' overrides 'ImmutableState::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'PoolInitializer_v4' overrides 'ImmutableState::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'PoolInitializer_v4' overrides 'ImmutableState::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'PoolInitializer_v4' overrides 'ImmutableState::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'PoolInitializer_v4' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'PoolInitializer_v4' overrides 'ImmutableState::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'PoolInitializer_v4' overrides 'ImmutableState::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'PoolInitializer_v4' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'PoolInitializer_v4' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'PoolInitializer_v4' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'PoolInitializer_v4' overrides 'ImmutableState::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'PoolInitializer_v4' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'PoolInitializer_v4' overrides 'ImmutableState::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'PoolInitializer_v4' overrides 'ImmutableState::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'PoolInitializer_v4' overrides 'ImmutableState::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'PoolInitializer_v4' overrides 'ImmutableState::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'PoolInitializer_v4' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'PoolInitializer_v4' overrides 'ImmutableState::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'PoolInitializer_v4' overrides 'ImmutableState::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'PoolInitializer_v4' overrides 'ImmutableState::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'PoolInitializer_v4' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'PoolInitializer_v4' overrides 'ImmutableState::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'PoolInitializer_v4' overrides 'ImmutableState::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'PoolInitializer_v4' overrides 'ImmutableState::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'PoolInitializer_v4' overrides 'ImmutableState::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'PoolInitializer_v4' overrides 'ImmutableState::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'PoolInitializer_v4' overrides 'ImmutableState::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'PoolInitializer_v4' overrides 'ImmutableState::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'PoolInitializer_v4' overrides 'ImmutableState::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'PoolInitializer_v4' overrides 'ImmutableState::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'PoolInitializer_v4' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'PoolInitializer_v4' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'PoolInitializer_v4' overrides 'ImmutableState::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'PoolInitializer_v4' overrides 'ImmutableState::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'PoolInitializer_v4' overrides 'ImmutableState::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'PoolInitializer_v4' overrides 'ImmutableState::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'PoolInitializer_v4' overrides 'ImmutableState::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'PoolInitializer_v4' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'PoolInitializer_v4' overrides 'ImmutableState::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'PoolInitializer_v4' overrides 'ImmutableState::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'PoolInitializer_v4' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'PoolInitializer_v4' overrides 'ImmutableState::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'PoolInitializer_v4' overrides 'ImmutableState::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'PoolInitializer_v4' overrides 'ImmutableState::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'PoolInitializer_v4' overrides 'ImmutableState::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'PoolInitializer_v4' overrides 'ImmutableState::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'PoolInitializer_v4' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'PoolInitializer_v4' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'PoolInitializer_v4' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'PoolInitializer_v4' overrides 'ImmutableState::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'PoolInitializer_v4' overrides 'ImmutableState::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'PoolInitializer_v4' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'PoolInitializer_v4' overrides 'ImmutableState::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'PoolInitializer_v4' overrides 'ImmutableState::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'PoolInitializer_v4' overrides 'ImmutableState::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'PoolInitializer_v4' overrides 'ImmutableState::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'PoolInitializer_v4' overrides 'ImmutableState::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'PoolInitializer_v4' overrides 'ImmutableState::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'PoolInitializer_v4' overrides 'ImmutableState::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'PoolInitializer_v4' overrides 'ImmutableState::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'PoolInitializer_v4' overrides 'ImmutableState::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'PoolInitializer_v4' overrides 'ImmutableState::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'PoolInitializer_v4' overrides 'ImmutableState::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'PoolInitializer_v4' overrides 'ImmutableState::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'PoolInitializer_v4' overrides 'ImmutableState::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'PoolInitializer_v4' overrides 'ImmutableState::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'PoolInitializer_v4' overrides 'ImmutableState::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'PoolInitializer_v4' overrides 'ImmutableState::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'PoolInitializer_v4' overrides 'ImmutableState::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'PoolInitializer_v4' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'PoolInitializer_v4' overrides 'ImmutableState::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'PoolInitializer_v4' overrides 'ImmutableState::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'PoolInitializer_v4' overrides 'ImmutableState::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'PoolInitializer_v4' overrides 'ImmutableState::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'PoolInitializer_v4' overrides 'ImmutableState::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'PoolInitializer_v4' overrides 'ImmutableState::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'PoolInitializer_v4' overrides 'ImmutableState::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'PoolInitializer_v4' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
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
| warning | W200 | function 'mulDiv' in 'DeltaResolver' overrides 'ImmutableState::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'DeltaResolver' overrides 'ImmutableState::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'DeltaResolver' overrides 'ImmutableState::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'DeltaResolver' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'DeltaResolver' overrides 'ImmutableState::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'DeltaResolver' overrides 'ImmutableState::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'DeltaResolver' overrides 'ImmutableState::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'DeltaResolver' overrides 'ImmutableState::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'DeltaResolver' overrides 'ImmutableState::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'DeltaResolver' overrides 'ImmutableState::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'DeltaResolver' overrides 'ImmutableState::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'DeltaResolver' overrides 'ImmutableState::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'DeltaResolver' overrides 'ImmutableState::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'DeltaResolver' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'DeltaResolver' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'DeltaResolver' overrides 'ImmutableState::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'DeltaResolver' overrides 'ImmutableState::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'DeltaResolver' overrides 'ImmutableState::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'DeltaResolver' overrides 'ImmutableState::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'DeltaResolver' overrides 'ImmutableState::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'DeltaResolver' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'poolId' in 'DeltaResolver' overrides 'ImmutableState::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'DeltaResolver' overrides 'ImmutableState::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'DeltaResolver' overrides 'ImmutableState::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'DeltaResolver' overrides 'ImmutableState::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'DeltaResolver' overrides 'ImmutableState::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'DeltaResolver' overrides 'ImmutableState::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'DeltaResolver' overrides 'ImmutableState::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'DeltaResolver' overrides 'ImmutableState::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'DeltaResolver' overrides 'ImmutableState::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'DeltaResolver' overrides 'ImmutableState::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'DeltaResolver' overrides 'ImmutableState::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'DeltaResolver' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'DeltaResolver' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'validateMinOut' in 'DeltaResolver' overrides 'ImmutableState::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'DeltaResolver' overrides 'ImmutableState::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'DeltaResolver' overrides 'ImmutableState::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'DeltaResolver' overrides 'ImmutableState::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'DeltaResolver' overrides 'ImmutableState::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'DeltaResolver' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W200 | function 'poolId' in 'SafeCallback' overrides 'ImmutableState::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'SafeCallback' overrides 'ImmutableState::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'SafeCallback' overrides 'ImmutableState::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'SafeCallback' overrides 'ImmutableState::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'SafeCallback' overrides 'ImmutableState::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'SafeCallback' overrides 'ImmutableState::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'SafeCallback' overrides 'ImmutableState::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'SafeCallback' overrides 'ImmutableState::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'SafeCallback' overrides 'ImmutableState::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'SafeCallback' overrides 'ImmutableState::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'SafeCallback' overrides 'ImmutableState::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'SafeCallback' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'SafeCallback' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'validateMinOut' in 'SafeCallback' overrides 'ImmutableState::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'SafeCallback' overrides 'ImmutableState::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'SafeCallback' overrides 'ImmutableState::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'SafeCallback' overrides 'ImmutableState::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'SafeCallback' overrides 'ImmutableState::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'SafeCallback' overrides a base function but is not marked 'override' |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
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
| warning | W200 | function 'poolId' in 'SafeCallback' overrides 'ImmutableState::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'SafeCallback' overrides 'ImmutableState::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'SafeCallback' overrides 'ImmutableState::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'SafeCallback' overrides 'ImmutableState::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'SafeCallback' overrides 'ImmutableState::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'SafeCallback' overrides 'ImmutableState::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'SafeCallback' overrides 'ImmutableState::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'SafeCallback' overrides 'ImmutableState::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'SafeCallback' overrides 'ImmutableState::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'SafeCallback' overrides 'ImmutableState::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'SafeCallback' overrides 'ImmutableState::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'SafeCallback' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'SafeCallback' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'validateMinOut' in 'SafeCallback' overrides 'ImmutableState::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'SafeCallback' overrides 'ImmutableState::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'SafeCallback' overrides 'ImmutableState::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'SafeCallback' overrides 'ImmutableState::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'SafeCallback' overrides 'ImmutableState::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'SafeCallback' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'mulDiv' in 'BaseActionsRouter' overrides 'SafeCallback::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'BaseActionsRouter' overrides 'SafeCallback::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'BaseActionsRouter' overrides 'SafeCallback::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'BaseActionsRouter' overrides 'SafeCallback::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'BaseActionsRouter' overrides 'SafeCallback::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'BaseActionsRouter' overrides 'SafeCallback::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'BaseActionsRouter' overrides 'SafeCallback::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'BaseActionsRouter' overrides 'SafeCallback::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'BaseActionsRouter' overrides 'SafeCallback::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'BaseActionsRouter' overrides 'SafeCallback::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'BaseActionsRouter' overrides 'SafeCallback::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'BaseActionsRouter' overrides 'SafeCallback::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'BaseActionsRouter' overrides 'SafeCallback::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'BaseActionsRouter' overrides 'SafeCallback::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'BaseActionsRouter' overrides 'SafeCallback::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'BaseActionsRouter' overrides 'SafeCallback::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'BaseActionsRouter' overrides 'SafeCallback::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'BaseActionsRouter' overrides 'SafeCallback::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'BaseActionsRouter' overrides 'SafeCallback::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'BaseActionsRouter' overrides 'SafeCallback::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'poolId' in 'BaseActionsRouter' overrides 'SafeCallback::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'BaseActionsRouter' overrides 'SafeCallback::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'BaseActionsRouter' overrides 'SafeCallback::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'BaseActionsRouter' overrides 'SafeCallback::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'BaseActionsRouter' overrides 'SafeCallback::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'BaseActionsRouter' overrides 'SafeCallback::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'BaseActionsRouter' overrides 'SafeCallback::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'BaseActionsRouter' overrides 'SafeCallback::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'BaseActionsRouter' overrides 'SafeCallback::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'BaseActionsRouter' overrides 'SafeCallback::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'BaseActionsRouter' overrides 'SafeCallback::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'BaseActionsRouter' overrides 'SafeCallback::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'validateMinOut' in 'BaseActionsRouter' overrides 'SafeCallback::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'BaseActionsRouter' overrides 'SafeCallback::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'BaseActionsRouter' overrides 'SafeCallback::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'BaseActionsRouter' overrides 'SafeCallback::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'BaseActionsRouter' overrides 'SafeCallback::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'BaseActionsRouter' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'subscribe' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'unsubscribe' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | W116 | function 'subscribe' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'unsubscribe' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | VALIDATION_WARNING | 'using X for *' is supported; all library functions are available but type-specific filtering is not enforced at compile time |
| warning | VALIDATION_WARNING | abstract contract 'Notifier' has 2 unimplemented function(s): [_setUnsubscribed, _setSubscribed] |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Notifier' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'permit' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'permitBatch' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W121 | duplicate constant state variable 'ZERO_DELTA' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | W116 | function 'permit' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'permitBatch' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Permit2Forwarder' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'onNEP17Payment' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'NativeWrapper' overrides 'ImmutableState::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'NativeWrapper' overrides 'ImmutableState::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'NativeWrapper' overrides 'ImmutableState::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'NativeWrapper' overrides 'ImmutableState::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'NativeWrapper' overrides 'ImmutableState::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'NativeWrapper' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'NativeWrapper' overrides 'ImmutableState::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'NativeWrapper' overrides 'ImmutableState::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'NativeWrapper' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'NativeWrapper' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'NativeWrapper' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'NativeWrapper' overrides 'ImmutableState::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'NativeWrapper' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'NativeWrapper' overrides 'ImmutableState::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'NativeWrapper' overrides 'ImmutableState::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'NativeWrapper' overrides 'ImmutableState::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'NativeWrapper' overrides 'ImmutableState::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'NativeWrapper' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'NativeWrapper' overrides 'ImmutableState::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'NativeWrapper' overrides 'ImmutableState::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'NativeWrapper' overrides 'ImmutableState::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'NativeWrapper' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'NativeWrapper' overrides 'ImmutableState::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'NativeWrapper' overrides 'ImmutableState::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'NativeWrapper' overrides 'ImmutableState::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'NativeWrapper' overrides 'ImmutableState::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'NativeWrapper' overrides 'ImmutableState::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'NativeWrapper' overrides 'ImmutableState::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'NativeWrapper' overrides 'ImmutableState::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'NativeWrapper' overrides 'ImmutableState::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'NativeWrapper' overrides 'ImmutableState::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'NativeWrapper' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'NativeWrapper' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'NativeWrapper' overrides 'ImmutableState::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'NativeWrapper' overrides 'ImmutableState::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'NativeWrapper' overrides 'ImmutableState::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'NativeWrapper' overrides 'ImmutableState::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'NativeWrapper' overrides 'ImmutableState::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'NativeWrapper' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'NativeWrapper' overrides 'ImmutableState::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'NativeWrapper' overrides 'ImmutableState::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'NativeWrapper' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'NativeWrapper' overrides 'ImmutableState::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'NativeWrapper' overrides 'ImmutableState::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'NativeWrapper' overrides 'ImmutableState::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'NativeWrapper' overrides 'ImmutableState::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'NativeWrapper' overrides 'ImmutableState::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'NativeWrapper' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'NativeWrapper' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'NativeWrapper' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'NativeWrapper' overrides 'ImmutableState::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'NativeWrapper' overrides 'ImmutableState::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'NativeWrapper' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'NativeWrapper' overrides 'ImmutableState::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'NativeWrapper' overrides 'ImmutableState::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'NativeWrapper' overrides 'ImmutableState::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'NativeWrapper' overrides 'ImmutableState::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'NativeWrapper' overrides 'ImmutableState::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'NativeWrapper' overrides 'ImmutableState::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'NativeWrapper' overrides 'ImmutableState::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'NativeWrapper' overrides 'ImmutableState::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'NativeWrapper' overrides 'ImmutableState::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'NativeWrapper' overrides 'ImmutableState::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'NativeWrapper' overrides 'ImmutableState::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'NativeWrapper' overrides 'ImmutableState::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'NativeWrapper' overrides 'ImmutableState::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'NativeWrapper' overrides 'ImmutableState::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'NativeWrapper' overrides 'ImmutableState::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'NativeWrapper' overrides 'ImmutableState::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'NativeWrapper' overrides 'ImmutableState::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'NativeWrapper' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'NativeWrapper' overrides 'ImmutableState::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'NativeWrapper' overrides 'ImmutableState::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'NativeWrapper' overrides 'ImmutableState::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'NativeWrapper' overrides 'ImmutableState::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'NativeWrapper' overrides 'ImmutableState::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'NativeWrapper' overrides 'ImmutableState::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'NativeWrapper' overrides 'ImmutableState::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'NativeWrapper' overrides 'ImmutableState::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'NativeWrapper' overrides 'ImmutableState::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'NativeWrapper' overrides 'ImmutableState::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'NativeWrapper' overrides 'ImmutableState::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'NativeWrapper' overrides 'ImmutableState::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'NativeWrapper' overrides 'ImmutableState::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'NativeWrapper' overrides 'ImmutableState::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'NativeWrapper' overrides 'ImmutableState::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'NativeWrapper' overrides 'ImmutableState::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'NativeWrapper' overrides 'ImmutableState::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'NativeWrapper' overrides 'ImmutableState::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'NativeWrapper' overrides 'ImmutableState::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'NativeWrapper' overrides 'ImmutableState::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'NativeWrapper' overrides 'ImmutableState::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'NativeWrapper' overrides 'ImmutableState::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'NativeWrapper' overrides 'ImmutableState::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'NativeWrapper' overrides 'ImmutableState::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'NativeWrapper' overrides 'ImmutableState::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'NativeWrapper' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W111 | function 'revokeNonce' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'permit' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'permitForAll' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'initializePool' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'multicall' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'subscribe' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'unsubscribe' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'permit' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'permitBatch' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'onNEP17Payment' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'modifyLiquidities' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'modifyLiquiditiesWithoutUnlock' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESERVES_OF_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CURRENCY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'NONZERO_DELTA_COUNT_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'IS_UNLOCKED_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_TICK_SPACING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MIN_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MAX_SQRT_PRICE_MINUS_MIN_SQRT_PRICE_MINUS_ONE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_POSITION_INFO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_UPPER_200_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_8_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MASK_24_BITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_UNSUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SET_SUBSCRIBE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_LOWER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'TICK_UPPER_OFFSET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PERMIT_FOR_ALL_TYPEHASH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UPPER_BIT_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LOCKED_BY_SLOT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPEN_DELTA' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'CONTRACT_BALANCE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'MSG_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_THIS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OFFSET_OR_LENGTH_MASK_AND_WORD_ALIGN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SLICE_ERROR_SELECTOR' detected while merging libraries |
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
| warning | W121 | duplicate constant state variable 'RESOLUTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'Q96' detected while merging libraries |
| warning | INVALID_STORAGE_RETURN | function 'get' return value 'State' uses 'storage' data location (treated as Any) |
| warning | RETURN_TYPE_UNMAPPED | function 'positionInfo' returns 'Any', which may not map cleanly to Neo manifest types |
| warning | RETURN_TYPE_UNMAPPED | function 'poolKeys' returns 'Any', which may not map cleanly to Neo manifest types |
| warning | W104 | function 'transferFrom' with 3 parameters (ERC-721 pattern) detected. NEP-11 uses transfer(to, tokenId, data) with 3 parameters instead. Authorization is via Runtime.checkWitness(owner), not msg.sender. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W114 | NFT contract (has ownerOf) but missing onNEP11Payment callback. Other contracts cannot send NFTs to this contract. |
| warning | W116 | function 'revokeNonce' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'permit' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'permitForAll' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'initializePool' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'multicall' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'subscribe' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'unsubscribe' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'permit' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'permitBatch' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'modifyLiquidities' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'modifyLiquiditiesWithoutUnlock' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides 'ERC721::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'EIP712_v4' overrides 'ERC721::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'EIP712_v4' overrides 'ERC721::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'EIP712_v4' overrides 'ERC721::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'EIP712_v4' overrides 'ERC721::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'EIP712_v4' overrides 'ERC721::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'EIP712_v4' overrides 'ERC721::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'EIP712_v4' overrides 'ERC721::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'EIP712_v4' overrides 'ERC721::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'EIP712_v4' overrides 'ERC721::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'EIP712_v4' overrides 'ERC721::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'EIP712_v4' overrides 'ERC721::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'EIP712_v4' overrides 'ERC721::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'EIP712_v4' overrides 'ERC721::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'EIP712_v4' overrides 'ERC721::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'EIP712_v4' overrides 'ERC721::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'EIP712_v4' overrides 'ERC721::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'EIP712_v4' overrides 'ERC721::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'EIP712_v4' overrides 'ERC721::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'EIP712_v4' overrides 'ERC721::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'EIP712_v4' overrides 'ERC721::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'EIP712_v4' overrides 'ERC721::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'EIP712_v4' overrides 'ERC721::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'EIP712_v4' overrides 'ERC721::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'EIP712_v4' overrides 'ERC721::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'EIP712_v4' overrides 'ERC721::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'EIP712_v4' overrides 'ERC721::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'EIP712_v4' overrides 'ERC721::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'EIP712_v4' overrides 'ERC721::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'EIP712_v4' overrides 'ERC721::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'EIP712_v4' overrides 'ERC721::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'EIP712_v4' overrides 'ERC721::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'EIP712_v4' overrides 'ERC721::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'EIP712_v4' overrides 'ERC721::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'EIP712_v4' overrides 'ERC721::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'EIP712_v4' overrides 'ERC721::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'EIP712_v4' overrides 'ERC721::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'EIP712_v4' overrides 'ERC721::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'EIP712_v4' overrides 'ERC721::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'EIP712_v4' overrides 'ERC721::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'EIP712_v4' overrides 'ERC721::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'EIP712_v4' overrides 'ERC721::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'EIP712_v4' overrides 'ERC721::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'EIP712_v4' overrides 'ERC721::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'EIP712_v4' overrides 'ERC721::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'EIP712_v4' overrides 'ERC721::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'EIP712_v4' overrides 'ERC721::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'EIP712_v4' overrides 'ERC721::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'EIP712_v4' overrides 'ERC721::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'EIP712_v4' overrides 'ERC721::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'EIP712_v4' overrides 'ERC721::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'EIP712_v4' overrides 'ERC721::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'EIP712_v4' overrides 'ERC721::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'EIP712_v4' overrides 'ERC721::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'EIP712_v4' overrides 'ERC721::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'EIP712_v4' overrides 'ERC721::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'EIP712_v4' overrides 'ERC721::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'EIP712_v4' overrides 'ERC721::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'EIP712_v4' overrides 'ERC721::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'EIP712_v4' overrides 'ERC721::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'EIP712_v4' overrides 'ERC721::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'EIP712_v4' overrides 'ERC721::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'EIP712_v4' overrides 'ERC721::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'EIP712_v4' overrides 'ERC721::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'EIP712_v4' overrides 'ERC721::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'EIP712_v4' overrides 'ERC721::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'EIP712_v4' overrides 'ERC721::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'EIP712_v4' overrides 'ERC721::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'EIP712_v4' overrides 'ERC721::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'EIP712_v4' overrides 'ERC721::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'EIP712_v4' overrides 'ERC721::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'EIP712_v4' overrides 'ERC721::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'EIP712_v4' overrides 'ERC721::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'EIP712_v4' overrides 'ERC721::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'EIP712_v4' overrides 'ERC721::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'EIP712_v4' overrides 'ERC721::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'EIP712_v4' overrides 'ERC721::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'EIP712_v4' overrides 'ERC721::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'EIP712_v4' overrides 'ERC721::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'EIP712_v4' overrides 'ERC721::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'EIP712_v4' overrides 'ERC721::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'EIP712_v4' overrides 'ERC721::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'EIP712_v4' overrides 'ERC721::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'EIP712_v4' overrides 'ERC721::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'EIP712_v4' overrides 'ERC721::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'EIP712_v4' overrides 'ERC721::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'EIP712_v4' overrides 'ERC721::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'EIP712_v4' overrides 'ERC721::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'EIP712_v4' overrides 'ERC721::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'EIP712_v4' overrides 'ERC721::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'EIP712_v4' overrides 'ERC721::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'EIP712_v4' overrides 'ERC721::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'EIP712_v4' overrides 'ERC721::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'EIP712_v4' overrides 'ERC721::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'EIP712_v4' overrides 'ERC721::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'EIP712_v4' overrides 'ERC721::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'EIP712_v4' overrides 'ERC721::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'EIP712_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides 'EIP712_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'UnorderedNonce' overrides 'EIP712_v4::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'UnorderedNonce' overrides 'EIP712_v4::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'UnorderedNonce' overrides 'EIP712_v4::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'UnorderedNonce' overrides 'EIP712_v4::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'UnorderedNonce' overrides 'EIP712_v4::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'UnorderedNonce' overrides 'EIP712_v4::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'UnorderedNonce' overrides 'EIP712_v4::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'UnorderedNonce' overrides 'EIP712_v4::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'UnorderedNonce' overrides 'EIP712_v4::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'UnorderedNonce' overrides 'EIP712_v4::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'UnorderedNonce' overrides 'EIP712_v4::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'UnorderedNonce' overrides 'EIP712_v4::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'UnorderedNonce' overrides 'EIP712_v4::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'UnorderedNonce' overrides 'EIP712_v4::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'UnorderedNonce' overrides 'EIP712_v4::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'UnorderedNonce' overrides 'EIP712_v4::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'UnorderedNonce' overrides 'EIP712_v4::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'UnorderedNonce' overrides 'EIP712_v4::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'UnorderedNonce' overrides 'EIP712_v4::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'UnorderedNonce' overrides 'EIP712_v4::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'UnorderedNonce' overrides 'EIP712_v4::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'UnorderedNonce' overrides 'EIP712_v4::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'UnorderedNonce' overrides 'EIP712_v4::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'UnorderedNonce' overrides 'EIP712_v4::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'UnorderedNonce' overrides 'EIP712_v4::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'UnorderedNonce' overrides 'EIP712_v4::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'UnorderedNonce' overrides 'EIP712_v4::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'UnorderedNonce' overrides 'EIP712_v4::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'UnorderedNonce' overrides 'EIP712_v4::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'UnorderedNonce' overrides 'EIP712_v4::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'UnorderedNonce' overrides 'EIP712_v4::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'UnorderedNonce' overrides 'EIP712_v4::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'UnorderedNonce' overrides 'EIP712_v4::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'UnorderedNonce' overrides 'EIP712_v4::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'UnorderedNonce' overrides 'EIP712_v4::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'UnorderedNonce' overrides 'EIP712_v4::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'UnorderedNonce' overrides 'EIP712_v4::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'UnorderedNonce' overrides 'EIP712_v4::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'UnorderedNonce' overrides 'EIP712_v4::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'UnorderedNonce' overrides 'EIP712_v4::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'UnorderedNonce' overrides 'EIP712_v4::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'UnorderedNonce' overrides 'EIP712_v4::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'UnorderedNonce' overrides 'EIP712_v4::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'UnorderedNonce' overrides 'EIP712_v4::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'UnorderedNonce' overrides 'EIP712_v4::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'UnorderedNonce' overrides 'EIP712_v4::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'UnorderedNonce' overrides 'EIP712_v4::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'UnorderedNonce' overrides 'EIP712_v4::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'UnorderedNonce' overrides 'EIP712_v4::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'UnorderedNonce' overrides 'EIP712_v4::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'UnorderedNonce' overrides 'EIP712_v4::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'UnorderedNonce' overrides 'EIP712_v4::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'UnorderedNonce' overrides 'EIP712_v4::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'UnorderedNonce' overrides 'EIP712_v4::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'UnorderedNonce' overrides 'EIP712_v4::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'UnorderedNonce' overrides 'EIP712_v4::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'UnorderedNonce' overrides 'EIP712_v4::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'UnorderedNonce' overrides 'EIP712_v4::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'UnorderedNonce' overrides 'EIP712_v4::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'UnorderedNonce' overrides 'EIP712_v4::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'UnorderedNonce' overrides 'EIP712_v4::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'UnorderedNonce' overrides 'EIP712_v4::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'UnorderedNonce' overrides 'EIP712_v4::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'UnorderedNonce' overrides 'EIP712_v4::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'UnorderedNonce' overrides 'EIP712_v4::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'UnorderedNonce' overrides 'EIP712_v4::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'UnorderedNonce' overrides 'EIP712_v4::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'UnorderedNonce' overrides 'EIP712_v4::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'UnorderedNonce' overrides 'EIP712_v4::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'UnorderedNonce' overrides 'EIP712_v4::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'UnorderedNonce' overrides 'EIP712_v4::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'UnorderedNonce' overrides 'EIP712_v4::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'UnorderedNonce' overrides 'EIP712_v4::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'UnorderedNonce' overrides 'EIP712_v4::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'UnorderedNonce' overrides 'EIP712_v4::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'UnorderedNonce' overrides 'EIP712_v4::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'UnorderedNonce' overrides 'EIP712_v4::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'UnorderedNonce' overrides 'EIP712_v4::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'UnorderedNonce' overrides 'EIP712_v4::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'UnorderedNonce' overrides 'EIP712_v4::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'UnorderedNonce' overrides 'EIP712_v4::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'UnorderedNonce' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ERC721Permit_v4' overrides 'UnorderedNonce::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'ERC721Permit_v4' overrides 'UnorderedNonce::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'ERC721Permit_v4' overrides 'UnorderedNonce::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'ERC721Permit_v4' overrides 'UnorderedNonce::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'ERC721Permit_v4' overrides 'UnorderedNonce::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'ERC721Permit_v4' overrides 'UnorderedNonce::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'ERC721Permit_v4' overrides 'UnorderedNonce::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'ERC721Permit_v4' overrides 'UnorderedNonce::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC721Permit_v4' overrides 'UnorderedNonce::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'ERC721Permit_v4' overrides 'UnorderedNonce::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'ERC721Permit_v4' overrides 'UnorderedNonce::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ERC721Permit_v4' overrides 'UnorderedNonce::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'ERC721Permit_v4' overrides 'UnorderedNonce::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'ERC721Permit_v4' overrides 'UnorderedNonce::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'ERC721Permit_v4' overrides 'UnorderedNonce::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'ERC721Permit_v4' overrides 'UnorderedNonce::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'ERC721Permit_v4' overrides 'UnorderedNonce::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'ERC721Permit_v4' overrides 'UnorderedNonce::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ERC721Permit_v4' overrides 'UnorderedNonce::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'ERC721Permit_v4' overrides 'UnorderedNonce::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'ERC721Permit_v4' overrides 'UnorderedNonce::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'ERC721Permit_v4' overrides 'UnorderedNonce::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'ERC721Permit_v4' overrides 'UnorderedNonce::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'ERC721Permit_v4' overrides 'UnorderedNonce::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'ERC721Permit_v4' overrides 'UnorderedNonce::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'ERC721Permit_v4' overrides 'UnorderedNonce::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'ERC721Permit_v4' overrides 'UnorderedNonce::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'ERC721Permit_v4' overrides 'UnorderedNonce::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'ERC721Permit_v4' overrides 'UnorderedNonce::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'ERC721Permit_v4' overrides 'UnorderedNonce::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'ERC721Permit_v4' overrides 'UnorderedNonce::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'ERC721Permit_v4' overrides 'UnorderedNonce::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'ERC721Permit_v4' overrides 'UnorderedNonce::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'ERC721Permit_v4' overrides 'UnorderedNonce::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'ERC721Permit_v4' overrides 'UnorderedNonce::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'ERC721Permit_v4' overrides 'UnorderedNonce::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'ERC721Permit_v4' overrides 'UnorderedNonce::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'ERC721Permit_v4' overrides 'UnorderedNonce::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'ERC721Permit_v4' overrides 'UnorderedNonce::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'ERC721Permit_v4' overrides 'UnorderedNonce::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'ERC721Permit_v4' overrides 'UnorderedNonce::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ERC721Permit_v4' overrides 'UnorderedNonce::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'ERC721Permit_v4' overrides 'UnorderedNonce::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'ERC721Permit_v4' overrides 'UnorderedNonce::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'ERC721Permit_v4' overrides 'UnorderedNonce::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'ERC721Permit_v4' overrides 'UnorderedNonce::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'ERC721Permit_v4' overrides 'UnorderedNonce::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'ERC721Permit_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides 'ERC721Permit_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides 'ERC721Permit_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides 'ERC721Permit_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides 'ERC721Permit_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides 'ERC721Permit_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides 'ERC721Permit_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides 'ERC721Permit_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ImmutableState' overrides 'ERC721Permit_v4::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'ImmutableState' overrides 'ERC721Permit_v4::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'ImmutableState' overrides 'ERC721Permit_v4::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'ImmutableState' overrides 'ERC721Permit_v4::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'ImmutableState' overrides 'ERC721Permit_v4::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ImmutableState' overrides 'ERC721Permit_v4::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'ImmutableState' overrides 'ERC721Permit_v4::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ImmutableState' overrides 'ERC721Permit_v4::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ImmutableState' overrides 'ERC721Permit_v4::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ImmutableState' overrides 'ERC721Permit_v4::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ImmutableState' overrides 'ERC721Permit_v4::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ImmutableState' overrides 'ERC721Permit_v4::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ImmutableState' overrides 'ERC721Permit_v4::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'ImmutableState' overrides 'ERC721Permit_v4::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'ImmutableState' overrides 'ERC721Permit_v4::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'ImmutableState' overrides 'ERC721Permit_v4::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ImmutableState' overrides 'ERC721Permit_v4::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ImmutableState' overrides 'ERC721Permit_v4::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ImmutableState' overrides 'ERC721Permit_v4::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'ImmutableState' overrides 'ERC721Permit_v4::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'ImmutableState' overrides 'ERC721Permit_v4::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ImmutableState' overrides 'ERC721Permit_v4::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'ImmutableState' overrides 'ERC721Permit_v4::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'ImmutableState' overrides 'ERC721Permit_v4::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'ImmutableState' overrides 'ERC721Permit_v4::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'ImmutableState' overrides 'ERC721Permit_v4::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'ImmutableState' overrides 'ERC721Permit_v4::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'ImmutableState' overrides 'ERC721Permit_v4::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'ImmutableState' overrides 'ERC721Permit_v4::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'ImmutableState' overrides 'ERC721Permit_v4::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'ImmutableState' overrides 'ERC721Permit_v4::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'ImmutableState' overrides 'ERC721Permit_v4::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'ImmutableState' overrides 'ERC721Permit_v4::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'ImmutableState' overrides 'ERC721Permit_v4::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'ImmutableState' overrides 'ERC721Permit_v4::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'ImmutableState' overrides 'ERC721Permit_v4::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'ImmutableState' overrides 'ERC721Permit_v4::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'ImmutableState' overrides 'ERC721Permit_v4::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'ImmutableState' overrides 'ERC721Permit_v4::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'ImmutableState' overrides 'ERC721Permit_v4::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ImmutableState' overrides 'ERC721Permit_v4::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ImmutableState' overrides 'ERC721Permit_v4::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'ImmutableState' overrides 'ERC721Permit_v4::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'ImmutableState' overrides 'ERC721Permit_v4::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'ImmutableState' overrides 'ERC721Permit_v4::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'ImmutableState' overrides 'ERC721Permit_v4::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'ImmutableState' overrides 'ERC721Permit_v4::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'ImmutableState' overrides 'ERC721Permit_v4::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ImmutableState' overrides 'ERC721Permit_v4::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'ImmutableState' overrides 'ERC721Permit_v4::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'ImmutableState' overrides 'ERC721Permit_v4::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'ImmutableState' overrides 'ERC721Permit_v4::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'ImmutableState' overrides 'ERC721Permit_v4::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'ImmutableState' overrides 'ERC721Permit_v4::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'ImmutableState' overrides 'ERC721Permit_v4::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'ImmutableState' overrides 'ERC721Permit_v4::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'ImmutableState' overrides 'ERC721Permit_v4::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ImmutableState' overrides 'ERC721Permit_v4::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ImmutableState' overrides 'ERC721Permit_v4::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'ImmutableState' overrides 'ERC721Permit_v4::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'ImmutableState' overrides 'ERC721Permit_v4::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'ImmutableState' overrides 'ERC721Permit_v4::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'ImmutableState' overrides 'ERC721Permit_v4::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'ImmutableState' overrides 'ERC721Permit_v4::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'ImmutableState' overrides 'ERC721Permit_v4::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'ImmutableState' overrides 'ERC721Permit_v4::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'ImmutableState' overrides 'ERC721Permit_v4::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'ImmutableState' overrides 'ERC721Permit_v4::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'ImmutableState' overrides 'ERC721Permit_v4::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'ImmutableState' overrides 'ERC721Permit_v4::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ImmutableState' overrides 'ERC721Permit_v4::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'ImmutableState' overrides 'ERC721Permit_v4::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'ImmutableState' overrides 'ERC721Permit_v4::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'ImmutableState' overrides 'ERC721Permit_v4::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'ImmutableState' overrides 'ERC721Permit_v4::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'ImmutableState' overrides 'ERC721Permit_v4::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'ImmutableState' overrides 'ERC721Permit_v4::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'ImmutableState' overrides 'ERC721Permit_v4::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'ImmutableState' overrides 'ERC721Permit_v4::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'ImmutableState' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'PoolInitializer_v4' overrides 'ImmutableState::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'PoolInitializer_v4' overrides 'ImmutableState::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'PoolInitializer_v4' overrides 'ImmutableState::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'PoolInitializer_v4' overrides 'ImmutableState::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'PoolInitializer_v4' overrides 'ImmutableState::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'PoolInitializer_v4' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'PoolInitializer_v4' overrides 'ImmutableState::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'PoolInitializer_v4' overrides 'ImmutableState::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'PoolInitializer_v4' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'PoolInitializer_v4' overrides 'ImmutableState::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'PoolInitializer_v4' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'PoolInitializer_v4' overrides 'ImmutableState::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'PoolInitializer_v4' overrides 'ImmutableState::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'PoolInitializer_v4' overrides 'ImmutableState::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'PoolInitializer_v4' overrides 'ImmutableState::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'PoolInitializer_v4' overrides 'ImmutableState::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'PoolInitializer_v4' overrides 'ImmutableState::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'PoolInitializer_v4' overrides 'ImmutableState::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'PoolInitializer_v4' overrides 'ImmutableState::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'PoolInitializer_v4' overrides 'ImmutableState::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'PoolInitializer_v4' overrides 'ImmutableState::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'PoolInitializer_v4' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'PoolInitializer_v4' overrides 'ImmutableState::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'PoolInitializer_v4' overrides 'ImmutableState::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'PoolInitializer_v4' overrides 'ImmutableState::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'PoolInitializer_v4' overrides 'ImmutableState::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'PoolInitializer_v4' overrides 'ImmutableState::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'PoolInitializer_v4' overrides 'ImmutableState::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'PoolInitializer_v4' overrides 'ImmutableState::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'PoolInitializer_v4' overrides 'ImmutableState::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'PoolInitializer_v4' overrides 'ImmutableState::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'PoolInitializer_v4' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'PoolInitializer_v4' overrides 'ImmutableState::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'PoolInitializer_v4' overrides 'ImmutableState::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'PoolInitializer_v4' overrides 'ImmutableState::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'PoolInitializer_v4' overrides 'ImmutableState::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'PoolInitializer_v4' overrides 'ImmutableState::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'PoolInitializer_v4' overrides 'ImmutableState::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'PoolInitializer_v4' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'PoolInitializer_v4' overrides 'ImmutableState::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'PoolInitializer_v4' overrides 'ImmutableState::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'PoolInitializer_v4' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'PoolInitializer_v4' overrides 'ImmutableState::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'PoolInitializer_v4' overrides 'ImmutableState::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'PoolInitializer_v4' overrides 'ImmutableState::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'PoolInitializer_v4' overrides 'ImmutableState::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'PoolInitializer_v4' overrides 'ImmutableState::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'PoolInitializer_v4' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'PoolInitializer_v4' overrides 'ImmutableState::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'PoolInitializer_v4' overrides 'ImmutableState::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'PoolInitializer_v4' overrides 'ImmutableState::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'PoolInitializer_v4' overrides 'ImmutableState::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'PoolInitializer_v4' overrides 'ImmutableState::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'PoolInitializer_v4' overrides 'ImmutableState::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'PoolInitializer_v4' overrides 'ImmutableState::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'PoolInitializer_v4' overrides 'ImmutableState::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'PoolInitializer_v4' overrides 'ImmutableState::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'PoolInitializer_v4' overrides 'ImmutableState::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'PoolInitializer_v4' overrides 'ImmutableState::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'PoolInitializer_v4' overrides 'ImmutableState::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'PoolInitializer_v4' overrides 'ImmutableState::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'PoolInitializer_v4' overrides 'ImmutableState::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'PoolInitializer_v4' overrides 'ImmutableState::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'PoolInitializer_v4' overrides 'ImmutableState::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'PoolInitializer_v4' overrides 'ImmutableState::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'PoolInitializer_v4' overrides 'ImmutableState::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'PoolInitializer_v4' overrides 'ImmutableState::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'PoolInitializer_v4' overrides 'ImmutableState::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'PoolInitializer_v4' overrides 'ImmutableState::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'PoolInitializer_v4' overrides 'ImmutableState::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'PoolInitializer_v4' overrides 'ImmutableState::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'PoolInitializer_v4' overrides 'ImmutableState::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'PoolInitializer_v4' overrides 'ImmutableState::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'PoolInitializer_v4' overrides 'ImmutableState::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'PoolInitializer_v4' overrides 'ImmutableState::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'PoolInitializer_v4' overrides 'ImmutableState::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'PoolInitializer_v4' overrides 'ImmutableState::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'PoolInitializer_v4' overrides 'ImmutableState::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'PoolInitializer_v4' overrides 'ImmutableState::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'PoolInitializer_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides 'PoolInitializer_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides 'PoolInitializer_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides 'PoolInitializer_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides 'PoolInitializer_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides 'PoolInitializer_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides 'PoolInitializer_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides 'PoolInitializer_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'Multicall_v4' overrides 'PoolInitializer_v4::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'Multicall_v4' overrides 'PoolInitializer_v4::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'Multicall_v4' overrides 'PoolInitializer_v4::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'Multicall_v4' overrides 'PoolInitializer_v4::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'Multicall_v4' overrides 'PoolInitializer_v4::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'Multicall_v4' overrides 'PoolInitializer_v4::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'Multicall_v4' overrides 'PoolInitializer_v4::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Multicall_v4' overrides 'PoolInitializer_v4::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Multicall_v4' overrides 'PoolInitializer_v4::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Multicall_v4' overrides 'PoolInitializer_v4::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Multicall_v4' overrides 'PoolInitializer_v4::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Multicall_v4' overrides 'PoolInitializer_v4::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Multicall_v4' overrides 'PoolInitializer_v4::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'Multicall_v4' overrides 'PoolInitializer_v4::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'Multicall_v4' overrides 'PoolInitializer_v4::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'Multicall_v4' overrides 'PoolInitializer_v4::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'Multicall_v4' overrides 'PoolInitializer_v4::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'Multicall_v4' overrides 'PoolInitializer_v4::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Multicall_v4' overrides 'PoolInitializer_v4::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'Multicall_v4' overrides 'PoolInitializer_v4::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'Multicall_v4' overrides 'PoolInitializer_v4::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'Multicall_v4' overrides 'PoolInitializer_v4::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'Multicall_v4' overrides 'PoolInitializer_v4::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'Multicall_v4' overrides 'PoolInitializer_v4::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'Multicall_v4' overrides 'PoolInitializer_v4::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'Multicall_v4' overrides 'PoolInitializer_v4::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'Multicall_v4' overrides 'PoolInitializer_v4::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'Multicall_v4' overrides 'PoolInitializer_v4::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'Multicall_v4' overrides 'PoolInitializer_v4::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'Multicall_v4' overrides 'PoolInitializer_v4::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'Multicall_v4' overrides 'PoolInitializer_v4::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'Multicall_v4' overrides 'PoolInitializer_v4::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'Multicall_v4' overrides 'PoolInitializer_v4::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'Multicall_v4' overrides 'PoolInitializer_v4::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'Multicall_v4' overrides 'PoolInitializer_v4::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'Multicall_v4' overrides 'PoolInitializer_v4::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'Multicall_v4' overrides 'PoolInitializer_v4::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'Multicall_v4' overrides 'PoolInitializer_v4::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'Multicall_v4' overrides 'PoolInitializer_v4::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'Multicall_v4' overrides 'PoolInitializer_v4::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'Multicall_v4' overrides 'PoolInitializer_v4::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'Multicall_v4' overrides 'PoolInitializer_v4::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'Multicall_v4' overrides 'PoolInitializer_v4::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'Multicall_v4' overrides 'PoolInitializer_v4::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'Multicall_v4' overrides 'PoolInitializer_v4::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'Multicall_v4' overrides 'PoolInitializer_v4::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'Multicall_v4' overrides 'PoolInitializer_v4::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'Multicall_v4' overrides 'PoolInitializer_v4::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'Multicall_v4' overrides 'PoolInitializer_v4::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'Multicall_v4' overrides 'PoolInitializer_v4::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'Multicall_v4' overrides 'PoolInitializer_v4::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'Multicall_v4' overrides 'PoolInitializer_v4::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'Multicall_v4' overrides 'PoolInitializer_v4::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'Multicall_v4' overrides 'PoolInitializer_v4::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'Multicall_v4' overrides 'PoolInitializer_v4::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'Multicall_v4' overrides 'PoolInitializer_v4::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'Multicall_v4' overrides 'PoolInitializer_v4::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'Multicall_v4' overrides 'PoolInitializer_v4::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'Multicall_v4' overrides 'PoolInitializer_v4::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'Multicall_v4' overrides 'PoolInitializer_v4::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'Multicall_v4' overrides 'PoolInitializer_v4::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'Multicall_v4' overrides 'PoolInitializer_v4::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'Multicall_v4' overrides 'PoolInitializer_v4::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'Multicall_v4' overrides 'PoolInitializer_v4::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'Multicall_v4' overrides 'PoolInitializer_v4::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'Multicall_v4' overrides 'PoolInitializer_v4::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'Multicall_v4' overrides 'PoolInitializer_v4::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'Multicall_v4' overrides 'PoolInitializer_v4::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'Multicall_v4' overrides 'PoolInitializer_v4::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'Multicall_v4' overrides 'PoolInitializer_v4::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'Multicall_v4' overrides 'PoolInitializer_v4::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'Multicall_v4' overrides 'PoolInitializer_v4::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'Multicall_v4' overrides 'PoolInitializer_v4::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'Multicall_v4' overrides 'PoolInitializer_v4::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'Multicall_v4' overrides 'PoolInitializer_v4::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'Multicall_v4' overrides 'PoolInitializer_v4::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'Multicall_v4' overrides 'PoolInitializer_v4::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'Multicall_v4' overrides 'PoolInitializer_v4::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'Multicall_v4' overrides 'PoolInitializer_v4::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'Multicall_v4' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'Multicall_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'Multicall_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'Multicall_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'Multicall_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'Multicall_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'Multicall_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides 'Multicall_v4::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'DeltaResolver' overrides 'Multicall_v4::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'DeltaResolver' overrides 'Multicall_v4::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'DeltaResolver' overrides 'Multicall_v4::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'DeltaResolver' overrides 'Multicall_v4::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'DeltaResolver' overrides 'Multicall_v4::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'DeltaResolver' overrides 'Multicall_v4::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'DeltaResolver' overrides 'Multicall_v4::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'DeltaResolver' overrides 'Multicall_v4::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'DeltaResolver' overrides 'Multicall_v4::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'DeltaResolver' overrides 'Multicall_v4::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'DeltaResolver' overrides 'Multicall_v4::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'DeltaResolver' overrides 'Multicall_v4::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'DeltaResolver' overrides 'Multicall_v4::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'DeltaResolver' overrides 'Multicall_v4::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'DeltaResolver' overrides 'Multicall_v4::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'DeltaResolver' overrides 'Multicall_v4::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'DeltaResolver' overrides 'Multicall_v4::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'DeltaResolver' overrides 'Multicall_v4::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'DeltaResolver' overrides 'Multicall_v4::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'DeltaResolver' overrides 'Multicall_v4::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'DeltaResolver' overrides 'Multicall_v4::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'DeltaResolver' overrides 'Multicall_v4::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'DeltaResolver' overrides 'Multicall_v4::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'DeltaResolver' overrides 'Multicall_v4::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'DeltaResolver' overrides 'Multicall_v4::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'DeltaResolver' overrides 'Multicall_v4::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'DeltaResolver' overrides 'Multicall_v4::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'DeltaResolver' overrides 'Multicall_v4::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'DeltaResolver' overrides 'Multicall_v4::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'DeltaResolver' overrides 'Multicall_v4::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'DeltaResolver' overrides 'Multicall_v4::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'DeltaResolver' overrides 'Multicall_v4::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'DeltaResolver' overrides 'Multicall_v4::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'DeltaResolver' overrides 'Multicall_v4::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'DeltaResolver' overrides 'Multicall_v4::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'DeltaResolver' overrides 'Multicall_v4::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'DeltaResolver' overrides 'Multicall_v4::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'DeltaResolver' overrides 'Multicall_v4::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'DeltaResolver' overrides 'Multicall_v4::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'DeltaResolver' overrides 'Multicall_v4::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'DeltaResolver' overrides 'Multicall_v4::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'DeltaResolver' overrides 'Multicall_v4::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'DeltaResolver' overrides 'Multicall_v4::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'DeltaResolver' overrides 'Multicall_v4::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'DeltaResolver' overrides 'Multicall_v4::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'DeltaResolver' overrides 'Multicall_v4::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'DeltaResolver' overrides 'Multicall_v4::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'DeltaResolver' overrides 'Multicall_v4::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'DeltaResolver' overrides 'Multicall_v4::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'DeltaResolver' overrides 'Multicall_v4::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'DeltaResolver' overrides 'Multicall_v4::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'DeltaResolver' overrides 'Multicall_v4::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'DeltaResolver' overrides 'Multicall_v4::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'DeltaResolver' overrides 'Multicall_v4::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'DeltaResolver' overrides 'Multicall_v4::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'DeltaResolver' overrides 'Multicall_v4::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'DeltaResolver' overrides 'Multicall_v4::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'DeltaResolver' overrides 'Multicall_v4::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'DeltaResolver' overrides 'Multicall_v4::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'DeltaResolver' overrides 'Multicall_v4::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'DeltaResolver' overrides 'Multicall_v4::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'DeltaResolver' overrides 'Multicall_v4::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'DeltaResolver' overrides 'Multicall_v4::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'DeltaResolver' overrides 'Multicall_v4::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'DeltaResolver' overrides 'Multicall_v4::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'DeltaResolver' overrides 'Multicall_v4::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'DeltaResolver' overrides 'Multicall_v4::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'DeltaResolver' overrides 'Multicall_v4::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'DeltaResolver' overrides 'Multicall_v4::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'DeltaResolver' overrides 'Multicall_v4::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'DeltaResolver' overrides 'Multicall_v4::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'DeltaResolver' overrides 'Multicall_v4::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'DeltaResolver' overrides 'Multicall_v4::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'DeltaResolver' overrides 'Multicall_v4::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'DeltaResolver' overrides 'Multicall_v4::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'DeltaResolver' overrides 'Multicall_v4::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'DeltaResolver' overrides 'Multicall_v4::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'DeltaResolver' overrides 'Multicall_v4::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'DeltaResolver' overrides 'Multicall_v4::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'DeltaResolver' overrides 'Multicall_v4::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'DeltaResolver' overrides 'Multicall_v4::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'DeltaResolver' overrides 'Multicall_v4::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'DeltaResolver' overrides 'Multicall_v4::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'DeltaResolver' overrides 'Multicall_v4::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'DeltaResolver' overrides 'Multicall_v4::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'DeltaResolver' overrides 'Multicall_v4::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'DeltaResolver' overrides 'Multicall_v4::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'DeltaResolver' overrides 'Multicall_v4::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'DeltaResolver' overrides 'Multicall_v4::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'DeltaResolver' overrides 'Multicall_v4::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'DeltaResolver' overrides 'Multicall_v4::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'DeltaResolver' overrides 'Multicall_v4::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'DeltaResolver' overrides 'Multicall_v4::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'DeltaResolver' overrides 'Multicall_v4::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'DeltaResolver' overrides 'Multicall_v4::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'DeltaResolver' overrides 'Multicall_v4::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'DeltaResolver' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides 'DeltaResolver::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ReentrancyLock' overrides 'DeltaResolver::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'ReentrancyLock' overrides 'DeltaResolver::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'ReentrancyLock' overrides 'DeltaResolver::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'ReentrancyLock' overrides 'DeltaResolver::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'ReentrancyLock' overrides 'DeltaResolver::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ReentrancyLock' overrides 'DeltaResolver::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'ReentrancyLock' overrides 'DeltaResolver::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ReentrancyLock' overrides 'DeltaResolver::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ReentrancyLock' overrides 'DeltaResolver::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ReentrancyLock' overrides 'DeltaResolver::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ReentrancyLock' overrides 'DeltaResolver::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ReentrancyLock' overrides 'DeltaResolver::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ReentrancyLock' overrides 'DeltaResolver::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'ReentrancyLock' overrides 'DeltaResolver::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'ReentrancyLock' overrides 'DeltaResolver::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'ReentrancyLock' overrides 'DeltaResolver::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ReentrancyLock' overrides 'DeltaResolver::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'ReentrancyLock' overrides 'DeltaResolver::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ReentrancyLock' overrides 'DeltaResolver::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'ReentrancyLock' overrides 'DeltaResolver::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'ReentrancyLock' overrides 'DeltaResolver::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ReentrancyLock' overrides 'DeltaResolver::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'ReentrancyLock' overrides 'DeltaResolver::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'ReentrancyLock' overrides 'DeltaResolver::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'ReentrancyLock' overrides 'DeltaResolver::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'ReentrancyLock' overrides 'DeltaResolver::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'ReentrancyLock' overrides 'DeltaResolver::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'ReentrancyLock' overrides 'DeltaResolver::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'ReentrancyLock' overrides 'DeltaResolver::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'ReentrancyLock' overrides 'DeltaResolver::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'ReentrancyLock' overrides 'DeltaResolver::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'ReentrancyLock' overrides 'DeltaResolver::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'ReentrancyLock' overrides 'DeltaResolver::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'ReentrancyLock' overrides 'DeltaResolver::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'ReentrancyLock' overrides 'DeltaResolver::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'ReentrancyLock' overrides 'DeltaResolver::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'ReentrancyLock' overrides 'DeltaResolver::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'ReentrancyLock' overrides 'DeltaResolver::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'ReentrancyLock' overrides 'DeltaResolver::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'ReentrancyLock' overrides 'DeltaResolver::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ReentrancyLock' overrides 'DeltaResolver::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ReentrancyLock' overrides 'DeltaResolver::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'ReentrancyLock' overrides 'DeltaResolver::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'ReentrancyLock' overrides 'DeltaResolver::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'ReentrancyLock' overrides 'DeltaResolver::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'ReentrancyLock' overrides 'DeltaResolver::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'ReentrancyLock' overrides 'DeltaResolver::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'ReentrancyLock' overrides 'DeltaResolver::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'ReentrancyLock' overrides 'DeltaResolver::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'ReentrancyLock' overrides 'DeltaResolver::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'ReentrancyLock' overrides 'DeltaResolver::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'ReentrancyLock' overrides 'DeltaResolver::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'ReentrancyLock' overrides 'DeltaResolver::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'ReentrancyLock' overrides 'DeltaResolver::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'ReentrancyLock' overrides 'DeltaResolver::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'ReentrancyLock' overrides 'DeltaResolver::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'ReentrancyLock' overrides 'DeltaResolver::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ReentrancyLock' overrides 'DeltaResolver::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ReentrancyLock' overrides 'DeltaResolver::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'ReentrancyLock' overrides 'DeltaResolver::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'ReentrancyLock' overrides 'DeltaResolver::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'ReentrancyLock' overrides 'DeltaResolver::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'ReentrancyLock' overrides 'DeltaResolver::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'ReentrancyLock' overrides 'DeltaResolver::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'ReentrancyLock' overrides 'DeltaResolver::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'ReentrancyLock' overrides 'DeltaResolver::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'ReentrancyLock' overrides 'DeltaResolver::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'ReentrancyLock' overrides 'DeltaResolver::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'ReentrancyLock' overrides 'DeltaResolver::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'ReentrancyLock' overrides 'DeltaResolver::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'ReentrancyLock' overrides 'DeltaResolver::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'ReentrancyLock' overrides 'DeltaResolver::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'ReentrancyLock' overrides 'DeltaResolver::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'ReentrancyLock' overrides 'DeltaResolver::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'ReentrancyLock' overrides 'DeltaResolver::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'ReentrancyLock' overrides 'DeltaResolver::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'ReentrancyLock' overrides 'DeltaResolver::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'ReentrancyLock' overrides 'DeltaResolver::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'ReentrancyLock' overrides 'DeltaResolver::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'ReentrancyLock' overrides 'DeltaResolver::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'ReentrancyLock' overrides 'DeltaResolver::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'ReentrancyLock' overrides 'DeltaResolver::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'ReentrancyLock' overrides 'DeltaResolver::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'ReentrancyLock' overrides 'DeltaResolver::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'ReentrancyLock' overrides 'DeltaResolver::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'ReentrancyLock' overrides 'DeltaResolver::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'ReentrancyLock' overrides 'DeltaResolver::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'ReentrancyLock' overrides 'DeltaResolver::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'ReentrancyLock' overrides 'DeltaResolver::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'ReentrancyLock' overrides 'DeltaResolver::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'ReentrancyLock' overrides 'DeltaResolver::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'ReentrancyLock' overrides 'DeltaResolver::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'ReentrancyLock' overrides 'DeltaResolver::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'ReentrancyLock' overrides 'DeltaResolver::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'ReentrancyLock' overrides 'DeltaResolver::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'ReentrancyLock' overrides 'DeltaResolver::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'ReentrancyLock' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ReentrancyLock::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ReentrancyLock::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ReentrancyLock::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ReentrancyLock::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ReentrancyLock::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ReentrancyLock::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides 'ReentrancyLock::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'SafeCallback' overrides 'ReentrancyLock::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'SafeCallback' overrides 'ReentrancyLock::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'SafeCallback' overrides 'ReentrancyLock::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'SafeCallback' overrides 'ReentrancyLock::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'SafeCallback' overrides 'ReentrancyLock::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides 'ReentrancyLock::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'SafeCallback' overrides 'ReentrancyLock::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'SafeCallback' overrides 'ReentrancyLock::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides 'ReentrancyLock::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides 'ReentrancyLock::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides 'ReentrancyLock::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'SafeCallback' overrides 'ReentrancyLock::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides 'ReentrancyLock::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'SafeCallback' overrides 'ReentrancyLock::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'SafeCallback' overrides 'ReentrancyLock::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'SafeCallback' overrides 'ReentrancyLock::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'SafeCallback' overrides 'ReentrancyLock::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides 'ReentrancyLock::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'SafeCallback' overrides 'ReentrancyLock::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'SafeCallback' overrides 'ReentrancyLock::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'SafeCallback' overrides 'ReentrancyLock::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'SafeCallback' overrides 'ReentrancyLock::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'SafeCallback' overrides 'ReentrancyLock::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'SafeCallback' overrides 'ReentrancyLock::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'SafeCallback' overrides 'ReentrancyLock::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'SafeCallback' overrides 'ReentrancyLock::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'SafeCallback' overrides 'ReentrancyLock::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'SafeCallback' overrides 'ReentrancyLock::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'SafeCallback' overrides 'ReentrancyLock::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'SafeCallback' overrides 'ReentrancyLock::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'SafeCallback' overrides 'ReentrancyLock::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides 'ReentrancyLock::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides 'ReentrancyLock::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'SafeCallback' overrides 'ReentrancyLock::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'SafeCallback' overrides 'ReentrancyLock::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'SafeCallback' overrides 'ReentrancyLock::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'SafeCallback' overrides 'ReentrancyLock::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'SafeCallback' overrides 'ReentrancyLock::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides 'ReentrancyLock::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'SafeCallback' overrides 'ReentrancyLock::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'SafeCallback' overrides 'ReentrancyLock::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides 'ReentrancyLock::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'SafeCallback' overrides 'ReentrancyLock::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'SafeCallback' overrides 'ReentrancyLock::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'SafeCallback' overrides 'ReentrancyLock::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'SafeCallback' overrides 'ReentrancyLock::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'SafeCallback' overrides 'ReentrancyLock::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides 'ReentrancyLock::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides 'ReentrancyLock::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides 'ReentrancyLock::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'SafeCallback' overrides 'ReentrancyLock::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'SafeCallback' overrides 'ReentrancyLock::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides 'ReentrancyLock::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'SafeCallback' overrides 'ReentrancyLock::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'SafeCallback' overrides 'ReentrancyLock::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'SafeCallback' overrides 'ReentrancyLock::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'SafeCallback' overrides 'ReentrancyLock::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'SafeCallback' overrides 'ReentrancyLock::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'SafeCallback' overrides 'ReentrancyLock::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'SafeCallback' overrides 'ReentrancyLock::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'SafeCallback' overrides 'ReentrancyLock::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'SafeCallback' overrides 'ReentrancyLock::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'SafeCallback' overrides 'ReentrancyLock::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'SafeCallback' overrides 'ReentrancyLock::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'SafeCallback' overrides 'ReentrancyLock::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'SafeCallback' overrides 'ReentrancyLock::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'SafeCallback' overrides 'ReentrancyLock::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'SafeCallback' overrides 'ReentrancyLock::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'SafeCallback' overrides 'ReentrancyLock::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'SafeCallback' overrides 'ReentrancyLock::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'SafeCallback' overrides 'ReentrancyLock::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides 'ReentrancyLock::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'SafeCallback' overrides 'ReentrancyLock::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'SafeCallback' overrides 'ReentrancyLock::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'SafeCallback' overrides 'ReentrancyLock::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'SafeCallback' overrides 'ReentrancyLock::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'SafeCallback' overrides 'ReentrancyLock::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'SafeCallback' overrides 'ReentrancyLock::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'SafeCallback' overrides 'ReentrancyLock::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'SafeCallback' overrides 'ReentrancyLock::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'SafeCallback' overrides 'ReentrancyLock::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'SafeCallback' overrides 'ReentrancyLock::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'SafeCallback' overrides 'ReentrancyLock::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'SafeCallback' overrides 'ReentrancyLock::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'SafeCallback' overrides 'ReentrancyLock::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'SafeCallback' overrides 'ReentrancyLock::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'SafeCallback' overrides 'ReentrancyLock::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'SafeCallback' overrides 'ReentrancyLock::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'SafeCallback' overrides 'ReentrancyLock::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'SafeCallback' overrides 'ReentrancyLock::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'SafeCallback' overrides 'ReentrancyLock::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'SafeCallback' overrides 'ReentrancyLock::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'SafeCallback' overrides 'ReentrancyLock::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'SafeCallback' overrides 'ReentrancyLock::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'SafeCallback' overrides 'ReentrancyLock::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'SafeCallback' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'SafeCallback' overrides 'ReentrancyLock::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'SafeCallback' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'mulDiv' in 'BaseActionsRouter' overrides 'SafeCallback::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'BaseActionsRouter' overrides 'SafeCallback::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'BaseActionsRouter' overrides 'SafeCallback::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'BaseActionsRouter' overrides 'SafeCallback::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'BaseActionsRouter' overrides 'SafeCallback::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'BaseActionsRouter' overrides 'SafeCallback::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'BaseActionsRouter' overrides 'SafeCallback::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'BaseActionsRouter' overrides 'SafeCallback::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'BaseActionsRouter' overrides 'SafeCallback::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'BaseActionsRouter' overrides 'SafeCallback::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'BaseActionsRouter' overrides 'SafeCallback::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'BaseActionsRouter' overrides 'SafeCallback::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'BaseActionsRouter' overrides 'SafeCallback::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'BaseActionsRouter' overrides 'SafeCallback::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'BaseActionsRouter' overrides 'SafeCallback::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'BaseActionsRouter' overrides 'SafeCallback::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'BaseActionsRouter' overrides 'SafeCallback::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'BaseActionsRouter' overrides 'SafeCallback::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'BaseActionsRouter' overrides 'SafeCallback::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'BaseActionsRouter' overrides 'SafeCallback::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'poolId' in 'BaseActionsRouter' overrides 'SafeCallback::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'BaseActionsRouter' overrides 'SafeCallback::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'BaseActionsRouter' overrides 'SafeCallback::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'BaseActionsRouter' overrides 'SafeCallback::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'BaseActionsRouter' overrides 'SafeCallback::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'BaseActionsRouter' overrides 'SafeCallback::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'BaseActionsRouter' overrides 'SafeCallback::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'BaseActionsRouter' overrides 'SafeCallback::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'BaseActionsRouter' overrides 'SafeCallback::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'BaseActionsRouter' overrides 'SafeCallback::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'BaseActionsRouter' overrides 'SafeCallback::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'BaseActionsRouter' overrides 'SafeCallback::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
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
| warning | W200 | function 'validateMinOut' in 'BaseActionsRouter' overrides 'SafeCallback::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'BaseActionsRouter' overrides 'SafeCallback::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'BaseActionsRouter' overrides 'SafeCallback::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'BaseActionsRouter' overrides 'SafeCallback::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'BaseActionsRouter' overrides 'SafeCallback::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'BaseActionsRouter' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides 'BaseActionsRouter::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'Notifier' overrides 'BaseActionsRouter::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'Notifier' overrides 'BaseActionsRouter::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'Notifier' overrides 'BaseActionsRouter::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'Notifier' overrides 'BaseActionsRouter::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'Notifier' overrides 'BaseActionsRouter::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'Notifier' overrides 'BaseActionsRouter::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'Notifier' overrides 'BaseActionsRouter::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Notifier' overrides 'BaseActionsRouter::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Notifier' overrides 'BaseActionsRouter::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Notifier' overrides 'BaseActionsRouter::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Notifier' overrides 'BaseActionsRouter::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Notifier' overrides 'BaseActionsRouter::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Notifier' overrides 'BaseActionsRouter::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'Notifier' overrides 'BaseActionsRouter::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'Notifier' overrides 'BaseActionsRouter::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'Notifier' overrides 'BaseActionsRouter::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'Notifier' overrides 'BaseActionsRouter::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'Notifier' overrides 'BaseActionsRouter::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Notifier' overrides 'BaseActionsRouter::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'Notifier' overrides 'BaseActionsRouter::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'Notifier' overrides 'BaseActionsRouter::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'Notifier' overrides 'BaseActionsRouter::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'Notifier' overrides 'BaseActionsRouter::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'Notifier' overrides 'BaseActionsRouter::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'Notifier' overrides 'BaseActionsRouter::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'Notifier' overrides 'BaseActionsRouter::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'Notifier' overrides 'BaseActionsRouter::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'Notifier' overrides 'BaseActionsRouter::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'Notifier' overrides 'BaseActionsRouter::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'Notifier' overrides 'BaseActionsRouter::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'Notifier' overrides 'BaseActionsRouter::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'Notifier' overrides 'BaseActionsRouter::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'Notifier' overrides 'BaseActionsRouter::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'Notifier' overrides 'BaseActionsRouter::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'Notifier' overrides 'BaseActionsRouter::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'Notifier' overrides 'BaseActionsRouter::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'Notifier' overrides 'BaseActionsRouter::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'Notifier' overrides 'BaseActionsRouter::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'Notifier' overrides 'BaseActionsRouter::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'Notifier' overrides 'BaseActionsRouter::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'Notifier' overrides 'BaseActionsRouter::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'Notifier' overrides 'BaseActionsRouter::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'Notifier' overrides 'BaseActionsRouter::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'Notifier' overrides 'BaseActionsRouter::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'Notifier' overrides 'BaseActionsRouter::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'Notifier' overrides 'BaseActionsRouter::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'Notifier' overrides 'BaseActionsRouter::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'Notifier' overrides 'BaseActionsRouter::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'Notifier' overrides 'BaseActionsRouter::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'Notifier' overrides 'BaseActionsRouter::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'Notifier' overrides 'BaseActionsRouter::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'Notifier' overrides 'BaseActionsRouter::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'Notifier' overrides 'BaseActionsRouter::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'Notifier' overrides 'BaseActionsRouter::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'Notifier' overrides 'BaseActionsRouter::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'Notifier' overrides 'BaseActionsRouter::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'Notifier' overrides 'BaseActionsRouter::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'Notifier' overrides 'BaseActionsRouter::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'Notifier' overrides 'BaseActionsRouter::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'Notifier' overrides 'BaseActionsRouter::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'Notifier' overrides 'BaseActionsRouter::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'Notifier' overrides 'BaseActionsRouter::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'Notifier' overrides 'BaseActionsRouter::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'Notifier' overrides 'BaseActionsRouter::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'Notifier' overrides 'BaseActionsRouter::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'Notifier' overrides 'BaseActionsRouter::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'Notifier' overrides 'BaseActionsRouter::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'Notifier' overrides 'BaseActionsRouter::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'Notifier' overrides 'BaseActionsRouter::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'Notifier' overrides 'BaseActionsRouter::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'Notifier' overrides 'BaseActionsRouter::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'Notifier' overrides 'BaseActionsRouter::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'Notifier' overrides 'BaseActionsRouter::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'Notifier' overrides 'BaseActionsRouter::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'Notifier' overrides 'BaseActionsRouter::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'Notifier' overrides 'BaseActionsRouter::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'Notifier' overrides 'BaseActionsRouter::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'Notifier' overrides 'BaseActionsRouter::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'Notifier' overrides 'BaseActionsRouter::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'Notifier' overrides 'BaseActionsRouter::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'Notifier' overrides 'BaseActionsRouter::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'Notifier' overrides 'BaseActionsRouter::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'Notifier' overrides 'BaseActionsRouter::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'Notifier' overrides 'BaseActionsRouter::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'Notifier' overrides 'BaseActionsRouter::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'Notifier' overrides 'BaseActionsRouter::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'Notifier' overrides 'BaseActionsRouter::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'Notifier' overrides 'BaseActionsRouter::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'Notifier' overrides 'BaseActionsRouter::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'Notifier' overrides 'BaseActionsRouter::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'Notifier' overrides 'BaseActionsRouter::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'Notifier' overrides 'BaseActionsRouter::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'Notifier' overrides 'BaseActionsRouter::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'Notifier' overrides 'BaseActionsRouter::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'Notifier' overrides 'BaseActionsRouter::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'Notifier' overrides 'BaseActionsRouter::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'Notifier' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides 'Notifier::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides 'Notifier::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides 'Notifier::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides 'Notifier::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides 'Notifier::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides 'Notifier::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides 'Notifier::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'Permit2Forwarder' overrides 'Notifier::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'Permit2Forwarder' overrides 'Notifier::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'Permit2Forwarder' overrides 'Notifier::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'Permit2Forwarder' overrides 'Notifier::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'Permit2Forwarder' overrides 'Notifier::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'Permit2Forwarder' overrides 'Notifier::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'Permit2Forwarder' overrides 'Notifier::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Permit2Forwarder' overrides 'Notifier::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Permit2Forwarder' overrides 'Notifier::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Permit2Forwarder' overrides 'Notifier::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Permit2Forwarder' overrides 'Notifier::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Permit2Forwarder' overrides 'Notifier::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Permit2Forwarder' overrides 'Notifier::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'Permit2Forwarder' overrides 'Notifier::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'Permit2Forwarder' overrides 'Notifier::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'Permit2Forwarder' overrides 'Notifier::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'Permit2Forwarder' overrides 'Notifier::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'Permit2Forwarder' overrides 'Notifier::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'Permit2Forwarder' overrides 'Notifier::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'Permit2Forwarder' overrides 'Notifier::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'Permit2Forwarder' overrides 'Notifier::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'Permit2Forwarder' overrides 'Notifier::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'Permit2Forwarder' overrides 'Notifier::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'Permit2Forwarder' overrides 'Notifier::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'Permit2Forwarder' overrides 'Notifier::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'Permit2Forwarder' overrides 'Notifier::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'Permit2Forwarder' overrides 'Notifier::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'Permit2Forwarder' overrides 'Notifier::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'Permit2Forwarder' overrides 'Notifier::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'Permit2Forwarder' overrides 'Notifier::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'Permit2Forwarder' overrides 'Notifier::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'Permit2Forwarder' overrides 'Notifier::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'Permit2Forwarder' overrides 'Notifier::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'Permit2Forwarder' overrides 'Notifier::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'Permit2Forwarder' overrides 'Notifier::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'Permit2Forwarder' overrides 'Notifier::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'Permit2Forwarder' overrides 'Notifier::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'Permit2Forwarder' overrides 'Notifier::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'Permit2Forwarder' overrides 'Notifier::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'Permit2Forwarder' overrides 'Notifier::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'Permit2Forwarder' overrides 'Notifier::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'Permit2Forwarder' overrides 'Notifier::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'Permit2Forwarder' overrides 'Notifier::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'Permit2Forwarder' overrides 'Notifier::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'Permit2Forwarder' overrides 'Notifier::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'Permit2Forwarder' overrides 'Notifier::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'Permit2Forwarder' overrides 'Notifier::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'Permit2Forwarder' overrides 'Notifier::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'Permit2Forwarder' overrides 'Notifier::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'Permit2Forwarder' overrides 'Notifier::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'Permit2Forwarder' overrides 'Notifier::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'Permit2Forwarder' overrides 'Notifier::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'Permit2Forwarder' overrides 'Notifier::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'Permit2Forwarder' overrides 'Notifier::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'Permit2Forwarder' overrides 'Notifier::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'Permit2Forwarder' overrides 'Notifier::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'Permit2Forwarder' overrides 'Notifier::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'Permit2Forwarder' overrides 'Notifier::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'Permit2Forwarder' overrides 'Notifier::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'Permit2Forwarder' overrides 'Notifier::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'Permit2Forwarder' overrides 'Notifier::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'Permit2Forwarder' overrides 'Notifier::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'Permit2Forwarder' overrides 'Notifier::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'Permit2Forwarder' overrides 'Notifier::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'Permit2Forwarder' overrides 'Notifier::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'Permit2Forwarder' overrides 'Notifier::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'Permit2Forwarder' overrides 'Notifier::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'Permit2Forwarder' overrides 'Notifier::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'Permit2Forwarder' overrides 'Notifier::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'Permit2Forwarder' overrides 'Notifier::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'Permit2Forwarder' overrides 'Notifier::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'Permit2Forwarder' overrides 'Notifier::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'Permit2Forwarder' overrides 'Notifier::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'Permit2Forwarder' overrides 'Notifier::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'Permit2Forwarder' overrides 'Notifier::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'Permit2Forwarder' overrides 'Notifier::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'Permit2Forwarder' overrides 'Notifier::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'Permit2Forwarder' overrides 'Notifier::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'Permit2Forwarder' overrides 'Notifier::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'Permit2Forwarder' overrides 'Notifier::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'Permit2Forwarder' overrides 'Notifier::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'Permit2Forwarder' overrides 'Notifier::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'Permit2Forwarder' overrides 'Notifier::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'Permit2Forwarder' overrides 'Notifier::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'Permit2Forwarder' overrides 'Notifier::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'Permit2Forwarder' overrides 'Notifier::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'Permit2Forwarder' overrides 'Notifier::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'Permit2Forwarder' overrides 'Notifier::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'Permit2Forwarder' overrides 'Notifier::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'Permit2Forwarder' overrides 'Notifier::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'Permit2Forwarder' overrides 'Notifier::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'Permit2Forwarder' overrides 'Notifier::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'Permit2Forwarder' overrides 'Notifier::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'Permit2Forwarder' overrides 'Notifier::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'Permit2Forwarder' overrides 'Notifier::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'Permit2Forwarder' overrides 'Notifier::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'Permit2Forwarder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'Permit2Forwarder::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'Permit2Forwarder::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'Permit2Forwarder::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'Permit2Forwarder::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'Permit2Forwarder::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'Permit2Forwarder::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides 'Permit2Forwarder::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'NativeWrapper' overrides 'Permit2Forwarder::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'NativeWrapper' overrides 'Permit2Forwarder::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'NativeWrapper' overrides 'Permit2Forwarder::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'NativeWrapper' overrides 'Permit2Forwarder::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'NativeWrapper' overrides 'Permit2Forwarder::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'NativeWrapper' overrides 'Permit2Forwarder::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'NativeWrapper' overrides 'Permit2Forwarder::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'NativeWrapper' overrides 'Permit2Forwarder::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'NativeWrapper' overrides 'Permit2Forwarder::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'NativeWrapper' overrides 'Permit2Forwarder::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'NativeWrapper' overrides 'Permit2Forwarder::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'NativeWrapper' overrides 'Permit2Forwarder::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'NativeWrapper' overrides 'Permit2Forwarder::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'NativeWrapper' overrides 'Permit2Forwarder::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'NativeWrapper' overrides 'Permit2Forwarder::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'NativeWrapper' overrides 'Permit2Forwarder::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'NativeWrapper' overrides 'Permit2Forwarder::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'NativeWrapper' overrides 'Permit2Forwarder::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'NativeWrapper' overrides 'Permit2Forwarder::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'NativeWrapper' overrides 'Permit2Forwarder::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'NativeWrapper' overrides 'Permit2Forwarder::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'NativeWrapper' overrides 'Permit2Forwarder::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'NativeWrapper' overrides 'Permit2Forwarder::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'NativeWrapper' overrides 'Permit2Forwarder::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'NativeWrapper' overrides 'Permit2Forwarder::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'NativeWrapper' overrides 'Permit2Forwarder::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'NativeWrapper' overrides 'Permit2Forwarder::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'NativeWrapper' overrides 'Permit2Forwarder::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'NativeWrapper' overrides 'Permit2Forwarder::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'NativeWrapper' overrides 'Permit2Forwarder::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'NativeWrapper' overrides 'Permit2Forwarder::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'NativeWrapper' overrides 'Permit2Forwarder::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'NativeWrapper' overrides 'Permit2Forwarder::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'NativeWrapper' overrides 'Permit2Forwarder::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'NativeWrapper' overrides 'Permit2Forwarder::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'NativeWrapper' overrides 'Permit2Forwarder::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'NativeWrapper' overrides 'Permit2Forwarder::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'NativeWrapper' overrides 'Permit2Forwarder::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'NativeWrapper' overrides 'Permit2Forwarder::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'NativeWrapper' overrides 'Permit2Forwarder::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'NativeWrapper' overrides 'Permit2Forwarder::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'NativeWrapper' overrides 'Permit2Forwarder::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'NativeWrapper' overrides 'Permit2Forwarder::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'NativeWrapper' overrides 'Permit2Forwarder::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'NativeWrapper' overrides 'Permit2Forwarder::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'NativeWrapper' overrides 'Permit2Forwarder::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'NativeWrapper' overrides 'Permit2Forwarder::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'NativeWrapper' overrides 'Permit2Forwarder::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'NativeWrapper' overrides 'Permit2Forwarder::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'NativeWrapper' overrides 'Permit2Forwarder::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'NativeWrapper' overrides 'Permit2Forwarder::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'NativeWrapper' overrides 'Permit2Forwarder::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'NativeWrapper' overrides 'Permit2Forwarder::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'NativeWrapper' overrides 'Permit2Forwarder::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'NativeWrapper' overrides 'Permit2Forwarder::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'NativeWrapper' overrides 'Permit2Forwarder::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'NativeWrapper' overrides 'Permit2Forwarder::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'NativeWrapper' overrides 'Permit2Forwarder::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'NativeWrapper' overrides 'Permit2Forwarder::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'NativeWrapper' overrides 'Permit2Forwarder::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'NativeWrapper' overrides 'Permit2Forwarder::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'NativeWrapper' overrides 'Permit2Forwarder::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'NativeWrapper' overrides 'Permit2Forwarder::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'NativeWrapper' overrides 'Permit2Forwarder::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'NativeWrapper' overrides 'Permit2Forwarder::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'NativeWrapper' overrides 'Permit2Forwarder::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'NativeWrapper' overrides 'Permit2Forwarder::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'NativeWrapper' overrides 'Permit2Forwarder::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'NativeWrapper' overrides 'Permit2Forwarder::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'NativeWrapper' overrides 'Permit2Forwarder::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'NativeWrapper' overrides 'Permit2Forwarder::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'NativeWrapper' overrides 'Permit2Forwarder::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'NativeWrapper' overrides 'Permit2Forwarder::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'NativeWrapper' overrides 'Permit2Forwarder::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'NativeWrapper' overrides 'Permit2Forwarder::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'NativeWrapper' overrides 'Permit2Forwarder::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'NativeWrapper' overrides 'Permit2Forwarder::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'NativeWrapper' overrides 'Permit2Forwarder::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'NativeWrapper' overrides 'Permit2Forwarder::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'NativeWrapper' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides 'NativeWrapper::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides 'NativeWrapper::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides 'NativeWrapper::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides 'NativeWrapper::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides 'NativeWrapper::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides 'NativeWrapper::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides 'NativeWrapper::revertWith' which is not marked 'virtual' |
| warning | W200 | function 'revertWith' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'PositionManager' overrides 'NativeWrapper::bubbleUpAndRevertWith' which is not marked 'virtual' |
| warning | W200 | function 'bubbleUpAndRevertWith' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'transfer' in 'PositionManager' overrides 'NativeWrapper::transfer' which is not marked 'virtual' |
| warning | W200 | function 'transfer' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOfSelf' in 'PositionManager' overrides 'NativeWrapper::balanceOfSelf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOfSelf' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'balanceOf' in 'PositionManager' overrides 'NativeWrapper::balanceOf' which is not marked 'virtual' |
| warning | W200 | function 'balanceOf' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isAddressZero' in 'PositionManager' overrides 'NativeWrapper::isAddressZero' which is not marked 'virtual' |
| warning | W200 | function 'isAddressZero' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'PositionManager' overrides 'NativeWrapper::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'fromId' in 'PositionManager' overrides 'NativeWrapper::fromId' which is not marked 'virtual' |
| warning | W200 | function 'fromId' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'PositionManager' overrides 'NativeWrapper::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'PositionManager' overrides 'NativeWrapper::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'PositionManager' overrides 'NativeWrapper::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'PositionManager' overrides 'NativeWrapper::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'PositionManager' overrides 'NativeWrapper::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'PositionManager' overrides 'NativeWrapper::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount0' in 'PositionManager' overrides 'NativeWrapper::amount0' which is not marked 'virtual' |
| warning | W200 | function 'amount0' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'amount1' in 'PositionManager' overrides 'NativeWrapper::amount1' which is not marked 'virtual' |
| warning | W200 | function 'amount1' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSpecifiedDelta' in 'PositionManager' overrides 'NativeWrapper::getSpecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getSpecifiedDelta' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getUnspecifiedDelta' in 'PositionManager' overrides 'NativeWrapper::getUnspecifiedDelta' which is not marked 'virtual' |
| warning | W200 | function 'getUnspecifiedDelta' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toId' in 'PositionManager' overrides 'NativeWrapper::toId' which is not marked 'virtual' |
| warning | W200 | function 'toId' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'PositionManager' overrides 'NativeWrapper::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDivRoundingUp' in 'PositionManager' overrides 'NativeWrapper::mulDivRoundingUp' which is not marked 'virtual' |
| warning | W200 | function 'mulDivRoundingUp' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'addDelta' in 'PositionManager' overrides 'NativeWrapper::addDelta' which is not marked 'virtual' |
| warning | W200 | function 'addDelta' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'PositionManager' overrides 'NativeWrapper::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'calculatePositionKey' in 'PositionManager' overrides 'NativeWrapper::calculatePositionKey' which is not marked 'virtual' |
| warning | W200 | function 'calculatePositionKey' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'update' in 'PositionManager' overrides 'NativeWrapper::update' which is not marked 'virtual' |
| warning | W200 | function 'update' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSlot0' in 'PositionManager' overrides 'NativeWrapper::getSlot0' which is not marked 'virtual' |
| warning | W200 | function 'getSlot0' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickInfo' in 'PositionManager' overrides 'NativeWrapper::getTickInfo' which is not marked 'virtual' |
| warning | W200 | function 'getTickInfo' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickLiquidity' in 'PositionManager' overrides 'NativeWrapper::getTickLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getTickLiquidity' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'PositionManager' overrides 'NativeWrapper::getTickFeeGrowthOutside' which is not marked 'virtual' |
| warning | W200 | function 'getTickFeeGrowthOutside' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'PositionManager' overrides 'NativeWrapper::getFeeGrowthGlobals' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthGlobals' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidity' in 'PositionManager' overrides 'NativeWrapper::getLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidity' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickBitmap' in 'PositionManager' overrides 'NativeWrapper::getTickBitmap' which is not marked 'virtual' |
| warning | W200 | function 'getTickBitmap' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'PositionManager' overrides 'NativeWrapper::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionInfo' in 'PositionManager' overrides 'NativeWrapper::getPositionInfo' which is not marked 'virtual' |
| warning | W200 | function 'getPositionInfo' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPositionLiquidity' in 'PositionManager' overrides 'NativeWrapper::getPositionLiquidity' which is not marked 'virtual' |
| warning | W200 | function 'getPositionLiquidity' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getFeeGrowthInside' in 'PositionManager' overrides 'NativeWrapper::getFeeGrowthInside' which is not marked 'virtual' |
| warning | W200 | function 'getFeeGrowthInside' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPoolStateSlot' in 'PositionManager' overrides 'NativeWrapper::_getPoolStateSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPoolStateSlot' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getTickInfoSlot' in 'PositionManager' overrides 'NativeWrapper::_getTickInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getTickInfoSlot' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getPositionInfoSlot' in 'PositionManager' overrides 'NativeWrapper::_getPositionInfoSlot' which is not marked 'virtual' |
| warning | W200 | function '_getPositionInfoSlot' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'PositionManager' overrides 'NativeWrapper::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'resetCurrency' in 'PositionManager' overrides 'NativeWrapper::resetCurrency' which is not marked 'virtual' |
| warning | W200 | function 'resetCurrency' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'PositionManager' overrides 'NativeWrapper::syncCurrencyAndReserves' which is not marked 'virtual' |
| warning | W200 | function 'syncCurrencyAndReserves' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'PositionManager' overrides 'NativeWrapper::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'read' in 'PositionManager' overrides 'NativeWrapper::read' which is not marked 'virtual' |
| warning | W200 | function 'read' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'increment' in 'PositionManager' overrides 'NativeWrapper::increment' which is not marked 'virtual' |
| warning | W200 | function 'increment' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decrement' in 'PositionManager' overrides 'NativeWrapper::decrement' which is not marked 'virtual' |
| warning | W200 | function 'decrement' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unlock' in 'PositionManager' overrides 'NativeWrapper::unlock' which is not marked 'virtual' |
| warning | W200 | function 'unlock' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'lock' in 'PositionManager' overrides 'NativeWrapper::lock' which is not marked 'virtual' |
| warning | W200 | function 'lock' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'PositionManager' overrides 'NativeWrapper::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedReserves' in 'PositionManager' overrides 'NativeWrapper::getSyncedReserves' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedReserves' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSyncedCurrency' in 'PositionManager' overrides 'NativeWrapper::getSyncedCurrency' which is not marked 'virtual' |
| warning | W200 | function 'getSyncedCurrency' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'PositionManager' overrides 'NativeWrapper::getNonzeroDeltaCount' which is not marked 'virtual' |
| warning | W200 | function 'getNonzeroDeltaCount' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'currencyDelta' in 'PositionManager' overrides 'NativeWrapper::currencyDelta' which is not marked 'virtual' |
| warning | W200 | function 'currencyDelta' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'isUnlocked' in 'PositionManager' overrides 'NativeWrapper::isUnlocked' which is not marked 'virtual' |
| warning | W200 | function 'isUnlocked' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mostSignificantBit' in 'PositionManager' overrides 'NativeWrapper::mostSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'mostSignificantBit' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'leastSignificantBit' in 'PositionManager' overrides 'NativeWrapper::leastSignificantBit' which is not marked 'virtual' |
| warning | W200 | function 'leastSignificantBit' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'maxUsableTick' in 'PositionManager' overrides 'NativeWrapper::maxUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'maxUsableTick' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'minUsableTick' in 'PositionManager' overrides 'NativeWrapper::minUsableTick' which is not marked 'virtual' |
| warning | W200 | function 'minUsableTick' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'PositionManager' overrides 'NativeWrapper::getSqrtPriceAtTick' which is not marked 'virtual' |
| warning | W200 | function 'getSqrtPriceAtTick' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'PositionManager' overrides 'NativeWrapper::getTickAtSqrtPrice' which is not marked 'virtual' |
| warning | W200 | function 'getTickAtSqrtPrice' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'poolId' in 'PositionManager' overrides 'NativeWrapper::poolId' which is not marked 'virtual' |
| warning | W200 | function 'poolId' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickLower' in 'PositionManager' overrides 'NativeWrapper::tickLower' which is not marked 'virtual' |
| warning | W200 | function 'tickLower' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tickUpper' in 'PositionManager' overrides 'NativeWrapper::tickUpper' which is not marked 'virtual' |
| warning | W200 | function 'tickUpper' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hasSubscriber' in 'PositionManager' overrides 'NativeWrapper::hasSubscriber' which is not marked 'virtual' |
| warning | W200 | function 'hasSubscriber' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setSubscribe' in 'PositionManager' overrides 'NativeWrapper::setSubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setSubscribe' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'setUnsubscribe' in 'PositionManager' overrides 'NativeWrapper::setUnsubscribe' which is not marked 'virtual' |
| warning | W200 | function 'setUnsubscribe' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'initialize' in 'PositionManager' overrides 'NativeWrapper::initialize' which is not marked 'virtual' |
| warning | W200 | function 'initialize' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermit' in 'PositionManager' overrides 'NativeWrapper::hashPermit' which is not marked 'virtual' |
| warning | W200 | function 'hashPermit' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'hashPermitForAll' in 'PositionManager' overrides 'NativeWrapper::hashPermitForAll' which is not marked 'virtual' |
| warning | W200 | function 'hashPermitForAll' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verify' in 'PositionManager' overrides 'NativeWrapper::verify' which is not marked 'virtual' |
| warning | W200 | function 'verify' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'set' in 'PositionManager' overrides 'NativeWrapper::set' which is not marked 'virtual' |
| warning | W200 | function 'set' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'get' in 'PositionManager' overrides 'NativeWrapper::get' which is not marked 'virtual' |
| warning | W200 | function 'get' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'PositionManager' overrides 'NativeWrapper::getPoolAndSwapDirection' which is not marked 'virtual' |
| warning | W200 | function 'getPoolAndSwapDirection' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeActionsRouterParams' in 'PositionManager' overrides 'NativeWrapper::decodeActionsRouterParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeActionsRouterParams' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'PositionManager' overrides 'NativeWrapper::decodeModifyLiquidityParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeModifyLiquidityParams' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'PositionManager' overrides 'NativeWrapper::decodeIncreaseLiquidityFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeIncreaseLiquidityFromDeltasParams' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintParams' in 'PositionManager' overrides 'NativeWrapper::decodeMintParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintParams' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'PositionManager' overrides 'NativeWrapper::decodeMintFromDeltasParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeMintFromDeltasParams' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeBurnParams' in 'PositionManager' overrides 'NativeWrapper::decodeBurnParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeBurnParams' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInParams' in 'PositionManager' overrides 'NativeWrapper::decodeSwapExactInParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInParams' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'PositionManager' overrides 'NativeWrapper::decodeSwapExactInSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactInSingleParams' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'PositionManager' overrides 'NativeWrapper::decodeSwapExactOutParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutParams' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'PositionManager' overrides 'NativeWrapper::decodeSwapExactOutSingleParams' which is not marked 'virtual' |
| warning | W200 | function 'decodeSwapExactOutSingleParams' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrency' in 'PositionManager' overrides 'NativeWrapper::decodeCurrency' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrency' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPair' in 'PositionManager' overrides 'NativeWrapper::decodeCurrencyPair' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPair' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'PositionManager' overrides 'NativeWrapper::decodeCurrencyPairAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyPairAndAddress' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'PositionManager' overrides 'NativeWrapper::decodeCurrencyAndAddress' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndAddress' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'PositionManager' overrides 'NativeWrapper::decodeCurrencyAddressAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAddressAndUint256' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'PositionManager' overrides 'NativeWrapper::decodeCurrencyAndUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyAndUint256' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeUint256' in 'PositionManager' overrides 'NativeWrapper::decodeUint256' which is not marked 'virtual' |
| warning | W200 | function 'decodeUint256' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'PositionManager' overrides 'NativeWrapper::decodeCurrencyUint256AndBool' which is not marked 'virtual' |
| warning | W200 | function 'decodeCurrencyUint256AndBool' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toBytes' in 'PositionManager' overrides 'NativeWrapper::toBytes' which is not marked 'virtual' |
| warning | W200 | function 'toBytes' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMinOut' in 'PositionManager' overrides 'NativeWrapper::validateMinOut' which is not marked 'virtual' |
| warning | W200 | function 'validateMinOut' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'validateMaxIn' in 'PositionManager' overrides 'NativeWrapper::validateMaxIn' which is not marked 'virtual' |
| warning | W200 | function 'validateMaxIn' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount0' in 'PositionManager' overrides 'NativeWrapper::getLiquidityForAmount0' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount0' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmount1' in 'PositionManager' overrides 'NativeWrapper::getLiquidityForAmount1' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmount1' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | W200 | function 'getLiquidityForAmounts' in 'PositionManager' overrides 'NativeWrapper::getLiquidityForAmounts' which is not marked 'virtual' |
| warning | W200 | function 'getLiquidityForAmounts' in 'PositionManager' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'PositionManager' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@uniswap/v4-periphery/src/PositionManager.sol`