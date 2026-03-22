// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title Neo N3 Runtime Library
 * @dev Runtime services and utilities for Neo N3 blockchain
 * @author Jimmy <jimmy@r3e.network>
 * 
 * This library provides runtime services including:
 * - Event emission and notification system
 * - Witness verification and authorization
 * - Gas management and optimization
 * - Execution context information
 * - Error handling and debugging
 */

import "../contracts/Syscalls.sol";
import "../contracts/NativeCalls.sol";

library Runtime {
    using Syscalls for *;
    
    // Events
    event RuntimeInitialized();
    event NotificationEmitted(string indexed eventName, bytes data);
    event WitnessVerified(address indexed account, bool success);
    event GasOptimization(uint256 saved, string technique);
    
    // ========== Event and Notification System ==========
    
    /**
     * @dev Emit notification (maps to Runtime.Notify)
     */
    function notify(string memory eventName, bytes memory data) internal {
        // Call Neo Runtime.Notify
        bytes memory notificationData = abi.encode(eventName, data);
        Syscalls.notify(notificationData);

        emit NotificationEmitted(eventName, data);
    }
    
    /**
     * @dev Emit indexed notification with topics
     */
    function notifyIndexed(
        string memory eventName,
        bytes32[] memory topics,
        bytes memory data
    ) internal {
        require(topics.length <= 4, "Runtime: too many topics");
        
        // Encode topics and data together
        bytes memory indexedData = abi.encode(topics, data);
        notify(eventName, indexedData);
    }
    
    /**
     * @dev Emit standard ERC-20 Transfer event
     */
    function notifyTransfer(address from, address to, uint256 amount) internal {
        notify("Transfer", abi.encode(from, to, amount));
    }
    
    /**
     * @dev Emit standard ERC-20 Approval event
     */
    function notifyApproval(address owner, address spender, uint256 amount) internal {
        notify("Approval", abi.encode(owner, spender, amount));
    }
    
    /**
     * @dev Emit NFT Transfer event
     */
    function notifyNFTTransfer(address from, address to, bytes32 tokenId) internal {
        notify("Transfer", abi.encode(from, to, 1, tokenId));
    }
    
    /**
     * @dev Get all notifications for current transaction
     */
    function getNotifications() internal view returns (Syscalls.Notification[] memory) {
        return Syscalls.getNotifications();
    }
    
    /**
     * @dev Get notifications for specific contract
     */
    function getContractNotifications(address contractHash) 
        internal 
        view 
        returns (Syscalls.Notification[] memory) 
    {
        return Syscalls.getNotifications(contractHash);
    }
    
    // ========== Witness and Authorization ==========
    
    /**
     * @dev Check witness for account
     */
    function checkWitness(address account) internal view returns (bool) {
        return Syscalls.checkWitness(account);
    }

    /**
     * @dev Check witness by public key
     */
    function checkWitness(bytes memory publicKey) internal view returns (bool) {
        return Syscalls.checkWitness(publicKey);
    }
    
    /**
     * @dev Require witness or revert
     */
    function requireWitness(address account) internal view {
        require(checkWitness(account), "Runtime: invalid witness");
    }
    
    /**
     * @dev Check multiple witnesses (OR logic)
     */
    function checkAnyWitness(address[] memory accounts) internal view returns (bool) {
        for (uint256 i = 0; i < accounts.length; i++) {
            if (checkWitness(accounts[i])) {
                return true;
            }
        }
        return false;
    }
    
    /**
     * @dev Check multiple witnesses (AND logic)
     */
    function checkAllWitnesses(address[] memory accounts) internal view returns (bool) {
        for (uint256 i = 0; i < accounts.length; i++) {
            if (!checkWitness(accounts[i])) {
                return false;
            }
        }
        return true;
    }
    
    /**
     * @dev Multi-signature witness verification
     */
    function checkMultiSigWitness(
        address[] memory signers,
        uint256 threshold
    ) internal view returns (bool) {
        require(threshold > 0, "Runtime: threshold must be positive");
        require(threshold <= signers.length, "Runtime: threshold exceeds signers");
        
        uint256 validWitnesses = 0;
        
        for (uint256 i = 0; i < signers.length; i++) {
            address signer = signers[i];

            // Prevent duplicate signers from satisfying quorum multiple times.
            for (uint256 j = 0; j < i; j++) {
                require(signers[j] != signer, "Runtime: duplicate signer");
            }

            if (checkWitness(signer)) {
                validWitnesses++;
                if (validWitnesses >= threshold) {
                    return true;
                }
            }
        }
        
        return false;
    }
    
    // ========== Execution Context ==========
    
    /**
     * @dev Get current execution context
     */
    function getExecutionContext() internal view returns (
        address executingContract,
        address callingContract,
        address entryContract,
        uint8 trigger,
        uint256 gasLeft_,
        uint256 invocationCounter
    ) {
        executingContract = Syscalls.getExecutingScriptHash();
        callingContract = Syscalls.getCallingScriptHash();
        entryContract = Syscalls.getEntryScriptHash();
        trigger = Syscalls.getTrigger();
        gasLeft_ = Syscalls.gasLeft();
        invocationCounter = Syscalls.getInvocationCounter();
    }

    /**
     * @dev Get current call flags
     */
    function getCallFlags() internal view returns (uint8) {
        return Syscalls.getCallFlags();
    }

    /**
     * @dev Get current script container
     */
    function getScriptContainer() internal view returns (Syscalls.Transaction memory) {
        return Syscalls.getScriptContainer();
    }

    /**
     * @dev Load script with arguments
     */
    function loadScript(bytes memory script, uint8 callFlags, bytes[] memory args) internal {
        Syscalls.loadScript(script, callFlags, args);
    }
    
    /**
     * @dev Check execution trigger type
     */
    function getTriggerType() internal view returns (uint8) {
        return Syscalls.getTrigger();
    }
    
    /**
     * @dev Check if in application trigger
     */
    function isApplicationTrigger() internal view returns (bool) {
        return getTriggerType() == 0x40;
    }

    /**
     * @dev Check if in verification trigger
     */
    function isVerificationTrigger() internal view returns (bool) {
        return getTriggerType() == 0x20;
    }
    
    // ========== Gas Management ==========
    
    /**
     * @dev Get remaining gas
     */
    function gasLeft() internal view returns (uint256) {
        return Syscalls.gasLeft();
    }
    
    /**
     * @dev Burn gas for optimization
     */
    function burnGas(uint256 amount) internal {
        require(gasLeft() > amount, "Runtime: insufficient gas to burn");
        Syscalls.burnGas(amount);
    }
    
    /**
     * @dev Placeholder for gas-optimized batch execution.
     * @notice NeoVM does not support first-class function callbacks. This stub
     *         exists to preserve API compatibility; it always reverts. Implement
     *         batch logic inline at the call site instead.
     */
    function optimizeGasUsage(
        function() internal /*batchOperation*/,
        uint256 /*expectedSavings*/
    ) internal {
        revert("Runtime: optimizeGasUsage callback unsupported");
    }
    
    /**
     * @dev Check if sufficient gas for operation
     */
    function hasSufficientGas(uint256 requiredGas) internal view returns (bool) {
        return gasLeft() >= requiredGas;
    }
    
    /**
     * @dev Require minimum gas or revert
     */
    function requireGas(uint256 minimumGas) internal view {
        require(hasSufficientGas(minimumGas), "Runtime: insufficient gas");
    }
    
    // ========== Time and Block Operations ==========
    
    /**
     * @dev Get current timestamp
     */
    function getTimestamp() internal view returns (uint256) {
        return Syscalls.getTime();
    }
    
    /**
     * @dev Get current block index
     */
    function getBlockIndex() internal view returns (uint256) {
        return Syscalls.getCurrentIndex();
    }
    
    /**
     * @dev Check if specific time has passed
     */
    function isAfterTime(uint256 timestamp) internal view returns (bool) {
        return getTimestamp() > timestamp;
    }
    
    /**
     * @dev Check if specific block has passed
     */
    function isAfterBlock(uint256 blockIndex) internal view returns (bool) {
        return getBlockIndex() > blockIndex;
    }
    
    /**
     * @dev Calculate time until block
     */
    function timeUntilBlock(uint256 targetBlock) internal view returns (uint256) {
        uint256 currentBlock = getBlockIndex();
        if (targetBlock <= currentBlock) return 0;
        
        uint256 blocksRemaining = targetBlock - currentBlock;
        uint256 averageBlockTime = 15; // 15 seconds per block
        return blocksRemaining * averageBlockTime;
    }
    
    // ========== Logging and Debugging ==========
    
    /**
     * @dev Log message to Neo logs
     */
    function log(string memory message) internal {
        Syscalls.log(message);
    }
    
    /**
     * @dev Log with data
     */
    function logWithData(string memory message, bytes memory data) internal {
        string memory fullMessage = string(abi.encodePacked(message, ": ", _bytesToHex(data)));
        log(fullMessage);
    }
    
    /**
     * @dev Debug assertion
     */
    function assert(bool condition, string memory message) internal view {
        if (!condition) {
            revert(message);
        }
    }
    
    /**
     * @dev Debug checkpoint
     */
    function checkpoint(string memory name) internal {
        log(string(abi.encodePacked("CHECKPOINT: ", name, " at block ", _uint256ToString(getBlockIndex()))));
    }
    
    // ========== Error Handling ==========
    
    /**
     * @dev Safe external call with error handling
     */
    function safeExternalCall(
        address target,
        bytes memory data
    ) internal returns (bool success, bytes memory returnData) {
        require(target != address(0), "Runtime: call to zero address");
        target;
        data;
        log("External call failed: unsupported in standalone Runtime library; use Syscalls.contractCall");
        return (false, "");
    }
    
    /**
     * @dev Placeholder for external call wrapper.
     * @notice Always reverts. Use `Syscalls.contractCall()` for inter-contract
     *         calls on Neo N3.
     */
    function externalCall(address target, bytes memory data) internal returns (bytes memory) {
        target;
        data;
        revert("Runtime: external call unsupported; use Syscalls.contractCall");
    }
    
    // ========== Statistics and Monitoring ==========

    /**
     * @dev Get runtime statistics
     */
    function getStats() internal view returns (
        uint256 currentBlock,
        uint256 invocations
    ) {
        currentBlock = getBlockIndex();
        invocations = Syscalls.getInvocationCounter();
    }
    
    // ========== Utility Functions ==========
    
    /**
     * @dev Convert bytes to hex string
     */
    function _bytesToHex(bytes memory data) private pure returns (string memory) {
        bytes memory alphabet = "0123456789abcdef";
        bytes memory str = new bytes(2 + data.length * 2);
        str[0] = "0";
        str[1] = "x";
        
        for (uint256 i = 0; i < data.length; i++) {
            str[2 + i * 2] = alphabet[uint8(data[i] >> 4)];
            str[3 + i * 2] = alphabet[uint8(data[i] & 0x0f)];
        }
        
        return string(str);
    }
    
    /**
     * @dev Convert uint256 to string
     */
    function _uint256ToString(uint256 value) private pure returns (string memory) {
        if (value == 0) {
            return "0";
        }
        
        uint256 temp = value;
        uint256 digits;
        
        while (temp != 0) {
            digits++;
            temp /= 10;
        }
        
        bytes memory buffer = new bytes(digits);
        
        while (value != 0) {
            digits -= 1;
            buffer[digits] = bytes1(uint8(48 + uint256(value % 10)));
            value /= 10;
        }
        
        return string(buffer);
    }
    
    /**
     * @dev Generate unique ID
     */
    function generateUniqueId() internal view returns (bytes32) {
        return keccak256(abi.encode(
            Syscalls.getExecutingScriptHash(),
            getBlockIndex(),
            getTimestamp(),
            Syscalls.getInvocationCounter()
        ));
    }
    
    /**
     * @dev Create deterministic random seed
     */
    function createRandomSeed(bytes memory entropy) internal view returns (uint256) {
        return uint256(keccak256(abi.encode(
            Syscalls.getCurrentRandom(),
            entropy,
            getTimestamp(),
            getBlockIndex()
        )));
    }
    
    // ========== Performance Optimization ==========
    
    /**
     * @dev Batch notifications for gas efficiency
     */
    function batchNotify(
        string[] memory eventNames,
        bytes[] memory data
    ) internal {
        require(eventNames.length == data.length, "Runtime: array length mismatch");
        require(eventNames.length > 0, "Runtime: empty arrays");
        require(eventNames.length <= 50, "Runtime: too many notifications");
        
        for (uint256 i = 0; i < eventNames.length; i++) {
            notify(eventNames[i], data[i]);
        }
    }
    
    /**
     * @dev Placeholder for conditional gas-gated execution.
     * @notice NeoVM does not support first-class function callbacks. This stub
     *         always reverts. Use `gasLeft()` checks inline instead.
     */
    function executeIfGasAvailable(
        uint256 /*requiredGas*/,
        function() internal /*operation*/
    ) internal {
        revert("Runtime: executeIfGasAvailable callback unsupported");
    }

    /**
     * @dev Placeholder for gas-budgeted loop execution.
     * @notice NeoVM does not support first-class function callbacks. This stub
     *         always reverts. Use a standard `for` loop with `gasLeft()` guards.
     */
    function optimizedLoop(
        uint256 /*iterations*/,
        uint256 /*gasPerIteration*/,
        function(uint256) internal /*loopBody*/
    ) internal {
        revert("Runtime: optimizedLoop callback unsupported");
    }
    
    // ========== Access Control Integration ==========
    
    /**
     * @dev Role-based access control
     * @notice Neo N3 RoleManagement does not expose a generic hasRole method.
     *         This falls back to witness verification. For role-specific checks,
     *         use NativeCalls.getDesignatedByRole() directly.
     */
    function hasRole(address account, bytes32 role) internal view returns (bool) {
        // Neo N3 RoleManagement only supports getDesignatedByRole/designateAsRole,
        // not arbitrary role checks. Fall back to witness verification.
        role; // silence unused parameter warning
        return checkWitness(account);
    }
    
    /**
     * @dev Committee member check
     */
    function isCommitteeMember(address account) internal view returns (bool) {
        return NativeCalls.isCommittee(account);
    }
    
    /**
     * @dev Validator check
     */
    function isValidator(address account) internal view returns (bool) {
        address[] memory validators = NativeCalls.getNextBlockValidators();
        for (uint256 i = 0; i < validators.length; i++) {
            if (validators[i] == account) {
                return true;
            }
        }
        return false;
    }
    
    // ========== Oracle Integration ==========
    
    /**
     * @dev Handle oracle response
     */
    function handleOracleResponse(
        string memory url,
        bytes memory userData,
        uint256 code,
        bytes memory result
    ) internal {
        require(msg.sender == getOracleContract(), "Runtime: unauthorized oracle response");
        
        // Process oracle response
        if (code == 0) {
            // Success
            notify("OracleResponse", abi.encode(url, result, userData));
        } else {
            // Error
            notify("OracleError", abi.encode(url, code, userData));
        }
    }
    
    /**
     * @dev Get oracle contract address
     */
    function getOracleContract() internal pure returns (address) {
        return 0xfe924b7cfe89ddd271abaf7210a80a7e11178758; // Oracle contract hash
    }
    
    // ========== Error Recovery ==========
    
    /**
     * @dev Placeholder for try/fallback pattern.
     * @notice NeoVM does not support first-class function callbacks. This stub
     *         always reverts. Use Solidity `try/catch` blocks instead.
     */
    function tryWithFallback(
        function() internal /*primaryOperation*/,
        function() internal /*fallbackOperation*/
    ) internal {
        revert("Runtime: tryWithFallback callback unsupported");
    }

    /**
     * @dev Placeholder for standalone primary execution.
     * @notice Always reverts. Use Solidity `try/catch` for error recovery.
     */
    function executePrimary(function() internal /*operation*/) internal {
        revert("Runtime: executePrimary unsupported in standalone mode");
    }
    
    /**
     * @dev Placeholder for graceful degradation under gas pressure.
     * @notice NeoVM does not support first-class function callbacks. This stub
     *         always reverts. Use `gasLeft()` checks with if/else branching.
     */
    function degradeGracefully(
        function() internal /*fullOperation*/,
        function() internal /*limitedOperation*/,
        uint256 /*fullOperationGas*/
    ) internal {
        revert("Runtime: degradeGracefully callback unsupported");
    }
    
    // ========== Platform Integration ==========
    
    /**
     * @dev Get platform information
     */
    function getPlatformInfo() internal view returns (
        string memory platform,
        uint32 network,
        uint8 addressVersion
    ) {
        platform = Syscalls.getPlatform();
        network = Syscalls.getNetwork();
        addressVersion = Syscalls.getAddressVersion();
    }
    
    /**
     * @dev Check if running on specific network
     */
    function isMainNet() internal view returns (bool) {
        return Syscalls.getNetwork() == 860833102; // Neo N3 MainNet
    }
    
    function isTestNet() internal view returns (bool) {
        return Syscalls.getNetwork() == 894710606; // Neo N3 TestNet
    }
    
    /**
     * @dev Get invocation counter
     */
    function getInvocationCounter() internal view returns (uint256) {
        return Syscalls.getInvocationCounter();
    }

    /**
     * @dev Get current transaction signers
     */
    function getCurrentSigners() internal view returns (Syscalls.Signer[] memory) {
        return Syscalls.getCurrentSigners();
    }
    
    // ========== Advanced Runtime Features ==========
    
    /**
     * @dev Create execution checkpoint
     */
    function createCheckpoint(string memory name, bytes memory state) internal {
        bytes32 checkpointId = keccak256(abi.encode(name, getTimestamp()));
        
        notify("Checkpoint", abi.encode(
            checkpointId,
            name,
            state,
            getBlockIndex(),
            getTimestamp(),
            gasLeft()
        ));
    }
    
    /**
     * @dev Placeholder for execution measurement.
     * @notice NeoVM does not support first-class function callbacks. This stub
     *         always reverts. Use `gasLeft()` before/after to measure cost.
     */
    function measureExecution(
        function() internal /*operation*/,
        string memory /*operationName*/
    ) internal {
        revert("Runtime: measureExecution callback unsupported");
    }
    
    /**
     * @dev Emergency runtime stop
     */
    function emergencyStop(string memory reason) internal {
        notify("EmergencyStop", abi.encode(
            Syscalls.getExecutingScriptHash(),
            reason,
            getTimestamp(),
            gasLeft()
        ));
        
        // Log critical information
        log(string(abi.encodePacked("EMERGENCY STOP: ", reason)));
        
        revert(reason);
    }
}
