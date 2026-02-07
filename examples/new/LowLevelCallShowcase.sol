// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

interface ICounter {
    function get() external view returns (uint256);
    function set(uint256 value) external;
}

/**
 * @title LowLevelCallShowcase
 * @notice Demonstrates Neo-supported low-level call patterns using abi.encodeWith* helpers.
 */
contract LowLevelCallShowcase {
    function readViaSignature(address target) external returns (bool ok, bytes memory data) {
        bytes memory payload = abi.encodeWithSignature("get()");
        (ok, data) = target.staticcall(payload);
    }

    function readViaSelector(address target) external returns (bool ok, bytes memory data) {
        bytes memory payload = abi.encodeWithSelector(ICounter.get.selector);
        (ok, data) = target.staticcall(payload);
    }

    function writeViaSignature(address target, uint256 value)
        external
        returns (bool ok, bytes memory data)
    {
        bytes memory payload = abi.encodeWithSignature("set(uint256)", value);
        (ok, data) = target.call(payload);
    }
}
