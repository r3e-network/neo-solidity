import { extendConfig, extendEnvironment, task, types, subtask } from "hardhat/config";
import { lazyObject } from "hardhat/plugins";
import { HardhatConfig, HardhatUserConfig, HardhatRuntimeEnvironment } from "hardhat/types";
import { 
  NeoHardhatConfig, 
  CompilationResult,
  DeploymentOptions,
  DeploymentResult,
  TestResult,
  DebugSession,
  GasReport,
  VerificationData
} from "@neo-solidity/types";

// Import all tasks and subtasks
import "./tasks/compile";
import "./tasks/clean";
import "./tasks/verify-contract";
import "./tasks/deploy";
import "./tasks/test";
import "./tasks/debug";
import "./tasks/gas-report";
import "./tasks/analyze";
import "./tasks/optimize";

// Import core components
import { NeoSolidityCompiler } from "./compiler";
import { ArtifactManager } from "./artifacts";
import { DebugManager } from "./debug";
import { GasProfiler } from "./profiler";
import { DeploymentManager } from "./deployment";
import { NetworkManager } from "./network";
import { SourceMapGenerator } from "./sourcemap";
import { OptimizationEngine } from "./optimizer";

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
      debugger: DebugManager;
      profiler: GasProfiler;
      deployer: DeploymentManager;
      network: NetworkManager;
      sourcemap: SourceMapGenerator;
      optimizer: OptimizationEngine;
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
    artifacts: new ArtifactManager(hre.config.paths.artifacts),
    debugger: new DebugManager(hre.config.neoSolc),
    profiler: new GasProfiler(hre.config.neoSolc),
    deployer: new DeploymentManager(hre.config.neoSolc, hre.network),
    network: new NetworkManager(hre.config.neoSolc.networks),
    sourcemap: new SourceMapGenerator(),
    optimizer: new OptimizationEngine(hre.config.neoSolc)
  }));
});

export * from "./compiler";
export * from "./artifacts";
export * from "./debug";
export * from "./profiler";
export * from "./deployment";
export * from "./network";
export * from "./sourcemap";
export * from "./optimizer";
export * from "./types";