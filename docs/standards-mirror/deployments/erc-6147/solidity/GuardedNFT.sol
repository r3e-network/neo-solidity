// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title GuardedNFT — ERC-6147 NFT with guard co-signer, compiled to Neo N3.
contract GuardedNFT {
    string public buildTag = "guarded-v1";
    string public name = "Guarded NFT";
    string public symbol = "GNFT";

    address public deployer;
    uint256 private _next;
    mapping(uint256 => address) public ownerOf;
    mapping(uint256 => address) public guardOf;

    function claimDeployer() public {
        require(deployer == address(0), "GNFT: already claimed");
        deployer = msg.sender;
    }

    function mint(address to) public returns (uint256 tokenId) {
        require(msg.sender == deployer, "GNFT: deployer only");
        tokenId = ++_next;
        ownerOf[tokenId] = to;
    }

    function setGuard(uint256 tokenId, address guard) public {
        require(msg.sender == ownerOf[tokenId], "GNFT: not owner");
        guardOf[tokenId] = guard;
    }

    function tokenCount() public view returns (uint256) { return _next; }
}
