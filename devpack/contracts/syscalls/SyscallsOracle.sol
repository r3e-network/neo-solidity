// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "./SyscallsTypes.sol";
import "./SyscallsBase.sol";

/**
 * @title Syscalls Oracle — Neo N3 Oracle operations
 */

library SyscallsOracle {
    // ========== Oracle System Calls ==========
    
    /**
     * @dev Make oracle request
     */
    function oracleRequest(
        string memory url,
        string memory filter,
        string memory callback,
        bytes memory userData,
        uint256 gasForResponse
    ) internal {
        bytes memory data = abi.encode(url, filter, callback, userData, gasForResponse);
        SyscallsBase.contractCall(SyscallsBase.ORACLE_CONTRACT, "request", data);
    }
    
    /**
     * @dev Get oracle price
     */
    function getOraclePrice() internal view returns (uint256) {
        bytes memory result = SyscallsBase.contractCall(SyscallsBase.ORACLE_CONTRACT, "getPrice", "");
        return abi.decode(result, (uint256));
    }

    /**
     * @dev Get oracle nodes (Cockatrice upgrade)
     */
    function getOracleNodes() internal view returns (bytes[] memory) {
        bytes memory result = SyscallsBase.contractCall(SyscallsBase.ORACLE_CONTRACT, "getOracleNodes", "");
        return abi.decode(result, (bytes[]));
    }

    /**
     * @dev Get oracle requests iterator (Cockatrice upgrade)
     */
    function getOracleRequests() internal view returns (Iterator memory) {
        bytes memory result = SyscallsBase.contractCall(SyscallsBase.ORACLE_CONTRACT, "getRequests", "");
        return abi.decode(result, (Iterator));
    }
    
}
