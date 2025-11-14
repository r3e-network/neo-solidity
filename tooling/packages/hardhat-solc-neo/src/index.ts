import { extendConfig, extendEnvironment } from "hardhat/config";
import { lazyObject } from "hardhat/plugins";
import { HardhatConfig, HardhatUserConfig, HardhatRuntimeEnvironment } from "hardhat/types";
import { NeoHardhatConfig } from "@neo-solidity/types";

// Import supported tasks
import "./tasks/compile";
import "./tasks/clean";
import "./tasks/verify-contract";

// Import core components
import { NeoSolidityCompiler } from "./compiler";
import { ArtifactManager } from "./artifacts";

// Configuration extension
declare module "hardhat/types/config" {
  export interface HardhatUserConfig {
    neoSolc?: Partial<NeoHardhatConfig>;
  }

  export interface HardhatConfig {
    neoSolc: NeoHardhatConfig;
  }
}

// Runtime environment extension  
declare module "hardhat/types/runtime" {
  export interface HardhatRuntimeEnvironment {
    neoSolc: {
      compiler: NeoSolidityCompiler;
      artifacts: ArtifactManager;
    };
  }
}

// Default configuration
const DEFAULT_NEO_HARDHAT_CONFIG: NeoHardhatConfig = {
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      },
      outputSelection: {
        "*": {
          "*": [
            "abi",
            "evm.bytecode",
            "evm.deployedBytecode", 
            "evm.methodIdentifiers",
            "evm.gasEstimates",
            "metadata",
            "storageLayout",
            "evm.assembly",
            "evm.legacyAssembly"
          ]
        }
      },
      neo: {
        generateNef: true,
        generateManifest: true,
        optimizeGas: true,
        debugInfo: true
      }
    }
  },
  networks: {
    hardhat: {
      magic: 860833102,
      addressVersion: 53,
      rpc: {
        url: "http://127.0.0.1:10332",
        timeout: 30000
      },
      accounts: [],
      gasLimit: "9007199254740991",
      gasPrice: "0",
      blockGasLimit: "9007199254740991"
    }
  },
  paths: {
    sources: "./contracts",
    artifacts: "./artifacts",
    cache: "./cache",
    tests: "./test"
  },
  neo: {
    rpcUrl: "http://127.0.0.1:10332",
    privateKey: "",
    addressVersion: 53,
    magic: 860833102,
    gasLimit: "9007199254740991",
    gasPrice: "0"
  }
};

// Extend Hardhat configuration
extendConfig(
  (config: HardhatConfig, userConfig: Readonly<HardhatUserConfig>) => {
    config.neoSolc = {
      ...DEFAULT_NEO_HARDHAT_CONFIG,
      ...userConfig.neoSolc,
      solidity: {
        ...DEFAULT_NEO_HARDHAT_CONFIG.solidity,
        ...userConfig.neoSolc?.solidity,
        settings: {
          ...DEFAULT_NEO_HARDHAT_CONFIG.solidity.settings,
          ...userConfig.neoSolc?.solidity?.settings,
          neo: {
            ...DEFAULT_NEO_HARDHAT_CONFIG.solidity.settings.neo,
            ...userConfig.neoSolc?.solidity?.settings?.neo
          }
        }
      },
      networks: {
        ...DEFAULT_NEO_HARDHAT_CONFIG.networks,
        ...userConfig.neoSolc?.networks
      },
      paths: {
        ...DEFAULT_NEO_HARDHAT_CONFIG.paths,
        ...userConfig.neoSolc?.paths
      },
      neo: {
        ...DEFAULT_NEO_HARDHAT_CONFIG.neo,
        ...userConfig.neoSolc?.neo
      }
    };
  }
);

// Extend Hardhat runtime environment
extendEnvironment((hre: HardhatRuntimeEnvironment) => {
  hre.neoSolc = lazyObject(() => ({
    compiler: new NeoSolidityCompiler(hre.config.neoSolc, hre.config.paths),
    artifacts: new ArtifactManager(hre.config.paths.artifacts)
  }));
});

export * from "./compiler";
export * from "./artifacts";
export * from "./types";
