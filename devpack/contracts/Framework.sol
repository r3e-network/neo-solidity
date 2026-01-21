// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title Neo N3 Framework
 * @dev Extended framework for Neo N3 Solidity contracts.
 *
 * `FrameworkBase.sol` is designed to keep manifests minimally-permissioned by
 * default. This `Framework.sol` contract adds an explicit public dynamic call
 * surface (`callContract`) for advanced scenarios.
 *
 * WARNING: Fully dynamic contract calls force full wildcard permissions in the
 * Neo N3 manifest (`{"contract":"*","methods":"*"}`).
 */

import "./FrameworkBase.sol";
import "../libraries/Neo.sol";

contract Framework is FrameworkBase {
    /**
     * @dev Call another contract (fully dynamic).
     *
     * Because both the target and method can be user-controlled, Neo N3
     * permission inference cannot restrict this safely without wildcards.
     */
    function callContract(address contractHash, string calldata method, bytes calldata params)
        public
        withWitness
        returns (bytes memory)
    {
        return Neo.callContract(contractHash, method, params);
    }
}

