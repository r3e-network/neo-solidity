// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title EventIndexedShowcase
 * @notice Demonstrates events with indexed params,
 *         anonymous events, and multi-topic events.
 */
contract EventIndexedShowcase {
    event Transfer(
        address indexed from,
        address indexed to,
        uint256 amount
    );

    event Approval(
        address indexed owner,
        address indexed spender,
        uint256 value
    );

    event MetadataUpdate(
        uint256 indexed tokenId,
        string key,
        string value
    );

    /// @notice Anonymous event — no topic[0] selector
    event AnonymousLog(uint256 data) anonymous;

    mapping(address => uint256) public balances;

    function mint(address to, uint256 amount) public {
        balances[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function transfer(address to, uint256 amount) public {
        require(balances[msg.sender] >= amount, "insufficient");
        balances[msg.sender] -= amount;
        balances[to] += amount;
        emit Transfer(msg.sender, to, amount);
    }

    function approve(address spender, uint256 value) public {
        emit Approval(msg.sender, spender, value);
    }

    function updateMetadata(
        uint256 tokenId,
        string memory key,
        string memory val
    ) public {
        emit MetadataUpdate(tokenId, key, val);
    }

    function emitAnonymous(uint256 data) public {
        emit AnonymousLog(data);
    }
}
