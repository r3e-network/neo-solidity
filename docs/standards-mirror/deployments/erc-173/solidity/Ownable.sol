// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title Ownable — ERC-173 contract ownership reference, compiled to Neo N3.
contract Ownable {
    string public buildTag = "ownable-v1";
    address public ownerAddress;
    address public pendingOwner;

    function getOwner() public view returns (address) {
        return ownerAddress;
    }

    function transferOwnership(address newOwner) public {
        require(msg.sender == ownerAddress || ownerAddress == address(0), "Ownable: not owner");
        require(newOwner != address(0), "Ownable: zero owner");
        pendingOwner = newOwner;
    }

    function acceptOwnership() public {
        require(msg.sender == pendingOwner, "Ownable: not pending owner");
        ownerAddress = pendingOwner;
        delete pendingOwner;
    }

    /// First call sets the owner (since constructor msg.sender is ManagementContract on Neo).
    function claimOwnership() public {
        require(ownerAddress == address(0), "Ownable: already claimed");
        ownerAddress = msg.sender;
    }
}
