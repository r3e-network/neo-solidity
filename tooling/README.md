# Neo-Solidity Comprehensive Development Tooling

A complete toolchain under active development for Neo-Solidity. Several packages remain experimental—review the status table below before adopting any component in production.

> ⚠️ **Current Status (Aug 2024)**  
> - `@neo-solidity/hardhat-solc-neo`: compile/clean/verify tasks work; advanced Hardhat integration is still evolving.  
> - `@neo-solidity/hardhat-neo-deployer`: builds/signs/sends real Neo N3 deploy transactions (NEF + manifest); still experimental.  
> - `@neo-solidity/neo-foundry` (`neo-forge`, `neo-cast`, `neo-anvil`): CLI scaffolding only; commands print placeholder messages.  
> - `@neo-solidity/abi-router`, `@neo-solidity/cli-tools`: prototypes still missing full ABI coverage and verified deployments.

## 🏗️ Architecture Overview

```
Neo-Solidity Tooling Ecosystem
├── Hardhat Integration
│   ├── @neo-solidity/hardhat-solc-neo      # Compilation plugin
│   └── @neo-solidity/hardhat-neo-deployer  # Deployment plugin
├── Foundry Integration  
│   └── @neo-solidity/neo-foundry           # Complete Foundry adapter
├── Core Libraries
│   ├── @neo-solidity/types                 # Shared type definitions
│   ├── @neo-solidity/abi-router            # ABI compatibility layer
│   └── @neo-solidity/cli-tools             # Command-line tools
└── Developer Experience
    ├── Network configurations               # Neo network presets
    ├── Artifact management                 # Build output handling
    └── Debugging support                   # Development debugging
```

## 🚀 Quick Start

### 1. Hardhat Setup (compile + deploy)

```bash
npm install --save-dev @neo-solidity/hardhat-solc-neo
npm install --save-dev @neo-solidity/hardhat-neo-deployer

# hardhat.config.ts
import "@neo-solidity/hardhat-solc-neo";
import "@neo-solidity/hardhat-neo-deployer";

export default {
  neoSolc: {
    solidity: {
      version: "0.8.20",
      settings: {
        optimizer: { enabled: true, runs: 200 },
        neo: { callt: true }
      }
    }
  },
  neoNetworks: {
    testnet: {
      rpcUrls: ["https://testnet1.neo.coz.io:443"],
      magic: 894710606,
      accounts: [process.env.NEO_WIF || ""] // WIF or private key (hex)
    }
  }
};

# Compile contracts
npx hardhat neo-compile

# Deploy contracts
npx hardhat neo-deploy --contract MyContract --network testnet
```

### 2. Foundry Setup (scaffolding only)

```bash
npm install -g @neo-solidity/neo-foundry

# Initialize project
neo-forge init my-neo-project
cd my-neo-project

# neo-foundry.toml configuration
[profile.default]
src = "src"
test = "test" 
script = "script"
out = "out"

# Build and test (currently print placeholder output)
neo-forge build
neo-forge test
```

### 3. CLI Tools

```bash
npm install -g @neo-solidity/cli-tools

# Compile contracts directly
solc-neo compile contracts/*.sol --optimize --gas-model hybrid

# Analyze contracts
solc-neo analyze contracts/*.sol --gas-report --size-report

# Verify on-chain NEF/manifest matches your local compilation output
solc-neo verify-contract --address N123... --source Token.sol --contract Token --network testnet
```

## 📦 Package Ecosystem

### Core Packages

#### `@neo-solidity/types`
Shared TypeScript interfaces and type definitions for all tooling packages.

**Key Types:**
- `NeoSolidityConfig` - Compiler configuration
- `NeoNetworkConfig` - Network definitions  
- `BuildArtifact` - Compilation artifacts
- `ContractDeployment` - Deployment results
- `NeoRpcProvider` - RPC interface

#### `@neo-solidity/abi-router` 
ABI-compatible interface layer that bridges Ethereum tooling to Neo contracts. It currently supports static ABIs and best-effort event scanning; transaction signing + deployments are handled by `@neo-solidity/hardhat-neo-deployer` or native Neo tooling.

**Current capabilities / caveats:**
- ✅ Ethereum-style contract interaction (call/send/estimate).
- ✅ `AbiRouter.deployContract({ nef, manifest }, abi)` when you pass in the compiler artifacts manually.
- ✅ Basic event filtering (linear scan via RPC application logs).
- ⚠️ No automatic artifact registry or dynamic ABI decoding yet.
- ⚠️ Large block ranges can be slow because filtering is sequential.

```typescript
import { readFileSync } from 'fs';
import { AbiRouter } from '@neo-solidity/abi-router';

const router = new AbiRouter(neoRpcProvider);
const artifacts = { nef: readFileSync('Token.nef', 'hex'), manifest: require('./Token.manifest.json') };

// Deploy a NEF/manifest pair compiled by neo-solc
const deployed = await router.deployContract(artifacts, abi);

// Wrap an existing Neo contract with Ethereum-style methods
const contract = router.createContract(deployed.address, abi, signer);
await contract.transfer(recipient, amount);
const balance = await contract.balanceOf(account);
```

### Hardhat Integration

#### `@neo-solidity/hardhat-solc-neo`
Hardhat plugin for compiling Solidity to NeoVM bytecode. The runtime extension only exposes `hre.neoSolc.compiler` and `hre.neoSolc.artifacts`; other helpers were removed until a real Neo RPC workflow exists.

**Tasks (currently supported):**
- `neo-compile` - Compile contracts
- `neo-clean` - Clean build artifacts  
- `neo-verify` - Verify on-chain NEF/manifest matches local build artifact (updates deployment metadata)

Pair this with `@neo-solidity/hardhat-neo-deployer` if you want `neo-deploy` tasks inside Hardhat.

**Configuration:**
```typescript
neoSolc: {
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: { enabled: true, runs: 200 },
      neo: { callt: true }
    }
  }
}
```

#### `@neo-solidity/hardhat-neo-deployer`
Experimental Hardhat plugin for deploying Neo N3 contracts. It builds/signs/sends `ContractManagement.deploy` transactions using the NEF + manifest embedded in build artifacts.

**Status**
- `neo-deploy` / `neo-deploy-batch`: submit real deployment transactions.
- `neo-deploy-estimate`: uses `invokescript` + `calculatenetworkfee` to estimate fees.
- `neo-accounts` helpers: derive accounts from WIF/private key; no encrypted keystore yet.

### Foundry Integration

#### `@neo-solidity/neo-foundry`
Prototype Foundry-compatible tooling for Neo. `neo-forge`, `neo-cast`, and `neo-anvil` currently emit placeholder output and should be considered scaffolding.

**Tools:**
- `neo-forge` - Build/test CLI (prints stub messages today)
- `neo-cast` - Contract interaction tool (WIP)
- `neo-anvil` - Local Neo blockchain stub

**Commands:**
```bash
# Build system (WIP)
neo-forge build --watch
neo-forge test --gas-report
neo-forge clean

# Contract interaction (WIP)
neo-cast call 0x123... balanceOf 0xabc...
neo-cast send 0x123... transfer 0xdef... 100

# Local blockchain (WIP)
neo-anvil --port 40332 --accounts 10
```

## 🌐 Network Configuration

### Predefined Networks

```typescript
// Built-in network configurations
const networks = {
  mainnet: {
    name: "Neo MainNet",
    rpcUrls: ["https://mainnet1.neo.coz.io:443"],
    magic: 860833102,
    nativeTokens: { gas: "0xd2a4...", neo: "0xef40..." }
  },
  testnet: {
    name: "Neo TestNet", 
    rpcUrls: ["https://testnet1.neo.coz.io:443"],
    magic: 894710606,
    testnet: true
  },
  private: {
    name: "Neo Private",
    rpcUrls: ["http://localhost:40332"],
    magic: 12345,
    testnet: true
  }
};
```

### Custom Network Setup

```typescript
// hardhat.config.ts
neoNetworks: {
  "custom-network": {
    name: "Custom Neo Network",
    rpcUrls: ["https://rpc.custom-neo.com"],
    magic: 123456,
    addressVersion: 0x35,
    accounts: ["0x..."] // Private keys or mnemonic
  }
}
```

## 🔧 Development Workflow

### 1. Project Structure

```
my-neo-project/
├── contracts/           # Solidity source files
│   ├── Token.sol
│   └── interfaces/
├── test/               # Test files
│   └── Token.test.sol
├── scripts/            # Deployment scripts
│   └── deploy.ts
├── artifacts/          # Build artifacts
│   └── contracts/
├── deployments/        # Deployment records
│   ├── testnet/
│   └── mainnet/
├── hardhat.config.ts   # Hardhat configuration
└── neo-foundry.toml   # Neo Foundry configuration
```

### 2. Contract Development

```solidity
// contracts/Token.sol
pragma solidity ^0.8.19;

import "@neo-solidity/contracts/token/ERC20/ERC20.sol";

contract MyToken is ERC20 {
    constructor(string memory name, string memory symbol) 
        ERC20(name, symbol) {
        _mint(msg.sender, 1000000 * 10**18);
    }
}
```

### 3. Testing

```typescript
// test/Token.test.ts
import { expect } from "chai";
import { ethers } from "hardhat";

describe("Token", function () {
  it("Should deploy and mint initial supply", async function () {
    const Token = await ethers.getContractFactory("MyToken");
    const token = await Token.deploy("Test Token", "TEST");
    
    const [owner] = await ethers.getSigners();
    const balance = await token.balanceOf(owner.address);
    
    expect(balance).to.equal(ethers.parseEther("1000000"));
  });
});
```

### 4. Deployment

```typescript
// scripts/deploy.ts
import { ethers } from "hardhat";

async function main() {
  const Token = await ethers.getContractFactory("MyToken");
  const token = await Token.deploy("My Token", "MTK");
  
  await token.waitForDeployment();
  console.log("Token deployed to:", await token.getAddress());
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
```

## 🛠️ Advanced Features

### Artifact Management

The tooling provides comprehensive artifact management:

```typescript
import { ArtifactManager } from '@neo-solidity/artifacts';

const artifacts = new ArtifactManager('./artifacts');

// Get build artifacts
const artifact = await artifacts.getBuildArtifact('Token');

// Export/import artifacts
await artifacts.exportArtifacts('./dist');
await artifacts.importArtifacts('./backup');

// Artifact validation and comparison
const validation = await artifacts.validateArtifact(artifact);
const comparison = await artifacts.compareArtifacts(old, new);
```

### Debugging Support

Built-in debugging capabilities:

```typescript
import { DebugSession } from '@neo-solidity/debugger';

// Start debug session
const session = await debugger.startSession({
  contract: 'Token',
  method: 'transfer',
  args: [recipient, amount]
});

// Set breakpoints
await session.setBreakpoint('Token.sol', 42);

// Step through execution
await session.stepInto();
await session.continue();
```

### Gas Optimization

Analyze and optimize gas usage:

```bash
# Generate gas reports
neo-forge test --gas-report

# Analyze optimization opportunities
solc-neo analyze contracts/*.sol --gas-report --size-report

# Output format options
solc-neo analyze --output table   # Console table
solc-neo analyze --output json    # JSON format  
solc-neo analyze --output csv     # CSV export
```

## 📚 API Reference

### Compiler Configuration

```typescript
interface NeoSolidityConfig {
  version?: string;
  optimizer?: {
    enabled: boolean;
    runs: number;
  };
  neo?: {
    gasCostModel?: 'ethereum' | 'neo' | 'hybrid';
    storageOptimization?: boolean;
    eventOptimization?: boolean;
  };
}
```

### Network Configuration

```typescript
interface NeoNetworkConfig {
  name: string;
  rpcUrls: string[];
  magic: number;
  addressVersion: number;
  nativeTokens: {
    gas: NeoToken;
    neo: NeoToken;
  };
}
```

### Contract Interface

```typescript
interface ContractWrapper {
  address: string;
  interface: Interface;
  
  // Read-only calls
  call(method: string, args: any[]): Promise<any>;
  
  // State-changing transactions  
  send(method: string, args: any[]): Promise<TransactionResponse>;
  
  // Gas estimation
  estimateGas(method: string, args: any[]): Promise<bigint>;
  
  // Event handling
  on(event: string, listener: Function): this;
  queryFilter(event: string, filter?: any): Promise<any[]>;
}
```

## 🔍 Troubleshooting

### Common Issues

**Compiler Not Found**
```bash
# Install compiler
npm install -g @neo-solidity/cli-tools
solc-neo install latest
```

**Network Connection Issues**
```typescript
// Check network configuration
neoNetworks: {
  testnet: {
    rpcUrls: ["https://testnet1.neo.coz.io:443"], // Verify URL
    magic: 894710606, // Correct magic number
    timeout: 30000    // Increase timeout
  }
}
```

**Gas Estimation Failures**
```typescript
// Increase gas limits
const tx = await contract.method({
  gasLimit: "50000000", // 0.5 GAS
  gasPrice: "1000"
});
```

### Debug Mode

Enable debug logging:

```bash
DEBUG=neo-solidity:* npx hardhat neo-compile
DEBUG=neo-foundry:* neo-forge build
```

## 🤝 Contributing

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
git clone https://github.com/r3e-network/neo-solidity
cd neo-solidity/tooling
npm install
npm run build
```

### Testing

```bash
npm test                 # Run all tests
npm run test:watch       # Watch mode
npm run test:coverage    # Coverage report
```

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built for the Neo ecosystem** 🚀
