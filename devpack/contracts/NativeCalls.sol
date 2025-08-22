// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title Neo N3 Native Contract Calls
 * @dev Direct integration with Neo N3 native contracts
 * @author Jimmy <jimmy@r3e.network>
 * 
 * This library provides direct access to Neo N3 native contracts:
 * - NEO: Native NEO token and governance
 * - GAS: Native GAS token
 * - ContractManagement: Contract deployment and management
 * - Policy: Network policy management
 * - Oracle: Oracle services
 * - RoleManagement: Role and permission management
 */

import "./Syscalls.sol";

library NativeCalls {
    using Syscalls for *;
    
    // Native contract script hashes (Neo N3 MainNet)
    address constant NEO_CONTRACT = 0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5;
    address constant GAS_CONTRACT = 0xd2a4cff31913016155e38e474a2c06d08be276cf;
    address constant CONTRACT_MANAGEMENT = 0xfffdc93764dbaddd97c48f252a53ea4643faa3fd;
    address constant POLICY_CONTRACT = 0xcc5e4edd9f5f8dba8bb65734541df7a1c081c67b;
    address constant ORACLE_CONTRACT = 0xfe924b7cfe89ddd271abaf7210a80a7e11178758;
    address constant ROLE_MANAGEMENT = 0x49cf4e5378ffcd4dec034fd98a174c5491e395e2;
    
    // ========== NEO Token Native Contract ==========
    
    /**
     * @dev Get NEO total supply
     */
    function neoTotalSupply() internal view returns (uint256) {
        bytes memory result = Syscalls.contractCall(NEO_CONTRACT, "totalSupply", "");
        return abi.decode(result, (uint256));
    }
    
    /**
     * @dev Get NEO balance of account
     */
    function neoBalanceOf(address account) internal view returns (uint256) {
        bytes memory params = abi.encode(account);
        bytes memory result = Syscalls.contractCall(NEO_CONTRACT, "balanceOf", params);
        return abi.decode(result, (uint256));
    }
    
    /**
     * @dev Transfer NEO tokens
     */
    function neoTransfer(address from, address to, uint256 amount, bytes memory data) 
        internal 
        returns (bool) 
    {
        bytes memory params = abi.encode(from, to, amount, data);
        bytes memory result = Syscalls.contractCall(NEO_CONTRACT, "transfer", params);
        return abi.decode(result, (bool));
    }
    
    /**
     * @dev Get NEO decimals
     */
    function neoDecimals() internal pure returns (uint8) {
        return 0; // NEO is indivisible
    }
    
    /**
     * @dev Get NEO symbol
     */
    function neoSymbol() internal pure returns (string memory) {
        return "NEO";
    }
    
    /**
     * @dev Vote for validator
     */
    function vote(address account, bytes memory publicKey) internal returns (bool) {
        bytes memory params = abi.encode(account, publicKey);
        bytes memory result = Syscalls.contractCall(NEO_CONTRACT, "vote", params);
        return abi.decode(result, (bool));
    }
    
    /**
     * @dev Get candidates
     */
    function getCandidates() internal view returns (NeoCandidate[] memory) {
        bytes memory result = Syscalls.contractCall(NEO_CONTRACT, "getCandidates", "");
        return abi.decode(result, (NeoCandidate[]));
    }
    
    /**
     * @dev Register as candidate
     */
    function registerCandidate(bytes memory publicKey) internal returns (bool) {
        bytes memory params = abi.encode(publicKey);
        bytes memory result = Syscalls.contractCall(NEO_CONTRACT, "registerCandidate", params);
        return abi.decode(result, (bool));
    }
    
    /**
     * @dev Unregister candidate
     */
    function unregisterCandidate(bytes memory publicKey) internal returns (bool) {
        bytes memory params = abi.encode(publicKey);
        bytes memory result = Syscalls.contractCall(NEO_CONTRACT, "unregisterCandidate", params);
        return abi.decode(result, (bool));
    }
    
    /**
     * @dev Get GAS per block
     */
    function getGasPerBlock() internal view returns (uint256) {
        bytes memory result = Syscalls.contractCall(NEO_CONTRACT, "getGasPerBlock", "");
        return abi.decode(result, (uint256));
    }
    
    /**
     * @dev Set GAS per block (committee only)
     */
    function setGasPerBlock(uint256 gasPerBlock) internal {
        bytes memory params = abi.encode(gasPerBlock);
        Syscalls.contractCall(NEO_CONTRACT, "setGasPerBlock", params);
    }
    
    /**
     * @dev Get account state
     */
    function getAccountState(address account) internal view returns (AccountState memory) {
        bytes memory params = abi.encode(account);
        bytes memory result = Syscalls.contractCall(NEO_CONTRACT, "getAccountState", params);
        return abi.decode(result, (AccountState));
    }
    
    // ========== GAS Token Native Contract ==========
    
    /**
     * @dev Get GAS total supply
     */
    function gasTotalSupply() internal view returns (uint256) {
        bytes memory result = Syscalls.contractCall(GAS_CONTRACT, "totalSupply", "");
        return abi.decode(result, (uint256));
    }
    
    /**
     * @dev Get GAS balance of account
     */
    function gasBalanceOf(address account) internal view returns (uint256) {
        bytes memory params = abi.encode(account);
        bytes memory result = Syscalls.contractCall(GAS_CONTRACT, "balanceOf", params);
        return abi.decode(result, (uint256));
    }
    
    /**
     * @dev Transfer GAS tokens
     */
    function gasTransfer(address from, address to, uint256 amount, bytes memory data) 
        internal 
        returns (bool) 
    {
        bytes memory params = abi.encode(from, to, amount, data);
        bytes memory result = Syscalls.contractCall(GAS_CONTRACT, "transfer", params);
        return abi.decode(result, (bool));
    }
    
    /**
     * @dev Get GAS decimals
     */
    function gasDecimals() internal pure returns (uint8) {
        return 8;
    }
    
    /**
     * @dev Get GAS symbol
     */
    function gasSymbol() internal pure returns (string memory) {
        return "GAS";
    }
    
    // ========== Contract Management Native Contract ==========
    
    /**
     * @dev Deploy new contract
     */
    function deployContract(bytes memory nef, bytes memory manifest) internal returns (address) {
        bytes memory params = abi.encode(nef, manifest);
        bytes memory result = Syscalls.contractCall(CONTRACT_MANAGEMENT, "deploy", params);
        return abi.decode(result, (address));
    }
    
    /**
     * @dev Update contract
     */
    function updateContract(bytes memory nef, bytes memory manifest) internal {
        bytes memory params = abi.encode(nef, manifest);
        Syscalls.contractCall(CONTRACT_MANAGEMENT, "update", params);
    }
    
    /**
     * @dev Destroy contract
     */
    function destroyContract() internal {
        Syscalls.contractCall(CONTRACT_MANAGEMENT, "destroy", "");
    }
    
    /**
     * @dev Get contract by hash
     */
    function getContract(address hash) internal view returns (ContractState memory) {
        bytes memory params = abi.encode(hash);
        bytes memory result = Syscalls.contractCall(CONTRACT_MANAGEMENT, "getContract", params);
        return abi.decode(result, (ContractState));
    }
    
    /**
     * @dev List all contracts
     */
    function listContracts() internal view returns (address[] memory) {
        bytes memory result = Syscalls.contractCall(CONTRACT_MANAGEMENT, "listContracts", "");
        return abi.decode(result, (address[]));
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
        bytes memory result = Syscalls.contractCall(CONTRACT_MANAGEMENT, "hasMethod", params);
        return abi.decode(result, (bool));
    }
    
    /**
     * @dev Get minimum deployment fee
     */
    function getMinimumDeploymentFee() internal view returns (uint256) {
        bytes memory result = Syscalls.contractCall(CONTRACT_MANAGEMENT, "getMinimumDeploymentFee", "");
        return abi.decode(result, (uint256));
    }
    
    /**
     * @dev Set minimum deployment fee
     */
    function setMinimumDeploymentFee(uint256 value) internal {
        bytes memory params = abi.encode(value);
        Syscalls.contractCall(CONTRACT_MANAGEMENT, "setMinimumDeploymentFee", params);
    }
    
    // ========== Policy Native Contract ==========
    
    /**
     * @dev Get fee per byte
     */
    function getFeePerByte() internal view returns (uint256) {
        bytes memory result = Syscalls.contractCall(POLICY_CONTRACT, "getFeePerByte", "");
        return abi.decode(result, (uint256));
    }
    
    /**
     * @dev Set fee per byte
     */
    function setFeePerByte(uint256 value) internal {
        bytes memory params = abi.encode(value);
        Syscalls.contractCall(POLICY_CONTRACT, "setFeePerByte", params);
    }
    
    /**
     * @dev Get execution fee factor
     */
    function getExecFeeFactor() internal view returns (uint32) {
        bytes memory result = Syscalls.contractCall(POLICY_CONTRACT, "getExecFeeFactor", "");
        return abi.decode(result, (uint32));
    }
    
    /**
     * @dev Set execution fee factor
     */
    function setExecFeeFactor(uint32 value) internal {
        bytes memory params = abi.encode(value);
        Syscalls.contractCall(POLICY_CONTRACT, "setExecFeeFactor", params);
    }
    
    /**
     * @dev Get storage price
     */
    function getStoragePrice() internal view returns (uint256) {
        bytes memory result = Syscalls.contractCall(POLICY_CONTRACT, "getStoragePrice", "");
        return abi.decode(result, (uint256));
    }
    
    /**
     * @dev Set storage price
     */
    function setStoragePrice(uint256 value) internal {
        bytes memory params = abi.encode(value);
        Syscalls.contractCall(POLICY_CONTRACT, "setStoragePrice", params);
    }
    
    /**
     * @dev Block account
     */
    function blockAccount(address account) internal {
        bytes memory params = abi.encode(account);
        Syscalls.contractCall(POLICY_CONTRACT, "blockAccount", params);
    }
    
    /**
     * @dev Unblock account
     */
    function unblockAccount(address account) internal {
        bytes memory params = abi.encode(account);
        Syscalls.contractCall(POLICY_CONTRACT, "unblockAccount", params);
    }
    
    /**
     * @dev Check if account is blocked
     */
    function isBlocked(address account) internal view returns (bool) {
        bytes memory params = abi.encode(account);
        bytes memory result = Syscalls.contractCall(POLICY_CONTRACT, "isBlocked", params);
        return abi.decode(result, (bool));
    }
    
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
        Syscalls.contractCall(ORACLE_CONTRACT, "request", params);
    }
    
    /**
     * @dev Get oracle price
     */
    function getOraclePrice() internal view returns (uint256) {
        bytes memory result = Syscalls.contractCall(ORACLE_CONTRACT, "getPrice", "");
        return abi.decode(result, (uint256));
    }
    
    /**
     * @dev Set oracle price
     */
    function setOraclePrice(uint256 price) internal {
        bytes memory params = abi.encode(price);
        Syscalls.contractCall(ORACLE_CONTRACT, "setPrice", params);
    }
    
    // ========== Role Management Native Contract ==========
    
    /**
     * @dev Designate as role
     */
    function designateAsRole(bytes1 role, address[] memory nodes) internal {
        bytes memory params = abi.encode(role, nodes);
        Syscalls.contractCall(ROLE_MANAGEMENT, "designateAsRole", params);
    }
    
    /**
     * @dev Get designated by role
     */
    function getDesignatedByRole(bytes1 role, uint256 index) internal view returns (address[] memory) {
        bytes memory params = abi.encode(role, index);
        bytes memory result = Syscalls.contractCall(ROLE_MANAGEMENT, "getDesignatedByRole", params);
        return abi.decode(result, (address[]));
    }
    
    // ========== Data Structures ==========
    
    struct NeoCandidate {
        bytes publicKey;
        uint256 votes;
        bool active;
    }
    
    struct AccountState {
        uint256 neoBalance;
        uint256 gasBalance;
        uint256 lastGasClaimBlock;
        bytes voteTo;
    }
    
    struct ContractState {
        address hash;
        bytes nef;
        bytes manifest;
        uint256 updateCounter;
    }
    
    // ========== Helper Functions ==========
    
    /**
     * @dev Check if native contract exists
     */
    function isNativeContract(address contractHash) internal pure returns (bool) {
        return contractHash == NEO_CONTRACT ||
               contractHash == GAS_CONTRACT ||
               contractHash == CONTRACT_MANAGEMENT ||
               contractHash == POLICY_CONTRACT ||
               contractHash == ORACLE_CONTRACT ||
               contractHash == ROLE_MANAGEMENT;
    }
    
    /**
     * @dev Get native contract name
     */
    function getNativeContractName(address contractHash) internal pure returns (string memory) {
        if (contractHash == NEO_CONTRACT) return "NeoToken";
        if (contractHash == GAS_CONTRACT) return "GasToken";
        if (contractHash == CONTRACT_MANAGEMENT) return "ContractManagement";
        if (contractHash == POLICY_CONTRACT) return "PolicyContract";
        if (contractHash == ORACLE_CONTRACT) return "OracleContract";
        if (contractHash == ROLE_MANAGEMENT) return "RoleManagement";
        return "Unknown";
    }
    
    /**
     * @dev Get all native contract addresses
     */
    function getAllNativeContracts() internal pure returns (address[] memory) {
        address[] memory contracts = new address[](6);
        contracts[0] = NEO_CONTRACT;
        contracts[1] = GAS_CONTRACT;
        contracts[2] = CONTRACT_MANAGEMENT;
        contracts[3] = POLICY_CONTRACT;
        contracts[4] = ORACLE_CONTRACT;
        contracts[5] = ROLE_MANAGEMENT;
        return contracts;
    }
    
    /**
     * @dev Estimate gas for native contract call
     */
    function estimateNativeCallGas(
        address contractHash,
        string memory method,
        bytes memory params
    ) internal view returns (uint256) {
        // Base gas cost for native contract calls
        uint256 baseGas = 1000000; // 0.01 GAS
        
        // Additional gas based on method complexity
        if (contractHash == NEO_CONTRACT) {
            if (keccak256(bytes(method)) == keccak256("vote")) return baseGas * 100;
            if (keccak256(bytes(method)) == keccak256("registerCandidate")) return baseGas * 1000;
        }
        
        if (contractHash == CONTRACT_MANAGEMENT) {
            if (keccak256(bytes(method)) == keccak256("deploy")) return baseGas * 500;
            if (keccak256(bytes(method)) == keccak256("update")) return baseGas * 300;
        }
        
        if (contractHash == ORACLE_CONTRACT) {
            if (keccak256(bytes(method)) == keccak256("request")) return baseGas * 50;
        }
        
        return baseGas;
    }
    
    /**
     * @dev Batch native contract calls
     */
    function batchNativeCalls(
        address[] memory contracts,
        string[] memory methods,
        bytes[] memory params
    ) internal returns (bytes[] memory results) {
        require(contracts.length == methods.length, "NativeCalls: array length mismatch");
        require(contracts.length == params.length, "NativeCalls: array length mismatch");
        require(contracts.length > 0, "NativeCalls: empty arrays");
        require(contracts.length <= 10, "NativeCalls: too many calls");
        
        results = new bytes[](contracts.length);
        
        for (uint256 i = 0; i < contracts.length; i++) {
            require(isNativeContract(contracts[i]), "NativeCalls: not a native contract");
            results[i] = Syscalls.contractCall(contracts[i], methods[i], params[i]);
        }
    }
    
    /**
     * @dev Get network configuration
     */
    function getNetworkConfiguration() internal view returns (NetworkConfig memory) {
        return NetworkConfig({
            feePerByte: getFeePerByte(),
            execFeeFactor: getExecFeeFactor(),
            storagePrice: getStoragePrice(),
            gasPerBlock: getGasPerBlock(),
            oraclePrice: getOraclePrice(),
            minimumDeploymentFee: getMinimumDeploymentFee()
        });
    }
    
    struct NetworkConfig {
        uint256 feePerByte;
        uint32 execFeeFactor;
        uint256 storagePrice;
        uint256 gasPerBlock;
        uint256 oraclePrice;
        uint256 minimumDeploymentFee;
    }
    
    // ========== Governance Functions ==========
    
    /**
     * @dev Get committee members
     */
    function getCommittee() internal view returns (address[] memory) {
        bytes memory result = Syscalls.contractCall(NEO_CONTRACT, "getCommittee", "");
        return abi.decode(result, (address[]));
    }
    
    /**
     * @dev Get next block validators
     */
    function getNextBlockValidators() internal view returns (address[] memory) {
        bytes memory result = Syscalls.contractCall(NEO_CONTRACT, "getNextBlockValidators", "");
        return abi.decode(result, (address[]));
    }
    
    /**
     * @dev Check if address is committee member
     */
    function isCommittee(address account) internal view returns (bool) {
        address[] memory committee = getCommittee();
        for (uint256 i = 0; i < committee.length; i++) {
            if (committee[i] == account) {
                return true;
            }
        }
        return false;
    }
    
    /**
     * @dev Check if address is validator
     */
    function isValidator(address account) internal view returns (bool) {
        address[] memory validators = getNextBlockValidators();
        for (uint256 i = 0; i < validators.length; i++) {
            if (validators[i] == account) {
                return true;
            }
        }
        return false;
    }
    
    // ========== Native Contract Utilities ==========
    
    /**
     * @dev Get native contract version
     */
    function getNativeContractVersion(address contractHash) internal view returns (string memory) {
        bytes memory result = Syscalls.contractCall(contractHash, "version", "");
        return abi.decode(result, (string));
    }
    
    /**
     * @dev Get native contract manifest
     */
    function getNativeContractManifest(address contractHash) internal view returns (bytes memory) {
        ContractState memory state = getContract(contractHash);
        return state.manifest;
    }
    
    /**
     * @dev Safe native contract call with error handling
     */
    function safeNativeCall(
        address contractHash,
        string memory method,
        bytes memory params
    ) internal returns (bool success, bytes memory result) {
        require(isNativeContract(contractHash), "NativeCalls: not a native contract");
        
        try this.externalNativeCall(contractHash, method, params) returns (bytes memory data) {
            return (true, data);
        } catch {
            return (false, "");
        }
    }
    
    /**
     * @dev External wrapper for try/catch
     */
    function externalNativeCall(
        address contractHash,
        string calldata method,
        bytes calldata params
    ) external returns (bytes memory) {
        return Syscalls.contractCall(contractHash, method, params);
    }
}