// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title Neo N3 Storage Library
 * @dev Storage operations for Neo N3 blockchain
 * @author Jimmy <jimmy@r3e.network>
 *
 * `Storage` is a compiler intrinsic: the `neo-devpack-solidity` compiler
 * lowers the members below directly to `System.Storage.*` syscalls. The
 * Solidity bodies in this file exist for editor tooling (signatures/docs)
 * and are never compiled. Only the members listed here are supported;
 * calling anything else fails compilation with a diagnostic.
 *
 * Supported members:
 * - getContext() / getReadOnlyContext() / asReadOnly(context)
 * - put(key, value) / get(key) / remove(key) / find(prefix)
 * - putContractMetadata(name, version, author, extra)
 *
 * Notes:
 * - Neo N3 storage contexts are always private to the owning contract, so a
 *   separate "local" storage API is unnecessary (the former `*Local` members
 *   mapped to syscalls that do not exist on Neo N3 and were removed).
 * - Higher-level helpers (batch operations, prefix counting/clearing, typed
 *   put/get wrappers, ...) were removed because the compiler cannot lower
 *   their loop bodies as intrinsics; implement them in contract code using
 *   `put`/`get`/`remove`/`find` and iterator `next()`/`value()`.
 */

import "../contracts/Syscalls.sol";

library Storage {
    using Syscalls for *;

    // Events for storage operations
    event StorageOperation(string indexed operation, bytes key, uint256 size);

    // ========== Context ==========

    /**
     * @dev Get current storage context
     */
    function getContext() internal view returns (Syscalls.StorageContext memory) {
        return Syscalls.getStorageContext();
    }

    /**
     * @dev Get read-only storage context
     */
    function getReadOnlyContext() internal view returns (Syscalls.StorageContext memory) {
        return Syscalls.getReadOnlyStorageContext();
    }

    /**
     * @dev Convert storage context to read-only
     */
    function asReadOnly(Syscalls.StorageContext memory context)
        internal
        view
        returns (Syscalls.StorageContext memory)
    {
        return Syscalls.storageAsReadOnly(context);
    }

    // ========== Basic Storage Operations ==========

    /**
     * @dev Store value by key
     */
    function put(bytes memory key, bytes memory value) internal {
        Syscalls.StorageContext memory context = getContext();
        Syscalls.storagePut(context, key, value);

        emit StorageOperation("PUT", key, value.length);
    }

    /**
     * @dev Get value by key
     */
    function get(bytes memory key) internal view returns (bytes memory) {
        Syscalls.StorageContext memory context = getReadOnlyContext();
        return Syscalls.storageGet(context, key);
    }

    /**
     * @dev Remove value by key
     */
    function remove(bytes memory key) internal {
        Syscalls.StorageContext memory context = getContext();
        Syscalls.storageDelete(context, key);

        emit StorageOperation("DELETE", key, 0);
    }

    // ========== Iterator Operations ==========

    /**
     * @dev Find all keys with prefix
     */
    function find(bytes memory prefix) internal view returns (Iterator memory) {
        Syscalls.StorageContext memory context = getReadOnlyContext();
        return Syscalls.storageFind(context, prefix);
    }

    // ========== Neo-Specific Extensions ==========

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
}
