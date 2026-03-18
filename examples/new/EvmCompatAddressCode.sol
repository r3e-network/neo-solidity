// SPDX-License-Identifier: MIT
pragma solidity ^0.8.34;

contract EvmCompatAddressCode {
    function codeOf(address account) public view returns (bytes memory) {
        return account.code;
    }
}
