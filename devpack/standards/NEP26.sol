// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title NEP-26 Upgrade Lifecycle Standard (Convention)
 * @dev Minimal, production-oriented upgrade/destroy surface for Neo N3.
 *
 * Neo N3 performs in-place upgrades through ContractManagement native methods:
 * - update(nef, manifest[, data])
 * - destroy()
 *
 * This file provides a reusable interface + guarded mixin so contracts can
 * expose canonical `update` and `destroy` entrypoints that compiler standard
 * detection can recognize as `NEP-26`.
 */

import "../contracts/FrameworkBase.sol";
import "../contracts/NativeCalls.sol";

/**
 * @dev Conventional NEP-26 interface.
 *
 * Note: NEP-26 is commonly treated as an ecosystem convention (upgrade
 * lifecycle) rather than a strict token-like interface spec.
 */
interface INEP26 {
    function update(bytes calldata nef, bytes calldata manifest) external;
    function update(bytes calldata nef, bytes calldata manifest, bytes calldata data) external;
    function destroy() external;

    event ContractUpdated(address indexed updater, uint256 nefBytes, uint256 manifestBytes, bool withData);
    event ContractDestroyed(address indexed destroyer);
}

/**
 * @title NEP26Upgradable
 * @dev Owner + witness guarded implementation of INEP26.
 */
abstract contract NEP26Upgradable is INEP26, FrameworkBase {
    error NEP26EmptyNef();
    error NEP26EmptyManifest();

    function update(bytes calldata nef, bytes calldata manifest)
        public
        virtual
        override
        onlyOwner
        withWitness
    {
        if (nef.length == 0) revert NEP26EmptyNef();
        if (manifest.length == 0) revert NEP26EmptyManifest();

        NativeCalls.updateContract(nef, manifest);
        emit ContractUpdated(msg.sender, nef.length, manifest.length, false);
    }

    function update(bytes calldata nef, bytes calldata manifest, bytes calldata data)
        public
        virtual
        override
        onlyOwner
        withWitness
    {
        if (nef.length == 0) revert NEP26EmptyNef();
        if (manifest.length == 0) revert NEP26EmptyManifest();

        NativeCalls.updateContract(nef, manifest, data);
        emit ContractUpdated(msg.sender, nef.length, manifest.length, true);
    }

    function destroy() public virtual override onlyOwner withWitness {
        emit ContractDestroyed(msg.sender);
        NativeCalls.destroyContract();
    }
}
