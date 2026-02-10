// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

library BadLib {
    function doSomething(uint256 x) external pure returns (uint256) {
        return x + 1;
    }
}

contract User {
    function use_lib(uint256 x) public pure returns (uint256) {
        return BadLib.doSomething(x);
    }
}
