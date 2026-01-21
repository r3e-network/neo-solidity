// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title NEP17Rescue
 * @dev Optional extension for NEP-17 tokens to recover other NEP-17 tokens.
 *
 * This is intentionally NOT part of the base NEP-17 implementation because
 * recovering arbitrary tokens requires a dynamic external contract call
 * (`transfer` on an unknown contract hash), which expands manifest permissions.
 *
 * If you don't need token rescue functionality, inherit `NEP17` directly.
 */

import "../standards/NEP17.sol";

abstract contract NEP17Rescue is NEP17 {
    function emergencyTokenRecovery(
        address token,
        address to,
        uint256 amount,
        bytes calldata data
    ) external onlyOwner {
        require(token != address(this), "NEP17: cannot recover own tokens");
        require(to != address(0), "NEP17: cannot recover to zero address");

        bool ok = INEP17(token).transfer(address(this), to, amount, data);
        require(ok, "NEP17: token recovery failed");
    }
}

