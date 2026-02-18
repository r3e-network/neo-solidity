// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title NEP-26 NEP-11 Receiver Callback
 * @dev Callback invoked when a contract receives NEP-11 tokens.
 * Spec: https://github.com/neo-project/proposals/blob/master/nep-26.mediawiki
 */
interface INEP26Receiver {
    function onNEP11Payment(
        address from,
        uint256 amount,
        bytes32 tokenId,
        bytes calldata data
    ) external;
}
