// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title Neo N3 Framework
 * @dev Base framework providing Neo N3 blockchain integration for Solidity contracts
 * @author Jimmy <jimmy@r3e.network>
 * 
 * This framework provides:
 * - Neo N3 syscall integration
 * - Native contract access
 * - Storage context management
 * - Runtime services
 * - Event emission compatibility
 */

import "./Syscalls.sol";
import "./NativeCalls.sol";
import "../libraries/Neo.sol";
import "../libraries/Storage.sol";
import "../libraries/Runtime.sol";

contract Framework {
    using Neo for *;
    using Storage for *;
    using Runtime for *;
    
    // Framework state
    address private _owner;
    bool private _initialized;
    uint256 private _version;
    
    // Neo N3 integration
    bytes4 private constant NEO_MAGIC = 0x3346454E; // "NEF3"
    uint256 private constant MIN_GAS_LIMIT = 20000000; // 0.2 GAS
    
    // Events
    event FrameworkInitialized(address indexed owner, uint256 version);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event ContractUpgraded(uint256 indexed oldVersion, uint256 indexed newVersion);
    event EmergencyStop(address indexed caller, string reason);
    
    // Modifiers
    modifier onlyOwner() {
        require(msg.sender == _owner, "Framework: caller is not the owner");
        _;
    }
    
    modifier whenInitialized() {
        require(_initialized, "Framework: not initialized");
        _;
    }
    
    modifier withWitness() {
        require(Runtime.checkWitness(msg.sender), "Framework: invalid witness");
        _;
    }
    
    modifier withGasLimit(uint256 minGas) {
        require(Runtime.gasLeft() >= minGas, "Framework: insufficient gas");
        _;
    }
    
    /**
     * @dev Initialize the framework
     */
    constructor() {
        _owner = msg.sender;
        _version = 1;
        _initialized = true;
        
        // Initialize Neo integration
        Storage.initializeContext();
        Runtime.initializeServices();
        
        emit FrameworkInitialized(_owner, _version);
    }
    
    /**
     * @dev Get framework version
     */
    function version() public view returns (uint256) {
        return _version;
    }
    
    /**
     * @dev Get contract owner
     */
    function owner() public view returns (address) {
        return _owner;
    }
    
    /**
     * @dev Check if framework is initialized
     */
    function initialized() public view returns (bool) {
        return _initialized;
    }
    
    /**
     * @dev Transfer ownership of the contract
     */
    function transferOwnership(address newOwner) public onlyOwner withWitness {
        require(newOwner != address(0), "Framework: new owner is the zero address");
        require(newOwner != _owner, "Framework: new owner is the same as current owner");
        
        emit OwnershipTransferred(_owner, newOwner);
        _owner = newOwner;
    }
    
    /**
     * @dev Renounce ownership of the contract
     */
    function renounceOwnership() public onlyOwner withWitness {
        emit OwnershipTransferred(_owner, address(0));
        _owner = address(0);
    }
    
    /**
     * @dev Upgrade contract version
     */
    function upgradeContract(bytes calldata nef, bytes calldata manifest, uint256 newVersion) 
        public 
        onlyOwner 
        withWitness 
        withGasLimit(MIN_GAS_LIMIT)
    {
        require(newVersion > _version, "Framework: version must be higher");
        require(nef.length > 0, "Framework: NEF cannot be empty");
        require(manifest.length > 0, "Framework: manifest cannot be empty");
        
        uint256 oldVersion = _version;
        _version = newVersion;
        
        // Call ContractManagement.update
        NativeCalls.updateContract(nef, manifest);
        
        emit ContractUpgraded(oldVersion, newVersion);
    }
    
    /**
     * @dev Emergency stop function
     */
    function emergencyStop(string calldata reason) public onlyOwner withWitness {
        // Emit emergency event
        emit EmergencyStop(msg.sender, reason);
        
        // Notify via Runtime
        Runtime.notify("EmergencyStop", abi.encode(msg.sender, reason, block.timestamp));
        
        // Optional: Call self-destruct via ContractManagement
        // NativeCalls.destroyContract();
    }
    
    /**
     * @dev Get current block information
     */
    function getCurrentBlock() public view returns (
        uint256 index,
        bytes32 hash,
        uint256 timestamp,
        bytes32 merkleRoot
    ) {
        return Neo.getCurrentBlock();
    }
    
    /**
     * @dev Get transaction information
     */
    function getTransaction(bytes32 txHash) public view returns (
        bytes32 hash,
        uint256 nonce,
        address sender,
        uint256 gasLimit,
        uint256 gasPrice
    ) {
        return Neo.getTransaction(txHash);
    }
    
    /**
     * @dev Get contract balance (GAS)
     */
    function getBalance() public view returns (uint256) {
        return Neo.getGasBalance(address(this));
    }
    
    /**
     * @dev Get NEO balance
     */
    function getNeoBalance(address account) public view returns (uint256) {
        return Neo.getNeoBalance(account);
    }
    
    /**
     * @dev Transfer GAS to another address
     */
    function transferGas(address to, uint256 amount) public onlyOwner withWitness {
        require(to != address(0), "Framework: cannot transfer to zero address");
        require(amount > 0, "Framework: amount must be greater than zero");
        
        bool success = Neo.transferGas(address(this), to, amount);
        require(success, "Framework: GAS transfer failed");
    }
    
    /**
     * @dev Advanced storage operations
     */
    function setStorageValue(bytes calldata key, bytes calldata value) public onlyOwner {
        Storage.put(key, value);
    }
    
    function getStorageValue(bytes calldata key) public view returns (bytes memory) {
        return Storage.get(key);
    }
    
    function deleteStorageValue(bytes calldata key) public onlyOwner {
        Storage.delete(key);
    }
    
    /**
     * @dev Iterate through storage
     */
    function findStorageValues(bytes calldata prefix) public view returns (bytes[] memory values) {
        Storage.Iterator memory iterator = Storage.find(prefix);
        values = new bytes[](100); // Max 100 results
        uint256 count = 0;
        
        while (iterator.next() && count < 100) {
            values[count] = iterator.value();
            count++;
        }
        
        // Resize array to actual count
        assembly {
            mstore(values, count)
        }
    }
    
    /**
     * @dev Call another contract
     */
    function callContract(
        address contractHash,
        string calldata method,
        bytes calldata params
    ) public withWitness returns (bytes memory) {
        return Neo.callContract(contractHash, method, params);
    }
    
    /**
     * @dev Deploy new contract
     */
    function deployContract(
        bytes calldata nef,
        bytes calldata manifest
    ) public onlyOwner withWitness withGasLimit(MIN_GAS_LIMIT) returns (address) {
        return NativeCalls.deployContract(nef, manifest);
    }
    
    /**
     * @dev Get contract information
     */
    function getContractInfo(address contractHash) public view returns (
        string memory name,
        bytes memory script,
        bytes memory manifest
    ) {
        return Neo.getContractInfo(contractHash);
    }
    
    /**
     * @dev Verify signature
     */
    function verifySignature(
        bytes32 hash,
        bytes calldata publicKey,
        bytes calldata signature
    ) public pure returns (bool) {
        return Neo.verifySignature(hash, publicKey, signature);
    }
    
    /**
     * @dev Get random number (using Neo's randomness)
     */
    function getRandom() public view returns (uint256) {
        return Neo.getRandom();
    }
    
    /**
     * @dev Check if account is committee member
     */
    function isCommittee(address account) public view returns (bool) {
        return Neo.isCommittee(account);
    }
    
    /**
     * @dev Get current committee members
     */
    function getCommittee() public view returns (address[] memory) {
        return Neo.getCommittee();
    }
    
    /**
     * @dev Advanced event emission
     */
    function emitEvent(string calldata eventName, bytes calldata data) public {
        Runtime.notify(eventName, data);
    }
    
    /**
     * @dev Emit indexed event (up to 4 topics)
     */
    function emitIndexedEvent(
        string calldata eventName,
        bytes32[] calldata topics,
        bytes calldata data
    ) public {
        require(topics.length <= 4, "Framework: too many topics");
        Runtime.notifyIndexed(eventName, topics, data);
    }
    
    /**
     * @dev Batch operations for gas efficiency
     */
    function batchStorageOperations(
        bytes[] calldata keys,
        bytes[] calldata values,
        bool[] calldata isDelete
    ) public onlyOwner {
        require(keys.length == values.length, "Framework: array length mismatch");
        require(keys.length == isDelete.length, "Framework: array length mismatch");
        require(keys.length > 0, "Framework: empty arrays");
        require(keys.length <= 100, "Framework: too many operations");
        
        for (uint256 i = 0; i < keys.length; i++) {
            if (isDelete[i]) {
                Storage.delete(keys[i]);
            } else {
                Storage.put(keys[i], values[i]);
            }
        }
    }
    
    /**
     * @dev Get gas consumption for operation
     */
    function estimateGas(bytes calldata operation) public view returns (uint256) {
        return Neo.estimateGas(operation);
    }
    
    /**
     * @dev Get current gas price
     */
    function getGasPrice() public view returns (uint256) {
        return Neo.getGasPrice();
    }
    
    /**
     * @dev Get storage price per byte
     */
    function getStoragePrice() public view returns (uint256) {
        return Neo.getStoragePrice();
    }
    
    /**
     * @dev Framework diagnostics
     */
    function getDiagnostics() public view returns (
        uint256 currentBlock,
        uint256 gasBalance,
        uint256 neoBalance,
        uint256 storageUsage,
        bool isCommitteeMember
    ) {
        currentBlock = block.number;
        gasBalance = Neo.getGasBalance(address(this));
        neoBalance = Neo.getNeoBalance(address(this));
        storageUsage = Storage.getUsage();
        isCommitteeMember = Neo.isCommittee(address(this));
    }
    
    /**
     * @dev Framework metadata
     */
    function getFrameworkInfo() public pure returns (
        string memory name,
        string memory version,
        string memory author,
        string memory repository
    ) {
        return (
            "Neo N3 Solidity Framework",
            "1.0.0",
            "Jimmy <jimmy@r3e.network>",
            "https://github.com/r3e-network/neo-solidity"
        );
    }
}