// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract SimpleNFT {
    string public name = "Neo Artifacts";
    string public symbol = "ART";

    mapping(uint256 => address) private owners;
    mapping(address => uint256) private balances;

    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId, bytes data);

    function ownerOf(uint256 tokenId) public view returns (address) {
        address owner = owners[tokenId];
        require(owner != address(0), "nonexistent token");
        return owner;
    }

    function balanceOf(address account) public view returns (uint256) {
        require(account != address(0), "zero address");
        return balances[account];
    }

    function mint(address to, uint256 tokenId) public {
        require(to != address(0), "zero address");
        require(owners[tokenId] == address(0), "already minted");

        owners[tokenId] = to;
        balances[to] += 1;

        emit Transfer(address(0), to, tokenId, "");
    }

    function transfer(address to, uint256 tokenId, bytes memory data) public {
        require(to != address(0), "zero address");
        address from = ownerOf(tokenId);
        require(msg.sender == from, "only owner can transfer");

        owners[tokenId] = to;
        balances[from] -= 1;
        balances[to] += 1;
        emit Transfer(from, to, tokenId, data);
    }

    // Compatibility wrapper for familiar ERC-721 UX.
    function transferFrom(address from, address to, uint256 tokenId) public {
        require(from == ownerOf(tokenId), "not owner");
        transfer(to, tokenId, "");
    }
}
