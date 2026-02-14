// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title WitnessGuardShowcase
 * @notice Demonstrates witness-based authorization for owner/guardian workflows.
 */
contract WitnessGuardShowcase {
    address public owner;

    mapping(address => bool) public isGuardian;
    mapping(address => uint256) public lockedUntil;

    event GuardianUpdated(address indexed guardian, bool enabled);
    event AccountLocked(address indexed account, uint256 untilTimestamp);
    event AccountUnlocked(address indexed account);
    event PrivilegedAction(address indexed actor, uint256 value, bytes32 memoHash);

    error Unauthorized();
    error InvalidOwner(address account);
    error InvalidOwnerHex();
    error InvalidWitness(address account);
    error AccountLockedError(address account, uint256 untilTimestamp);

    modifier onlyOwnerWithWitness() {
        if (msg.sender != owner) revert Unauthorized();
        if (!Runtime.checkWitness(msg.sender)) revert InvalidWitness(msg.sender);
        _;
    }

    modifier onlyGuardianWithWitness() {
        if (!isGuardian[msg.sender]) revert Unauthorized();
        if (!Runtime.checkWitness(msg.sender)) revert InvalidWitness(msg.sender);
        _;
    }

    constructor(string memory initialOwnerHex, address[] memory initialGuardians) {
        address initialOwner = _parseHexAddress(initialOwnerHex);
        if (initialOwner == address(0)) revert InvalidOwner(initialOwner);
        owner = initialOwner;

        for (uint256 i = 0; i < initialGuardians.length; i++) {
            address guardian = initialGuardians[i];
            if (guardian != address(0)) {
                isGuardian[guardian] = true;
                emit GuardianUpdated(guardian, true);
            }
        }
    }

    function setGuardian(address guardian, bool enabled) external onlyOwnerWithWitness {
        isGuardian[guardian] = enabled;
        emit GuardianUpdated(guardian, enabled);
    }

    function lockAccount(address account, uint256 durationSeconds)
        external
        onlyGuardianWithWitness
    {
        uint256 lockUntil = block.timestamp + durationSeconds;
        if (lockUntil > lockedUntil[account]) {
            lockedUntil[account] = lockUntil;
            emit AccountLocked(account, lockUntil);
        }
    }

    function unlockAccount(address account) external onlyOwnerWithWitness {
        lockedUntil[account] = 0;
        emit AccountUnlocked(account);
    }

    function privilegedAction(
        address actor,
        uint256 value,
        string calldata memo
    ) external {
        if (!Runtime.checkWitness(actor)) revert InvalidWitness(actor);

        uint256 untilTimestamp = lockedUntil[actor];
        if (untilTimestamp > block.timestamp) {
            revert AccountLockedError(actor, untilTimestamp);
        }

        emit PrivilegedAction(actor, value, keccak256(bytes(memo)));
    }

    function isLocked(address account) external view returns (bool) {
        return lockedUntil[account] > block.timestamp;
    }

    function _parseHexAddress(string memory raw) private pure returns (address) {
        bytes memory text = bytes(raw);
        uint256 offset = 0;
        if (
            text.length == 42 &&
            text[0] == bytes1("0") &&
            (text[1] == bytes1("x") || text[1] == bytes1("X"))
        ) {
            offset = 2;
        } else if (text.length != 40) {
            revert InvalidOwnerHex();
        }

        uint160 value = 0;
        for (uint256 i = 0; i < 40; i++) {
            value = (value << 4) | uint160(_hexNibble(uint8(text[offset + i])));
        }
        return address(value);
    }

    function _hexNibble(uint8 c) private pure returns (uint8) {
        if (c >= uint8(bytes1("0")) && c <= uint8(bytes1("9"))) {
            return c - uint8(bytes1("0"));
        }
        if (c >= uint8(bytes1("a")) && c <= uint8(bytes1("f"))) {
            return c - uint8(bytes1("a")) + 10;
        }
        if (c >= uint8(bytes1("A")) && c <= uint8(bytes1("F"))) {
            return c - uint8(bytes1("A")) + 10;
        }
        revert InvalidOwnerHex();
    }
}
