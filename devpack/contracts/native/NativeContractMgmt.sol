// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "./NativeTypes.sol";
import "../NativeContracts.sol";
import "../Syscalls.sol";

/**
 * @title Native ContractMgmt — Neo N3 ContractMgmt native contract operations
 */

library NativeContractMgmt {
    // ========== Contract Management Native Contract ==========
    
    /**
     * @dev Deploy new contract
     */
    function deployContract(bytes memory nef, bytes memory manifest) internal returns (address) {
        bytes memory params = abi.encode(nef, manifest);
        bytes memory result = Syscalls.contractCall(NativeContracts.CONTRACT_MANAGEMENT, "deploy", params);
        return abi.decode(result, (address));
    }

    /**
     * @dev Deploy new contract and pass deployment data to `_deploy(data, false)`
     *
     * The `data` parameter is forwarded to the deployed contract's `_deploy(data, update)` entrypoint
     * (with `update == false`). For neo-devpack-solidity-compiled contracts with parameterised constructors,
     * this is typically a JSON-encoded array (e.g. `[7]`) or StdLib.serialize(...) bytes.
     */
    function deployContract(bytes memory nef, bytes memory manifest, bytes memory data)
        internal
        returns (address)
    {
        bytes memory params = abi.encode(nef, manifest, data);
        bytes memory result = Syscalls.contractCall(NativeContracts.CONTRACT_MANAGEMENT, "deploy", params);
        return abi.decode(result, (address));
    }
    
    /**
     * @dev Update contract
     */
    function updateContract(bytes memory nef, bytes memory manifest) internal {
        bytes memory params = abi.encode(nef, manifest);
        Syscalls.contractCall(NativeContracts.CONTRACT_MANAGEMENT, "update", params);
    }

    /**
     * @dev Update contract and pass update data to `_deploy(data, true)`
     *
     * The `data` parameter is forwarded to the updated contract's `_deploy(data, update)` entrypoint
     * (with `update == true`). This can be used for migration flows when a contract implements custom
     * `_deploy` logic.
     */
    function updateContract(bytes memory nef, bytes memory manifest, bytes memory data) internal {
        bytes memory params = abi.encode(nef, manifest, data);
        Syscalls.contractCall(NativeContracts.CONTRACT_MANAGEMENT, "update", params);
    }
    
    /**
     * @dev Destroy contract
     */
    function destroyContract() internal {
        Syscalls.contractCall(NativeContracts.CONTRACT_MANAGEMENT, "destroy", "");
    }
    
    /**
     * @dev Get contract by hash
     */
    function getContract(address hash) internal view returns (ContractState memory) {
        bytes memory params = abi.encode(hash);
        bytes memory result = Syscalls.contractCall(NativeContracts.CONTRACT_MANAGEMENT, "getContract", params);
        return abi.decode(result, (ContractState));
    }

    /**
     * @dev Get contract by id
     */
    function getContractById(int256 id) internal view returns (ContractState memory) {
        bytes memory params = abi.encode(id);
        bytes memory result = Syscalls.contractCall(NativeContracts.CONTRACT_MANAGEMENT, "getContractById", params);
        return abi.decode(result, (ContractState));
    }
    
    /**
     * @dev List all contracts
     */
    function listContracts() internal view returns (Syscalls.Iterator memory) {
        // Calls ContractManagement.listContracts() which returns a Neo iterator
        // of deployed contract hashes.
        //
        // Consume it via:
        //   while (it.next()) { bytes memory hash = it.value(); ... }
        bytes memory params;
        bytes memory result = Syscalls.contractCall(NativeContracts.CONTRACT_MANAGEMENT, "listContracts", params);
        return abi.decode(result, (Syscalls.Iterator));
    }
    
    /**
     * @dev Check if contract has method
     */
    function hasMethod(address hash, string memory method, uint8 paramCount) 
        internal 
        view 
        returns (bool) 
    {
        bytes memory params = abi.encode(hash, method, paramCount);
        bytes memory result = Syscalls.contractCall(NativeContracts.CONTRACT_MANAGEMENT, "hasMethod", params);
        return abi.decode(result, (bool));
    }

    /**
     * @dev Check if contract exists
     */
    function isContract(address hash) internal view returns (bool) {
        bytes memory params = abi.encode(hash);
        bytes memory result = Syscalls.contractCall(NativeContracts.CONTRACT_MANAGEMENT, "isContract", params);
        return abi.decode(result, (bool));
    }
    
    /**
     * @dev Get minimum deployment fee
     */
    function getMinimumDeploymentFee() internal view returns (uint256) {
        bytes memory result = Syscalls.contractCall(NativeContracts.CONTRACT_MANAGEMENT, "getMinimumDeploymentFee", "");
        return abi.decode(result, (uint256));
    }
    
    /**
     * @dev Set minimum deployment fee
     */
    function setMinimumDeploymentFee(uint256 value) internal {
        bytes memory params = abi.encode(value);
        Syscalls.contractCall(NativeContracts.CONTRACT_MANAGEMENT, "setMinimumDeploymentFee", params);
    }
    
}
