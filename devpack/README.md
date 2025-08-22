# Neo N3 Devpack for Solidity

**Complete Neo N3 smart contract development framework for Solidity**

This devpack provides comprehensive support for Neo N3 blockchain features in Solidity contracts, including all syscalls, native contracts, and NEP standards.

## 🎯 Features

### ✅ Core Neo N3 Integration
- **All Syscalls**: Complete mapping of Neo N3 system calls to Solidity functions
- **Native Contracts**: Direct integration with NEO, GAS, ContractManagement, Policy, Oracle
- **Storage Context**: Advanced storage operations with context management
- **Interop Services**: Full interoperability service support
- **Event System**: Runtime.Notify integration with indexed parameters

### ✅ NEP Standards Support
- **NEP-17**: Fungible token standard (enhanced ERC-20)
- **NEP-11**: Non-fungible token standard (enhanced ERC-721)
- **NEP-24**: Centralized Oracle standard
- **NEP-26**: Royalty standard for NFTs
- **Custom NEPs**: Framework for implementing additional standards

### ✅ Advanced Features
- **Multi-Signature**: Built-in multisig support with threshold configuration
- **Time Locks**: Transaction time-locking with block height or timestamp
- **Access Control**: Role-based permissions with hierarchical management
- **Upgradeable Contracts**: Contract update mechanisms with governance
- **Cross-Contract Calls**: Secure inter-contract communication

## 📚 Quick Start

### Installation

```bash
# Install Neo Solidity Compiler with devpack
git clone https://github.com/r3e-network/neo-solidity.git
cd neo-solidity
make install

# Or use npm package
npm install -g @r3e-network/neo-solidity-devpack
```

### Basic Usage

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@r3e-network/neo-devpack/contracts/Framework.sol";
import "@r3e-network/neo-devpack/standards/NEP17.sol";

contract MyToken is NEP17, Framework {
    using Neo for *;
    using Storage for *;
    using Runtime for *;
    
    constructor(string memory name, string memory symbol, uint8 decimals) 
        NEP17(name, symbol, decimals) 
    {
        // Initialize with Neo N3 features
        Storage.setContext(address(this));
        Runtime.checkWitness(tx.sender);
    }
    
    function mint(address to, uint256 amount) public onlyOwner {
        require(Neo.verifyWithWitness(msg.sender), "Invalid witness");
        _mint(to, amount);
        
        // Emit Neo-compatible event
        Runtime.notify("Transfer", abi.encode(address(0), to, amount));
    }
}
```

## 🏗️ Architecture

### Core Components

```
devpack/
├── contracts/           # Core framework contracts
│   ├── Framework.sol   # Base framework with Neo integration
│   ├── NativeCalls.sol # Native contract interfaces
│   └── Syscalls.sol    # System call mappings
├── standards/          # NEP standard implementations
│   ├── NEP17.sol      # Fungible tokens
│   ├── NEP11.sol      # Non-fungible tokens
│   └── NEP24.sol      # Oracle standard
├── libraries/          # Utility libraries
│   ├── Neo.sol        # Neo blockchain utilities
│   ├── Storage.sol    # Advanced storage operations
│   └── Runtime.sol    # Runtime services
└── examples/           # Complete contract examples
    ├── Token.sol      # Full NEP-17 token
    ├── NFT.sol        # Full NEP-11 NFT
    └── Oracle.sol     # Oracle integration
```

## 📖 API Reference

### System Calls (Syscalls.sol)

All Neo N3 syscalls mapped to Solidity functions:

```solidity
library Syscalls {
    // Blockchain state
    function getBlockHash(uint256 index) external view returns (bytes32);
    function getBlock(uint256 index) external view returns (Block memory);
    function getTransaction(bytes32 hash) external view returns (Transaction memory);
    function getTransactionHeight(bytes32 hash) external view returns (uint256);
    
    // Contract management
    function createContract(bytes calldata nef, bytes calldata manifest) external returns (address);
    function updateContract(bytes calldata nef, bytes calldata manifest) external;
    function destroyContract() external;
    
    // Storage operations
    function storageGet(bytes calldata key) external view returns (bytes memory);
    function storagePut(bytes calldata key, bytes calldata value) external;
    function storageDelete(bytes calldata key) external;
    function storageFind(bytes calldata prefix) external view returns (Iterator memory);
    
    // Cryptographic functions
    function sha256(bytes calldata data) external pure returns (bytes32);
    function ripemd160(bytes calldata data) external pure returns (bytes20);
    function verifyWithECDsa(bytes32 hash, bytes calldata pubkey, bytes calldata signature) external pure returns (bool);
    function murmurHash(bytes calldata data, uint32 seed) external pure returns (bytes4);
    
    // JSON operations
    function jsonSerialize(bytes calldata value) external pure returns (bytes memory);
    function jsonDeserialize(bytes calldata json) external pure returns (bytes memory);
    
    // Base64 operations
    function base64Encode(bytes calldata data) external pure returns (string memory);
    function base64Decode(string calldata data) external pure returns (bytes memory);
    
    // Iterator operations
    function iteratorNext(Iterator memory it) external returns (bool);
    function iteratorValue(Iterator memory it) external view returns (bytes memory);
}
```

### Native Contracts Integration

```solidity
// Native contract interfaces
interface INEO {
    function balanceOf(address account) external view returns (uint256);
    function transfer(address from, address to, uint256 amount) external returns (bool);
    function getTotalSupply() external view returns (uint256);
    function getGasPerBlock() external view returns (uint256);
    function setGasPerBlock(uint256 gasPerBlock) external;
    function getRegisterPrice() external view returns (uint256);
    function setRegisterPrice(uint256 registerPrice) external;
}

interface IGAS {
    function balanceOf(address account) external view returns (uint256);
    function transfer(address from, address to, uint256 amount) external returns (bool);
    function getTotalSupply() external view returns (uint256);
}

interface IContractManagement {
    function deploy(bytes calldata nefFile, bytes calldata manifest) external returns (address);
    function update(bytes calldata nefFile, bytes calldata manifest) external;
    function destroy() external;
    function getContract(address hash) external view returns (Contract memory);
    function listContracts() external view returns (address[] memory);
    function hasMethod(address hash, string calldata method, uint8 paramCount) external view returns (bool);
    function getMinimumDeploymentFee() external view returns (uint256);
}

interface IPolicy {
    function getFeePerByte() external view returns (uint256);
    function setFeePerByte(uint256 value) external;
    function getExecFeeFactor() external view returns (uint32);
    function setExecFeeFactor(uint32 value) external;
    function getStoragePrice() external view returns (uint256);
    function setStoragePrice(uint256 value) external;
    function isBlocked(address account) external view returns (bool);
    function blockAccount(address account) external;
    function unblockAccount(address account) external;
}

interface IOracle {
    function request(string calldata url, string calldata filter, string calldata callback, bytes calldata userData, uint256 gasForResponse) external;
    function getPrice() external view returns (uint256);
    function setPrice(uint256 price) external;
}

interface IRoleManagement {
    function hasRole(address account, bytes1 role) external view returns (bool);
    function getDesignatedByRole(bytes1 role, uint256 index) external view returns (address[] memory);
    function designateAsRole(bytes1 role, address[] calldata nodes) external;
}
```

## 📦 Complete Implementation

Let me create the complete devpack implementation:

**Author**: Jimmy <jimmy@r3e.network>  
**Repository**: https://github.com/r3e-network/neo-solidity