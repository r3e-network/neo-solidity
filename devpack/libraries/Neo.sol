// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title Neo N3 Blockchain Utilities
 * @dev Comprehensive library for Neo N3 blockchain integration
 * @author Jimmy <jimmy@r3e.network>
 * 
 * This library provides high-level access to Neo N3 blockchain features:
 * - Block and transaction information
 * - Account and balance management
 * - Cryptographic operations
 * - Network state queries
 * - Gas and fee calculations
 */

import "../contracts/Syscalls.sol";
import "../contracts/NativeCalls.sol";

library Neo {
    using Syscalls for *;
    using NativeCalls for *;
    
    // ========== Block Information ==========
    
    /**
     * @dev Get current block information
     */
    function getCurrentBlock() internal view returns (
        uint256 index,
        bytes32 hash,
        uint256 timestamp,
        bytes32 merkleRoot
    ) {
        Syscalls.Block memory currentBlock = Syscalls.getBlock(Syscalls.getCurrentIndex());
        return (currentBlock.index, currentBlock.hash, currentBlock.timestamp, currentBlock.merkleRoot);
    }
    
    /**
     * @dev Get block by index
     */
    function getBlockByIndex(uint256 index) internal view returns (Syscalls.Block memory) {
        return Syscalls.getBlock(index);
    }
    
    /**
     * @dev Get current block height
     */
    function getBlockHeight() internal view returns (uint256) {
        return Syscalls.getCurrentIndex();
    }
    
    /**
     * @dev Get block timestamp
     */
    function getBlockTime() internal view returns (uint256) {
        return Syscalls.getTime();
    }
    
    // ========== Transaction Information ==========
    
    /**
     * @dev Get transaction information
     */
    function getTransaction(bytes32 txHash) internal view returns (
        bytes32 hash,
        uint256 nonce,
        address sender,
        uint256 gasLimit,
        uint256 gasPrice
    ) {
        Syscalls.Transaction memory tx = Syscalls.getTransaction(txHash);
        return (tx.hash, tx.nonce, tx.sender, tx.systemFee + tx.networkFee, tx.networkFee);
    }
    
    /**
     * @dev Get transaction height
     */
    function getTransactionHeight(bytes32 txHash) internal view returns (uint256) {
        return Syscalls.getTransactionHeight(txHash);
    }
    
    /**
     * @dev Check if transaction exists
     */
    function transactionExists(bytes32 txHash) internal view returns (bool) {
        return getTransactionHeight(txHash) > 0;
    }
    
    // ========== Account and Balance Management ==========
    
    /**
     * @dev Get NEO balance of account
     */
    function getNeoBalance(address account) internal view returns (uint256) {
        return NativeCalls.neoBalanceOf(account);
    }
    
    /**
     * @dev Get GAS balance of account
     */
    function getGasBalance(address account) internal view returns (uint256) {
        return NativeCalls.gasBalanceOf(account);
    }
    
    /**
     * @dev Transfer NEO tokens
     */
    function transferNeo(address from, address to, uint256 amount) internal returns (bool) {
        return NativeCalls.neoTransfer(from, to, amount, "");
    }
    
    /**
     * @dev Transfer GAS tokens
     */
    function transferGas(address from, address to, uint256 amount) internal returns (bool) {
        return NativeCalls.gasTransfer(from, to, amount, "");
    }
    
    /**
     * @dev Get account's total portfolio value (NEO + GAS)
     */
    function getPortfolioValue(address account) internal view returns (
        uint256 neoBalance,
        uint256 gasBalance,
        uint256 totalValueInGas
    ) {
        neoBalance = getNeoBalance(account);
        gasBalance = getGasBalance(account);
        
        // Simple approximation: 1 NEO = current gas per block * blocks per day
        uint256 gasPerBlock = NativeCalls.getGasPerBlock();
        uint256 blocksPerDay = 5760; // Approximately 15 second blocks
        uint256 neoValueInGas = neoBalance * gasPerBlock * blocksPerDay;
        
        totalValueInGas = gasBalance + neoValueInGas;
    }
    
    // ========== Cryptographic Operations ==========
    
    /**
     * @dev Verify signature with witness
     */
    function verifyWithWitness(address account) internal view returns (bool) {
        return Syscalls.checkWitness(account);
    }
    
    /**
     * @dev Verify ECDSA signature
     */
    function verifySignature(
        bytes32 hash,
        bytes memory publicKey,
        bytes memory signature
    ) internal pure returns (bool) {
        return Syscalls.verifyWithECDsa(hash, publicKey, signature, 23); // secp256r1
    }
    
    /**
     * @dev SHA256 hash
     */
    function sha256Hash(bytes memory data) internal pure returns (bytes32) {
        return Syscalls.sha256(data);
    }
    
    /**
     * @dev RIPEMD160 hash
     */
    function ripemd160Hash(bytes memory data) internal pure returns (bytes20) {
        return Syscalls.ripemd160(data);
    }
    
    /**
     * @dev Get random number
     */
    function getRandom() internal view returns (uint256) {
        return Syscalls.getCurrentRandom();
    }
    
    // ========== Contract Management ==========
    
    /**
     * @dev Call another contract
     */
    function callContract(
        address contractHash,
        string memory method,
        bytes memory params
    ) internal returns (bytes memory) {
        return Syscalls.contractCall(contractHash, method, params);
    }
    
    /**
     * @dev Deploy new contract
     */
    function deployContract(bytes memory nef, bytes memory manifest) internal returns (address) {
        return NativeCalls.deployContract(nef, manifest);
    }
    
    /**
     * @dev Get contract information
     */
    function getContractInfo(address contractHash) internal view returns (
        string memory name,
        bytes memory script,
        bytes memory manifest
    ) {
        NativeCalls.ContractState memory state = NativeCalls.getContract(contractHash);
        return ("Contract", state.nef, state.manifest);
    }
    
    /**
     * @dev Check if contract exists
     */
    function contractExists(address contractHash) internal view returns (bool) {
        return Syscalls.contractExists(contractHash);
    }
    
    // ========== Network Information ==========
    
    /**
     * @dev Get network magic number
     */
    function getNetworkMagic() internal pure returns (uint32) {
        return Syscalls.getNetwork();
    }
    
    /**
     * @dev Get current gas price
     */
    function getGasPrice() internal view returns (uint256) {
        return NativeCalls.getFeePerByte();
    }
    
    /**
     * @dev Get storage price per byte
     */
    function getStoragePrice() internal view returns (uint256) {
        return NativeCalls.getStoragePrice();
    }
    
    /**
     * @dev Estimate gas for operation
     */
    function estimateGas(bytes memory operation) internal view returns (uint256) {
        // Base gas estimation based on operation size
        uint256 baseGas = 1000000; // 0.01 GAS
        uint256 dataGas = operation.length * 1000; // 0.00001 GAS per byte
        
        return baseGas + dataGas;
    }
    
    // ========== Governance Functions ==========
    
    /**
     * @dev Check if account is committee member
     */
    function isCommittee(address account) internal view returns (bool) {
        return NativeCalls.isCommittee(account);
    }
    
    /**
     * @dev Get committee members
     */
    function getCommittee() internal view returns (address[] memory) {
        return NativeCalls.getCommittee();
    }
    
    /**
     * @dev Get next block validators
     */
    function getValidators() internal view returns (address[] memory) {
        return NativeCalls.getNextBlockValidators();
    }
    
    /**
     * @dev Check if account is validator
     */
    function isValidator(address account) internal view returns (bool) {
        return NativeCalls.isValidator(account);
    }
    
    /**
     * @dev Vote for candidate
     */
    function vote(address account, bytes memory publicKey) internal returns (bool) {
        return NativeCalls.vote(account, publicKey);
    }
    
    // ========== Utility Functions ==========
    
    /**
     * @dev Convert script hash to address
     */
    function scriptHashToAddress(bytes20 scriptHash) internal pure returns (address) {
        return Syscalls.scriptHashToAddress(scriptHash);
    }
    
    /**
     * @dev Convert address to script hash
     */
    function addressToScriptHash(address addr) internal pure returns (bytes20) {
        return Syscalls.addressToScriptHash(addr);
    }
    
    /**
     * @dev Validate Neo address
     */
    function isValidAddress(address addr) internal pure returns (bool) {
        return Syscalls.isValidAddress(addr);
    }
    
    /**
     * @dev Get executing contract hash
     */
    function getExecutingContract() internal view returns (address) {
        return Syscalls.getExecutingScriptHash();
    }
    
    /**
     * @dev Get calling contract hash
     */
    function getCallingContract() internal view returns (address) {
        return Syscalls.getCallingScriptHash();
    }
    
    /**
     * @dev Get entry script hash
     */
    function getEntryScript() internal view returns (address) {
        return Syscalls.getEntryScriptHash();
    }
    
    // ========== Gas Management ==========
    
    /**
     * @dev Get gas left in current execution
     */
    function gasLeft() internal view returns (uint256) {
        return Syscalls.gasLeft();
    }
    
    /**
     * @dev Burn gas (for gas optimization)
     */
    function burnGas(uint256 amount) internal {
        Syscalls.burnGas(amount);
    }
    
    /**
     * @dev Calculate storage cost
     */
    function calculateStorageCost(uint256 bytes_) internal view returns (uint256) {
        return bytes_ * getStoragePrice();
    }
    
    /**
     * @dev Calculate deployment cost
     */
    function calculateDeploymentCost(uint256 nefSize, uint256 manifestSize) 
        internal 
        view 
        returns (uint256) 
    {
        uint256 minimumFee = NativeCalls.getMinimumDeploymentFee();
        uint256 storageCost = calculateStorageCost(nefSize + manifestSize);
        return minimumFee + storageCost;
    }
    
    // ========== Advanced Features ==========
    
    /**
     * @dev Multi-signature verification
     */
    function verifyMultiSig(
        bytes32 hash,
        bytes[] memory publicKeys,
        bytes[] memory signatures,
        uint256 threshold
    ) internal pure returns (bool) {
        require(publicKeys.length >= threshold, "Neo: insufficient public keys");
        require(signatures.length >= threshold, "Neo: insufficient signatures");
        
        uint256 validSignatures = 0;
        
        for (uint256 i = 0; i < signatures.length && validSignatures < threshold; i++) {
            for (uint256 j = 0; j < publicKeys.length; j++) {
                if (verifySignature(hash, publicKeys[j], signatures[i])) {
                    validSignatures++;
                    break;
                }
            }
        }
        
        return validSignatures >= threshold;
    }
    
    /**
     * @dev Time-based operations
     */
    function isAfterBlock(uint256 blockHeight) internal view returns (bool) {
        return getBlockHeight() > blockHeight;
    }
    
    function isAfterTime(uint256 timestamp) internal view returns (bool) {
        return getBlockTime() > timestamp;
    }
    
    /**
     * @dev Network health check
     */
    function getNetworkHealth() internal view returns (
        uint256 blockHeight,
        uint256 committeeSize,
        uint256 validatorCount,
        uint256 gasPerBlock,
        bool isHealthy
    ) {
        blockHeight = getBlockHeight();
        address[] memory committee = getCommittee();
        address[] memory validators = getValidators();
        committeeSize = committee.length;
        validatorCount = validators.length;
        gasPerBlock = NativeCalls.getGasPerBlock();
        
        // Simple health check
        isHealthy = committeeSize >= 7 && validatorCount >= 4 && gasPerBlock > 0;
    }
    
    /**
     * @dev Oracle integration helper
     */
    function requestOracleData(
        string memory url,
        string memory filter,
        string memory callback,
        bytes memory userData
    ) internal {
        uint256 gasForResponse = 100000000; // 1 GAS
        NativeCalls.requestOracleData(url, filter, callback, userData, gasForResponse);
    }
    
    /**
     * @dev Policy information
     */
    function getPolicyInfo() internal view returns (
        uint256 feePerByte,
        uint32 execFeeFactor,
        uint256 storagePrice
    ) {
        feePerByte = NativeCalls.getFeePerByte();
        execFeeFactor = NativeCalls.getExecFeeFactor();
        storagePrice = NativeCalls.getStoragePrice();
    }
    
    /**
     * @dev Emergency functions
     */
    function emergencyBurnGas() internal {
        uint256 gasRemaining = gasLeft();
        if (gasRemaining > 1000000) { // Keep 0.01 GAS for cleanup
            burnGas(gasRemaining - 1000000);
        }
    }
    
    /**
     * @dev Platform information
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
     * @dev Safe contract call with error handling
     */
    function safeContractCall(
        address contractHash,
        string memory method,
        bytes memory params
    ) internal returns (bool success, bytes memory result) {
        try Neo.callContract(contractHash, method, params) returns (bytes memory data) {
            return (true, data);
        } catch {
            return (false, "");
        }
    }
    
    /**
     * @dev Batch contract calls
     */
    function batchContractCalls(
        address[] memory contracts,
        string[] memory methods,
        bytes[] memory params
    ) internal returns (bytes[] memory results) {
        require(contracts.length == methods.length, "Neo: array length mismatch");
        require(contracts.length == params.length, "Neo: array length mismatch");
        require(contracts.length > 0, "Neo: empty arrays");
        require(contracts.length <= 10, "Neo: too many calls");
        
        results = new bytes[](contracts.length);
        
        for (uint256 i = 0; i < contracts.length; i++) {
            (bool success, bytes memory result) = safeContractCall(contracts[i], methods[i], params[i]);
            results[i] = success ? result : "";
        }
    }
    
    /**
     * @dev Calculate optimal gas limit for operation
     */
    function calculateOptimalGasLimit(
        uint256 baseOperations,
        uint256 storageOperations,
        uint256 contractCalls
    ) internal view returns (uint256) {
        uint256 baseGas = baseOperations * 100000; // 0.001 GAS per basic operation
        uint256 storageGas = storageOperations * 1000000; // 0.01 GAS per storage operation
        uint256 callGas = contractCalls * 10000000; // 0.1 GAS per contract call
        
        return baseGas + storageGas + callGas;
    }
}