// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title ModifierShowcase
 * @notice Demonstrates function modifiers, modifier chaining,
 *         modifiers with arguments, and the _ placeholder.
 */
contract ModifierShowcase {
    address public owner;
    bool public paused;
    uint256 public value;
    uint256 public callCount;

    constructor() {
        owner = msg.sender;
        paused = false;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "not owner");
        _;
    }

    modifier whenNotPaused() {
        require(!paused, "paused");
        _;
    }

    modifier costs(uint256 minValue) {
        require(msg.value >= minValue, "insufficient value");
        _;
    }

    modifier counted() {
        callCount += 1;
        _;
    }

    function pause() public onlyOwner {
        paused = true;
    }

    function unpause() public onlyOwner {
        paused = false;
    }

    /// @notice Chained modifiers: onlyOwner + whenNotPaused + counted
    function setValue(uint256 _v) public onlyOwner whenNotPaused counted {
        value = _v;
    }

    /// @notice Modifier with argument
    function deposit() public payable costs(1) whenNotPaused counted {
        value += msg.value;
    }

    function getCallCount() public view returns (uint256) {
        return callCount;
    }
}
