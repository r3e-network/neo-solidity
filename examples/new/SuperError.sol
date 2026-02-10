// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/// @dev This file intentionally uses `super` to verify the compiler
///      produces a clear diagnostic error with a workaround suggestion.

contract Base {
    function greet() public pure virtual returns (string memory) {
        return "Base";
    }
}

contract Child is Base {
    function greet() public pure override returns (string memory) {
        return "Child";
    }

    function parentGreet() public pure returns (string memory) {
        return super.greet();
    }
}
