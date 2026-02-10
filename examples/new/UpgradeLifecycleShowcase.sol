// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title UpgradeLifecycleShowcase
 * @notice Demonstrates owner+witness gated upgrade and destroy flows on Neo N3.
 *
 * Neo N3 upgrades are performed via ContractManagement native contract calls.
 * Owner is initialized from `tx.origin` at deploy time (constructor `msg.sender` on Neo
 * is the ContractManagement native contract).
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
    error InvalidVersion();
    error EmptyArtifact();

    modifier onlyOwnerWithWitness() {
        if (msg.sender != owner) revert NotOwner();
        if (!Runtime.checkWitness(msg.sender)) revert InvalidWitness();
        _;
    }

    constructor(uint256 initialVersion) {
        owner = tx.origin;
        version = initialVersion == 0 ? 1 : initialVersion;
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
