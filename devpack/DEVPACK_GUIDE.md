# Neo N3 Devpack for Solidity - Complete Guide

**A comprehensive development framework for building Neo N3 smart contracts in Solidity**

## 🎯 Overview

The Neo N3 Devpack provides complete integration between Solidity smart contracts and the Neo N3 blockchain, including all syscalls, native contracts, and NEP standards support.

## 🚀 Quick Start

### Installation

```bash
# Install via npm
npm install @r3e-network/neo-solidity-devpack

# Or clone and build
git clone https://github.com/r3e-network/neo-solidity.git
cd neo-solidity/devpack
npm install
```

### Basic Usage

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@r3e-network/neo-devpack/contracts/Framework.sol";
import "@r3e-network/neo-devpack/standards/NEP17.sol";

contract MyToken is NEP17, Framework {
    constructor() NEP17("My Token", "MTK", 18, 1000000, 10000000) {
        // Your token is now ready with full Neo N3 integration!
    }
}
```

## 📚 Core Components

### 🏗️ Framework.sol
Base framework providing Neo N3 integration:

```solidity
import "@r3e-network/neo-devpack/contracts/Framework.sol";

contract MyContract is Framework {
    function myFunction() public withWitness {
        // Function requires valid witness (signature)
        
        // Access Neo blockchain data
        (uint256 blockIndex, bytes32 hash, uint256 timestamp,) = getCurrentBlock();
        
        // Interact with native contracts
        uint256 gasBalance = getBalance();
        
        // Emit Neo-compatible events
        emitEvent("MyEvent", abi.encode("data"));
    }
}
```

### 🔧 Syscalls.sol
Complete Neo N3 syscall integration:

```solidity
import "@r3e-network/neo-devpack/contracts/Syscalls.sol";

contract MySyscallContract {
    using Syscalls for *;
    
    function useBlockchain() public view {
        // Get blockchain information
        uint256 currentHeight = Syscalls.getCurrentIndex();
        Syscalls.Block memory block = Syscalls.getBlock(currentHeight);
        
        // Storage operations
        Syscalls.StorageContext memory ctx = Syscalls.getStorageContext();
        bytes memory value = Syscalls.storageGet(ctx, "mykey");
        
        // Cryptographic operations
        bytes32 hash = Syscalls.sha256("data");
        bool verified = Syscalls.checkWitness(msg.sender);
    }
}
```

### 🏛️ NativeCalls.sol
Direct integration with Neo native contracts:

```solidity
import "@r3e-network/neo-devpack/contracts/NativeCalls.sol";

contract MyNativeContract {
    using NativeCalls for *;
    
    function useNativeContracts() public {
        // NEO token operations
        uint256 neoBalance = NativeCalls.neoBalanceOf(msg.sender);
        bool success = NativeCalls.neoTransfer(msg.sender, address(this), 1, "");
        
        // GAS token operations
        uint256 gasBalance = NativeCalls.gasBalanceOf(msg.sender);
        
        // Contract management
        address newContract = NativeCalls.deployContract(nefData, manifestData);
        
        // Policy queries
        uint256 storagePrice = NativeCalls.getStoragePrice();
        
        // Oracle requests
        NativeCalls.requestOracleData("https://api.example.com", "$.price", "callback", "", 10000000);
    }
}
```

## 📋 NEP Standards

### 🪙 NEP-17 Fungible Tokens

```solidity
import "@r3e-network/neo-devpack/standards/NEP17.sol";

contract MyNEP17Token is NEP17 {
    constructor() NEP17("My Token", "MTK", 18, 1000000, 0) {
        // Token with 18 decimals, 1M initial supply, no max supply
    }
    
    function customMint(address to, uint256 amount) public onlyMinter {
        mint(to, amount);
        
        // Emit custom event
        emitEvent("CustomMint", abi.encode(to, amount));
    }
}
```

### 🎨 NEP-11 Non-Fungible Tokens

```solidity
import "@r3e-network/neo-devpack/standards/NEP11.sol";

contract MyNEP11NFT is NEP11 {
    constructor() NEP11("My NFT", "MNFT", 0, "https://api.mynft.com/", 10000, false) {
        // Indivisible NFT with 10k max supply
    }
    
    function mintNFT(address to, bytes memory metadata) public onlyMinter returns (bytes32) {
        bytes32 tokenId = bytes32(_currentTokenId++);
        mint(to, tokenId, metadata);
        return tokenId;
    }
}
```

### 🔮 NEP-24 Oracle Integration

```solidity
import "@r3e-network/neo-devpack/standards/NEP24.sol";

contract MyOracleContract is NEP24Oracle {
    constructor() NEP24Oracle(1000000) { // 0.01 GAS per request
    }
    
    function getPriceData(string memory symbol) public returns (uint256 requestId) {
        return requestPriceData(symbol, "priceCallback");
    }
    
    function priceCallback(uint256 requestId, uint256 code, bytes calldata result, bytes calldata userData) external override {
        if (code == 0) {
            uint256 price = abi.decode(result, (uint256));
            string memory symbol = abi.decode(userData, (string));
            
            // Store price data
            Storage.put(abi.encode("price", symbol), abi.encode(price, block.timestamp));
        }
    }
}
```

## 🛠️ Advanced Libraries

### 🌐 Neo.sol - Blockchain Utilities

```solidity
import "@r3e-network/neo-devpack/libraries/Neo.sol";

contract MyContract {
    using Neo for *;
    
    function advancedOperations() public {
        // Blockchain queries
        (uint256 index, bytes32 hash,,) = Neo.getCurrentBlock();
        
        // Account management
        uint256 neoBalance = Neo.getNeoBalance(msg.sender);
        uint256 gasBalance = Neo.getGasBalance(msg.sender);
        
        // Governance
        bool isCommittee = Neo.isCommittee(msg.sender);
        address[] memory validators = Neo.getValidators();
        
        // Contract interaction
        bytes memory result = Neo.callContract(targetContract, "method", params);
        
        // Security
        bool verified = Neo.verifyWithWitness(msg.sender);
        uint256 random = Neo.getRandom();
    }
}
```

### 💾 Storage.sol - Advanced Storage

```solidity
import "@r3e-network/neo-devpack/libraries/Storage.sol";

contract MyStorageContract {
    using Storage for *;
    
    function advancedStorage() public {
        // Initialize storage
        Storage.initializeContext();
        
        // Basic operations
        Storage.put("key", "value");
        bytes memory value = Storage.get("key");
        
        // Advanced operations
        bytes[] memory values = Storage.findValues("prefix");
        uint256 count = Storage.count("prefix");
        
        // Batch operations
        bytes[] memory keys = ["key1", "key2", "key3"];
        bytes[] memory vals = ["val1", "val2", "val3"];
        Storage.batchPut(keys, vals);
        
        // Typed storage
        Storage.putUint256("balance", 1000);
        Storage.putAddress("owner", msg.sender);
        Storage.putString("name", "MyContract");
        
        // Security
        Storage.putSecure("secret", "sensitive_data");
        bytes memory secureData = Storage.getSecure("secret");
    }
}
```

### ⚡ Runtime.sol - Runtime Services

```solidity
import "@r3e-network/neo-devpack/libraries/Runtime.sol";

contract MyRuntimeContract {
    using Runtime for *;
    
    function runtimeOperations() public {
        // Event emission
        Runtime.notify("MyEvent", abi.encode("data"));
        Runtime.notifyTransfer(address(0), msg.sender, 1000);
        
        // Authorization
        Runtime.requireWitness(msg.sender);
        bool hasRole = Runtime.hasRole(msg.sender, "ADMIN");
        
        // Gas management
        uint256 gasRemaining = Runtime.gasLeft();
        Runtime.requireGas(1000000); // Require 0.01 GAS
        
        // Time operations
        uint256 currentTime = Runtime.getTimestamp();
        bool isAfter = Runtime.isAfterTime(1640995200); // After specific timestamp
        
        // Execution context
        (address executing, address calling,,,) = Runtime.getExecutionContext();
        
        // Error handling
        (bool success, bytes memory result) = Runtime.safeExternalCall(target, data);
    }
}
```

## 🎨 Complete Examples

### 💰 Advanced NEP-17 Token

See [`examples/CompleteNEP17Token.sol`](./examples/CompleteNEP17Token.sol) for:
- ✅ Full NEP-17 compliance
- ✅ Staking system with rewards
- ✅ Oracle price integration
- ✅ Governance system
- ✅ Multi-signature operations
- ✅ Emergency controls
- ✅ Gas optimization

### 🖼️ Advanced NEP-11 NFT

See [`examples/CompleteNEP11NFT.sol`](./examples/CompleteNEP11NFT.sol) for:
- ✅ Full NEP-11 compliance
- ✅ Royalty system
- ✅ Marketplace integration
- ✅ Oracle metadata updates
- ✅ Curation system
- ✅ Fractionalization
- ✅ Bundle creation

## 🔧 Integration Examples

### Basic Token Implementation

```solidity
pragma solidity ^0.8.19;

import "@r3e-network/neo-devpack/standards/NEP17.sol";

contract SimpleToken is NEP17 {
    constructor() NEP17("Simple Token", "SIMPLE", 8, 1000000, 0) {
        // 1M tokens with 8 decimals, no max supply
    }
    
    // Token is ready to use with all Neo N3 features!
}
```

### Basic NFT Implementation

```solidity
pragma solidity ^0.8.19;

import "@r3e-network/neo-devpack/standards/NEP11.sol";

contract SimpleNFT is NEP11 {
    constructor() NEP11("Simple NFT", "SNFT", 0, "https://api.simple.nft/", 1000, false) {
        // Collection with 1000 max supply, indivisible
    }
    
    function mintNFT(address to, string memory metadata) public onlyMinter returns (bytes32) {
        bytes32 tokenId = generateTokenId(msg.sender, block.timestamp);
        mint(to, tokenId, bytes(metadata));
        return tokenId;
    }
}
```

### Oracle Integration

```solidity
pragma solidity ^0.8.19;

import "@r3e-network/neo-devpack/standards/NEP24.sol";

contract PriceOracle is NEP24Oracle {
    mapping(string => uint256) public prices;
    
    constructor() NEP24Oracle(1000000) {} // 0.01 GAS per request
    
    function updatePrice(string memory symbol) public returns (uint256) {
        return requestPriceData(symbol, "updatePriceCallback");
    }
    
    function updatePriceCallback(uint256, uint256 code, bytes calldata result, bytes calldata userData) external {
        if (code == 0) {
            string memory symbol = abi.decode(userData, (string));
            uint256 price = abi.decode(result, (uint256));
            prices[symbol] = price;
        }
    }
}
```

## 🔒 Security Best Practices

### Witness Verification

```solidity
contract SecureContract is Framework {
    function secureFunction() public withWitness {
        // Function automatically verifies witness
        // Only callable with valid signature
    }
    
    function manualWitnessCheck() public {
        require(Runtime.checkWitness(msg.sender), "Invalid witness");
        // Manual witness verification
    }
    
    function multiSigOperation() public {
        address[] memory signers = [signer1, signer2, signer3];
        require(Runtime.checkMultiSigWitness(signers, 2), "Insufficient signatures");
        // Requires 2 of 3 signatures
    }
}
```

### Gas Management

```solidity
contract GasOptimizedContract is Framework {
    function expensiveOperation() public withGasLimit(50000000) { // Require 0.5 GAS
        // Gas-intensive operation
        
        // Optimize with batching
        Runtime.optimizeGasUsage(
            function() {
                // Batch operations here
            },
            10000000 // Expected savings
        );
    }
    
    function conditionalOperation() public {
        Runtime.executeIfGasAvailable(
            10000000, // Required gas
            function() {
                // Execute only if enough gas
            }
        );
    }
}
```

### Storage Security

```solidity
contract SecureStorageContract is Framework {
    using Storage for *;
    
    function secureStorage() public {
        // Secure storage with checksum
        Storage.putSecure("sensitive_data", abi.encode(secretValue));
        bytes memory data = Storage.getSecure("sensitive_data");
        
        // Access-controlled storage
        Storage.putWithAccess("admin_data", abi.encode(adminValue), owner());
        
        // Expiring storage
        Storage.putWithExpiration("temp_data", abi.encode(tempValue), block.number + 1000);
    }
}
```

## 📊 Performance Optimization

### Batch Operations

```solidity
contract OptimizedContract is NEP17 {
    function batchTransfers() public {
        // Use built-in batch transfer
        address[] memory recipients = [addr1, addr2, addr3];
        uint256[] memory amounts = [100, 200, 300];
        batchTransfer(recipients, amounts, new bytes[](3));
    }
    
    function batchStorage() public {
        // Batch storage operations
        bytes[] memory keys = [key1, key2, key3];
        bytes[] memory values = [val1, val2, val3];
        Storage.batchPut(keys, values);
    }
}
```

### Gas Optimization

```solidity
contract GasOptimized is Framework {
    function optimizedLoop() public {
        Runtime.optimizedLoop(
            1000, // iterations
            5000,  // gas per iteration
            function(uint256 i) {
                // Loop body - automatically limited by available gas
                Storage.put(abi.encode("item", i), abi.encode(i * 2));
            }
        );
    }
}
```

## 🎯 Deployment Guide

### Hardhat Configuration

```javascript
// hardhat.config.js
require('@r3e-network/neo-solidity-devpack');

module.exports = {
  solidity: {
    version: "0.8.19",
    settings: {
      optimizer: { enabled: true, runs: 200 },
      neo: {
        devpack: true,
        syscalls: "all",
        nativeContracts: "all",
        nepStandards: ["NEP-17", "NEP-11", "NEP-24"]
      }
    }
  },
  networks: {
    neo_testnet: {
      url: "http://seed1t5.neo.org:20332",
      accounts: ["your-private-key"]
    }
  }
};
```

### Deployment Script

```javascript
// scripts/deploy.js
const { ethers } = require("hardhat");

async function main() {
  // Deploy NEP-17 token
  const Token = await ethers.getContractFactory("CompleteNEP17Token");
  const token = await Token.deploy(
    "My Token",     // name
    "MTK",          // symbol
    18,             // decimals
    1000000,        // initial supply
    10000000,       // max supply
    oracleAddress   // oracle contract
  );
  
  await token.deployed();
  console.log("Token deployed to:", token.address);
  
  // Deploy NFT collection
  const NFT = await ethers.getContractFactory("CompleteNEP11NFT");
  const nft = await NFT.deploy(
    "My NFT Collection",
    "MNFT",
    "Premium NFT collection",
    "https://api.mynft.com/",
    1000,
    oracleAddress
  );
  
  await nft.deployed();
  console.log("NFT deployed to:", nft.address);
}

main().catch(console.error);
```

### Compilation

```bash
# Compile with Neo devpack
npx hardhat compile

# Deploy to TestNet
npx hardhat run scripts/deploy.js --network neo_testnet

# Verify contracts
npx hardhat verify --network neo_testnet <contract-address> "constructor" "args"
```

## 🧪 Testing

### Unit Tests

```javascript
const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("NEP-17 Token", function () {
  let token;
  let owner, addr1, addr2;

  beforeEach(async function () {
    [owner, addr1, addr2] = await ethers.getSigners();
    
    const Token = await ethers.getContractFactory("CompleteNEP17Token");
    token = await Token.deploy("Test Token", "TEST", 18, 1000000, 0, ethers.constants.AddressZero);
    await token.deployed();
  });

  it("Should have correct initial state", async function () {
    expect(await token.name()).to.equal("Test Token");
    expect(await token.symbol()).to.equal("TEST");
    expect(await token.decimals()).to.equal(18);
    expect(await token.totalSupply()).to.equal(1000000);
  });

  it("Should transfer tokens correctly", async function () {
    await token.transfer(addr1.address, 1000, "0x");
    expect(await token.balanceOf(addr1.address)).to.equal(1000);
  });

  it("Should integrate with Neo features", async function () {
    // Test Neo-specific features
    const blockInfo = await token.getCurrentBlock();
    expect(blockInfo.index).to.be.a('number');
    
    const gasBalance = await token.getBalance();
    expect(gasBalance).to.be.a('number');
  });
});
```

### Integration Tests

```javascript
describe("Neo Integration", function () {
  it("Should generate correct Neo contract files", async function () {
    // Test compilation outputs
    const artifacts = await hre.artifacts.readArtifact("CompleteNEP17Token");
    
    expect(artifacts.nef).to.exist;
    expect(artifacts.manifest).to.exist;
    expect(artifacts.manifest.abi.methods).to.have.length.greaterThan(0);
  });

  it("Should be deployable to Neo TestNet", async function () {
    // This would require actual Neo TestNet connection
    // For unit tests, we mock the deployment
  });
});
```

## 📖 API Reference

### Framework Methods

| Method | Description | Gas Cost |
|--------|-------------|----------|
| `getCurrentBlock()` | Get current block info | Low |
| `getBalance()` | Get contract GAS balance | Low |
| `transferGas()` | Transfer GAS tokens | Medium |
| `callContract()` | Call another contract | High |
| `deployContract()` | Deploy new contract | Very High |

### Storage Methods

| Method | Description | Gas Cost |
|--------|-------------|----------|
| `put(key, value)` | Store value | Medium |
| `get(key)` | Retrieve value | Low |
| `find(prefix)` | Find keys with prefix | High |
| `batchPut()` | Batch store operations | Medium |

### Runtime Methods

| Method | Description | Gas Cost |
|--------|-------------|----------|
| `notify()` | Emit event | Low |
| `checkWitness()` | Verify signature | Medium |
| `gasLeft()` | Get remaining gas | Low |
| `log()` | Write to logs | Low |

## 🚨 Error Handling

### Common Errors

```solidity
contract ErrorHandling is Framework {
    function handleErrors() public {
        try this.riskyOperation() {
            // Success
        } catch Error(string memory reason) {
            Runtime.log(string(abi.encodePacked("Error: ", reason)));
        } catch {
            Runtime.log("Unknown error occurred");
        }
    }
    
    function riskyOperation() external {
        require(Neo.gasLeft() > 1000000, "Insufficient gas");
        require(Runtime.checkWitness(msg.sender), "Invalid witness");
        
        // Operation that might fail
    }
}
```

## 📈 Best Practices

### 1. Always Use Witness Verification
```solidity
function secureFunction() public withWitness {
    // Secure by default
}
```

### 2. Optimize Gas Usage
```solidity
function batchOperation() public {
    Runtime.optimizeGasUsage(
        function() {
            // Batch multiple operations
        },
        expectedSavings
    );
}
```

### 3. Use Typed Storage
```solidity
// Good
Storage.putUint256("balance", amount);

// Avoid
Storage.put("balance", abi.encode(amount));
```

### 4. Handle Oracle Responses
```solidity
function oracleCallback(uint256 requestId, uint256 code, bytes calldata result, bytes calldata userData) external {
    if (code == 0) {
        // Handle success
    } else {
        // Handle error
        Runtime.log("Oracle request failed");
    }
}
```

### 5. Implement Emergency Controls
```solidity
function emergencyStop() public onlyOwner withWitness {
    // Emergency pause functionality
    _pause();
    Runtime.notify("EmergencyStop", abi.encode(msg.sender, block.timestamp));
}
```

## 🎓 Learning Resources

### Step-by-Step Tutorials

1. **[Basic Token](./tutorials/basic-token.md)** - Create your first NEP-17 token
2. **[NFT Collection](./tutorials/nft-collection.md)** - Build complete NFT marketplace
3. **[Oracle Integration](./tutorials/oracle-integration.md)** - Use external data sources
4. **[DeFi Protocol](./tutorials/defi-protocol.md)** - Build advanced DeFi applications

### Code Examples

- **[Token Examples](./examples/)** - Various token implementations
- **[NFT Examples](./examples/)** - Different NFT use cases
- **[Oracle Examples](./examples/)** - Oracle integration patterns
- **[DeFi Examples](./examples/)** - DeFi protocol examples

## 💬 Support

- **📖 Documentation**: Complete API reference and guides
- **🐛 Issues**: [GitHub Issues](https://github.com/r3e-network/neo-solidity/issues)
- **💬 Discord**: Community support and discussions
- **📧 Email**: jimmy@r3e.network for technical support

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Add tests for new features
4. Submit pull request
5. Follow our [contributing guidelines](../CONTRIBUTING.md)

---

**Built with ❤️ by R3E Network**

*Bringing Ethereum's developer ecosystem to Neo blockchain with full N3 integration*