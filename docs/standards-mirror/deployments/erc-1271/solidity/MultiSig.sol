// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title MultiSig — ERC-1271 contract-signature reference, compiled to Neo N3.
contract MultiSig {
    string public buildTag = "multisig-v1";
    bytes4 private constant MAGIC = 0x1626ba7e;

    address[] public owners;
    uint256 public threshold;
    address public deployer;

    function setup(address[] memory owners_, uint256 threshold_) public {
        require(deployer == address(0), "MultiSig: already setup");
        require(owners_.length > 0 && threshold_ > 0 && threshold_ <= owners_.length, "MultiSig: bad params");
        deployer = msg.sender;
        owners = owners_;
        threshold = threshold_;
    }

    function isOwner(address a) public view returns (bool) {
        for (uint i; i < owners.length; ++i) if (owners[i] == a) return true;
        return false;
    }

    function ownerCount() public view returns (uint256) { return owners.length; }

    /// Stub isValidSignature — returns MAGIC iff the (address) recovered (off-chain
    /// in a real impl) matches an owner. Demo: returns MAGIC iff caller is an owner.
    function isValidSignature(bytes32 /*hash*/, bytes memory /*sig*/)
        public view returns (bytes4)
    {
        if (isOwner(msg.sender)) return MAGIC;
        return 0xffffffff;
    }
}
