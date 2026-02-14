// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title UpgradeLifecycleShowcase
 * @notice Demonstrates owner+witness gated upgrade and destroy flows on Neo N3.
 *
 * Neo N3 upgrades are performed via ContractManagement native contract calls.
 * Constructor `msg.sender` on Neo deploy is the ContractManagement native contract, so
 * owner is initialized from an explicit constructor argument.
 * This sample keeps those operations explicit and tightly authorized.
 */
contract UpgradeLifecycleShowcase {
    address public owner;
    uint256 public version;

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event ContractUpgraded(uint256 indexed previousVersion, uint256 indexed newVersion);
    event ContractDestroyed(address indexed by);

    error NotOwner();
    error InvalidWitness();
    error InvalidOwner();
    error InvalidOwnerHex();
    error InvalidVersion();
    error EmptyArtifact();

    modifier onlyOwnerWithWitness() {
        if (msg.sender != owner) revert NotOwner();
        if (!Runtime.checkWitness(msg.sender)) revert InvalidWitness();
        _;
    }

    constructor(uint256 initialVersion, string memory initialOwnerHex) {
        owner = _parseHexAddress(initialOwnerHex);
        if (owner == address(0)) revert InvalidOwner();
        version = initialVersion == 0 ? 1 : initialVersion;
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

    function transferOwnership(address newOwner) external onlyOwnerWithWitness {
        if (newOwner == address(0)) revert InvalidOwner();

        address previousOwner = owner;
        owner = newOwner;
        emit OwnershipTransferred(previousOwner, newOwner);
    }

    function upgrade(
        bytes calldata nef,
        bytes calldata manifest,
        uint256 newVersion
    ) external onlyOwnerWithWitness {
        if (nef.length == 0 || manifest.length == 0) revert EmptyArtifact();
        if (newVersion <= version) revert InvalidVersion();

        uint256 previousVersion = version;
        version = newVersion;

        NativeCalls.updateContract(nef, manifest);
        emit ContractUpgraded(previousVersion, newVersion);
    }

    function destroyContract() external onlyOwnerWithWitness {
        emit ContractDestroyed(msg.sender);
        NativeCalls.destroyContract();
    }

    function gasBalance() external view returns (uint256) {
        return NativeCalls.gasBalanceOf(address(this));
    }
}
