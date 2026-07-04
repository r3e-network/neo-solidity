// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "../NativeContracts.sol";
import "../Syscalls.sol";

/**
 * @title Native Oracle — Neo N3 Oracle native contract operations
 */

library NativeOracle {
    address constant ORACLE_CONTRACT = NativeContracts.ORACLE_CONTRACT;
    // ========== Oracle Native Contract ==========
    
    /**
     * @dev Request oracle data
     */
    function requestOracleData(
        string memory url,
        string memory filter,
        string memory callback,
        bytes memory userData,
        uint256 gasForResponse
    ) internal {
        bytes memory params = abi.encode(url, filter, callback, userData, gasForResponse);
        Syscalls.contractCall(NativeContracts.ORACLE_CONTRACT, "request", params);
    }
    
    /**
     * @dev Get oracle price
     */
    function getOraclePrice() internal view returns (uint256) {
        bytes memory result = Syscalls.contractCall(NativeContracts.ORACLE_CONTRACT, "getPrice", "");
        return abi.decode(result, (uint256));
    }
    
    /**
     * @dev Set oracle price
     */
    function setOraclePrice(uint256 price) internal {
        bytes memory params = abi.encode(price);
        Syscalls.contractCall(NativeContracts.ORACLE_CONTRACT, "setPrice", params);
    }

    /**
     * @dev Finish oracle response (oracle nodes)
     */
    function oracleFinish() internal {
        Syscalls.contractCall(NativeContracts.ORACLE_CONTRACT, "finish", "");
    }

    /**
     * @dev Verify oracle response transaction
     */
    function oracleVerify() internal view returns (bool) {
        bytes memory result = Syscalls.contractCall(NativeContracts.ORACLE_CONTRACT, "verify", "");
        return abi.decode(result, (bool));
    }

    /**
     * @dev Get oracle nodes (public keys of registered oracle nodes)
     * Cockatrice upgrade: returns ECPoint[] for currently active oracle nodes.
     */
    function getOracleNodes() internal view returns (bytes[] memory) {
        bytes memory result = Syscalls.contractCall(NativeContracts.ORACLE_CONTRACT, "getOracleNodes", "");
        return abi.decode(result, (bytes[]));
    }

    /**
     * @dev Get oracle requests (iterator over pending/finished requests)
     * Cockatrice upgrade: useful for request status inspection.
     */
    function getOracleRequests() internal view returns (Syscalls.Iterator memory) {
        bytes memory result = Syscalls.contractCall(NativeContracts.ORACLE_CONTRACT, "getRequests", "");
        return abi.decode(result, (Syscalls.Iterator));
    }

    /**
     * @dev Get a specific oracle request by ID
     * Cockatrice upgrade: returns request data for a specific request ID.
     */
    function getOracleRequest(uint256 requestId) internal view returns (bytes memory) {
        bytes memory params = abi.encode(requestId);
        return Syscalls.contractCall(NativeContracts.ORACLE_CONTRACT, "getRequest", params);
    }
    
}
