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

library Runtime {
    using Syscalls for *;
    
    // Runtime configuration
    struct Config {
        bool initialized;
        uint256 notificationCount;
        uint256 totalGasUsed;
        mapping(string => uint256) eventCounts;
    }
    
    // Global runtime configuration
    Config private _config;
    
    // Events
    event RuntimeInitialized();
    event NotificationEmitted(string indexed eventName, bytes data);
    event WitnessVerified(address indexed account, bool success);
    event GasOptimization(uint256 saved, string technique);
    
    // ========== Initialization ==========
    
    /**
     * @dev Initialize runtime services
     */
    function initializeServices() internal {
        if (!_config.initialized) {
            _config.initialized = true;
            emit RuntimeInitialized();
        }
    }
    
    /**
     * @dev Check if runtime is initialized
     */
    function isInitialized() internal view returns (bool) {
        return _config.initialized;
    }
    
    // ========== Event and Notification System ==========
    
    /**
     * @dev Emit notification (maps to Runtime.Notify)
     */
    function notify(string memory eventName, bytes memory data) internal {
        require(_config.initialized, "Runtime: not initialized");
        
        // Call Neo Runtime.Notify
        bytes memory notificationData = abi.encode(eventName, data);
        Syscalls.notify(notificationData);
        
        // Track statistics
        _config.notificationCount++;
        _config.eventCounts[eventName]++;
        
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
        return Syscalls.getNotifications(address(0));
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
        bool success = Syscalls.checkWitness(account);
        emit WitnessVerified(account, success);
        return success;
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
            if (checkWitness(signers[i])) {
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
        Syscalls.TriggerType trigger,
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
     * @dev Check execution trigger type
     */
    function getTriggerType() internal view returns (Syscalls.TriggerType) {
        return Syscalls.getTrigger();
    }
    
    /**
     * @dev Check if in application trigger
     */
    function isApplicationTrigger() internal view returns (bool) {
        return getTriggerType() == Syscalls.TriggerType.Application;
    }
    
    /**
     * @dev Check if in verification trigger
     */
    function isVerificationTrigger() internal view returns (bool) {
        return getTriggerType() == Syscalls.TriggerType.Verification;
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
     * @dev Optimize gas usage with batching
     */
    function optimizeGasUsage(
        function() internal batchOperation,
        uint256 expectedSavings
    ) internal {
        uint256 gasBefore = gasLeft();
        batchOperation();
        uint256 gasAfter = gasLeft();
        
        uint256 actualUsage = gasBefore - gasAfter;
        if (actualUsage < expectedSavings) {
            emit GasOptimization(expectedSavings - actualUsage, "batching");
        }
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
        
        try Runtime.externalCall(target, data) returns (bytes memory result) {
            return (true, result);
        } catch Error(string memory reason) {
            log(string(abi.encodePacked("External call failed: ", reason)));
            return (false, bytes(reason));
        } catch {
            log("External call failed: unknown error");
            return (false, "");
        }
    }
    
    /**
     * @dev External call wrapper for try/catch
     */
    function externalCall(address target, bytes memory data) external returns (bytes memory) {
        (bool success, bytes memory result) = target.call(data);
        require(success, "Runtime: external call failed");
        return result;
    }
    
    // ========== Statistics and Monitoring ==========
    
    /**
     * @dev Get runtime statistics
     */
    function getStats() internal view returns (
        uint256 notifications,
        uint256 gasUsed,
        uint256 currentBlock,
        uint256 invocations
    ) {
        notifications = _config.notificationCount;
        gasUsed = _config.totalGasUsed;
        currentBlock = getBlockIndex();
        invocations = Syscalls.getInvocationCounter();
    }
    
    /**
     * @dev Get event count for specific event
     */
    function getEventCount(string memory eventName) internal view returns (uint256) {
        return _config.eventCounts[eventName];
    }
    
    /**
     * @dev Reset statistics (admin only)
     */
    function resetStats() internal {
        _config.notificationCount = 0;
        _config.totalGasUsed = 0;
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
     * @dev Conditional execution based on gas available
     */
    function executeIfGasAvailable(
        uint256 requiredGas,
        function() internal operation
    ) internal {
        if (gasLeft() >= requiredGas) {
            operation();
        }
    }
    
    /**
     * @dev Gas-optimized loop execution
     */
    function optimizedLoop(
        uint256 iterations,
        uint256 gasPerIteration,
        function(uint256) internal loopBody
    ) internal {
        uint256 maxIterations = gasLeft() / gasPerIteration;
        uint256 actualIterations = iterations > maxIterations ? maxIterations : iterations;
        
        for (uint256 i = 0; i < actualIterations; i++) {
            loopBody(i);
        }
        
        if (actualIterations < iterations) {
            log(string(abi.encodePacked(
                "Loop truncated: ", 
                _uint256ToString(actualIterations),
                " of ",
                _uint256ToString(iterations),
                " iterations"
            )));
        }
    }
    
    // ========== Access Control Integration ==========
    
    /**
     * @dev Role-based access control
     */
    function hasRole(address account, bytes32 role) internal view returns (bool) {
        // Integrate with RoleManagement native contract
        address roleManagement = 0x49cf4e5378ffcd4dec034fd98a174c5491e395e2;
        
        bytes memory params = abi.encode(account, bytes1(uint8(uint256(role))));
        bytes memory result = Syscalls.contractCall(roleManagement, "hasRole", params);
        
        if (result.length > 0) {
            return abi.decode(result, (bool));
        }
        
        // Fallback to witness check
        return checkWitness(account);
    }
    
    /**
     * @dev Committee member check
     */
    function isCommitteeMember(address account) internal view returns (bool) {
        // Check if account is in committee
        address[] memory committee = Syscalls.getCommittee();
        for (uint256 i = 0; i < committee.length; i++) {
            if (committee[i] == account) {
                return true;
            }
        }
        return false;
    }
    
    /**
     * @dev Validator check
     */
    function isValidator(address account) internal view returns (bool) {
        address[] memory validators = Syscalls.getNextBlockValidators();
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
     * @dev Try operation with fallback
     */
    function tryWithFallback(
        function() internal primaryOperation,
        function() internal fallbackOperation
    ) internal {
        try Runtime.executePrimary(primaryOperation) {
            // Success
        } catch {
            fallbackOperation();
        }
    }
    
    /**
     * @dev Execute primary operation (for try/catch)
     */
    function executePrimary(function() internal operation) external {
        operation();
    }
    
    /**
     * @dev Graceful degradation on low gas
     */
    function degradeGracefully(
        function() internal fullOperation,
        function() internal limitedOperation,
        uint256 fullOperationGas
    ) internal {
        if (gasLeft() >= fullOperationGas) {
            fullOperation();
        } else {
            limitedOperation();
            log("Graceful degradation: using limited operation due to low gas");
        }
    }
    
    // ========== Platform Integration ==========
    
    /**
     * @dev Get platform information
     */
    function getPlatformInfo() internal pure returns (
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
    function isMainNet() internal pure returns (bool) {
        return Syscalls.getNetwork() == 860833102; // Neo N3 MainNet
    }
    
    function isTestNet() internal pure returns (bool) {
        return Syscalls.getNetwork() == 894710606; // Neo N3 TestNet
    }
    
    /**
     * @dev Get invocation counter
     */
    function getInvocationCounter() internal view returns (uint256) {
        return Syscalls.getInvocationCounter();
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
     * @dev Measure execution time
     */
    function measureExecution(
        function() internal operation,
        string memory operationName
    ) internal {
        uint256 startTime = getTimestamp();
        uint256 startGas = gasLeft();
        
        operation();
        
        uint256 endTime = getTimestamp();
        uint256 endGas = gasLeft();
        
        notify("ExecutionMeasured", abi.encode(
            operationName,
            endTime - startTime,  // Time elapsed
            startGas - endGas,    // Gas consumed
            getBlockIndex()
        ));
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