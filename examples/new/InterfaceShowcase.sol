// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title InterfaceShowcase
 * @notice Demonstrates interface definition, implementation,
 *         interface inheritance, and ERC-165 style introspection.
 */

interface INEP17 {
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address from, address to, uint256 amount) external returns (bool);
}

interface INEP17Metadata is INEP17 {
    function name() external view returns (string memory);
    function symbol() external view returns (string memory);
    function decimals() external view returns (uint8);
}

contract SimpleToken is INEP17Metadata {
    string private _name;
    string private _symbol;
    uint256 private _totalSupply;
    mapping(address => uint256) private _balances;

    constructor(string memory tokenName, string memory tokenSymbol, uint256 initialSupply) {
        _name = tokenName;
        _symbol = tokenSymbol;
        _totalSupply = initialSupply;
        _balances[msg.sender] = initialSupply;
    }

    function name() external view override returns (string memory) {
        return _name;
    }

    function symbol() external view override returns (string memory) {
        return _symbol;
    }

    function decimals() external pure override returns (uint8) {
        return 8;
    }

    function totalSupply() external view override returns (uint256) {
        return _totalSupply;
    }

    function balanceOf(address account) external view override returns (uint256) {
        return _balances[account];
    }

    function transfer(address from, address to, uint256 amount) external override returns (bool) {
        require(_balances[from] >= amount, "insufficient balance");
        _balances[from] -= amount;
        _balances[to] += amount;
        return true;
    }

    function supportsInterface(bytes4 interfaceId) external pure returns (bool) {
        return interfaceId == type(INEP17).interfaceId ||
               interfaceId == type(INEP17Metadata).interfaceId;
    }
}
