// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title Neo N3 Storage Library
 * @dev Advanced storage operations for Neo N3 blockchain
 * @author Jimmy <jimmy@r3e.network>
 * 
 * This library provides enhanced storage capabilities:
 * - Context-aware storage operations
 * - Iterator support for range queries
 * - Batch operations for efficiency
 * - Storage usage tracking
 * - Advanced key management
 */

import "../contracts/Syscalls.sol";

library Storage {
    using Syscalls for *;
    
    // Storage configuration
    struct Config {
        bool initialized;
        address contextContract;
        uint256 totalOperations;
        uint256 totalBytes;
        mapping(bytes => bool) keyExists;
    }
    
    // Global storage configuration
    Config private _config;
    
    // Events for storage operations
    event StorageInitialized(address indexed contract_);
    event StorageOperation(string indexed operation, bytes key, uint256 size);
    event BatchStorageOperation(uint256 operations, uint256 totalSize);
    
    // ========== Initialization ==========
    
    /**
     * @dev Initialize storage context
     */
    function initializeContext() internal {
        if (!_config.initialized) {
            _config.contextContract = address(this);
            _config.initialized = true;
            emit StorageInitialized(address(this));
        }
    }
    
    /**
     * @dev Set storage context to specific contract
     */
    function setContext(address contractHash) internal {
        _config.contextContract = contractHash;
        _config.initialized = true;
    }
    
    /**
     * @dev Get current storage context
     */
    function getContext() internal view returns (Syscalls.StorageContext memory) {
        require(_config.initialized, "Storage: not initialized");
        return Syscalls.getStorageContext();
    }
    
    /**
     * @dev Get read-only storage context
     */
    function getReadOnlyContext() internal view returns (Syscalls.StorageContext memory) {
        return Syscalls.getReadOnlyStorageContext();
    }
    
    // ========== Basic Storage Operations ==========
    
    /**
     * @dev Store value by key
     */
    function put(bytes memory key, bytes memory value) internal {
        require(_config.initialized, "Storage: not initialized");
        
        Syscalls.StorageContext memory context = getContext();
        Syscalls.storagePut(context, key, value);
        
        // Track usage
        _config.totalOperations++;
        _config.totalBytes += value.length;
        _config.keyExists[key] = true;
        
        emit StorageOperation("PUT", key, value.length);
    }
    
    /**
     * @dev Get value by key
     */
    function get(bytes memory key) internal view returns (bytes memory) {
        require(_config.initialized, "Storage: not initialized");
        
        Syscalls.StorageContext memory context = getReadOnlyContext();
        return Syscalls.storageGet(context, key);
    }
    
    /**
     * @dev Delete value by key
     */
    function delete(bytes memory key) internal {
        require(_config.initialized, "Storage: not initialized");
        
        Syscalls.StorageContext memory context = getContext();
        Syscalls.storageDelete(context, key);
        
        // Track usage
        _config.totalOperations++;
        _config.keyExists[key] = false;
        
        emit StorageOperation("DELETE", key, 0);
    }
    
    /**
     * @dev Check if key exists
     */
    function exists(bytes memory key) internal view returns (bool) {
        bytes memory value = get(key);
        return value.length > 0;
    }
    
    // ========== Iterator Operations ==========
    
    /**
     * @dev Find all keys with prefix
     */
    function find(bytes memory prefix) internal view returns (Iterator memory) {
        require(_config.initialized, "Storage: not initialized");
        
        Syscalls.StorageContext memory context = getReadOnlyContext();
        return Syscalls.storageFind(context, prefix);
    }
    
    /**
     * @dev Get all values with prefix
     */
    function findValues(bytes memory prefix) internal view returns (bytes[] memory values) {
        Iterator memory iterator = find(prefix);
        values = new bytes[](1000); // Max 1000 results
        uint256 count = 0;
        
        while (iterator.next() && count < 1000) {
            values[count] = iterator.value();
            count++;
        }
        
        // Resize array to actual count
        assembly {
            mstore(values, count)
        }
    }
    
    /**
     * @dev Get all keys with prefix
     */
    function findKeys(bytes memory prefix) internal view returns (bytes[] memory keys) {
        Iterator memory iterator = find(prefix);
        keys = new bytes[](1000); // Max 1000 results
        uint256 count = 0;
        
        while (iterator.next() && count < 1000) {
            keys[count] = iterator.currentKey;
            count++;
        }
        
        // Resize array to actual count
        assembly {
            mstore(keys, count)
        }
    }
    
    /**
     * @dev Count entries with prefix
     */
    function count(bytes memory prefix) internal view returns (uint256) {
        Iterator memory iterator = find(prefix);
        uint256 total = 0;
        
        while (iterator.next()) {
            total++;
        }
        
        return total;
    }
    
    // ========== Batch Operations ==========
    
    /**
     * @dev Batch put operations
     */
    function batchPut(bytes[] memory keys, bytes[] memory values) internal {
        require(keys.length == values.length, "Storage: array length mismatch");
        require(keys.length > 0, "Storage: empty arrays");
        require(keys.length <= 100, "Storage: too many operations");
        
        uint256 totalSize = 0;
        
        for (uint256 i = 0; i < keys.length; i++) {
            put(keys[i], values[i]);
            totalSize += values[i].length;
        }
        
        emit BatchStorageOperation(keys.length, totalSize);
    }
    
    /**
     * @dev Batch get operations
     */
    function batchGet(bytes[] memory keys) internal view returns (bytes[] memory values) {
        require(keys.length > 0, "Storage: empty array");
        require(keys.length <= 100, "Storage: too many operations");
        
        values = new bytes[](keys.length);
        
        for (uint256 i = 0; i < keys.length; i++) {
            values[i] = get(keys[i]);
        }
    }
    
    /**
     * @dev Batch delete operations
     */
    function batchDelete(bytes[] memory keys) internal {
        require(keys.length > 0, "Storage: empty array");
        require(keys.length <= 100, "Storage: too many operations");
        
        for (uint256 i = 0; i < keys.length; i++) {
            delete(keys[i]);
        }
        
        emit BatchStorageOperation(keys.length, 0);
    }
    
    // ========== Advanced Key Management ==========
    
    /**
     * @dev Generate Solidity-compatible mapping key
     */
    function generateMappingKey(bytes memory slot, bytes memory key) internal pure returns (bytes memory) {
        return abi.encode(keccak256(abi.encode(key, slot)));
    }
    
    /**
     * @dev Generate array element key
     */
    function generateArrayKey(bytes memory slot, uint256 index) internal pure returns (bytes memory) {
        bytes32 baseSlot = keccak256(slot);
        return abi.encode(bytes32(uint256(baseSlot) + index));
    }
    
    /**
     * @dev Generate nested mapping key
     */
    function generateNestedMappingKey(
        bytes memory slot,
        bytes memory key1,
        bytes memory key2
    ) internal pure returns (bytes memory) {
        bytes32 innerSlot = keccak256(abi.encode(key1, slot));
        return abi.encode(keccak256(abi.encode(key2, innerSlot)));
    }
    
    /**
     * @dev Generate prefixed key
     */
    function prefixKey(string memory prefix, bytes memory key) internal pure returns (bytes memory) {
        return abi.encodePacked(prefix, key);
    }
    
    // ========== Storage Patterns ==========
    
    /**
     * @dev Store mapping value (Solidity mapping compatibility)
     */
    function setMapping(
        bytes memory slot,
        bytes memory key,
        bytes memory value
    ) internal {
        bytes memory storageKey = generateMappingKey(slot, key);
        put(storageKey, value);
    }
    
    /**
     * @dev Get mapping value
     */
    function getMapping(bytes memory slot, bytes memory key) internal view returns (bytes memory) {
        bytes memory storageKey = generateMappingKey(slot, key);
        return get(storageKey);
    }
    
    /**
     * @dev Store array element
     */
    function setArrayElement(
        bytes memory slot,
        uint256 index,
        bytes memory value
    ) internal {
        bytes memory storageKey = generateArrayKey(slot, index);
        put(storageKey, value);
    }
    
    /**
     * @dev Get array element
     */
    function getArrayElement(bytes memory slot, uint256 index) internal view returns (bytes memory) {
        bytes memory storageKey = generateArrayKey(slot, index);
        return get(storageKey);
    }
    
    /**
     * @dev Store array length
     */
    function setArrayLength(bytes memory slot, uint256 length) internal {
        put(slot, abi.encode(length));
    }
    
    /**
     * @dev Get array length
     */
    function getArrayLength(bytes memory slot) internal view returns (uint256) {
        bytes memory data = get(slot);
        if (data.length == 0) return 0;
        return abi.decode(data, (uint256));
    }
    
    // ========== Storage Analytics ==========
    
    /**
     * @dev Get storage usage statistics
     */
    function getUsage() internal view returns (uint256) {
        return _config.totalBytes;
    }
    
    /**
     * @dev Get total operations count
     */
    function getOperationCount() internal view returns (uint256) {
        return _config.totalOperations;
    }
    
    /**
     * @dev Estimate storage cost for operation
     */
    function estimateCost(uint256 dataSize) internal view returns (uint256) {
        uint256 storagePrice = NativeCalls.getStoragePrice();
        return dataSize * storagePrice;
    }
    
    /**
     * @dev Get storage efficiency ratio
     */
    function getEfficiencyRatio() internal view returns (uint256) {
        if (_config.totalOperations == 0) return 0;
        return _config.totalBytes / _config.totalOperations;
    }
    
    // ========== Utility Functions ==========
    
    /**
     * @dev Clear all storage with prefix (expensive operation)
     */
    function clearPrefix(bytes memory prefix) internal {
        bytes[] memory keys = findKeys(prefix);
        batchDelete(keys);
    }
    
    /**
     * @dev Copy storage range
     */
    function copyRange(
        bytes memory sourcePrefix,
        bytes memory targetPrefix
    ) internal {
        bytes[] memory keys = findKeys(sourcePrefix);
        bytes[] memory values = findValues(sourcePrefix);
        
        require(keys.length == values.length, "Storage: data corruption");
        
        bytes[] memory newKeys = new bytes[](keys.length);
        
        for (uint256 i = 0; i < keys.length; i++) {
            // Replace prefix
            newKeys[i] = abi.encodePacked(targetPrefix, keys[i][sourcePrefix.length:]);
        }
        
        batchPut(newKeys, values);
    }
    
    /**
     * @dev Validate storage key format
     */
    function isValidKey(bytes memory key) internal pure returns (bool) {
        return key.length > 0 && key.length <= 64; // Neo storage key limit
    }
    
    /**
     * @dev Compress storage value using RLE compression
     */
    function compress(bytes memory data) internal pure returns (bytes memory) {
        if (data.length < 4) {
            return data; // Too small to compress effectively
        }
        
        // Run-length encoding compression
        bytes memory compressed = new bytes(data.length * 2); // Worst case size
        uint256 compressedIndex = 0;
        uint256 i = 0;
        
        while (i < data.length) {
            bytes1 currentByte = data[i];
            uint256 runLength = 1;
            
            // Count consecutive identical bytes
            while (i + runLength < data.length && data[i + runLength] == currentByte && runLength < 255) {
                runLength++;
            }
            
            // Store run-length encoded data
            if (runLength >= 4 || currentByte == 0x00) {
                // Use RLE for runs of 4+ or zeros
                compressed[compressedIndex++] = 0xFF; // Escape byte
                compressed[compressedIndex++] = bytes1(uint8(runLength));
                compressed[compressedIndex++] = currentByte;
            } else {
                // Store literal bytes
                for (uint256 j = 0; j < runLength; j++) {
                    compressed[compressedIndex++] = currentByte;
                }
            }
            
            i += runLength;
        }
        
        // Resize to actual compressed size
        bytes memory result = new bytes(compressedIndex);
        for (uint256 k = 0; k < compressedIndex; k++) {
            result[k] = compressed[k];
        }
        
        return result;
    }
    
    /**
     * @dev Decompress RLE-compressed storage value
     */
    function decompress(bytes memory compressedData) internal pure returns (bytes memory) {
        if (compressedData.length == 0) {
            return compressedData;
        }
        
        // Estimate maximum decompressed size
        bytes memory decompressed = new bytes(compressedData.length * 255); // Worst case
        uint256 decompressedIndex = 0;
        uint256 i = 0;
        
        while (i < compressedData.length) {
            if (compressedData[i] == 0xFF && i + 2 < compressedData.length) {
                // RLE sequence
                uint256 runLength = uint8(compressedData[i + 1]);
                bytes1 value = compressedData[i + 2];
                
                for (uint256 j = 0; j < runLength; j++) {
                    decompressed[decompressedIndex++] = value;
                }
                
                i += 3;
            } else {
                // Literal byte
                decompressed[decompressedIndex++] = compressedData[i];
                i++;
            }
        }
        
        // Resize to actual decompressed size
        bytes memory result = new bytes(decompressedIndex);
        for (uint256 k = 0; k < decompressedIndex; k++) {
            result[k] = decompressed[k];
        }
        
        return result;
    }
    
    // ========== Advanced Storage Patterns ==========
    
    /**
     * @dev Atomic storage update
     */
    function atomicUpdate(
        bytes memory key,
        function(bytes memory) internal pure returns (bytes memory) updateFunction
    ) internal {
        bytes memory currentValue = get(key);
        bytes memory newValue = updateFunction(currentValue);
        put(key, newValue);
    }
    
    /**
     * @dev Conditional storage update
     */
    function conditionalPut(
        bytes memory key,
        bytes memory value,
        function(bytes memory) internal pure returns (bool) condition
    ) internal returns (bool) {
        bytes memory currentValue = get(key);
        
        if (condition(currentValue)) {
            put(key, value);
            return true;
        }
        
        return false;
    }
    
    /**
     * @dev Store with expiration (using block height)
     */
    function putWithExpiration(
        bytes memory key,
        bytes memory value,
        uint256 expirationBlock
    ) internal {
        bytes memory wrappedValue = abi.encode(value, expirationBlock);
        put(key, wrappedValue);
    }
    
    /**
     * @dev Get value with expiration check
     */
    function getWithExpiration(bytes memory key) internal view returns (bytes memory) {
        bytes memory wrappedValue = get(key);
        if (wrappedValue.length == 0) return "";
        
        (bytes memory value, uint256 expirationBlock) = abi.decode(wrappedValue, (bytes, uint256));
        
        if (block.number > expirationBlock) {
            return ""; // Expired
        }
        
        return value;
    }
    
    /**
     * @dev Clean expired entries
     */
    function cleanExpired(bytes memory prefix) internal {
        Iterator memory iterator = find(prefix);
        bytes[] memory expiredKeys = new bytes[](100);
        uint256 expiredCount = 0;
        
        while (iterator.next() && expiredCount < 100) {
            bytes memory value = iterator.value();
            if (value.length >= 32) {
                (, uint256 expirationBlock) = abi.decode(value, (bytes, uint256));
                if (block.number > expirationBlock) {
                    expiredKeys[expiredCount] = iterator.currentKey;
                    expiredCount++;
                }
            }
        }
        
        // Resize and delete expired keys
        assembly {
            mstore(expiredKeys, expiredCount)
        }
        
        if (expiredCount > 0) {
            batchDelete(expiredKeys);
        }
    }
    
    // ========== Storage Optimization ==========
    
    /**
     * @dev Pack multiple values into single storage slot
     */
    function packValues(bytes[] memory values) internal pure returns (bytes memory) {
        return abi.encode(values);
    }
    
    /**
     * @dev Unpack multiple values from single storage slot
     */
    function unpackValues(bytes memory packedData) internal pure returns (bytes[] memory) {
        return abi.decode(packedData, (bytes[]));
    }
    
    /**
     * @dev Store packed values
     */
    function putPacked(bytes memory key, bytes[] memory values) internal {
        bytes memory packedData = packValues(values);
        put(key, packedData);
    }
    
    /**
     * @dev Get unpacked values
     */
    function getPacked(bytes memory key) internal view returns (bytes[] memory) {
        bytes memory packedData = get(key);
        if (packedData.length == 0) {
            return new bytes[](0);
        }
        return unpackValues(packedData);
    }
    
    // ========== Specialized Storage Types ==========
    
    /**
     * @dev Store uint256 value
     */
    function putUint256(bytes memory key, uint256 value) internal {
        put(key, abi.encode(value));
    }
    
    /**
     * @dev Get uint256 value
     */
    function getUint256(bytes memory key) internal view returns (uint256) {
        bytes memory data = get(key);
        if (data.length == 0) return 0;
        return abi.decode(data, (uint256));
    }
    
    /**
     * @dev Store address value
     */
    function putAddress(bytes memory key, address value) internal {
        put(key, abi.encode(value));
    }
    
    /**
     * @dev Get address value
     */
    function getAddress(bytes memory key) internal view returns (address) {
        bytes memory data = get(key);
        if (data.length == 0) return address(0);
        return abi.decode(data, (address));
    }
    
    /**
     * @dev Store string value
     */
    function putString(bytes memory key, string memory value) internal {
        put(key, bytes(value));
    }
    
    /**
     * @dev Get string value
     */
    function getString(bytes memory key) internal view returns (string memory) {
        bytes memory data = get(key);
        return string(data);
    }
    
    /**
     * @dev Store boolean value
     */
    function putBool(bytes memory key, bool value) internal {
        put(key, abi.encode(value));
    }
    
    /**
     * @dev Get boolean value
     */
    function getBool(bytes memory key) internal view returns (bool) {
        bytes memory data = get(key);
        if (data.length == 0) return false;
        return abi.decode(data, (bool));
    }
    
    // ========== Storage Security ==========
    
    /**
     * @dev Secure storage with checksum
     */
    function putSecure(bytes memory key, bytes memory value) internal {
        bytes32 checksum = keccak256(value);
        bytes memory secureValue = abi.encode(value, checksum);
        put(key, secureValue);
    }
    
    /**
     * @dev Get secure storage with validation
     */
    function getSecure(bytes memory key) internal view returns (bytes memory) {
        bytes memory secureValue = get(key);
        if (secureValue.length == 0) return "";
        
        (bytes memory value, bytes32 checksum) = abi.decode(secureValue, (bytes, bytes32));
        
        require(keccak256(value) == checksum, "Storage: data corruption detected");
        return value;
    }
    
    /**
     * @dev Access control for storage keys
     */
    function putWithAccess(
        bytes memory key,
        bytes memory value,
        address requiredSigner
    ) internal {
        require(Runtime.checkWitness(requiredSigner), "Storage: unauthorized access");
        put(key, value);
    }
    
    // ========== Storage Migration ==========
    
    /**
     * @dev Migrate storage from old key format to new
     */
    function migrateKeys(
        bytes memory oldPrefix,
        bytes memory newPrefix,
        function(bytes memory) internal pure returns (bytes memory) keyTransform
    ) internal {
        bytes[] memory oldKeys = findKeys(oldPrefix);
        bytes[] memory values = findValues(oldPrefix);
        
        require(oldKeys.length == values.length, "Storage: migration data mismatch");
        
        // Create new keys and store values
        for (uint256 i = 0; i < oldKeys.length; i++) {
            bytes memory newKey = keyTransform(oldKeys[i]);
            put(newKey, values[i]);
        }
        
        // Clean old keys
        batchDelete(oldKeys);
    }
    
    /**
     * @dev Backup storage range
     */
    function backup(bytes memory prefix, bytes memory backupPrefix) internal {
        copyRange(prefix, backupPrefix);
    }
    
    /**
     * @dev Restore storage from backup
     */
    function restore(bytes memory backupPrefix, bytes memory targetPrefix) internal {
        copyRange(backupPrefix, targetPrefix);
    }
    
    // ========== Neo-Specific Extensions ==========
    
    /**
     * @dev Store with Neo-compatible key encoding
     */
    function putNeoKey(bytes20 key, bytes memory value) internal {
        put(abi.encode(key), value);
    }
    
    /**
     * @dev Get with Neo-compatible key encoding
     */
    function getNeoKey(bytes20 key) internal view returns (bytes memory) {
        return get(abi.encode(key));
    }
    
    /**
     * @dev Store contract metadata
     */
    function putContractMetadata(
        string memory name,
        string memory version,
        string memory author,
        bytes memory extra
    ) internal {
        bytes memory metadata = abi.encode(name, version, author, extra, block.timestamp);
        put("__CONTRACT_METADATA__", metadata);
    }
    
    /**
     * @dev Get contract metadata
     */
    function getContractMetadata() internal view returns (
        string memory name,
        string memory version,
        string memory author,
        bytes memory extra,
        uint256 timestamp
    ) {
        bytes memory metadata = get("__CONTRACT_METADATA__");
        if (metadata.length == 0) {
            return ("", "", "", "", 0);
        }
        return abi.decode(metadata, (string, string, string, bytes, uint256));
    }
}